SET SEARCH_PATH TO public;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Types
CREATE TYPE identity_type AS ENUM ('SYSTEM', 'MEMBER');
CREATE TYPE message_status AS ENUM ('SENT', 'DELIVERED', 'READ');

-- Member
CREATE TABLE IF NOT EXISTS member
(
    id          UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name        VARCHAR(100) NOT NULL,
    email       VARCHAR(255) NOT NULL UNIQUE,
    password    VARCHAR(100) NOT NULL,
    profile_pic VARCHAR(255),
    status      VARCHAR(50),
    created_at  TIMESTAMP
                    WITH TIME ZONE                DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS members_email_idx ON member (email);

-- Message
CREATE TABLE IF NOT EXISTS message
(
    id           UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    identity     identity_type             DEFAULT 'MEMBER',
    sent_from_id UUID REFERENCES member (id),
    sent_to_id   UUID REFERENCES member (id),
    message_text TEXT NOT NULL,
    status       message_status            DEFAULT 'SENT',
    created_at   TIMESTAMP
                     WITH TIME ZONE        DEFAULT NOW()
);

-- Group
CREATE TABLE IF NOT EXISTS message_group
(
    id          UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    description VARCHAR(255),
    creator     UUID REFERENCES member (id) ON DELETE CASCADE,
    created_at  TIMESTAMP
                    WITH TIME ZONE        DEFAULT NOW()
);

-- Group Member
CREATE TABLE IF NOT EXISTS group_member
(
    group_id  UUID REFERENCES message_group (id) ON DELETE CASCADE,
    member_id UUID REFERENCES member (id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, member_id)
);