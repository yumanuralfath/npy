@echo off
setlocal enabledelayedexpansion
chcp 65001 >nul
cls

:: ==========================================
:: npy Interactive CLI Shell Launcher
:: ==========================================

:: Set warna (cyan on black)
color 0B

:: ASCII ART TITLE (NPY)
echo.
echo  ███╗   ██╗██████╗ ██╗   ██╗
echo  ████╗  ██║██╔══██╗╚██╗ ██╔╝
echo  ██╔██╗ ██║██████╔╝ ╚████╔╝ 
echo  ██║╚██╗██║██╔══=╝   ╚██╔╝  
echo  ██║ ╚████║██║        ██║   
echo  ╚═╝  ╚═══╝╚═╝        ╚═╝   
echo --------------------------------------------------------------
echo     Data pipeline and scraper for network pharmacology
echo                          by Yumana
echo --------------------------------------------------------------
echo.

:: Tentukan path exe berdasarkan lokasi BAT ini
set EXE=%~dp0npy.exe

if not exist "%EXE%" (
    echo [ERROR] npy.exe tidak ditemukan di folder ini!
    echo Pastikan file ada di: %~dp0
    pause
    exit /b 1
)

echo Memulai NPY CLI...
echo.

:: Jika Windows Terminal tersedia → gunakan itu
where wt.exe >nul 2>&1
if %errorlevel%==0 (
    wt.exe -w 0 nt -d "%EXE_FOLDER%" powershell -NoExit -Command "& ".\npy.exe --help" %*"
    exit /b 0
)

:: FALLBACK → pakai PowerShell biasa
powershell -NoExit -ExecutionPolicy Bypass ^
  -Command "Set-Location '%EXE_FOLDER%'; chcp 65001 >$null; & ".\npy.exe --help" %*"