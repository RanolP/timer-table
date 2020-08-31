use chrono::prelude::*;
use palette::Srgba;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lecture {
    pub begin: NaiveTime,
    pub end: NaiveTime,
    pub subject: String,
}

pub type DayTimetable = Vec<Lecture>;

pub type Timetable = [DayTimetable; 5];

#[derive(Debug, Serialize, Deserialize)]
pub struct CellColor {
    pub background: Srgba,
    pub foreground: Srgba,
}

pub type Theme = HashMap<String, CellColor>;
