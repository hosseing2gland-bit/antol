CREATE TABLE licenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    license_key VARCHAR(255) UNIQUE NOT NULL,
    hardware_id VARCHAR(255),
    max_profiles INTEGER NOT NULL DEFAULT 10,
    features JSONB,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    activated_at TIMESTAMP,
    last_validation TIMESTAMP
);
