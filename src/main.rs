#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate rocket;
extern crate jsonwebtoken;

use dotenv::dotenv;
use rocket_contrib::database;

mod schema;
mod auth;
mod people;
mod clients;
mod oauth2;
// mod oauth2;

#[database("oauth_db")]
pub struct DbConn(diesel::PgConnection);

// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }

// #[get("/authorize?<state>&<auth_form..>")]
// fn authorize(state: Option<&RawStr>, auth_form: Form<oauth2::AuthForm>) -> Result<JsonValue, OAuthErr> {
    
//     match auth_form.response_type {
//         Code => {

//         },
//         _ => 
//     }
//     "OK".to_string()
// }

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/people", people::routes()).launch();
}
