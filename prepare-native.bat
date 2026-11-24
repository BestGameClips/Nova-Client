@echo off
cd /d "%~dp0"
node tools/prepare.mjs --decompile
if errorlevel 1 pause
