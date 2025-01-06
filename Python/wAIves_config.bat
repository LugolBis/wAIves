:: Obtention du répertoire courant
set "CURRENT_DIR=%~dp0"

:: Variable stockant le nom du script Powershell
set "SCRIPT_NAME=wAIves_config.ps1"

:: Exécution du script Powershell
PowerShell -NoProfile -ExecutionPolicy Bypass -File "%CURRENT_DIR%%SCRIPT_NAME%"
