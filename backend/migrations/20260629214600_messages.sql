ALTER TABLE vaultchat.messages
RENAME COLUMN content TO receiver_content;

ALTER TABLE vaultchat.messages
ADD COLUMN sender_content TEXT NOT NULL DEFAULT '';
