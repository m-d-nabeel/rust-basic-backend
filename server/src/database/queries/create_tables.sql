CREATE TABLE IF NOT EXISTS users
(
    id          SERIAL PRIMARY KEY,
    name        varchar(255) NOT NULL,
    description TEXT,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS messages
(
    id           SERIAL PRIMARY KEY,
    user_id      INTEGER REFERENCES users (id) ON DELETE CASCADE,
    message_text TEXT NOT NULL,
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS channels
(
    id           SERIAL PRIMARY KEY,
    channel_name varchar(255) NOT NULL,
    description  TEXT,
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS channel_messages
(
    id         SERIAL PRIMARY KEY,
    channel_id INTEGER REFERENCES channels (id) ON DELETE CASCADE,
    message_id INTEGER REFERENCES messages (id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

-- Adding indexes for faster joins
CREATE INDEX idx_user_id ON messages (user_id);
CREATE INDEX idx_channel_id ON channel_messages (channel_id);
CREATE INDEX idx_message_id ON channel_messages (message_id);