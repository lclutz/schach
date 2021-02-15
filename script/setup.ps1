# script/setup.ps1: Initial build and clean rebuild

$ErrorActionPreference = "Stop"
$oldpwd = $PWD
Set-Location $(Join-Path -Path $PSScriptRoot -ChildPath ..)

function vcvarsall {
    $ids = 'Community', 'Professional', 'Enterprise', 'BuildTools' | `
        ForEach-Object { 'Microsoft.VisualStudio.Product.' + $_ }

    $vsPath = &(Join-Path -Path ${env:ProgramFiles(x86)} `
            -ChildPath "\Microsoft Visual Studio\Installer\vswhere.exe") `
            -products $ids -property installationpath

    # TODO: only tested this for BuildTools, not for any version of the full
    # VisualStudio IDE.
    Import-Module (Join-Path $vsPath "Common7\Tools\Microsoft.VisualStudio.DevShell.dll")
    if([System.IntPtr]::Size -eq 4) {
        Enter-VsDevShell -VsInstallPath $vsPath -SkipAutomaticLocation -DevCmdArguments '-arch=x86'
    } else {
        Enter-VsDevShell -VsInstallPath $vsPath -SkipAutomaticLocation -DevCmdArguments '-arch=x64'
    }
}

vcvarsall
if (Test-Path build) {
    Remove-Item -Path build -Force -Recurse
}
New-Item -Path "." -Name "build" -ItemType "directory"
Set-Location build
cmake --configure ..
cmake --build .
Set-Location $oldpwd
