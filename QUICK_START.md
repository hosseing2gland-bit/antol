# 🚀 Quick Start Guide

## راه‌اندازی سریع پروژه

### 1️⃣ Backend (Rust API)

```bash
# نصب Dependencies
cd anti-detect-mvp/backend

# تنظیم Database
export DATABASE_URL="postgres://admin:admin123@localhost/antidetect"

# اجرای Migration ها
sqlx migrate run

# اجرای سرور
cargo run --release
```

✅ Backend روی `http://localhost:3000` اجرا می‌شود

---

### 2️⃣ Admin App (مدیریت سیستم)

```bash
cd anti-detect-mvp/admin-app

# نصب Dependencies
npm install

# اجرا در حالت Development
npm run tauri dev

# یا Build برای Production
npm run tauri build
```

**لاگین پیش‌فرض:**
- Email: `admin@demo.com`
- Password: `admin123`

---

### 3️⃣ Client App (Anti-Detection Browser)

```bash
cd anti-detect-mvp/client-app

# نصب Dependencies
npm install

# اجرا در حالت Development
npm run tauri dev

# یا Build برای Production
npm run tauri build
```

---

## 🎮 نحوه استفاده

### ایجاد پروفایل جدید:

1. باز کردن Client App
2. کلیک روی "New Profile"
3. نام پروفایل را وارد کنید
4. (اختیاری) پروکسی اضافه کنید
5. کلیک روی "Create Profile"

### راه‌اندازی Browser با Anti-Detection:

1. انتخاب پروفایل
2. کلیک روی دکمه ▶️ "Launch"
3. مرورگر با fingerprint منحصر به فرد باز می‌شود

### تست Anti-Detection:

پس از راه‌اندازی مرورگر، این سایت‌ها را باز کنید:

- https://whoer.net
- https://browserleaks.com
- https://pixelscan.net

هر بار fingerprint متفاوتی خواهید دید! 🎉

---

## 🔧 Build برای Windows

### Client App:
```bash
cd anti-detect-mvp/client-app
npm run tauri build
```
📦 خروجی: `src-tauri/target/release/bundle/nsis/client-app_0.0.0_x64-setup.exe`

### Admin App:
```bash
cd anti-detect-mvp/admin-app
npm run tauri build
```
📦 خروجی: `src-tauri/target/release/bundle/nsis/admin-app_0.0.0_x64-setup.exe`

---

## 🐛 عیب‌یابی

### مشکل: Backend اجرا نمی‌شود
```bash
# چک کردن PostgreSQL
psql -U postgres -c "SELECT version();"

# ایجاد Database
createdb antidetect
```

### مشکل: Client App compile نمی‌شود
```bash
# نصب Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# نصب Tauri CLI
npm install -g @tauri-apps/cli
```

### مشکل: Browser باز نمی‌شود
- مطمئن شوید Chrome یا Chromium نصب است
- Windows: `C:\Program Files\Google\Chrome\Application\chrome.exe`
- macOS: `/Applications/Google Chrome.app`
- Linux: `/usr/bin/google-chrome`

---

## 📚 مستندات بیشتر

- [API Documentation](API_DOCUMENTATION.md)
- [User Guide - Admin](USER_GUIDE_ADMIN.md)
- [User Guide - Client](USER_GUIDE_CLIENT.md)
- [Deployment Checklist](DEPLOYMENT_CHECKLIST.md)

---

**نکته:** برای استفاده کامل، Backend باید در حال اجرا باشد.
