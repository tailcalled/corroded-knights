use std::io::{self, BufRead, Write};
use crate::board::*;
use crate::strategy::*;

pub struct User();

pub fn print_plain(board: &Board) {
    println!("   ABCDEFGH   ");
    println!();
    for y in (0..8).rev() {
        print!("{:}  ", y+1);
        for x in 0..8 {
            print!("{:}", board[Loc(x, y)].map(|piece| to_char(piece)).unwrap_or('.'));
        }
        println!("  {:}", y+1);
    }
    println!();
    println!("   ABCDEFGH   ");
}
pub fn print_termion(board: &Board) {
    use termion::color::{Fg, Bg, Cyan, Blue, Black, White, Reset};
    println!("   ABCDEFGH   ");
    println!();
    for y in (0..8).rev() {
        print!("{:}  ", y+1);
        for x in 0..8 {
            match (x+y)%2 == 1 {
                false => print!("{:}", Bg(Cyan)),
                true => print!("{:}", Bg(Blue)),
            }
            match board[Loc(x, y)] {
                None => {},
                Some(Piece { color: Color::White, .. }) => print!("{:}", Fg(White)),
                Some(Piece { color: Color::Black, .. }) => print!("{:}", Fg(Black)),
            }
            print!("{:}", board[Loc(x, y)].map(|piece| to_emoji(piece)).unwrap_or(' '));
            print!("{:}{:}", Bg(Reset), Fg(Reset));
        }
        println!("  {:}", y+1);
    }
    println!();
    println!("   ABCDEFGH   ");
}

pub fn to_char(piece: Piece) -> char {
    use Kind::*;
    use Color::*;
    let ch = match &piece.kind {
        Pawn => 'p',
        Rook => 'r',
        Knight => 'n',
        Bishop => 'b',
        Queen => 'q',
        King => 'k',
    };
    if piece.color == White {
        ch.to_ascii_uppercase()
    }
    else {
        ch
    }
}
pub fn to_emoji(piece: Piece) -> char {
    use Kind::*;
    match &piece.kind {
        Pawn => '♟',
        Rook => '♜',
        Knight => '♞',
        Bishop => '♝',
        Queen => '♛',
        King => '♚',
    }
}

fn parse_command(chars: impl Iterator<Item = char>) -> Option<Loc> {
    let mut file: Option<isize> = None;
    let mut rank: Option<isize> = None;
    for ch in chars {
        if ch >= '1' && ch <= '8' {
            if rank == None {
                rank = Some((ch as isize) - ('1' as isize))
            }
            else {
                println!("Duplicate file");
                return None
            }
        }
        else if ch >= 'a' && ch <= 'h' {
            if file == None {
                file = Some((ch as isize) - ('a' as isize))
            }
            else {
                println!("Duplicate file");
                return None
            }
        }
        else if ch >= 'A' && ch <= 'H' {
            if file == None {
                file = Some((ch as isize) - ('A' as isize))
            }
            else {
                println!("Duplicate file");
                return None
            }
        }
        else if !ch.is_whitespace() {
            println!("Unexpected character: '{}'", ch)
        }
    }
    let Some(file) = file else { println!("Missing file"); return None };
    let Some(rank) = rank else { println!("Missing rank"); return None };
    Some(Loc(file, rank))
}

fn get_command() -> Loc {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let chs = line.chars().peekable();
        if let Some(loc) = parse_command(chs) {
            return loc
        }
    }
}

fn get_move(moves: Vec<Move>) -> Option<Move> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    for (i, mv) in moves.iter().enumerate() {
        println!("{:}. {:?}", i+1, mv)
    }
    print!("> ");
    stdout.flush().unwrap();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    if let Ok(ix) = line.trim().parse::<usize>() {
        Some(moves[ix-1])
    }
    else {
        println!("Parse error. Expected index.");
        None
    }
}

impl Strategy for User {
    fn choose_move(&mut self, board: &Board) -> Move {
        println!();
        println!();
        println!();
        print_termion(board);
        println!();
        loop {
            let loc = get_command();
            if board[loc] == None {
                println!("No piece here");
                continue;
            }
            println!("{:?}", loc);
            println!("{:?}", board[loc]);
            let moves = board.get_moves_for(loc);
            if moves.len() == 0 {
                println!("You cannot move this piece");
                continue;
            }
            if let Some(mv) = get_move(moves) {
                return mv;
            }
        }
    }
}