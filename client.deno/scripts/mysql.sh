#!/bin/sh

date_now=$(date +"%Y-%m-%d_%H-%M")
export URL_SERVER=http://0.0.0.0:8181
export TOKEN_SECRET=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs

mysqldump -uroot -pSecret123 mydb > /data/app/bak/mydb$date_now.sql

7z a /data/app/bak/mydb$date_now.sql.7z /data/app/bak/mydb$date_now.sql

/data/app/bak/client -u /data/app/bak/mydb$date_now.sql.7z
/data/app/bak/client -u /data/app/bak/mysql.bash

find /data/app/bak -mtime +18 -type f ! -name 'backup.sh' -delete