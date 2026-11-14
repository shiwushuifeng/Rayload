//! Out-of-process plugin host.
//!
//! A plugin is a child process speaking newline-delimited JSON: one request
//! object per line on stdin, one response object per line on stdout. The host
//! binds the child to its own lifetime and enforces a hard request timeout so
//! a wedged plugin cannot hang the app.

use std::io;
use std::process::Stdio;
use std::time::Duration;

use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

/// Every request carries this protocol version; plugins answer with the same.
pub const PROTOCOL_VERSION: u64 = 1;
/// Requests answer within five seconds or the plugin is killed.
pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RESPONSE_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("plugin io error: {0}")]
    Io(#[from] io::Error),
    #[error("plugin returned invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("plugin did not answer within {0:?} and was killed")]
    Timeout(Duration),
    #[error("plugin closed stdout without answering")]
    Closed,
    #[error("plugin protocol version {0} is not supported")]
    Protocol(u64),
}

pub struct PluginHost {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl PluginHost {
    /// Starts a plugin. The caller owns program, arguments and environment.
    pub fn spawn(mut command: Command) -> io::Result<Self> {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);
        bind_child_lifetime(&mut command);
        let mut child = command.spawn()?;
        let Some(stdin) = child.stdin.take() else {
            return Err(io::Error::other("plugin stdin was not piped"));
        };
        let Some(stdout) = child.stdout.take() else {
            return Err(io::Error::other("plugin stdout was not piped"));
        };
        Ok(Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        })
    }

    pub fn pid(&self) -> Option<u32> {
        self.child.id()
    }

    pub async fn request(&mut self, request: &Value) -> Result<Value, Error> {
        self.request_within(request, DEFAULT_REQUEST_TIMEOUT).await
    }

    /// Version handshake run once per plugin process. The response must echo
    /// the protocol version before business messages are accepted.
    pub async fn handshake(&mut self) -> Result<(), Error> {
        let response = self
            .request_within(
                &serde_json::json!({ "method": "handshake", "version": PROTOCOL_VERSION }),
                DEFAULT_REQUEST_TIMEOUT,
            )
            .await?;
        let version = response.get("version").and_then(Value::as_u64).unwrap_or(0);
        if version != PROTOCOL_VERSION {
            self.terminate().await;
            return Err(Error::Protocol(version));
        }
        Ok(())
    }

    /// Sends one JSON line and waits for one JSON line back. Any failure kills
    /// the plugin: after a partial line the stream cannot be trusted.
    pub async fn request_within(
        &mut self,
        request: &Value,
        timeout: Duration,
    ) -> Result<Value, Error> {
        let mut request = request.clone();
        if let Some(object) = request.as_object_mut() {
            object
                .entry("version")
                .or_insert(serde_json::json!(PROTOCOL_VERSION));
        }
        let mut line = serde_json::to_vec(&request)?;
        line.push(b'\n');
        self.stdin.write_all(&line).await?;
        self.stdin.flush().await?;

        let outcome = match tokio::time::timeout(timeout, read_response(&mut self.stdout)).await {
            Ok(Ok(Some(response))) => Ok(serde_json::from_str(&response)?),
            Ok(Ok(None)) => Err(Error::Closed),
            Ok(Err(error)) => Err(Error::Io(error)),
            Err(_) => Err(Error::Timeout(timeout)),
        };
        if outcome.is_err() {
            self.terminate().await;
        }
        outcome
    }

    async fn terminate(&mut self) {
        let _ = self.child.start_kill();
        let _ = tokio::time::timeout(Duration::from_secs(1), self.child.wait()).await;
    }
}

async fn read_response(stdout: &mut BufReader<ChildStdout>) -> io::Result<Option<String>> {
    let mut line = String::new();
    let read = stdout.take(MAX_RESPONSE_BYTES).read_line(&mut line).await?;
    if read == 0 {
        return Ok(None);
    }
    if read as u64 == MAX_RESPONSE_BYTES && !line.ends_with('\n') {
        return Err(io::Error::other("plugin response exceeded 8 MiB"));
    }
    Ok(Some(line))
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn bind_child_lifetime(command: &mut Command) {
    let parent = std::process::id() as libc::pid_t;
    // SAFETY: only async-signal-safe libc calls run between fork and exec.
    unsafe {
        command.pre_exec(move || {
            if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM) != 0 {
                return Err(io::Error::last_os_error());
            }
            // The parent may have died between fork and prctl.
            if libc::getppid() != parent {
                libc::_exit(1);
            }
            Ok(())
        });
    }
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn bind_child_lifetime(_command: &mut Command) {}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn fixture() -> Command {
        Command::new(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/echo-plugin.sh"),
        )
    }

    #[tokio::test]
    async fn echoes_json_through_stdin_and_stdout() {
        let mut host = PluginHost::spawn(fixture()).unwrap();

        let response = host
            .request(&serde_json::json!({ "id": 1, "method": "echo" }))
            .await
            .unwrap();

        assert_eq!(
            response,
            serde_json::json!({ "id": 1, "method": "echo", "version": PROTOCOL_VERSION })
        );
    }

    #[tokio::test]
    async fn handshake_accepts_protocol_version_one() {
        let mut host = PluginHost::spawn(fixture()).unwrap();
        host.handshake().await.unwrap();
    }

    #[tokio::test]
    async fn handshake_rejects_an_unknown_version() {
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "read line; echo '{\"version\": 99}'"]);
        let mut host = PluginHost::spawn(command).unwrap();
        let error = host.handshake().await.unwrap_err();
        assert!(matches!(error, Error::Protocol(99)), "{error}");
    }

    #[tokio::test]
    async fn handles_multiple_requests_on_one_host() {
        let mut host = PluginHost::spawn(fixture()).unwrap();

        for id in 1..4 {
            let request = serde_json::json!({ "id": id });
            let response = host.request(&request).await.unwrap();
            assert_eq!(response["id"], request["id"]);
            assert_eq!(response["version"], PROTOCOL_VERSION);
        }
    }

    #[tokio::test]
    async fn kills_a_wedged_plugin_after_the_timeout() {
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "sleep 30"]);
        let mut host = PluginHost::spawn(command).unwrap();
        let pid = host.pid().unwrap();

        let error = host
            .request_within(&serde_json::json!({}), Duration::from_millis(200))
            .await
            .unwrap_err();

        assert!(matches!(error, Error::Timeout(_)), "{error}");
        assert!(
            !std::path::Path::new(&format!("/proc/{pid}")).exists(),
            "plugin {pid} survived the timeout kill"
        );
    }

    #[tokio::test]
    async fn rejects_invalid_plugin_output() {
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "read line; echo not-json; sleep 5"]);
        let mut host = PluginHost::spawn(command).unwrap();

        let error = host.request(&serde_json::json!({})).await.unwrap_err();

        assert!(matches!(error, Error::Json(_)), "{error}");
    }

    #[tokio::test]
    async fn reports_a_plugin_that_exits_without_answering() {
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "read line; exit 0"]);
        let mut host = PluginHost::spawn(command).unwrap();

        let error = host.request(&serde_json::json!({})).await.unwrap_err();

        assert!(matches!(error, Error::Closed), "{error}");
    }
}
