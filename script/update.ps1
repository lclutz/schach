# script/update.ps1: Incremental build

$ErrorActionPreference = "Stop"
$oldpwd = $PWD
Set-Location $(Join-Path -Path $PSScriptRoot -ChildPath ..)

Set-Location .\build
cmake --build .
Set-Location $oldpwd
