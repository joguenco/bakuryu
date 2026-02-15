CREATE TABLE entity (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    folder_path VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_name UNIQUE (name)
);

CREATE TABLE access_tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL,
    status boolean DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_token UNIQUE (token)
);

INSERT INTO access_tokens (token, status) 
VALUES('eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs', true);
INSERT INTO entity (name, folder_path) 
VALUES('Jorge Luis', '/data/bakuryu/1234567890');
