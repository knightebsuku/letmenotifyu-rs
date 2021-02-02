use rusqlite::{Connection, Result, NO_PARAMS};
use std::path::Path;

fn movie_changes() -> Vec<(i32, &'static str)> {
    let changes = vec![
        (2, "INSERT INTO genre(name) VALUES('Animation')"),
        (3, "INSERT INTO genre(name) VALUES('Comedy')"),
        (4, "INSERT INTO genre(name) VALUES('Action')"),
        (5, "INSERT INTO genre(name) VALUES('Adventure')"),
        (6, "INSERT INTO genre(name) VALUES('Drama')"),
        (7, "INSERT INTO genre(name) VALUES('Horror')"),
        (8, "INSERT INTO genre(name) VALUES('Sci-Fi')"),
        (9, "INSERT INTO genre(name) VALUES('Musical')"),
        (10, "INSERT INTO genre(name) VALUES('Crime')"),
        (11, "INSERT INTO genre(name) VALUES('Fantasy')"),
        (12, "INSERT INTO genre(name) VALUES('Documentary')"),
        (13, "INSERT INTO genre(name) VALUES('Biography')"),
        (14, "INSERT INTO genre(name) VALUES('Sport')"),
        (15, "INSERT INTO genre(name) VALUES('Thriller')"),
        (16, "INSERT INTO genre(name) VALUES('Romance')"),
        (17, "INSERT INTO genre(name) VALUES('Mystery')"),
        (18, "INSERT INTO genre(name) VALUES('Western')"),
        (19, "INSERT INTO genre(name) VALUES('Family')"),
        (20, "INSERT INTO genre(name) VALUES('Music')"),
        (21, "INSERT INTO genre(name) VALUES('War')"),
        (22, "INSERT INTO genre(name) VALUES('History')"),
        (23, "INSERT INTO genre(name) VALUES('Reality-TV')"),
        (24, "INSERT INTO genre(name) VALUES('Film-Noir')"),
        (25, "INSERT INTO genre(name) VALUES('Talk-Show')"),
        (26, "INSERT INTO genre(name) VALUES('News')"),
        (27, "INSERT INTO status(name) VALUES('New')"),
        (28, "INSERT INTO status(name) VALUES('Downloading')"),
        (29, "INSERT INTO status(name) VALUES('Complete')"),
        (30, "INSERT INTO status(name) VALUES('Error')"),
    ];
    changes
}

fn migrate_movie(conn: &mut Connection) -> Result<()> {
    println!("Running the movie migration");

    let mut version: i32 =
        conn.query_row("SELECT max(version) from migration", NO_PARAMS, |r| {
            r.get(0)
        })?;
    println!("The version latest version is {:?}", version);
    for change in movie_changes() {
        if change.0 > version {
            conn.execute(&change.1, NO_PARAMS)?;
            version = change.0;
            conn.execute("INSERT INTO migration(version) VALUES(?)", &[&version])?;
            println!("migration {} applied", change.0);
        } else {
            println!(
                "new migration version {} is small than latest version {}....not applying",
                change.0, version
            );
        }
    }
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
            title TEXT, date_added TEXT, yify_id INTEGER,genre_id INTEGER,rating REAL,
            youtube_url TEXT, imdb_url TEXT,description TEXT,year INTEGER,
           FOREIGN KEY(genre_id) REFERENCES genre(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE torrent(id INTEGER PRIMARY KEY, link TEXT, hashTEXT,
            movie_id INTEGER,quality TEXT,
            FOREIGN KEY(movie_id) REFERENCES movie(id))",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE migration(id INTEGER PRIMARY KEY, version INTEGER)",
        NO_PARAMS,
    )?;
    tx.execute(
        "CREATE TABLE image(id INTEGER PRIMARY KEY, path TEXT, movie_id INTEGER,
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
    tx.execute("INSERT INTO migration(version) values(1)", NO_PARAMS)?;
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
