use askama::Template;
use rocket::{get, launch, response::Redirect, routes};

mod utils;

#[derive(Template)]
#[template(path = "index.html")]  // relative to templates/ folder
struct IndexTemplate {}

#[get("/")]
pub fn index() {
    Redirect::to("/index.html");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
