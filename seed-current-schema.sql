-- =====================================================
-- Demo Data Seed Script (Current Schema)
-- Anti-Detect Browser Platform
-- =====================================================

BEGIN;

-- =====================================================
-- 1. USERS (کاربران)
-- =====================================================

-- Admin اصلی
INSERT INTO users (id, email, password_hash, role, subscription_tier, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'admin@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'admin',
  'enterprise',
  NOW() - INTERVAL '30 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر عادی 1
INSERT INTO users (id, email, password_hash, role, subscription_tier, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'user1@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  'pro',
  NOW() - INTERVAL '20 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر عادی 2
INSERT INTO users (id, email, password_hash, role, subscription_tier, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'user2@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  'basic',
  NOW() - INTERVAL '15 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر تست
INSERT INTO users (id, email, password_hash, role, subscription_tier, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'test@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  'free',
  NOW() - INTERVAL '5 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- =====================================================
-- 2. LICENSES (لایسنس‌ها)
-- =====================================================

-- License Enterprise
INSERT INTO licenses (id, license_key, max_profiles, features, expires_at, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'ENTERPRISE-2024-UNLIMITED',
  100,
  '{"anti_detect": true, "proxy_support": true, "automation": true}'::jsonb,
  NOW() + INTERVAL '365 days',
  NOW() - INTERVAL '30 days',
  NOW() - INTERVAL '30 days'
);

-- License Pro
INSERT INTO licenses (id, license_key, max_profiles, features, expires_at, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'PRO-2024-DEMO-ACTIVE1',
  20,
  '{"anti_detect": true, "proxy_support": true}'::jsonb,
  NOW() + INTERVAL '90 days',
  NOW() - INTERVAL '20 days',
  NOW() - INTERVAL '20 days'
);

-- License Basic
INSERT INTO licenses (id, license_key, max_profiles, features, expires_at, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'BASIC-2024-DEMO-ACTIVE2',
  5,
  '{"anti_detect": true}'::jsonb,
  NOW() + INTERVAL '60 days',
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '15 days'
);

-- License Trial (منقضی شده)
INSERT INTO licenses (id, license_key, max_profiles, features, expires_at, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'TRIAL-2024-DEMO-EXPIRED',
  2,
  '{"anti_detect": false}'::jsonb,
  NOW() - INTERVAL '5 days',
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '15 days'
);

-- License فعال بدون کاربر
INSERT INTO licenses (id, license_key, max_profiles, features, expires_at, created_at)
VALUES (
  gen_random_uuid(),
  'DEMO-2024-FULL-ACCESS',
  10,
  '{"anti_detect": true, "proxy_support": true}'::jsonb,
  NOW() + INTERVAL '180 days',
  NOW() - INTERVAL '10 days'
);

-- =====================================================
-- 3. PROXIES (پروکسی‌ها)
-- =====================================================

-- Proxy 1: HTTP آمریکا
INSERT INTO proxies (id, user_id, name, protocol, host, port, username, password, country, created_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'US Proxy 1',
  'http',
  'us-proxy1.example.com',
  8080,
  'proxy_user',
  'proxy_pass',
  'US',
  NOW() - INTERVAL '20 days'
);

-- Proxy 2: SOCKS5 UK
INSERT INTO proxies (id, user_id, name, protocol, host, port, username, password, country, created_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'UK Proxy SOCKS5',
  'socks5',
  'uk-proxy.example.com',
  1080,
  'uk_user',
  'uk_pass',
  'GB',
  NOW() - INTERVAL '18 days'
);

-- Proxy 3: HTTPS ژاپن
INSERT INTO proxies (id, user_id, name, protocol, host, port, username, password, country, created_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Japan HTTPS Proxy',
  'https',
  'jp-secure.example.com',
  443,
  'japan_user',
  'japan_pass',
  'JP',
  NOW() - INTERVAL '15 days'
);

-- Proxy 4: SOCKS5 امارات
INSERT INTO proxies (id, user_id, name, protocol, host, port, username, password, country, created_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'admin@demo.com' LIMIT 1),
  'Dubai Premium Proxy',
  'socks5',
  'dubai-vip.example.com',
  1080,
  'dubai_premium',
  'secure_pass_123',
  'AE',
  NOW() - INTERVAL '25 days'
);

-- Proxy 5: HTTP کانادا
INSERT INTO proxies (id, user_id, name, protocol, host, port, country, created_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'test@demo.com' LIMIT 1),
  'Canada Free Proxy',
  'http',
  'ca-free.example.com',
  3128,
  'CA',
  NOW() - INTERVAL '5 days'
);

-- =====================================================
-- 4. PROFILES (پروفایل‌های مرورگر)
-- =====================================================

-- Profile 1: Windows Chrome
INSERT INTO profiles (id, user_id, name, fingerprint_config, proxy_id, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'Windows 10 Chrome',
  '{
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "screen": {"width": 1920, "height": 1080},
    "timezone": "America/New_York",
    "locale": "en-US",
    "webgl_vendor": "Google Inc.",
    "webgl_renderer": "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0)",
    "canvas_noise": true,
    "webgl_noise": true,
    "audio_noise": true,
    "fonts": ["Arial", "Times New Roman", "Calibri"]
  }'::jsonb,
  (SELECT id FROM proxies WHERE name = 'US Proxy 1' LIMIT 1),
  'Production profile for e-commerce',
  NOW() - INTERVAL '20 days',
  NOW() - INTERVAL '2 hours',
  NOW() - INTERVAL '2 hours'
);

-- Profile 2: MacOS Safari
INSERT INTO profiles (id, user_id, name, fingerprint_config, proxy_id, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'MacOS Safari Profile',
  '{
    "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    "screen": {"width": 2560, "height": 1440},
    "timezone": "Europe/London",
    "locale": "en-GB",
    "webgl_vendor": "Apple Inc.",
    "webgl_renderer": "Apple M1",
    "canvas_noise": true,
    "webgl_noise": false,
    "audio_noise": true,
    "fonts": ["Helvetica", "SF Pro", "Arial"]
  }'::jsonb,
  (SELECT id FROM proxies WHERE name = 'UK Proxy SOCKS5' LIMIT 1),
  'UK market research',
  NOW() - INTERVAL '18 days',
  NOW() - INTERVAL '5 days',
  NOW() - INTERVAL '5 days'
);

