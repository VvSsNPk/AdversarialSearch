extern crate num_traits;
extern crate once_cell;
extern crate serde;
extern crate geo;
extern crate rand;
extern crate team489;

use std::path::PathBuf;
use geo::Centroid;
use team489::handler::Action;
use team489::simple_greedy_agent;

pub mod board;
pub mod handler;
pub mod monte;
pub mod graph;
pub mod minimax;
pub mod montecarlo;
pub mod elevateboard;
pub mod new_mcts;
fn main() {
    let mut path = PathBuf::new();
    path.push("agent-configs/ss24.1.2.8.json");
    simple_greedy_agent(&path).expect("error");
}
