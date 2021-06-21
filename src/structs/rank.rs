use serde::{Serialize};

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Rank {
    pub id: i32,
    pub level: i32,
    pub name: &'static str,
    pub title: &'static str,
    pub officer: bool,
    pub nationality: &'static str,
}