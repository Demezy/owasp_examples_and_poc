use rocket::{fs::FileServer, response::content::RawHtml};

mod routes;
mod service;

#[macro_use]
extern crate rocket;

// this is our get route hich will be requested at the "/" location wherever it is mounted
#[get("/")]
async fn say_hello() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <h1>Welcome to ssrf service.</h1>
        <p>Rust based service for public image board! Absolutely secure</p>
    <a href="/gallery">gallery</a>
    "#,
    )
}

// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
fn rocket() -> _ {
    rocket::build()
        // .configure(rocket::config::Config::figment().join(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
        .register("/", catchers![routes::not_found])
        .mount("/", routes![say_hello, routes::gallery])
        .mount("/api", routes![routes::photo_upload])
        .mount("/content", FileServer::from("content"))
        .mount("/static", FileServer::from("static"))
}
