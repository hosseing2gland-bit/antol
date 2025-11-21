# Anti-Detect Browser MVP

## راه‌اندازی اتصال به سرور اختصاصی

۱. فایل `.env` را در ریشه پروژه و پوشه‌های client-app و admin-app قرار دهید:

```
# ریشه پروژه
API_URL=http://108.143.173.222:8000
POSTGRES_HOST=108.143.173.222
POSTGRES_PORT=5432
POSTGRES_DB=antol
POSTGRES_USER=admin
POSTGRES_PASSWORD=ABCDqwer1234
REDIS_HOST=108.143.173.222
REDIS_PORT=6379
REDIS_PASSWORD=ABCDqwer1234
MINIO_HOST=108.143.173.222
MINIO_PORT=9000
MINIO_ACCESS_KEY=admin
MINIO_SECRET_KEY=ABCDqwer1234

# client-app و admin-app
VITE_API_URL=http://108.143.173.222:8000
```

۲. مطمئن شوید backend روی سرور اجرا شده و دیتابیس و سرویس‌ها فعال هستند.

۳. کلاینت و ادمین را build و اجرا کنید تا به سرور متصل شوند.

۴. برای تست اتصال، یک درخواست login یا دریافت پروفایل ارسال کنید.

---

یک سیستم کامل مدیریت مرورگر Anti-Detect با قابلیت‌های حرفه‌ای

## 🎯 ویژگی‌های اصلی

### ✅ Anti-Detection Features
- Canvas Fingerprint Noise
- WebGL Fingerprint Spoofing  
- Audio Context Noise
- User Agent Randomization
- Hardware Spoofing (CPU, RAM)
- Screen Resolution Spoofing
- Timezone & Language
- WebRTC Leak Protection
- Fake Media Devices
- Client Rects Noise
- Battery API Spoofing

### Backend (Rust + Axum)
- Authentication (JWT)
- User Management
- License System
- Profile Management
- Proxy Management
- PostgreSQL + Redis + MinIO

### Desktop Apps (Tauri + React)
- Admin App - پنل مدیریت
- Client App - اپلیکیشن کاربر با Anti-Detection

## 🚀 Quick Start

### Backend
\`\`\`bash
cd anti-detect-mvp/backend
export DATABASE_URL="postgres://admin:admin123@localhost/antidetect"
cargo run --release
\`\`\`

### Client App
\`\`\`bash
cd anti-detect-mvp/client-app
npm install
npm run tauri dev
\`\`\`

## 📝 Documentation

- API_DOCUMENTATION.md
- USER_GUIDE_ADMIN.md
- USER_GUIDE_CLIENT.md
- DEPLOYMENT_CHECKLIST.md
- NEXT_STEPS.md

## 📈 Status: 85% Complete

✅ Backend API
✅ Admin App
✅ Client App  
✅ Anti-Detection Core
✅ Browser Launcher
⏳ Custom Chromium
⏳ Production Deploy
