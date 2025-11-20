-- =====================================================
-- Demo Data Seed Script
-- Anti-Detect Browser Platform
-- =====================================================

BEGIN;

-- پاک کردن داده‌های قبلی (اختیاری)
-- TRUNCATE TABLE users, licenses, profiles, proxies CASCADE;

-- =====================================================
-- 1. USERS (کاربران)
-- =====================================================

-- Admin اصلی (قبلاً ساخته شده)
INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'admin@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'admin',
  true,
  NOW() - INTERVAL '30 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر عادی 1
INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'user1@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  true,
  NOW() - INTERVAL '20 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر عادی 2
INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'user2@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  true,
  NOW() - INTERVAL '15 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر عادی 3
INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'user3@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  false,
  NOW() - INTERVAL '10 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- کاربر تست
INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'test@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'user',
  true,
  NOW() - INTERVAL '5 days',
  NOW()
) ON CONFLICT (email) DO NOTHING;

-- =====================================================
-- 2. LICENSES (لایسنس‌ها)
-- =====================================================

-- License Trial (منقضی شده)
INSERT INTO licenses (id, key, user_id, plan, max_profiles, expires_at, is_active, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'TRIAL-2024-DEMO-EXPIRED',
  (SELECT id FROM users WHERE email = 'user3@demo.com' LIMIT 1),
  'trial',
  2,
  NOW() - INTERVAL '5 days',
  false,
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '15 days'
);

-- License Basic (فعال)
INSERT INTO licenses (id, key, user_id, plan, max_profiles, expires_at, is_active, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'BASIC-2024-DEMO-ACTIVE1',
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'basic',
  5,
  NOW() + INTERVAL '60 days',
  true,
  NOW() - INTERVAL '20 days',
  NOW() - INTERVAL '20 days'
);

-- License Pro (فعال)
INSERT INTO licenses (id, key, user_id, plan, max_profiles, expires_at, is_active, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'PRO-2024-DEMO-ACTIVE2',
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'pro',
  10,
  NOW() + INTERVAL '90 days',
  true,
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '15 days'
);

-- License Enterprise (فعال)
INSERT INTO licenses (id, key, user_id, plan, max_profiles, expires_at, is_active, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'ENTERPRISE-2024-UNLIMITED',
  (SELECT id FROM users WHERE email = 'admin@demo.com' LIMIT 1),
  'enterprise',
  100,
  NOW() + INTERVAL '365 days',
  true,
  NOW() - INTERVAL '30 days',
  NOW() - INTERVAL '30 days'
);

-- License برای آزمایش (فعال، بدون کاربر)
INSERT INTO licenses (id, key, user_id, plan, max_profiles, expires_at, is_active, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'DEMO-2024-FULL-ACCESS',
  NULL,
  'pro',
  10,
  NOW() + INTERVAL '180 days',
  true,
  NOW() - INTERVAL '10 days',
  NULL
);

-- License Basic دیگر
INSERT INTO licenses (id, key, user_id, plan, max_profiles, expires_at, is_active, created_at, activated_at)
VALUES (
  gen_random_uuid(),
  'BASIC-2024-TEST-KEY',
  (SELECT id FROM users WHERE email = 'test@demo.com' LIMIT 1),
  'basic',
  3,
  NOW() + INTERVAL '30 days',
  true,
  NOW() - INTERVAL '5 days',
  NOW() - INTERVAL '5 days'
);

-- =====================================================
-- 3. PROXIES (پروکسی‌ها)
-- =====================================================

-- Proxy 1: HTTP آمریکا
INSERT INTO proxies (id, user_id, name, proxy_type, host, port, username, password, is_active, created_at, last_checked_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'US Proxy 1',
  'http',
  'us-proxy1.example.com',
  8080,
  'proxy_user',
  'proxy_pass',
  true,
  NOW() - INTERVAL '20 days',
  NOW() - INTERVAL '1 hour'
);

-- Proxy 2: SOCKS5 UK
INSERT INTO proxies (id, user_id, name, proxy_type, host, port, username, password, is_active, created_at, last_checked_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'UK Proxy SOCKS5',
  'socks5',
  'uk-proxy.example.com',
  1080,
  'uk_user',
  'uk_pass',
  true,
  NOW() - INTERVAL '18 days',
  NOW() - INTERVAL '30 minutes'
);

-- Proxy 3: HTTPS ژاپن
INSERT INTO proxies (id, user_id, name, proxy_type, host, port, username, password, is_active, created_at, last_checked_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Japan HTTPS Proxy',
  'https',
  'jp-secure.example.com',
  443,
  'japan_user',
  'japan_pass',
  true,
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '2 hours'
);

