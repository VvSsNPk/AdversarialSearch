use rand::prelude::SliceRandom;
use rand::rngs::OsRng;
use board::player::{AbstractPlayer, Player};
use board::player::Player::{PlayerA, PlayerB, PlayerC};
use monte::Game;
use montecarlo::NodeState;
use montecarlo::NodeState::{EXPANDABLE, TERMINAL};
use monte::BoardAction;
use montecarlo::NodeState::EXPANDED;

pub struct NodeMCTS{
    pub action: Option<BoardAction>,
    pub children: Vec<NodeMCTS>,
    pub root_player: Player,
    pub state: NodeState,
    pub visits: u32,
    pub total_value: f32,
}

impl NodeMCTS{
    pub fn new(action: Option<BoardAction>,root_player: Player) -> NodeMCTS{
        NodeMCTS{
            action,
            children: Vec::new(),
            root_player,
            state: EXPANDABLE,
            visits: 0,
            total_value: 0.0
        }
    }

    pub fn run_iter(&mut self,game: &mut Game, policy: &Policy) -> f32{
        let val = match self.state {
            EXPANDED => {
                let child = policy.select_child(self,game.get_turn()==self.root_player);
                game.apply_action(&child.action.unwrap());
                child.run_iter(game,policy)
            }
            EXPANDABLE => {
                let root_player = self.root_player;
                match self.expand(game){
                    None => return game.get_reward_for_player(root_player),
                    Some(best_child) => {
                        game.apply_action(&best_child.action.unwrap());
                        let mut available = game.get_actions();
                        loop{
                            if (game.board.check_winner(PlayerA) && !game.board.check_winner(PlayerB) && !game.board.check_winner(PlayerC)) ||
                                (game.board.check_winner(PlayerB) && game.board.check_winner(PlayerC)) ||
                                (game.board.check_winner(PlayerA)) && game.board.check_winner(PlayerC) ||
                                (game.board.check_winner(PlayerA) && game.board.check_winner(PlayerB)){
                                break
                            }
                            if available.is_empty(){
                                game.player = game.player.other_player();
                                available = game.get_actions();
                                continue;
                            }
                            let action =  available.choose(&mut OsRng).unwrap();
                            game.apply_action(&action);
                            available = game.get_actions();
                        }
                        let reward = game.get_reward_for_player(root_player);
                        best_child.visits +=1;
                        best_child.total_value += reward;
                        reward
                    }
                }
            }
            TERMINAL => {game.get_reward_for_player(self.root_player)}
        };
        self.visits += 1;
        self.total_value += val;
        val
    }

    pub fn expand(&mut self, game: &Game) -> Option<&mut NodeMCTS>{
        let acts = game.get_actions();
        if acts.is_empty(){
            self.state= TERMINAL;
            return None
        }
        let mut child_actions = Vec::new();
        for child in &self.children{
            child_actions.push(child.action.expect("No node with action"));
        }
        let mut candidate_actions = Vec::new();
        for action in acts{
            if !child_actions.contains(&action){
                candidate_actions.push(action);
            }
        }
        if candidate_actions.len() == 1{
            self.children.push(NodeMCTS::new(Some(candidate_actions[0]),self.root_player));
            self.state = EXPANDED;
        }else{
            let rand_action = *candidate_actions.choose(&mut OsRng).unwrap();
            let node = NodeMCTS::new(Some(rand_action),self.root_player);
            self.children.push(node)
        }
        self.children.last_mut()
    }
}


pub struct Policy{
    pub exploration : f32,
}

impl Policy{
    pub fn new(exploration: f32) -> Self{
        Self{
            exploration,
        }
    }

    fn select_child<'a>(&'a self, node: &'a mut NodeMCTS,is_root_turn: bool) -> &mut NodeMCTS{
        match is_root_turn{
            true => {
                let mut highest: f32 = f32::NEG_INFINITY;
                let mut best_child = None;
                let parent_visits = (node.visits as f32).ln();
                for child in node.children.iter_mut(){
                    if child.visits == 0{
                        return child;
                    }
                    let child_ucb = child.total_value / child.visits as f32 + self.exploration*(parent_visits/child.visits as f32).sqrt();
                    if child_ucb > highest{
                        highest = child_ucb;
                        best_child =  Some(child);
                    }
                }
                best_child.unwrap()
            },
            false => {
                let mut highest: f32 = f32::INFINITY;
                let mut best_child = None;
                let parent_visits = (node.visits as f32).ln();
                for child in node.children.iter_mut(){
                    if child.visits == 0{
                        return child;
                    }
                    let child_ucb = child.total_value / child.visits as f32 - self.exploration*(parent_visits/child.visits as f32).sqrt();
                    if child_ucb < highest{
                        highest = child_ucb;
                        best_child =  Some(child);
                    }
                }
                best_child.unwrap()
            }
        }
    }
}

pub struct TreeMCTS{
    root: NodeMCTS,
    root_game_state : Game,
    policy: Policy,
}

impl TreeMCTS{
    pub fn new(game: Game, policy: Policy) -> TreeMCTS{
        let node = NodeMCTS::new(None,game.get_turn());
        TreeMCTS{
            root: node,
            root_game_state: game,
            policy,
        }
    }

    pub fn run(&mut self,iterations: usize){
        for _ in 0..iterations{
            self.root.run_iter(&mut self.root_game_state.clone(),&self.policy);
        }
    }

    pub fn get_best_action(&mut self) -> Option<BoardAction>{
        self.root.children.as_slice().into_iter().reduce(|a,b| if a.visits > b.visits {a} else { b }).map(|n| n.action.unwrap())
    }
}