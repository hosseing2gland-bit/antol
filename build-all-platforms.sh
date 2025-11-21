#!/bin/bash
# اسکریپت Build کامل برای Windows و macOS

echo "🚀 شروع Build برای Windows و macOS"
echo "======================================"

# رنگ‌ها برای خروجی
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# تابع نمایش پیام موفقیت
success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# تابع نمایش پیام خطا
error() {
    echo -e "${RED}❌ $1${NC}"
}

# تابع نمایش پیام هشدار
warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# بررسی وجود ابزارهای مورد نیاز
echo ""
echo "📋 بررسی ابزارهای مورد نیاز..."
echo "--------------------------------"

# Node.js
if command -v node &> /dev/null; then
    success "Node.js $(node --version)"
else
    error "Node.js نصب نیست"
    exit 1
fi

# npm
if command -v npm &> /dev/null; then
    success "npm $(npm --version)"
else
    error "npm نصب نیست"
    exit 1
fi

# Rust
if command -v rustc &> /dev/null; then
    success "Rust $(rustc --version | cut -d' ' -f2)"
else
    error "Rust نصب نیست"
    exit 1
fi

# Cargo
if command -v cargo &> /dev/null; then
    success "Cargo $(cargo --version | cut -d' ' -f2)"
else
    error "Cargo نصب نیست"
    exit 1
fi

echo ""
echo "🔧 Build Client App"
echo "==================="

cd /workspaces/antol/anti-detect-mvp/client-app || exit 1

# نصب dependencies
echo "📦 نصب Dependencies..."
npm install || { error "خطا در نصب dependencies"; exit 1; }
success "Dependencies نصب شدند"

# Build Frontend
echo ""
echo "🎨 Build Frontend (React + Vite)..."
npm run build || { error "خطا در build frontend"; exit 1; }
success "Frontend build شد"

# Build Tauri for Windows (cross-compile اگر روی لینوکس هستیم)
echo ""
echo "🪟 Build برای Windows..."
warning "برای build Windows از macOS یا Windows استفاده کنید"
warning "از Dev Container نمی‌توان به صورت cross-compile برای Windows build کرد"

# Build برای macOS (فقط روی macOS ممکن است)
echo ""
echo "🍎 Build برای macOS..."
warning "برای build macOS باید روی سیستم macOS باشید"
warning "از Dev Container نمی‌توان برای macOS build کرد"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 راهنمای Build برای هر پلتفرم"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo ""
echo "### 🪟 Build روی Windows:"
echo "1. کد را روی ماشین Windows کلون کنید"
echo "2. نصب کنید:"
echo "   - Node.js: https://nodejs.org"
echo "   - Rust: https://rustup.rs"
echo "   - WebView2: https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
echo "3. در PowerShell اجرا کنید:"
echo ""
echo "   cd anti-detect-mvp/client-app"
echo "   npm install"
echo "   npm run tauri build"
echo ""
echo "4. فایل‌های نصبی در این مسیر:"
echo "   src-tauri/target/release/bundle/msi/"
echo "   src-tauri/target/release/bundle/nsis/"
echo ""

echo "### 🍎 Build روی macOS:"
echo "1. کد را روی ماشین macOS کلون کنید"
echo "2. نصب کنید:"
echo "   brew install node"
echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
echo "3. در Terminal اجرا کنید:"
echo ""
echo "   cd anti-detect-mvp/client-app"
echo "   npm install"
echo "   npm run tauri build"
echo ""
echo "4. فایل نصبی در این مسیر:"
echo "   src-tauri/target/release/bundle/dmg/"
echo "   src-tauri/target/release/bundle/macos/"
echo ""

echo "### همینطور برای Admin App"
echo ""

echo ""
echo "🔧 Build Admin App"
echo "=================="

cd /workspaces/antol/anti-detect-mvp/admin-app || exit 1

# نصب dependencies
echo "📦 نصب Dependencies..."
npm install || { error "خطا در نصب dependencies"; exit 1; }
success "Dependencies نصب شدند"

# Build Frontend
echo ""
echo "🎨 Build Frontend (React + Vite)..."
npm run build || { error "خطا در build frontend"; exit 1; }
success "Frontend build شد"

echo ""
echo "✅ Build Frontend برای Client و Admin کامل شد!"
echo ""
echo "⚠️  توجه: برای build نهایی Tauri (installers) باید:"
echo "   - روی Windows: از ماشین Windows استفاده کنید"
echo "   - روی macOS: از ماشین macOS استفاده کنید"
echo ""
echo "📦 فایل‌های آماده برای توزیع:"
echo "   - client-app/dist/    (Frontend)"
echo "   - admin-app/dist/     (Frontend)"
echo ""
