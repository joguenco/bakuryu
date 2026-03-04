# PostgreSQL

* Create user
```
sudo su - postgres
```
```
psql
```
```
CREATE ROLE bakuryu WITH LOGIN NOSUPERUSER CREATEDB NOCREATEROLE INHERIT NOREPLICATION CONNECTION LIMIT -1 PASSWORD 'b';
```
* Show **pg_hba.conf** path
```
show hba_file;
```
and Ctrl+D to exit psql and postgres user session
* Update pg_hba.conf

Search line
```
local   all             postgres                                peer
```
Add in the next line
```
local   all             bakuryu                                  scram-sha-256
```
Restart postgresql service
```
sudo systemctl restart postgresql
```
* Test mew bakuryu user
```
psql -d postgres -U bakuryu -W
```
* Create database
```
create database bakuryu
```
### For access to database from remote host 
* Edit **postgresql.conf** file in the same directory of **pg_hba.conf** file
* Enable or add: listen_addresses = 'ip server'
* In **pg_hba.conf** add the next line:
```
host    all             bakuryu          remote.host.ip/24.mask.number         trust
```