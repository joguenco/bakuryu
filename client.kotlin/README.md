# Client Kotlin
Client for Bakuryu backup server written in Kotlin.

## Requirements
- Java 21
- Gradle 8.14.4

## Usage
Create .env file in resources directory with the following content:
```
URL_SERVER=http://0.0.0.0:8181
TOKEN_SECRET=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs
```
## Formart
- Check
```
gradle ktfmtCheck
```
- Format
```
gradle ktfmtFormat
```

## Run
- List available commands:
```
gradle run --args="-h"
```
- Upload file:
```
gradle run --args="-u /data/backup.7z"
```
## Build
```
gradle :app:jar
```
## Run jar
- List available commands:
```
java -jar app/build/libs/app.jar -h
```
- Upload file:
```
java -jar app/build/libs/app.jar -u /data/backup.7z
```