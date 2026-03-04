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