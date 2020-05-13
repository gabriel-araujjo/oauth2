use crate::schema::clients;
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

pub mod errors;
pub mod repository;
mod routes;

pub use self::routes::routes;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Insertable, FromForm)]
#[table_name = "people"]
pub struct InsertablePerson {
    name: String,
    email: String,
    #[column_name = "hash"]
    password: String,
}
