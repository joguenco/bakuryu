@echo off
setlocal

Call :GetDateTime Year Month Day Hour Minute Second
set date_now=%Year%-%Month%-%Day%_%Hour%-%Minute%
set URL_SERVER=http://0.0.0.0:8181
set TOKEN_SECRET=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs

echo date_now: %date_now%

expdp system/MySecret0@orcl schemas=demo,demo_data DIRECTORY=backup dumpfile=demo%date_now%.dmp reuse_dumpfiles=y logfile=expdp.log
7z a C:\app\backup\demo%date_now%.7z C:\app\backup\demo%date_now%.dmp

C:\app\backup\client.exe -u C:\app\backup\demo%date_now%.7z
C:\app\backup\client.exe -u C:\app\backup\oracle.bat

forfiles /p "C:\app\backup" /s /m *.7z /d -30 /c "cmd /c del /q @path"
forfiles /p "C:\app\backup" /s /m *.dmp /d -30 /c "cmd /c del /q @path"

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