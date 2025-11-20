# 📚 API Documentation - Anti-Detect Browser Platform

**Base URL:** `http://localhost:3000/api`  
**Version:** 1.0  
**Last Updated:** November 20, 2025

---

## 📋 فهرست مطالب

1. [Authentication](#authentication)
2. [Users Management](#users-management)
3. [Licenses Management](#licenses-management)
4. [Profiles Management](#profiles-management)
5. [Proxies Management](#proxies-management)
6. [Error Handling](#error-handling)
7. [Data Models](#data-models)

---

## 🔐 Authentication

### POST `/api/auth/login`
ورود کاربر و دریافت JWT token

**Request Body:**
```json
{
  "email": "admin@demo.com",
  "password": "admin123"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid",
    "email": "admin@demo.com",
    "role": "admin",
    "is_active": true,
    "created_at": "2025-11-20T10:00:00Z"
  }
}
```

**Errors:**
- `401 Unauthorized`: اطلاعات ورود نادرست
- `403 Forbidden`: حساب غیرفعال است

---

### POST `/api/auth/register`
ثبت‌نام کاربر جدید

**Request Body:**
```json
{
  "email": "newuser@demo.com",
  "password": "securepass123"
}
```

**Response (201 Created):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid",
    "email": "newuser@demo.com",
    "role": "user",
    "is_active": true,
    "created_at": "2025-11-20T10:00:00Z"
  }
}
```

**Errors:**
- `400 Bad Request`: ایمیل قبلاً استفاده شده
- `422 Unprocessable Entity`: اطلاعات نامعتبر

---

## 👥 Users Management

### GET `/api/users`
دریافت لیست تمام کاربران (فقط Admin)

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
[
  {
    "id": "uuid",
    "email": "user@demo.com",
    "role": "user",
    "is_active": true,
    "created_at": "2025-11-20T10:00:00Z"
  }
]
```

---

### POST `/api/users`
ایجاد کاربر جدید (فقط Admin)

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "email": "newuser@demo.com",
  "password": "password123",
  "role": "user"  // "admin" | "user"
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "email": "newuser@demo.com",
  "role": "user",
  "is_active": true,
  "created_at": "2025-11-20T10:00:00Z"
}
```

---

### GET `/api/users/:id`
دریافت اطلاعات یک کاربر

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "email": "user@demo.com",
  "role": "user",
  "is_active": true,
  "created_at": "2025-11-20T10:00:00Z"
}
```

---

### PUT `/api/users/:id`
بروزرسانی کاربر (فقط Admin)

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "email": "updated@demo.com",
  "is_active": false,
  "role": "admin"
}
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "email": "updated@demo.com",
  "role": "admin",
  "is_active": false,
  "created_at": "2025-11-20T10:00:00Z"
}
```

---

### DELETE `/api/users/:id`
حذف کاربر (فقط Admin)

**Headers:**
```
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

## 🔑 Licenses Management

### GET `/api/licenses`
دریافت لیست لایسنس‌ها

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
[
  {
    "id": "uuid",
    "key": "BASIC-2024-DEMO-ACTIVE1",
    "user_id": "uuid",
    "plan": "basic",
    "max_profiles": 5,
    "expires_at": "2026-01-20T10:00:00Z",
    "is_active": true,
    "created_at": "2025-11-01T10:00:00Z",
    "activated_at": "2025-11-01T10:00:00Z"
  }
]
```

---

### POST `/api/licenses`
ایجاد لایسنس جدید (فقط Admin)

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "plan": "Basic",  // "Trial" | "Basic" | "Pro" | "Enterprise"
  "max_profiles": 5,
  "duration_days": 30
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "key": "BASIC-A1B2-C3D4-E5F6",
  "plan": "basic",
  "max_profiles": 5,
  "expires_at": "2025-12-20T10:00:00Z",
  "is_active": true,
  "created_at": "2025-11-20T10:00:00Z"
}
```

---

### GET `/api/licenses/:id`
دریافت اطلاعات یک لایسنس

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "key": "BASIC-2024-DEMO-ACTIVE1",
  "user_id": "uuid",
  "plan": "basic",
  "max_profiles": 5,
  "expires_at": "2026-01-20T10:00:00Z",
  "is_active": true,
  "created_at": "2025-11-01T10:00:00Z",
  "activated_at": "2025-11-01T10:00:00Z"
}
```

---

### POST `/api/licenses/activate/:license_key`
فعال‌سازی لایسنس

**Request Body:**
```json
{
  "hardware_id": "unique-hardware-identifier"
}
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "key": "DEMO-2024-FULL-ACCESS",
  "user_id": "uuid",
  "plan": "pro",
  "max_profiles": 10,
  "expires_at": "2026-05-20T10:00:00Z",
  "is_active": true,
  "activated_at": "2025-11-20T10:00:00Z"
}
```

---

### POST `/api/licenses/:id/revoke`
لغو لایسنس (فقط Admin)

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "message": "License revoked successfully"
}
```

