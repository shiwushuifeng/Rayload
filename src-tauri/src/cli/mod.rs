//! Modern headless CLI: YAML config, interactive init, daemon, systemd and devices.

mod api;
mod commands;
mod config;
mod style;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};

use style::ColorChoice;

use config::Overrides;

#[derive(Parser)]
#[command(
    name = "ray",
    bin_name = "ray",
    version,
    about = "Rayload headless downloader",
    visible_alias = "rayload"
)]
struct Cli {
    /// Configuration file (default: ~/.config/rayload/config.yml).
    #[arg(long, global = true, env = "RAYLOAD_CONFIG")]
    config: Option<PathBuf>,

    /// Engine state and LAN identity directory.
    #[arg(long, global = true, env = "RAYLOAD_DATA_DIR")]
    data_dir: Option<PathBuf>,

    #[arg(long, global = true, env = "RAYLOAD_DOWNLOAD_DIR")]
    download_dir: Option<PathBuf>,

    #[arg(long, global = true, env = "RAYLOAD_ARIA2_BIN")]
    engine_bin: Option<PathBuf>,

    #[arg(long, global = true, env = "RAYLOAD_DEVICE_NAME")]
    device_name: Option<String>,

    #[arg(long, global = true, env = "RAYLOAD_LAN_PORT")]
    lan_port: Option<u16>,

    #[arg(long, global = true, env = "RAYLOAD_RPC_PORT")]
    rpc_port: Option<u16>,

    #[arg(long, global = true, env = "RAYLOAD_LAN_USERNAME")]
    lan_username: Option<String>,

    #[arg(long, global = true, env = "RAYLOAD_LAN_PASSWORD")]
    lan_password: Option<String>,

    #[arg(
        long,
        global = true,
        env = "RAYLOAD_REQUIRE_MANUAL_CONFIRM",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    require_manual_confirm: Option<bool>,

    /// Disable the LAN API for this run.
    #[arg(long, global = true)]
    no_lan: bool,

    /// Increase log verbosity.
    #[arg(short = 'v', long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Print machine-readable JSON where the command supports it.
    #[arg(long, global = true)]
    json: bool,

    /// Only print results, never decorative output.
    #[arg(short = 'q', long, global = true)]
    quiet: bool,

