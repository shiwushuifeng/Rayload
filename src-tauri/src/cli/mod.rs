//! Modern headless CLI: YAML config, interactive init, daemon, systemd and devices.

mod api;
mod commands;
mod config;
mod style;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};

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
        /// Run detached in the background.
        #[arg(short = 'd', long)]
        daemon: bool,
    },
    /// Add a URL to the download queue.
    Add {
        url: String,
        /// Override the saved file name.
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
    /// Pause a download.
    Pause { gid: String },
    /// Resume a paused download.
    Resume { gid: String },
    /// Remove a download from Aria2; downloaded files are kept.
    #[command(visible_alias = "remove")]
    Rm { gid: String },
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
    /// Install and enable the systemd service.
    Install,
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
    match dispatch(command, &overrides) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ray: {error}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(command: Command, overrides: &Overrides) -> Result<(), String> {
    match command {
        Command::Init { force } => commands::init(overrides, force),
        Command::Start { daemon } => commands::start(overrides, daemon),
        Command::Add { url, out, json } => commands::add(overrides, &url, out.as_deref(), json),
        Command::List { json } => commands::list(overrides, json),
        Command::Pause { gid } => commands::pause(overrides, &gid),
        Command::Resume { gid } => commands::resume(overrides, &gid),
        Command::Rm { gid } => commands::remove(overrides, &gid),
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
        Command::Install => commands::install(overrides),
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

    #[test]
    fn documented_short_flags_are_dispatch_commands() {
        assert_eq!(normalized_command("-s"), Some("status"));
        assert_eq!(normalized_command("-a"), Some("auth"));
        assert_eq!(normalized_command("-c"), Some("config"));
        assert_eq!(normalized_command("-d"), Some("device"));
    }
}
