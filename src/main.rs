#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod oauth2;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/authorize")]
fn authorize(request: oauth2::AuthRequest) -> String {
    "OK".to_string()
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}