    /// Colorize output.
    #[arg(long, global = true, value_enum, default_value_t = ColorChoice::Auto)]
    color: ColorChoice,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Interactively create config.yml.
    #[command(visible_alias = "i")]
    Init {
        #[arg(long)]
        force: bool,
    },
    /// Start the Aria2 engine and the LAN API.
    #[command(visible_alias = "up")]
    Start {
        /// Kept for compatibility; background is now the default.
        #[arg(short = 'd', long, hide = true)]
        daemon: bool,
        /// Stay attached to the terminal instead of detaching.
        #[arg(short = 'f', long)]
        foreground: bool,
        /// Stop the running instance before starting.
        #[arg(long)]
        restart: bool,
    },
    /// Add one or more URLs to the download queue.
    Add {
        /// URLs to download. `-` reads them from stdin.
        #[arg(num_args = 0..)]
        urls: Vec<String>,
        /// Read URLs from a file, one per line (`-` for stdin).
        #[arg(long)]
        input_file: Option<PathBuf>,
        /// Override the saved file name; only valid for a single URL.
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// List downloads in a table.
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        json: bool,
    },
    /// Pause a download (GID, unique GID prefix, or `all`).
    Pause { target: String },
    /// Resume a paused download (GID, unique GID prefix, or `all`).
    Resume { target: String },
    /// Remove downloads from Aria2; downloaded files are kept.
    #[command(visible_alias = "remove")]
    Rm { target: String },
    /// Follow progress until every matching task finishes.
    #[command(visible_alias = "w")]
    Watch {
        /// GID, unique GID prefix, or `all`; defaults to every task.
        target: Option<String>,
        /// Refresh interval in milliseconds.
        #[arg(long, default_value_t = 1000)]
        interval_ms: u64,
        #[arg(long)]
        json: bool,
    },
    /// Print the daemon log, optionally following it.
    Log {
        #[arg(short = 'n', long, default_value_t = 40)]
        lines: usize,
        #[arg(short = 'f', long)]
        follow: bool,
    },
    /// Inspect local JSON plugins.
    #[command(visible_alias = "p")]
    Plugin {
        #[command(subcommand)]
        action: Option<PluginAction>,
    },
    /// Stop a background instance.
    Stop,
    /// Stop the installed service (or a manual background instance).
    Off,
    /// Install and enable the systemd service.
    Install {
        /// Install a user-level service instead of a system-wide one.
        #[arg(long)]
        user: bool,
    },
    /// Audit the runtime environment.
    SecurityCheck {
        #[arg(long)]
        json: bool,
    },
    /// Show a status tree.
    #[command(visible_alias = "s")]
    Status {
        #[arg(long)]
        json: bool,
    },
    /// Change the LAN admin credentials in config.yml.
    #[command(visible_alias = "a")]
    Auth,
    /// Show or change configuration values.
    #[command(visible_alias = "c")]
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
    /// Pairing and device management.
    #[command(visible_alias = "d")]
    Device {
        #[command(subcommand)]
        action: Option<DeviceAction>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Print the effective configuration.
    Show,
    /// Print the configuration file path.
    Path,
    /// Print one dotted key.
    Get { key: String },
    /// Set one dotted key.
    Set { key: String, value: String },
}

#[derive(Subcommand)]
enum PluginAction {
    /// List JSON plugin manifests.
    #[command(visible_alias = "ls")]
    List {
        /// Directory to scan instead of the default plugin locations.
        #[arg(long)]
        dir: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum DeviceAction {
    /// List paired devices.
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        json: bool,
    },
    /// Revoke a paired device.
    #[command(visible_alias = "rm")]
    Revoke { id: String },
    /// List pairing requests waiting for approval.
    Pending {
        #[arg(long)]
        json: bool,
    },
    /// Approve a pending pairing request.
    Allow { id: String },
    /// Reject a pending pairing request.
    Reject { id: String },
}

pub fn run() -> ExitCode {
    let cli = Cli::parse_from(normalize_args());
    style::set_quiet(cli.quiet);
    style::set_color_choice(cli.color);
    init_logger(cli.verbose);
    let overrides = Overrides {
        config: cli.config,
        data_dir: cli.data_dir,
        download_dir: cli.download_dir,
        engine_bin: cli.engine_bin,
        device_name: cli.device_name,
        lan_port: cli.lan_port,
        rpc_port: cli.rpc_port,
        lan_username: cli.lan_username,
        lan_password: cli.lan_password,
        require_manual_confirm: cli.require_manual_confirm,
        no_lan: cli.no_lan,
    };
    let Some(command) = cli.command else {
        let mut root = Cli::command();
        let _ = root.print_help();
        println!();
        return ExitCode::from(2);
    };
    match dispatch(command, &overrides, cli.json) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ray: {error}");
            ExitCode::from(exit_code_for(&error))
        }
    }
}

/// Distinct exit codes let scripts tell "retry later" from "give up".
/// 0 ok, 1 general, 2 usage, 3 daemon down, 4 nothing matched, 5 credentials.
fn exit_code_for(message: &str) -> u8 {
    if message.starts_with(api::NOT_RUNNING_PREFIX) {
        3
    } else if message.starts_with(api::NO_RESPONSE_PREFIX) {
        3
    } else if message.starts_with(api::AUTH_PREFIX) {
        5
    } else if message.starts_with("no task matches") {
        4
    } else if message.contains("cannot be empty") || message.contains("invalid value") {
        2
    } else {
        1
    }
}

