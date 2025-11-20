# 🚀 راهنمای نصب و اجرا روی ویندوز (برای Demo)

## ✅ چیزهایی که نیاز دارید:

### 1. فایل‌های نصبی (دانلود از GitHub):
- `admin-app-setup.exe` (برای پنل مدیریت)
- `client-app-setup.exe` (برای کاربران)

### 2. Docker Desktop برای Windows:
- دانلود: https://www.docker.com/products/docker-desktop/
- **فقط یک بار نصب کنید**

---

## 📝 مرحله 1: راه‌اندازی Backend (روی یک لپ‌تاپ)

### A. نصب Docker Desktop
1. فایل Docker Desktop را دانلود و نصب کنید
2. بعد از نصب، Docker را باز کنید
3. منتظر بمانید تا در System Tray آیکون Docker سبز شود

### B. دانلود و اجرای پروژه Backend

#### گزینه 1: استفاده از Git (ساده‌تر)
```powershell
# در PowerShell یا CMD:
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol/anti-detect-mvp
```

#### گزینه 2: دانلود ZIP
1. برو به: https://github.com/hosseing2gland-bit/antol
2. کلیک روی `Code` → `Download ZIP`
3. Extract کن
4. در PowerShell به پوشه `anti-detect-mvp` برو

### C. راه‌اندازی Database و Services

```powershell
# در پوشه anti-detect-mvp:
docker-compose up -d

# چک کردن (باید 3 سرویس UP باشد):
docker-compose ps
```

خروجی باید شبیه این باشد:
```
NAME                  STATUS    PORTS
antidetect_postgres   Up        0.0.0.0:5432->5432/tcp
antidetect_redis      Up        0.0.0.0:6379->6379/tcp
antidetect_minio      Up        0.0.0.0:9000-9001->9000-9001/tcp
```

### D. راه‌اندازی Backend API

#### گزینه 1: استفاده از فایل Executable (پیشنهادی)
```powershell
# دانلود backend.exe از GitHub Actions Artifacts
# سپس در همان پوشه:
.\backend.exe
```

#### گزینه 2: Build از Source (اگر Rust نصب دارید)
```powershell
cd backend
$env:DATABASE_URL="postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db"
cargo run --release
```

✅ **باید این پیام را ببینید:**
```
✅ Server running on http://0.0.0.0:3000
📚 API Documentation: http://0.0.0.0:3000/api
```

### E. ایجاد کاربر Admin (فقط یک بار)

در یک PowerShell جدید:
```powershell
docker exec -it antidetect_postgres psql -U antidetect_user -d antidetect_db
```

سپس در PostgreSQL:
```sql
INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'admin@demo.com',
  '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5RzbE6bVTQK0W',
  'admin',
  true,
  NOW(),
  NOW()
);

\q
```

**نکته:** رمز عبور: `admin123`

---

## 📱 مرحله 2: نصب و استفاده از Apps

### A. نصب Admin App (روی همان لپ‌تاپ یا لپ‌تاپ دیگر)

1. **راست‌کلیک** روی `admin-app-setup.exe`
2. `Run as Administrator`
3. مراحل نصب را دنبال کنید
4. بعد از نصب، از Start Menu: `Anti-Detect Admin` را باز کنید

### B. نصب Client App

1. **راست‌کلیک** روی `client-app-setup.exe`
2. `Run as Administrator`
3. مراحل نصب را دنبال کنید
4. از Start Menu: `Anti-Detect Client` را باز کنید

---

## 🎯 مرحله 3: تست کامل

### Test Admin App:
```
1. باز کردن Admin App
2. Login: admin@demo.com / admin123
3. Dashboard → مشاهده آمار
4. Users → اضافه کردن کاربر جدید
   - Email: user@test.com
   - Password: user123
   - Role: User
5. Licenses → ایجاد لایسنس جدید
   - Plan: Basic
   - Max Profiles: 5
6. کپی کردن License Key که ساخته شد
```

### Test Client App:
```
1. باز کردن Client App
2. وارد کردن License Key (که از Admin گرفتید)
3. Register:
   - Email: user@test.com
   - Password: user123
4. Login
5. Create Profile:
   - Name: My Profile
   - انتخاب تنظیمات دلخواه
6. Launch Browser با اون Profile
```

---

## 🔧 اگر مشکلی پیش آمد:

### مشکل 1: "Cannot connect to backend"
```powershell
# چک کنید backend در حال اجرا است:
# در مرورگر باز کنید:
http://localhost:3000

# باید ببینید: "Anti-Detect Browser Backend API"
```

### مشکل 2: Docker اجرا نمی‌شود
```
1. Docker Desktop را ببندید
2. Windows را Restart کنید
3. Docker Desktop را دوباره باز کنید
4. منتظر بمانید تا سبز شود (2-3 دقیقه)
```

### مشکل 3: Apps نصب نمی‌شوند
```
1. Windows Defender را موقتاً غیرفعال کنید
2. راست‌کلیک → Run as Administrator
3. اگر باز نشد، Antivirus را چک کنید
```

### مشکل 4: Database خالی است
```powershell
# ریست کردن Database:
docker-compose down -v
docker-compose up -d

# دوباره کاربر Admin را بسازید (مرحله 1.E)
```

---

## 📊 برای Demo به رئیس:

### سناریو پیشنهادی (10 دقیقه):

**صفحه 1: Admin Panel** (5 دقیقه)
```
✅ نشان دادن Dashboard
✅ مدیریت کاربران (اضافه/حذف)
✅ ساخت لایسنس جدید
✅ نشان دادن Profiles و Proxies
```

**صفحه 2: Client App** (5 دقیقه)
```
✅ ورود با لایسنس
✅ ثبت‌نام کاربر
✅ ساخت Browser Profile
✅ تنظیم Anti-Detection
✅ راه‌اندازی مرورگر
```

### نکات مهم Demo:
1. ✅ **قبل از Demo:** همه چیز را یک بار تست کنید
2. ✅ **Data نمونه:** 2-3 کاربر، 3-4 لایسنس، 5-6 پروفایل آماده کنید
3. ✅ **Screen Recording:** یک backup ویدیو داشته باشید
4. ✅ **Laptop آماده:** شارژ کامل، اینترنت پایدار

---

## 💡 سوالات متداول:

### Q: آیا برای هر Demo باید همه مراحل را انجام دهم?
**A:** خیر! فقط یک بار:
- نصب Docker Desktop
- نصب Admin و Client Apps
- راه‌اندازی اولیه Backend

بعد از اون، برای Demoهای بعدی فقط:
```powershell
docker-compose up -d
.\backend.exe
```

### Q: آیا نیاز به اینترنت دارم؟
**A:** فقط برای دانلود اولیه. بعد از نصب، همه چیز Offline کار می‌کند.

### Q: چند لپ‌تاپ نیاز دارم؟
**A:** یکی کافیه! می‌تونید هم Admin و هم Client را روی یک لپ‌تاپ نصب کنید.

### Q: بعد از تایید رئیس چطور به Server واقعی منتقل کنم؟
**A:** خیلی ساده! فقط:
1. یک سرور Linux اجاره کنید (DigitalOcean, AWS, ...)
2. همین `docker-compose.yml` و `backend` را آپلود کنید
3. در Apps فقط IP localhost را به IP سرور تغییر دهید
4. Build مجدد Apps

---

## 🎉 آماده هستید!

همه چیز آماده است. موفق باشید! 🚀

**پشتیبانی:** اگر سوالی داشتید، issue بسازید: https://github.com/hosseing2gland-bit/antol/issues
