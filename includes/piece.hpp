#ifndef PIECE_HPP
#define PIECE_HPP

struct Piece {
    static const uint_fast8_t None = 0;
    static const uint_fast8_t King = 1;
    static const uint_fast8_t Queen = 2;
    static const uint_fast8_t Bishop = 3;
    static const uint_fast8_t Knight = 4;
    static const uint_fast8_t Rook = 5;
    static const uint_fast8_t Pawn = 6;

    static const uint_fast8_t White = 0x10;
    static const uint_fast8_t Black = 0x20;
};

#endif // PIECE_HPP
