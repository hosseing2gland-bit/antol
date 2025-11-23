# Anti-Detect Browser MVP

A complete Anti-Detect Browser management system with professional capabilities.

## Project Structure

```
antol/
├── anti-detect-mvp/
│   ├── backend/        # Rust + Axum backend (PostgreSQL, Redis, MinIO)
│   ├── client-app/     # Tauri + React client app (anti-detection browser)
│   └── admin-app/      # Tauri + React admin dashboard
├── .github/workflows/  # CI/CD pipelines
└── *.md                # Documentation files
```

## Environment Setup

Copy the `.env.example` files to `.env` in the following locations:
- Root directory: `.env.example` → `.env`
- Backend: `anti-detect-mvp/backend/.env.example` → `.env`
- Client App: `anti-detect-mvp/client-app/.env.example` → `.env`
- Admin App: `anti-detect-mvp/admin-app/.env.example` → `.env`

Configure the environment variables with your actual values (database credentials, API URLs, etc.).

---

## Anti-Detection Features

| Feature | Status |
|---------|--------|
| Canvas Fingerprint Noise | ✅ |
| WebGL Fingerprint Spoofing | ✅ |
| Audio Context Noise | ✅ |
| User Agent Randomization | ✅ |
| Hardware Spoofing (CPU, RAM) | ✅ |
| Screen Resolution Spoofing | ✅ |
| Timezone & Language | ✅ |
| WebRTC Leak Protection | ✅ |
| Fake Media Devices | ✅ |
| Client Rects Noise | ✅ |
| Battery API Spoofing | ✅ |

---

## Quick Start with Docker

The easiest way to get started is with Docker:

```bash
# Start PostgreSQL, Redis, and MinIO
docker compose up -d

# Verify services are running
docker compose ps
```

Then run the backend and apps as described below.

---

## Quick Start (Manual)

### Prerequisites

- **Rust** (1.70+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js** (18+): https://nodejs.org/
- **Docker** (for databases): https://docker.com/
- Or install PostgreSQL, Redis, MinIO manually

### Running the Backend

```bash
cd anti-detect-mvp/backend

# Copy and configure environment
cp .env.example .env
# Edit .env with your database credentials

# Build and run
cargo build --release
cargo run --release
```

The backend runs on `http://localhost:3000`

### Running the Client App (Development)

```bash
cd anti-detect-mvp/client-app

# Copy and configure environment
cp .env.example .env

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Or build for production
npm run tauri build
```

### Running the Admin App (Development)

```bash
cd anti-detect-mvp/admin-app

# Copy and configure environment
cp .env.example .env

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Or build for production
npm run tauri build
```

---

## Running Tests

### Anti-Detection System Test
```bash
python3 test_anti_detection.py
```

### Admin App Structure Test
```bash
python3 test_admin_app.py
```

### Backend API Test (requires running backend)
```bash
bash test-api.sh
# Or with custom API URL:
API_URL=http://your-server:3000 bash test-api.sh
```

---

## GitHub Actions CI/CD

### CI Pipeline (ci.yml)
Automatically runs on push/PR to `main`:
- Backend build and tests (Rust)
- Client App build and TypeScript validation
- Admin App build and TypeScript validation
- Uploads build artifacts

### macOS Builds (build-macos.yml)
Manual trigger or on version tags:
- Builds .app and .dmg for Intel (x86_64)
- Builds .app and .dmg for Apple Silicon (ARM64)

**Trigger manually:**
1. Go to Actions tab in GitHub
2. Select "macOS Builds"
3. Click "Run workflow"

### All Platforms Build (build-all-platforms.yml)
Builds for Windows, macOS, and Linux simultaneously.

---

## Make Commands

Use the Makefile for common operations:

```bash
make help           # Show all available commands
make dev-setup      # Install all dependencies
make docker-up      # Start Docker services
make docker-down    # Stop Docker services
make build          # Build all components
make test           # Run all tests
make clean          # Clean build artifacts
make release VERSION=1.0.0  # Create a release
```

---

## Documentation

### User Guides
- [Quick Start Guide](QUICK_START.md)
- [Admin User Guide](USER_GUIDE_ADMIN.md)
- [Client User Guide](USER_GUIDE_CLIENT.md)
- [Features Guide](FEATURES.md)

### Technical Documentation
- [API Documentation](API_DOCUMENTATION.md)
- [Deployment Guide](DEPLOYMENT_GUIDE.md)
- [Deployment Runbook](DEPLOYMENT_RUNBOOK.md)
- [Deployment Checklist](DEPLOYMENT_CHECKLIST.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)

### Build Guides
- [Build for macOS](BUILD_MACOS.md)
- [Build for Windows](BUILD_WINDOWS.md)
- [GitHub Actions Guide](.github/GITHUB_ACTIONS.md)

---

## Default Credentials (Development)

- **Admin Login**: `admin@demo.com` / `admin123`

---

## Status: 100% MVP Complete

| Component | Status |
|-----------|--------|
| Backend API | ✅ Complete |
| Admin App | ✅ Complete |
| Client App | ✅ Complete |
| Anti-Detection Core | ✅ Complete |
| Browser Launcher | ✅ Complete |
| CI/CD Pipelines | ✅ Complete |
| Docker Setup | ✅ Complete |
| Documentation | ✅ Complete |
| Release Workflow | ✅ Complete |

### Future Enhancements
- Custom Chromium build
- Mobile support (Android/iOS)
- Cloud profile sync

---

## License

MIT License - See [LICENSE](LICENSE) for details.
