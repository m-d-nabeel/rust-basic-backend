CREATE SCHEMA IF NOT EXISTS chatician;
SET search_path TO chatician;

-- The rest of your SQL script follows...
CREATE TABLE IF NOT EXISTS chatician.users
(
    id          SERIAL PRIMARY KEY,
    name        varchar(255) NOT NULL UNIQUE,
    description TEXT,
    created_at  timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chatician.messages
(
    id           SERIAL PRIMARY KEY,
    user_id      INTEGER REFERENCES chatician.users (id) ON DELETE CASCADE,
    message_text TEXT NOT NULL,
    created_at   timestamptz DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS chatician.channels
(
    id           SERIAL PRIMARY KEY,
    channel_name varchar(255) NOT NULL UNIQUE,
    description  TEXT,
    created_by   INTEGER REFERENCES chatician.users (id),
    created_at   timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chatician.channel_messages
(
    id         SERIAL PRIMARY KEY,
    channel_id INTEGER REFERENCES chatician.channels (id) ON DELETE CASCADE,
    message_id INTEGER REFERENCES chatician.messages (id) ON DELETE CASCADE,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- Adding indexes for faster joins
CREATE INDEX IF NOT EXISTS idx_user_id ON chatician.messages (user_id);
CREATE INDEX IF NOT EXISTS idx_channel_id ON chatician.channel_messages (channel_id);
CREATE INDEX IF NOT EXISTS idx_message_id ON chatician.channel_messages (message_id);


-- Insert 'SYSTEM' user if it doesn't exist
INSERT INTO chatician.users (name, description)
SELECT 'SYSTEM', 'SYSTEM USER, FOR DEVELOPMENT CASE'
WHERE NOT EXISTS (SELECT 1 FROM chatician.users WHERE name = 'SYSTEM');

-- Insert 'SYSTEM' channel if it doesn't exist
INSERT INTO chatician.channels (channel_name, description, created_by)
SELECT 'GENERAL', 'General discussions & default channel', u.id
FROM chatician.users u
WHERE name = 'SYSTEM' AND NOT EXISTS (
                SELECT 1
                FROM chatician.channels
                WHERE channel_name = 'GENERAL' AND created_by = u.id
            );

-- SELECT *
-- FROM chatician.channel_messages
--          JOIN chatician.messages c on c.id = channel_messages.message_id
-- WHERE channel_id = 1
-- ORDER BY c.created_at DESC;
--
--
-- INSERT INTO chatician.users (name, description)
-- VALUES ('Nabeel', 'First USER LOL');
--
-- INSERT INTO chatician.channels(channel_name, description, created_by)
-- VALUES ('hello_world', 'first channel created by create_channel', 2)
-- RETURNING id, channel_name,description,created_by,created_at;

