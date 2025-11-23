# Anti-Detect Browser MVP - Makefile
# Common commands for development and deployment

.PHONY: help dev-setup dev-backend dev-client dev-admin build test clean docker-up docker-down release

# Default target
help:
	@echo "Anti-Detect Browser MVP - Available Commands"
	@echo "============================================="
	@echo ""
	@echo "Development:"
	@echo "  make dev-setup     - Install all dependencies"
	@echo "  make dev-backend   - Run backend in development mode"
	@echo "  make dev-client    - Run client app in development mode"
	@echo "  make dev-admin     - Run admin app in development mode"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-up     - Start PostgreSQL, Redis, MinIO"
	@echo "  make docker-down   - Stop all Docker services"
	@echo "  make docker-logs   - View Docker logs"
	@echo ""
	@echo "Build:"
	@echo "  make build         - Build all components for production"
	@echo "  make build-backend - Build backend only"
	@echo "  make build-client  - Build client app only"
	@echo "  make build-admin   - Build admin app only"
	@echo ""
	@echo "Testing:"
	@echo "  make test          - Run all tests"
	@echo "  make test-backend  - Run backend tests"
	@echo "  make test-api      - Test backend API endpoints"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make release       - Create a new release tag"

# ==================
# Development Setup
# ==================

dev-setup:
	@echo "Installing dependencies..."
	cd anti-detect-mvp/backend && cargo build
	cd anti-detect-mvp/client-app && npm install
	cd anti-detect-mvp/admin-app && npm install
	@echo "✅ All dependencies installed"

dev-backend:
	@echo "Starting backend..."
	cd anti-detect-mvp/backend && cargo run --release

dev-client:
	@echo "Starting client app..."
	cd anti-detect-mvp/client-app && npm run tauri dev

dev-admin:
	@echo "Starting admin app..."
	cd anti-detect-mvp/admin-app && npm run tauri dev

# ==================
# Docker
# ==================

docker-up:
	@echo "Starting Docker services..."
	docker compose up -d
	@echo "✅ Services started"
	@echo "PostgreSQL: localhost:5432"
	@echo "Redis: localhost:6379"
	@echo "MinIO: localhost:9000 (Console: localhost:9001)"

docker-down:
	@echo "Stopping Docker services..."
	docker compose down
	@echo "✅ Services stopped"

docker-logs:
	docker compose logs -f

docker-reset:
	@echo "Resetting Docker volumes..."
	docker compose down -v
	docker compose up -d
	@echo "✅ Services reset with fresh data"

# ==================
# Build
# ==================

build: build-backend build-client build-admin
	@echo "✅ All builds complete"

build-backend:
	@echo "Building backend..."
	cd anti-detect-mvp/backend && cargo build --release
	@echo "✅ Backend built: anti-detect-mvp/backend/target/release/backend"

build-client:
	@echo "Building client app..."
	cd anti-detect-mvp/client-app && npm run build && npm run tauri build
	@echo "✅ Client app built"

build-admin:
	@echo "Building admin app..."
	cd anti-detect-mvp/admin-app && npm run build && npm run tauri build
	@echo "✅ Admin app built"

# ==================
# Testing
# ==================

test: test-structure test-backend
	@echo "✅ All tests complete"

test-structure:
	@echo "Running structure tests..."
	python3 test_anti_detection.py
	python3 test_admin_app.py

test-backend:
	@echo "Running backend tests..."
	cd anti-detect-mvp/backend && cargo test

test-api:
	@echo "Testing API endpoints..."
	bash test-api.sh

# ==================
# Utilities
# ==================

clean:
	@echo "Cleaning build artifacts..."
	cd anti-detect-mvp/backend && cargo clean
	rm -rf anti-detect-mvp/client-app/dist
	rm -rf anti-detect-mvp/admin-app/dist
	rm -rf anti-detect-mvp/client-app/src-tauri/target
	rm -rf anti-detect-mvp/admin-app/src-tauri/target
	@echo "✅ Clean complete"

# Create a new release
# Usage: make release VERSION=1.0.0
release:
ifndef VERSION
	$(error VERSION is not set. Usage: make release VERSION=1.0.0)
endif
	@echo "Creating release v$(VERSION)..."
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	git push origin v$(VERSION)
	@echo "✅ Release v$(VERSION) created and pushed"

# Check status
status:
	@echo "=== Git Status ==="
	git status
	@echo ""
	@echo "=== Docker Status ==="
	docker compose ps 2>/dev/null || echo "Docker not running"
