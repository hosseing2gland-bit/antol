#!/usr/bin/env bash
# Test script to verify network error fix

echo "=========================================="
echo "Network Error Fix - Verification Script"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_env_file() {
    local file=$1
    local key=$2
    local expected=$3
    
    if [ -f "$file" ]; then
        value=$(grep "^$key=" "$file" | cut -d'=' -f2)
        if [ "$value" = "$expected" ]; then
            echo -e "${GREEN}✓${NC} $file has correct $key: $value"
            return 0
        else
            echo -e "${RED}✗${NC} $file has incorrect $key: $value (expected: $expected)"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠${NC} $file not found"
        return 1
    fi
}

check_code_pattern() {
    local file=$1
    local pattern=$2
    local description=$3
    
    if [ -f "$file" ]; then
        if grep -q "$pattern" "$file"; then
            echo -e "${GREEN}✓${NC} $file contains $description"
            return 0
        else
            echo -e "${RED}✗${NC} $file missing $description"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠${NC} $file not found"
        return 1
    fi
}

echo "1. Checking Environment Files"
echo "------------------------------"
check_env_file ".env" "API_URL" "http://108.143.173.222:3000"
check_env_file "anti-detect-mvp/.env" "API_PORT" "3000"
check_env_file "anti-detect-mvp/admin-app/.env" "VITE_API_URL" "http://108.143.173.222:3000"
check_env_file "anti-detect-mvp/client-app/.env" "VITE_API_URL" "http://108.143.173.222:3000"
echo ""

echo "2. Checking Backend Code"
echo "------------------------"
check_code_pattern "anti-detect-mvp/backend/src/main.rs" "API_PORT" "configurable port via API_PORT"
check_code_pattern "anti-detect-mvp/backend/src/main.rs" "unwrap_or(3000)" "default port 3000"
echo ""

echo "3. Checking Admin App Code"
echo "--------------------------"
check_code_pattern "anti-detect-mvp/admin-app/src/api.ts" "import.meta.env.VITE_API_URL" "environment variable usage"
check_code_pattern "anti-detect-mvp/admin-app/src-tauri/tauri.conf.json" "localhost:3000" "localhost support in Tauri scope"
check_code_pattern "anti-detect-mvp/admin-app/src-tauri/tauri.conf.json" "108.143.173.222:3000" "remote server in Tauri scope"
echo ""

echo "4. Checking Client App Code"
echo "----------------------------"
check_code_pattern "anti-detect-mvp/client-app/src/api.ts" "import.meta.env.VITE_API_URL" "environment variable usage"
check_code_pattern "anti-detect-mvp/client-app/src-tauri/tauri.conf.json" "localhost:3000" "localhost support in Tauri scope"
check_code_pattern "anti-detect-mvp/client-app/src-tauri/tauri.conf.json" "108.143.173.222:3000" "remote server in Tauri scope"
echo ""

echo "5. Backend Compilation Test"
echo "----------------------------"
cd anti-detect-mvp/backend
if cargo check 2>&1 | grep -q "Finished"; then
    echo -e "${GREEN}✓${NC} Backend compiles successfully"
elif cargo check 2>&1 | grep -q "error"; then
    echo -e "${RED}✗${NC} Backend compilation failed - check cargo output"
else
    echo -e "${GREEN}✓${NC} Backend compiles successfully (with warnings)"
fi
cd ../..
echo ""

echo "=========================================="
echo "Verification Complete"
echo "=========================================="
echo ""
echo "Next Steps:"
echo "1. Make sure backend is running: cd anti-detect-mvp/backend && cargo run"
echo "2. Test connection: curl http://108.143.173.222:3000/"
echo "3. Run admin app: cd anti-detect-mvp/admin-app && npm run tauri dev"
echo ""
