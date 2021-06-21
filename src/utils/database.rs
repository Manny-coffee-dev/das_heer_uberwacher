use postgres::{Client, Error, NoTls};
use dotenv::dotenv;
use std::{env};

use crate::Rank;
use crate::Person;
use crate::Unit;

pub fn establish_connection() -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&database_url, NoTls)?;

    // Create units table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS units (
            id          INTEGER PRIMARY KEY,
           parent_id   INTEGER,
            name        VARCHAR NOT NULL,
            unit_type   VARCHAR NOT NULL,
            kills       INTEGER NOT NULL,
            losses      INTEGER NOT NULL,
            leader_id   INTEGER NOT NULL);
    ",
    )?;

    // Create personnel table
    client.batch_execute(
      "
      CREATE TABLE IF NOT EXISTS personnel (
          id          INTEGER PRIMARY KEY,
          first_name  VARCHAR NOT NULL,
          last_name   VARCHAR NOT NULL,
          age         INTEGER NOT NULL,
          nationality VARCHAR NOT NULL,
          rank        INTEGER NOT NULL,
          officer     BOOLEAN NOT NULL,
          state       VARCHAR NOT NULL);
    ",
    )?;

    // Create ranks table
    client.batch_execute(
      "
      CREATE TABLE IF NOT EXISTS ranks (
          id          SERIAL PRIMARY KEY,
          level       INTEGER NOT NULL,
          name        VARCHAR NOT NULL,
          title       VARCHAR NOT NULL,
          officer     BOOLEAN NOT NULL,
          nationality VARCHAR NOT NULL);
    ",
    )?;
  Ok(())
}

pub fn save_unit(unit: &Unit) -> Result<(), Error> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut client = Client::connect(&database_url, NoTls)?;
  client.execute(
      "INSERT INTO units (id, parent_id, name, unit_type, kills, losses, leader_id) VALUES ($1, $2, $3, $4, $5, $6, $7);",
      &[
        &unit.id, 
        &unit.parent_id,
        &unit.name,
        &unit.unit_type.to_string(),
        &unit.kills,
        &unit.losses,
        &unit.leader_id,
      ],
  )?;
  Ok(())
}

pub fn save_person(person: &Person) -> Result<(), Error> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut client = Client::connect(&database_url, NoTls)?;
  client.execute(
      "INSERT INTO personnel (id, first_name, last_name, age, nationality, rank, officer, state) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
      &[
        &person.id, 
        &person.first_name,
        &person.last_name,
        &person.age,
        &person.nationality,
        &person.rank,
        &person.officer,
        &person.state.to_string(),
      ],
  )?;
  Ok(())
}

pub fn save_rank(rank: &Rank) -> Result<(), Error> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut client = Client::connect(&database_url, NoTls)?;
  client.execute(
      "INSERT INTO ranks (id, level, name, title, officer, nationality) VALUES ($1, $2, $3, $4, $5, $6);",
      &[
        &rank.id, 
        &rank.level,
        &rank.name,
        &rank.title,
        &rank.officer,
        &rank.nationality,
      ],
  )?;
  Ok(())
}