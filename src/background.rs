use super::yify::{Data, Yify};
use bytes::Bytes;
use chrono::prelude::{DateTime, Local};
use reqwest;
use reqwest::Result as ReqResult;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn update() -> JoinHandle<()> {
    let handle = thread::spawn(|| loop {
        println!("Running in the background");

        if let Ok(file) = File::open("movies.json") {
            let reader = BufReader::new(file);

            let res: Result<Yify, serde_json::Error> = serde_json::from_reader(reader);
            if let Ok(yify) = res {
                match get_movies(&yify.data) {
                    Ok(()) => println!("Movies inserted"),
                    Err(error) => println!("Unable to insert movies, {}", error),
                }
            }
        } else {
            println!("Unable to find movies.json");
        }
        thread::sleep(Duration::from_secs(5));
    });
    handle
}

fn get_movies(data: &Data) -> Result<()> {
    if let Ok(mut conn) = Connection::open("db.sqlite") {
        let current_date: DateTime<Local> = Local::now();
        for movie in data.movies.iter() {
            let genre: i32 = conn.query_row(
                "SELECT id from genre where name=?",
                &[&movie.genres[0]],
                |r| r.get(0),
            )?;
            println!("{}", genre);
            let tx = conn.transaction()?;
            tx.execute(
                "INSERT INTO movie(yify_id,genre_id,title,year,
                       imdb_url,rating,description,youtube_url,date_added) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![
                    movie.id,
                    genre,
                    movie.title,
                    movie.year,
                    movie.imdb_code,
                    movie.rating,
                    movie.description_full,
                    movie.yt_trailer_code,
                    current_date.date().to_string(),
                ],
            )?;
            let row_id = tx.last_insert_rowid();
            for torrent in movie.torrents.iter() {
                tx.execute(
                    "INSERT into torrent(movie_id, link,hash, quality) VALUES(?1,?2,?3,?4)",
                    params![row_id, torrent.url, torrent.hash, torrent.quality],
                )?;
            }
            if let Ok((bytes, extension)) = get_movie_image(&movie.medium_cover_image) {
                let mut file_name = String::from(&movie.title);
                file_name.push_str(&extension);
                let image_path = Path::new("images").join(&file_name);

                if let Ok(()) = save_movie_image(&image_path, &bytes) {
                    tx.execute(
                        "INSERT INTO image(movie_id,path) VALUES(?1,?2)",
                        params![row_id, image_path.to_str()],
                    )?;
                }
            }
            tx.commit()?;

            println!("Movie title is {}", movie.title);
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn save_movie_image(path: &PathBuf, image: &Bytes) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(image)?;
    Ok(())
}

fn get_movie_image(image_url: &String) -> ReqResult<(Bytes, String)> {
    let mut extension: String = ".jpeg".to_string();
    if image_url.ends_with(".png") {
        extension = ".png".to_string();
    } else if image_url.ends_with(".jpg") {
        extension = ".jpg".to_string();
    }
    let image_body = reqwest::blocking::get(image_url)?;
    let image_bytes = image_body.bytes()?;
    Ok((image_bytes, extension))
}

fn new_genre() -> Result<()> {
    Ok(())
}
