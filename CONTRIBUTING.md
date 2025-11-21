# 🤝 Contributing Guide

از اینکه می‌خواهید به این پروژه کمک کنید متشکریم! 🎉

## 📋 فهرست

1. [Code of Conduct](#code-of-conduct)
2. [چطور شروع کنم؟](#چطور-شروع-کنم)
3. [Development Setup](#development-setup)
4. [Contribution Workflow](#contribution-workflow)
5. [Coding Standards](#coding-standards)
6. [Testing](#testing)
7. [Documentation](#documentation)

---

## Code of Conduct

- محترمانه رفتار کنید
- از زبان توهین‌آمیز خودداری کنید
- به نظرات دیگران احترام بگذارید
- روی حل مسئله تمرکز کنید نه حمله شخصی

---

## چطور شروع کنم؟

### برای مبتدی‌ها:

1. **مستندات را بخوانید:**
   - `README.md`
   - `QUICK_START.md`
   - `FEATURES.md`

2. **Issues با label `good first issue` را ببینید:**
   ```bash
   # در GitHub Issues
   label:good-first-issue
   ```

3. **در Discussion ها شرکت کنید:**
   - سوال بپرسید
   - ایده بدهید
   - تجربه به اشتراک بگذارید

---

## Development Setup

### پیش‌نیازها:

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js
# دانلود از nodejs.org

# PostgreSQL
# macOS: brew install postgresql
# Ubuntu: sudo apt install postgresql

# Redis
# macOS: brew install redis
# Ubuntu: sudo apt install redis-server
```

### Clone و Setup:

```bash
# Clone repository
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol

# Backend setup
cd anti-detect-mvp/backend
cargo build
sqlx migrate run

# Admin App setup
cd ../admin-app
npm install

# Client App setup
cd ../client-app
npm install
```

---

## Contribution Workflow

### 1. Fork Repository

```bash
# Fork روی GitHub
# سپس clone کنید:
git clone https://github.com/YOUR_USERNAME/antol.git
cd antol
git remote add upstream https://github.com/hosseing2gland-bit/antol.git
```

### 2. ایجاد Branch

```bash
# برای feature جدید:
git checkout -b feature/amazing-feature

# برای bug fix:
git checkout -b fix/bug-description

# برای documentation:
git checkout -b docs/update-readme
```

### 3. توسعه

```bash
# کد بنویسید
# تست کنید
# Commit کنید

git add .
git commit -m "Add amazing feature"
```

### 4. Push و Pull Request

```bash
# Push به fork خودتان
git push origin feature/amazing-feature

# سپس در GitHub یک Pull Request باز کنید
```

---

## Coding Standards

### Rust Code:

```rust
// استفاده از rustfmt
cargo fmt

// استفاده از clippy
cargo clippy -- -D warnings

// نام‌گذاری:
// - snake_case برای functions و variables
// - PascalCase برای structs و enums
// - SCREAMING_SNAKE_CASE برای constants

// مثال:
pub struct ProfileConfig {
    pub user_agent: String,
    pub screen_width: u32,
}

pub fn generate_fingerprint() -> FingerprintConfig {
    // ...
}

const MAX_PROFILES: usize = 100;
```

### TypeScript/React:

```typescript
// استفاده از prettier
npm run format

// نام‌گذاری:
// - camelCase برای variables و functions
// - PascalCase برای Components
// - UPPER_CASE برای constants

// مثال:
interface ProfileConfig {
    userAgent: string;
    screenWidth: number;
}

function generateFingerprint(): FingerprintConfig {
    // ...
}

const MAX_PROFILES = 100;

// Components:
export default function ProfileList() {
    // ...
}
```

### Commit Messages:

```bash
# فرمت:
<type>: <description>

[optional body]

[optional footer]

# Types:
# feat: قابلیت جدید
# fix: رفع باگ
# docs: تغییرات در documentation
# style: formatting, missing semi colons, etc
# refactor: refactoring code
# test: اضافه کردن tests
# chore: updating build tasks, etc

# مثال‌ها:
feat: add canvas fingerprint noise

fix: resolve timezone offset calculation bug

docs: update README with new features

refactor: simplify fingerprint generation logic
```

---

## Testing

### Backend Tests:

```bash
cd anti-detect-mvp/backend
cargo test
```

### Frontend Tests:

```bash
cd anti-detect-mvp/client-app
npm test
```

### Manual Testing:

1. راه‌اندازی backend
2. راه‌اندازی client app
3. ایجاد پروفایل
4. Launch browser
5. تست anti-detection:
   - https://whoer.net
   - https://browserleaks.com

---

## Documentation

### کد باید خود-توضیح دهنده باشد:

```rust
/// Generate a random but realistic fingerprint configuration
/// 
/// # Returns
/// A `FingerprintConfig` with randomized values for all parameters
/// 
/// # Example
/// ```
/// let fingerprint = FingerprintConfig::generate_random();
/// assert!(fingerprint.screen_width > 0);
/// ```
pub fn generate_random() -> Self {
    // ...
}
```

### Documentation Files:

اگر قابلیت جدیدی اضافه می‌کنید:

- `README.md` را آپدیت کنید
- `FEATURES.md` را آپدیت کنید
- `CHANGELOG.md` را آپدیت کنید

---

## نکات مهم

### ✅ DO:
- کد تمیز بنویسید
- تست اضافه کنید
- Documentation بنویسید
- Commit messages واضح
- یک issue در هر PR

### ❌ DON'T:
- کدهای commented-out
- Console.log در production
- Hardcoded secrets
- Breaking changes بدون discussion

---

## سوالات؟

- GitHub Issues باز کنید
- در Discussions شرکت کنید
- Email: hossein@example.com

---

**ممنون از مشارکت شما! 🙏**
