CREATE TABLE movie (
    id UUID PRIMARY KEY UNIQUE DEFAULT (gen_random_uuid()),
    title TEXT NOT NULL UNIQUE
);