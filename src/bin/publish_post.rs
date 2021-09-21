extern crate diesel;
extern crate rusty_journal;

use self::diesel::prelude::*;
use self::models::Post;
use self::rusty_journal::*;
use std::env::args;

fn main() {
    use rusty_journal::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = establish_connection();

    diesel::update(posts.find(id))
        .set(published.eq(1))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));

    let post = posts
        .find(id)
        .first::<Post>(&connection)
        .expect("Error loading user");

    println!("Published post {} ({})", post.title, post.id);
}