/// Collects URLs from the arguments and an optional input file.
fn collect_urls(urls: Vec<String>, input_file: Option<&Path>) -> Result<Vec<String>, String> {
    let mut collected = urls;
    if let Some(path) = input_file {
        // A dash reads stdin so xargs-style pipelines work.
        let content = if path == Path::new("-") {
            let mut buffer = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut buffer)
                .map_err(|error| error.to_string())?;
            buffer
        } else {
            std::fs::read_to_string(path).map_err(|error| error.to_string())?
        };
        collected.extend(
            content
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty() && !line.starts_with('#')),
        );
    }
    Ok(collected)
}

fn dispatch(command: Command, overrides: &Overrides, json: bool) -> Result<(), String> {
    match command {
        Command::Init { force } => commands::init(overrides, force),
        Command::Start {
            daemon,
            foreground,
            restart,
        } => commands::start(overrides, daemon || !foreground, restart),
        Command::Add {
            urls,
            input_file,
            out,
            json: sub_json,
        } => {
            let urls = collect_urls(urls, input_file.as_deref())?;
            commands::add(overrides, &urls, out.as_deref(), json || sub_json)
        }
        Command::List { json: sub_json } => commands::list(overrides, json || sub_json),
        Command::Pause { target } => commands::pause(overrides, &target),
        Command::Resume { target } => commands::resume(overrides, &target),
        Command::Rm { target } => commands::remove(overrides, &target),
        Command::Watch {
            target,
            interval_ms,
            json: sub_json,
        } => commands::watch(overrides, target.as_deref(), interval_ms, json || sub_json),
        Command::Log { lines, follow } => commands::log(overrides, lines, follow),
        Command::Plugin { action } => match action.unwrap_or(PluginAction::List {
            dir: None,
            json: false,
        }) {
            PluginAction::List { dir, json } => {
                commands::plugin_list(overrides, dir.as_deref(), json)
            }
        },
        Command::Stop => commands::stop(overrides),
        Command::Off => commands::off(overrides),
        Command::Install { user } => commands::install(overrides, user),
        Command::SecurityCheck { json } => commands::security_check(overrides, json),
        Command::Status { json } => commands::status(overrides, json),
        Command::Auth => commands::auth(overrides),
        Command::Config { action } => match action.unwrap_or(ConfigAction::Show) {
            ConfigAction::Show => commands::config_show(overrides),
            ConfigAction::Path => commands::config_path(overrides),
            ConfigAction::Get { key } => commands::config_get(overrides, &key),
            ConfigAction::Set { key, value } => commands::config_set(overrides, &key, &value),
        },
        Command::Device { action } => match action.unwrap_or(DeviceAction::List { json: false }) {
            DeviceAction::List { json } => commands::device_list(overrides, json),
            DeviceAction::Revoke { id } => commands::device_revoke(overrides, &id),
            DeviceAction::Pending { json } => commands::device_pending(overrides, json),
            DeviceAction::Allow { id } => commands::device_allow(overrides, &id),
            DeviceAction::Reject { id } => commands::device_reject(overrides, &id),
        },
    }
}

/// Accepts the documented `-s`, `-a`, `-c` and `-d` shorthands.
fn normalize_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return args;
    }
    if let Some(command) = normalized_command(&args[1]) {
        args[1] = command.to_string();
    }
    args
}

fn normalized_command(argument: &str) -> Option<&'static str> {
    match argument {
        "-s" | "--status" => Some("status"),
        "-a" | "--auth" => Some("auth"),
        "-c" => Some("config"),
        "-d" | "--device" => Some("device"),
        _ => None,
    }
}

struct CliLogger;

impl log::Log for CliLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        log::max_level() >= metadata.level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "[{}] {}: {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: CliLogger = CliLogger;

