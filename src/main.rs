#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::request::Form;

mod oauth2;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/authorize?<auth_form..>")]
fn authorize(auth_form: Form<oauth2::AuthForm>) -> String {
    match auth_form.response_type {
        Code => 
    }
    
    "OK".to_string()
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}
