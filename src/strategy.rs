pub mod user;
pub mod random;

use crate::board::*;

pub trait Strategy {
    fn choose_move(&mut self, board: &Board) -> Move;
}