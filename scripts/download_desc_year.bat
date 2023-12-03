@echo off
setlocal enabledelayedexpansion

:: Check if directory and file names are provided
if "%~1"=="" (
    echo Year number not provided.
    exit /b 1
)

if not exist "aoc_%~1" (
    echo Directory "aoc_%~1" does not exist.
    exit /b 1
)

mkdir ".\aoc_%~1\docs"

:: Loop from 01 to 25 for the second argument
for /l %%i in (1, 1, 25) do (
    echo Downloading %%i
    SET "PADDED_NUM=%%i"
    if %%i lss 10 SET "PADDED_NUM=0%%i"
    SET OUTPUT=".\aoc_%~1\docs\day!PADDED_NUM!.md"
    python %~dp0\download_desc.py %~1 !PADDED_NUM! %~2 > !OUTPUT!
)

endlocal
