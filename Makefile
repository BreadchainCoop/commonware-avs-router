# Makefile for CommonWare AVS Router Development Environment

.PHONY: help setup up down logs clean validate check-env restart build-local

# Default target
help:
	@echo "CommonWare AVS Router - Docker Compose Environment"
	@echo ""
	@echo "Available targets:"
	@echo "  setup      - Set up environment (copy .env.example to .env)"
	@echo "  up         - Start all services in detached mode"
	@echo "  down       - Stop all services"
	@echo "  logs       - View logs from all services"
	@echo "  clean      - Stop services and remove volumes"
	@echo "  validate   - Validate docker-compose configuration"
	@echo "  check-env  - Check if .env file exists and contains required variables"
	@echo "  restart    - Restart all services"
	@echo "  build-local- Build images locally (if Dockerfiles exist)"
	@echo ""
	@echo "Examples:"
	@echo "  make setup up     - Set up and start environment"
	@echo "  make logs         - View logs"
	@echo "  make clean        - Full cleanup"

# Check if .env file exists
check-env:
	@if [ ! -f .env ]; then \
		echo "❌ .env file not found. Run 'make setup' first."; \
		exit 1; \
	fi
	@if ! grep -q "PRIVATE_KEY=" .env || grep -q "PRIVATE_KEY=$$" .env; then \
		echo "⚠️  PRIVATE_KEY not set in .env file"; \
		echo "   Please edit .env and set your private key"; \
		exit 1; \
	fi
	@echo "✅ Environment configuration looks good"

# Set up environment
setup:
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "✅ Created .env file from .env.example"; \
		echo "📝 Please edit .env and set your PRIVATE_KEY"; \
	else \
		echo "ℹ️  .env file already exists"; \
	fi

# Start services
up: check-env validate
	@echo "🚀 Starting AVS environment..."
	docker compose up -d
	@echo "✅ Services started. Run 'make logs' to view logs"

# Stop services
down:
	@echo "🛑 Stopping AVS environment..."
	docker compose down

# View logs
logs:
	docker compose logs -f

# Clean environment (remove volumes)
clean:
	@echo "🧹 Cleaning up environment..."
	docker compose down -v
	docker system prune -f

# Validate configuration
validate:
	@echo "🔍 Validating configuration..."
	@./validate-compose.sh

# Restart services
restart: down up

# Build images locally (if Dockerfiles exist)
build-local:
	@echo "🔨 Building local images..."
	@if [ -f Dockerfile ]; then \
		docker build -t commonware-avs-router:local .; \
	else \
		echo "ℹ️  No Dockerfile found for router"; \
	fi
	@if [ -f commonware-avs-node/Dockerfile ]; then \
		docker build -t commonware-avs-node:local ./commonware-avs-node; \
	else \
		echo "ℹ️  No Dockerfile found for node"; \
	fi

# Service-specific targets
logs-router:
	docker compose logs -f avs-router

logs-node1:
	docker compose logs -f avs-node-contributor-1

logs-node2:
	docker compose logs -f avs-node-contributor-2

logs-node3:
	docker compose logs -f avs-node-contributor-3

logs-ethereum:
	docker compose logs -f ethereum

logs-eigenlayer:
	docker compose logs -f eigenlayer

# Status
status:
	docker compose ps

# Interactive shell access
shell-router:
	docker compose exec avs-router sh

shell-node1:
	docker compose exec avs-node-contributor-1 sh

shell-ethereum:
	docker compose exec ethereum sh