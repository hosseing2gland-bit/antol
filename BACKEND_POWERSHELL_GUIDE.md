# 🚀 راهنمای اجرای Backend در Windows PowerShell

## روش 1: استفاده از Script خودکار (پیشنهادی ⭐)

```powershell
# باز کردن PowerShell در پوشه پروژه
cd C:\path\to\antol

# اجرای script خودکار
.\START_BACKEND_WINDOWS.ps1
```

---

## روش 2: دستی (مرحله به مرحله)

### مرحله 1️⃣: راه‌اندازی Docker Services

```powershell
# رفتن به پوشه پروژه
cd anti-detect-mvp

# شروع Docker services (PostgreSQL, Redis, MinIO)
docker compose up -d

# چک کردن وضعیت
docker compose ps
```

**باید ببینید:**
```
antidetect_postgres   Up
antidetect_redis      Up
antidetect_minio      Up
```

### مرحله 2️⃣: اجرای Backend

```powershell
# رفتن به پوشه backend
cd backend

# اگر اولین بار است، باید build کنید (5-10 دقیقه طول می‌کشد)
cargo build --release

# اجرای backend
.\target\release\backend.exe
```

**خروجی موفق:**
```
Backend running on http://127.0.0.1:3000
Database connected successfully
```

---

## 🧪 تست Backend

### روش 1: از PowerShell

```powershell
# تست Health Check
Invoke-WebRequest -Uri "http://localhost:3000/health" | Select-Object -ExpandProperty Content

# تست Login
$body = @{
    email = "admin@demo.com"
    password = "admin123"
} | ConvertTo-Json

Invoke-WebRequest -Uri "http://localhost:3000/api/auth/login" `
    -Method POST `
    -ContentType "application/json" `
    -Body $body | Select-Object -ExpandProperty Content
```

### روش 2: از مرورگر

باز کردن این URL در مرورگر:
```
http://localhost:3000/health
```

باید ببینید: `{"status":"ok"}`

---

## 🛑 متوقف کردن Backend

در PowerShell که backend اجرا شده:
```
Ctrl + C
```

برای متوقف کردن Docker services:
```powershell
cd anti-detect-mvp
docker compose down
```

---

## 🔧 عیب‌یابی

### مشکل 1: "Docker command not found"

**راه‌حل:**
1. Docker Desktop را نصب کنید: https://www.docker.com/products/docker-desktop
2. بعد از نصب، Docker Desktop را باز کنید
3. PowerShell را ببندید و دوباره باز کنید

### مشکل 2: "Port 3000 already in use"

**راه‌حل:**
```powershell
# پیدا کردن process که از port 3000 استفاده می‌کند
netstat -ano | findstr :3000

# فرض کنید PID برابر 12345 است
taskkill /PID 12345 /F
```

### مشکل 3: "Cannot connect to database"

**راه‌حل:**
```powershell
# چک کردن وضعیت PostgreSQL
docker ps | findstr postgres

# اگر running نبود
cd anti-detect-mvp
docker compose restart postgres

# صبر کنید 5 ثانیه
Start-Sleep -Seconds 5

# دوباره backend را اجرا کنید
```

### مشکل 4: "cargo: command not found"

**راه‌حل:**
1. Rust را نصب کنید: https://rustup.rs
2. PowerShell را ببندید و دوباره باز کنید
3. تست کنید:
```powershell
cargo --version
```

---

## 📝 Checklist راه‌اندازی

- [ ] Docker Desktop نصب و running است
- [ ] Rust و Cargo نصب است (`cargo --version`)
- [ ] PostgreSQL container running است
- [ ] Redis container running است
- [ ] MinIO container running است
- [ ] Backend compile شده است (`target\release\backend.exe` موجود است)
- [ ] Backend در حال اجرا است (http://localhost:3000)
- [ ] Health check موفق است

---

## 🎯 دستورات سریع

```powershell
# همه در یک بار (از ریشه پروژه)
cd anti-detect-mvp && docker compose up -d && cd backend && .\target\release\backend.exe

# متوقف کردن همه چیز
cd anti-detect-mvp && docker compose down
```

---

## 💡 نکات مهم

1. **همیشه Docker Desktop را اول باز کنید**
   - Backend به PostgreSQL نیاز دارد
   - PostgreSQL در Docker اجرا می‌شود

2. **اولین build زمان‌بر است**
   - اولین `cargo build --release` حدود 5-10 دقیقه طول می‌کشد
   - دفعات بعدی خیلی سریع‌تر است (فقط تغییرات compile می‌شود)

3. **Port ها**
   - Backend: 3000
   - PostgreSQL: 5432
   - Redis: 6379
   - MinIO: 9000, 9001

4. **در صورت تغییر کد**
   ```powershell
   # متوقف کردن backend (Ctrl+C)
   # rebuild
   cargo build --release
   # اجرای دوباره
   .\target\release\backend.exe
   ```

---

**موفق باشید! 🚀**
