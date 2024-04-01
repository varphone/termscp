const TERMSCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const TERMSCP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// Crates
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate magic_crypt;

// External libs
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

// Include
mod activity_manager;
mod cli_opts;
mod config;
mod explorer;
mod filetransfer;
mod host;
mod support;
mod system;
mod ui;
mod utils;

// namespaces
use activity_manager::{ActivityManager, NextActivity};
use cli_opts::{Args, ArgsSubcommands, BookmarkParams, HostParams, Remote, RunOpts, Task};
use filetransfer::FileTransferParams;
use system::logging::{self, LogLevel};

fn main() {
    let mut args: Args = argh::from_env();

    if let Some(ref secure_password) = args.secure_password {
        match decrypt_secure_password(&secure_password) {
            Ok(password) => {
                args.password = Some(password);
            }
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(255);
            }
        }
    }

    // Parse args
    let run_opts: RunOpts = match parse_args(args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(255);
        }
    };
    // Setup logging
    if let Err(err) = logging::init(run_opts.log_level) {
        eprintln!("Failed to initialize logging: {err}");
    }
    info!("termscp {} started!", TERMSCP_VERSION);
    // Run
    info!("Starting activity manager...");
    let rc = run(run_opts);
    info!("termscp terminated with exitcode {}", rc);
    // Then return
    std::process::exit(rc);
}

/// Decrypt secure password
///
/// # Arguments
///
/// * `secure_password` - Encrypted password, base64 encoded string
///
/// # Returns
///
/// * `Result<String, String>` - Decrypted password or error message
fn decrypt_secure_password(secure_password: &str) -> Result<String, String> {
    use magic_crypt::{MagicCrypt256, MagicCryptTrait};

    const KEY_DATA: &[u8] = include_bytes!("../assets/secure-key.bin");

    let i1 = KEY_DATA[128] as usize;
    let i2 = KEY_DATA[129] as usize;
    let mut key_buf = [0u8; 32];
    let mut iv_buf = [0u8; 16];
    key_buf.copy_from_slice(&KEY_DATA[i1..i1 + 32]);
    iv_buf.copy_from_slice(&KEY_DATA[i2..i2 + 16]);

    let key = String::from_utf8_lossy(&key_buf);
    let iv = String::from_utf8_lossy(&iv_buf);

    let mc: MagicCrypt256 = new_magic_crypt!(key, 256, iv);
    mc.decrypt_base64_to_string(secure_password)
        .map_err(|e| format!("Could not decrypt secure password: {e}"))
}

/// Parse arguments
/// In case of success returns `RunOpts`
/// in case something is wrong returns the error message
fn parse_args(args: Args) -> Result<RunOpts, String> {
    let run_opts = match args.nested {
        Some(ArgsSubcommands::Update(_)) => RunOpts::update(),
        Some(ArgsSubcommands::LoadTheme(args)) => RunOpts::import_theme(args.theme),
        Some(ArgsSubcommands::Config(_)) => RunOpts::config(),
        None => {
            let mut run_opts: RunOpts = RunOpts::default();
            // Version
            if args.version {
                return Err(format!(
                    "termscp - {TERMSCP_VERSION} - Developed by {TERMSCP_AUTHORS}",
                ));
            }
            // Logging
            if args.debug {
                run_opts.log_level = LogLevel::Trace;
            } else if args.quiet {
                run_opts.log_level = LogLevel::Off;
            }
            // Match ticks
            run_opts.ticks = Duration::from_millis(args.ticks);
            // Remote argument
            match parse_address_arg(&args) {
                Err(err) => return Err(err),
                Ok(Remote::None) => {}
                Ok(remote) => {
                    // Set params
                    run_opts.remote = remote;
                    // In this case the first activity will be FileTransfer
                    run_opts.task = Task::Activity(NextActivity::FileTransfer);
                }
            }

            // Local directory
            if let Some(localdir) = args.positional.get(1) {
                // Change working directory if local dir is set
                let localdir: PathBuf = PathBuf::from(localdir);
                if let Err(err) = env::set_current_dir(localdir.as_path()) {
                    return Err(format!("Bad working directory argument: {err}"));
                }
            }

            run_opts
        }
    };

    Ok(run_opts)
}

/// Parse address argument from cli args
fn parse_address_arg(args: &Args) -> Result<Remote, String> {
    if let Some(remote) = args.positional.first() {
        if args.address_as_bookmark {
            Ok(Remote::Bookmark(BookmarkParams::new(
                remote,
                args.password.as_ref(),
            )))
        } else {
            // Parse address
            parse_remote_address(remote.as_str())
                .map(|x| Remote::Host(HostParams::new(x, args.password.as_deref())))
        }
    } else {
        Ok(Remote::None)
    }
}

/// Parse remote address
fn parse_remote_address(remote: &str) -> Result<FileTransferParams, String> {
    utils::parser::parse_remote_opt(remote).map_err(|e| format!("Bad address option: {e}"))
}

/// Run task and return rc
fn run(run_opts: RunOpts) -> i32 {
    match run_opts.task {
        Task::ImportTheme(theme) => run_import_theme(&theme),
        Task::InstallUpdate => run_install_update(),
        Task::Activity(activity) => run_activity(activity, run_opts.ticks, run_opts.remote),
    }
}

fn run_import_theme(theme: &Path) -> i32 {
    match support::import_theme(theme) {
        Ok(_) => {
            println!("Theme has been successfully imported!");
            0
        }
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

fn run_install_update() -> i32 {
    match support::install_update() {
        Ok(msg) => {
            println!("{msg}");
            0
        }
        Err(err) => {
            eprintln!("Could not install update: {err}");
            1
        }
    }
}

fn run_activity(activity: NextActivity, ticks: Duration, remote: Remote) -> i32 {
    // Create activity manager (and context too)
    let mut manager: ActivityManager = match ActivityManager::new(ticks) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Could not start activity manager: {err}");
            return 1;
        }
    };
    // Set file transfer params if set
    match remote {
        Remote::Bookmark(BookmarkParams { name, password }) => {
            if let Err(err) = manager.resolve_bookmark_name(&name, password.as_deref()) {
                eprintln!("{err}");
                return 1;
            }
        }
        Remote::Host(HostParams { params, password }) => {
            if let Err(err) = manager.set_filetransfer_params(params, password.as_deref()) {
                eprintln!("{err}");
                return 1;
            }
        }
        Remote::None => {}
    }
    manager.run(activity);

    0
}
