use serde::Serialize;
use chrono::{NaiveDate};

#[derive(Serialize)]
pub struct DisplayData {
    pub title: String,
    pub subtitle: String,
    pub timezone: String,
    pub lines: Vec<DisplayLine>,
    pub hijri: HijriInfo,
    pub bangla: BanglaInfo,
}


#[derive(Serialize)]
pub struct DisplayLine {
    pub name: String,
    pub schedules: Vec<Schedule>,
    pub elapsed_flags: Option<[bool; 7]>
}


#[derive(Serialize)]
pub struct HijriInfo {
    pub start: NaiveDate,
    pub month: u8,
    pub year: u16,
    pub names: [String; 12]
}

#[derive(Serialize)]
pub struct BanglaInfo {
    pub names: [String; 12]
}

#[derive(Serialize)]
pub struct Schedule {
    pub date: NaiveDate,
    pub hour: u8,
    pub min: u8,
}




