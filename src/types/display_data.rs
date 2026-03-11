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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::serde::json::serde_json;

    #[test]
    fn test_display_data_serialization() {
        let data = DisplayData {
            title: "Test Title".into(),
            subtitle: "Test Subtitle".into(),
            timezone: "UTC".into(),
            lines: vec![
                DisplayLine {
                    name: "Test Line".into(),
                    schedules: vec![
                        Schedule { date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), hour: 12, min: 0 }
                    ],
                    elapsed_flags: Some([true; 7]),
                }
            ],
            hijri: HijriInfo {
                start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                month: 1,
                year: 1445,
                names: [
                    "M1".into(), "M2".into(), "M3".into(), "M4".into(),
                    "M5".into(), "M6".into(), "M7".into(), "M8".into(),
                    "M9".into(), "M10".into(), "M11".into(), "M12".into(),
                ]
            },
            bangla: BanglaInfo {
                names: [
                    "B1".into(), "B2".into(), "B3".into(), "B4".into(),
                    "B5".into(), "B6".into(), "B7".into(), "B8".into(),
                    "B9".into(), "B10".into(), "B11".into(), "B12".into(),
                ]
            },
        };

        let serialized = serde_json::to_string(&data).unwrap();
        assert!(serialized.contains("\"title\":\"Test Title\""));
        assert!(serialized.contains("\"name\":\"Test Line\""));
    }
}




