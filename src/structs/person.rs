use serde::{Serialize, Deserialize};
use std::{fmt};

#[derive(Serialize, Debug, Clone)]
pub struct Person {
  pub id: i32,
  pub first_name: &'static str,
  pub last_name: &'static str,
  pub age: i32,
  pub nationality: &'static str,
  pub rank: i32,
  pub officer: bool,
  pub state: PersonState,
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