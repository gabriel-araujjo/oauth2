use super::{Person, InsertablePerson};
use super::errors::*;
use crate::schema::people;
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel;
use diesel::debug_query;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get(id: Uuid, conn: &PgConnection) -> PeopleResult<Person> {
    Ok(people::table.find(id).first(conn)?)
}

pub fn insert(mut person: InsertablePerson, conn: &PgConnection) -> PeopleResult<Uuid> {
    person.password = hash(person.password.to_owned(), DEFAULT_COST)?;
    let query = diesel::insert_into(people::table)
        .values(person)
        .returning(people::columns::id);

    dbg!(debug_query::<diesel::pg::Pg, _>(&query).to_string());

    Ok(query.get_result(conn)?)
}

pub fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(people::table.find(id))
        .execute(conn)
}

pub fn get_by_email_checking_password(email: String, password: String, conn: &PgConnection) -> PeopleResult<Person> {
    match people::table.filter(people::columns::email.eq(email)).first::<Person>(conn) {
        Ok(person) => {
            match verify(password, person.hash.as_str()) {
                Ok(true) => Ok(person),
                _ => Err(PeopleError::AuthenticationFail),
            }
        },
        _ => Err(PeopleError::AuthenticationFail),
    }   
}
