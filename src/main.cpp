#define SDL_MAIN_HANDLED

#include <iostream>
#include <SDL.h>

#include "board.hpp"
#include "helper.hpp"

constexpr int WIDTH = 800;
constexpr int HEIGHT = WIDTH;
constexpr uint32_t WINDOW_BG = 0x181818FF;
constexpr uint32_t DARK_SQUARE_BG = 0xb58863FF;
constexpr uint32_t LIGHT_SQUARE_BG = 0xf0d9b5FF;

void render_board(SDL_Renderer *renderer)
{
    sdl(SDL_SetRenderDrawColor(renderer, RGBA(DARK_SQUARE_BG)));
    SDL_Rect rect = {0, 0, WIDTH, HEIGHT};
    sdl(SDL_RenderFillRect(renderer, &rect));

    sdl(SDL_SetRenderDrawColor(renderer, RGBA(LIGHT_SQUARE_BG)));
    for (int rank = 0; rank < 8; rank++)
    {
        for (int file = 0; file < 8; file++)
        {
            if ((rank + file) % 2 == 0)
            {
                SDL_Rect light_square = {
                    rank * (WIDTH / 8),
                    file * (HEIGHT / 8),
                    WIDTH / 8,
                    HEIGHT / 8};

                sdl(SDL_RenderFillRect(renderer, &light_square));
            }
        }
    }
}

void render_pieces(SDL_Renderer *renderer,
                   const Board &board,
                   SDL_Texture *texture)
{
    for (size_t square = 0; square < 64; square++)
    {
        if (board[square] == 0)
            continue;

        SDL_Rect source_rect = {
            ((board[square] & 0x0F) - 1) * 334,
            (board[square] & 0xF0) == Piece::White ? 0 : 334,
            334, 334};

        SDL_Rect dest_rect = {
            (int)(square % 8) * (WIDTH / 8),
            (int)(square / 8) * (HEIGHT / 8),
            WIDTH / 8,
            HEIGHT / 8};

        sdl(SDL_RenderCopy(renderer, texture, &source_rect, &dest_rect));
    }
}

void render(SDL_Renderer *renderer, const Board &board, SDL_Texture *texture)
{
    sdl(SDL_SetRenderDrawColor(renderer, RGBA(WINDOW_BG)));
    sdl(SDL_RenderClear(renderer));

    render_board(renderer);
    render_pieces(renderer, board, texture);

    SDL_RenderPresent(renderer);
}

int main()
{
    sdl(SDL_Init(SDL_INIT_VIDEO));
    defer(SDL_Quit());

    auto *window = sdl(SDL_CreateWindow("Schach",
                                        SDL_WINDOWPOS_CENTERED,
                                        SDL_WINDOWPOS_CENTERED,
                                        WIDTH, HEIGHT,
                                        SDL_WINDOW_RESIZABLE));
    defer(SDL_DestroyWindow(window));

    auto *renderer = sdl(SDL_CreateRenderer(window, -1,
                                            SDL_RENDERER_ACCELERATED));
    defer(SDL_DestroyRenderer(renderer));

    sdl(SDL_RenderSetLogicalSize(renderer, WIDTH, HEIGHT));
    SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "best");

    SDL_Surface *pieces_surface = sdl(SDL_LoadBMP("assets/pieces.bmp"));
    defer(SDL_FreeSurface(pieces_surface));

    SDL_Texture *pieces_texture = SDL_CreateTextureFromSurface(renderer,
                                                               pieces_surface);
    defer(SDL_DestroyTexture(pieces_texture));

    Board board;
    board.loadPositionFromFen(Board::startFEN);

    render(renderer, board, pieces_texture);

    bool running = true;
    while (running)
    {
        SDL_Event event;
        SDL_WaitEvent(&event);
        switch (event.type)
        {
        case SDL_QUIT:
            running = false;
            break;
        case SDL_WINDOWEVENT:
            switch (event.window.event)
            {
            case SDL_WINDOWEVENT_RESIZED:
                render(renderer, board, pieces_texture);
                break;
            default:
                break;
            }
            break;
        default:
            break;
        }
    }

    return 0;
}
