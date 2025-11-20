# �� راهنمای اجرای Local روی macOS

این راهنما برای اجرای کامل پروژه Anti-Detect MVP روی مک شماست.

## 📋 پیش‌نیازها

قبل از شروع، مطمئن شوید این موارد رو نصب دارید:
- macOS 11 (Big Sur) یا بالاتر
- حداقل 8GB RAM
- حداقل 5GB فضای خالی

---

## 1️⃣ نصب ابزارهای مورد نیاز

### نصب Homebrew
اگر Homebrew ندارید، اول اونو نصب کنید:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

بعد از نصب، دستور زیر رو اجرا کنید (بسته به نوع پردازنده):
```bash
# برای Apple Silicon (M1/M2/M3):
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"

# برای Intel:
echo 'eval "$(/usr/local/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/usr/local/bin/brew shellenv)"
```

### نصب Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

در حین نصب، گزینه پیش‌فرض (1) رو انتخاب کنید. بعد از نصب:
```bash
source $HOME/.cargo/env
rustc --version  # بررسی نصب موفق
```

### نصب Node.js و npm
```bash
brew install node@18
brew link node@18

node --version  # باید 18.x باشه
npm --version
```

### نصب PostgreSQL
```bash
brew install postgresql@16
brew services start postgresql@16

# بررسی که PostgreSQL اجرا شده
brew services list | grep postgresql
```

---

## 2️⃣ دریافت کد پروژه

```bash
# کلون کردن پروژه
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol

# رفتن به main branch (آخرین نسخه)
git checkout main
cd anti-detect-mvp
```

---

## 3️⃣ راه‌اندازی Database

### ساخت دیتابیس PostgreSQL

```bash
# اتصال به PostgreSQL
psql postgres
```

در محیط `psql` این دستورات رو اجرا کنید:
```sql
-- ساخت کاربر
CREATE USER antidetect_user WITH PASSWORD 'antidetect123';

-- ساخت دیتابیس
CREATE DATABASE antidetect_db OWNER antidetect_user;

-- اعطای دسترسی‌ها
GRANT ALL PRIVILEGES ON DATABASE antidetect_db TO antidetect_user;

-- خروج
\q
```

### اجرای Migration فایل‌ها

```bash
cd backend

# اجرای migrations به ترتیب
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/001_init.sql
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/002_proxies.sql
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/003_profiles.sql
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/004_licenses.sql

# بررسی جداول ساخته شده
psql -h localhost -U antidetect_user -d antidetect_db -c "\dt"
```

اگر پسورد خواست: `antidetect123`

---

## 4️⃣ راه‌اندازی Backend Server

```bash
cd backend

# نصب sqlx-cli (فقط یک بار)
cargo install sqlx-cli --no-default-features --features postgres

# تنظیم متغیر محیطی DATABASE_URL
export DATABASE_URL="postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db"

# ساخت فایل .env
echo 'DATABASE_URL=postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db' > .env
echo 'JWT_SECRET=your-super-secret-jwt-key-change-in-production' >> .env
echo 'RUST_LOG=info' >> .env

# Build و اجرای backend
cargo build --release
cargo run --release
```

✅ Backend اگر موفق بود روی `http://localhost:8080` اجرا میشه.

### تست Backend

در یک ترمینال جدید:
```bash
# تست endpoint
curl http://localhost:8080/api/auth/login

# ثبت نام کاربر تست
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}'
```

---

## 5️⃣ راه‌اندازی Admin App (Tauri)

در یک **ترمینال جدید**:

```bash
cd anti-detect-mvp/admin-app

# نصب dependencies
npm install

# اجرا در حالت development
npm run dev
```

✅ Admin App توی مرورگر باز میشه روی `http://localhost:5173`

### Build کردن نسخه Desktop (اختیاری)

```bash
# Build برای macOS
npm run tauri build

# فایل .app و .dmg ساخته میشه در:
# src-tauri/target/release/bundle/
```

---

## 6️⃣ راه‌اندازی Client App (Tauri)

در یک **ترمینال جدید**:

```bash
cd anti-detect-mvp/client-app

# نصب dependencies
npm install

# اجرا در حالت development
npm run dev
```

✅ Client App روی پورت دیگری باز میشه (معمولاً `5174`)

### Build کردن نسخه Desktop (اختیاری)

```bash
npm run tauri build
```

---

## 🎯 خلاصه دستورات (اجرای سریع)

