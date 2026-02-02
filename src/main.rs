mod utils;
mod ctrls;

use askama::Template;
use rocket::{fs::{FileServer, relative}, response::content::RawHtml};
use ctrls::pull;

#[macro_use]
extern crate rocket;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
fn index() -> RawHtml<String> {
    let html = IndexTemplate.render().unwrap();
    RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", pull::mount())
        .mount("/static", FileServer::from(relative!("static")))
}
