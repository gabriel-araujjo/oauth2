use crate::DbConn;
use super::{Person, InsertablePerson, repository};
use rocket::request::Form;
use rocket_contrib::json::Json;

#[get("/<id>")]
fn get(id: i32, conn: DbConn) -> Option<Json<Person>> {
    repository::get(id, &conn).map(|person| Json(person)).ok()
}

#[post("/", data="<person>")]
fn post(person: Form<InsertablePerson>, conn: DbConn) -> Option<Json<i32>> {
    dbg!(repository::insert(person.into_inner(), &conn)).map(|id| Json(id)).ok()
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Option<Json<usize>> {
    repository::delete(id, &conn).map(|delations| Json(delations)).ok()
}

pub fn routes() -> std::vec::Vec<rocket::Route>{
    routes![get, post, delete]
}
