pub mod board;
pub mod strategy;

use board::*;
use strategy::Strategy;
use strategy::user;
use strategy::random;
use strategy::minimax;

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

fn play(white: &mut dyn Strategy, black: &mut dyn Strategy, print_board: bool, title: String) -> WinState {
    let mut board = Board::new();
    let mut move_count = 0;
    while board.win_state() == WinState::Playing {
        if print_board {
            println!("{}, {} to play, move {}", title, board.turn, move_count);
            user::print_termion(&board);
        }
        let mv = match board.turn {
            Color::White => white.choose_move(&board),
            Color::Black => black.choose_move(&board)
        };
        if print_board {
            println!("{:?}", mv);
        }
        board.apply_move(mv);
        move_count += 1;
        if move_count > 500 {
            println!("Too long game, continuing...");
            break;
        }
    }
    user::print_termion(&board);
    board.win_state()
}
fn tournament(players: &mut Vec<(String, Box<dyn FnMut() -> Box<dyn Strategy>>)>) -> Vec<i32> {
    let mut scores = vec![0; players.len()];
    let mut white_scores = vec![0; players.len()];
    let mut black_scores = vec![0; players.len()];
    let mut results: Vec<Vec<i32>> = vec![vec![0; players.len()]; players.len()];
    for white_index in 0..players.len() {
        for black_index in 0..players.len() {
            println!("White {} vs Black {}", players[white_index].0, players[black_index].0);
            let mut white_strategy = players[white_index].1();
            let mut black_strategy = players[black_index].1();
            let result = play(&mut *white_strategy, &mut *black_strategy, true, format!("White {} vs Black {}", players[white_index].0, players[black_index].0));
            println!("{:?}", result);
            match result {
                WinState::Winner(Color::White) => {
                    results[white_index][black_index] = 1;
                    scores[white_index] += 1;
                    white_scores[white_index] += 1;
                    scores[black_index] -= 1;
                    black_scores[black_index] -= 1;
                }
                WinState::Winner(Color::Black) => {
                    results[white_index][black_index] = -1;
                    scores[white_index] -= 1;
                    white_scores[white_index] -= 1;
                    scores[black_index] += 1;
                    black_scores[black_index] += 1;
                }
                _ => {}
            }
            println!();
            println!();
        }
    }
    let rankings = argsort(&scores);
    for i in &rankings {
        println!("{}: {} ({} white vs {} black)", players[*i].0, scores[*i], white_scores[*i], black_scores[*i])
    }
    println!();
    println!();
    for i in &rankings {
        print!("{}", players[*i].0.chars().next().unwrap());
    }
    println!();
    use termion::color::{Fg, Green, Red, Reset};
    for i in &rankings {
        for j in &rankings {
            if results[*i][*j] == 1 {
                print!("{}+{}", Fg(Green), Fg(Reset));
            }
            else if results[*i][*j] == -1 {
                print!("{}x{}", Fg(Red), Fg(Reset));
            }
            else {
                print!(" ")
            }
        }
        println!(" {}", players[*i].0)
    }
    scores
}

fn main() {
    let mut white = minimax::Maximax { depth: 2 };
    let mut black = random::Random::seeded(2);
    println!("{:?}", play(&mut white, &mut black, true, "Test Game".into()));
    let mut players: Vec<(String, Box<dyn FnMut() -> Box<dyn Strategy>>)> = vec![
        ("Random(seed=42)".into(), Box::new(|| Box::new(random::Random::seeded(42)))),
        ("Random(seed=314)".into(), Box::new(|| Box::new(random::Random::seeded(314)))),
        ("Random(seed=1337)".into(), Box::new(|| Box::new(random::Random::seeded(1337)))),
        ("Random(seed=2)".into(), Box::new(|| Box::new(random::Random::seeded(2)))),
        ("Minimax(depth=0)".into(), Box::new(|| Box::new(minimax::BasicMinimax { depth: 0 }))),
        ("Minimax(depth=1)".into(), Box::new(|| Box::new(minimax::BasicMinimax { depth: 1 }))),
        ("Minimax(depth=2)".into(), Box::new(|| Box::new(minimax::BasicMinimax { depth: 2 }))),
        ("Maximax(depth=2)".into(), Box::new(|| Box::new(minimax::Maximax { depth: 2 }))),
        ("Minimin(depth=2)".into(), Box::new(|| Box::new(minimax::Minimin { depth: 2 }))),
        ("SinglePlayer(depth=2)".into(), Box::new(|| Box::new(minimax::SinglePlayer { depth: 2 }))),
        ("Additive(depth=1)".into(), Box::new(|| Box::new(minimax::Additive { depth: 1 }))),
        ("Additive(depth=2)".into(), Box::new(|| Box::new(minimax::Additive { depth: 2 }))),
    ];
    println!("{:?}", tournament(&mut players));
}
