# 🚀 راهنمای Deploy Backend روی سرور

## پیش‌نیازها

- ✅ یک سرور Linux (Ubuntu 22.04 یا 24.04 پیشنهادی)
- ✅ دسترسی SSH به سرور
- ✅ یک دامنه یا subdomain (مثلاً `api.yourdomain.com`)
- ✅ حداقل 2GB RAM و 20GB فضای دیسک

---

## مرحله 1️⃣: نصب ابزارهای لازم روی سرور

SSH به سرورت و این دستورات رو اجرا کن:

```bash
# آپدیت سیستم
sudo apt update && sudo apt upgrade -y

# نصب ابزارهای پایه
sudo apt install -y curl wget git build-essential pkg-config libssl-dev

# نصب Docker و Docker Compose
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo apt install -y docker-compose-plugin

# اضافه کردن user به گروه docker
sudo usermod -aG docker $USER
newgrp docker

# نصب Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# نصب Nginx (برای reverse proxy)
sudo apt install -y nginx certbot python3-certbot-nginx
```

---

## مرحله 2️⃣: کپی پروژه به سرور

از کامپیوتر خودت:

```bash
# روش 1: Clone از GitHub
ssh user@your-server-ip
cd /home/user
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol

# روش 2: Upload از کامپیوتر خودت
# از کامپیوتر محلی:
scp -r C:\path\to\antol user@your-server-ip:/home/user/
```

---

## مرحله 3️⃣: تنظیمات Environment Variables

روی سرور:

```bash
cd /home/user/antol/anti-detect-mvp

# ساخت فایل .env اصلی
cat > .env << 'EOF'
# Database
DATABASE_URL=postgresql://antidetect_user:your_secure_password_here@localhost:5432/antidetect_db

# Redis
REDIS_URL=redis://localhost:6379

# MinIO (S3)
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=your_secure_minio_password_here
MINIO_BUCKET_NAME=antidetect-browser

# JWT Secret (برای امنیت، یک رشته رندوم 64 کاراکتری بساز)
JWT_SECRET=$(openssl rand -base64 64)

# Server
RUST_LOG=info
HOST=0.0.0.0
PORT=3000
EOF

# تغییر password های پیش‌فرض
nano .env
```

**⚠️ مهم:** حتماً password های پیش‌فرض رو تغییر بده!

---

## مرحله 4️⃣: راه‌اندازی Docker Services

```bash
cd /home/user/antol/anti-detect-mvp

# شروع PostgreSQL, Redis, MinIO
docker compose up -d

# چک کردن وضعیت
docker compose ps

# دیدن logs
docker compose logs -f
```

باید ببینی:
```
antidetect_postgres   Up
antidetect_redis      Up
antidetect_minio      Up
```

---

## مرحله 5️⃣: Build و اجرای Backend

```bash
cd /home/user/antol/anti-detect-mvp/backend

# Build در حالت release (5-10 دقیقه طول می‌کشد)
cargo build --release

# تست اجرا
export $(cat ../.env | xargs)
./target/release/backend
```

اگر همه چیز OK بود، `Ctrl+C` بزن و برو مرحله بعد.

---

## مرحله 6️⃣: ساخت systemd Service (اجرای خودکار)

```bash
# ساخت service file
sudo nano /etc/systemd/system/antidetect-backend.service
```

محتوای فایل:

```ini
[Unit]
Description=Anti-Detect Browser Backend
After=network.target docker.service
Requires=docker.service

[Service]
Type=simple
User=your-username
WorkingDirectory=/home/your-username/antol/anti-detect-mvp/backend
EnvironmentFile=/home/your-username/antol/anti-detect-mvp/.env
ExecStart=/home/your-username/antol/anti-detect-mvp/backend/target/release/backend
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**⚠️ نکته:** `your-username` رو با username واقعی سرورت عوض کن!

فعال‌سازی service:

```bash
# Reload systemd
sudo systemctl daemon-reload

# فعال کردن برای شروع خودکار
sudo systemctl enable antidetect-backend

# شروع service
sudo systemctl start antidetect-backend

# چک کردن وضعیت
sudo systemctl status antidetect-backend

# دیدن logs
sudo journalctl -u antidetect-backend -f
```

---

## مرحله 7️⃣: تنظیم Nginx Reverse Proxy

```bash
sudo nano /etc/nginx/sites-available/antidetect-api
```

محتوای فایل:

```nginx
server {
    listen 80;
    server_name api.yourdomain.com;  # تغییر بده به دامنه خودت

    # محدودیت حجم آپلود
    client_max_body_size 100M;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;

        # CORS headers (اگر نیاز داری)
        add_header 'Access-Control-Allow-Origin' '*' always;
        add_header 'Access-Control-Allow-Methods' 'GET, POST, PUT, DELETE, OPTIONS' always;
        add_header 'Access-Control-Allow-Headers' 'Authorization, Content-Type' always;
    }
}
```

فعال‌سازی:

```bash
# لینک به sites-enabled
sudo ln -s /etc/nginx/sites-available/antidetect-api /etc/nginx/sites-enabled/

