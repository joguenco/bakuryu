# Bakuryu

Application for upload backup files

## Description
This application is used when a backup of the database is generated on the client's server and then the file is transferred to another backup server using the http protocol.

The authentication method is bearer token, this token is in database.

## Requirements
- Rust https://rust-lang.org/
- [PostgreSQL](./doc/POSTGRES.md)
### Test request
- Httpie cli https://httpie.io/cli
- HTTPie extension for VS Code
In doc folder, use the Request.httpie file for testing or run commands in terminal

## Upgrade Rust
```
rustup update
```
## Code format
```
cargo fmt
```
## Run
```
cargo run
```
## Hot Reload
```
cargo install --locked watchexec-cli
```
```
watchexec -w src -r cargo run
```
## Diesel
Install CLI
```
cargo install diesel_cli --no-default-features --features postgres
```
Create .env file
```
DATABASE_URL=postgres://bakuryu:b@localhost/bakuryu
PRIVATE_KEY=0123456789qwertyuiopasdfghjklzxcvbnm
```
Only when project is created
```
diesel setup
```
Create migration
```
diesel migration generate create_access_tokens
```
Run migration
```
diesel migration run
```
Redo migration
```
diesel migration revert
```

