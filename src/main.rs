pub mod board;
pub mod strategy;

use board::*;
use strategy::Strategy;
use strategy::user;
use strategy::random;

fn main() {
    let mut board = Board::new();
    let mut white = random::Random::seeded(42);
    let mut black = random::Random::seeded(314);
    let print_board = true;
    while board.win_state() == WinState::Playing {
        if print_board {
            user::print_termion(&board);
        }
        let mv = match board.turn {
            Color::White => white.choose_move(&board),
            Color::Black => black.choose_move(&board)
        };
        println!("{:?}", mv);
        board.apply_move(mv);
    }
    if print_board {
        user::print_termion(&board);
    }
    println!("{:?}", board.win_state());
}
