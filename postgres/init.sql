CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE SCHEMA IF NOT EXISTS vaultchat;

CREATE TABLE IF NOT EXISTS vaultchat.users (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    pseudo VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL
);

create table if not exists vaultchat.messages (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    sender_id uuid references vaultchat.users(id) on delete cascade,
    receiver_id uuid references vaultchat.users(id) on delete cascade,
    content TEXT not null,
    sended_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

create index if not exists idx_messages_participants on vaultchat.messages(sender_id, receiver_id);