# 🔥 Anti-Detection Features - راهنمای فنی

## معماری Anti-Detection

این پروژه از سه لایه اصلی برای جلوگیری از fingerprinting استفاده می‌کند:

### 1. **Fingerprint Generation** (Rust)
تولید fingerprint تصادفی اما واقع‌گرایانه

### 2. **JavaScript Injection** (در Browser)
تزریق کدهای JavaScript قبل از لود شدن صفحه

### 3. **Chrome Flags** (CLI Arguments)
استفاده از فلگ‌های Chromium برای تغییر رفتار

---

## 🎯 لیست کامل قابلیت‌ها

### ✅ Canvas Fingerprinting Protection

**چیست؟**
سایت‌ها از Canvas API برای تولید یک تصویر منحصر به فرد استفاده می‌کنند و از آن به عنوان fingerprint استفاده می‌کنند.

**راه‌حل ما:**
```javascript
// اضافه کردن نویز به Canvas
ctx.fillStyle = 'rgba(...)';
// نویز تصادفی اما ثابت برای هر session
```

**تست:**
```bash
# باز کردن browserleaks.com/canvas
# هر بار fingerprint متفاوت
```

---

### ✅ WebGL Fingerprinting Protection

**چیست؟**
GPU و driver شناسایی می‌شود از طریق WebGL.

**راه‌حل ما:**
```javascript
// Override کردن WebGL vendor/renderer
WebGLRenderingContext.prototype.getParameter = function(param) {
    if (param === UNMASKED_VENDOR_WEBGL) return "Intel Inc.";
    if (param === UNMASKED_RENDERER_WEBGL) return "Intel Iris OpenGL Engine";
}
```

**Vendors تصادفی:**
- NVIDIA GeForce GTX 1060
- AMD Radeon RX 580
- Intel UHD Graphics 630
- Apple M1/M2 GPU

---

### ✅ Audio Context Fingerprinting Protection

**چیست؟**
هر سیستم صدای کمی متفاوت تولید می‌کند.

**راه‌حل ما:**
```javascript
// اضافه کردن نویز به Audio signals
analyser.getFloatFrequencyData = function(array) {
    // نویز بسیار کم (0.0001)
    for (let i = 0; i < array.length; i++) {
        array[i] += (Math.random() - 0.5) * 0.0001;
    }
}
```

---

### ✅ Hardware Spoofing

**پارامترها:**
- `navigator.hardwareConcurrency` - CPU cores (2, 4, 6, 8, 12, 16)
- `navigator.deviceMemory` - RAM (2, 4, 8, 16, 32 GB)
- `navigator.maxTouchPoints` - Touch support (0 or 10)

**تولید تصادفی:**
```rust
let hardware_concurrency = [2, 4, 6, 8, 12, 16][rng.gen_range(0..6)];
let device_memory = [2, 4, 8, 16, 32][rng.gen_range(0..5)];
```

---

### ✅ Screen Resolution Spoofing

**Resolution های رایج:**
- 1920×1080 (Full HD)
- 1366×768 (Laptop)
- 2560×1440 (QHD)
- 3840×2160 (4K)

**پیاده‌سازی:**
```javascript
Object.defineProperty(screen, 'width', { get: () => 1920 });
Object.defineProperty(screen, 'height', { get: () => 1080 });
```

---

### ✅ User Agent Randomization

**ساختار:**
```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) 
AppleWebKit/537.36 (KHTML, like Gecko) 
Chrome/131.0.0.0 Safari/537.36
```

**تولید:**
```rust
fn generate_user_agent(browser_version: &str, os: &str) -> String {
    match os {
        "Windows" => format!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) ..."),
        "macOS" => format!("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ..."),
        // ...
    }
}
```

---

### ✅ Timezone & Language Spoofing

**Timezone های پشتیبانی شده:**
- America/New_York (-300)
- America/Los_Angeles (-480)
- Europe/London (0)
- Europe/Paris (+60)
- Asia/Tokyo (+540)
- Asia/Dubai (+240)

**Override:**
```javascript
Date.prototype.getTimezoneOffset = function() {
    return CONFIG.timezone_offset;
}
```

---

### ✅ WebRTC Leak Protection

**مشکل:**
WebRTC می‌تواند IP واقعی شما را لو بدهد حتی با استفاده از VPN!

