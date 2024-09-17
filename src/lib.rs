extern crate num_traits;
extern crate once_cell;
extern crate serde;
extern crate serde_json;
extern crate geo;
extern crate rand;


use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde_json::from_reader;
use board::Board;
use board::lazy_constants::DIRECTIONS;
use board::piece::Piece;
use board::player::Player::PlayerA;
use elevateboard::MCTSTree;
use graph::Graph;
use handler::{Action, JsonData, Percept, Receiver, Sender};
use monte::Game;
use new_mcts::{Policy, TreeMCTS};


pub mod board;
pub mod handler;
pub mod monte;
pub mod graph;
pub mod minimax;
pub mod montecarlo;
pub mod elevateboard;

pub mod new_mcts;

pub fn simple_greedy_agent(path_buf: &PathBuf) -> Result<(),Box<dyn Error>>{
    let f = File::open(path_buf)?;
    let mut buffer = BufReader::new(f);
    let r:JsonData = from_reader(&mut buffer)?;
    let initial_sender = Sender{
        agent: r.agent,
        pwd: r.pwd,
        actions: vec![],
        single_request: false,
    };
    let client = reqwest::blocking::Client::new();
    let mut url = String::from(r.url);
    url.push_str("act/");
    url.push_str(&*r.env.clone());
    let mut res = client.put(url.clone()).json(&initial_sender).send().unwrap();
    loop {
        let mut board = Board::new();
        if res.status().is_client_error(){
            println!("error");
            break;
        }
        let mut y: Receiver = res.json().unwrap();
        let errors = y.errors;
        let message = y.messages;
        dbg!(errors);
        dbg!(message);
        let mut percept = Percept{
            A: None,
            B: None,
            C: None,
        };
        let action = y.action_requests.pop().unwrap();
        board.init(action.percept.clone());
        percept = action.percept;
        let mut mov = None;
        let before = board.clone();
        dbg!(before);
        mov = simple_monte_carlo(&mut board);
        let after = board.clone();
        dbg!(after);
        let mut final_sender = initial_sender.clone();
        let mut action = Action{
            run: action.run.clone(),
            action: Vec::new(),
        };
        for m in mov.unwrap(){
            action.action.push((m.get_x(),m.get_y()));
        }
        let sent = action.clone();
        dbg!(sent);
        final_sender.actions = vec![action.clone()];
        res = client.put(url.clone()).json(&final_sender).send().unwrap();
    }


    Ok(())
}
pub fn simple_monte_carlo(board: &mut Board) -> Option<Vec<Piece<i32>>>{
    let graph = Graph::new(board);
    let game = Game{
        board: board.clone(),
        player: PlayerA,
    };
    let policy = Policy::new(2.5);
    let mut search_tree = TreeMCTS::new(game,policy);
    search_tree.run(10000);
    let best_action = search_tree.get_best_action();
    let from = best_action.unwrap().from;
    let to = best_action.unwrap().to;
    let x = from.minus(&to);
    board.move_piece(&from,&to);
    if DIRECTIONS.contains(&x){
        return Some(vec![from,to])
    }else{
        let res = graph.bfs(&from,&to);
        res
    }
}

pub fn monte_carlo_tree_search(board: &mut Board) -> Option<Vec<Piece<i32>>>{
    let graph = Graph::new(board);
    let mut mcts_tree = MCTSTree::new(board.clone(),PlayerA);
    mcts_tree.run(15000);
    let action = mcts_tree.get_best_action().unwrap();
    let x = action.from.minus(&action.to);
    board.move_piece(&action.from,&action.to);
    if DIRECTIONS.contains(&x){
        return Some(vec![action.from,action.to])
    }else {
        let res = graph.bfs(&action.from,&action.to);
        res
    }
}

