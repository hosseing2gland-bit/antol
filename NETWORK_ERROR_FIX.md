# Network Error Fix - Documentation

## Problem
The Tauri applications (admin-app and client-app) were showing network errors:
```
Network Error: Cannot connect to server. Please check:
1. Your internet connection
2. VPN if enabled
3. Server is running at http://108.143.173.222:3000
```

## Root Causes

### 1. Port Mismatch
- Backend was hardcoded to run on port **3000** in `backend/src/main.rs`
- Environment files (`.env`) specified port **8000**
- This caused confusion about which port the backend actually runs on

### 2. Hardcoded API URLs
- Both `admin-app/src/api.ts` and `client-app/src/api.ts` had hardcoded URLs
- They were not reading from environment variables (`VITE_API_URL`)
- Changes to `.env` files had no effect

### 3. Tauri HTTP Scope Limitations
- `tauri.conf.json` only allowed `http://108.143.173.222:3000/**`
- If backend port changed, Tauri would block the requests
- No localhost support for development

## Solutions Implemented

### 1. Made Backend Port Configurable
**File**: `anti-detect-mvp/backend/src/main.rs`

Changed from:
```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
```

To:
```rust
// Get port from environment variable or use default 3000
let port = std::env::var("API_PORT")
    .ok()
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or(3000);

let addr = SocketAddr::from(([0, 0, 0, 0], port));
```

Now you can set `API_PORT` in `.env` to change the backend port.

### 2. Fixed API URL Configuration in Apps
**Files**: 
- `anti-detect-mvp/admin-app/src/api.ts`
- `anti-detect-mvp/client-app/src/api.ts`

Changed from:
```typescript
export const API_URL = 'http://108.143.173.222:3000/api';
```

To:
```typescript
// API Configuration - Use environment variable with fallback
export const API_URL = import.meta.env.VITE_API_URL 
  ? `${import.meta.env.VITE_API_URL}/api`
  : 'http://108.143.173.222:3000/api';
```

Now the apps read from `VITE_API_URL` environment variable.

### 3. Expanded Tauri HTTP Scope
**Files**:
- `anti-detect-mvp/admin-app/src-tauri/tauri.conf.json`
- `anti-detect-mvp/client-app/src-tauri/tauri.conf.json`

Changed from:
```json
"scope": ["http://108.143.173.222:3000/**"]
```

To:
```json
"scope": [
  "http://108.143.173.222:3000/**",
  "http://108.143.173.222:8000/**",
  "http://localhost:3000/**",
  "http://localhost:8000/**"
]
```

Now supports multiple ports and localhost for development.

### 4. Unified Environment Configuration
Updated all `.env` files to use port **3000** consistently:
- Root `.env`: `API_URL=http://108.143.173.222:3000`
- `anti-detect-mvp/.env`: `API_PORT=3000`
- `anti-detect-mvp/admin-app/.env`: `VITE_API_URL=http://108.143.173.222:3000`
- `anti-detect-mvp/client-app/.env`: `VITE_API_URL=http://108.143.173.222:3000`

## How to Use

### For Development (localhost)
1. Update `.env` files to use localhost:
   ```
   VITE_API_URL=http://localhost:3000
   ```

2. Start backend:
   ```bash
   cd anti-detect-mvp/backend
   cargo run
   ```

3. Start admin-app:
   ```bash
   cd anti-detect-mvp/admin-app
   npm run tauri dev
   ```

### For Production (Remote Server)
1. Make sure `.env` files have the correct server IP:
   ```
   VITE_API_URL=http://108.143.173.222:3000
   ```

2. Backend should be running on the server at port 3000

3. Build and run the apps:
   ```bash
   cd anti-detect-mvp/admin-app
   npm run tauri build
   ```

### To Change Backend Port
1. Edit `anti-detect-mvp/.env`:
   ```
   API_PORT=8000
   ```

2. Update app `.env` files to match:
   ```
   VITE_API_URL=http://108.143.173.222:8000
   ```

3. Restart backend and rebuild apps

## Testing the Fix

1. **Check backend is running**:
   ```bash
   curl http://108.143.173.222:3000/
   ```
   Should return: "Anti-Detect Browser Backend API"

2. **Check API endpoint**:
   ```bash
   curl http://108.143.173.222:3000/api/auth/login
   ```

3. **Run the Tauri app** and check console logs for:
   ```
   🔍 Environment Check:
   - isTauri: true
   - window.__TAURI__: true
   - API_URL: http://108.143.173.222:3000/api
   ```

## Notes

- The backend defaults to port 3000 if `API_PORT` is not set
- Apps fallback to hardcoded URL if `VITE_API_URL` is not set
- Tauri HTTP scope now supports both development and production URLs
- All environment variable changes require rebuilding the Tauri apps
