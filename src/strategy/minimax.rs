use crate::board::*;
use crate::strategy::Strategy;

// Searches the game tree for the best possible move
pub struct BasicMinimax {
    pub depth: u8,
}

type Evaluation = i32;
pub fn flip(color: Color) -> Evaluation {
    match color {
        Color::White => 1,
        Color::Black => -1
    }
}

impl BasicMinimax {
    fn evaluate_piece(piece: Piece) -> Evaluation {
        use Kind::*;
        flip(piece.color) * match piece.kind {
            Pawn => 1,
            Knight => 3,
            Bishop => 3,
            Rook => 5,
            Queen => 9,
            King => 4
        }
    }
    fn evaluate_board(board: &Board) -> Evaluation {
        let mut score = 0;
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = board[Loc(x, y)] {
                    score += BasicMinimax::evaluate_piece(piece) * 100 + (y as Evaluation)
                }
            }
        }
        score
    }
    fn evaluate_recursive(depth: u8, board: &mut Board) -> Evaluation {
        if depth == 0 || board.win_state() != WinState::Playing {
            BasicMinimax::evaluate_board(board) + flip(board.turn) * (depth as Evaluation)
        }
        else {
            BasicMinimax::search(depth-1, board).1
        }
    }
    fn search(depth: u8, board: &mut Board) -> (Move, Evaluation) {
        let mut best = None;
        for x in 0..8 {
            for y in 0..8 {
                for mv in board.get_moves_for(Loc(x, y)) {
                    let score = board.consider_move(mv, |new_board| {
                        BasicMinimax::evaluate_recursive(depth, new_board)
                    });
                    if best == None {
                        best = Some((mv, score));
                    }
                    else if let Some((bmv, bscore)) = best {
                        if bscore * flip(board.turn) < score * flip(board.turn) {
                            best = Some((mv, score));
                        }
                    }
                }
            }
        }
        best.unwrap()
    }
}

impl Strategy for BasicMinimax {
    fn choose_move(&mut self, board: &Board) -> Move {
        let mut board = board.clone();
        BasicMinimax::search(self.depth, &mut board).0
    }
}

// Searches the game tree for the best possible move, assuming the opponent will cooperate
pub struct Maximax {
    pub depth: u8,
}
// Searches the game tree for the worst possible move, assuming the opponent will do its best
pub struct Minimin {
    pub depth: u8,
}

impl Maximax {
    // Assumes it is currently the opponent's turn
    fn evaluate(depth: u8, board: &mut Board) -> Evaluation {
        if depth == 0 || board.win_state() != WinState::Playing {
            -flip(board.turn) * BasicMinimax::evaluate_board(board) + 10 * (depth as Evaluation)
        }
        else {
            -Minimin::search(depth-1, board).1
        }
    }
    fn search(depth: u8, board: &mut Board) -> (Move, Evaluation) {
        let mut best = None;
        for x in 0..8 {
            for y in 0..8 {
                for mv in board.get_moves_for(Loc(x, y)) {
                    let score = board.consider_move(mv, |new_board| {
                        Maximax::evaluate(depth, new_board)
                    });
                    if let Some((attacked, _)) = mv.attack {
                        if attacked.kind == Kind::King {
                        }
                    }
                    if best == None {
                        best = Some((mv, score));
                    }
                    else if let Some((bmv, bscore)) = best {
                        if bscore < score {
                            best = Some((mv, score));
                            if let Some((attacked, _)) = bmv.attack {
                                if attacked.kind == Kind::King {
                                }
                            }
                            if let Some((attacked, _)) = mv.attack {
                                if attacked.kind == Kind::King {
                                }
                            }
                        }
                    }
                }
            }
        }
        best.unwrap()
    }
}

impl Minimin {
    // Assumes it is currently the opponent's turn
    fn evaluate(depth: u8, board: &mut Board) -> Evaluation {
        if depth == 0 || board.win_state() != WinState::Playing {
            -flip(board.turn) * BasicMinimax::evaluate_board(board) - 10 * (depth as Evaluation)
        }
        else {
            -Maximax::search(depth-1, board).1
        }
    }
    fn search(depth: u8, board: &mut Board) -> (Move, Evaluation) {
        let mut best = None;
        for x in 0..8 {
            for y in 0..8 {
                for mv in board.get_moves_for(Loc(x, y)) {
                    let score = board.consider_move(mv, |new_board| {
                        Minimin::evaluate(depth, new_board)
                    });
                    if best == None {
                        best = Some((mv, score));
                    }
                    else if let Some((bmv, bscore)) = best {
                        if bscore > score {
                            best = Some((mv, score));
                        }
                    }
                }
            }
        }
        best.unwrap()
    }
}

