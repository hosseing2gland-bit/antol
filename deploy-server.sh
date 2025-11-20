#!/bin/bash

# Script خودکار برای Deploy Backend روی سرور
# Anti-Detect Browser Backend Deployment

set -e

echo "========================================"
echo "  Anti-Detect Backend - Auto Deploy    "
echo "========================================"
echo ""

# رنگ‌ها برای خروجی
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# تابع برای چاپ پیام‌ها
print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# چک کردن root
if [ "$EUID" -eq 0 ]; then 
    print_error "لطفاً این script رو با root اجرا نکنید!"
    exit 1
fi

# گرفتن اطلاعات از کاربر
echo ""
print_info "لطفاً اطلاعات زیر را وارد کنید:"
echo ""

read -p "دامنه یا IP سرور (مثلاً api.yourdomain.com): " DOMAIN
read -p "یوزرنیم سرور (پیش‌فرض: $USER): " SERVER_USER
SERVER_USER=${SERVER_USER:-$USER}

read -p "پسورد PostgreSQL (پیشنهاد: رندوم و قوی): " DB_PASSWORD
read -p "پسورد MinIO (پیشنهاد: رندوم و قوی): " MINIO_PASSWORD

echo ""
print_info "شروع نصب..."
echo ""

# مرحله 1: آپدیت سیستم
print_info "[1/10] آپدیت سیستم..."
sudo apt update -qq && sudo apt upgrade -y -qq
print_success "سیستم آپدیت شد"

# مرحله 2: نصب ابزارها
print_info "[2/10] نصب ابزارهای پایه..."
sudo apt install -y -qq curl wget git build-essential pkg-config libssl-dev
print_success "ابزارهای پایه نصب شدند"

# مرحله 3: نصب Docker
print_info "[3/10] نصب Docker..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
    rm get-docker.sh
    print_success "Docker نصب شد"
else
    print_success "Docker قبلاً نصب شده"
fi

# مرحله 4: نصب Rust
print_info "[4/10] نصب Rust..."
if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    print_success "Rust نصب شد"
else
    print_success "Rust قبلاً نصب شده"
fi

# مرحله 5: نصب Nginx
print_info "[5/10] نصب Nginx..."
sudo apt install -y -qq nginx
print_success "Nginx نصب شد"

# مرحله 6: Clone پروژه
print_info "[6/10] دانلود پروژه..."
if [ ! -d "$HOME/antol" ]; then
    cd $HOME
    git clone https://github.com/hosseing2gland-bit/antol.git
    print_success "پروژه دانلود شد"
else
    cd $HOME/antol
    git pull origin main
    print_success "پروژه آپدیت شد"
fi

# مرحله 7: ساخت فایل .env
print_info "[7/10] تنظیم environment variables..."
cd $HOME/antol/anti-detect-mvp

JWT_SECRET=$(openssl rand -base64 64 | tr -d '\n')

cat > .env << EOF
DATABASE_URL=postgresql://antidetect_user:${DB_PASSWORD}@localhost:5432/antidetect_db
REDIS_URL=redis://localhost:6379
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=${MINIO_PASSWORD}
MINIO_BUCKET_NAME=antidetect-browser
JWT_SECRET=${JWT_SECRET}
RUST_LOG=info
HOST=0.0.0.0
PORT=3000
EOF

# تنظیم docker-compose با پسوردها
sed -i "s/POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=${DB_PASSWORD}/" docker-compose.yml
sed -i "s/MINIO_ROOT_PASSWORD=.*/MINIO_ROOT_PASSWORD=${MINIO_PASSWORD}/" docker-compose.yml

print_success "Environment variables تنظیم شدند"

# مرحله 8: راه‌اندازی Docker
print_info "[8/10] راه‌اندازی Docker services..."
docker compose down 2>/dev/null || true
docker compose up -d

# صبر برای ready شدن services
print_info "صبر برای ready شدن database..."
sleep 10

