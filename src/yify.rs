use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Torrents {
    pub url: String,
    pub hash: String,
    pub quality: String,
}

#[derive(Deserialize, Debug)]
pub struct Movies {
    pub id: i32,
    pub url: String,
    pub imdb_code: String,
    pub title: String,
    pub year: i32,
    pub rating: f64,
    pub genres: Vec<String>,
    pub description_full: String,
    pub yt_trailer_code: String,
    pub medium_cover_image: String,
    pub torrents: Vec<Torrents>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub movie_count: i32,
    pub movies: Vec<Movies>,
}

#[derive(Deserialize, Debug)]
pub struct Yify {
    pub status: String,
    pub data: Data,
}
