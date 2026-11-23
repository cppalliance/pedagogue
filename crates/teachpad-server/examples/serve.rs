//! Runs the server and the demo together, for a browser.
//!
//! ```text
//! cargo run -p teachpad-server --example serve -- [--script <file>] [--fast] [--port <number>] [--ui <folder>]
//! ```
//!
//! It prints the address to open. `--script` plays a sample script in place
//! of the built-in straw poll lesson. `--fast` plays it with no waits, for
//! screenshots. `--port` asks for a fixed port. `--ui` serves the UI files
//! from a folder such as `ui/dist`, which `npm run watch` keeps current, so
//! a style edit shows on refresh. Ctrl+C stops it.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, bail};
use teachpad_demo::{Pace, STRAW_POLL, Script, run};
use teachpad_server::{Options, bind, serve};
use teachpad_session::Session;

const USAGE: &str = "usage: serve [--script <file>] [--fast] [--port <number>] [--ui <folder>]";

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

/// What the command line asks for.
#[derive(Debug, Default)]
struct Args {
    script: Option<PathBuf>,
    fast: bool,
    port: Option<u16>,
    ui: Option<PathBuf>,
}

impl Args {
    fn parse(mut words: impl Iterator<Item = String>) -> anyhow::Result<Self> {
        let mut args = Self::default();
        while let Some(word) = words.next() {
            let mut value = || {
                words
                    .next()
                    .with_context(|| format!("{word} needs a value. {USAGE}"))
            };
            match word.as_str() {
                "--script" => args.script = Some(PathBuf::from(value()?)),
                "--fast" => args.fast = true,
                "--port" => {
                    let port = value()?;
                    let number = port
                        .parse()
                        .with_context(|| format!("`{port}` is not a port number"))?;
                    args.port = Some(number);
                }
                "--ui" => args.ui = Some(PathBuf::from(value()?)),
                other => bail!("unknown argument `{other}`. {USAGE}"),
            }
        }
        Ok(args)
    }
}

#[expect(
    clippy::disallowed_methods,
    reason = "this binary entry point installs the one log subscriber of the process"
)]
fn start() -> anyhow::Result<()> {
    let args = Args::parse(std::env::args().skip(1))?;
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("install the log subscriber")?;

    // Everything that can fail on bad input runs before the server starts.
    let script = match &args.script {
        Some(path) => {
            let text = std::fs::read_to_string(path)
                .with_context(|| format!("read the script {}", path.display()))?;
            Script::parse(&text).with_context(|| format!("parse the script {}", path.display()))?
        }
        None => Script::parse(STRAW_POLL).context("parse the built-in script")?,
    };
    let listener = bind(args.port)?;
    let address = listener
        .local_addr()
        .context("read the address of the listener")?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("start the async runtime")?;
    runtime.block_on(async {
        let (session, mut events) = Session::new();
        let options = match args.ui {
            Some(folder) => Options::new().with_ui_folder(folder),
            None => Options::new(),
        };
        let pace = if args.fast { Pace::Instant } else { Pace::Live };
        println!("http://{address}");

        let app = serve(listener, session.clone(), options, ctrl_c());
        let demo = run(&session, &mut events, &script, pace);
        tokio::select! {
            ended = app => ended.context("serve the app"),
            played = demo => played.context("play the script"),
        }
    })
}

/// Resolves when the user presses Ctrl+C. If the handler cannot be
/// installed, the server runs until the process is killed.
async fn ctrl_c() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "Ctrl+C cannot be watched, so only a kill stops this server");
        std::future::pending::<()>().await;
    }
}
