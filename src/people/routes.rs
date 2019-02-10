use crate::DbConn;
use super::{Person, InsertablePerson, repository};
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/<id>")]
fn get(id: Uuid, conn: DbConn) -> Option<Json<Person>> {
    repository::get(id.into_inner(), &conn).map(|person| Json(person)).ok()
}

#[post("/", data="<person>")]
fn post(person: Form<InsertablePerson>, conn: DbConn) -> Option<Json<i32>> {
    dbg!(repository::insert(person.into_inner(), &conn)).map(|id| Json(id)).ok()
}

#[delete("/<id>")]
fn delete(id: Uuid, conn: DbConn) -> Option<Json<usize>> {
    repository::delete(id.into_inner(), &conn).map(|delations| Json(delations)).ok()
}

pub fn routes() -> std::vec::Vec<rocket::Route>{
    routes![get, post, delete]
}
