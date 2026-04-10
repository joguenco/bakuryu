# Configuration 
Steps for connect the main server to the backup server

1. Get the PRIVATE_KEY from the backup server, this variable is set in .env, for example:
```
PRIVATE_KEY=0123456789qwertyuiopasdfghjklzxcvbnm
```

2. Visit the https://jwt.resolvedor.dev and set the information in the form:
- Identifier (VAT identification)
- Name (Friendly name of client or main server)
- Email
- Role (Backup for client or Super for administrator)
- Expiration Date
- Audience (URL server)
- Issuer (Service name)
- Private Key (PRIVATE_KEY variable of the server)

3. Click in Generate button and Copy button, then create .env file in main server on backup folder path with URL_SERVER and TOKEN_SECRET variables, such as in README.md of this project

4. Connect to bakuryu database and set the TOKEN_SECRET
- Connect direct to database or connect to remote database with port forwarding
```
ssh -L 5432:127.0.0.1:5432 -C -N -l root 192.168.1.18
```
5. Execute the next function
- p_name -> Entity name
- p_token -> TOKEN_SECRET
- p_path -> Folder path where the backup will be saved
```
select fun_set_client(
'Entity name', 
'TOKEN_SECRET', 
'/data/folder');
```
6. Generate the executable
- For GNU/Linux
```
deno run build_linux
```
- For Windows
```
deno run build_windows
```
7. Copy the executable to the main server on backup folder path
8. In the main sever, execute
- For show option commands
```
./client --help
```
- Ping
```
./client -p
```
- Get server information
```
./client -s
```
- Upload file
```
./client -u /data/bak/file.7z
```
9. Create .sh or .bat script such as in scripts folder