# Troubleshooting Guide

Common issues and their solutions for the Anti-Detect Browser MVP.

## Table of Contents
1. [Backend Issues](#backend-issues)
2. [Client/Admin App Issues](#clientadmin-app-issues)
3. [Docker Issues](#docker-issues)
4. [Build Issues](#build-issues)
5. [Runtime Issues](#runtime-issues)

---

## Backend Issues

### Database Connection Failed

**Error:**
```
Failed to connect to Postgres
```

**Solutions:**
1. Ensure PostgreSQL is running:
   ```bash
   docker compose ps   # Check if postgres container is up
   docker compose up -d postgres  # Start if not running
   ```

2. Verify connection string in `.env`:
   ```
   DATABASE_URL=postgresql://antidetect_user:antidetect123@localhost:5432/antidetect_db
   ```

3. Test connection manually:
   ```bash
   psql -h localhost -U antidetect_user -d antidetect_db
   ```

### Redis Connection Failed

**Error:**
```
Connection refused (os error 111) - Redis
```

**Solutions:**
1. Start Redis:
   ```bash
   docker compose up -d redis
   ```

2. Verify Redis is accessible:
   ```bash
   redis-cli ping
   ```

### Port Already in Use

**Error:**
```
Failed to bind to address: Address already in use
```

**Solutions:**
1. Find and kill the process:
   ```bash
   lsof -i :3000
   kill -9 <PID>
   ```

2. Or change the port in `.env`:
   ```
   PORT=3001
   ```

---

## Client/Admin App Issues

### npm install Fails

**Error:**
```
npm ERR! code ERESOLVE
```

**Solutions:**
1. Clear npm cache:
   ```bash
   npm cache clean --force
   rm -rf node_modules package-lock.json
   npm install
   ```

2. Use legacy peer deps:
   ```bash
   npm install --legacy-peer-deps
   ```

### Tauri Build Fails

**Error:**
```
Error: failed to bundle project
```

**Solutions:**

**On Linux:**
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
```

**On macOS:**
```bash
xcode-select --install
```

**On Windows:**
- Install Visual Studio Build Tools with C++ workload
- Install WebView2 Runtime

### TypeScript Errors

**Error:**
```
Cannot find module 'xxx' or its corresponding type declarations
```

**Solutions:**
1. Reinstall dependencies:
   ```bash
   rm -rf node_modules
   npm install
   ```

2. Check tsconfig.json for correct paths

### API Connection Failed

**Error:**
```
Network Error / CORS error
```

**Solutions:**
1. Verify backend is running:
   ```bash
   curl http://localhost:3000/
   ```

2. Check VITE_API_URL in `.env`:
   ```
   VITE_API_URL=http://localhost:3000
   ```

3. Ensure backend has CORS enabled (it does by default)

---

## Docker Issues

### Container Won't Start

**Solutions:**
1. Check logs:
   ```bash
   docker compose logs postgres
   docker compose logs redis
   ```

2. Reset volumes:
   ```bash
   docker compose down -v
   docker compose up -d
   ```

### Port Conflict

**Error:**
```
Bind for 0.0.0.0:5432 failed: port is already allocated
```

**Solutions:**
1. Stop conflicting service:
   ```bash
   sudo systemctl stop postgresql
   ```

2. Or change port in docker-compose.yml:
   ```yaml
   ports:
     - "5433:5432"
   ```

### Out of Disk Space

**Solutions:**
1. Clean up Docker:
   ```bash
   docker system prune -a
   docker volume prune
   ```

---

## Build Issues

### Rust Build Fails

**Error:**
```
error[E0433]: failed to resolve
```

**Solutions:**
1. Update Rust:
   ```bash
   rustup update
   ```

2. Clean and rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

### macOS Code Signing Issues

**Error:**
```
codesign failed with exit code 1
```

**Solutions:**
1. For development (unsigned):
   ```bash
   xattr -cr /path/to/app.app
   ```

2. For distribution, set up proper signing certificates

### Windows Build Fails

**Error:**
```
LINK : fatal error LNK1181: cannot open input file
```

**Solutions:**
1. Install Visual Studio Build Tools
2. Run from "Developer Command Prompt"

---

## Runtime Issues

### Browser Won't Launch

**Solutions:**
1. Ensure Chrome/Chromium is installed
2. Check Chrome path:
   - Windows: `C:\Program Files\Google\Chrome\Application\chrome.exe`
   - macOS: `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`
   - Linux: `/usr/bin/google-chrome` or `/usr/bin/chromium-browser`

### Anti-Detection Not Working

**Solutions:**
1. Verify fingerprint is generated (check console)
2. Test on fingerprint testing sites:
   - https://browserleaks.com
   - https://pixelscan.net
   - https://whoer.net

### App Crashes on Start

**Solutions:**
1. Check logs in terminal
2. Clear app data:
   - Linux: `~/.local/share/client-app/`
   - macOS: `~/Library/Application Support/client-app/`
   - Windows: `%APPDATA%\client-app\`

---

## Getting Help

If none of these solutions work:

1. Check existing issues: https://github.com/hosseing2gland-bit/antol/issues
2. Create a new issue with:
   - Operating system and version
   - Error message (full text)
   - Steps to reproduce
   - Relevant log output

---

## Quick Diagnostic Commands

```bash
# Check all services
make status

# Test backend
curl http://localhost:3000/

# Test database
docker compose exec postgres pg_isready

# Test Redis
docker compose exec redis redis-cli ping

# View all logs
docker compose logs -f

# Run structure tests
python3 test_anti_detection.py
python3 test_admin_app.py
```
