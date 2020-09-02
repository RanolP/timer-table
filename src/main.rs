#![windows_subsystem = "windows"]

mod app;
mod data;
mod font;
mod sound;

use app::{App, Flags};
use data::{Theme, Timetable};
use iced::{window, Application, Font, Settings};
use std::fs::File;

fn main() {
    pretty_env_logger::init();

    let timetable: Timetable = File::open("./config/timetable.json")
        .and_then(|file| serde_json::from_reader(file).map_err(Into::into))
        .expect("Can't read ./config/timetable.json as Timetable format.");

    let theme: Theme = File::open("./config/theme.json")
        .and_then(|file| serde_json::from_reader(file).map_err(Into::into))
        .expect("Can't read ./config/theme.json as Theme format.");

    App::run(Settings {
        default_font: match font::TEXT {
            Font::Default => None,
            Font::External { bytes, .. } => Some(bytes),
        },
        flags: Flags { timetable, theme },
        ..Default::default()
    });
}
