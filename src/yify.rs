use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Torrent {
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
    rating: i32,
    genre: Vec<String>,
    description_full: String,
    yt_trailer_code: String,
    medium_cover_image: String,
    torrents: Vec<Torrent>,
}

#[derive(Deserialize, Debug)]
struct Yify {
    status: String,
    data: Vec<Movies>,
}
