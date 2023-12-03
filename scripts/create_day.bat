@echo off
setlocal

:: Check if directory and file names are provided
if "%~1"=="" (
    echo Year name not provided.
    exit /b 1
)
if "%~2"=="" (
    echo File name not provided.
    exit /b 1
)

:: Set the directory and file names

if not exist "%~1" (
    echo Directory "%~1" does not exist.
    exit /b 1
)

echo D | xcopy "%~dp0\_daytemplate.rs" ".\%1\src\%2.rs" /I /Y
echo "Created ./%1/src/%2.rs"

copy NUL ".\inputs\%1\%2.txt"
echo "Created ./inputs/%1/%2.txt"

endlocal
