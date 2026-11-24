@echo off
setlocal
cd /d "%~dp0"
set "JAR=%~dp0nova-client.jar"
if not exist "%JAR%" set "JAR=%~dp0build\libs\nova-client.jar"
if not exist "%JAR%" (
  echo nova-client.jar introuvable. Recupere l'artefact GitHub Actions.
  pause
  exit /b 1
)
if not defined APPDATA exit /b 1
set "MODS=%APPDATA%\.minecraft\mods"
if not exist "%MODS%" mkdir "%MODS%"
copy /Y "%JAR%" "%MODS%\nova-client.jar" >nul || exit /b 1
echo Nova installe. Choisis le profil Fabric 26.3 dans le launcher officiel.
start "" "minecraft://"
pause
