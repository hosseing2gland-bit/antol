#!/bin/bash
# اسکریپت راه‌اندازی کامل Backend روی سرور

SERVER_IP="108.143.173.222"
SERVER_USER="berellian"
SERVER_PASS="ABCDqwer1234"

echo "🚀 شروع راه‌اندازی Backend روی سرور..."

# تست اتصال
echo "📡 تست اتصال به سرور..."
sshpass -p "$SERVER_PASS" ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 $SERVER_USER@$SERVER_IP 'echo "اتصال موفق!"' || { echo "❌ خطا در اتصال به سرور"; exit 1; }

# بررسی وضعیت Backend
echo "🔍 بررسی وضعیت Backend..."
sshpass -p "$SERVER_PASS" ssh -o ConnectTimeout=5 $SERVER_USER@$SERVER_IP 'pgrep -f "target/release/backend" && echo "✅ Backend در حال اجرا" || echo "❌ Backend متوقف است"'

# بررسی لاگ
echo "📋 لاگ Backend:"
sshpass -p "$SERVER_PASS" ssh -o ConnectTimeout=5 $SERVER_USER@$SERVER_IP 'tail -10 /home/berellian/antol/anti-detect-mvp/backend.log 2>/dev/null || echo "فایل لاگ پیدا نشد"'

# تست API
echo "🧪 تست API از داخل سرور..."
sshpass -p "$SERVER_PASS" ssh -o ConnectTimeout=10 $SERVER_USER@$SERVER_IP 'timeout 5 curl -s http://localhost:3000/api || echo "API پاسخ نداد"'

echo "✅ تمام!"
