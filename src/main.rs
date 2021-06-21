use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::{env, fmt};

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

pub fn store_formation(formation: Vec<Unit>){
  for unit in &formation{
    store_sub_units(&unit);
  }
}

pub fn store_sub_units(unit: &Unit) {
  for sub_unit in &unit.sub_units{
    let _ = save_unit(&sub_unit);
    for person in &sub_unit.personnel{
      let _ = save_person(&person);
    }
    for sub_sub_unit in &sub_unit.sub_units{
      store_sub_units(&sub_sub_unit);
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PersonState {
  Ready,
  MIA,
  KIA,
  WIA,
  POW,
}

impl fmt::Display for PersonState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UnitType {
  Infantry,
  _Armoured,
  _Recon,
  _Support,
}

impl fmt::Display for UnitType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Rank {
  id: i32,
  level: i32,
  name: &'static str,
  title: &'static str,
  officer: bool,
  nationality: &'static str,
}

#[derive(Serialize, Debug, Clone)]
pub struct Person {
  id: i32,
  first_name: &'static str,
  last_name: &'static str,
  age: i32,
  nationality: &'static str,
  rank: i32,
  officer: bool,
  state: PersonState,
}

#[derive(Serialize, Debug, Clone)]
pub struct Unit {
  id: i32,
  parent_id: std::option::Option<i32>,
  name: &'static str,
  unit_type: UnitType,
  kills: i32,
  losses: i32,
  leader_id: i32,
  sub_units: Vec<Unit>,
  personnel: Vec<Person>,
}


fn main() {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  print!("\n---- Das Heer Überwacher ----\n           v{}  \n", VERSION);

  let database_init = establish_connection();
  if database_init.is_err() {
    println!("Unable to connect to database! - {:?}", database_init.unwrap_err());
  }

  // Initialise ranks
  let mut ranks = Vec::<Rank>::new();

  let hauptmann = Rank{
    id: 0,
    level: 2,
    name: "Hauptmann",
    title: "Hptm.",
    officer: true,
    nationality: "DE",
  };
  ranks.push(hauptmann);

  let leutnant = Rank{
    id: 1,
    level: 1,
    name: "Leunant",
    title: "Lt.",
    officer: true,
    nationality: "DE",
  };
  ranks.push(leutnant);

  for rank in &ranks {
    let _ = save_rank(&rank);
  }


  let mut formation = Vec::<Unit>::new();

  let huaptmann_mann = Person{
    id: 1,
    first_name: "Checkov",
    last_name: "Mann",
    age: 30,
    nationality: "DE",
    rank: hauptmann.level,
    officer: true,
    state: PersonState::Ready,
  };

  let mut company_hq = Unit{
    id: 0,
    parent_id: None,
    name: "First Company",
    unit_type: UnitType::Infantry,
    kills: 0,
    losses: 0,
    leader_id: huaptmann_mann.id,
    sub_units: Vec::<Unit>::new(),
    personnel: Vec::<Person>::new(),
  };

  company_hq.personnel.push(huaptmann_mann);

  let leutnant_keller = Person{
    id: 2,
    first_name: "Christoff",
    last_name: "Keller",
    age: 25,
    nationality: "DE",
    rank: leutnant.level,
    officer: true,
    state: PersonState::WIA,
  };

  let mut platoon_1 = Unit{
    id: 1,
    parent_id: Some(0),
    name: "First Platoon",
    unit_type: UnitType::Infantry,
    kills: 0,
    losses: 0,
    leader_id: leutnant_keller.id,
    sub_units: Vec::<Unit>::new(),
    personnel: Vec::<Person>::new(),
  };

  platoon_1.personnel.push(leutnant_keller);

  let leutnant_gaunt = Person{
    id: 3,
    first_name: "Thomas",
    last_name: "Gaunt",
    age: 23,
    nationality: "DE",
    rank: leutnant.level,
    officer: true,
    state: PersonState::Ready,
  };

  let mut platoon_2 = Unit{
    id: 2,
    parent_id: Some(0),
    name: "Second Platoon",
    unit_type: UnitType::Infantry,
    kills: 0,
    losses: 0,
    leader_id: leutnant_gaunt.id,
    sub_units: Vec::<Unit>::new(),
    personnel: Vec::<Person>::new(),
  };

  platoon_2.personnel.push(leutnant_gaunt);
  
  company_hq.sub_units.push(platoon_1);
  company_hq.sub_units.push(platoon_2);
  formation.push(company_hq);

  store_formation(formation);
}
