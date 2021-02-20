use super::movie;
use super::yify::Yify;
use reqwest;
use reqwest::Result as ReqResult;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn update() -> JoinHandle<()> {
    let handle = thread::spawn(|| loop {
        println!("Running in the background");

        if let Ok(movie_json) = fetch_movies() {
            match movie::create(&movie_json.data) {
                Ok(()) => println!("Movies inserted"),
                Err(error) => println!("Unable to insert movies, {}", error),
            }
        }
        thread::sleep(Duration::from_secs(20));
    });
    handle
}

fn fetch_movies() -> Result<Yify, Box<dyn Error>> {
    let url = reqwest::Url::parse_with_params(
        "https://yts.mx/api/v2/list_movies.json",
        &[("quality", "1080p")],
    )?;
    let movie_json: Yify = reqwest::blocking::get(url)?.json()?;
    Ok(movie_json)
}
