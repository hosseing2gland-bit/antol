# 🪟 راهنمای Build برای Windows

## پیش‌نیازها

### 1. نصب Node.js
- دانلود از: https://nodejs.org/
- نسخه پیشنهادی: LTS (22.x یا بالاتر)
- نصب کنید و `node --version` را در PowerShell تست کنید

### 2. نصب Rust
```powershell
# دانلود و نصب rustup
# از https://rustup.rs/ دانلود کنید
# یا مستقیم:
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe
```

بعد از نصب:
```powershell
rustc --version
cargo --version
```

### 3. نصب Visual Studio Build Tools
**مهم**: Tauri نیاز به MSVC compiler داره

دانلود از: https://visualstudio.microsoft.com/downloads/

انتخاب کنید:
- ✅ Desktop development with C++
- ✅ Windows 10/11 SDK

### 4. نصب WebView2
معمولاً روی Windows 11 نصب هست. اگر نبود:
- دانلود از: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

## مراحل Build

### Client App (برنامه کاربر)

```powershell
# 1. رفتن به پوشه پروژه
cd anti-detect-mvp\client-app

# 2. نصب dependencies
npm install

# 3. Build frontend
npm run build

# 4. Build Tauri (نصب installer)
npm run tauri build
```

**⏱️ زمان build**: حدود 5-10 دقیقه (اولین بار بیشتر)

**📦 فایل‌های خروجی**:
```
src-tauri\target\release\bundle\msi\client-app_0.0.0_x64_en-US.msi
src-tauri\target\release\bundle\nsis\client-app_0.0.0_x64-setup.exe
```

### Admin App (برنامه مدیریت)

```powershell
# 1. رفتن به پوشه
cd ..\admin-app

# 2. نصب dependencies
npm install

# 3. Build frontend
npm run build

# 4. Build Tauri
npm run tauri build
```

**📦 فایل‌های خروجی**:
```
src-tauri\target\release\bundle\msi\admin-app_0.0.0_x64_en-US.msi
src-tauri\target\release\bundle\nsis\admin-app_0.0.0_x64-setup.exe
```

## تنظیمات قبل از Build

### تغییر API URL

اگر می‌خواهید به سرور واقعی متصل بشه، `.env` رو ویرایش کنید:

**client-app/.env**:
```env
VITE_API_URL=http://108.143.173.222:3000
```

**admin-app/.env**:
```env
VITE_API_URL=http://108.143.173.222:3000
```

## عیب‌یابی

### خطا: "MSVC compiler not found"
**راه‌حل**: Visual Studio Build Tools رو نصب کنید (بالا توضیح داده شد)

### خطا: "WebView2 not found"
**راه‌حل**: WebView2 Runtime نصب کنید

### خطا: npm install fails
**راه‌حل**:
```powershell
# پاک کردن cache
npm cache clean --force
Remove-Item -Recurse -Force node_modules
npm install
```

### Build خیلی طول می‌کشه
این طبیعیه! اولین build Rust می‌تونه 10-15 دقیقه طول بکشه.

## تست Installer

بعد از build:

```powershell
# رفتن به پوشه نصب
cd src-tauri\target\release\bundle\msi

# اجرای installer
.\client-app_0.0.0_x64_en-US.msi
```

## نکات مهم

- ✅ همیشه PowerShell رو **به عنوان Administrator** اجرا کنید
- ✅ اتصال اینترنت برای دانلود dependencies لازمه
- ✅ حداقل 5GB فضای خالی روی دیسک داشته باشید
- ✅ Antivirus ممکنه build رو کند کنه - موقتاً غیرفعالش کنید

## آپدیت برنامه

برای آپدیت نسخه:

**client-app/src-tauri/tauri.conf.json**:
```json
{
  "package": {
    "version": "1.0.1"  // تغییر نسخه
  }
}
```

بعد دوباره build کنید.

## توزیع

فایل‌های `.msi` یا `.exe` رو می‌تونید مستقیم به کاربران بدید.

**توصیه**: از `.msi` برای نصب رسمی استفاده کنید (قابل uninstall از Control Panel)