fn init_logger(verbose: u8) {
    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(if verbose > 0 {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::api::{AUTH_PREFIX as AUTH, NOT_RUNNING_PREFIX as NOT_RUNNING, NO_RESPONSE_PREFIX as NO_RESPONSE};

    #[test]
    fn documented_short_flags_are_dispatch_commands() {
        assert_eq!(normalized_command("-s"), Some("status"));
        assert_eq!(normalized_command("-a"), Some("auth"));
        assert_eq!(normalized_command("-c"), Some("config"));
        assert_eq!(normalized_command("-d"), Some("device"));
    }

    #[test]
    fn exit_codes_separate_failure_kinds() {
        assert_eq!(exit_code_for(&format!("{NOT_RUNNING}: connection refused")), 3);
        assert_eq!(exit_code_for(&format!("{NO_RESPONSE}: timeout")), 3);
        assert_eq!(exit_code_for(&format!("{AUTH}: HTTP 401")), 5);
        assert_eq!(exit_code_for("no task matches 'abc'"), 4);
        assert_eq!(exit_code_for("URL cannot be empty"), 2);
        assert_eq!(exit_code_for("could not remove x"), 1);
        assert_eq!(exit_code_for(""), 1);
    }

    #[test]
    fn add_accepts_multiple_and_stdin_urls() {
        let parsed = Cli::try_parse_from(["ray", "add", "https://a", "https://b"])
            .expect("multiple urls parse");
        match parsed.command {
            Some(Command::Add { urls, .. }) => {
                assert_eq!(urls, vec!["https://a".to_string(), "https://b".to_string()]);
            }
            _ => panic!("expected add"),
        }

        let from_file = Cli::try_parse_from(["ray", "add", "--input-file", "urls.txt"])
            .expect("input file parses");
        match from_file.command {
            Some(Command::Add { input_file, urls, .. }) => {
                assert!(urls.is_empty());
                assert_eq!(input_file.as_deref().unwrap().to_string_lossy(), "urls.txt");
            }
            _ => panic!("expected add"),
        }
    }

    #[test]
    fn global_json_quiet_and_color_apply_to_every_subcommand() {
        let parsed = Cli::try_parse_from(["ray", "--json", "--quiet", "--color", "never", "list"])
            .expect("globals parse");
        assert!(parsed.json);
        assert!(parsed.quiet);
        assert_eq!(parsed.color, ColorChoice::Never);
    }

    #[test]
    fn targets_accept_prefixes_and_all() {
        // The selector parsing itself lives in resolve_targets; this covers the
        // argument shapes the CLI must accept.
        let all = Cli::try_parse_from(["ray", "pause", "all"]).expect("all parses");
        match all.command {
            Some(Command::Pause { target }) => assert_eq!(target, "all"),
            _ => panic!("expected pause"),
        }
        let prefix = Cli::try_parse_from(["ray", "rm", "abcd"]).expect("prefix parses");
        match prefix.command {
            Some(Command::Rm { target }) => assert_eq!(target, "abcd"),
            _ => panic!("expected rm"),
        }
    }

    #[test]
    fn watch_takes_interval_and_optional_target() {
        let parsed = Cli::try_parse_from(["ray", "watch", "abcd", "--interval-ms", "250"])
            .expect("watch parses");
        match parsed.command {
            Some(Command::Watch {
                target,
                interval_ms,
                ..
            }) => {
                assert_eq!(target.as_deref(), Some("abcd"));
                assert_eq!(interval_ms, 250);
            }
            _ => panic!("expected watch"),
        }
        let defaults = Cli::try_parse_from(["ray", "watch"]).expect("watch defaults parse");
        match defaults.command {
            Some(Command::Watch { target, interval_ms, .. }) => {
                assert!(target.is_none());
                assert_eq!(interval_ms, 1000);
            }
            _ => panic!("expected watch"),
        }
    }

    #[test]
    fn start_defaults_to_background_and_accepts_foreground() {
        let detached = Cli::try_parse_from(["ray", "start"]).expect("start parses");
        match detached.command {
            Some(Command::Start {
                daemon,
                foreground,
                restart,
            }) => {
                assert!(!daemon && !foreground && !restart);
            }
            _ => panic!("expected start"),
        }

        let attached =
            Cli::try_parse_from(["ray", "start", "--foreground"]).expect("foreground parses");
        match attached.command {
            Some(Command::Start { foreground, .. }) => assert!(foreground),
            _ => panic!("expected start"),
        }
    }
}
