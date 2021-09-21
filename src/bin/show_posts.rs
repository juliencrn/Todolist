extern crate diesel;
extern crate rusty_journal;

use self::diesel::prelude::*;
use self::rusty_journal::*;

use self::models::*;

fn main() {
    use rusty_journal::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(1))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
