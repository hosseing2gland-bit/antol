# 🚀 راهنمای اجرا روی macOS

## 1️⃣ نصب پیش‌نیازها

### نصب Homebrew (اگر نداری)
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### نصب ابزارهای مورد نیاز
```bash
# نصب Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# نصب Node.js
brew install node

# نصب PostgreSQL
brew install postgresql@16
brew services start postgresql@16

# نصب Redis (اختیاری - فعلاً استفاده نمیشه)
brew install redis
brew services start redis
```

## 2️⃣ کلون Repository

```bash
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol
git checkout claude/debug-and-fix-01QwYEHEemXmK37T8KS2Rcr6
cd anti-detect-mvp
```

## 3️⃣ راه‌اندازی Database

### ساخت دیتابیس و کاربر
```bash
# اتصال به PostgreSQL
psql postgres

# در محیط psql این دستورات رو اجرا کن:
CREATE USER antidetect_user WITH PASSWORD 'antidetect123';
CREATE DATABASE antidetect_db OWNER antidetect_user;
\q
```

### اجرای Migrations
```bash
cd backend
export DATABASE_URL="postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db"

# اجرای migrations
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/001_init.sql
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/002_proxies.sql
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/003_profiles.sql
psql -h localhost -U antidetect_user -d antidetect_db -f migrations/004_licenses.sql

# بررسی جداول ساخته شده
psql -h localhost -U antidetect_user -d antidetect_db -c "\dt"
```

## 4️⃣ راه‌اندازی Backend

```bash
cd backend

# نصب sqlx-cli (اگر قبلاً نصب نکردی)
cargo install sqlx-cli --no-default-features --features postgres

# Generate sqlx cache
export DATABASE_URL="postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db"
cargo sqlx prepare

# کامپایل و اجرای backend
cargo build --release
./target/release/backend
```

Backend روی `http://localhost:8080` اجرا میشه.

### تست Backend:
```bash
# در ترمینال جدید:
curl http://localhost:8080/api/auth/login

# ثبت نام کاربر جدید:
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}'
```

## 5️⃣ راه‌اندازی Admin App

```bash
# در ترمینال جدید:
cd admin-app

# نصب dependencies (اگر node_modules نیست)
npm install

# اجرا در حالت development
npm run dev
```

Admin App روی `http://localhost:5173` (یا پورت دیگری که Vite نشون میده) اجرا میشه.

## 6️⃣ راه‌اندازی Client App

```bash
# در ترمینال جدید:
cd client-app

# نصب dependencies (اگر node_modules نیست)
npm install

# اجرا در حالت development
npm run dev
```

Client App روی پورت دیگری (معمولاً `5174`) اجرا میشه.

## 🎯 خلاصه دستورات سریع

اگر همه چی نصب شده، فقط این دستورات رو اجرا کن:

### ترمینال 1 - Backend:
```bash
cd anti-detect-mvp/backend
export DATABASE_URL="postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db"
cargo run --release
```

### ترمینال 2 - Admin App:
```bash
cd anti-detect-mvp/admin-app
npm run dev
```

### ترمینال 3 - Client App:
```bash
cd anti-detect-mvp/client-app
npm run dev
```

## ⚠️ مشکلات احتمالی

### PostgreSQL متصل نمیشه:
```bash
# بررسی وضعیت PostgreSQL
brew services list

# راه‌اندازی مجدد
brew services restart postgresql@16

# بررسی اتصال
psql postgres -c "SELECT version();"
```

### پورت اشغال است:
```bash
# پیدا کردن process که پورت 8080 رو گرفته
lsof -i :8080

# Kill کردن process
kill -9 <PID>
```

### مشکل Rust compilation:
```bash
# آپدیت Rust
rustup update

# پاک کردن cache و build مجدد
cd backend
cargo clean
cargo build --release
```

## 📝 نکات مهم

1. **Database Password**: پسورد در فایل `.env` ست شده: `antidetect123`
2. **Ports**:
   - Backend: `8080`
   - Admin App: `5173`
   - Client App: `5174`
   - PostgreSQL: `5432`
3. **Development Mode**: برای development از `npm run dev` استفاده کن نه `npm run build`

## 🎉 موفقیت!

اگر همه چی درست کار کرد، باید:
- Backend در حال اجرا باشه و به `curl http://localhost:8080/api/auth/login` پاسخ بده
- Admin App و Client App توی مرورگر باز شن
- بتونی کاربر جدید register کنی

برای هر مشکلی، لاگ‌های ترمینال رو بررسی کن!
