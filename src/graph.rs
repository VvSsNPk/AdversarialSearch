use std::collections::{HashMap, HashSet, VecDeque};
use board::Board;
use board::lazy_constants::{BOARD, DIRECTIONS};
use board::piece::Piece;
use board::player::BoardPiece::Hold;
use board::player::Player::{PlayerA, PlayerB, PlayerC};
#[derive(Debug)]
pub struct Graph{
    graph: HashMap<Piece<i32>,Vec<Piece<i32>>>,
}

impl Graph{
    pub fn new(board: &Board) -> Self{
        let mut map = HashMap::new();
        for i in BOARD.iter(){
            for dir in DIRECTIONS.iter(){
                let next = i.add(&dir);
                if BOARD.contains(&next){
                    let value = board.get_player(&next);
                    let next_to_next = next.add(&dir);
                    if value.holds() || value.is_blocked(){
                        if value.get_holds() != Some(PlayerA){
                            if board.contains_home(&next,PlayerA){
                                let y = map.entry(i.clone()).or_insert(Vec::new());
                                y.push(next);
                                map.entry(next).or_insert(Vec::new());
                            }
                        }
                        if BOARD.contains(&next_to_next){
                            let val = board.get_player(&next_to_next);
                            if val.is_empty(){
                                let x = map.entry(i.clone()).or_insert(Vec::new());
                                x.push(next_to_next);
                                map.entry(next_to_next).or_insert(Vec::new());
                            }else if val.holds(){
                                if board.contains_home(&next_to_next, PlayerA) {
                                    if val.get_holds() != Some(PlayerA){
                                        let y = map.entry(i.clone()).or_insert(Vec::new());
                                        y.push(next_to_next);
                                        map.entry(next_to_next).or_insert(Vec::new());
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }

        Self{
            graph:map
        }
    }

    pub fn bfs(&self, start: &Piece<i32>, end: &Piece<i32>) -> Option<Vec<Piece<i32>>>{
        let mut fringe = VecDeque::new();
        let start_pair = Pair::new(start);
        let mut visited = HashSet::new();
        visited.insert(start.clone());
        fringe.push_back(start_pair);
        while let Some(mut p) = fringe.pop_front(){
            visited.insert(p.node);
            if p.node == *end{
                p.path.push(end.clone());
                return Some(p.path);
            }
            for edge in self.graph.get(&p.node).unwrap(){
                if !visited.contains(edge) {
                    let mut pair = Pair::new(edge);
                    pair.path.extend(p.path.clone());
                    pair.path.push(p.node);
                    fringe.push_back(pair);
                }
            }
        }

        None
    }

}

#[derive(Debug)]
struct Pair{
    node: Piece<i32>,
    pub path : Vec<Piece<i32>>
}

impl Pair{
    pub fn new(node: &Piece<i32>) -> Self{
        Self{
            node: node.clone(),
            path : Vec::new(),
        }
    }
}
