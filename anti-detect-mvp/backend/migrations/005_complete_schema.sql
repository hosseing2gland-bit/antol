-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- User roles enum
CREATE TYPE user_role AS ENUM ('admin', 'user');

-- License plans enum
CREATE TYPE license_plan AS ENUM ('trial', 'basic', 'pro', 'enterprise');

-- Proxy types enum
CREATE TYPE proxy_type AS ENUM ('http', 'https', 'socks5');

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Licenses table
CREATE TABLE licenses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(255) UNIQUE NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    plan license_plan NOT NULL DEFAULT 'trial',
    max_profiles INTEGER NOT NULL DEFAULT 5,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    activated_at TIMESTAMP WITH TIME ZONE
);

-- Proxies table
CREATE TABLE proxies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    proxy_type proxy_type NOT NULL DEFAULT 'http',
    host VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_checked_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Profiles table
CREATE TABLE profiles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    fingerprint JSONB NOT NULL DEFAULT '{}',
    proxy_id UUID REFERENCES proxies(id) ON DELETE SET NULL,
    user_agent TEXT NOT NULL,
    timezone VARCHAR(100) NOT NULL DEFAULT 'America/New_York',
    locale VARCHAR(10) NOT NULL DEFAULT 'en-US',
    webgl_vendor VARCHAR(255) NOT NULL DEFAULT 'Google Inc.',
    webgl_renderer VARCHAR(255) NOT NULL DEFAULT 'ANGLE (Intel HD Graphics 630 Direct3D11 vs_5_0 ps_5_0)',
    canvas_noise BOOLEAN NOT NULL DEFAULT true,
    webgl_noise BOOLEAN NOT NULL DEFAULT true,
    audio_noise BOOLEAN NOT NULL DEFAULT true,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_licenses_key ON licenses(key);
CREATE INDEX idx_licenses_user_id ON licenses(user_id);
CREATE INDEX idx_proxies_user_id ON proxies(user_id);
CREATE INDEX idx_profiles_user_id ON profiles(user_id);
CREATE INDEX idx_profiles_proxy_id ON profiles(proxy_id);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to users table
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default admin user (password: admin123)
INSERT INTO users (email, password_hash, role) VALUES (
    'admin@antidetect.local',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lk7NQ6ZxLzDy',
    'admin'
);
