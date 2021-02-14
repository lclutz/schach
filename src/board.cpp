#include "board.hpp"

const string Board::startFEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

uint_fast8_t Board::pieceFromFENSymbol(char symbol)
{
    switch (symbol)
    {
    case 'k':
        return Piece::King;
    case 'p':
        return Piece::Pawn;
    case 'n':
        return Piece::Knight;
    case 'b':
        return Piece::Bishop;
    case 'r':
        return Piece::Rook;
    case 'q':
        return Piece::Queen;
    default:
        return Piece::None;
    }
}

void Board::loadPositionFromFen(string fen)
{
    size_t file = 0;
    size_t rank = 0;

    for (auto symbol : fen)
    {
        if (symbol == '/')
        {
            file = 0;
            rank++;
        }
        else
        {
            if (isdigit(symbol))
            {
                file += (int)symbol - (int)'0';
            }
            else
            {
                auto color = isupper(symbol) ? Piece::White : Piece::Black;
                auto type = pieceFromFENSymbol((char)tolower(symbol));
                this->square[rank * 8 + file] = color | type;
                file++;
            }
        }
    }
}
