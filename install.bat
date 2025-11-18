@echo off
setlocal enabledelayedexpansion

echo ================================================
echo    ResMan TUI - Resource Monitor Installer
echo ================================================
echo.

:: Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo WARNING: Not running as administrator.
    echo Some features may not work properly.
    echo.
)

:: Check if Rust is installed
echo [1/4] Checking for Rust installation...
where cargo >nul 2>&1
if %errorLevel% neq 0 (
    echo.
    echo Rust is not installed!
    echo.
    echo Installing Rust...
    echo Please follow the installer instructions.
    echo After installation, you may need to restart this script.
    echo.
    pause

    :: Download and run rustup installer
    powershell -Command "Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile '%TEMP%\rustup-init.exe'"
    %TEMP%\rustup-init.exe

    echo.
    echo Rust installation complete!
    echo Please restart your terminal and run this installer again.
    echo.
    pause
    exit /b 0
) else (
    cargo --version
    echo Rust is already installed!
)

echo.
echo [2/4] Building ResMan TUI...
echo This may take a few minutes on first build...
echo.

:: Ask user if they want GPU support
set /p GPU_SUPPORT="Do you want GPU support (NVIDIA only)? (y/n): "
if /i "%GPU_SUPPORT%"=="y" (
    echo Building with NVIDIA GPU support...
    cargo build --release --features gpu-nvidia
    if %errorLevel% neq 0 (
        echo.
        echo ERROR: Build failed!
        echo Make sure you have CUDA Toolkit installed for GPU support.
        echo Download from: https://developer.nvidia.com/cuda-downloads
        echo.
        echo Falling back to build without GPU support...
        cargo build --release
        if %errorLevel% neq 0 (
            echo.
            echo ERROR: Build failed! Please check the error messages above.
            pause
            exit /b 1
        )
    )
) else (
    echo Building without GPU support...
    cargo build --release
    if %errorLevel% neq 0 (
        echo.
        echo ERROR: Build failed! Please check the error messages above.
        pause
        exit /b 1
    )
)

echo.
echo [3/4] Installing ResMan globally...
echo.

if /i "%GPU_SUPPORT%"=="y" (
    cargo install --path . --features gpu-nvidia --force
) else (
    cargo install --path . --force
)

if %errorLevel% neq 0 (
    echo.
    echo ERROR: Installation failed!
    pause
    exit /b 1
)

echo.
echo [4/4] Verifying installation...
echo.

:: Check if resmon is in PATH
where resmon >nul 2>&1
if %errorLevel% neq 0 (
    echo.
    echo WARNING: 'resmon' not found in PATH!
    echo.
    echo Please add the following to your PATH:
    echo %USERPROFILE%\.cargo\bin
    echo.
    echo To add to PATH:
    echo 1. Press Win + X, select "System"
    echo 2. Click "Advanced system settings"
    echo 3. Click "Environment Variables"
    echo 4. Under "User variables", select "Path" and click "Edit"
    echo 5. Click "New" and add: %USERPROFILE%\.cargo\bin
    echo 6. Click OK on all windows
    echo 7. Restart your terminal
    echo.
) else (
    echo Installation successful!
    resmon --version 2>nul || echo resmon installed successfully!
)

echo.
echo ================================================
echo          Installation Complete!
echo ================================================
echo.
echo To run ResMan TUI, simply type:
echo     resmon
echo.
echo Controls:
echo   - Up/Down or j/k   : Navigate processes
echo   - Enter            : View process details
echo   - Esc              : Close details
echo   - q                : Quit
echo.
echo If 'resmon' command is not found, restart your terminal
echo or add %USERPROFILE%\.cargo\bin to your PATH.
echo.
pause
