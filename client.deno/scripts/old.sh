#!/bin/sh

# Backup with scp and rsync

date_now=$(date +"%Y-%m-%d_%H-%M")

scp -P 8080 /data/bak$date_now.sql.7z user@server.my:/backup/owner
rsync -rvz -e 'ssh -p 8080' --progress /data/files user@server.my:/cloud/owner/files
