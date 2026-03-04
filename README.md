# Bakuryu

Application for upload backup files written in Rust, PostgreSQL, Diesel, Actix

## Description
This application is used when a backup of the database is generated on the client's server, and then the file is transferred to another backup server using the http protocol.

The authentication method is bearer token, this token is in database.

# Client Deno
Application for connect to Bakuryu server written in TypeScript with Deno

# Client Kotlin
Application for connect to Bakuryu server written in Kotlin with gradle 
