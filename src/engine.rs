use super::*;

use crc::SquareState;
use crc::CRC;
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
        let mut crc = [SquareState::Unoccupied; 64];

        for square in 0..64 {
            let mask = 1 << square;
            if (self.position.white.kings & mask) != 0 {
                crc[square] = SquareState::WhiteKing;
            }
            if (self.position.white.queens & mask) != 0 {
                crc[square] = SquareState::WhiteQueen;
            }
            if (self.position.white.knights & mask) != 0 {
                crc[square] = SquareState::WhiteKnight;
            }
            if (self.position.white.bishops & mask) != 0 {
                crc[square] = SquareState::WhiteBishop;
            }
            if (self.position.white.rooks & mask) != 0 {
                crc[square] = SquareState::WhiteRook;
            }
            if (self.position.white.pawns & mask) != 0 {
                crc[square] = SquareState::WhitePawn;
            }
            if (self.position.black.kings & mask) != 0 {
                crc[square] = SquareState::BlackKing;
            }
            if (self.position.black.queens & mask) != 0 {
                crc[square] = SquareState::BlackQueen;
            }
            if (self.position.black.knights & mask) != 0 {
                crc[square] = SquareState::BlackKnight;
            }
            if (self.position.black.bishops & mask) != 0 {
                crc[square] = SquareState::BlackBishop;
            }
            if (self.position.black.rooks & mask) != 0 {
                crc[square] = SquareState::BlackRook;
            }
            if (self.position.black.pawns & mask) != 0 {
                crc[square] = SquareState::BlackPawn;
            }
        }

        return crc;
    }
}
