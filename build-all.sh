#!/bin/bash

# 🚀 Build Script برای Production
# این اسکریپت همه چیز رو build می‌کنه

set -e

echo "🔨 Starting Production Build..."
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# ======================================
# 1. Backend Build
# ======================================
echo -e "${BLUE}📦 Building Backend...${NC}"
cd anti-detect-mvp/backend
cargo build --release
echo -e "${GREEN}✅ Backend built successfully${NC}"
echo ""

# ======================================
# 2. Admin App Build
# ======================================
echo -e "${BLUE}📦 Building Admin App...${NC}"
cd ../admin-app
npm install --production=false
npm run build
npm run tauri build
echo -e "${GREEN}✅ Admin App built successfully${NC}"
echo ""

# ======================================
# 3. Client App Build
# ======================================
echo -e "${BLUE}📦 Building Client App...${NC}"
cd ../client-app
npm install --production=false
npm run build
npm run tauri build
echo -e "${GREEN}✅ Client App built successfully${NC}"
echo ""

# ======================================
# 4. Collect Artifacts
# ======================================
echo -e "${BLUE}📦 Collecting build artifacts...${NC}"
cd ../..
mkdir -p dist/{backend,admin,client}

# Backend
cp anti-detect-mvp/backend/target/release/backend dist/backend/ 2>/dev/null || echo "Backend binary not found"

# Admin App (Windows)
if [ -f "anti-detect-mvp/admin-app/src-tauri/target/release/bundle/nsis/admin-app_0.0.0_x64-setup.exe" ]; then
    cp anti-detect-mvp/admin-app/src-tauri/target/release/bundle/nsis/admin-app_0.0.0_x64-setup.exe dist/admin/
    echo "✓ Admin App (Windows installer) copied"
fi

# Admin App (macOS)
if [ -f "anti-detect-mvp/admin-app/src-tauri/target/release/bundle/dmg/admin-app_0.0.0_x64.dmg" ]; then
    cp anti-detect-mvp/admin-app/src-tauri/target/release/bundle/dmg/admin-app_0.0.0_x64.dmg dist/admin/
    echo "✓ Admin App (macOS dmg) copied"
fi

# Client App (Windows)
if [ -f "anti-detect-mvp/client-app/src-tauri/target/release/bundle/nsis/client-app_0.0.0_x64-setup.exe" ]; then
    cp anti-detect-mvp/client-app/src-tauri/target/release/bundle/nsis/client-app_0.0.0_x64-setup.exe dist/client/
    echo "✓ Client App (Windows installer) copied"
fi

# Client App (macOS)
if [ -f "anti-detect-mvp/client-app/src-tauri/target/release/bundle/dmg/client-app_0.0.0_x64.dmg" ]; then
    cp anti-detect-mvp/client-app/src-tauri/target/release/bundle/dmg/client-app_0.0.0_x64.dmg dist/client/
    echo "✓ Client App (macOS dmg) copied"
fi

echo ""
echo -e "${GREEN}🎉 Build completed successfully!${NC}"
echo ""
echo "📦 Build artifacts are in: ./dist/"
echo ""
ls -lh dist/*/
