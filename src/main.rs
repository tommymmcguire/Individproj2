use rusqlite::{Connection, params, Error};
use csv::Reader;
use reqwest;
use std::io::Write;
use std::error::Error as StdError;

fn create_table() -> Result<(), Error> {
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS IMDB_Movie_Data (
            rank INTEGER,
            title TEXT,
            genre TEXT,
            description TEXT,
            director TEXT,
            actors TEXT,
            year INTEGER,
            runtime_minutes INTEGER,
            rating REAL,
            votes INTEGER,
            revenue_millions REAL,
            metascore INTEGER
        )",
        params![],
    )?;
    Ok(())
}

fn load_csv_into_db(file_path: &str) -> Result<(), Box<dyn StdError>> {
    let file = std::fs::File::open(file_path)?;
    let mut conn = Connection::open("IMDB_Movie_Data.db")?;
    let mut tx = conn.transaction()?;

    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;

        // Check if any field contains an empty string
        if record.iter().any(|field| field.is_empty()) {
            // Skip this row because it contains an empty string
            continue;
        }

        tx.execute(
            "INSERT INTO IMDB_Movie_Data VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                record[0].parse::<i32>()?,
                record[1].to_string(),
                record[2].to_string(),
                record[3].to_string(),
                record[4].to_string(),
                record[5].to_string(),
                record[6].parse::<i32>()?,
                record[7].parse::<i32>()?,
                record[8].parse::<f64>()?,
                record[9].parse::<i32>()?,
                record[10].parse::<f64>()?,
                record[11].parse::<i32>()?,
            ],
        )?;
    }

    tx.commit()?;
    Ok(())
}


fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn StdError>> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(file_path)?;
    let bytes = response.bytes()?;
    file.write_all(&bytes)?;
    Ok(())
}

fn query_top5() -> Result<(), Error> {
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM IMDB_Movie_Data LIMIT 5")?;

    println!("Top 5 rows of the IMDB_Movie_Data table:");

    let rows = stmt.query_map(params![], |row| {
        Ok((
            row.get::<usize, i32>(0),
            row.get::<usize, String>(1),
            row.get::<usize, String>(2),
            row.get::<usize, String>(3),
            row.get::<usize, String>(4),
            row.get::<usize, String>(5),
            row.get::<usize, i32>(6),
            row.get::<usize, i32>(7),
            row.get::<usize, f64>(8),
            row.get::<usize, i32>(9),
            row.get::<usize, f64>(10),
            row.get::<usize, i32>(11),
        ))
    })?;

    for row in rows {
        let row = row?;
        println!("{:?}", row);
    }

    Ok(())
}

fn query_best_genre(genre: &str) -> Result<(), Error> {
    // Similar modification for the query that expects multiple rows
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    let mut stmt = conn.prepare(
        "SELECT rating, metascore, title, genre, description, actors
         FROM IMDB_Movie_Data
         WHERE genre LIKE ?
         ORDER BY rating DESC
         LIMIT 3",
    )?;

    let genre_param = format!("%{}%", genre);
    println!("Best movies based on genre {}: ", genre);

    let rows = stmt.query_map(params![genre_param], |row| {
        Ok((
            row.get::<usize, f64>(0),
            row.get::<usize, i32>(1),
            row.get::<usize, String>(2),
            row.get::<usize, String>(3),
            row.get::<usize, String>(4),
            row.get::<usize, String>(5),
        ))
    })?;

    for row in rows {
        let row = row?;
        println!("{:?}", row);
    }

    Ok(())
}


fn create_record(
    rank: i32,
    title: &str,
    genre: &str,
    description: &str,
    director: &str,
    actors: &str,
    year: i32,
    runtime_minutes: i32,
    rating: f64,
    votes: i32,
    revenue_millions: f64,
    metascore: i32,
) -> Result<(), Error> {
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    conn.execute(
        "INSERT INTO IMDB_Movie_Data (rank, title, genre, description, director, actors, year, runtime_minutes, rating, votes, revenue_millions, metascore) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            rank, title, genre, description, director, actors, year, runtime_minutes, rating, votes, revenue_millions, metascore
        ],
    )?;
    Ok(())
}

fn read_record(rank: i32) -> Result<(), Error> {
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    let mut stmt = conn.prepare("SELECT * FROM IMDB_Movie_Data WHERE rank = ?")?;

    let record = stmt.query_row(params![rank], |row| {
        Ok((
            row.get::<usize, i32>(0),
            row.get::<usize, String>(1),
            row.get::<usize, String>(2),
            row.get::<usize, String>(3),
            row.get::<usize, String>(4),
            row.get::<usize, String>(5),
            row.get::<usize, i32>(6),
            row.get::<usize, i32>(7),
            row.get::<usize, f64>(8),
            row.get::<usize, i32>(9),
            row.get::<usize, f64>(10),
            row.get::<usize, i32>(11),
        ))
    })?;

    println!("Record with rank {}: {:?}", rank, record);

    Ok(())
}

fn update_record(rank: i32, new_rating: f64) -> Result<(), Error> {
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    conn.execute(
        "UPDATE IMDB_Movie_Data SET rating = ? WHERE rank = ?",
        params![new_rating, rank],
    )?;
    Ok(())
}

fn delete_record(rank: i32) -> Result<(), Error> {
    let conn = Connection::open("IMDB_Movie_Data.db")?;
    conn.execute("DELETE FROM IMDB_Movie_Data WHERE rank = ?", params![rank])?;
    Ok(())
}


fn main() -> Result<(), Box<dyn StdError>> {
    // Extract data from URL and load it into the SQLite database
    extract("https://raw.githubusercontent.com/laxmimerit/All-CSV-ML-Data-Files-Download/master/IMDB-Movie-Data.csv", "IMDB-Movie-Data.csv")?;
    create_table()?; // Create the table
    load_csv_into_db("IMDB-Movie-Data.csv")?;

    // Query the top 5 rows
    query_top5()?;

    // Query the best movies based on genre
    query_best_genre("Action")?;

    // Create a new record
    create_record(
        1001,
        "New Movie",
        "Drama",
        "A new movie",
        "Director X",
        "Actor A, Actor B",
        2023,
        120,
        8.5,
        1000,
        50.0,
        85,
    )?;

    // Read a record
    read_record(1001)?;

    Ok(())
}