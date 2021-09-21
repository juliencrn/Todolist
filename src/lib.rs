#[macro_use]
extern crate diesel;
extern crate dotenv;

// use self::models::{NewPost, Post};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io::stdout;

pub mod cli;
pub mod models;
pub mod schema;
pub mod tasks;

// use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};

pub fn run(cli_args: CommandLineArgs) -> anyhow::Result<()> {
    let connection = establish_connection();

    // Perform the action.
    match cli_args.action {
        Add { text } => tasks::add(text, &connection),
        Done { position } => todo!(),
        // List => tasks::list(&connection),
        List => tasks::list(&connection, &mut stdout()),
        Update { id, text } => todo!(),
        Delete { id } => todo!(),
    }?;

    Ok(())
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
