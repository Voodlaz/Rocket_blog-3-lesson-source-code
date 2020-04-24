#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn homepage() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("homepage", &context)
}

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/", routes![homepage])
    .launch();
}