-- Profile 3: Linux Firefox
INSERT INTO profiles (id, user_id, name, fingerprint_config, proxy_id, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Linux Firefox Dev',
  '{
    "user_agent": "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
    "screen": {"width": 1366, "height": 768},
    "timezone": "Asia/Tokyo",
    "locale": "ja-JP",
    "webgl_vendor": "Mesa",
    "webgl_renderer": "Mesa DRI Intel(R) HD Graphics",
    "canvas_noise": false,
    "webgl_noise": false,
    "audio_noise": false,
    "fonts": ["Ubuntu", "DejaVu Sans", "Noto Sans"]
  }'::jsonb,
  (SELECT id FROM proxies WHERE name = 'Japan HTTPS Proxy' LIMIT 1),
  'Development testing',
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '1 day',
  NOW() - INTERVAL '1 day'
);

-- Profile 4: Windows Edge
INSERT INTO profiles (id, user_id, name, fingerprint_config, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Windows Edge Business',
  '{
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
    "screen": {"width": 1440, "height": 900},
    "timezone": "America/Chicago",
    "locale": "en-US",
    "webgl_vendor": "Google Inc.",
    "webgl_renderer": "ANGLE (NVIDIA GeForce GTX 1660)",
    "canvas_noise": true,
    "webgl_noise": true,
    "audio_noise": true,
    "fonts": ["Segoe UI", "Arial", "Verdana"]
  }'::jsonb,
  'Business account management',
  NOW() - INTERVAL '14 days',
  NOW() - INTERVAL '3 hours',
  NOW() - INTERVAL '3 hours'
);

