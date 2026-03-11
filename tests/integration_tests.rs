use rocket::local::blocking::Client;
use rocket::http::Status;
use masjid_clock;

#[test]
fn test_index_route() {
    let client = Client::tracked(masjid_clock::stage()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Note: Template rendering requires the template files to be accessible
}

#[test]
fn test_api_display_data_route() {
    let client = Client::tracked(masjid_clock::stage()).expect("valid rocket instance");
    let response = client.get("/api/display-data").dispatch();
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().unwrap();
    assert!(body.contains("Mirpur DOHS Central Masjid"));
}
