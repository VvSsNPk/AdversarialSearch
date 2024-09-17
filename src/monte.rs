use std::fmt::{Debug, Display, Formatter};
use board::Board;
use board::piece::Piece;
use board::player::{AbstractPlayer, Player};
use board::player::Player::{PlayerA, PlayerB, PlayerC};

#[derive(Clone)]
pub struct Game{
    pub board: Board,
    pub player: Player,
}
#[derive(Debug,PartialEq,Clone,Copy,Eq)]
pub struct BoardAction{
    pub from: Piece<i32>,
    pub to: Piece<i32>,
}
impl BoardAction{
    pub fn new(from: Piece<i32>,to:Piece<i32>) -> Self{
        Self{
            from,to
        }
    }
}
impl Display for BoardAction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} -> {}",self.from,self.to)
    }
}
impl Game{
    pub fn get_actions(&self) -> Vec<BoardAction> {
        let mut ans = Vec::new();
        let player_pos = self.board.get_player_pos(self.player);
        for i in player_pos{
            let to = self.board.get_forward_moves(&i,self.player);
            for j in to{
                let board_action = BoardAction::new(i.clone(),j);
                ans.push(board_action);
            }
        }
        ans
    }

    pub fn apply_action(&mut self, action: &BoardAction) {
        self.board.move_piece(&action.from,&action.to);
        if self.player == PlayerA{
            self.player =PlayerB
        }else if self.player == PlayerB{
            self.player = PlayerC
        }else{
            self.player = PlayerA
        }
    }

    pub fn get_turn(&self) -> Player {
        self.player
    }

    pub fn get_reward_for_player(&self,player: Player) -> f32 {
        if self.board.check_winner(player) {
            if self.board.check_winner(player.other_player()) || self.board.check_winner(player.other_player().other_player()) {
                1.0
            }else{
                2.0
            }
        }else{
            -3.0
        }
    }
}


