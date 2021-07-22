pub mod cli;
pub mod tasks;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::io::stdout;
use std::path::PathBuf;
use tasks::Task;

pub fn run(cli_args: CommandLineArgs) -> anyhow::Result<()> {
    // Unpack the journal file and apply default if it's empty.
    let journal_file = cli_args
        .journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    // Perform the action.
    match cli_args.action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file, &mut stdout()),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
