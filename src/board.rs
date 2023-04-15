use std::ops::{Add, Sub, Mul, Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn opposite(self) -> Color {
        use Color::*;
        match self {
            White => Black,
            Black => White
        }
    }
    pub fn forward(self) -> isize {
        use Color::*;
        match self {
            White => 1,
            Black => -1
        }
    }
    pub fn forwardv(self) -> Loc {
        Loc(0, self.forward())
    }
    pub fn pawn_rank(self) -> isize {
        use Color::*;
        match self {
            White => 1,
            Black => 6
        }
    }
    pub fn back_rank(self) -> isize {
        use Color::*;
        match self {
            White => 0,
            Black => 7
        }
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Color::*;
        match self {
            White => write!(f, "White"),
            Black => write!(f, "Black"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastleState {
    pub white: (bool, bool),
    pub black: (bool, bool),
}
impl Index<Color> for CastleState {
    type Output = (bool, bool);
    fn index(&self, ix: Color) -> &Self::Output {
        match ix {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }
}
impl IndexMut<Color> for CastleState {
    fn index_mut(&mut self, ix: Color) -> &mut Self::Output {
        match ix {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Loc(pub isize, pub isize);
impl Loc {
    pub fn rank(self) -> isize { self.1 }
    pub fn file(self) -> isize { self.0 }
    pub fn rotl(self) -> Loc {
        Loc(-self.1, self.0)
    }
    pub fn rotr(self) -> Loc {
        Loc(self.1, -self.0)
    }
    pub fn is_valid(self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }
}
impl Index<Loc> for Board {
    type Output = Option<Piece>;
    fn index(&self, ix: Loc) -> &Self::Output {
        &self.positions[(7-ix.1) as usize][ix.0 as usize]
    }
}
impl IndexMut<Loc> for Board {
    fn index_mut(&mut self, ix: Loc) -> &mut Self::Output {
        &mut self.positions[(7-ix.1) as usize][ix.0 as usize]
    }
}
impl Add<Loc> for Loc {
    type Output = Loc;
    fn add(self, rhs: Loc) -> Loc {
        Loc(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<Loc> for Loc {
    type Output = Loc;
    fn sub(self, rhs: Loc) -> Loc {
        Loc(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul<isize> for Loc {
    type Output = Loc;
    fn mul(self, rhs: isize) -> Loc {
        Loc(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastleMove {
    pub rook: Piece,
    pub rook_from: Loc,
    pub rook_to: Loc
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub player: Color,
    pub from: Loc,
    pub to: Loc,
    pub piece: Piece,
    pub attack: Option<(Piece, Loc)>,
    pub promote: Option<Piece>,
    pub castle: Option<CastleMove>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    pub positions: [[Option<Piece>; 8]; 8],
    pub castling: CastleState,
    pub moves_since_progress: usize,
    pub en_passant_file: Option<isize>,
    pub turn: Color,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WinState {
    Playing,
    Winner(Color),
    Draw
}

impl Board {
    pub fn new() -> Board {
        use Kind::*;
        use Color::*;
        Board {
            positions: [ 
                [ Some(Piece { kind: Rook, color: Black }) , Some(Piece { kind: Knight, color: Black }), Some(Piece { kind: Bishop, color: Black }), Some(Piece { kind: Queen, color: Black }), Some(Piece { kind: King, color: Black }), Some(Piece { kind: Bishop, color: Black }), Some(Piece { kind: Knight, color: Black }), Some(Piece { kind: Rook, color: Black })],
                [ Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black }), Some(Piece { kind: Pawn, color: Black })],
                [ None, None, None, None, None, None, None, None],
                [ None, None, None, None, None, None, None, None],
                [ None, None, None, None, None, None, None, None],
                [ None, None, None, None, None, None, None, None],
                [ Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White }), Some(Piece { kind: Pawn, color: White })],
                [ Some(Piece { kind: Rook, color: White }), Some(Piece { kind: Knight, color: White }), Some(Piece { kind: Bishop, color: White }), Some(Piece { kind: Queen, color: White }), Some(Piece { kind: King, color: White }), Some(Piece { kind: Bishop, color: White }), Some(Piece { kind: Knight, color: White }), Some(Piece { kind: Rook, color: White })],
            ],
            castling: CastleState { white: (false, false), black: (false, false) },
            moves_since_progress: 0,
            en_passant_file: None,
            turn: White,
        }
    }
    pub fn get_moves_for(&self, loc: Loc) -> Vec<Move> {
        use Kind::*;
        let Some(piece) = self[loc] else { return vec![] };
        if piece.color != self.turn {
            return vec![];
        }
        let mut moves = vec![];
        match piece.kind {
            Pawn => {
                // normal pawn move
                let target = loc + piece.color.forwardv();
                if target.is_valid() && self[target] == None {
                    moves.push(loc + piece.color.forwardv())
                }
                // double pawn move
                let target = loc + piece.color.forwardv() * 2;
                if target.is_valid() && self[target] == None && piece.color.pawn_rank() == loc.rank() {
                    moves.push(target)
                }
                // pawn attack
                for offset in [-1, 1] {
                    let target = loc + Loc(offset, piece.color.forward());
                    if target.is_valid() && self[target] != None {
                        moves.push(target)
                    }
                }
            }
            Rook => {
                for dir in [Loc(1, 0), Loc(0, 1), Loc(-1, 0), Loc(0, -1)] {
                    for offset in 1..7 {
                        let target = loc + dir * offset;
                        if !target.is_valid() || self[target].map(|x| x.color) == Some(piece.color) {
                            break;
                        }
                        moves.push(target);
                        if self[target] != None {
                            break;
                        }
                    }
                }
            }
            Knight => {
                for long in [-2, 2] {
                    for short in [-1, 1] {
                        for delta in [Loc(long, short), Loc(short, long)] {
                            let target = loc + delta;
                            if target.is_valid() {
                                moves.push(target);
                            }
                        }
                    }
                }
            }
            Bishop => {
                for dir in [Loc(1, 1), Loc(1, -1), Loc(-1, -1), Loc(-1, 1)] {
                    for offset in 1..7 {
                        let target = loc + dir * offset;
                        if !target.is_valid() || self[target].map(|x| x.color) == Some(piece.color) {
                            break;
                        }
                        moves.push(target);
                        if self[target] != None {
                            break;
                        }
                    }
                }
            }
            Queen => {
                for dir in [Loc(1, 0), Loc(0, 1), Loc(-1, 0), Loc(0, -1), Loc(1, 1), Loc(1, -1), Loc(-1, -1), Loc(-1, 1)] {
                    for offset in 1..7 {
                        let target = loc + dir * offset;
                        if !target.is_valid() || self[target].map(|x| x.color) == Some(piece.color) {
                            break;
                        }
                        moves.push(target);
                        if self[target] != None {
                            break;
                        }
                    }
                }
            }
            King => {
                for dir in [Loc(1, 0), Loc(0, 1), Loc(-1, 0), Loc(0, -1), Loc(1, 1), Loc(1, -1), Loc(-1, -1), Loc(-1, 1)] {
                    let target = loc + dir;
                    if !target.is_valid() {
                        continue;
                    }
                    moves.push(target);
                }
            }
        }
        let mut moves: Vec<Move> = moves.iter().map(|to| Move {
            player: piece.color,
            from: loc,
            to: *to,
            piece: piece,
            attack: self[*to].map(|piece| (piece, *to)),
            promote: None,
            castle: None
        }).filter(|mv| mv.attack.map(|(pc, _)| pc.color) != Some(self.turn)).collect();
        // special moves
        match piece.kind {
            Pawn => {
                // promotions
                let promote_rank = piece.color.pawn_rank() + 5 * piece.color.forward();
                if promote_rank == loc.rank() {
                    moves = [Rook, Knight, Bishop, Queen].iter().flat_map(|kind| moves.iter().map(|mv| {
                        let mut mv = *mv;
                        mv.promote = Some(Piece {
                            kind: *kind,
                            color: piece.color
                        });
                        mv
                    })).collect();
                }
                // en passant
                let en_passant_rank = piece.color.pawn_rank() + 3 * piece.color.forward();
                if (self.en_passant_file == Some(loc.file() + 1) || self.en_passant_file == Some(loc.file() - 1)) && en_passant_rank == loc.rank() {
                    let target = Loc(self.en_passant_file.unwrap(), en_passant_rank);
                    moves.push(Move {
                        player: piece.color,
                        from: loc,
                        to: Loc(self.en_passant_file.unwrap(), loc.rank() + piece.color.forward()),
                        piece: piece,
                        attack: Some((self[target].unwrap(), target)),
                        promote: None,
                        castle: None
                    })
                }
            }
            King => {
                // castling
                // TODO you shouldn't be allowed to castle if you're threatened
                for (rook_file, mut spaces, king_dir, castled) in [(0, 1..4, -1, self.castling[piece.color].0), (7, 5..7, 1, self.castling[piece.color].1)] {
                    if !castled {
                        let rook_loc = Loc(rook_file, loc.rank());
                        let Some(rook) = self[rook_loc] else { continue; };
                        if spaces.all(|file| self[Loc(file, loc.rank())] == None) {
                            moves.push(Move {
                                player: piece.color,
                                from: loc,
                                to: loc + Loc(2*king_dir, 0),
                                piece: piece,
                                attack: None,
                                promote: None,
                                castle: Some(CastleMove {
                                    rook: rook,
                                    rook_from: rook_loc,
                                    rook_to: loc + Loc(king_dir, 0)
                                })
                            })
                        }
                    }
                }
            }
            _ => {}
        }
        moves
    }
    pub fn apply_move(&mut self, mv: Move) {
        let mut piece = self[mv.from].unwrap();
        self[mv.from] = None;
        if let Some((_, position)) = mv.attack {
            self[position] = None;
        }
        if let Some(promotion) = mv.promote {
            piece = promotion;
        }
        self[mv.to] = Some(piece);
        if let Some(castle_move) = mv.castle {
            let rook = self[castle_move.rook_from].unwrap();
            self[castle_move.rook_from] = None;
            self[castle_move.rook_to] = Some(rook);
        }
        if piece.kind == Kind::King || (piece.kind == Kind::Rook && mv.from == Loc(0, piece.color.back_rank())) {
            self.castling[piece.color].0 = true;
        }
        if piece.kind == Kind::King || (piece.kind == Kind::Rook && mv.from == Loc(7, piece.color.back_rank())) {
            self.castling[piece.color].1 = true;
        }
        if piece.kind == Kind::Pawn && (mv.from.rank() - mv.to.rank()).abs() == 2 {
            self.en_passant_file = Some(mv.to.file());
        }
        else {
            self.en_passant_file = None;
        }
        self.turn = self.turn.opposite();
        if mv.attack == None && mv.piece.kind != Kind::Pawn && mv.castle == None {
            self.moves_since_progress += 1;
        }
        else {
            self.moves_since_progress = 0;
        }
    }
    pub fn consider_move<T>(&mut self, mv: Move, f: impl FnOnce(&mut Board) -> T) -> T {
        let prev_castling = self.castling;
        let prev_en_passant = self.en_passant_file;
        let prev_progress = self.moves_since_progress;
        self.apply_move(mv);
        let res = f(self);
        // Undo move
        if let Some(castle_move) = mv.castle {
            self[castle_move.rook_to] = None;
            self[castle_move.rook_from] = Some(castle_move.rook);
        }
        self[mv.from] = Some(mv.piece);
        self[mv.to] = None;
        if let Some((attacked, loc)) = mv.attack {
            self[loc] = Some(attacked);
        }
        self.castling = prev_castling;
        self.en_passant_file = prev_en_passant;
        self.moves_since_progress = prev_progress;
        self.turn = self.turn.opposite();
        res
    }
    pub fn win_state(&self) -> WinState {
        if self.moves_since_progress >= 50 {
            WinState::Draw
        }
        // TODO right now I'm just letting winning = taking the opponents' king, but really we should implement the checkmating nonsense
        else if !(0..8).any(|x| (0..8).any(|y| self[Loc(x, y)] == Some(Piece {kind: Kind::King, color:self.turn}))) {
            WinState::Winner(self.turn.opposite())
        }
        else {
            WinState::Playing
        }
    }
}