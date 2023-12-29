SET SEARCH_PATH TO public;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE TABLE IF NOT EXISTS users
-- (
--     id          UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
--     name        varchar(255) NOT NULL UNIQUE,
--     description TEXT,
--     created_at  TIMESTAMP
--                     WITH TIME ZONE                DEFAULT NOW()
-- );
--
-- CREATE TABLE IF NOT EXISTS messages
-- (
--     id           UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
--     user_id      UUID REFERENCES users (id) ON DELETE CASCADE,
--     message_text TEXT NOT NULL,
--     created_at   TIMESTAMP
--                      WITH TIME ZONE        DEFAULT NOW()
-- );


-- CREATE TABLE IF NOT EXISTS channels
-- (
--     id           UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
--     channel_name varchar(255) NOT NULL UNIQUE,
--     description  TEXT,
--     created_by   UUID REFERENCES users (id),
--     created_at   TIMESTAMP
--                      WITH TIME ZONE                DEFAULT NOW()
-- );
--
-- CREATE TABLE IF NOT EXISTS channel_messages
-- (
--     id         UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
--     channel_id UUID REFERENCES channels (id) ON DELETE CASCADE,
--     message_id UUID REFERENCES messages (id) ON DELETE CASCADE,
--     created_at TIMESTAMP
--                    WITH TIME ZONE        DEFAULT NOW()
-- );

-- -- Adding indexes for faster joins
-- CREATE INDEX IF NOT EXISTS idx_user_id ON messages (user_id);
-- CREATE INDEX IF NOT EXISTS idx_channel_id ON channel_messages (channel_id);
-- CREATE INDEX IF NOT EXISTS idx_message_id ON channel_messages (message_id);


-- -- Insert 'SYSTEM' user if it doesn't exist
-- INSERT INTO users (name, description)
-- SELECT 'SYSTEM', 'SYSTEM USER, FOR DEVELOPMENT CASE'
-- WHERE NOT EXISTS (SELECT 1 FROM users WHERE name = 'SYSTEM');
--
-- -- Insert 'SYSTEM' channel if it doesn't exist
-- INSERT INTO channels (channel_name, description, created_by)
-- SELECT 'GENERAL', 'General discussions & default channel', u.id
-- FROM users u
-- WHERE name = 'SYSTEM'
--   AND NOT EXISTS (SELECT 1
--                   FROM channels
--                   WHERE channel_name = 'GENERAL'
--                     AND created_by = u.id);

-- SELECT *
-- FROM channel_messages
--          JOIN messages c on c.id = channel_messages.message_id
-- WHERE channel_id = 1
-- ORDER BY c.created_at DESC;
--
--
-- INSERT INTO users (name, description)
-- VALUES ('Nabeel', 'First USER LOL');
--
-- INSERT INTO channels(channel_name, description, created_by)
-- VALUES ('hello_world', 'first channel created by create_channel', 2)
-- RETURNING id, channel_name,description,created_by,created_at;


-- REAL USER
-- SELECT * FROM pg_available_extensions WHERE name = 'uuid-ossp';
-- -- generate universally unique identifiers (UUIDs)

CREATE TABLE IF NOT EXISTS real_user
(
    id         UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name       VARCHAR(100) NOT NULL,
    email      VARCHAR(255) NOT NULL UNIQUE,
    password   VARCHAR(100) NOT NULL,
    created_at TIMESTAMP
                   WITH TIME ZONE                DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS users_email_idx ON real_user (email);