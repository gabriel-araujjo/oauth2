use super::{Person, InsertablePerson};
use crate::schema::people;
use diesel;
use diesel::debug_query;
use diesel::prelude::*;

const fn visible_columns() -> (people::columns::id, people::columns::name, people::columns::email) {
    use self::people::dsl::*;
    (id, name, email)
}

pub fn get(id: i32, conn: &PgConnection) -> QueryResult<Person> {
    people::table.select(visible_columns()).filter(people::columns::id.eq(id)).first(conn)
}

pub fn insert(person: InsertablePerson, conn: &PgConnection) -> QueryResult<i32> {
    let query = diesel::insert_into(people::table)
        .values(person)
        .returning(people::columns::id);

    dbg!(debug_query::<diesel::pg::Pg, _>(&query).to_string());

    query.get_result(conn)
}

pub fn delete(id: i32, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(people::table.find(id))
        .execute(conn)
}

// fn get_by_email_checking_password(email: &str, password: &str) -> QueryResult<Person> {
    
// }
