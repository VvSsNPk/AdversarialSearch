use rand::prelude::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use board::Board;
use board::piece::Piece;
use board::player::{AbstractPlayer, Player};
use board::player::Player::{PlayerA, PlayerB, PlayerC};
use montecarlo::NodeState::{EXPANDABLE, EXPANDED, TERMINAL};
#[derive(Debug)]
pub struct MCTSNode{
    pub board: Board,
    pub board_action: BoardAction,
    player: Player,
    pub child: Vec<MCTSNode>,
    pub visits: u32,
    value: f32,
    node_state: NodeState,
}
const EXPLORATION_CONSTANT : f32 = 2.5;
impl MCTSNode{
    pub fn new(board: Board,player: Player,board_action: BoardAction) -> Self{
        Self{
            board,
            board_action,
            player,
            child: Vec::new(),
            visits:0,
            value:0.0,
            node_state: EXPANDABLE
        }
    }

    pub fn simulate(&self) -> u32{
        let mut board = self.board.clone();
        let mut player = PlayerA;
        let mut count = 0;
        loop{
            if board.check_winner(PlayerA)&&board.check_winner(PlayerB) || board.check_winner(PlayerB) && board.check_winner(PlayerC) ||
                board.check_winner(PlayerA) && board.check_winner(PlayerC){
                break
            }
            if board.check_winner(player){
                player = player.other_player();
                continue;
            }
            //println!("player turn : {},\n{}",player,board);
            let moves = board.get_moves_map(player);
            let choice = moves.iter().choose(&mut thread_rng()).unwrap();
            let start = choice.0;
            let end = choice.1.choose(&mut thread_rng()).unwrap();
            board.move_piece(start,&end);
            count += 1;
            if player == PlayerA{
                player = PlayerB
            }else if player == PlayerB {
                player = PlayerC
            }else{
                player = PlayerA
            }
        }
        //println!("{}, \nmoves : {}",board,count);
        if board.check_winner(PlayerA){
            if board.check_winner(PlayerC) || board.check_winner(PlayerB){
                1
            }else {
                2
            }
        }else{
            0
        }
    }

    pub fn expand(&mut self) -> Option<&mut MCTSNode>{
        let pos = self.board.get_moves_map(self.player);
        let mut acts = Vec::new();
        for  i in pos{
            for j in i.1{
                acts.push(BoardAction::new(i.0.clone(),j));
            }
        }
        //println!("{:?}",acts);
        if acts.is_empty(){
            self.node_state == TERMINAL;
            return None
        }else {
            let mut child_actions = Vec::new();
            for chile in &self.child{
                child_actions.push(chile.board_action);
            }
            let mut candidate_actions = Vec::new();
            for i in acts{
                if !child_actions.contains(&i){
                    //println!("here");
                    candidate_actions.push(i);
                }
            }
            if candidate_actions.len() == 1{
                self.node_state = EXPANDED;
                let mut board = self.board.clone();
                let act = candidate_actions[0];
                board.move_piece(&act.from,&act.to);
                self.child.push(MCTSNode::new(board,self.player.other_player(),act));
            }else{
                let rand_action = *candidate_actions.choose(&mut thread_rng()).unwrap();
                let mut board = self.board.clone();
                board.move_piece(&rand_action.from,&rand_action.to);
                self.child.push(MCTSNode::new(board,self.player.other_player(),rand_action));
            }
            self.child.last_mut()
        }

    }

    pub fn run_iteration(&mut self) -> u32{
      let delta = match self.node_state {
          EXPANDED => {
              let child = self.get_best_child().unwrap();
              child.run_iteration()
          }
          EXPANDABLE => {
              let res = self.simulate();
              if let Some(x) = self.expand(){
                  x.visits += 1;
                  x.value += res as f32;
              }
              res
          }
          TERMINAL => {
              let res = self.simulate();
              res
          }
      };
        self.value += delta as f32;
        self.visits += 1;
        delta
    }

    pub fn get_best_child(&mut self) -> Option<&mut MCTSNode>{
        if self.player == PlayerA {
            let mut highest_ucb = f32::NEG_INFINITY;
            let mut best_child = None;
            let parent_visits = (self.visits as f32).ln();
            for child in self.child.iter_mut() {
                if child.visits == 0 {
                    return Some(child)
                }
                let child_ucb = child.value / child.visits as f32 + EXPLORATION_CONSTANT * (parent_visits / child.visits as f32).sqrt();
                if child_ucb > highest_ucb {
                    highest_ucb = child_ucb;
                    best_child = Some(child)
                }
            }
            best_child
        }else{
            let mut lowest_ucb = f32::INFINITY;
            let mut best_child = None;
            let parent_visits = (self.visits as f32).ln();
            for child in self.child.iter_mut(){
                if child.visits == 0{
                    return Some(child)
                }
                let child_ucb = child.value / child.visits as f32 - EXPLORATION_CONSTANT * (parent_visits / child.visits as f32).sqrt();
                if child_ucb < lowest_ucb{
                    lowest_ucb = child_ucb;
                    best_child = Some(child)
                }
            }
            best_child
        }
    }
    pub fn get_no_of_child(&self) -> u64{
        let mut count = 1;
        for i in &self.child{
            let x = i.get_no_of_child();
            count += x;
        }
        count
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum NodeState{
    EXPANDED,
    EXPANDABLE,
    TERMINAL,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct BoardAction{
    pub from: Piece<i32>,
    pub to: Piece<i32>,
}

impl BoardAction{
    pub fn new(from: Piece<i32>,to: Piece<i32>) -> Self{
        Self{
            from,
            to,
        }
    }
}