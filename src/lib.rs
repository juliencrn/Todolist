#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod cli;
pub mod models;
pub mod schema;

use cli::{Action::*, CommandLineArgs};

pub fn run(cli_args: CommandLineArgs) -> anyhow::Result<()> {
    let connection = establish_connection();

    // Perform the action.
    match cli_args.action {
        Add { text } => models::add(text, &connection),
        Done { id } => models::complete(id, &connection),
        List => models::list(&connection),
        ListAll => models::list_all(&connection),
        Update { id, text } => models::update(id, text, &connection),
        Delete { id } => models::delete(id, &connection),
    }?;

    Ok(())
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