اگر همه چیز نصب شده و دیتابیس راه‌اندازی شده، فقط این دستورات رو در **3 ترمینال جداگانه** اجرا کنید:

### ترمینال 1️⃣ - Backend:
```bash
cd antol/anti-detect-mvp/backend
export DATABASE_URL="postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db"
cargo run --release
```

### ترمینال 2️⃣ - Admin App:
```bash
cd antol/anti-detect-mvp/admin-app
npm run dev
```

### ترمینال 3️⃣ - Client App:
```bash
cd antol/anti-detect-mvp/client-app
npm run dev
```

---

## ⚠️ مشکلات احتمالی و راه‌حل‌ها

### PostgreSQL اجرا نمیشه یا متصل نمیشه

```bash
# بررسی وضعیت
brew services list

# راه‌اندازی مجدد
brew services restart postgresql@16

# بررسی اتصال
psql postgres -c "SELECT version();"

# اگر پورت 5432 اشغال است
lsof -i :5432
# process رو kill کنید یا پورت رو تغییر بدید
```

### Backend compile نمیشه

```bash
# آپدیت Rust
rustup update stable

# پاک کردن cache
cd backend
cargo clean
rm -rf target/

# Build مجدد
cargo build --release
```

### npm install خطا میده

```bash
# پاک کردن cache
rm -rf node_modules package-lock.json

# نصب مجدد
npm install

# اگر باز خطا داد، از نسخه قدیمی‌تر Node استفاده کنید:
brew unlink node
brew install node@18
brew link node@18
```

### پورت اشغال است (Port already in use)

```bash
# پیدا کردن process
lsof -i :8080    # برای backend
lsof -i :5173    # برای admin-app
lsof -i :5174    # برای client-app

# Kill کردن process
kill -9 <PID>
```

### Icon ها load نمیشن (Tauri)

```bash
# بررسی وجود فایل‌های icon
ls admin-app/src-tauri/icons/
ls client-app/src-tauri/icons/

# اگر وجود نداشتن، دوباره clone کنید یا از GitHub بگیرید
```

---

## 📝 نکات مهم

1. **Database Credentials**:
   - Username: `antidetect_user`
   - Password: `antidetect123`
   - Database: `antidetect_db`
   - Port: `5432`

2. **Application Ports**:
   - Backend API: `8080`
   - Admin App (web): `5173`
   - Client App (web): `5174`

3. **Development vs Production**:
   - برای توسعه: `npm run dev` و `cargo run`
   - برای build نهایی: `npm run tauri build` و `cargo build --release`

4. **Environment Variables**:
   - Backend از فایل `.env` در `backend/` استفاده می‌کنه
   - مطمئن شوید `DATABASE_URL` صحیح است

5. **Hot Reload**:
   - Backend: با `cargo watch -x run` می‌تونید auto-reload داشته باشید
   - Frontend: با `npm run dev` به صورت خودکار reload میشه

---

## 🔍 بررسی موفقیت اجرا

اگر همه چیز درست کار کنه، باید:

✅ Backend در حال اجرا باشه:
```bash
curl http://localhost:8080/api/auth/login
# باید پاسخی مثل {"error":"..."} یا {"message":"..."} برگردونه
```

✅ Admin App در مرورگر باز بشه و صفحه login نمایش داده بشه

✅ Client App در مرورگر باز بشه

✅ بتونید کاربر جدید ثبت‌نام کنید و login کنید

---

## 🆘 کمک بیشتر

اگر مشکلی پیش اومد:

1. **لاگ‌ها رو بررسی کنید**: 
   - Backend: خروجی ترمینال `cargo run`
   - Frontend: خروجی ترمینال `npm run dev`
   - Browser Console: F12 در مرورگر

2. **مشکلات رایج**:
   - Database connection: بررسی کنید PostgreSQL در حال اجراست
   - Port conflicts: از `lsof` برای پیدا کردن استفاده کنید
   - Permission errors: ممکنه نیاز به `sudo` باشه (مخصوصاً برای PostgreSQL)

3. **Documentation**:
   - [Tauri Docs](https://tauri.app/v1/guides/)
   - [Rust Book](https://doc.rust-lang.org/book/)
   - [PostgreSQL Docs](https://www.postgresql.org/docs/)

---

## 🎉 تبریک!

حالا پروژه Anti-Detect MVP شما روی مک در حال اجراست! 

می‌تونید شروع به توسعه کنید یا از برنامه استفاده کنید. 🚀
