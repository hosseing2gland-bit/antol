# راه‌اندازی Backend در Windows PowerShell
# Anti-Detect Browser - Backend Startup Script

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Anti-Detect Backend - Windows Setup  " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Check Docker
Write-Host "[1/4] Checking Docker..." -ForegroundColor Yellow
try {
    docker --version | Out-Null
    Write-Host "✓ Docker is installed" -ForegroundColor Green
} catch {
    Write-Host "✗ Docker not found! Please install Docker Desktop" -ForegroundColor Red
    Write-Host "  Download: https://www.docker.com/products/docker-desktop" -ForegroundColor Yellow
    exit 1
}

# Step 2: Start Docker Services
Write-Host ""
Write-Host "[2/4] Starting Docker services..." -ForegroundColor Yellow
Set-Location anti-detect-mvp

# Check if services are already running
$postgresRunning = docker ps --filter "name=antidetect_postgres" --format "{{.Names}}" | Select-String -Pattern "antidetect_postgres"

if ($postgresRunning) {
    Write-Host "✓ Docker services already running" -ForegroundColor Green
} else {
    Write-Host "  Starting PostgreSQL, Redis, MinIO..." -ForegroundColor Cyan
    docker compose up -d
    
    Write-Host "  Waiting for services to be ready..." -ForegroundColor Cyan
    Start-Sleep -Seconds 5
    
    Write-Host "✓ Docker services started" -ForegroundColor Green
}

# Step 3: Check Services Health
Write-Host ""
Write-Host "[3/4] Checking services health..." -ForegroundColor Yellow

$services = @("antidetect_postgres", "antidetect_redis", "antidetect_minio")
foreach ($service in $services) {
    $status = docker ps --filter "name=$service" --format "{{.Status}}" 
    if ($status -match "Up") {
        Write-Host "  ✓ $service is running" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $service is not running!" -ForegroundColor Red
        exit 1
    }
}

# Step 4: Start Backend
Write-Host ""
Write-Host "[4/4] Starting Backend..." -ForegroundColor Yellow
Set-Location backend

# Check if binary exists
if (Test-Path "target\release\backend.exe") {
    Write-Host "✓ Backend binary found" -ForegroundColor Green
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "  Backend is starting on port 3000...  " -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "API URL: http://localhost:3000" -ForegroundColor Cyan
    Write-Host "Health Check: http://localhost:3000/health" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Press Ctrl+C to stop" -ForegroundColor Yellow
    Write-Host ""
    
    # Run backend
    & ".\target\release\backend.exe"
} else {
    Write-Host "✗ Backend binary not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Building backend for the first time..." -ForegroundColor Yellow
    Write-Host "This may take 5-10 minutes..." -ForegroundColor Yellow
    Write-Host ""
    
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✓ Build successful! Starting backend..." -ForegroundColor Green
        Write-Host ""
        & ".\target\release\backend.exe"
    } else {
        Write-Host ""
        Write-Host "✗ Build failed! Check errors above." -ForegroundColor Red
        exit 1
    }
}