# تست تنظیمات
sudo nginx -t

# ریستارت Nginx
sudo systemctl restart nginx
```

---

## مرحله 8️⃣: نصب SSL Certificate (HTTPS)

```bash
# نصب SSL با Let's Encrypt
sudo certbot --nginx -d api.yourdomain.com

# تست auto-renewal
sudo certbot renew --dry-run
```

بعد از نصب SSL، Nginx خودکار به HTTPS تغییر می‌کنه.

---

## مرحله 9️⃣: تنظیم Firewall

```bash
# باز کردن port های لازم
sudo ufw allow 22/tcp      # SSH
sudo ufw allow 80/tcp      # HTTP
sudo ufw allow 443/tcp     # HTTPS
sudo ufw enable

# چک کردن وضعیت
sudo ufw status
```

---

## مرحله 🔟: تست نهایی

از کامپیوتر خودت:

```bash
# تست API
curl https://api.yourdomain.com/api/auth/login \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}'
```

باید یک JWT token دریافت کنی! ✅

---

## 📱 تنظیم Admin App و Client App

حالا در فایل‌های frontend:

### Admin App: `admin-app/src/store.ts`
```typescript
const API_URL = 'https://api.yourdomain.com/api'
```

### Client App: `client-app/src/store.ts`
```typescript
const API_URL = 'https://api.yourdomain.com/api'
```

بعد rebuild کن:
```bash
cd admin-app
npm run build
npm run tauri build

cd ../client-app
npm run build
npm run tauri build
```

---

## 🔧 دستورات مفید

### چک کردن logs
```bash
# Backend logs
sudo journalctl -u antidetect-backend -f

# Docker logs
docker compose logs -f

# Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

### ریستارت services
```bash
# ریستارت backend
sudo systemctl restart antidetect-backend

# ریستارت Docker
docker compose restart

# ریستارت Nginx
sudo systemctl restart nginx
```

### آپدیت کد
```bash
cd /home/user/antol
git pull origin main

cd anti-detect-mvp/backend
cargo build --release

sudo systemctl restart antidetect-backend
```

---

## 🔒 امنیت (بعد از deploy)

```bash
# 1. تغییر password های پیش‌فرض database
docker exec -it antidetect_postgres psql -U antidetect_user -d antidetect_db
\password antidetect_user

# 2. غیرفعال کردن password authentication برای root
sudo nano /etc/ssh/sshd_config
# PermitRootLogin no
sudo systemctl restart sshd

# 3. نصب fail2ban
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban

# 4. آپدیت منظم
sudo apt update && sudo apt upgrade -y
```

---

## 📊 مانیتورینگ (اختیاری)

```bash
# نصب htop برای مانیتور منابع
sudo apt install -y htop

# دیدن استفاده از منابع
htop

# دیدن استفاده از دیسک
df -h

# دیدن استفاده از RAM
free -h
```

---

## 🚨 عیب‌یابی

### Backend شروع نمی‌شه
```bash
sudo systemctl status antidetect-backend
sudo journalctl -u antidetect-backend -n 50
```

### Database وصل نمی‌شه
```bash
docker compose ps
docker compose logs postgres
```

### Nginx error
```bash
sudo nginx -t
sudo tail -f /var/log/nginx/error.log
```

---

## ✅ Checklist نهایی

- [ ] سرور آپدیت شده
- [ ] Docker نصب و running
- [ ] PostgreSQL, Redis, MinIO در Docker بالا هستند
- [ ] Backend compile و در حال اجرا
- [ ] systemd service فعال و auto-start
- [ ] Nginx reverse proxy تنظیم شده
- [ ] SSL certificate نصب شده (HTTPS)
- [ ] Firewall تنظیم شده
- [ ] DNS برای دامنه تنظیم شده (A record به IP سرور)
- [ ] Password های پیش‌فرض تغییر کرده
- [ ] Frontend apps به API جدید وصل شدند

---

## 🎯 URL های نهایی

```
Backend API: https://api.yourdomain.com/api
Health Check: https://api.yourdomain.com/health
Login: https://api.yourdomain.com/api/auth/login
```

---

**موفق باشید! 🚀**

اگر سوالی داشتی یا مشکلی پیش اومد، بگو کمکت کنم.