-- Profile 5: Android Chrome Mobile
INSERT INTO profiles (id, user_id, name, fingerprint_config, proxy_id, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'admin@demo.com' LIMIT 1),
  'Android Mobile',
  '{
    "user_agent": "Mozilla/5.0 (Linux; Android 13; SM-S908B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    "screen": {"width": 412, "height": 915},
    "timezone": "Asia/Dubai",
    "locale": "ar-AE",
    "webgl_vendor": "ARM",
    "webgl_renderer": "Mali-G78",
    "canvas_noise": true,
    "webgl_noise": true,
    "audio_noise": false,
    "fonts": ["Roboto", "Noto Sans"]
  }'::jsonb,
  (SELECT id FROM proxies WHERE name = 'Dubai Premium Proxy' LIMIT 1),
  'Mobile testing',
  NOW() - INTERVAL '25 days',
  NOW() - INTERVAL '30 minutes',
  NOW() - INTERVAL '30 minutes'
);

-- Profile 6: Clean Windows
INSERT INTO profiles (id, user_id, name, fingerprint_config, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'Clean Windows Profile',
  '{
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    "screen": {"width": 1920, "height": 1080},
    "timezone": "America/Los_Angeles",
    "locale": "en-US",
    "webgl_vendor": "Google Inc.",
    "webgl_renderer": "ANGLE (AMD Radeon)",
    "canvas_noise": true,
    "webgl_noise": true,
    "audio_noise": true,
    "fonts": ["Arial", "Calibri", "Segoe UI"]
  }'::jsonb,
  'No proxy - direct connection',
  NOW() - INTERVAL '10 days',
  NOW() - INTERVAL '1 hour',
  NOW() - INTERVAL '1 hour'
);

-- Profile 7: MacOS Test
INSERT INTO profiles (id, user_id, name, fingerprint_config, notes, created_at, updated_at, last_used)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'test@demo.com' LIMIT 1),
  'MacOS Test Profile',
  '{
    "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "screen": {"width": 1680, "height": 1050},
    "timezone": "America/New_York",
    "locale": "en-US",
    "webgl_vendor": "Apple Inc.",
    "webgl_renderer": "Apple GPU",
    "canvas_noise": false,
    "webgl_noise": false,
    "audio_noise": false,
    "fonts": ["SF Pro", "Helvetica", "Arial"]
  }'::jsonb,
  'Testing environment',
  NOW() - INTERVAL '5 days',
  NOW() - INTERVAL '12 hours',
  NOW() - INTERVAL '12 hours'
);

COMMIT;

-- =====================================================
-- نمایش خلاصه داده‌های ایجاد شده
-- =====================================================

SELECT '=== کاربران ===' as section;
SELECT email, role, subscription_tier FROM users ORDER BY created_at DESC;

SELECT '=== لایسنس‌ها ===' as section;
SELECT license_key, max_profiles, expires_at FROM licenses ORDER BY created_at DESC;

SELECT '=== پروکسی‌ها ===' as section;
SELECT name, protocol, host, port, country FROM proxies ORDER BY created_at DESC;

SELECT '=== پروفایل‌ها ===' as section;
SELECT name, notes FROM profiles ORDER BY created_at DESC;

SELECT '=== آمار کلی ===' as section;
SELECT 
  (SELECT COUNT(*) FROM users) as total_users,
  (SELECT COUNT(*) FROM licenses) as total_licenses,
  (SELECT COUNT(*) FROM profiles) as total_profiles,
  (SELECT COUNT(*) FROM proxies) as total_proxies;

-- =====================================================
-- اطلاعات Login
-- =====================================================
SELECT '=== اطلاعات ورود ===' as info;
SELECT 
  'همه کاربران با password: admin123' as note,
  STRING_AGG(email, ', ' ORDER BY email) as available_users
FROM users;
