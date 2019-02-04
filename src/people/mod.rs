use serde_derive::{Serialize, Deserialize};
use crate::schema::people;

pub mod repository;
mod routes;

pub use self::routes::routes;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "people"]
pub struct InsertablePerson {
    name: String,
    email: String,
    password: String,
}
