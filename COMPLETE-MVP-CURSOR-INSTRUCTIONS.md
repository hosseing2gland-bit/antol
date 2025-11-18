# پروژه Anti-Detect Browser MVP - دستورالعمل کامل برای Cursor Agent

## 📋 مشخصات کلی پروژه

### هدف
ساخت یک سیستم مدیریت چند حساب کاربری (Multi-Account Management) با قابلیت Anti-Detection مشابه GoLogin از صفر تا صد با بالاترین استانداردهای امنیتی Enterprise-Grade.

### کلاینت‌های مورد نیاز
1. **Admin App** - اپلیکیشن دسکتاپ مدیریتی (Windows/macOS)
2. **Client App** - اپلیکیشن دسکتاپ کاربر نهایی (Windows/macOS)

### اولویت‌های امنیتی
- ✅ Memory Safety (Rust)
- ✅ Anti-Reverse Engineering
- ✅ Encrypted Storage
- ✅ Hardware-Bound Licensing
- ✅ Anti-Debug & Anti-VM
- ✅ Code Signing
- ✅ Secure IPC
- ✅ Zero-Trust Architecture

---

## 🏗️ معماری فنی

### Stack انتخابی

#### Backend
- **زبان**: Rust
- **فریم‌ورک**: Axum (یا Actix-web)
- **دلیل انتخاب**: 
  - Memory safety بدون Garbage Collector
  - Performance بالا
  - Compiled binary (سخت‌تر برای Reverse Engineering)
  - Zero-cost abstractions

#### Desktop Applications
- **فریم‌ورک**: Tauri 2.0
- **Frontend**: React + TypeScript
- **دلیل انتخاب**:
  - Backend Rust (امنیت بالا)
  - حجم کم (5-10MB vs 100+MB Electron)
  - OS Webview (بدون نیاز به bundle کردن browser)
  - Sandboxed IPC
  - Built-in CSP و امنیت

#### Database
- **Primary**: PostgreSQL 16+
- **Cache**: Redis 7+
- **دلیل انتخاب**:
  - ACID compliance
  - Row-level security
  - Encryption at rest
  - Enterprise-proven

#### Storage
- **Solution**: MinIO (S3-compatible)
- **دلیل انتخاب**:
  - Self-hosted
  - Encryption
  - Versioning
  - IAM policies

#### Custom Browser
- **Base**: fingerprint-chromium
- **Build Process**: Modified Chromium with CLI flags
- **Distribution**: Bundled with client app

---

## 📁 ساختار کامل پروژه

```
anti-detect-mvp/
├── backend/                          # Rust Backend API Server
│   ├── Cargo.toml
│   ├── migrations/
│   │   ├── 001_init.sql
│   │   ├── 002_profiles.sql
│   │   └── 003_licenses.sql
│   └── src/
│       ├── main.rs
│       ├── models/
│       ├── handlers/
│       ├── services/
│       ├── middleware/
│       ├── db/
│       └── security/
│
├── license-server/                   # License Validation Server
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── license_gen.rs
│       ├── hardware_bind.rs
│       ├── validation.rs
│       └── analytics.rs
│
├── admin-app/                        # Admin Desktop (Tauri)
│   ├── src-tauri/                   # Rust backend
│   │   └── src/
│   │       ├── main.rs
│   │       ├── commands/
│   │       └── security/
│   └── src/                         # React frontend
│       ├── components/
│       ├── hooks/
│       └── services/
│
├── client-app/                       # Client Desktop (Tauri)
│   ├── src-tauri/                   # Rust backend
│   │   └── src/
│   │       ├── main.rs
│   │       ├── commands/
│   │       ├── browser/
│   │       └── security/
│   └── src/                         # React frontend
│       ├── components/
│       └── hooks/
│
├── chromium-builds/                  # Custom Chromium
│   ├── build.sh
│   ├── patches/
│   └── binaries/
│
└── infrastructure/
    ├── docker/
    └── scripts/
```

---

## 🔐 امنیت - لایه‌های حفاظتی

### 1. License System (Hardware-Bound)
```rust
// license-server/src/license_gen.rs
pub struct License {
    pub key: String,              // Encrypted license key
    pub hardware_id: String,      // CPU-ID + MAC + BIOS hash
    pub expiry: DateTime<Utc>,
    pub features: Vec<Feature>,
}

pub fn generate_license(hw_id: &str) -> Result<License> {
    // Use Argon2id for key derivation
    // Bind to hardware fingerprint
    // Sign with Ed25519
}
```

### 2. Anti-Debug & Anti-VM
```rust
// client-app/src-tauri/src/security/anti_debug.rs
pub fn detect_debugger() -> bool {
    #[cfg(windows)]
    {
        unsafe {
            use winapi::um::debugapi::IsDebuggerPresent;
            IsDebuggerPresent() != 0
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // Use sysctl to detect debugger
        check_sysctl_debugger()
    }
}

pub fn detect_vm() -> bool {
    // Check for VMware, VirtualBox, QEMU artifacts
    check_vm_artifacts()
}
```

