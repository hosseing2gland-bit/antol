#!/bin/bash

# راهنمای نصب Systemd Service برای Backend

echo "========================================="
echo "📦 نصب Systemd Service برای Backend"
echo "========================================="
echo ""

# رنگ‌ها
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}مرحله 1: کپی فایل service${NC}"
sudo cp /home/berellian/antidetect-backend.service /etc/systemd/system/
echo -e "${GREEN}✅ فایل کپی شد${NC}"
echo ""

echo -e "${BLUE}مرحله 2: Reload systemd${NC}"
sudo systemctl daemon-reload
echo -e "${GREEN}✅ Systemd reload شد${NC}"
echo ""

echo -e "${BLUE}مرحله 3: فعال‌سازی service${NC}"
sudo systemctl enable antidetect-backend.service
echo -e "${GREEN}✅ Service فعال شد (اجرا در startup)${NC}"
echo ""

echo -e "${BLUE}مرحله 4: شروع service${NC}"
sudo systemctl start antidetect-backend.service
echo -e "${GREEN}✅ Service شروع شد${NC}"
echo ""

echo -e "${BLUE}مرحله 5: چک کردن وضعیت${NC}"
sudo systemctl status antidetect-backend.service
echo ""

echo "========================================="
echo -e "${GREEN}📝 دستورات مفید:${NC}"
echo "========================================="
echo "• مشاهده وضعیت:    sudo systemctl status antidetect-backend"
echo "• توقف service:    sudo systemctl stop antidetect-backend"
echo "• شروع service:    sudo systemctl start antidetect-backend"
echo "• Restart:         sudo systemctl restart antidetect-backend"
echo "• مشاهده لاگ:      sudo journalctl -u antidetect-backend -f"
echo "• غیرفعال کردن:    sudo systemctl disable antidetect-backend"
echo ""
echo -e "${GREEN}✅ نصب کامل شد!${NC}"
