use crate::board::*;
use crate::strategy::Strategy;
use rand::prelude::*;

pub struct Random(StdRng);

impl Random {
    pub fn seeded(seed: u64) -> Random {
        Random(StdRng::seed_from_u64(seed))
    }
}
impl Strategy for Random {
    fn choose_move(&mut self, board: &Board) -> Move {
        let mut all_moves = vec![];
        for x in 0..8 {
            for y in 0..8 {
                all_moves.extend(board.get_moves_for(Loc(x, y)));
            }
        }
        *all_moves.choose(&mut self.0).unwrap()
    }
}