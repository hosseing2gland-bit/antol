# 📝 Changelog

همه تغییرات مهم این پروژه در این فایل ثبت می‌شود.

## [0.2.0] - 2025-11-21

### ✨ افزوده شده
- **Anti-Detection Core Module** - پیاده‌سازی کامل سیستم ضد شناسایی
  - Canvas Fingerprint Noise
  - WebGL Fingerprint Spoofing
  - Audio Context Noise
  - Hardware Spoofing (CPU, RAM, Touch)
  - Screen Resolution Spoofing
  - User Agent Randomization
  - Timezone & Language Spoofing
  - WebRTC Leak Protection
  - Fake Media Devices
  - Client Rects Noise
  - Battery API Spoofing
  - Font Fingerprint Protection

- **Tauri Commands** - 5 command جدید برای مدیریت browser
  - `generate_fingerprint()` - تولید fingerprint تصادفی
  - `launch_browser()` - راه‌اندازی browser با fingerprint
  - `stop_browser()` - متوقف کردن browser
  - `get_active_browsers()` - لیست browser های فعال
  - `stop_all_browsers()` - متوقف کردن همه

- **UI Components** - کامپوننت‌های جدید React
  - `BrowserLauncher.tsx` - Hook برای مدیریت browser
  - `FingerprintEditor.tsx` - ویرایش fingerprint
  - `FingerprintGenerator.tsx` - نمایش fingerprint تولید شده

- **Documentation** - مستندات کامل
  - `QUICK_START.md` - راهنمای شروع سریع
  - `FEATURES.md` - توضیح کامل قابلیت‌ها
  - `CHANGELOG.md` - تاریخچه تغییرات
  - `test-fingerprint.html` - صفحه تست fingerprint

### 🔧 بهبود یافته
- `Profiles.tsx` - پشتیبانی از browser launch با fingerprint
- `README.md` - بازنویسی کامل با اطلاعات به‌روز
- Project structure - پاکسازی فایل‌های غیرضروری

### 🗑️ حذف شده
- فایل‌های build قدیمی (200MB+)
- Documentation تکراری (9 فایل)
- Build caches (2.4GB)
- Log files و temporary files

### 🐛 رفع شده
- مشکل port در Tauri config (1420 → 5173)
- مشکل timezone offset در fingerprint generation
- Warnings در Rust code

---

## [0.1.0] - 2025-11-20

### ✨ افزوده شده
- Backend API با Rust + Axum
- Admin App با Tauri + React
- Client App با Tauri + React
- PostgreSQL Database
- Redis Cache
- MinIO Storage
- Authentication System (JWT)
- User Management
- License System
- Profile Management
- Proxy Management

---

## نکات نگارش

فرمت بر اساس [Keep a Changelog](https://keepachangelog.com/)

انواع تغییرات:
- `✨ افزوده شده` - قابلیت‌های جدید
- `🔧 بهبود یافته` - تغییرات در قابلیت‌های موجود
- `🐛 رفع شده` - رفع باگ‌ها
- `🗑️ حذف شده` - حذف قابلیت‌ها
- `🔒 امنیتی` - آسیب‌پذیری‌های رفع شده
- `📝 مستندات` - تغییرات در documentation
