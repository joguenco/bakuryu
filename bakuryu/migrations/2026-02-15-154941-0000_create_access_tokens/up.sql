CREATE TABLE access_tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL,
    status boolean DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT access_tokens_token_uk UNIQUE (token)
);

COMMENT ON COLUMN access_tokens.token IS 'JWT token used for authentication and authorization.';

CREATE TABLE entities (
    id SERIAL PRIMARY KEY,
    access_token_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    folder_path VARCHAR NOT NULL,
    store_type VARCHAR DEFAULT 'file system',
    observation VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT entities_name_uk UNIQUE (name),
    CONSTRAINT entities_access_token_id_uk UNIQUE (access_token_id, name),
    CONSTRAINT entities_access_tokens_fk FOREIGN KEY (access_token_id) REFERENCES access_tokens(id) ON DELETE CASCADE
);
COMMENT ON TABLE entities IS 'Is one by one associated with an access token.';
COMMENT ON COLUMN entities.name IS 'Is present in JWT token for identification of the entity.';
COMMENT ON COLUMN entities.store_type IS 'file system or S3.';

CREATE TABLE file_details (
    id SERIAL PRIMARY KEY,
    entity_id INTEGER NOT NULL,
    file_name VARCHAR NOT NULL,
    size BIGINT NOT NULL,
    sha256 VARCHAR NOT NULL,
    is_sha256_valid BOOLEAN DEFAULT false,
    is_restored BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT file_details_entities_fk FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE
);
COMMENT ON COLUMN file_details.size IS 'Size of the file in megabytes (MB)';
COMMENT ON COLUMN file_details.is_restored IS 'Whether the file was restored from backup.';

WITH inserted_token AS (
    INSERT INTO access_tokens (token, status) 
    VALUES('eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJyYWRtaW4ucmVzb2x2ZWRvci5kZXYiLCJpYXQiOjE3NzExOTcwNDAsImV4cCI6Mzk5NjEwNDQwMC4wLCJhdWQiOiJyZXNvbHZlZG9yLmRldiIsInN1YiI6ImJ1c3NpbmVzQHJlc29sdmVkb3IuZGV2IiwiY2xpZW50IjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb3JnZSBMdWlzIiwiZW1haWwiOiJqb3JnZWx1aXNAcmVzb2x2ZWRvci5kZXYiLCJyb2xlIjpbImRlbW8iXSwic2VydmljZSI6IlJhZG1pbiJ9.anbHFr1WgMi4l-8FYoneVAmSG7h6LN6xTsD0nWtXBfs', true)
    returning id
)

INSERT INTO entities (name, folder_path, access_token_id) 
VALUES('Jorge Luis', '/data/bakuryu/jorgeluis', (SELECT id FROM inserted_token));
