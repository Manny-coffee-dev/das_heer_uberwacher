use serde::{Serialize};

use super::schema::{personnel, units, ranks};

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct Person {
  pub id: i32,
  pub unit_id: std::option::Option<i32>,
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
  pub nationality: String,
  pub rank: i32,
  pub state: String,
}

#[derive(Insertable)]
#[table_name="personnel"]
pub struct NewPerson<'a> {
  pub unit_id: std::option::Option<i32>,
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub age: i32,
  pub nationality: &'a str,
  pub rank: i32,
  pub state: &'a str,
}

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct Rank {
    pub id: i32,
    pub level: i32,
    pub name: String,
    pub title: String,
    pub nationality: String,
}

#[derive(Insertable)]
#[table_name="ranks"]
pub struct NewRank<'a> {
  pub level: i32,
  pub name: &'a str,
  pub title: &'a str,
  pub nationality: &'a str,
}

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct Unit {
  pub id: i32,
  pub parent_id: std::option::Option<i32>,
  pub name: String,
  pub unit_type: String,
  pub kills: i32,
  pub losses: i32,
  pub leader_id: std::option::Option<i32>,
}

#[derive(Insertable)]
#[table_name="units"]
pub struct NewUnit<'a> {
  pub parent_id: std::option::Option<i32>,
  pub name: &'a str,
  pub unit_type: &'a str,
  pub kills: i32,
  pub losses: i32,
  pub leader_id: std::option::Option<i32>,
}
