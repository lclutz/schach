#[derive(Clone, Copy)]
pub enum SquareState {
    Unoccupied,
    WhiteKing,
    WhiteQueen,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackPawn,
}

pub type CRC = [SquareState; 64];
