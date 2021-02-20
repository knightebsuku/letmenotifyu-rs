use super::yify::{Data, Movie, Torrents};
use bytes::Bytes;
use chrono::prelude::{DateTime, Local};
use reqwest;
use reqwest::Result as ReqResult;
use rusqlite::{params, Connection, Result, Transaction};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const JPG: &str = ".jpg";
const PNG: &str = ".png";
const JPEG: &str = ".jpeg";

fn detail(tx: &Transaction, data: &Movie, genre_id: i64, date: DateTime<Local>) -> Result<i64> {
    tx.execute(
                "INSERT INTO movie(yify_id,genre_id,title,year,
                       imdb_url,rating,description,youtube_url,date_added) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![
                    data.id,
                    genre_id,
                    data.title,
                    data.year,
                    data.imdb_code,
                    data.rating,
                    data.description_full,
                    data.yt_trailer_code,
                    date.date().to_string(),
                ],
            )?;
    let row_id = tx.last_insert_rowid();
    Ok(row_id)
}
fn torrent(row_id: &i64, tx: &Transaction, torrents: &Vec<Torrents>) -> Result<()> {
    for t in torrents.iter() {
        tx.execute(
            "INSERT into torrent(movie_id, link,hash, quality) VALUES(?1,?2,?3,?4)",
            params![row_id, t.url, t.hash, t.quality],
        )?;
    }
    Ok(())
}

fn get_genre(conn: &Connection, name: &String) -> Result<i64> {
    if let Ok(id) = conn.query_row("SELECT id from genre where name=?", params![name], |r| {
        r.get(0)
    }) {
        Ok(id)
    } else {
        conn.execute("INSERT INTO genre(name) VALUES(1?)", params![name])?;
        let row_id = conn.last_insert_rowid();
        Ok(row_id)
    }
}

fn fetch_image(image_url: &String) -> ReqResult<(Bytes, &str)> {
    let mut extension = JPEG;
    if image_url.ends_with(".png") {
        extension = PNG;
    } else if image_url.ends_with(".jpg") {
        extension = JPG;
    }
    let image_bytes = reqwest::blocking::get(image_url)?.bytes()?;

    Ok((image_bytes, extension))
}

fn image(
    tx: &Transaction,
    row_id: &i64,
    title: &String,
    image: &Bytes,
    extension: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file_name = String::from(title);
    file_name.push_str(extension);
    let image_path = Path::new("images").join(&file_name);

    let mut file = File::create(&image_path)?;
    file.write_all(image)?;

    tx.execute(
        "INSERT INTO image(movie_id,path) VALUES(?1,?2)",
        params![row_id, image_path.to_str()],
    )?;
    Ok(())
}

pub fn create(data: &Data) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("db.sqlite")?;
    let current_date: DateTime<Local> = Local::now();
    for movie in data.movies.iter() {
        println!("Working on {}", &movie.title);
        let genre_id: i64 = get_genre(&conn, &movie.genres[0])?;
        let tx = conn.transaction()?;
        if let Ok(row_id) = detail(&tx, &movie, genre_id, current_date) {
            torrent(&row_id, &tx, &movie.torrents)?;
            let (image_bytes, extension) = fetch_image(&movie.medium_cover_image)?;
            image(&tx, &row_id, &movie.title, &image_bytes, &extension)?;
            tx.commit()?;
        } else {
            println!("MOvie already exists");
        }
    }
    Ok(())
}
