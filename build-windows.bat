@echo off
echo ========================================
echo   Anti-Detect Browser - Windows Build
echo ========================================
echo.

REM Check Node.js
echo [1/5] Checking Node.js...
node --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Node.js not found! Install from https://nodejs.org/
    pause
    exit /b 1
)
echo [OK] Node.js installed

REM Check Rust
echo [2/5] Checking Rust...
cargo --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Rust not found! Install from https://rustup.rs/
    pause
    exit /b 1
)
echo [OK] Rust installed

echo.
echo ========================================
echo   Building Client App
echo ========================================
cd anti-detect-mvp\client-app

echo [3/5] Installing dependencies...
call npm install
if errorlevel 1 (
    echo [ERROR] npm install failed
    pause
    exit /b 1
)

echo [4/5] Building Tauri app...
call npm run tauri build
if errorlevel 1 (
    echo [ERROR] Build failed
    pause
    exit /b 1
)

echo.
echo ========================================
echo   Building Admin App
echo ========================================
cd ..\admin-app

echo [3/5] Installing dependencies...
call npm install
if errorlevel 1 (
    echo [ERROR] npm install failed
    pause
    exit /b 1
)

echo [5/5] Building Tauri app...
call npm run tauri build
if errorlevel 1 (
    echo [ERROR] Build failed
    pause
    exit /b 1
)

echo.
echo ========================================
echo   Build Complete!
echo ========================================
echo.
echo Client App Installer:
echo   client-app\src-tauri\target\release\bundle\msi\
echo.
echo Admin App Installer:
echo   admin-app\src-tauri\target\release\bundle\msi\
echo.
pause
