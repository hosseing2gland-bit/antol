# 🚀 Deploy Backend بدون دامنه (فقط با IP)

## مزایا و معایب

### ✅ مزایا
- سریع‌تر (نیاز به DNS ندارد)
- رایگان (نیاز به دامنه ندارد)
- برای تست و development عالی است

### ❌ معایب
- نمی‌تونی SSL/HTTPS داشته باشی (امنیت کمتر)
- IP ممکنه تغییر کنه
- برای production توصیه نمی‌شه

---

## روش 1️⃣: استفاده از HTTP (بدون SSL)

### مرحله 1: نصب و راه‌اندازی

روی سرورت:

```bash
# SSH به سرور
ssh user@your-server-ip

# Clone پروژه
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol/anti-detect-mvp

# نصب Docker (اگر نداری)
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
newgrp docker

# نصب Rust (اگر نداری)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# ساخت .env
cat > .env << 'EOF'
DATABASE_URL=postgresql://antidetect_user:your_password@localhost:5432/antidetect_db
REDIS_URL=redis://localhost:6379
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=your_minio_password
MINIO_BUCKET_NAME=antidetect-browser
JWT_SECRET=$(openssl rand -base64 64)
RUST_LOG=info
HOST=0.0.0.0
PORT=3000
EOF

# راه‌اندازی Docker
docker compose up -d

# Build backend
cd backend
cargo build --release

# اجرا
export $(cat ../.env | xargs)
nohup ./target/release/backend > backend.log 2>&1 &
```

### مرحله 2: باز کردن Port در Firewall

```bash
# اگر UFW داری
sudo ufw allow 3000/tcp
sudo ufw allow 22/tcp
sudo ufw enable

# اگر iptables داری
sudo iptables -A INPUT -p tcp --dport 3000 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 22 -j ACCEPT
sudo iptables-save
```

### مرحله 3: تست

از کامپیوتر خودت:

```bash
curl http://YOUR_SERVER_IP:3000/api/auth/login \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}'
```

---

## روش 2️⃣: استفاده از Nginx (پیشنهادی)

حتی بدون دامنه، Nginx کمک می‌کنه:

```bash
# نصب Nginx
sudo apt install -y nginx

# تنظیمات Nginx
sudo nano /etc/nginx/sites-available/default
```

محتوا:

```nginx
server {
    listen 80;
    server_name _;  # قبول همه درخواست‌ها

    client_max_body_size 100M;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;

        # CORS
        add_header 'Access-Control-Allow-Origin' '*' always;
        add_header 'Access-Control-Allow-Methods' 'GET, POST, PUT, DELETE, OPTIONS' always;
        add_header 'Access-Control-Allow-Headers' 'Authorization, Content-Type' always;
    }
}
```

```bash
# تست و ریستارت
sudo nginx -t
sudo systemctl restart nginx

# باز کردن port 80
sudo ufw allow 80/tcp
```

حالا می‌تونی به جای port 3000، از port 80 استفاده کنی:

```bash
curl http://YOUR_SERVER_IP/api/auth/login \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}'
```

---

## تنظیم Admin App و Client App

### گزینه 1: با Port 3000

در `store.ts`:

```typescript
const API_URL = 'http://YOUR_SERVER_IP:3000/api'
```

مثال:
```typescript
const API_URL = 'http://45.123.45.67:3000/api'
```

### گزینه 2: با Nginx (Port 80)

```typescript
const API_URL = 'http://YOUR_SERVER_IP/api'
```

مثال:
```typescript
const API_URL = 'http://45.123.45.67/api'
```

---

## Script خودکار برای IP

یک script سریع:

```bash
#!/bin/bash

# دریافت IP عمومی سرور
SERVER_IP=$(curl -s ifconfig.me)

echo "✅ IP سرور شما: $SERVER_IP"
echo ""
echo "📝 تنظیمات Frontend:"
echo "const API_URL = 'http://$SERVER_IP:3000/api'"
echo ""
echo "🧪 تست API:"
echo "curl http://$SERVER_IP:3000/api/auth/login -X POST -H 'Content-Type: application/json' -d '{\"email\":\"admin@demo.com\",\"password\":\"admin123\"}'"
```

---

## ⚠️ نکات امنیتی

### با HTTP (بدون SSL):
- ❌ Password ها plain text ارسال می‌شن
- ❌ Token ها قابل intercept هستن
- ✅ فقط برای development/testing

### راه‌حل‌های امنیتی:

1. **استفاده از VPN**
   ```bash
   # نصب WireGuard یا OpenVPN
   # اتصال از طریق VPN
   ```

2. **محدود کردن دسترسی به IP خاص**
   ```bash
   # فقط IP خودت رو مجاز کن
   sudo ufw allow from YOUR_HOME_IP to any port 3000
   ```

3. **استفاده از SSH Tunnel**
   ```bash
   # از کامپیوتر خودت
   ssh -L 3000:localhost:3000 user@server-ip
   
   # بعد در app از این استفاده کن:
   # const API_URL = 'http://localhost:3000/api'
   ```

---

## 🔄 Auto-start با systemd

برای اینکه backend بعد از ریستارت سرور خودکار بالا بیاد:

```bash
sudo nano /etc/systemd/system/antidetect-backend.service
```

```ini
[Unit]
Description=Anti-Detect Backend
After=network.target docker.service

[Service]
Type=simple
User=YOUR_USERNAME
WorkingDirectory=/home/YOUR_USERNAME/antol/anti-detect-mvp/backend
EnvironmentFile=/home/YOUR_USERNAME/antol/anti-detect-mvp/.env
ExecStart=/home/YOUR_USERNAME/antol/anti-detect-mvp/backend/target/release/backend
Restart=always

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable antidetect-backend
sudo systemctl start antidetect-backend
sudo systemctl status antidetect-backend
```

---

## 📊 چک کردن IP سرور

روی سرور:

```bash
# IP عمومی
curl ifconfig.me

# یا
curl ipinfo.io/ip

# همه IP ها
ip addr show
```

---

## 🚀 دستور کامل یک‌خطی

```bash
curl -fsSL https://raw.githubusercontent.com/hosseing2gland-bit/antol/main/deploy-server.sh | bash
```

بعد فقط IP سرورت رو بنویس و Enter!

---

## ✅ Checklist

- [ ] سرور با SSH قابل دسترسی است
- [ ] Docker نصب شده
- [ ] Port 3000 (یا 80) باز است
- [ ] Backend در حال اجرا است
- [ ] IP سرور رو داری
- [ ] Frontend به IP صحیح متصل شده

---

## 🎯 خلاصه

### برای Development:
```
Backend: http://YOUR_IP:3000/api
Frontend API_URL: 'http://YOUR_IP:3000/api'
```

### برای Production (با Nginx):
```
Backend: http://YOUR_IP/api
Frontend API_URL: 'http://YOUR_IP/api'
```

### برای امنیت بیشتر:
```
استفاده از SSH Tunnel:
ssh -L 3000:localhost:3000 user@server-ip
Frontend API_URL: 'http://localhost:3000/api'
```

---

## 🔥 Quick Start

```bash
# 1. روی سرور
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol/anti-detect-mvp
docker compose up -d
cd backend && cargo build --release
export $(cat ../.env | xargs) && ./target/release/backend &

# 2. باز کردن port
sudo ufw allow 3000/tcp

# 3. دریافت IP
curl ifconfig.me

# 4. تست
curl http://YOUR_IP:3000/api/auth/login -X POST \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}'
```

موفق باشید! 🎉
