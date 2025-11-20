#!/bin/bash

# رنگ‌ها برای خروجی
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== راه‌اندازی پروژه Anti-Detect MVP ===${NC}\n"

# تشخیص معماری Mac
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-apple-darwin"
    echo -e "${GREEN}✓ معماری شناسایی شد: Apple Silicon (M1/M2/M3)${NC}"
else
    TARGET="x86_64-apple-darwin"
    echo -e "${GREEN}✓ معماری شناسایی شد: Intel${NC}"
fi

# بررسی نصب ابزارها
echo -e "\n${YELLOW}بررسی ابزارهای مورد نیاز...${NC}"

# بررسی Homebrew
if ! command -v brew &> /dev/null; then
    echo -e "${RED}✗ Homebrew نصب نیست!${NC}"
    echo "برای نصب دستور زیر را اجرا کنید:"
    echo '/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
    exit 1
else
    echo -e "${GREEN}✓ Homebrew نصب شده${NC}"
fi

# بررسی Node.js
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}Node.js نصب نیست. در حال نصب...${NC}"
    brew install node@18
else
    echo -e "${GREEN}✓ Node.js نصب شده ($(node --version))${NC}"
fi

# بررسی Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust نصب نیست. در حال نصب...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo -e "${GREEN}✓ Rust نصب شده ($(rustc --version))${NC}"
fi

# اضافه کردن target
echo -e "\n${YELLOW}اضافه کردن target برای معماری شما...${NC}"
rustup target add $TARGET
echo -e "${GREEN}✓ Target $TARGET اضافه شد${NC}"

# بررسی PostgreSQL
if ! command -v psql &> /dev/null; then
    echo -e "${YELLOW}PostgreSQL نصب نیست. در حال نصب...${NC}"
    brew install postgresql@14
    brew services start postgresql@14
else
    echo -e "${GREEN}✓ PostgreSQL نصب شده${NC}"
    # اطمینان از اجرای سرویس
    brew services start postgresql@14 2>/dev/null
fi

# نصب sqlx-cli
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}نصب sqlx-cli...${NC}"
    cargo install sqlx-cli --no-default-features --features postgres
else
    echo -e "${GREEN}✓ sqlx-cli نصب شده${NC}"
fi

echo -e "\n${GREEN}=== راه‌اندازی Backend ===${NC}"
cd anti-detect-mvp/backend

# ایجاد .env اگر وجود ندارد
if [ ! -f .env ]; then
    echo -e "${YELLOW}ایجاد فایل .env...${NC}"
    cat > .env << 'EOF'
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/antidetect
JWT_SECRET=your-super-secret-key-change-this-in-production
RUST_LOG=info
EOF
    echo -e "${GREEN}✓ فایل .env ایجاد شد${NC}"
fi

# ایجاد دیتابیس
echo -e "${YELLOW}راه‌اندازی دیتابیس...${NC}"
createdb antidetect 2>/dev/null || echo "دیتابیس از قبل وجود دارد"
sqlx database create 2>/dev/null || true
sqlx migrate run || echo -e "${YELLOW}⚠ Migrations ممکن است قبلاً اجرا شده باشند${NC}"

cd ../..

echo -e "\n${GREEN}=== نصب Dependencies ===${NC}"

# Admin App
echo -e "${YELLOW}نصب dependencies Admin App...${NC}"
cd anti-detect-mvp/admin-app
npm install
cd ../..
echo -e "${GREEN}✓ Admin App آماده است${NC}"

# Client App
echo -e "${YELLOW}نصب dependencies Client App...${NC}"
cd anti-detect-mvp/client-app
npm install
cd ../..
echo -e "${GREEN}✓ Client App آماده است${NC}"

echo -e "\n${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║            راه‌اندازی با موفقیت انجام شد! 🎉            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${YELLOW}برای اجرای پروژه، یکی از روش‌های زیر را انتخاب کنید:${NC}\n"

echo -e "${GREEN}روش 1: اجرای دستی در ترمینال‌های جداگانه:${NC}"
echo -e "  ترمینال 1 (Backend):"
echo -e "    ${YELLOW}cd anti-detect-mvp/backend && cargo run --release${NC}"
echo -e ""
echo -e "  ترمینال 2 (License Server):"
echo -e "    ${YELLOW}cd anti-detect-mvp/license-server && cargo run --release${NC}"
echo -e ""
echo -e "  ترمینال 3 (Admin App):"
echo -e "    ${YELLOW}cd anti-detect-mvp/admin-app && npm run tauri dev${NC}"
echo -e ""
echo -e "  ترمینال 4 (Client App):"
echo -e "    ${YELLOW}cd anti-detect-mvp/client-app && npm run tauri dev${NC}"
echo -e ""

echo -e "${GREEN}روش 2: Build کردن برای Production:${NC}"
echo -e "  Admin App:"
echo -e "    ${YELLOW}cd anti-detect-mvp/admin-app && npm run tauri build -- --target $TARGET${NC}"
echo -e ""
echo -e "  Client App:"
echo -e "    ${YELLOW}cd anti-detect-mvp/client-app && npm run tauri build -- --target $TARGET${NC}"
echo -e ""

if command -v tmux &> /dev/null; then
    echo -e "${GREEN}روش 3: اجرا با tmux (اتوماتیک):${NC}"
    echo -e "    ${YELLOW}./run-with-tmux.sh${NC}"
    echo -e ""
fi

echo -e "${GREEN}روش 4: اجرا با Docker Compose:${NC}"
echo -e "    ${YELLOW}cd anti-detect-mvp && docker-compose up -d${NC}"
echo -e "    (توجه: Apps باید به صورت مستقیم اجرا شوند)"
echo -e ""

echo -e "${YELLOW}نکته:${NC} برای جزئیات بیشتر فایل ${GREEN}RUN_ON_MAC_LOCAL.md${NC} را مطالعه کنید."
