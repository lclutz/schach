use super::*;
use bitboard::Bitboard;

pub struct Position {
    pub white: Bitboard,
    pub black: Bitboard,
}

impl Position {
    pub fn new() -> Position {
        Position {
            white: Bitboard::new(),
            black: Bitboard::new(),
        }
    }
}
