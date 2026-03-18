#!/bin/sh

date_now=$(date +"%Y-%m-%d_%H-%M")

su - oracle -c "expdp system/MySecret0 schemas=demo,demo_data DIRECTORY=backup DUMPFILE=demo.$date_now.dmp LOGFILE=backup.log"
su - oracle -c "7za a /home/oracle/backup/demo.$date_now.dmp.7z /home/oracle/backup/demo.$date_now.dmp"

su - orale -c "java -jar /home/orale/backup/client.jar -u /home/orale/backup/demo.$date_now.dmp.7z"
su - orale -c "java -jar /home/orale/backup/client.jar -u /home/orale/backup/backup"

find /home/oracle/backup/ -mtime +20 -type f -name '*.dmp' -delete
find /home/oracle/backup/ -mtime +20 -type f -name '*.dmp.7z' -delete
