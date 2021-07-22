use anyhow;
use rusty_journal::cli::CommandLineArgs;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let args = CommandLineArgs::from_args();

    rusty_journal::run(args)?;

    Ok(())
}
