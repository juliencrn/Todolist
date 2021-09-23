use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write a task to the journal file.
    Add {
        #[structopt()]
        text: String,
    },
    /// Set a task completed by id.
    Done {
        #[structopt()]
        id: i32,
    },
    /// List resting tasks from the journal file.
    List,
    /// List all tasks from the journal file.
    ListAll,
    /// Update task description by id
    Update {
        #[structopt()]
        id: i32,
        #[structopt()]
        text: String,
    },
    /// Delete an given task by id
    Delete {
        #[structopt()]
        id: i32,
    },
    /// Delete all tasks
    Reset,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust and SQLite"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