-- Proxy 4: HTTP آلمان (غیرفعال)
INSERT INTO proxies (id, user_id, name, proxy_type, host, port, username, password, is_active, created_at, last_checked_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Germany Proxy (Offline)',
  'http',
  'de-proxy.example.com',
  8888,
  NULL,
  NULL,
  false,
  NOW() - INTERVAL '12 days',
  NOW() - INTERVAL '5 days'
);

-- Proxy 5: SOCKS5 امارات
INSERT INTO proxies (id, user_id, name, proxy_type, host, port, username, password, is_active, created_at, last_checked_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'admin@demo.com' LIMIT 1),
  'Dubai Premium Proxy',
  'socks5',
  'dubai-vip.example.com',
  1080,
  'dubai_premium',
  'secure_pass_123',
  true,
  NOW() - INTERVAL '25 days',
  NOW() - INTERVAL '10 minutes'
);

-- Proxy 6: HTTP کانادا
INSERT INTO proxies (id, user_id, name, proxy_type, host, port, is_active, created_at, last_checked_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'test@demo.com' LIMIT 1),
  'Canada Free Proxy',
  'http',
  'ca-free.example.com',
  3128,
  true,
  NOW() - INTERVAL '5 days',
  NOW() - INTERVAL '6 hours'
);

-- =====================================================
-- 4. PROFILES (پروفایل‌های مرورگر)
-- =====================================================

-- Profile 1: Windows Chrome
INSERT INTO profiles (id, user_id, name, fingerprint, proxy_id, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'Windows 10 Chrome',
  '{"screen":{"width":1920,"height":1080},"fonts":["Arial","Times New Roman"]}'::jsonb,
  (SELECT id FROM proxies WHERE name = 'US Proxy 1' LIMIT 1),
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
  'America/New_York',
  'en-US',
  'Google Inc.',
  'ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0)',
  true,
  true,
  true,
  true,
  NOW() - INTERVAL '20 days',
  NOW() - INTERVAL '2 hours'
);

-- Profile 2: MacOS Safari
INSERT INTO profiles (id, user_id, name, fingerprint, proxy_id, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'MacOS Safari Profile',
  '{"screen":{"width":2560,"height":1440},"fonts":["Helvetica","SF Pro"]}'::jsonb,
  (SELECT id FROM proxies WHERE name = 'UK Proxy SOCKS5' LIMIT 1),
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 14_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15',
  'Europe/London',
  'en-GB',
  'Apple Inc.',
  'Apple M1',
  true,
  false,
  true,
  true,
  NOW() - INTERVAL '18 days',
  NOW() - INTERVAL '5 days'
);

-- Profile 3: Linux Firefox
INSERT INTO profiles (id, user_id, name, fingerprint, proxy_id, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Linux Firefox Dev',
  '{"screen":{"width":1366,"height":768},"fonts":["Ubuntu","DejaVu Sans"]}'::jsonb,
  (SELECT id FROM proxies WHERE name = 'Japan HTTPS Proxy' LIMIT 1),
  'Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0',
  'Asia/Tokyo',
  'ja-JP',
  'Mesa',
  'Mesa DRI Intel(R) HD Graphics',
  false,
  false,
  false,
  true,
  NOW() - INTERVAL '15 days',
  NOW() - INTERVAL '1 day'
);

-- Profile 4: Windows Edge
INSERT INTO profiles (id, user_id, name, fingerprint, proxy_id, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Windows Edge Business',
  '{"screen":{"width":1440,"height":900},"fonts":["Segoe UI","Arial"]}'::jsonb,
  NULL,
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0',
  'America/Chicago',
  'en-US',
  'Google Inc.',
  'ANGLE (NVIDIA GeForce GTX 1660)',
  true,
  true,
  true,
  true,
  NOW() - INTERVAL '14 days',
  NOW() - INTERVAL '3 hours'
);

-- Profile 5: Android Chrome Mobile
INSERT INTO profiles (id, user_id, name, fingerprint, proxy_id, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'admin@demo.com' LIMIT 1),
  'Android Mobile',
  '{"screen":{"width":412,"height":915},"fonts":["Roboto","Noto Sans"]}'::jsonb,
  (SELECT id FROM proxies WHERE name = 'Dubai Premium Proxy' LIMIT 1),
  'Mozilla/5.0 (Linux; Android 13; SM-S908B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36',
  'Asia/Dubai',
  'ar-AE',
  'ARM',
  'Mali-G78',
  true,
  true,
  false,
  true,
  NOW() - INTERVAL '25 days',
  NOW() - INTERVAL '30 minutes'
);

