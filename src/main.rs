use std::fmt;

use petgraph::graph::Graph;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum UnitType {
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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Unit {
  id: i32,
  name: &'static str,
  unit_type: String,
  kills: i32,
  losses: i32,
  leader: &'static str,
}


fn main() {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  print!("\n---- Das Heer Überwacher ----\n           v{}  \n", VERSION);


  let mut graph = Graph::<Unit, u32>::new();
  let company_hq = graph.add_node(Unit{
    id: 0,
    name: "Company HQ",
    unit_type: UnitType::Infantry.to_string(),
    kills: 0,
    losses: 0,
    leader: "Hauptmann Mann",
  });

  let platoon_1 = graph.add_node(Unit{
    id: 1,
    name: "First Platoon",
    unit_type: UnitType::Infantry.to_string(),
    kills: 0,
    losses: 0,
    leader: "Leutnant Keller",
  });

  let platoon_2 = graph.add_node(Unit{
    id: 2,
    name: "Second Platoon",
    unit_type: UnitType::Infantry.to_string(),
    kills: 0,
    losses: 0,
    leader: "Leutnant Gaunt",
  });

  graph.extend_with_edges(&[
      (company_hq, platoon_1),
      (company_hq, platoon_2),
  ]);

  println!("{}", serde_json::to_string_pretty(&graph).unwrap());



  // let second_platoon = Unit{
  //   id: 2,
  //   name: "Second Platoon",
  //   unit_type: UnitType::Infantry,
  //   kills: 0,
  //   losses: 0,
  //   parent_unit: None,
  //   child_units: Vec::<&'static Unit>::new(),
  // };

  

  

}