---

## 🎭 Profiles Management

### GET `/api/profiles`
دریافت لیست پروفایل‌ها

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "name": "Windows 10 Chrome",
    "fingerprint": {},
    "proxy_id": "uuid",
    "user_agent": "Mozilla/5.0...",
    "timezone": "America/New_York",
    "locale": "en-US",
    "webgl_vendor": "Google Inc.",
    "webgl_renderer": "ANGLE...",
    "canvas_noise": true,
    "webgl_noise": true,
    "audio_noise": true,
    "is_active": true,
    "created_at": "2025-11-01T10:00:00Z",
    "last_used_at": "2025-11-20T08:00:00Z"
  }
]
```

---

### POST `/api/profiles`
ایجاد پروفایل جدید

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "name": "My New Profile",
  "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)...",
  "timezone": "America/New_York",
  "locale": "en-US",
  "proxy_id": "uuid",  // اختیاری
  "canvas_noise": true,
  "webgl_noise": true,
  "audio_noise": true
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "name": "My New Profile",
  "user_agent": "Mozilla/5.0...",
  "timezone": "America/New_York",
  "locale": "en-US",
  "canvas_noise": true,
  "webgl_noise": true,
  "audio_noise": true,
  "is_active": true,
  "created_at": "2025-11-20T10:00:00Z"
}
```

---

### GET `/api/profiles/:id`
دریافت اطلاعات یک پروفایل

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "name": "Windows 10 Chrome",
  "fingerprint": {...},
  "user_agent": "Mozilla/5.0...",
  "timezone": "America/New_York",
  "locale": "en-US",
  "is_active": true
}
```

---

### PUT `/api/profiles/:id`
بروزرسانی پروفایل

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "name": "Updated Profile Name",
  "proxy_id": "uuid",
  "is_active": false
}
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "name": "Updated Profile Name",
  "is_active": false
}
```

---

### DELETE `/api/profiles/:id`
حذف پروفایل

**Headers:**
```
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

## 🌐 Proxies Management

### GET `/api/proxies`
دریافت لیست پروکسی‌ها

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
[
  {
    "id": "uuid",
    "user_id": "uuid",
    "name": "US Proxy 1",
    "proxy_type": "http",
    "host": "us-proxy1.example.com",
    "port": 8080,
    "username": "proxy_user",
    "is_active": true,
    "last_checked_at": "2025-11-20T09:00:00Z",
    "created_at": "2025-11-01T10:00:00Z"
  }
]
```

---

### POST `/api/proxies`
ایجاد پروکسی جدید

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "name": "My Proxy",
  "proxy_type": "Http",  // "Http" | "Https" | "Socks5"
  "host": "proxy.example.com",
  "port": 8080,
  "username": "user",  // اختیاری
  "password": "pass"   // اختیاری
}
```

**Response (201 Created):**
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "name": "My Proxy",
  "proxy_type": "http",
  "host": "proxy.example.com",
  "port": 8080,
  "is_active": true,
  "created_at": "2025-11-20T10:00:00Z"
}
```

---

### GET `/api/proxies/:id`
دریافت اطلاعات یک پروکسی

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "name": "US Proxy 1",
  "proxy_type": "http",
  "host": "us-proxy1.example.com",
  "port": 8080,
  "is_active": true
}
```

---

### PUT `/api/proxies/:id`
بروزرسانی پروکسی

**Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "name": "Updated Proxy",
  "is_active": false,
  "username": "newuser",
  "password": "newpass"
}
```