### 3. Encrypted Storage
```rust
// client-app/src-tauri/src/security/encrypted_storage.rs
use aes_gcm::{Aes256Gcm, KeyInit};

pub struct SecureStorage {
    cipher: Aes256Gcm,
}

impl SecureStorage {
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // AES-256-GCM encryption
    }
    
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Decrypt with authentication
    }
}
```

### 4. Code Obfuscation
```toml
# Cargo.toml
[profile.release]
strip = "debuginfo"
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

### 5. IPC Security
```rust
// Tauri commands با access control
#[tauri::command]
#[require_auth]
pub async fn launch_browser(
    profile_id: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    // Validate license
    // Check permissions
    // Launch browser securely
}
```

---

## 🎯 Backend API - نقاط پایانی اصلی

### Authentication
```rust
POST /api/auth/register
POST /api/auth/login
POST /api/auth/refresh
POST /api/auth/logout
```

### Profiles
```rust
GET    /api/profiles
POST   /api/profiles
GET    /api/profiles/:id
PUT    /api/profiles/:id
DELETE /api/profiles/:id
POST   /api/profiles/:id/launch
POST   /api/profiles/:id/stop
```

### Admin
```rust
GET    /api/admin/licenses
POST   /api/admin/licenses
DELETE /api/admin/licenses/:id
GET    /api/admin/users
GET    /api/admin/analytics
```

---

## 🚀 Fingerprint Management

### Profile Configuration
```rust
pub struct ProfileConfig {
    // Platform
    pub platform: String,           // "windows" | "macos" | "linux"
    pub platform_version: String,   // "10.0" | "14.0"
    
    // Browser
    pub browser_version: String,    // "131.0.6778.86"
    pub user_agent: String,
    
    // Hardware
    pub hardware_concurrency: u8,   // CPU cores
    pub device_memory: u8,          // RAM in GB
    
    // Screen
    pub screen_width: u16,
    pub screen_height: u16,
    pub color_depth: u8,
    
    // Geolocation
    pub timezone: String,           // "America/New_York"
    pub language: String,           // "en-US"
    pub geolocation: Option<GeoLocation>,
    
    // Canvas & WebGL
    pub canvas_noise: f64,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    
    // WebRTC
    pub webrtc_leak_protection: bool,
    pub fake_media_devices: bool,
    
