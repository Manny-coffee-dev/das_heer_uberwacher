use serde::{Serialize, Deserialize};
use std::{fmt};

use crate::Person;

#[derive(Serialize, Debug, Clone)]
pub struct Unit {
  pub id: i32,
  pub parent_id: std::option::Option<i32>,
  pub name: &'static str,
  pub unit_type: UnitType,
  pub kills: i32,
  pub losses: i32,
  pub leader_id: i32,
  pub sub_units: Vec<Unit>,
  pub personnel: Vec<Person>,
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