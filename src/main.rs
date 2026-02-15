mod utils;
mod ctrls;

use askama::Template;
use rocket::{fs::{FileServer, relative}, response::content::RawHtml};
use ctrls::pull;
use tracing_subscriber::{fmt, EnvFilter};

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

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}


#[launch]
fn rocket() -> _ {
    init_tracing();
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", pull::mount())
        .mount("/static", FileServer::from(relative!("static")))
}
