extern crate rusqlite;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("mydb.db")?; // Connect to SQLite database

    // Create your tables and perform other database operations here

    println!("Hello, Rust CLI with SQLite!");
    Ok(())
}