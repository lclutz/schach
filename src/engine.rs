use super::*;

use crc::SquareState;
use crc::SquareState::*;
use crc::CRC;
use move_info::*;
use position::Position;

pub struct Engine {
    pub position: Position,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            position: Position::new(),
        }
    }

    pub fn from_fen(input: &str) -> Engine {
        let mut engine = Engine::new();
        let mut square = 0;

        for symbol in input.chars() {
            if symbol.is_digit(10) {
                square += symbol.to_digit(10).unwrap();
                continue;
            }

            match symbol {
                'k' => engine.position.black.kings |= 1 << square,
                'q' => engine.position.black.queens |= 1 << square,
                'n' => engine.position.black.knights |= 1 << square,
                'b' => engine.position.black.bishops |= 1 << square,
                'r' => engine.position.black.rooks |= 1 << square,
                'p' => engine.position.black.pawns |= 1 << square,
                'K' => engine.position.white.kings |= 1 << square,
                'Q' => engine.position.white.queens |= 1 << square,
                'N' => engine.position.white.knights |= 1 << square,
                'B' => engine.position.white.bishops |= 1 << square,
                'R' => engine.position.white.rooks |= 1 << square,
                'P' => engine.position.white.pawns |= 1 << square,
                '/' => {
                    assert_eq!(square % 8, 0);
                    continue;
                }
                _ => {}
            };
            square += 1;
        }

        return engine;
    }

    pub fn position_as_crc(&self) -> CRC {
        let mut crc = [Unoccupied; 64];

        for square in 0..64 {
            let mask = 0x8000000000000000 >> square;
            if (self.position.white.kings & mask) != 0 {
                crc[square] = WhiteKing;
            }
            if (self.position.white.queens & mask) != 0 {
                crc[square] = WhiteQueen;
            }
            if (self.position.white.knights & mask) != 0 {
                crc[square] = WhiteKnight;
            }
            if (self.position.white.bishops & mask) != 0 {
                crc[square] = WhiteBishop;
            }
            if (self.position.white.rooks & mask) != 0 {
                crc[square] = WhiteRook;
            }
            if (self.position.white.pawns & mask) != 0 {
                crc[square] = WhitePawn;
            }
            if (self.position.black.kings & mask) != 0 {
                crc[square] = BlackKing;
            }
            if (self.position.black.queens & mask) != 0 {
                crc[square] = BlackQueen;
            }
            if (self.position.black.knights & mask) != 0 {
                crc[square] = BlackKnight;
            }
            if (self.position.black.bishops & mask) != 0 {
                crc[square] = BlackBishop;
            }
            if (self.position.black.rooks & mask) != 0 {
                crc[square] = BlackRook;
            }
            if (self.position.black.pawns & mask) != 0 {
                crc[square] = BlackPawn;
            }
        }

        return crc;
    }

    pub fn piece_attack_mask(&self, piece: SquareState, index: usize) -> u64 {
        match piece {
            WhiteKing | BlackKing => return KING_MAP[index],
            WhiteBishop | BlackBishop => return BISHOP_MAP[index],
            WhiteRook | BlackRook => return ROOK_MAP[index],
            WhiteKnight | BlackKnight => return KNIGHT_MAP[index],
            WhiteQueen | BlackQueen => return QUEEN_MAP[index],
            _ => return 0,
        }
    }
}
