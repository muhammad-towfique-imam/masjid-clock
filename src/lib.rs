#[macro_use]
extern crate rocket;

pub mod controller;
pub mod types;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};
use controller::display_controller::get_display_data;

#[get("/")]
pub fn index() -> Template {
    Template::render("index",
        context! {
            title: "Masjid Clock",
        },
    )
}

pub fn stage() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![get_display_data])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
