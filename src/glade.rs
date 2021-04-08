extern crate gtk;
use glib::clone;
use glib::clone::Downgrade;
use gtk::prelude::*;
use gtk::MessageType;
use rusqlite::{params, Connection, Result, NO_PARAMS};

pub fn build_ui(builder: &gtk::Builder) -> Result<()> {
    let about: gtk::AboutDialog = builder.get_object("AboutWindow").unwrap();
    let btn_about_help: gtk::Button = builder.get_object("ButtonHelp").unwrap();
    btn_about_help.connect_clicked(move |_| {
        about.show();
    });

    let btn_pref: gtk::Button = builder.get_object("BtnPreferences").unwrap();
    btn_pref.connect_clicked(move |_| {
        println!("Pref Clicked");
        ui_preferences();
    });
    Ok(())
}

#[derive(Debug, Clone)]
struct UiPreference {
    dialog: gtk::Dialog,
    ent_interval: gtk::Entry,
    ent_host: gtk::Entry,
    ent_port: gtk::Entry,
    ent_quality: gtk::Entry,
    msg_dialog: gtk::MessageDialog,
}

impl UiPreference {
    fn new(builder: &gtk::Builder) -> UiPreference {
        UiPreference {
            dialog: builder.get_object("PrefDialog").unwrap(),
            ent_interval: builder.get_object("EntryInterval").unwrap(),
            ent_host: builder.get_object("EntryHost").unwrap(),
            ent_port: builder.get_object("EntryPort").unwrap(),
            ent_quality: builder.get_object("EntryQuality").unwrap(),
            msg_dialog: builder.get_object("MsgDialog").unwrap(),
        }
    }
    fn get_values(&self, conn: &Connection) -> Result<()> {
        let interval: String = conn.query_row(
            "SELECT value from config where key='interval'",
            NO_PARAMS,
            |row| row.get(0),
        )?;
        let host: String = conn.query_row(
            "SELECT value from config where key='host'",
            NO_PARAMS,
            |row| row.get(0),
        )?;
        let port: String = conn.query_row(
            "SELECT value from config where key='port'",
            NO_PARAMS,
            |row| row.get(0),
        )?;
        let quality: String = conn.query_row(
            "SELECT value from config where key='quality'",
            NO_PARAMS,
            |row| row.get(0),
        )?;
        self.ent_interval.set_text(&interval);
        self.ent_host.set_text(&host);
        self.ent_port.set_text(&port);
        self.ent_quality.set_text(&quality);
        Ok(())
    }

    fn show(&self) {
        self.dialog.show();
    }
    fn close(&self) {
        self.dialog.hide();
    }
    fn save_values(&self, conn: &Connection) -> Result<()> {
        println!("Save Clicked");
        let interval = self.ent_interval.get_text();
        let host = self.ent_host.get_text();
        let port = self.ent_port.get_text();
        let quality = self.ent_quality.get_text();

        conn.execute(
            "UPDATE config set value=? where key='interval'",
            params![interval.as_str()],
        )?;
        conn.execute(
            "UPDATE config set value=? where key='quality'",
            params![quality.as_str()],
        )?;
        conn.execute(
            "UPDATE config set value=? where key='host'",
            params![host.as_str()],
        )?;
        conn.execute(
            "UPDATE config set value=? where key='port'",
            params![port.as_str()],
        )?;
        Ok(())
    }
    fn error_message(&self, message: MessageType, error: Option<&str>) {
        let info = Some("Preferences Updated");

        match message {
            gtk::MessageType::Info => {
                self.msg_dialog.set_property_text(info);
            }
            gtk::MessageType::Error => {
                self.msg_dialog.set_property_text(error);
            }
            _ => {
                self.msg_dialog.set_property_text(info);
            }
        }
        self.msg_dialog.set_property_message_type(message);
        self.msg_dialog.show();
    }
    fn message_close(&self) {
        self.msg_dialog.close();
    }
}

// Create the Prefencecs menu
fn ui_preferences() -> Result<()> {
    let file = include_str!("ui/Preferences.glade");
    let builder = gtk::Builder::from_string(file);
    let conn: Connection = Connection::open("db.sqlite")?;

    let pref = UiPreference::new(&builder);
    match pref.get_values(&conn) {
        Err(err) => {
            pref.error_message(MessageType::Error, Some("Unable to fetch values"));
            println!("Error Log here");
        }
        _ => println!("OK"),
    }
    pref.show();

    let btn_close_pref: gtk::Button = builder.get_object("BtnClose").unwrap();
    btn_close_pref.connect_clicked(clone!(@strong pref => move |_| {
        pref.close();
    }));
    let btn_save: gtk::Button = builder.get_object("BtnSave").unwrap();
    btn_save.connect_clicked(clone!(@strong pref => move |_| {
        match pref.save_values(&conn){
            Ok(_) => {
                 pref.error_message(gtk::MessageType::Info, None);
            },
            Err(err) => {
                pref.error_message(MessageType::Error, Some("Unable to save preferences"));
                println!("Error message {}", err);
            }
        }
        pref.close();
    }));
    let btn_message: gtk::Button = builder.get_object("BtnMessageClose").unwrap();
    btn_message.connect_clicked(clone!(@strong pref => move |_| {
        pref.message_close();
    }));
    Ok(())
}
