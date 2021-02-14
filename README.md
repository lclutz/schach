# Schach

![CI](https://github.com/lclutz/schach/workflows/CI/badge.svg?branch=master)

Toy chess programm.

## Prerequisits

- [SDL2](https://www.libsdl.org/)

## Compile

### Windows + MSVC

```console
> .\script\bootstrap.ps1    # Automatically download SDL
> .\script\setup.ps1        # Initial build / clean rebuild
> .\script\update.ps1       # Incremental build
```

### Linux + gcc or clang

```console
$ ./script/setup    # Initial build / clean rebuild
$ ./script/update   # Incremental build
```
