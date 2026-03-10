#[macro_use]
extern crate rocket;

mod routes;
mod types;

use rocket_dyn_templates::{Template, context};
use routes::display_routes::get_display_data;

#[get("/")]
fn hello() -> Template {
    Template::render(
        "index",
        context! {
            title: "Welcome",
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/api", routes![get_display_data])
        .attach(Template::fairing())
}
