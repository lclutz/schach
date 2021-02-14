#ifndef SDL_HELPER_HPP
#define SDL_HELPER_HPP

#include <iostream>

#define SDL_MAIN_HANDLED
#include "SDL.h"

using std::cerr, std::endl;

int sdl(int code)
{
    if (code < 0)
    {
        cerr << "SDL error: " << SDL_GetError() << endl;
        exit(1);
    }
    return code;
}

template <typename T>
T *sdl(T *ptr)
{
    if (ptr == nullptr)
    {
        cerr << "SDL error: " << SDL_GetError() << endl;
        exit(1);
    }
    return ptr;
}

template <typename F>
struct Defer
{
    Defer(F f) : f(f) {}
    ~Defer() { f(); }
    Defer &operator=(Defer) = delete;
    F f;
};

#define CONCAT0(a, b) a##b
#define CONCAT(a, b) CONCAT0(a, b)
#define defer(body) Defer CONCAT(defer, __LINE__)([&]() { body; })

#define RGBA(color) (color & 0xFF000000) >> 24, \
                    (color & 0x00FF0000) >> 16, \
                    (color & 0x0000FF00) >> 8,  \
                    color & 0x000000FF

#endif // SDL_HELPER_HPP