-- Profile 6: Windows Chrome (بدون پروکسی)
INSERT INTO profiles (id, user_id, name, fingerprint, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user1@demo.com' LIMIT 1),
  'Clean Windows Profile',
  '{"screen":{"width":1920,"height":1080},"fonts":["Arial","Calibri"]}'::jsonb,
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36',
  'America/Los_Angeles',
  'en-US',
  'Google Inc.',
  'ANGLE (AMD Radeon)',
  true,
  true,
  true,
  true,
  NOW() - INTERVAL '10 days',
  NOW() - INTERVAL '1 hour'
);

-- Profile 7: MacOS Chrome
INSERT INTO profiles (id, user_id, name, fingerprint, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'test@demo.com' LIMIT 1),
  'MacOS Test Profile',
  '{"screen":{"width":1680,"height":1050},"fonts":["SF Pro","Helvetica"]}'::jsonb,
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
  'America/New_York',
  'en-US',
  'Apple Inc.',
  'Apple GPU',
  false,
  false,
  false,
  true,
  NOW() - INTERVAL '5 days',
  NOW() - INTERVAL '12 hours'
);

-- Profile 8: Windows Firefox (غیرفعال)
INSERT INTO profiles (id, user_id, name, fingerprint, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'user2@demo.com' LIMIT 1),
  'Old Firefox Profile',
  '{"screen":{"width":1280,"height":720},"fonts":["Arial"]}'::jsonb,
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:115.0) Gecko/20100101 Firefox/115.0',
  'Europe/Paris',
  'fr-FR',
  'Google Inc.',
  'ANGLE',
  true,
  false,
  true,
  false,
  NOW() - INTERVAL '30 days',
  NOW() - INTERVAL '20 days'
);

-- Profile 9: iOS Safari
INSERT INTO profiles (id, user_id, name, fingerprint, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'admin@demo.com' LIMIT 1),
  'iPhone 15 Pro',
  '{"screen":{"width":393,"height":852},"fonts":["SF Pro","Helvetica"]}'::jsonb,
  'Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Mobile/15E148 Safari/604.1',
  'Asia/Dubai',
  'en-AE',
  'Apple Inc.',
  'Apple A17 Pro GPU',
  true,
  true,
  false,
  true,
  NOW() - INTERVAL '8 days',
  NOW() - INTERVAL '4 hours'
);

-- Profile 10: Windows Chrome Incognito Mode Simulation
INSERT INTO profiles (id, user_id, name, fingerprint, user_agent, timezone, locale, webgl_vendor, webgl_renderer, canvas_noise, webgl_noise, audio_noise, is_active, created_at, last_used_at)
VALUES (
  gen_random_uuid(),
  (SELECT id FROM users WHERE email = 'test@demo.com' LIMIT 1),
  'Stealth Mode',
  '{"screen":{"width":1920,"height":1200},"fonts":["Times New Roman","Courier New"]}'::jsonb,
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
  'Europe/Berlin',
  'de-DE',
  'Google Inc.',
  'ANGLE (Intel)',
  true,
  true,
  true,
  true,
  NOW() - INTERVAL '3 days',
  NOW() - INTERVAL '6 hours'
);

COMMIT;

-- =====================================================
-- نمایش خلاصه داده‌های ایجاد شده
-- =====================================================

SELECT '=== کاربران ===' as section;
SELECT email, role, is_active FROM users ORDER BY created_at DESC;

SELECT '=== لایسنس‌ها ===' as section;
SELECT key, plan, max_profiles, is_active, expires_at FROM licenses ORDER BY created_at DESC;

SELECT '=== پروکسی‌ها ===' as section;
SELECT name, proxy_type, host, port, is_active FROM proxies ORDER BY created_at DESC;

SELECT '=== پروفایل‌ها ===' as section;
SELECT name, timezone, locale, is_active FROM profiles ORDER BY created_at DESC;

SELECT '=== آمار کلی ===' as section;
SELECT 
  (SELECT COUNT(*) FROM users) as total_users,
  (SELECT COUNT(*) FROM users WHERE is_active = true) as active_users,
  (SELECT COUNT(*) FROM licenses) as total_licenses,
  (SELECT COUNT(*) FROM licenses WHERE is_active = true) as active_licenses,
  (SELECT COUNT(*) FROM profiles) as total_profiles,
  (SELECT COUNT(*) FROM profiles WHERE is_active = true) as active_profiles,
  (SELECT COUNT(*) FROM proxies) as total_proxies,
  (SELECT COUNT(*) FROM proxies WHERE is_active = true) as active_proxies;

-- =====================================================
-- اطلاعات Login
-- =====================================================
SELECT '=== اطلاعات ورود ===' as info;
SELECT 
  'همه کاربران با password: admin123' as note,
  STRING_AGG(email, ', ') as available_users
FROM users;
