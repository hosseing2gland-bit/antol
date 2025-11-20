# Anti-Detect Browser MVP

یک سیستم کامل مدیریت مرورگر Anti-Detect با پنل ادمین حرفه‌ای

## 🎯 ویژگی‌ها

### Backend API (Rust + Axum)
- ✅ سیستم احراز هویت کامل (JWT)
- ✅ مدیریت کاربران (CRUD)
- ✅ سیستم لایسنس (ایجاد، فعال‌سازی، لغو)
- ✅ مدیریت پروفایل‌ها
- ✅ مدیریت پروکسی
- ✅ PostgreSQL Database
- ✅ Validation & Error Handling
- ✅ CORS Support

### Admin Panel (React + TypeScript)
- ✅ Dashboard با آمار کامل
- ✅ مدیریت کاربران
- ✅ ایجاد و مدیریت لایسنس
- ✅ نمایش پروفایل‌ها
- ✅ نمایش پروکسی‌ها
- ✅ تنظیمات
- ✅ Dark Theme UI
- ✅ State Management (Zustand)
- ✅ Routing (React Router)

## 🚀 نصب و راه‌اندازی

### پیش‌نیازها

```bash
# macOS
brew install postgresql rust node

# یا با Docker
docker-compose up -d postgres
```

### 1. راه‌اندازی Database

```bash
# ایجاد دیتابیس
createdb antidetect

# یا با psql
psql -U postgres
CREATE DATABASE antidetect;
\q
```

### 2. راه‌اندازی Backend

```bash
cd anti-detect-mvp/backend

# تنظیم متغیر محیطی
export DATABASE_URL="postgres://admin:admin123@localhost/antidetect"

# اجرای migrations
cargo install sqlx-cli
sqlx migrate run

# اجرای سرور
cargo run --release
```

سرور روی http://localhost:3000 اجرا میشه.

### 3. راه‌اندازی Admin App

```bash
cd anti-detect-mvp/admin-app

# نصب dependencies
npm install

# اجرای dev server
npm run dev
```

Admin Panel روی http://localhost:1420 اجرا میشه.

### 4. لاگین پیش‌فرض

```
Email: admin@antidetect.local
Password: admin123
```

## 📁 ساختار پروژه

```
anti-detect-mvp/
├── backend/                 # Rust API Server
│   ├── src/
│   │   ├── main.rs         # Entry point & routes
│   │   ├── models.rs       # Data models
│   │   └── handlers/       # API handlers
│   │       ├── auth.rs     # Login/Register
│   │       ├── users.rs    # User management
│   │       ├── licenses.rs # License management
│   │       ├── profiles.rs # Profile management
│   │       └── proxies.rs  # Proxy management
│   └── migrations/         # Database migrations
│
├── admin-app/              # React Admin Panel
│   ├── src/
│   │   ├── App.tsx         # Main component
│   │   ├── store.ts        # Zustand state
│   │   └── components/     # UI Components
│   │       ├── Login.tsx
│   │       ├── Dashboard.tsx
│   │       ├── Users.tsx
│   │       ├── Licenses.tsx
│   │       ├── Profiles.tsx
│   │       ├── Proxies.tsx
│   │       └── Settings.tsx
│   └── src-tauri/          # Tauri desktop app
│
└── client-app/             # End-user application
```

## 🔧 API Endpoints

### Authentication
- `POST /api/auth/login` - لاگین
- `POST /api/auth/register` - ثبت‌نام

### Users
- `GET /api/users` - لیست کاربران
- `POST /api/users` - ایجاد کاربر
- `GET /api/users/:id` - جزئیات کاربر
- `PUT /api/users/:id` - ویرایش کاربر
- `DELETE /api/users/:id` - حذف کاربر

### Licenses
- `GET /api/licenses` - لیست لایسنس‌ها
- `POST /api/licenses` - ایجاد لایسنس
- `GET /api/licenses/:id` - جزئیات لایسنس
- `POST /api/licenses/:id/revoke` - لغو لایسنس
- `POST /api/licenses/activate/:key` - فعال‌سازی لایسنس

### Profiles
- `GET /api/profiles` - لیست پروفایل‌ها
- `POST /api/profiles` - ایجاد پروفایل
- `GET /api/profiles/:id` - جزئیات پروفایل
- `PUT /api/profiles/:id` - ویرایش پروفایل
- `DELETE /api/profiles/:id` - حذف پروفایل

### Proxies
- `GET /api/proxies` - لیست پروکسی‌ها
- `POST /api/proxies` - ایجاد پروکسی
- `GET /api/proxies/:id` - جزئیات پروکسی
- `PUT /api/proxies/:id` - ویرایش پروکسی
- `DELETE /api/proxies/:id` - حذف پروکسی
- `POST /api/proxies/:id/test` - تست پروکسی

## 🛠️ Development

### Build Backend
```bash
cd backend
cargo build --release
```

### Build Admin App
```bash
cd admin-app
npm run build
npm run tauri build
```

### Run Tests
```bash
cd backend
cargo test
```

## 🎨 UI Features

- 🌙 Dark Mode Design
- 📊 Dashboard با آمار Real-time
- 📋 جداول قابل مرتب‌سازی
- 🔍 جستجو و فیلتر
- ✏️ Modal‌های ویرایش
- 🎯 واکنش‌گرای کامل
- 🔐 احراز هویت امن

## 🔒 Security

- Bcrypt password hashing
- JWT authentication
- Input validation
- SQL injection protection (SQLx)
- CORS configuration
- Environment variables for secrets

## 📦 Database Schema

### Users Table
- id (UUID)
- email (unique)
- password_hash
- role (admin/user)
- is_active
- created_at, updated_at

### Licenses Table
- id (UUID)
- license_key (unique)
- plan (trial/basic/pro/enterprise)
- max_profiles
- user_id (nullable)
- is_active
- expires_at
- activated_at
- created_at, updated_at

### Profiles Table
- id (UUID)
- user_id
- name
- user_agent
- screen_resolution
- timezone
- language
- webgl_vendor, webgl_renderer
- canvas_noise, audio_noise
- fonts
- proxy_id
- created_at, updated_at

### Proxies Table
- id (UUID)
- user_id
- proxy_type (http/https/socks5)
- host
- port
- username, password
- country
- created_at, updated_at

## 🚢 Deployment

### با Docker
```bash
docker-compose up -d
```

### بدون Docker
```bash
# Backend
cd backend && cargo run --release

# Admin App
cd admin-app && npm run build && npm run tauri build
```

## 📝 TODO

- [ ] Client App با anti-detection features
- [ ] Canvas fingerprinting protection
- [ ] WebGL fingerprinting protection
- [ ] Audio fingerprinting protection
- [ ] Font fingerprinting protection
- [ ] WebRTC leak protection
- [ ] User agent rotation
- [ ] Profile import/export
- [ ] Proxy testing functionality
- [ ] Real-time stats
- [ ] Email notifications
- [ ] 2FA authentication

## 🤝 Contributing

Pull requests are welcome!

## 📄 License

MIT License

---

Made with ❤️ for Anti-Detection
