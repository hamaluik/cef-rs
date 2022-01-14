use anyhow::Result;
use simplelog::*;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .expect("can initialize logging");

    let cef = cef97::Cef::initialize()?;
    cef.run()
}
