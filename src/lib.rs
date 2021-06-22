#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Rank, NewRank, Unit, NewUnit, Person, NewPerson};

pub fn create_rank<'a>(conn: &PgConnection, level: i32, name: &'a str, title: &'a str, nationality: &'a str) -> Rank {
    use schema::ranks;

    let new_rank = NewRank {
        level,
        name,
        title,
        nationality,
      };

    diesel::insert_into(ranks::table)
        .values(&new_rank)
        .get_result(conn)
        .expect("Error saving new rank")
}

pub fn create_person<'a>(conn: &PgConnection, unit_id: std::option::Option<i32>, first_name: &'a str, last_name: &'a str, age: i32, nationality: &'a str, rank: i32, state:  &'a str) -> Person {
    use schema::personnel;

    let new_person = NewPerson {
        unit_id,
        first_name,
        last_name,
        age,
        nationality,
        rank,
        state,
      };

    diesel::insert_into(personnel::table)
        .values(&new_person)
        .get_result(conn)
        .expect("Error saving new person")
}

pub fn create_unit<'a>(conn: &PgConnection, parent_id: std::option::Option<i32>, name: &'a str, unit_type: &'a str, kills: i32, losses: i32, leader_id: std::option::Option<i32>) -> Unit {
    use schema::units;

    let new_unit = NewUnit {
        parent_id,
        name,
        unit_type,
        kills,
        losses,
        leader_id,
      };

    diesel::insert_into(units::table)
        .values(&new_unit)
        .get_result(conn)
        .expect("Error saving new unit")
}