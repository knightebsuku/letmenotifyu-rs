use rusqlite::{Connection, Result, NO_PARAMS};
use std::path::Path;

fn migrate_movie(conn: &mut Connection) -> Result<()> {
    println!("Running the movie migration");
    let version = conn.execute("SELECT max(version) from migration", NO_PARAMS)?;
    println!("The version latest version is {}", version);
    //let tx = conn.transaction()?;
    // create migration for genre
    // create migration for status
    // create migration for config
    Ok(())
}

fn create_movie_table(conn: &mut Connection) -> Result<()> {
    println!("Creating new movie tables");
    // must start a transaction
    let tx = conn.transaction()?;
    tx.execute(
        "CREATE TABLE genre(id INTEGER PRIMARY KEY, name TEXT)",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE movie(id INTEGER PRIMARY KEY,
            title TEXT, date TEXT, yify_id INTEGER,genre_id INTEGER,
           FOREIGN KEY(genre_id) REFERENCES genre(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE detail(id INTEGER PRIMARY KEY, language TEXT,description TEXT,
            rating REAL, youtube TEXT, imdb TEXT, movie_id INTEGER,
            FOREIGN KEY(movie_id) REFERENCES movie(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE torrent_link(id INTEGER PRIMARY KEY, link TEXT, hash_sum TEXT,
            movie_id INTEGER,
            FOREIGN KEY(movie_id) REFERENCES movie(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE migration(id INTEGER PRIMARY KEY, version INTEGER, date TEXT)",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE images(id INTEGER PRIMARY KEY, path TEXT, movie_id INTEGER,
            FOREIGN KEY(movie_id) REFERENCES movie(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE status(id INTEGER PRIMARY KEY, name TEXT)",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE queue(id INTEGER PRIMARY KEY, movie_id INTEGER, status_id INTEGER,
            FOREIGN KEY(movie_id) REFERENCES movie(id),
            FOREIGN KEY(status_id) REFERENCES status(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE config(id INTEGER PRIMARY KEY, key TEXT, value TEXT)",
        NO_PARAMS,
    )?;
    tx.commit()?;
    println!("new movie database created");
    Ok(())
}

pub fn migration() -> Result<()> {
    if Path::new("db.sqlite").exists() {
        println!("database exists, running movie migration");
        let mut conn = Connection::open("db.sqlite")?;
        migrate_movie(&mut conn)?;
        Ok(())
    } else {
        println!("Creating a new database");
        let mut conn = Connection::open("db.sqlite")?;
        create_movie_table(&mut conn)?;
        Ok(())
    }
}
