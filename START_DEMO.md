# 🚀 راهنمای راه‌اندازی کامل برای Demo

## پیش‌نیازها (فقط یک‌بار):
```
✅ Docker Desktop (برای ویندوز)
✅ فایل‌های نصبی: admin-app-setup.exe, client-app-setup.exe
```

---

## مرحله 1: راه‌اندازی Backend (روی یک لپ‌تاپ)

### A. نصب Docker Desktop
1. دانلود از: https://www.docker.com/products/docker-desktop/
2. نصب و اجرا
3. در System Tray باید آیکون Docker سبز باشد

### B. راه‌اندازی Services
```bash
# در PowerShell یا CMD:
cd path\to\anti-detect-mvp
docker-compose up -d

# چک کردن:
docker-compose ps
# باید 3 سرویس UP باشد: postgres, redis, minio
```

### C. راه‌اندازی Backend API

**گزینه 1: استفاده از فایل Executable آماده** (توصیه می‌شود)
```bash
# فایل backend.exe را از artifacts GitHub دانلود کنید
# سپس اجرا کنید:
.\backend.exe
```

**گزینه 2: اگر Rust نصب دارید**
```bash
cd anti-detect-mvp\backend
set DATABASE_URL=postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db
cargo run --release
```

✅ **Backend آماده است!**  
باید پیام مشابه زیر ببینید:
```
Server running on http://0.0.0.0:3000
```

---

## مرحله 2: نصب Apps (روی لپ‌تاپ دیگر یا همان لپ‌تاپ)

### A. نصب Admin App
1. اجرای `admin-app-setup.exe`
2. مراحل نصب را دنبال کنید
3. از Start Menu اجرا کنید: "Anti-Detect Admin"

### B. نصب Client App
1. اجرای `client-app-setup.exe`  
2. مراحل نصب را دنبال کنید
3. از Start Menu اجرا کنید: "Anti-Detect Client"

---

## مرحله 3: راه‌اندازی اولیه (فقط یک‌بار)

### A. ایجاد حساب ادمین
```sql
# در PowerShell:
docker exec -it antidetect_postgres psql -U antidetect_user -d antidetect_db

-- در PostgreSQL:
INSERT INTO users (email, password_hash, role, is_active) 
VALUES ('admin@demo.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5RzbE6bVTQK0W', 'admin', true);
-- Password: admin123

\q
```

### B. ایجاد لایسنس نمونه
```sql
docker exec -it antidetect_postgres psql -U antidetect_user -d antidetect_db

INSERT INTO licenses (license_key, max_devices, expires_at, is_active) 
VALUES ('DEMO-2024-FULL-ACCESS', 5, '2025-12-31', true);

\q
```

---

## مرحله 4: تست کامل

### Test 1: Admin App
```
1. باز کردن Admin App
2. Login با: admin@demo.com / admin123
3. مشاهده Dashboard
4. رفتن به Users → اضافه کردن یک کاربر عادی
5. رفتن به Licenses → مشاهده لایسنس‌ها
6. رفتن به Profiles → مدیریت پروفایل‌ها
```

### Test 2: Client App
```
1. باز کردن Client App
2. ورود لایسنس: DEMO-2024-FULL-ACCESS
3. ثبت‌نام با ایمیل و پسورد
4. Login
5. ایجاد یک پروفایل جدید
6. تنظیم پروکسی (اختیاری)
7. راه‌اندازی Browser با پروفایل
```

---

## 🎯 برای Demo به رئیس:

### سناریو پیشنهادی (15 دقیقه):

**دقیقه 1-3: معرفی**
```
- نشان دادن 2 لپ‌تاپ/صفحه
- یکی Admin (مدیریت)
- یکی Client (کاربر)
```

**دقیقه 3-8: Admin App**
```
✅ Login به پنل ادمین
✅ نشان دادن Dashboard (آمار)
✅ اضافه کردن یک کاربر جدید
✅ ایجاد یک لایسنس جدید
✅ مشاهده لیست پروفایل‌ها
✅ تنظیمات پروکسی
```

**دقیقه 8-13: Client App**
```
✅ ورود با لایسنس
✅ ثبت‌نام کاربر جدید
✅ Login
✅ ایجاد پروفایل مرورگر
✅ تنظیم fingerprint
✅ افزودن پروکسی
✅ راه‌اندازی مرورگر
```

**دقیقه 13-15: سوال و جواب**

---

## 🔧 عیب‌یابی رایج

### مشکل 1: "Cannot connect to backend"
```
✅ چک: Backend در حال اجرا است؟
   → باز کردن http://localhost:3000 در مرورگر
   → باید پیام "Anti-Detect Browser Backend API" ببینید

✅ چک: Docker services اجرا هستند؟
   → docker-compose ps
   → باید همه UP باشند
```

### مشکل 2: "Database connection failed"
```
✅ چک PostgreSQL:
   docker exec -it antidetect_postgres psql -U antidetect_user -d antidetect_db -c "\dt"
```

### مشکل 3: "Apps can't install"
```
✅ راست‌کلیک → Run as Administrator
✅ Windows Defender را موقتاً غیرفعال کنید
```

---

## 📦 فایل‌های مورد نیاز برای Demo:

```
✅ admin-app-setup.exe (از GitHub Actions artifacts)
✅ client-app-setup.exe (از GitHub Actions artifacts)
✅ docker-compose.yml (در پوشه anti-detect-mvp)
✅ backend.exe (اگر از prebuilt استفاده می‌کنید)
```

---

## 🌐 انتقال به Production (بعد از تایید رئیس)

وقتی رئیس تایید کرد، فقط این مراحل:

### 1. خرید سرور (مثلاً DigitalOcean)
```
- CPU: 2 cores
- RAM: 4GB
- Storage: 50GB SSD
- هزینه: ~$20/ماه
```

### 2. انتقال Backend
```bash
# روی سرور:
git clone your-repo
cd anti-detect-mvp
docker-compose up -d
cargo build --release
./target/release/backend
```

### 3. تنظیم Apps
```javascript
// فقط این یک خط را تغییر دهید:
const API_URL = "https://your-server-ip:3000" // به جای localhost
```

### 4. Build مجدد Apps
```
npm run tauri build
```

✅ **تمام!** همه‌چیز روی سرور است.

---

## 🎁 نکات برای Demo موفق:

1. ✅ **تمرین کنید** - حداقل 2 بار قبل از ارائه
2. ✅ **Data نمونه** - 3-4 کاربر، 5-6 لایسنس، 10 پروفایل آماده
3. ✅ **Backup Plan** - اگر اینترنت/شبکه مشکل داشت، Screen Recording آماده
4. ✅ **Clean Install** - صبح روز Demo، همه‌چیز را از نو نصب کنید
5. ✅ **PowerPoint** - یک PDF/PPT کوتاه برای معرفی

---

## 📞 در صورت بروز مشکل:

1. چک Logs:
   ```bash
   # Backend logs:
   tail -f backend.log
   
   # Docker logs:
   docker-compose logs -f postgres
   ```

2. Restart همه‌چیز:
   ```bash
   docker-compose down
   docker-compose up -d
   cargo run --release
   ```

3. اگر باز هم مشکل بود، از backup screen recording استفاده کنید

---

**موفق باشید! 🎉**
