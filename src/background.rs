use rusqlite::{Connection, Result, NO_PARAMS};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Torrents {
    url: String,
    hash: String,
    quality: String,
}

#[derive(Deserialize, Debug)]
struct Movies {
    id: i32,
    url: String,
    imdb_code: String,
    title: String,
    year: i32,
    rating: f32,
    genres: Vec<String>,
    description_full: String,
    yt_trailer_code: String,
    medium_cover_image: String,
    torrents: Vec<Torrents>,
}

#[derive(Deserialize, Debug)]
struct Data {
    movie_count: i32,
    movies: Vec<Movies>,
}

#[derive(Deserialize, Debug)]
struct Yify {
    status: String,
    data: Data,
}

pub fn update() -> JoinHandle<()> {
    let handle = thread::spawn(|| loop {
        println!("Running in the background");

        if let Ok(file) = File::open("movies.json") {
            let reader = BufReader::new(file);

            let res: Result<Yify, serde_json::Error> = serde_json::from_reader(reader);
            if let Ok(yify) = res {
                //get_movies(&yify.data);
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
        for movie in data.movies.iter() {
            let genre: i32 = conn.query_row(
                "SELECT id from genre where name=?",
                &[&movie.genres[0]],
                |r| r.get(0),
            )?;
            println!("{}", genre);
            // let tx = conn.transaction();
            // tx.execute(
            //     "INSERT INTO movie(yify_id,genre_id,title,year,
            //            imdb_url,rating,description_full,youtube_url,date_added) values(?,?,?,?,?,?,?,?,?)",
            //     &[
            //         &movie.id,
            //         &genre,
            //         &movie.title,
            //         &movie.year,
            //         &movie.imdb_code,
            //         &movie.rating,
            //         &movie.description_full,
            //         &movie.yt_trailer_code,
            //     ],
            // )?;
            // let row_id = tx.last_insert_rowid();
            // for torrent in movie.torrents.iter() {
            //     tx.execute(
            //         "INSERT into torrent(movie_id, link,hash_sum, quality)",
            //         &[&row_id, &torrent.url, &torrent.hash, &torrent.quality],
            //     )?;
            // }
            // tx.commit();

            println!("Movie title is {}", movie.title)
        }
    }
}

fn get_movie_image() -> Result<()> {
    Ok(())
}
