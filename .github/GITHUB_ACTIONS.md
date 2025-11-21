# 🤖 GitHub Actions - Automatic Build

این workflow خودکار برنامه‌های Desktop رو برای Windows و macOS build می‌کنه.

## 🚀 اجرای خودکار

Workflow در این موارد اجرا می‌شه:

1. **Push به main branch**
2. **ایجاد Tag** (مثل `v1.0.0`)
3. **Pull Request به main**
4. **Manual** (از تب Actions)

## 📦 خروجی‌ها

### Windows
- ✅ Client App MSI installer
- ✅ Client App NSIS installer (.exe)
- ✅ Admin App MSI installer
- ✅ Admin App NSIS installer (.exe)

### macOS
- ✅ Client App DMG (Intel x86_64)
- ✅ Client App DMG (Apple Silicon ARM64)
- ✅ Admin App DMG (Intel x86_64)
- ✅ Admin App DMG (Apple Silicon ARM64)

## 📥 دانلود Artifacts

### از تب Actions:
1. رفتن به **Actions** در GitHub
2. انتخاب workflow run
3. دانلود artifacts از بخش **Artifacts**

### از Release:
اگر با tag push کنید (مثل `v1.0.0`):
```bash
git tag v1.0.0
git push origin v1.0.0
```

یک Release خودکار ساخته می‌شه با تمام installer ها.

## 🔧 تنظیمات محلی

### برای تست workflow در محلی:

```bash
# نصب act (GitHub Actions local runner)
brew install act  # macOS
# یا
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash  # Linux

# اجرای workflow
act push
```

## ⚙️ پیکربندی

### Environment Variables

اگر نیاز به code signing دارید، secrets رو در GitHub اضافه کنید:

**Settings → Secrets and variables → Actions**

#### Windows Code Signing:
- `WINDOWS_CERTIFICATE`: Base64 encoded certificate
- `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

#### macOS Code Signing:
- `APPLE_CERTIFICATE`: Base64 encoded certificate  
- `APPLE_CERTIFICATE_PASSWORD`: Certificate password
- `APPLE_ID`: Apple ID email
- `APPLE_PASSWORD`: App-specific password
- `APPLE_TEAM_ID`: Team ID

### تغییر Build برای Code Signing

به فایل `.github/workflows/build-apps.yml` اضافه کنید:

```yaml
# برای Windows
- name: Import Windows certificate
  run: |
    echo "${{ secrets.WINDOWS_CERTIFICATE }}" | base64 -d > cert.pfx
    
# برای macOS  
- name: Import Apple certificate
  run: |
    echo "${{ secrets.APPLE_CERTIFICATE }}" | base64 -d > cert.p12
    security import cert.p12 -P "${{ secrets.APPLE_CERTIFICATE_PASSWORD }}"
```

## 🐛 عیب‌یابی

### Build fail می‌شه

1. چک کنید logs در Actions tab
2. مطمئن شوید dependencies در `package.json` درست هستند
3. اطمینان حاصل کنید `tauri.conf.json` صحیح است

### Artifact upload نمی‌شه

مسیر فایل‌ها رو چک کنید:
```yaml
path: anti-detect-mvp/${{ matrix.app }}/src-tauri/target/release/bundle/msi/*.msi
```

### macOS build خیلی طول می‌کشه

طبیعیه! Build برای ARM64 + x86_64 حدود 20-30 دقیقه طول می‌کشه.

## 📊 زمان Build (تقریبی)

| Platform | Time |
|----------|------|
| Windows (هر app) | ~8-12 دقیقه |
| macOS (هر app) | ~15-25 دقیقه |
| **کل** | ~45-75 دقیقه |

## 💡 نکات

1. **Parallel Builds**: Windows و macOS همزمان build می‌شن
2. **Matrix Strategy**: Client و Admin app همزمان build می‌شن
3. **Cache**: Node modules cache می‌شه برای سرعت بیشتر
4. **Artifacts**: 90 روز نگهداری می‌شن

## 🔄 به‌روزرسانی نسخه

قبل از ایجاد release:

1. **بروزرسانی نسخه در package.json**:
```json
{
  "version": "1.0.1"
}
```

2. **بروزرسانی نسخه در tauri.conf.json**:
```json
{
  "package": {
    "version": "1.0.1"
  }
}
```

3. **ایجاد tag و push**:
```bash
git add .
git commit -m "Release v1.0.1"
git tag v1.0.1
git push origin main --tags
```

## 📝 مثال استفاده

```bash
# Clone repository
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol

# ایجاد تغییرات
# ...

# Commit و push
git add .
git commit -m "Add new feature"
git push origin main

# منتظر بمانید تا build تمام شه (~1 ساعت)
# Artifacts رو از Actions tab دانلود کنید
```

## 🎯 Release Workflow

```bash
# 1. بروزرسانی نسخه
npm version patch  # یا minor یا major

# 2. Commit
git add .
git commit -m "Bump version to $(node -p "require('./package.json').version")"

# 3. ایجاد tag
git tag v$(node -p "require('./package.json').version")

# 4. Push
git push origin main --tags

# 5. منتظر بمانید
# Release خودکار در GitHub ساخته می‌شه
```
