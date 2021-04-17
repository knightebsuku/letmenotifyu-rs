extern crate gtk;
use chrono::prelude::{DateTime, Local};
use gdk_pixbuf::Pixbuf;
use glib::clone;
use gtk::prelude::*;
use gtk::MessageType;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone)]
struct MovieDetail {
    dialog: gtk::Dialog,
    title: gtk::Label,
    rating: gtk::Label,
    year: gtk::Label,
    imdb: gtk::LinkButton,
    watch: gtk::Label,
    youtube: gtk::LinkButton,
    summary: gtk::TextBuffer,
    id: gtk::Label,
}

#[non_exhaustive]
struct Status;

impl Status {
    const NEW: i64 = 1;
    const DOWNLOADING: i64 = 2;
    const COMPLETE: i64 = 3;
    const FAIL: i64 = 4;
}

struct Movie {
    id: i64,
    rating: f64,
    year: i64,
    imdb: String,
    youtube: String,
    summary: String,
}

impl MovieDetail {
    fn new(builder: &gtk::Builder) -> MovieDetail {
        MovieDetail {
            dialog: builder.get_object("DetailDialog").unwrap(),
            title: builder.get_object("LblTitle").unwrap(),
            rating: builder.get_object("LblRating").unwrap(),
            year: builder.get_object("LblYear").unwrap(),
            imdb: builder.get_object("LBtnImdb").unwrap(),
            watch: builder.get_object("LblWatch").unwrap(),
            youtube: builder.get_object("LBtnYoutube").unwrap(),
            summary: builder.get_object("TextBufSummary").unwrap(),
            id: builder.get_object("LblID").unwrap(),
        }
    }
    fn show(&self) {
        self.dialog.show();
    }
    fn close(&self) {
        self.dialog.hide();
    }
    fn populate(&self, conn: &Connection, title: String) -> Result<()> {
        let movie_details = conn.query_row(
            "SELECT rating,year,imdb_url,youtube_url, description, id FROM movie WHERE title=?",
            &[&title],
            |row| {
                Ok(Movie {
                    rating: row.get(0)?,
                    year: row.get(1)?,
                    imdb: row.get(2)?,
                    youtube: row.get(3)?,
                    summary: row.get(4)?,
                    id: row.get(5)?,
                })
            },
        )?;
        //let detail = movie_details;
        self.title.set_text(&title);
        self.rating.set_text(&movie_details.rating.to_string());
        self.year.set_text(&movie_details.year.to_string());
        self.imdb.set_uri(&movie_details.imdb);
        self.youtube.set_uri(&movie_details.youtube);
        self.summary.set_text(&movie_details.summary);
        self.id.set_text(&movie_details.id.to_string());

        Ok(())
    }

    fn update_watch(&self, conn: &Connection) -> Result<()> {
        let movie_id = self.id.get_text().parse::<i64>().unwrap();
        conn.execute(
            "INSERT INTO queue(movie_id, status_id) VALUES(?,?)",
            params![movie_id, Status::NEW],
        )?;
        println!("Addede to watch list");
        self.watch.set_text("Yes");

        Ok(())
    }
}

fn ui_movie_detail(title: String) -> Result<()> {
    let file = include_str!("glade/MovieDetails.glade");
    let builder = gtk::Builder::from_string(file);
    let conn: Connection = Connection::open("db.sqlite")?;

    let detail = MovieDetail::new(&builder);
    match detail.populate(&conn, title) {
        Ok(_) => detail.show(),
        Err(err) => println!("Unable to fetch details: {}", err),
    }

    let btn_watch: gtk::Button = builder.get_object("BtnWatch").unwrap();
    btn_watch.connect_clicked(clone!(@strong detail => move |_| {
        detail.update_watch(&conn);
    }));

    let btn_close: gtk::Button = builder.get_object("BtnClose").unwrap();
    btn_close.connect_clicked(clone!(@strong detail => move |_| {
        detail.close();
    }));

    Ok(())
}
