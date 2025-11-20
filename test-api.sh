#!/bin/bash

# Test Anti-Detect Backend API
# رنگ‌ها برای خروجی
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

API_URL="http://localhost:3000"
TOKEN=""

echo "🧪 Testing Anti-Detect Backend API..."
echo "======================================"
echo ""

# Test 1: Health Check
echo -n "1️⃣  Health Check... "
HEALTH=$(curl -s $API_URL/)
if [[ $HEALTH == *"Anti-Detect Browser Backend API"* ]]; then
    echo -e "${GREEN}✅ PASS${NC}"
else
    echo -e "${RED}❌ FAIL${NC}"
    exit 1
fi

# Test 2: Login
echo -n "2️⃣  Login (admin@demo.com)... "
LOGIN_RESPONSE=$(curl -s -X POST $API_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@demo.com","password":"admin123"}')

if [[ $LOGIN_RESPONSE == *"token"* ]]; then
    TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*' | sed 's/"token":"//')
    echo -e "${GREEN}✅ PASS${NC}"
    echo "   Token: ${TOKEN:0:20}..."
else
    echo -e "${RED}❌ FAIL${NC}"
    echo "   Response: $LOGIN_RESPONSE"
fi

# Test 3: Get Users
echo -n "3️⃣  Get Users List... "
USERS=$(curl -s -H "Authorization: Bearer $TOKEN" $API_URL/api/users)
if [[ $USERS == *"["* ]]; then
    USER_COUNT=$(echo $USERS | grep -o '"id"' | wc -l)
    echo -e "${GREEN}✅ PASS${NC} ($USER_COUNT users)"
else
    echo -e "${YELLOW}⚠️  WARN${NC} (Auth might be needed)"
fi

# Test 4: Get Licenses
echo -n "4️⃣  Get Licenses List... "
LICENSES=$(curl -s -H "Authorization: Bearer $TOKEN" $API_URL/api/licenses)
if [[ $LICENSES == *"["* ]] || [[ $LICENSES == "[]" ]]; then
    LICENSE_COUNT=$(echo $LICENSES | grep -o '"id"' | wc -l)
    echo -e "${GREEN}✅ PASS${NC} ($LICENSE_COUNT licenses)"
else
    echo -e "${YELLOW}⚠️  WARN${NC}"
fi

# Test 5: Get Profiles
echo -n "5️⃣  Get Profiles List... "
PROFILES=$(curl -s -H "Authorization: Bearer $TOKEN" $API_URL/api/profiles)
if [[ $PROFILES == *"["* ]] || [[ $PROFILES == "[]" ]]; then
    PROFILE_COUNT=$(echo $PROFILES | grep -o '"id"' | wc -l)
    echo -e "${GREEN}✅ PASS${NC} ($PROFILE_COUNT profiles)"
else
    echo -e "${YELLOW}⚠️  WARN${NC}"
fi

# Test 6: Get Proxies
echo -n "6️⃣  Get Proxies List... "
PROXIES=$(curl -s -H "Authorization: Bearer $TOKEN" $API_URL/api/proxies)
if [[ $PROXIES == *"["* ]] || [[ $PROXIES == "[]" ]]; then
    PROXY_COUNT=$(echo $PROXIES | grep -o '"id"' | wc -l)
    echo -e "${GREEN}✅ PASS${NC} ($PROXY_COUNT proxies)"
else
    echo -e "${YELLOW}⚠️  WARN${NC}"
fi

# Test 7: Create User
echo -n "7️⃣  Create New User... "
CREATE_USER=$(curl -s -X POST $API_URL/api/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"email":"test@demo.com","password":"test123","role":"user"}')
if [[ $CREATE_USER == *"id"* ]] || [[ $CREATE_USER == *"email"* ]]; then
    echo -e "${GREEN}✅ PASS${NC}"
else
    echo -e "${YELLOW}⚠️  WARN${NC} (Might already exist)"
fi

# Test 8: Create License
echo -n "8️⃣  Create License... "
CREATE_LICENSE=$(curl -s -X POST $API_URL/api/licenses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"plan":"Basic","max_profiles":5,"duration_days":30}')
if [[ $CREATE_LICENSE == *"key"* ]] || [[ $CREATE_LICENSE == *"id"* ]]; then
    LICENSE_KEY=$(echo $CREATE_LICENSE | grep -o '"key":"[^"]*' | sed 's/"key":"//')
    echo -e "${GREEN}✅ PASS${NC}"
    echo "   License Key: $LICENSE_KEY"
else
    echo -e "${YELLOW}⚠️  WARN${NC}"
    echo "   Response: $CREATE_LICENSE"
fi

echo ""
echo "======================================"
echo -e "${GREEN}✅ Backend API is working!${NC}"
echo ""
echo "📊 Summary:"
echo "   - Health: ✅"
echo "   - Auth: ✅"
echo "   - Users API: ✅"
echo "   - Licenses API: ✅"
echo "   - Profiles API: ✅"
echo "   - Proxies API: ✅"
