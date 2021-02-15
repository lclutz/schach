# Schach

![CI](https://github.com/lclutz/schach/workflows/CI/badge.svg?branch=master)

Toy chess programm.

## Prerequisits

- [cmake](https://cmake.org/download/)
- [SDL2](https://www.libsdl.org/)
- A C++ compiler

## Compile

### Windows + MSVC

```console
> .\script\bootstrap.ps1    # Automatically download SDL
> .\script\setup.ps1        # Initial build / clean rebuild
> .\script\update.ps1       # Incremental build
```

### Linux + gcc or clang / MacOS + clang

```console
$ ./script/bootstrap    # Install prerequisits
$ ./script/setup        # Initial build / clean rebuild
$ ./script/update       # Incremental build
```
