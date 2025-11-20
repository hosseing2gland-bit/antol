# راهنمای اجرای Local روی macOS

این راهنما نحوه اجرای پروژه به صورت local روی سیستم macOS را توضیح می‌دهد.

## پیش‌نیازها

### 1. نصب Homebrew (اگر قبلاً نصب نکرده‌اید)
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 2. نصب ابزارهای مورد نیاز
```bash
# نصب Node.js
brew install node@18

# نصب Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# تنظیم target برای معماری Mac شما
# برای Mac با چیپ Intel:
rustup target add x86_64-apple-darwin

# برای Mac با چیپ Apple Silicon (M1/M2/M3):
rustup target add aarch64-apple-darwin

# نصب PostgreSQL
brew install postgresql@14
brew services start postgresql@14

# نصب Docker (اختیاری - برای اجرای کامل با Docker)
brew install --cask docker
```

## راه‌اندازی پروژه

### 1. Clone کردن پروژه
```bash
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol
```

### 2. راه‌اندازی Backend (Rust API)

```bash
cd anti-detect-mvp/backend

# ایجاد فایل .env
cat > .env << 'EOF'
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/antidetect
JWT_SECRET=your-super-secret-key-change-this-in-production
RUST_LOG=info
EOF

# ایجاد دیتابیس
createdb antidetect

# اجرای migrations
sqlx database create
sqlx migrate run

# اجرای backend
cargo run --release
```

Backend در آدرس `http://localhost:8080` اجرا خواهد شد.

### 3. راه‌اندازی License Server

در یک ترمینال جدید:

```bash
cd anti-detect-mvp/license-server

# اجرای license server
cargo run --release
```

License server در آدرس `http://localhost:8081` اجرا خواهد شد.

### 4. راه‌اندازی Admin App (Tauri Desktop)

در یک ترمینال جدید:

```bash
cd anti-detect-mvp/admin-app

# نصب dependencies
npm install

# اجرای در حالت development
npm run tauri dev

# یا build کردن برای production:
# برای Intel Mac:
npm run tauri build -- --target x86_64-apple-darwin

# برای Apple Silicon (M1/M2/M3):
npm run tauri build -- --target aarch64-apple-darwin
```

### 5. راه‌اندازی Client App (Tauri Desktop)

در یک ترمینال جدید:

```bash
cd anti-detect-mvp/client-app

# نصب dependencies
npm install

# اجرای در حالت development
npm run tauri dev

# یا build کردن برای production:
# برای Intel Mac:
npm run tauri build -- --target x86_64-apple-darwin

# برای Apple Silicon (M1/M2/M3):
npm run tauri build -- --target aarch64-apple-darwin
```

## ساختار ترمینال‌ها

برای اجرای کامل پروژه، 4 ترمینال نیاز دارید:

```
ترمینال 1: Backend (Port 8080)
│
├─ ترمینال 2: License Server (Port 8081)
│
├─ ترمینال 3: Admin App
│
└─ ترمینال 4: Client App
```

## اجرای سریع با tmux (پیشنهادی)

اگر `tmux` دارید می‌توانید با یک اسکریپت همه را اجرا کنید:

```bash
# نصب tmux
brew install tmux

# ایجاد اسکریپت اجرا
cat > run-all.sh << 'EOF'
#!/bin/bash

# ایجاد session جدید
tmux new-session -d -s antidetect

# ترمینال 1: Backend
tmux rename-window -t antidetect:0 'Backend'
tmux send-keys -t antidetect:0 'cd anti-detect-mvp/backend && cargo run --release' C-m

# ترمینال 2: License Server
tmux new-window -t antidetect:1 -n 'License'
tmux send-keys -t antidetect:1 'cd anti-detect-mvp/license-server && cargo run --release' C-m

# ترمینال 3: Admin App
tmux new-window -t antidetect:2 -n 'Admin'
tmux send-keys -t antidetect:2 'cd anti-detect-mvp/admin-app && npm run tauri dev' C-m

# ترمینال 4: Client App
tmux new-window -t antidetect:3 -n 'Client'
tmux send-keys -t antidetect:3 'cd anti-detect-mvp/client-app && npm run tauri dev' C-m

# اتصال به session
tmux attach-target antidetect
EOF

chmod +x run-all.sh
./run-all.sh
```

برای خروج از tmux: `Ctrl+B` سپس `D`
برای بستن همه: `tmux kill-session -t antidetect`

## اجرا با Docker Compose

برای اجرای آسان‌تر می‌توانید از Docker استفاده کنید:

```bash
cd anti-detect-mvp

# اجرای تمام سرویس‌ها
docker-compose up -d

# مشاهده لاگ‌ها
docker-compose logs -f

# متوقف کردن
docker-compose down
```

**نکته**: اپلیکیشن‌های Tauri (admin-app و client-app) باید به صورت مستقیم روی سیستم اجرا شوند، نه در Docker.

## عیب‌یابی (Troubleshooting)

### مشکل: ارور در نصب dependencies
```bash
# پاک کردن cache و نصب مجدد
cd admin-app  # یا client-app
rm -rf node_modules package-lock.json
npm install
```

### مشکل: ارور در Rust build
```bash
# به‌روزرسانی Rust
rustup update

# پاک کردن cache
cargo clean
```

### مشکل: دیتابیس به سرویس متصل نمی‌شود
```bash
# راه‌اندازی مجدد PostgreSQL
brew services restart postgresql@14

# بررسی وضعیت
brew services list
```

### مشکل: Port مورد نظر اشغال است
```bash
# پیدا کردن process در حال اجرا
lsof -i :8080  # یا هر port دیگری

# کشتن process
kill -9 <PID>
```

## نکات مهم

1. **معماری Mac**: اگر Mac با چیپ M1/M2/M3 دارید، از target `aarch64-apple-darwin` استفاده کنید، در غیر این صورت از `x86_64-apple-darwin`.

2. **مجوزهای امنیتی**: در اجرای اول، macOS ممکن است درخواست مجوز برای اپلیکیشن کند. از System Preferences > Security & Privacy اجازه دهید.

3. **Performance**: برای development از `cargo run` استفاده کنید، برای production از `cargo run --release` یا `cargo build --release`.

4. **Hot Reload**: در حالت development، Tauri به صورت خودکار تغییرات را reload می‌کند.

## منابع

- [Tauri Documentation](https://tauri.app/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Node.js Documentation](https://nodejs.org/)
