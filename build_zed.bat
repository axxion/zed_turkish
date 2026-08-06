@echo off
REM AYA Vakfi build environment - sets up MSVC + Rust + CMake
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul
set PATH=C:\Users\ahmet\.cargo\bin;C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin;%PATH%
cd /d D:\projeler\karala\zedturkce\zed
%*
