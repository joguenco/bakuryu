#!/bin/sh

date_now=$(date +"%Y-%m-%d_%H-%M")
export URL_SERVER=http://0.0.0.0:8181
export TOKEN_SECRET=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs

su - oracle -c "expdp system/MySecret0 schemas=demo,demo_data DIRECTORY=backup DUMPFILE=demo.$date_now.dmp LOGFILE=backup.log"
su - oracle -c "7za a /home/oracle/backup/demo.$date_now.dmp.7z /home/oracle/backup/demo.$date_now.dmp"

su - oracle -c "/home/oracle/backup/client -u /home/oracle/backup/demo.$date_now.dmp.7z"
su - oracle -c "/home/oracle/backup/client -u /home/oracle/backup/oracle.bash"

find /home/oracle/backup/ -mtime +20 -type f -name '*.dmp' -delete
find /home/oracle/backup/ -mtime +20 -type f -name '*.7z' -delete
