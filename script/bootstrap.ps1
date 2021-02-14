# script/bootstrap.ps1: Download 3rd-party libs

$ErrorActionPreference = "Stop"
Set-Location $(Join-Path -Path $(Split-Path -parent $MyInvocation.MyCommand.Definition) -ChildPath ..)

# Download SDL2
if (Test-Path SDL2) {
    Write-Host "SDL2 directory already exists."
}
else {
    $SDL_VERSION = '2.0.12'
    Invoke-WebRequest -OutFile SDL2.zip -Uri http://libsdl.org/release/SDL2-devel-$SDL_VERSION-VC.zip
    Expand-Archive -Path .\SDL2.zip -DestinationPath .
    Move-Item SDL2-$SDL_VERSION SDL2
    Remove-Item SDL2.zip
}
