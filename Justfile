set dotenv-load
set quiet

mod backend  "backend/backend.just"
mod frontend "frontend/frontend.just"
mod iot      "iot/iot.just"
mod mobile   "mobile/mobile.just"
mod seed     "seed/seed.just"

k6_image := "grafana/k6:0.41.0"
base_url := "http://localhost:8080"
influx_url := "http://localhost:8086/k6"

db:
    pgcli postgres://${DB_USERNAME}:${DB_PASSWORD}@localhost:${DB_PORT:-5432}/${DB_NAME}

# ─── dev ────────────────────────────────────────────────────────────────────

up:
    docker compose up -d

down:
    docker compose down

logs service="backend":
    docker compose logs -f {{ service }}

rebuild:
    docker compose up -d --build backend

# ─── load testing ───────────────────────────────────────────────────────────

# Run the full scaling benchmark: 1 → 2 → 4 backend replicas
benchmark: _infra
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p infra/load-tests/results
    for replicas in 1 2 4; do
        docker compose up --scale backend="$replicas" -d
        just _wait-backend
        just _k6 "$replicas"
    done

# Run a single k6 test against N replicas (default: 1)
test replicas="1": _infra
    docker compose up --scale backend={{ replicas }} -d
    just _wait-backend
    just _k6 {{ replicas }}

# ─── private helpers ────────────────────────────────────────────────────────

_infra:
    docker compose up -d db influxdb grafana caddy
    docker compose up migrator

_wait-backend:
    #!/usr/bin/env bash
    for _ in $(seq 1 30); do
        code=$(curl -s -o /dev/null -w '%{http_code}' {{ base_url }}/health 2>/dev/null || echo "000")
        [ "$code" = "200" ] && exit 0
        sleep 2
    done
    echo "ERROR: backend not healthy" >&2
    exit 1

_k6 replicas:
    docker run --rm \
        --network host \
        -v "$(pwd)/infra/load-tests:/scripts" \
        {{ k6_image }} run \
        --out "influxdb={{ influx_url }}" \
        -e BASE_URL={{ base_url }} \
        -e REPLICAS={{ replicas }} \
        /scripts/main.js
