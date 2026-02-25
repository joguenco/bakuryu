# Client terminal for Bakuryu

Client terminal for backup upload server

# Usage
Create .env file in root directory with the following content:
```
URL_BACKUP_SERVER=http://0.0.0.0:8181
TOKEN_SECRET=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs
```
# Run the program
```
deno run --allow-read --allow-env --allow-net main.ts --help
```
# Compile
- Windows
deno compile --allow-read --allow-env --allow-net --target x86_64-pc-windows-msvc main.ts
```
- GNU/Linux
```
deno compile --allow-read --allow-env --allow-net --target x86_64-unknown-linux-gnu main.ts
```
- Mac Os
```
deno compile --allow-read --allow-env --allow-net --target aarch64-apple-darwin main.ts
```