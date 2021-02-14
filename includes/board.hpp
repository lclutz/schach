#ifndef BOARD_HPP
#define BOARD_HPP

#include <cctype>
#include <cstdint>
#include <string>

#include "piece.hpp"

using std::string;

class Board
{
private:
    uint_fast8_t square[64] = {};
    uint_fast8_t pieceFromFENSymbol(char symbol);

public:
    static const string startFEN;

    uint_fast8_t &operator[](size_t idx)
    {
        return this->square[idx];
    }

    const uint_fast8_t &operator[](size_t idx) const
    {
        return this->square[idx];
    }

    void loadPositionFromFen(string fen);
};

#endif // BOARD_HPP
