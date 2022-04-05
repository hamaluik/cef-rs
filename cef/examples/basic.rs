use anyhow::Result;
use simplelog::*;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .expect("can initialize logging");

    let cef = cef::Cef::initialize()?;
    cef.run()
}
