use crate::types::display_data::DisplayData;
use rocket::serde::json::Json;

#[get("/display-data")]
pub fn get_display_data() -> Json<DisplayData> {
    Json(DisplayData{
        title: "Mirpur DOHS Central Masjid".into(),
        subtitle: "Dhaka, Bangladesh".into()
    })
}
