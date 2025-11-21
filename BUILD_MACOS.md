# 🍎 راهنمای Build برای macOS

## پیش‌نیازها

### 1. نصب Xcode Command Line Tools
```bash
xcode-select --install
```

تأیید نصب:
```bash
xcode-select -p
# باید نشان دهد: /Library/Developer/CommandLineTools
```

### 2. نصب Homebrew
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 3. نصب Node.js
```bash
brew install node
node --version  # باید 18+ باشه
npm --version
```

### 4. نصب Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

نصب target برای macOS:
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin  # برای M1/M2/M3 Mac
```

## مراحل Build

### Client App (برنامه کاربر)

```bash
# 1. رفتن به پوشه پروژه
cd anti-detect-mvp/client-app

# 2. نصب dependencies
npm install

# 3. Build frontend
npm run build

# 4. Build Tauri
npm run tauri build
```

**⏱️ زمان build**: 5-15 دقیقه

**📦 فایل‌های خروجی**:
```
src-tauri/target/release/bundle/dmg/client-app_0.0.0_x64.dmg
src-tauri/target/release/bundle/macos/client-app.app
```

### Admin App (برنامه مدیریت)

```bash
# 1. رفتن به پوشه
cd ../admin-app

# 2. نصب dependencies
npm install

# 3. Build frontend
npm run build

# 4. Build Tauri
npm run tauri build
```

**📦 فایل‌های خروجی**:
```
src-tauri/target/release/bundle/dmg/admin-app_0.0.0_x64.dmg
src-tauri/target/release/bundle/macos/admin-app.app
```

## Build برای Apple Silicon (M1/M2/M3)

اگر روی Mac با چیپ Apple Silicon هستید:

```bash
# Build برای ARM64
npm run tauri build -- --target aarch64-apple-darwin
```

خروجی:
```
src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/client-app_0.0.0_aarch64.dmg
```

## Build Universal Binary (Intel + Apple Silicon)

برای ساخت برنامه‌ای که روی هر دو معماری کار کنه:

```bash
# نصب هر دو target
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build universal
npm run tauri build -- --target universal-apple-darwin
```

## تنظیمات قبل از Build

### تغییر API URL

**client-app/.env**:
```env
VITE_API_URL=http://108.143.173.222:3000
```

**admin-app/.env**:
```env
VITE_API_URL=http://108.143.173.222:3000
```

### Code Signing (اختیاری ولی توصیه می‌شه)

برای توزیع خارج از App Store:

1. **ثبت نام Apple Developer** ($99/سال)
2. **ایجاد Certificate**:
   - وارد https://developer.apple.com شوید
   - Certificates, Identifiers & Profiles
   - Create a new Certificate: "Developer ID Application"

3. **تنظیم در tauri.conf.json**:

```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

### Notarization (برای macOS 10.15+)

بعد از build:

```bash
# Submit برای notarization
xcrun notarytool submit \
  src-tauri/target/release/bundle/dmg/client-app_0.0.0_x64.dmg \
  --apple-id "your-email@example.com" \
  --password "app-specific-password" \
  --team-id "YOUR_TEAM_ID" \
  --wait

# Staple نتیجه
xcrun stapler staple src-tauri/target/release/bundle/dmg/client-app_0.0.0_x64.dmg
```

## عیب‌یابی

### خطا: "xcrun: error: invalid active developer path"
**راه‌حل**:
```bash
xcode-select --install
```

### خطا: "command not found: cargo"
**راه‌حل**:
```bash
source $HOME/.cargo/env
# یا اضافه کردن به ~/.zshrc یا ~/.bash_profile
echo 'source $HOME/.cargo/env' >> ~/.zshrc
```

### خطا: npm install fails
**راه‌حل**:
```bash
# پاک کردن cache
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

### خطا: "Developer cannot be verified" هنگام باز کردن برنامه
**راه‌حل موقت**:
```bash
# حذف quarantine attribute
xattr -cr client-app.app
```

یا:
- System Preferences → Security & Privacy → "Open Anyway"

**راه‌حل دائمی**: Code signing و notarization

### Build برای architecture دیگر روی M1 Mac
```bash
# برای Intel
arch -x86_64 npm run tauri build -- --target x86_64-apple-darwin
```

## تست Build

```bash
# باز کردن DMG
open src-tauri/target/release/bundle/dmg/client-app_0.0.0_x64.dmg

# یا اجرای مستقیم .app
open src-tauri/target/release/bundle/macos/client-app.app
```

## ایجاد DMG سفارشی

برای ظاهر بهتر DMG:

**src-tauri/tauri.conf.json**:
```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "dmg": {
          "background": "../assets/dmg-background.png",
          "windowSize": {
            "width": 600,
            "height": 400
          }
        }
      }
    }
  }
}
```

## آپدیت نسخه

**src-tauri/tauri.conf.json**:
```json
{
  "package": {
    "version": "1.0.1"
  }
}
```

## نکات مهم

- ✅ اولین build ممکنه 15-20 دقیقه طول بکشه
- ✅ برای Mac M1/M2/M3 حتماً target مناسب رو انتخاب کنید
- ✅ بدون code signing، کاربران باید "Open Anyway" رو بزنن
- ✅ DMG فایل معمولی برای توزیع در macOS هست
- ✅ حداقل 10GB فضای خالی داشته باشید

## توزیع

### بدون Code Signing
- فقط DMG رو به کاربران بدید
- کاربران باید در Security & Privacy اجازه بدن

### با Code Signing + Notarization
- DMG امضا شده رو مستقیم توزیع کنید
- نیازی به تأیید manual نیست

## بهینه‌سازی سایز

```bash
# Strip symbols برای کاهش حجم
strip src-tauri/target/release/bundle/macos/client-app.app/Contents/MacOS/client-app
```

معمولاً سایز نهایی: **20-40 MB** (بسته به dependencies)
