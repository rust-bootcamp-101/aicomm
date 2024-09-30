-- Add migration script here

ALTER TABLE chats ADD COLUMN agents bigint[] NOT NULL DEFAULT '{}';

ALTER TABLE messages ADD COLUMN modified_content TEXT;

CREATE TYPE agent_type AS ENUM ('proxy', 'reply', 'tap');

CREATE TABLE IF NOT EXISTS chat_agents (
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL REFERENCES chats(id),
    name TEXT NOT NULL UNIQUE,
    type agent_type NOT NULL DEFAULT 'reply',
    prompt TEXT NOT NULL,
    args JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
