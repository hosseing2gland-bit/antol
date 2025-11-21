# 🚀 Anti-Detect Browser - راهنمای نصب و Build

## ✅ وضعیت فعلی پروژه

### Backend (سرور)
- ✅ **راه‌اندازی شده** - در حال اجرا روی `http://108.143.173.222:3000`
- ✅ **Systemd Service** - اجرای خودکار بعد از restart سرور
- ✅ **Database** - PostgreSQL با 5 کاربر، 5 لایسنس، 5 پروکسی، 7 پروفایل
- ✅ **Docker** - Redis، MinIO، PostgreSQL

### Frontend
- ✅ **Client App** - Build شده و آماده برای compile
- ✅ **Admin App** - Build شده و آماده برای compile
- ✅ **Environment** - متصل به سرور واقعی

---

## 📦 نصب Installer ها

### 🪟 Windows

**روش 1: استفاده از اسکریپت خودکار**
```cmd
build-windows.bat
```

**روش 2: دستی**
1. مطالعه [BUILD_WINDOWS.md](BUILD_WINDOWS.md)
2. نصب پیش‌نیازها
3. اجرای دستورات build

**فایل‌های نهایی**:
- `client-app\src-tauri\target\release\bundle\msi\*.msi`
- `admin-app\src-tauri\target\release\bundle\msi\*.msi`

### 🍎 macOS

**روش 1: استفاده از اسکریپت خودکار**
```bash
./build-macos.sh
```

**روش 2: دستی**
1. مطالعه [BUILD_MACOS.md](BUILD_MACOS.md)
2. نصب پیش‌نیازها
3. اجرای دستورات build

**فایل‌های نهایی**:
- `client-app/src-tauri/target/release/bundle/dmg/*.dmg`
- `admin-app/src-tauri/target/release/bundle/dmg/*.dmg`

---

## 🔐 اطلاعات ورود

### کاربران Demo (همه با password: `admin123`)

| Email | Role | Subscription |
|-------|------|-------------|
| `admin@demo.com` | Admin | Enterprise |
| `user1@demo.com` | User | Pro |
| `user2@demo.com` | User | Basic |
| `test@demo.com` | User | Free |

### API Base URL
```
http://108.143.173.222:3000
```

---

## 🛠️ مدیریت Backend (روی سرور)

### دستورات Systemd

```bash
# مشاهده وضعیت
sudo systemctl status antidetect-backend

# Restart
sudo systemctl restart antidetect-backend

# توقف
sudo systemctl stop antidetect-backend

# شروع
sudo systemctl start antidetect-backend

# مشاهده لاگ (real-time)
sudo journalctl -u antidetect-backend -f

# مشاهده لاگ (100 خط آخر)
sudo journalctl -u antidetect-backend -n 100
```

### دستورات Database

```bash
# ورود به PostgreSQL container
docker exec -it $(docker ps -qf name=postgres) psql -U antidetect_user -d antidetect_db

# لیست کاربران
SELECT email, role, subscription_tier FROM users;

# لیست لایسنس‌ها
SELECT license_key, max_profiles, expires_at FROM licenses;

# خروج از psql
\q
```

---

## 📁 ساختار پروژه

```
anti-detect-mvp/
├── backend/                    # Rust + Axum API
│   ├── src/
│   ├── migrations/            # Database migrations
│   └── target/release/        # Built binary
├── client-app/                # Client Desktop App (Tauri + React)
│   ├── src/                   # React source
│   ├── src-tauri/            # Tauri Rust code
│   └── dist/                  # Built frontend
└── admin-app/                 # Admin Desktop App (Tauri + React)
    ├── src/
    ├── src-tauri/
    └── dist/
```

---

## 🐛 عیب‌یابی

### Backend در حال اجرا نیست

```bash
# چک کردن وضعیت
sudo systemctl status antidetect-backend

# Restart
sudo systemctl restart antidetect-backend

# چک لاگ
tail -50 /home/berellian/antol/anti-detect-mvp/backend/backend.log
```

### دیتابیس متصل نمی‌شه

```bash
# چک کردن Docker containers
docker ps

# چک PostgreSQL
docker logs $(docker ps -qf name=postgres) | tail -20
```

### پورت 3000 بسته است

```bash
# چک کردن پورت
sudo netstat -tlnp | grep :3000

# باز کردن پورت در firewall (اگر لازم باشه)
sudo ufw allow 3000/tcp
```

---

## 📊 API Endpoints

### Authentication
- `POST /api/auth/login` - ورود کاربر
- `POST /api/auth/register` - ثبت‌نام

### Users (نیاز به توکن)
- `GET /api/users` - لیست کاربران
- `POST /api/users` - ایجاد کاربر
- `GET /api/users/:id` - جزئیات کاربر
- `PUT /api/users/:id` - بروزرسانی
- `DELETE /api/users/:id` - حذف

### Licenses
- `GET /api/licenses` - لیست لایسنس‌ها
- `POST /api/licenses` - ایجاد لایسنس
- `POST /api/licenses/activate/:key` - فعال‌سازی

### Profiles
- `GET /api/profiles` - لیست پروفایل‌ها
- `POST /api/profiles` - ایجاد پروفایل
- `PUT /api/profiles/:id` - بروزرسانی
- `DELETE /api/profiles/:id` - حذف

### Proxies
- `GET /api/proxies` - لیست پروکسی‌ها
- `POST /api/proxies` - ایجاد پروکسی
- `POST /api/proxies/:id/test` - تست پروکسی

**مستندات کامل**: [API_DOCUMENTATION.md](API_DOCUMENTATION.md)

---

## 🔧 توسعه و تست

### اجرای Backend به صورت محلی

```bash
cd anti-detect-mvp/backend
cargo run
```

### اجرای Client App در حالت Dev

```bash
cd anti-detect-mvp/client-app
npm run dev
```

### اجرای Admin App در حالت Dev

```bash
cd anti-detect-mvp/admin-app
npm run dev
```

---

## 📝 فایل‌های مهم

| فایل | توضیحات |
|------|---------|
| `BUILD_WINDOWS.md` | راهنمای کامل build برای Windows |
| `BUILD_MACOS.md` | راهنمای کامل build برای macOS |
| `build-windows.bat` | اسکریپت خودکار build Windows |
| `build-macos.sh` | اسکریپت خودکار build macOS |
| `antidetect-backend.service` | Systemd service file |
| `install-systemd-service.sh` | نصب systemd service |
| `seed-current-schema.sql` | دیتای demo |

---

## 🚀 مراحل بعدی (اختیاری)

- [ ] راه‌اندازی HTTPS با SSL certificate
- [ ] اتصال domain به سرور
- [ ] Auto-update برای installer ها
- [ ] Code signing برای macOS/Windows
- [ ] CI/CD با GitHub Actions
- [ ] Monitoring و Logging پیشرفته

---

## 💡 نکات مهم

1. **امنیت**: `JWT_SECRET` رو در production تغییر بدید
2. **Backup**: دیتابیس رو به صورت منظم backup بگیرید
3. **Updates**: همیشه dependencies رو به‌روز نگه دارید
4. **Logs**: لاگ‌ها رو منظم چک کنید

---

## 📞 پشتیبانی

در صورت بروز مشکل:
1. چک کردن لاگ‌های backend
2. چک کردن وضعیت systemd service  
3. چک کردن اتصال دیتابیس
4. مطالعه فایل‌های BUILD_*.md

---

**نسخه**: 1.0.0  
**آخرین بروزرسانی**: 21 نوامبر 2025
