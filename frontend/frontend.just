# ─── frontend (Vue / Vite) ────────────────────────────────────────────────────

api_url := env_var_or_default("API_URL", "http://localhost:8080")

dev:
    API_URL={{ api_url }} npm run dev

build:
    API_URL={{ api_url }} npm run build

install:
    npm install
