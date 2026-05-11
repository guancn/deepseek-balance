@echo off
setlocal enabledelayedexpansion

echo ============================================
echo  DeepSeek Balance - Windows Build
echo ============================================
echo.

:: Check for Rust
where rustc >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Rust is not installed.
    echo.
    echo Please install Rust first:
    echo   1. Visit https://rustup.rs
    echo   2. Download and run rustup-init.exe
    echo   3. Choose "Standard installation"
    echo   4. Re-open this terminal and run build.bat again
    echo.
    pause
    exit /b 1
)

:: Check for MSVC toolchain (optional, but recommended)
where cl >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [WARNING] MSVC toolchain not found.
    echo    For smallest binary, install Visual Studio Build Tools:
    echo    https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
    echo.
    echo    Alternatively, using GNU toolchain (MinGW):
    echo    rustup default stable-x86_64-pc-windows-gnu
    echo.
)

echo [1/2] Building release binary (optimized for size)...
echo.

cargo build --release 2>&1
if %ERRORLEVEL% neq 0 (
    echo.
    echo [ERROR] Build failed. See above for details.
    pause
    exit /b 1
)

echo.
echo [2/2] Build complete.
echo.

:: Display binary info
set "BINARY=target\release\deepseek-balance.exe"
if exist "%BINARY%" (
    for %%F in ("%BINARY%") do (
        set "size=%%~zF"
        set /a "size_kb=!size!/1024"
        echo   Binary: !size_kb! KB
        echo   Path:   %CD%\%BINARY%
    )
)

echo.
echo   To install: copy %BINARY% to your preferred location
echo   To run:     .\%BINARY%
echo   Auto-start: right-click tray icon -- Settings -- Save (enables auto-start)
echo.
pause
