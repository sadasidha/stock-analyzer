mod utils;

use askama::Template;
use rocket::response::content::RawHtml;

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
}
