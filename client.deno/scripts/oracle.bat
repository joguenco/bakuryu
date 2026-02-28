@echo off
setlocal

Call :GetDateTime Year Month Day Hour Minute Second
set date_now=%Year%-%Month%-%Day%_%Hour%-%Minute%

echo date_now: %date_now%

expdp system/MySecret0@orcl schemas=demo,demo_data DIRECTORY=backup dumpfile=demo%date_now%.dmp reuse_dumpfiles=y logfile=expdp.log
7z a C:\app\backup\demo%date_now%.7z C:\app\backup\demo%date_now%.dmp

C:\app\backup\client.exe -u C:\app\backup\demo%date_now%.7z
C:\app\backup\client.exe -u C:\app\backup\oracle.bat

echo "Delete uncompressed backup"
del C:\app\backup\demo%date_now%.dmp

echo "Delete older files than 30 days and .7z extension"
forfiles /p "C:\app\backup" /s /m *.7z /d -30 /c "cmd /c del /q @path"

:GetDateTime Year Month Day Hour Minute Second
@echo off & setlocal
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "YY=%dt:~2,2%" & set "YYYY=%dt:~0,4%" & set "MM=%dt:~4,2%" & set "DD=%dt:~6,2%"
set "HH=%dt:~8,2%" & set "Min=%dt:~10,2%" & set "Sec=%dt:~12,2%"
( ENDLOCAL
     IF "%~1" NEQ "" set "%~1=%YYYY%"
     IF "%~2" NEQ "" set "%~2=%MM%"
     IF "%~3" NEQ "" set "%~3=%DD%"
     IF "%~4" NEQ "" set "%~4=%HH%"
     IF "%~5" NEQ "" set "%~5=%Min%"
     IF "%~6" NEQ "" set "%~6=%Sec%"
)
exit /b