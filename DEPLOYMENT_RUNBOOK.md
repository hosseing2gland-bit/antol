# Deployment Runbook

Step-by-step guide for deploying the Anti-Detect Browser MVP to production.

## Prerequisites

- Linux server (Ubuntu 22.04 recommended)
- Docker and Docker Compose installed
- Domain name (optional, for HTTPS)
- At least 2GB RAM, 20GB disk space

---

## 1. Server Setup

### 1.1 Update System

```bash
sudo apt update && sudo apt upgrade -y
```

### 1.2 Install Docker

```bash
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER
```

### 1.3 Install Docker Compose

```bash
sudo apt install docker-compose-plugin
```

### 1.4 Clone Repository

```bash
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol
```

---

## 2. Configuration

### 2.1 Create Production Environment File

```bash
cp .env.example .env
nano .env
```

**Required changes:**
```env
# Use strong passwords!
POSTGRES_PASSWORD=<generate-strong-password>
REDIS_PASSWORD=<generate-strong-password>
JWT_SECRET=<generate-32-char-secret>
MINIO_SECRET_KEY=<generate-strong-password>

# Set your server IP/domain
API_URL=https://your-domain.com
```

### 2.2 Generate Secrets

```bash
# Generate random passwords
openssl rand -base64 32  # For passwords
openssl rand -hex 32     # For JWT secret
```

### 2.3 Configure Backend

```bash
cp anti-detect-mvp/backend/.env.example anti-detect-mvp/backend/.env
nano anti-detect-mvp/backend/.env
```

Update with production values:
```env
DATABASE_URL=postgresql://antidetect_user:<password>@postgres:5432/antidetect_db
JWT_SECRET=<your-jwt-secret>
PORT=3000
RUST_LOG=info
```

---

## 3. Database Setup

### 3.1 Start Database Services

```bash
docker compose up -d postgres redis minio
```

### 3.2 Wait for Services

```bash
docker compose logs -f postgres  # Wait for "ready to accept connections"
```

### 3.3 Run Migrations (if needed)

```bash
docker compose exec postgres psql -U antidetect_user -d antidetect_db -f /docker-entrypoint-initdb.d/01-seed.sql
```

---

## 4. Build and Deploy

### 4.1 Build Backend

```bash
cd anti-detect-mvp/backend
cargo build --release
```

### 4.2 Deploy with Docker Compose

```bash
cd /path/to/antol/anti-detect-mvp
docker compose -f docker-compose.production.yml up -d
```

### 4.3 Verify Deployment

```bash
# Check all containers are running
docker compose ps

# Test backend
curl http://localhost:3000/

# Check logs
docker compose logs -f backend
```

---

## 5. HTTPS Setup (Optional)

### 5.1 Install Certbot

```bash
sudo apt install certbot python3-certbot-nginx
```

### 5.2 Get SSL Certificate

```bash
sudo certbot --nginx -d your-domain.com
```

### 5.3 Update nginx.conf

Update the nginx configuration to use SSL certificates.

---

## 6. Systemd Service (Alternative to Docker)

### 6.1 Create Service File

```bash
sudo nano /etc/systemd/system/antidetect-backend.service
```

```ini
[Unit]
Description=Anti-Detect Browser Backend
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/antol/anti-detect-mvp/backend
ExecStart=/opt/antol/anti-detect-mvp/backend/target/release/backend
Restart=always
RestartSec=5
Environment=DATABASE_URL=postgresql://user:pass@localhost/antidetect_db
Environment=JWT_SECRET=your-secret
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### 6.2 Enable and Start

```bash
sudo systemctl daemon-reload
sudo systemctl enable antidetect-backend
sudo systemctl start antidetect-backend
```

---

## 7. Monitoring

### 7.1 Check Status

```bash
# Docker
docker compose ps
docker stats

# Systemd
sudo systemctl status antidetect-backend
```

### 7.2 View Logs

```bash
# Docker
docker compose logs -f backend

# Systemd
sudo journalctl -u antidetect-backend -f
```

### 7.3 Health Checks

```bash
# Backend health
curl http://localhost:3000/

# Database health
docker compose exec postgres pg_isready

# Redis health
docker compose exec redis redis-cli ping
```

---

## 8. Backup

### 8.1 Database Backup

```bash
docker compose exec postgres pg_dump -U antidetect_user antidetect_db > backup_$(date +%Y%m%d).sql
```

### 8.2 Restore Database

```bash
docker compose exec -T postgres psql -U antidetect_user antidetect_db < backup_20231120.sql
```

---

## 9. Updates

### 9.1 Pull Latest Code

```bash
cd /path/to/antol
git pull origin main
```

### 9.2 Rebuild and Restart

```bash
# Rebuild backend
cd anti-detect-mvp/backend
cargo build --release

# Restart services
docker compose restart backend
```

---

## 10. Rollback

### 10.1 Revert to Previous Version

```bash
git checkout <previous-commit-hash>
cargo build --release
docker compose restart backend
```

---

## Checklist

Before going live:

- [ ] Strong passwords configured
- [ ] JWT secret is unique and secure
- [ ] HTTPS enabled (for production)
- [ ] Firewall configured (only expose needed ports)
- [ ] Backup strategy in place
- [ ] Monitoring set up
- [ ] Logs are being collected
- [ ] Rate limiting enabled
- [ ] CORS configured for your domains

---

## Quick Commands Reference

```bash
# Start all services
docker compose up -d

# Stop all services
docker compose down

# View logs
docker compose logs -f

# Restart specific service
docker compose restart backend

# Check resource usage
docker stats

# Database shell
docker compose exec postgres psql -U antidetect_user -d antidetect_db

# Redis shell
docker compose exec redis redis-cli
```