    // Proxy
    pub proxy: Option<ProxyConfig>,
}
```

### Browser Launch
```rust
pub async fn launch_browser(profile: &Profile) -> Result<Process> {
    let chromium_path = get_chromium_binary_path();
    let user_data_dir = get_profile_data_dir(&profile.id);
    
    let args = vec![
        format!("--fingerprint={}", profile.fingerprint_seed),
        format!("--fingerprint-platform={}", profile.config.platform),
        format!("--fingerprint-platform-version={}", profile.config.platform_version),
        format!("--fingerprint-brand-version={}", profile.config.browser_version),
        format!("--fingerprint-hardware-concurrency={}", profile.config.hardware_concurrency),
        format!("--timezone={}", profile.config.timezone),
        format!("--lang={}", profile.config.language),
        format!("--user-data-dir={}", user_data_dir),
    ];
    
    if let Some(proxy) = &profile.config.proxy {
        args.push(format!("--proxy-server={}://{}:{}", 
            proxy.protocol, proxy.host, proxy.port));
    }
    
    Command::new(chromium_path)
        .args(&args)
        .spawn()
}
```

---

## 📦 Database Schema

### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    license_key VARCHAR(255) UNIQUE,
    hardware_id VARCHAR(255),
    subscription_tier VARCHAR(50) DEFAULT 'free',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

### Profiles Table
```sql
CREATE TABLE profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    fingerprint_config JSONB NOT NULL,
    proxy_id UUID REFERENCES proxies(id),
    tags TEXT[],
    notes TEXT,
    last_used TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, name)
);
```

### Proxies Table
```sql
CREATE TABLE proxies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    protocol VARCHAR(20) NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    country VARCHAR(2),
    created_at TIMESTAMP DEFAULT NOW()
);
```

### Licenses Table
```sql
CREATE TABLE licenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    license_key VARCHAR(255) UNIQUE NOT NULL,
    hardware_id VARCHAR(255),
    max_profiles INTEGER NOT NULL DEFAULT 10,
    features JSONB,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    activated_at TIMESTAMP,
    last_validation TIMESTAMP
);
```

---

## 🎨 UI Components - Admin App

### Dashboard
```tsx
// admin-app/src/components/Dashboard.tsx
export const Dashboard = () => {
  const { licenses, users, analytics } = useAdminData();
  
  return (
    <div className="dashboard">
      <StatsCard 
        title="Active Licenses" 
        value={licenses.active} 
      />
      <StatsCard 
        title="Total Users" 
        value={users.total} 
      />
      <RevenueChart data={analytics.revenue} />
      <RecentActivity activities={analytics.activities} />
    </div>
  );
};
```

### License Manager
```tsx
// admin-app/src/components/LicenseManager.tsx
export const LicenseManager = () => {
  const [licenses, setLicenses] = useState<License[]>([]);
  
  const generateLicense = async () => {
    const response = await invoke('generate_license', {
      maxProfiles: 100,
      expiryDays: 365,
      features: ['fingerprint_spoofing', 'proxy_support']
    });
    // Handle response
  };
  
  return (
    <div>
      <Button onClick={generateLicense}>Generate New License</Button>
      <LicenseTable licenses={licenses} />
    </div>
  );
};
```

---

## 🎨 UI Components - Client App

### Profile List
```tsx
// client-app/src/components/ProfileList.tsx
export const ProfileList = () => {
  const { profiles, launchProfile } = useProfiles();
  
  return (
    <div className="profile-list">
      {profiles.map(profile => (
        <ProfileCard 
          key={profile.id}
          profile={profile}
          onLaunch={() => launchProfile(profile.id)}
        />
      ))}
      <CreateProfileButton />
    </div>
  );
};
```

### Profile Editor
```tsx
// client-app/src/components/ProfileEditor.tsx
export const ProfileEditor = ({ profileId }: Props) => {
  const [config, setConfig] = useState<ProfileConfig>();
  
  return (
    <Form onSubmit={handleSave}>
      <Section title="Platform">
        <Select 
          value={config.platform}
          options={['windows', 'macos', 'linux']}
        />
      </Section>
      
      <Section title="Hardware">
        <NumberInput 
          label="CPU Cores"
          value={config.hardware_concurrency}
        />
        <NumberInput 
          label="Memory (GB)"
          value={config.device_memory}
        />
      </Section>
      
      <Section title="Proxy">
        <ProxySelector 
          value={config.proxy}
          onChange={setProxy}
        />
      </Section>
    </Form>
  );
};
```

---

## 🔧 Build & Deployment

### Build Scripts

#### Backend
```bash
# Build backend
cd backend
cargo build --release

# Run migrations
diesel migration run

# Start server
./target/release/backend-server
```

#### Admin App
```bash
cd admin-app
npm install
npm run tauri build

# Output:
# - Windows: src-tauri/target/release/bundle/nsis/admin-app_1.0.0_x64-setup.exe
# - macOS: src-tauri/target/release/bundle/dmg/admin-app_1.0.0_x64.dmg
```

#### Client App
```bash
cd client-app
npm install
npm run tauri build

# Output:
# - Windows: src-tauri/target/release/bundle/nsis/client-app_1.0.0_x64-setup.exe
# - macOS: src-tauri/target/release/bundle/dmg/client-app_1.0.0_x64.dmg
```

### Docker Deployment
```yaml
# docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: antidetect
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    
  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    
  minio:
    image: minio/minio:latest
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: ${MINIO_USER}
      MINIO_ROOT_PASSWORD: ${MINIO_PASSWORD}
    volumes:
      - minio_data:/data
    
  backend:
    build: ./backend
    ports:
      - "8080:8080"
    depends_on:
      - postgres
      - redis
      - minio
    environment:
      DATABASE_URL: ${DATABASE_URL}
      REDIS_URL: ${REDIS_URL}
      MINIO_URL: ${MINIO_URL}
```

---

## 🔒 Security Checklist

### Before Release
- [ ] Code signing certificates installed
- [ ] All secrets moved to environment variables
- [ ] License server deployed separately
- [ ] Anti-debug/Anti-VM enabled
- [ ] Binary stripped and optimized
- [ ] Encrypted storage implemented
- [ ] HTTPS/TLS enforced
- [ ] Rate limiting enabled
- [ ] Audit logging active
- [ ] Backup strategy tested

### Runtime Protection
- [ ] License validation on startup
- [ ] Hardware ID binding verified
- [ ] Periodic license checks (every hour)
- [ ] Remote kill switch functional
- [ ] Tamper detection active
- [ ] Memory encryption for sensitive data
- [ ] Secure IPC channels only
- [ ] No debug symbols in release

---

## 📊 MVP Timeline

### Phase 1: Infrastructure (Week 1-2)
- [ ] Setup monorepo structure
- [ ] Initialize Rust backend
- [ ] Setup PostgreSQL + Redis
- [ ] Basic authentication API
- [ ] License server skeleton

### Phase 2: Core Features (Week 3-4)
- [ ] Profile CRUD operations
- [ ] Fingerprint generation service
- [ ] Browser launch mechanism
- [ ] Proxy management
- [ ] Storage service

### Phase 3: Desktop Apps (Week 5-6)
- [ ] Admin app UI (React)
- [ ] Client app UI (React)
- [ ] Tauri commands implementation
- [ ] IPC security layer
- [ ] Local data encryption

### Phase 4: Integration (Week 7-8)
- [ ] Connect apps to backend
- [ ] License validation flow
- [ ] Profile sync
- [ ] Browser fingerprint testing
- [ ] Proxy rotation

### Phase 5: Security Hardening (Week 9-10)
- [ ] Anti-debug/Anti-VM
- [ ] Code obfuscation
- [ ] Binary signing
- [ ] Encrypted storage
- [ ] Kill switch

### Phase 6: Testing & Polish (Week 11-12)
- [ ] End-to-end testing
- [ ] Security penetration testing
- [ ] Performance optimization
- [ ] Bug fixes
- [ ] Documentation

---

## 🎓 Development Guidelines

### Rust Conventions
```rust
// Use descriptive error types
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("License validation failed")]
    InvalidLicense,
}

