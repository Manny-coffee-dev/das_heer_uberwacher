use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::{env, fmt};
use std::fs::File;
use std::io::Write;

pub fn establish_connection() -> Result<(), Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&database_url, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS units (
            id          INTEGER PRIMARY KEY,
            parent_id   INTEGER,
            name        VARCHAR NOT NULL,
            unit_type   VARCHAR NOT NULL,
            kills       INTEGER NOT NULL,
            losses      INTEGER NOT NULL,
            leader      VARCHAR NOT NULL);
    ",
    )?;
    Ok(())
}

pub fn save_unit(unit: &Unit) -> Result<(), Error> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut client = Client::connect(&database_url, NoTls)?;
  client.execute(
      "INSERT INTO units (id, parent_id, name, unit_type, kills, losses, leader) VALUES ($1, $2, $3, $4, $5, $6, $7);",
      &[
        &unit.id, 
        &unit.parent_id,
        &unit.name,
        &unit.unit_type.to_string(),
        &unit.kills,
        &unit.losses,
        &unit.leader,
      ],
  )?;
  Ok(())
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


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Unit {
  id: i32,
  parent_id: std::option::Option<i32>,
  name: &'static str,
  unit_type: UnitType,
  kills: i32,
  losses: i32,
  leader: &'static str,
}


fn main() {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  print!("\n---- Das Heer Überwacher ----\n           v{}  \n", VERSION);

  let database_init = establish_connection();
  if database_init.is_err() {
    println!("Unable to connect to database! - {:?}", database_init.unwrap_err());
  }

  let mut formation = Vec::<Unit>::new();

  let company_hq = Unit{
    id: 0,
    parent_id: None,
    name: "First Company",
    unit_type: UnitType::Infantry,
    kills: 0,
    losses: 0,
    leader: "Hauptmann Mann",
  };

  formation.push(company_hq);

  let platoon_1 = Unit{
    id: 1,
    parent_id: Some(company_hq.id),
    name: "First Platoon",
    unit_type: UnitType::Infantry,
    kills: 0,
    losses: 0,
    leader: "Leutnant Keller",
  };

  formation.push(platoon_1);

  let platoon_2 = Unit{
    id: 2,
    parent_id: Some(company_hq.id),
    name: "Second Platoon",
    unit_type: UnitType::Infantry,
    kills: 0,
    losses: 0,
    leader: "Leutnant Gaunt",
  };

  formation.push(platoon_2);

  for unit in &formation {
    let saved_unit = save_unit(&unit);
    println!("{:?}", saved_unit);
  }

  println!("{:?}", serde_json::to_string(&formation).unwrap());

  // Create a temporary file.
  let temp_directory = env::current_dir();
  let temp_file = temp_directory.ok().unwrap().join("heer.json");

  // Open a file in write-only (ignoring errors).
  // This creates the file if it does not exist (and empty the file if it exists).
  let mut file = File::create(temp_file).unwrap();

  // Write a &str in the file (ignoring the result).
  writeln!(&mut file, "{}", serde_json::to_string(&formation).unwrap()).unwrap();

}