**راه‌حل:**
```javascript
// غیرفعال کردن کامل WebRTC
window.RTCPeerConnection = function() {
    throw new Error('WebRTC disabled for privacy');
}
```

**تست:**
```bash
# browserleaks.com/webrtc
# نباید IP واقعی نشان داده شود
```

---

### ✅ Media Devices Spoofing

**دستگاه‌های جعلی:**
```javascript
navigator.mediaDevices.enumerateDevices = async () => [
    {
        deviceId: "default",
        kind: "audioinput",
        label: "Microphone (Built-in)",
        groupId: "xxx"
    },
    {
        deviceId: "default",
        kind: "videoinput",
        label: "HD Webcam (Built-in)",
        groupId: "yyy"
    }
]
```

---

### ✅ Client Rects Noise

**چیست؟**
اندازه‌گیری دقیق المنت‌های HTML برای fingerprinting.

**راه‌حل:**
```javascript
Element.prototype.getBoundingClientRect = function() {
    const rect = originalGetBoundingClientRect.call(this);
    return {
        x: rect.x + noise,
        y: rect.y + noise,
        width: rect.width + noise,
        // ...
    }
}
```

---

### ✅ Battery API Spoofing

**پارامترها:**
- `battery.level` - 0.0 to 1.0
- `battery.charging` - true/false
- `battery.chargingTime` - seconds
- `battery.dischargingTime` - seconds

---

### ✅ Font Fingerprinting Protection

**مشکل:**
لیست فونت‌های نصب شده برای fingerprinting استفاده می‌شود.

**راه‌حل:**
محدود کردن به فونت‌های رایج:
- Arial, Verdana, Helvetica
- Times New Roman, Georgia
- Courier New, Consolas
- (15 فونت استاندارد)

---

## 🧪 نحوه تست

### 1. راه‌اندازی Browser
```bash
cd anti-detect-mvp/client-app
npm run tauri dev
```

### 2. ایجاد پروفایل
- کلیک "New Profile"
- نام: "Test Profile"
- Create

### 3. Launch Browser
- کلیک ▶️ روی پروفایل

### 4. تست سایت‌ها
```
1. https://whoer.net
   - بررسی IP، DNS Leak، WebRTC
   
2. https://browserleaks.com
   - Canvas Fingerprint
   - WebGL Fingerprint
   - Audio Fingerprint
   
3. https://pixelscan.net
   - Fingerprint Score
   - Consistency Check
```

---

## 📊 مقایسه با رقبا

| Feature | ما | GoLogin | Multilogin | AdsPower |
|---------|-------|---------|-----------|----------|
| Canvas Noise | ✅ | ✅ | ✅ | ✅ |
| WebGL Spoofing | ✅ | ✅ | ✅ | ✅ |
| Audio Noise | ✅ | ✅ | ✅ | ❌ |
| WebRTC Protection | ✅ | ✅ | ✅ | ✅ |
| Hardware Spoofing | ✅ | ✅ | ✅ | ✅ |
| Client Rects Noise | ✅ | ❌ | ✅ | ❌ |
| Battery Spoofing | ✅ | ❌ | ❌ | ❌ |
| **Open Source** | ✅ | ❌ | ❌ | ❌ |
| **قیمت** | **رایگان** | $49/mo | $99/mo | $29/mo |

---

## 🔬 کد نمونه

### تولید Fingerprint:
```rust
let fingerprint = FingerprintConfig::generate_random();
// 40+ پارامتر تصادفی اما واقعی
```

### راه‌اندازی Browser:
```rust
let profile = BrowserProfile::new(
    "profile-id",
    "Profile Name",
    fingerprint,
    Some(proxy_config)
);

profile.launch()?;
```

### JavaScript Injection:
```javascript
const script = generate_injection_script(&fingerprint);
// 300+ خط کد anti-detection
```

---

## 🚀 مراحل بعدی

- [ ] Custom Chromium Build
- [ ] Mobile Fingerprints (Android/iOS)
- [ ] Browser Extensions Spoofing
- [ ] Plugin Detection Protection
- [ ] Automated Testing Suite

---

**توسعه‌دهنده:** این سیستم با Rust برای امنیت و سرعت بالا ساخته شده است.
