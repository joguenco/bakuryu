CREATE TABLE access_tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL,
    status boolean DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_token UNIQUE (token)
);

INSERT INTO access_tokens (token, status) VALUES('Secret', true);
