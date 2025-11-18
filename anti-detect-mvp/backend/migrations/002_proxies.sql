CREATE TABLE proxies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    protocol VARCHAR(20) NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    country VARCHAR(2),
    created_at TIMESTAMP DEFAULT NOW()
);
