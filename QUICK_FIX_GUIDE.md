# خلاصه راه‌حل خطای شبکه / Network Error Fix Summary

## مشکل / Problem
```
Network Error: Cannot connect to server. Please check:
1. Your internet connection
2. VPN if enabled  
3. Server is running at http://108.143.173.222:3000
```

## علت اصلی / Root Cause
1. **تفاوت پورت** - Backend روی پورت 3000 اجرا می‌شود ولی فایل‌های .env پورت 8000 را مشخص کرده بودند
2. **URL ثابت در کد** - آدرس API به صورت ثابت در کد نوشته شده بود و از متغیر محیطی استفاده نمی‌کرد
3. **محدودیت Tauri** - Tauri فقط اجازه دسترسی به یک پورت را می‌داد

## راه‌حل / Solution
✅ همه فایل‌های پیکربندی به‌روزرسانی شدند
✅ Backend حالا پورت را از متغیر محیطی می‌خواند
✅ Apps حالا URL را از متغیر محیطی می‌خوانند
✅ Tauri از چندین پورت پشتیبانی می‌کند

## چگونه استفاده کنیم / How to Use

### 1. اجرای Backend / Run Backend
```bash
cd anti-detect-mvp/backend
cargo run
```

Backend روی `http://108.143.173.222:3000` اجرا می‌شود

### 2. اجرای Admin App / Run Admin App
```bash
cd anti-detect-mvp/admin-app
npm run tauri dev
```

### 3. اجرای Client App / Run Client App
```bash
cd anti-detect-mvp/client-app
npm run tauri dev
```

## تغییر پورت Backend / Change Backend Port
اگر می‌خواهید پورت را تغییر دهید:

1. فایل `anti-detect-mvp/.env` را ویرایش کنید:
```env
API_PORT=8000
```

2. فایل‌های `.env` برنامه‌ها را هم تغییر دهید:
```env
# در anti-detect-mvp/admin-app/.env
VITE_API_URL=http://108.143.173.222:8000

# در anti-detect-mvp/client-app/.env  
VITE_API_URL=http://108.143.173.222:8000
```

3. Backend و Apps را مجدداً اجرا کنید

## تست اتصال / Test Connection
```bash
# تست Backend
curl http://108.143.173.222:3000/

# باید برگرداند / Should return:
# "Anti-Detect Browser Backend API"
```

## فایل‌های مهم تغییر یافته / Important Changed Files
- ✅ `anti-detect-mvp/backend/src/main.rs` - پورت قابل تنظیم
- ✅ `anti-detect-mvp/admin-app/src/api.ts` - URL از env می‌خواند
- ✅ `anti-detect-mvp/client-app/src/api.ts` - URL از env می‌خواند
- ✅ `anti-detect-mvp/admin-app/src-tauri/tauri.conf.json` - scope گسترده‌تر
- ✅ `anti-detect-mvp/client-app/src-tauri/tauri.conf.json` - scope گسترده‌تر
- ✅ همه فایل‌های `.env` - پورت 3000 یکسان

## مستندات کامل / Full Documentation
برای جزئیات بیشتر به `NETWORK_ERROR_FIX.md` مراجعه کنید

## اسکریپت تست / Test Script
برای تست خودکار اجرا کنید:
```bash
./verify-network-fix.sh
```
