#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    masjid_clock::stage()
}
