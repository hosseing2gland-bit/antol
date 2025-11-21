# گزارش وضعیت Backend و مراحل باقی‌مانده

## ✅ کارهای انجام شده:

1. **سرور Azure راه‌اندازی شد** - IP: 108.143.173.222
2. **Docker Services فعال** - PostgreSQL, Redis, MinIO
3. **Database Migrations اجرا شد** - جداول ایجاد شدند  
4. **Demo Data اضافه شد** - 5 کاربر، 5 لایسنس، 5 پروکسی، 7 پروفایل
5. **Port 3000 باز شد** - قابل دسترسی از اینترنت
6. **Frontend Build شد** - client-app و admin-app آماده

## ❌ مشکل فعلی:

**Backend Compile Error** - تناقض بین Models و Database Schema

### مشکلات شناسایی شده:

1. **User Model**: فیلد `is_active` وجود ندارد در دیتابیس
2. **Proxy Model**: 
   - در دیتابیس: `protocol` (VARCHAR)
   - در کد: `proxy_type` (ENUM)
3. **Profile Model**: 
   - در دیتابیس: `fingerprint_config` (JSONB)
   - در کد: فیلدهای جداگانه (user_agent, timezone, webgl_vendor, ...)
4. **License Model**: Schema مطابقت ندارد

## 🔧 راه‌حل پیشنهادی:

### گزینه 1: تغییر Backend Code (پیشنهادی ✅)
مزیت: Migration ها نیاز به اجرای مجدد ندارند

### گزینه 2: تغییر Database Schema  
معایب: باید migration ها دوباره نوشته و اجرا شوند

## 📋 مراحل برای Fix کردن:

### مرحله 1: Fix Models (models.rs)
```rust
// ✅ User - Fixed
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole, // VARCHAR in DB
    pub license_key: Option<String>,
    pub hardware_id: Option<String>,
    pub subscription_tier: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ❌ Proxy - Needs Fix
pub struct Proxy {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub protocol: String,  // Not ENUM!
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub country: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ❌ Profile - Needs Fix
pub struct Profile {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub fingerprint_config: serde_json::Value,  // Not separate fields!
    pub proxy_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ❌ License - Needs Fix  
pub struct License {
    pub id: Uuid,
    pub license_key: String,  // Column name is "license_key" not "key"
    pub hardware_id: Option<String>,
    pub max_profiles: i32,
    pub features: Option<serde_json::Value>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub last_validation: Option<DateTime<Utc>>,
}
```

### مرحله 2: Fix Handlers

**handlers/proxies.rs**:
- حذف `.bind(&req.proxy_type)` → `.bind(&req.proxy_type.to_string())`
- حذف فیلد `is_active` از UpdateProxyRequest

**handlers/profiles.rs**:
- حذف فیلدهای webgl_noise, audio_noise, canvas_noise
- استفاده از `fingerprint_config` به جای فیلدهای جداگانه

**handlers/licenses.rs**:
- تغییر `.bind(&license.key)` به `.bind(&license.license_key)`

### مرحله 3: Fix CreateProxyRequest

```rust
pub struct CreateProxyRequest {
    pub name: String,
    pub protocol: String,  // نه ProxyType!
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}
```

## 📊 Database Schema واقعی:

```sql
-- users
id, email, password_hash, role (VARCHAR), license_key, hardware_id, 
subscription_tier, created_at, updated_at

-- licenses  
id, license_key, hardware_id, max_profiles, features (JSONB), 
expires_at, created_at, activated_at, last_validation

-- proxies
id, user_id, name, protocol (VARCHAR), host, port, username, password, 
country, created_at

-- profiles
id, user_id, name, fingerprint_config (JSONB), proxy_id, tags (TEXT[]), 
notes, last_used, created_at, updated_at
```

## 🎯 اطلاعات Login برای تست:

```bash
# کاربران موجود (همه با password: admin123):
- admin@demo.com (Admin)
- user1@demo.com (User - Pro)
- user2@demo.com (User - Basic)
- test@demo.com (User - Free)
- admin@antidetect.local (Admin - اولین کاربر)

# تست API:
curl -X POST http://108.143.173.222:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}'
```

## 📝 مراحل بعدی:

1. ✅ Fix کامل models.rs
2. ✅ Fix تمام handlers
3. ✅ Build روی سرور
4. ⏳ Test API endpoints
5. ⏳ Systemd service
6. ⏳ Build Windows/Mac installers

## 💡 نکات مهم:

- Password همه کاربران: `admin123`
- Hash: `$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW`
- JWT Secret باید در .env تنظیم شود
- CORS برای http://108.143.173.222 باز است
