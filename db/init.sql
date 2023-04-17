-- create a table
CREATE TABLE test(
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    content TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE
);

-- add test data
INSERT INTO
    test (content, archived)
VALUES
    ('test row 1', true),
    ('test row 2', false);

CREATE TABLE message(
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    session_id TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE memory(
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role TEXT NOT NULL DEFAULT,
    content TEXT NOT NULL,
    session_id TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE
);