print_success "Docker services راه‌اندازی شدند"

# مرحله 9: Build Backend
print_info "[9/10] Build backend (این مرحله 5-10 دقیقه طول می‌کشد)..."
cd backend

# اجرای migrations
print_info "اجرای database migrations..."
export $(cat ../.env | xargs)
for migration in ../migrations/*.sql; do
    docker exec -i antidetect_postgres psql -U antidetect_user -d antidetect_db < "$migration" 2>/dev/null || true
done

# Build
cargo build --release
print_success "Backend build شد"

# مرحله 10: تنظیم systemd service
print_info "[10/10] تنظیم auto-start service..."

sudo tee /etc/systemd/system/antidetect-backend.service > /dev/null << EOF
[Unit]
Description=Anti-Detect Browser Backend
After=network.target docker.service
Requires=docker.service

[Service]
Type=simple
User=${SERVER_USER}
WorkingDirectory=${HOME}/antol/anti-detect-mvp/backend
EnvironmentFile=${HOME}/antol/anti-detect-mvp/.env
ExecStart=${HOME}/antol/anti-detect-mvp/backend/target/release/backend
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable antidetect-backend
sudo systemctl start antidetect-backend

print_success "Service تنظیم و شروع شد"

# تنظیم Nginx
print_info "تنظیم Nginx reverse proxy..."

sudo tee /etc/nginx/sites-available/antidetect-api > /dev/null << EOF
server {
    listen 80;
    server_name ${DOMAIN};

    client_max_body_size 100M;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;

        add_header 'Access-Control-Allow-Origin' '*' always;
        add_header 'Access-Control-Allow-Methods' 'GET, POST, PUT, DELETE, OPTIONS' always;
        add_header 'Access-Control-Allow-Headers' 'Authorization, Content-Type' always;
    }
}
EOF

sudo ln -sf /etc/nginx/sites-available/antidetect-api /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default
sudo nginx -t && sudo systemctl restart nginx

print_success "Nginx تنظیم شد"

# تنظیم Firewall
print_info "تنظیم Firewall..."
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
echo "y" | sudo ufw enable

print_success "Firewall تنظیم شد"

# خلاصه نهایی
echo ""
echo "========================================"
echo -e "${GREEN}✅ نصب با موفقیت کامل شد!${NC}"
echo "========================================"
echo ""
echo "🌐 URL های دسترسی:"
echo "   API: http://${DOMAIN}/api"
echo "   Health: http://${DOMAIN}/health"
echo ""
echo "📝 اطلاعات ورود پیش‌فرض:"
echo "   Email: admin@demo.com"
echo "   Password: admin123"
echo ""
echo "🔒 برای نصب SSL (HTTPS):"
echo "   sudo apt install -y certbot python3-certbot-nginx"
echo "   sudo certbot --nginx -d ${DOMAIN}"
echo ""
echo "📊 دستورات مفید:"
echo "   وضعیت backend: sudo systemctl status antidetect-backend"
echo "   Logs backend: sudo journalctl -u antidetect-backend -f"
echo "   Restart backend: sudo systemctl restart antidetect-backend"
echo ""
echo "⚙️  فایل تنظیمات: ${HOME}/antol/anti-detect-mvp/.env"
echo ""
echo "💾 پسوردهای شما:"
echo "   PostgreSQL: ${DB_PASSWORD}"
echo "   MinIO: ${MINIO_PASSWORD}"
echo "   JWT Secret: ${JWT_SECRET}"
echo ""
echo "⚠️  این پسوردها رو در یک جای امن ذخیره کنید!"
echo ""
echo "🎉 حالا می‌تونید backend رو تست کنید:"
echo "   curl http://${DOMAIN}/api/auth/login \\"
echo "     -X POST -H 'Content-Type: application/json' \\"
echo "     -d '{\"email\":\"admin@demo.com\",\"password\":\"admin123\"}'"
echo ""
