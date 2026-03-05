#!/bin/sh

date_now=$(date +"%Y-%m-%d_%H-%M")

su - oracle -c "expdp system/MySecret0 schemas=demo,demo_data DIRECTORY=backup DUMPFILE=demo.$date_now.dmp LOGFILE=backup.log"
su - oracle -c "7za a /home/oracle/backup/demo.$date_now.dmp.7z /home/oracle/backup/demo.$date_now.dmp"

find /home/oracle/backup/ -mtime +20 -type f -name '*.dmp' -delete