impl Strategy for Maximax {
    fn choose_move(&mut self, board: &Board) -> Move {
        let mut board = board.clone();
        let (mv, evaluation) = Maximax::search(self.depth, &mut board);
        println!("{}", evaluation);
        mv
    }
}
impl Strategy for Minimin {
    fn choose_move(&mut self, board: &Board) -> Move {
        let mut board = board.clone();
        Minimin::search(self.depth, &mut board).0
    }
}

// Searches the game tree for the best possible move, assuming that the opponent makes no moves
pub struct SinglePlayer {
    pub depth: u8,
}

impl SinglePlayer {
    fn evaluate(depth: u8, board: &mut Board) -> Evaluation {
        if depth == 0 || board.win_state() != WinState::Playing {
            flip(board.turn) * BasicMinimax::evaluate_board(board) + (depth as Evaluation)
        }
        else {
            SinglePlayer::search(depth-1, board).1
        }
    }
    fn search(depth: u8, board: &mut Board) -> (Move, Evaluation) {
        let mut best = None;
        for x in 0..8 {
            for y in 0..8 {
                for mv in board.get_moves_for(Loc(x, y)) {
                    let score = board.consider_move(mv, |new_board| {
                        new_board.turn = new_board.turn.opposite();
                        new_board.en_passant_file = None;
                        let s = SinglePlayer::evaluate(depth, new_board);
                        new_board.turn = new_board.turn.opposite();
                        s
                    });
                    if best == None {
                        best = Some((mv, score));
                    }
                    else if let Some((bmv, bscore)) = best {
                        if bscore < score {
                            best = Some((mv, score));
                        }
                    }
                }
            }
        }
        best.unwrap()
    }
}

impl Strategy for SinglePlayer {
    fn choose_move(&mut self, board: &Board) -> Move {
        let mut board = board.clone();
        SinglePlayer::search(self.depth, &mut board).0
    }
}

// Adds up the values for the expanded game tree and picks the best option among them
pub struct Additive {
    pub depth: u8,
}

impl Additive {
    fn evaluate(depth: u8, board: &mut Board, opponent: bool) -> f64 {
        if depth == 0 {
            BasicMinimax::evaluate_board(board) as f64
        }
        else if board.win_state() != WinState::Playing {
            let mut move_count = 0;
            for x in 0..8 {
                for y in 0..8 {
                    move_count += board.get_moves_for(Loc(x, y)).len();
                }
            }
            (BasicMinimax::evaluate_board(board) as f64) * (move_count as f64).powf(depth as f64)
        }
        else {
            let mut score = 0.0;
            let mut move_count = 0;
            for x in 0..8 {
                for y in 0..8 {
                    for mv in board.get_moves_for(Loc(x, y)) {
                        score += board.consider_move(mv, |new_board| {
                            Additive::evaluate(depth-1, new_board, !opponent)
                        });
                        move_count += 1;
                    }
                }
            }
            if opponent {
                score /= move_count as f64;
            }
            score
        }
    }
    fn search(depth: u8, board: &mut Board) -> (Move, f64) {
        let mut best = None;
        for x in 0..8 {
            for y in 0..8 {
                for mv in board.get_moves_for(Loc(x, y)) {
                    let score = board.consider_move(mv, |new_board| {
                        Additive::evaluate(depth, new_board, true)
                    });
                    if best == None {
                        best = Some((mv, score));
                    }
                    else if let Some((bmv, bscore)) = best {
                        if bscore * (flip(board.turn) as f64) < score * (flip(board.turn) as f64) {
                            best = Some((mv, score));
                        }
                    }
                }
            }
        }
        best.unwrap()
    }
}

impl Strategy for Additive {
    fn choose_move(&mut self, board: &Board) -> Move {
        let mut board = board.clone();
        let (mv, evaluation) = Additive::search(self.depth, &mut board);
        println!("Evaluation: {}", evaluation);
        mv
    }
}