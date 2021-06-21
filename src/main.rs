mod structs;
use crate::structs::rank::{Rank};
use crate::structs::person::{Person, PersonState};
use crate::structs::unit::{Unit, UnitType};

mod utils;
use utils::database;
use crate::database::{establish_connection, save_unit, save_rank, save_person};

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

  // Create formation
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

  // Save formation
  store_formation(formation);
}