**Response (200 OK):**
```json
{
  "id": "uuid",
  "name": "Updated Proxy",
  "is_active": false
}
```

---

### DELETE `/api/proxies/:id`
حذف پروکسی

**Headers:**
```
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

### POST `/api/proxies/:id/test`
تست اتصال پروکسی

**Headers:**
```
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "success": true,
  "ip": "1.2.3.4",
  "country": "US",
  "message": "Proxy is working"
}
```

---

## ⚠️ Error Handling

### Error Response Format

```json
{
  "error": "Error message",
  "details": "Additional details (optional)"
}
```

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | موفقیت‌آمیز |
| 201 | ایجاد شد |
| 204 | بدون محتوا (حذف موفق) |
| 400 | درخواست نادرست |
| 401 | احراز هویت نشده |
| 403 | دسترسی ممنوع |
| 404 | یافت نشد |
| 422 | داده نامعتبر |
| 500 | خطای سرور |

---

## 📊 Data Models

### User
```typescript
interface User {
  id: string;              // UUID
  email: string;           // unique
  role: "admin" | "user";
  is_active: boolean;
  created_at: string;      // ISO 8601
  updated_at: string;      // ISO 8601
}
```

### License
```typescript
interface License {
  id: string;
  key: string;                                    // unique
  user_id?: string;                               // nullable
  plan: "trial" | "basic" | "pro" | "enterprise";
  max_profiles: number;
  expires_at: string;
  is_active: boolean;
  created_at: string;
  activated_at?: string;                          // nullable
}
```

### Profile
```typescript
interface Profile {
  id: string;
  user_id: string;
  name: string;
  fingerprint: object;       // JSON
  proxy_id?: string;         // nullable
  user_agent: string;
  timezone: string;
  locale: string;
  webgl_vendor: string;
  webgl_renderer: string;
  canvas_noise: boolean;
  webgl_noise: boolean;
  audio_noise: boolean;
  is_active: boolean;
  created_at: string;
  last_used_at?: string;     // nullable
}
```

### Proxy
```typescript
interface Proxy {
  id: string;
  user_id: string;
  name: string;
  proxy_type: "http" | "https" | "socks5";
  host: string;
  port: number;
  username?: string;         // nullable
  password?: string;         // nullable (hidden in responses)
  is_active: boolean;
  last_checked_at?: string;  // nullable
  created_at: string;
}
```

---

## 🔒 Authentication & Authorization

### JWT Token
- **Algorithm:** HS256
- **Expiration:** 24 hours
- **Header:** `Authorization: Bearer <token>`

### Role-Based Access Control

| Endpoint | Admin | User |
|----------|-------|------|
| POST /api/users | ✅ | ❌ |
| PUT /api/users/:id | ✅ | ❌ |
| DELETE /api/users/:id | ✅ | ❌ |
| POST /api/licenses | ✅ | ❌ |
| POST /api/licenses/:id/revoke | ✅ | ❌ |
| GET /api/users | ✅ | ❌ |
| All other endpoints | ✅ | ✅ |

---

## 📝 Examples

### Complete Workflow Example

```bash
# 1. Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}'

# Response: {"token":"...","user":{...}}

# 2. Create a License
curl -X POST http://localhost:3000/api/licenses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"plan":"Basic","max_profiles":5,"duration_days":30}'

# 3. Create a Proxy
curl -X POST http://localhost:3000/api/proxies \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"name":"My Proxy","proxy_type":"Http","host":"proxy.com","port":8080}'

# 4. Create a Profile
curl -X POST http://localhost:3000/api/profiles \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"name":"Test Profile","user_agent":"Mozilla/5.0...","timezone":"America/New_York","locale":"en-US"}'

# 5. Get All Profiles
curl -X GET http://localhost:3000/api/profiles \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

## 🚀 Rate Limiting

Currently no rate limiting is implemented. Consider adding it for production:
- 100 requests per minute per IP
- 1000 requests per hour per user

---

## 📞 Support

For API issues or questions:
- GitHub Issues: https://github.com/hosseing2gland-bit/antol/issues
- Email: support@antidetect.demo

---

**Last Updated:** November 20, 2025  
**API Version:** 1.0  
**Maintained by:** Anti-Detect Team