// Use async/await consistently
pub async fn create_profile(
    pool: &PgPool,
    profile: NewProfile
) -> Result<Profile, AppError> {
    // Implementation
}

// Document public APIs
/// Launch a browser instance with the given profile configuration
/// 
/// # Arguments
/// * `profile` - The profile configuration
/// 
/// # Returns
/// Process handle of the launched browser
pub async fn launch_browser(profile: &Profile) -> Result<Process> {
    // Implementation
}
```

### React/TypeScript Conventions
```typescript
// Use hooks for state management
export const useProfiles = () => {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [loading, setLoading] = useState(false);
  
  const fetchProfiles = async () => {
    setLoading(true);
    const data = await invoke<Profile[]>('get_profiles');
    setProfiles(data);
    setLoading(false);
  };
  
  return { profiles, loading, fetchProfiles };
};

// Use proper TypeScript types
interface Profile {
  id: string;
  name: string;
  config: ProfileConfig;
  createdAt: Date;
}
```

---

## 🚨 Critical Security Notes

### DO NOT
- ❌ Store sensitive data in localStorage
- ❌ Log sensitive information
- ❌ Hardcode secrets in source code
- ❌ Trust client-side validation alone
- ❌ Skip input sanitization
- ❌ Use insecure random number generators
- ❌ Ignore certificate validation

### DO
- ✅ Use environment variables for secrets
- ✅ Validate all inputs server-side
- ✅ Encrypt sensitive data at rest
- ✅ Use prepared statements (prevent SQL injection)
- ✅ Implement rate limiting
- ✅ Log security events
- ✅ Use HTTPS everywhere
- ✅ Implement CSRF protection
- ✅ Use secure session management

---

## 📚 Resources

### Documentation
- Rust: https://doc.rust-lang.org/
- Tauri: https://tauri.app/
- Axum: https://docs.rs/axum/
- PostgreSQL: https://www.postgresql.org/docs/
- fingerprint-chromium: https://github.com/adryfish/fingerprint-chromium

### Security
- OWASP Top 10: https://owasp.org/www-project-top-ten/
- Rust Security: https://anssi-fr.github.io/rust-guide/
- Tauri Security: https://tauri.app/v1/references/security/

---

## 🎯 Success Criteria

### MVP Launch Checklist
- [ ] Admin can generate licenses
- [ ] Admin can view analytics
- [ ] Client can activate license
- [ ] Client can create 10+ profiles
- [ ] Each profile has unique fingerprint
- [ ] Browser launches with correct fingerprint
- [ ] Proxy rotation works
- [ ] License validation is secure
- [ ] Apps work on Windows & macOS
- [ ] No critical security vulnerabilities

### Performance Targets
- [ ] Browser launch time < 3 seconds
- [ ] Profile creation < 1 second
- [ ] API response time < 100ms (p95)
- [ ] App memory usage < 200MB
- [ ] Binary size < 50MB

---

## 📝 Final Notes for Cursor Agent

این پروژه باید **از صفر تا صد** ساخته شود با بالاترین کیفیت و امنیت. تمامی کدها باید:

1. **Clean & Maintainable**: کد تمیز و قابل نگهداری
2. **Well-Documented**: مستندسازی کامل
3. **Type-Safe**: استفاده از Type Safety در Rust و TypeScript
4. **Tested**: Unit tests و Integration tests
5. **Secure**: پیاده‌سازی تمام لایه‌های امنیتی
6. **Production-Ready**: آماده برای استفاده در production

این MVP باید قابلیت **ارتقا** به نسخه Enterprise داشته باشد و همه کدها باید با این هدف نوشته شوند.

**سطح امنیت باید Maximum باشد** - این یک محصول Enterprise است که نباید قابل نفوذ باشد.
