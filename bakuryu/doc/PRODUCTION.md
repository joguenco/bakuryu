# Deploy on production

## Compile
```
cargo build --release
```

## Configure and create database
- [PostgreSQL](./POSTGRES.md)
- Create or alter tables and insert or update data.

## Copy executable in server folder
Copy in folde and create and set .env file.

## Systemd
- Create file:
```
/etc/systemd/system/bakuryu.service
```
- Set this configuration:
```
[Unit]
Description=Bakuryu
Requires=network.target
After=network.target

[Service]
Type=simple
User=joguenco
Group=joguenco
Restart=always
RestartSec=3
WorkingDirectory=/home/joguenco/app/bakuryu
ExecStart=/home/joguenco/app/bakuryu/bakuryu
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=bakuryu

[Install]
WantedBy=multi-user.target
```
- You can now load, enable, start, stop and restart your app by running the following as root.
```
systemctl daemon-reload
systemctl enable bakuryu
systemctl start bakuryu
systemctl stop bakuryu
systemctl restart bakuryu
```

## Nginx
```
server {
    server_name default_server;
    client_max_body_size 2048M;

    location / {
        proxy_pass http://127.0.0.1:8181/;
        proxy_pass_header Server;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

server {
    server_name default_server;
    listen 80;
    client_max_body_size 2048M;
}

```
## Utils
- Change password
```
ALTER USER bakuryu WITH PASSWORD 'newpassword';
```
## Generate tokens
- Use https://github.com/joguenco/Radmin application
- Use PRIVATE_KEY variable in .env file to generate token, for example:
```
curl -X 'POST' \
  'http://localhost:8000/generator' \
  -H 'accept: application/json' \
  -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3Njk5OTkxODksImV4cCI6MTc4NTU1MTE4OSwiYXVkIjoicmVzb2x2ZWRvci5kZXYiLCJzdWIiOiJidXNzaW5lc0ByZXNvbHZlZG9yLmRldiIsImNsaWVudCI6IjEyMzQ1Njc4OTAiLCJuYW1lIjoiSm9yZ2UgTHVpcyIsImVtYWlsIjoiam9yZ2VsdWlzQHJlc29sdmVkb3IuZGV2Iiwicm9sZSI6WyJNYW5hZ2VyIl0sInNlcnZpY2UiOiJSYWRtaW4ifQ.Rg615F3IPVsAusaIqLzCupnTvr7D8KyJOAD-pxJ-DXU' \
  -H 'Content-Type: application/json' \
  -d '{
  "identifier": "1234567890123",
  "name": "Jorge Luis",
  "email": "jorgeluis@resolvedor.dev",
  "expitation_date": "2099-03-05",
  "role": [
    "backup"
  ],
  "private_key": "0123456789qwertyuiopasdfghjklzxcvbnm",
  "issuer": "bakuryu",
  "service": "http://localhost:8181"
}'
```
or access to swagger http://localhost:8000/docs