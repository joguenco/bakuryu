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

INSERT INTO access_tokens (token, status) VALUES('Secret', true);
INSERT INTO entity (name, folder_path) VALUES('Demo', '/home/jorgeluis/tmp');
