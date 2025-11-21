#!/bin/bash

echo "========================================"
echo "  Anti-Detect Browser - macOS Build"
echo "========================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check Node.js
echo -e "${BLUE}[1/5] Checking Node.js...${NC}"
if ! command -v node &> /dev/null; then
    echo -e "${RED}[ERROR] Node.js not found! Install with: brew install node${NC}"
    exit 1
fi
echo -e "${GREEN}[OK] Node.js $(node --version)${NC}"

# Check Rust
echo -e "${BLUE}[2/5] Checking Rust...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[ERROR] Rust not found! Install from https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}[OK] Rust $(rustc --version)${NC}"

# Check Xcode
echo -e "${BLUE}[3/5] Checking Xcode Command Line Tools...${NC}"
if ! xcode-select -p &> /dev/null; then
    echo -e "${RED}[ERROR] Xcode Command Line Tools not found!${NC}"
    echo "Run: xcode-select --install"
    exit 1
fi
echo -e "${GREEN}[OK] Xcode tools installed${NC}"

echo ""
echo "========================================"
echo "  Building Client App"
echo "========================================"
cd anti-detect-mvp/client-app || exit 1

echo -e "${BLUE}[4/5] Installing dependencies...${NC}"
npm install
if [ $? -ne 0 ]; then
    echo -e "${RED}[ERROR] npm install failed${NC}"
    exit 1
fi

echo -e "${BLUE}[5/5] Building Tauri app...${NC}"
npm run tauri build
if [ $? -ne 0 ]; then
    echo -e "${RED}[ERROR] Build failed${NC}"
    exit 1
fi

echo ""
echo "========================================"
echo "  Building Admin App"
echo "========================================"
cd ../admin-app || exit 1

echo -e "${BLUE}[4/5] Installing dependencies...${NC}"
npm install
if [ $? -ne 0 ]; then
    echo -e "${RED}[ERROR] npm install failed${NC}"
    exit 1
fi

echo -e "${BLUE}[5/5] Building Tauri app...${NC}"
npm run tauri build
if [ $? -ne 0 ]; then
    echo -e "${RED}[ERROR] Build failed${NC}"
    exit 1
fi

echo ""
echo "========================================"
echo -e "${GREEN}  Build Complete!${NC}"
echo "========================================"
echo ""
echo "Client App DMG:"
echo "  client-app/src-tauri/target/release/bundle/dmg/"
echo ""
echo "Admin App DMG:"
echo "  admin-app/src-tauri/target/release/bundle/dmg/"
echo ""
