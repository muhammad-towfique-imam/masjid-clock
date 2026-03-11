use crate::types::display_data::{BanglaInfo, DisplayData, DisplayLine, HijriInfo, Schedule};
use chrono::NaiveDate;
use rocket::serde::json::Json;

#[get("/display-data")]
pub fn get_display_data() -> Json<DisplayData> {
    Json(DisplayData {
        title: "Mirpur DOHS Central Masjid".into(),
        subtitle: "Dhaka, Bangladesh".into(),
        timezone: "Asia/Dhaka".into(),
        hijri: HijriInfo {
            start: NaiveDate::from_ymd_opt(2026, 2, 18).unwrap(),
            month: 9,
            year: 1447,
            names:[
                "Muharram".into(),
                "Safar".into(),
                "Rabiul Awl".into(),
                "Rabiul Akhr".into(),
                "Juma Ula".into(),
                "Juma Akhir".into(),
                "Rajab".into(),
                "Shaban".into(),
                "Ramadan".into(),
                "Shawwal".into(),
                "Zul-qaadh".into(),
                "Zul-hijjah".into()
            ]
        },
        bangla: BanglaInfo {
            names:[
                "Baishakh".into(),
                "Joishtha".into(),
                "Asharh".into(),
                "Shrabon".into(),
                "Bhadro".into(),
                "Ashshin".into(),
                "Kartik".into(),
                "Augrohayon".into(),
                "Poush".into(),
                "Magh".into(),
                "Falgun".into(),
                "Choitro".into()
            ]
        },
        lines: vec![
            DisplayLine {
                name: "Fazr".into(),
                elapsed_flags: Some([true; 7]),
                schedules: vec![
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(), hour: 5, min: 26 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(), hour: 5, min: 25 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 3).unwrap(), hour: 5, min: 23 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 6).unwrap(), hour: 5, min: 20 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 9).unwrap(), hour: 5, min: 17 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 12).unwrap(), hour: 5, min: 15 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(), hour: 5, min: 12 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 18).unwrap(), hour: 5, min: 9 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 21).unwrap(), hour: 5, min: 20 },
                ]
            },
            DisplayLine {
                name: "Zuhr".into(),
                elapsed_flags: Some([true; 7]),
                schedules: vec![
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(), hour: 13, min: 15 },
                ]
            },
            DisplayLine {
                name: "Asr".into(),
                elapsed_flags: Some([true; 7]),
                schedules: vec![
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(), hour: 16, min: 45 },
                ]
            },
            DisplayLine {
                name: "Magrib".into(),
                elapsed_flags: Some([true; 7]),
                schedules: vec![
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(), hour: 18, min: 22 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(), hour: 18, min: 23 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 3).unwrap(), hour: 18, min: 24 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 6).unwrap(), hour: 18, min: 25 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 9).unwrap(), hour: 18, min: 27 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 12).unwrap(), hour: 18, min: 28 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(), hour: 18, min: 29 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 18).unwrap(), hour: 18, min: 30 },
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 3, 21).unwrap(), hour: 18, min: 21 },
                ]
            },
            DisplayLine {
                name: "Isha".into(),
                elapsed_flags: Some([true; 7]),
                schedules: vec![
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(), hour: 20, min: 0 },
                ]
            },
            DisplayLine {
                name: "Jumma".into(),
                elapsed_flags: Some([false, false, false, false, false, true, false]),
                schedules: vec![
                    Schedule { date: NaiveDate::from_ymd_opt(2026, 2, 26).unwrap(), hour: 13, min: 0 },
                ]
            },
        ],
    })
}
