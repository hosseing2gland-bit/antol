# 🚀 Deployment Guide - Anti-Detect Browser

## Quick Start (Docker)

### 1. Production Deployment

```bash
# Clone repository
git clone https://github.com/hosseing2gland-bit/antol.git
cd antol/anti-detect-mvp

# Copy environment file
cp .env.example .env

# Edit .env with your production values
nano .env

# Start all services
docker-compose -f docker-compose.production.yml up -d

# Check logs
docker-compose -f docker-compose.production.yml logs -f

# Stop services
docker-compose -f docker-compose.production.yml down
```

### 2. Development Deployment

```bash
cd anti-detect-mvp

# Start database only
docker-compose up -d postgres

# Run backend manually
cd backend
export DATABASE_URL="postgres://antidetect_user:antidetect123@localhost:5432/antidetect_db"
cargo run --release

# Run admin app (separate terminal)
cd admin-app
npm install
npm run dev

# Run client app (separate terminal)
cd client-app
npm install
npm run dev
```

## 📦 Services

After deployment, services will be available at:

- **Admin Panel**: http://localhost:8080
- **Client App**: http://localhost:8081
- **Backend API**: http://localhost:3000
- **Database**: localhost:5432

With Nginx reverse proxy:
- **All services**: http://localhost
- **API**: http://localhost/api
- **Admin**: http://localhost/admin
- **Client**: http://localhost/

## 🔐 Security Checklist

Before production deployment:

- [ ] Change all default passwords in `.env`
- [ ] Set strong JWT_SECRET (minimum 32 characters)
- [ ] Enable HTTPS (uncomment SSL section in nginx.conf)
- [ ] Set up SSL certificates
- [ ] Configure firewall rules
- [ ] Enable database backups
- [ ] Set up monitoring
- [ ] Review CORS settings
- [ ] Enable rate limiting
- [ ] Set up log rotation

## 🔧 Environment Variables

### Required

| Variable | Description | Example |
|----------|-------------|---------|
| `POSTGRES_USER` | Database username | `antidetect_user` |
| `POSTGRES_PASSWORD` | Database password | `strong-password-here` |
| `POSTGRES_DB` | Database name | `antidetect_db` |
| `JWT_SECRET` | JWT signing key | `min-32-chars-random-string` |

### Optional

| Variable | Description | Default |
|----------|-------------|---------|
| `SMTP_HOST` | Email server host | - |
| `SMTP_PORT` | Email server port | `587` |
| `ADMIN_API_URL` | Backend URL for admin | `http://localhost:3000` |
| `CLIENT_API_URL` | Backend URL for client | `http://localhost:3000` |

## 🗄️ Database Management

### Backup

```bash
# Backup database
docker exec antidetect_postgres_prod pg_dump -U antidetect_user antidetect_db > backup_$(date +%Y%m%d_%H%M%S).sql

# Restore database
docker exec -i antidetect_postgres_prod psql -U antidetect_user antidetect_db < backup.sql
```

### Migrations

```bash
# Run migrations
docker exec antidetect_backend_prod sqlx migrate run

# Check migration status
docker exec antidetect_backend_prod sqlx migrate info
```

## 📊 Monitoring

### Check Service Health

```bash
# Check all services
docker-compose -f docker-compose.production.yml ps

# Check specific service logs
docker-compose -f docker-compose.production.yml logs backend
docker-compose -f docker-compose.production.yml logs postgres

# Follow logs in real-time
docker-compose -f docker-compose.production.yml logs -f
```

### Resource Usage

```bash
# Check container stats
docker stats

# Check disk usage
docker system df
```

## 🔄 Updates

```bash
# Pull latest code
git pull origin main

# Rebuild and restart services
docker-compose -f docker-compose.production.yml down
docker-compose -f docker-compose.production.yml build --no-cache
docker-compose -f docker-compose.production.yml up -d

# Check logs for errors
docker-compose -f docker-compose.production.yml logs -f
```

## 🐛 Troubleshooting

### Backend won't start

```bash
# Check logs
docker-compose logs backend

# Common issues:
# 1. Database not ready - wait a few seconds and restart
# 2. Migration errors - check DATABASE_URL
# 3. Port already in use - change port in docker-compose.yml
```

### Database connection failed

```bash
# Check postgres is running
docker-compose ps postgres

# Check postgres logs
docker-compose logs postgres

# Test connection
docker exec -it antidetect_postgres_prod psql -U antidetect_user -d antidetect_db
```

### Frontend can't reach backend

```bash
# Check VITE_API_URL in .env
# Make sure backend is running
curl http://localhost:3000/api/users

# Check CORS settings in backend
```

## 📱 SSL/HTTPS Setup

### Generate Self-Signed Certificate (Development)

```bash
mkdir -p ssl
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout ssl/key.pem -out ssl/cert.pem \
  -subj "/CN=localhost"
```

### Let's Encrypt (Production)

```bash
# Install certbot
apt-get install certbot

# Get certificate
certbot certonly --standalone -d your-domain.com

# Copy certificates
cp /etc/letsencrypt/live/your-domain.com/fullchain.pem ssl/cert.pem
cp /etc/letsencrypt/live/your-domain.com/privkey.pem ssl/key.pem

# Uncomment HTTPS section in nginx.conf
# Restart nginx
docker-compose restart nginx
```

## 🔧 Performance Tuning

### PostgreSQL

Edit `docker-compose.production.yml`:

```yaml
postgres:
  command:
    - "postgres"
    - "-c"
    - "max_connections=200"
    - "-c"
    - "shared_buffers=256MB"
    - "-c"
    - "effective_cache_size=1GB"
```

### Backend

Adjust worker threads in backend code or use environment variable.

### Nginx

Increase worker connections in `nginx.conf`:

```nginx
events {
    worker_connections 2048;
}
```

## 📞 Support

For issues or questions:
- GitHub Issues: https://github.com/hosseing2gland-bit/antol/issues
- Documentation: README.md
- Admin Guide: USER_GUIDE_ADMIN.md
