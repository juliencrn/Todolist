extern crate diesel;
extern crate rusty_journal;

use self::rusty_journal::*;
use std::io::{stdin, Read};

fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) /*-> Post*/
{
    use schema::posts;

    let new_post = NewPost {
        title,
        body,
        published: &0,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    let results = posts::table
        .filter(posts::dsl::title.like(format!("%{}%", new_post.title)))
        .load::<Post>(conn)
        .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }

    // return results;
}

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    create_post(&connection, title, &body);

    // TODO: Display post.id too
    println!("\nSaved draft {}", title);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
