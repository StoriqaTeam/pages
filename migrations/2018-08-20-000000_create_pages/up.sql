CREATE TABLE pages (
    id UUID PRIMARY KEY,
    slug VARCHAR UNIQUE NOT NULL,
    html VARCHAR NOT NULL,
    css VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
