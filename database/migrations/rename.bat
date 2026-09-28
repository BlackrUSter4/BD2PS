@echo off
setlocal enabledelayedexpansion

set counter=1

for %%f in (*.sql) do (
    set "filename=%%f"
    set "padded=000!counter!"
    set "padded=!padded:~-3!"
    ren "%%f" "!padded!_%%f"
    set /a counter+=1
)

echo Done! Renamed all SQL files with numeric prefixes.
pause