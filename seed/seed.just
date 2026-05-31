# ─── seed ────────────────────────────────────────────────────────────────────

base     := env_var_or_default("BASE_URL", "http://localhost:8080")
tok      := ".seed-token"
res_ids  := ".seed-reservoir-ids"
api_keys := ".seed-api-keys"

# Full seed: register → reservoirs → devices → rules → measurements
all: login reservoirs devices rules measurements
    @echo "Done."

# Register demo user or log in if already exists; saves JWT to .seed-token
login:
    #!/usr/bin/env bash
    set -euo pipefail
    TOKEN=$(curl -sf -X POST "{{base}}/auth/register" \
        -H "Content-Type: application/json" \
        -d @user.json 2>/dev/null | jq -r .token) \
    || TOKEN=$(curl -sf -X POST "{{base}}/auth/login" \
        -H "Content-Type: application/json" \
        -d @user.json | jq -r .token)
    echo "$TOKEN" > {{tok}}
    echo "Logged in as $(jq -r .email user.json)."

# Create reservoirs from reservoirs.json; IDs saved to .seed-reservoir-ids
reservoirs:
    #!/usr/bin/env bash
    set -euo pipefail
    TOKEN=$(cat {{tok}})
    > {{res_ids}}
    while IFS= read -r payload; do
        ID=$(curl -sf -X POST "{{base}}/reservoirs" \
            -H "Authorization: Bearer $TOKEN" \
            -H "Content-Type: application/json" \
            -d "$payload" | jq -r .id)
        echo "$ID" >> {{res_ids}}
        echo "  reservoir: $(echo "$payload" | jq -r .name)  id=$ID"
    done < <(jq -c '.[]' reservoirs.json)

# Create devices from devices.json; reservoir_index resolved to saved IDs;
# API keys saved to .seed-api-keys (one per line, same order as devices.json)
devices:
    #!/usr/bin/env bash
    set -euo pipefail
    TOKEN=$(cat {{tok}})
    mapfile -t RES_IDS < {{res_ids}}
    > {{api_keys}}
    while IFS= read -r payload; do
        IDX=$(echo "$payload" | jq -r '.reservoir_index')
        RID="${RES_IDS[$IDX]}"
        BODY=$(echo "$payload" | jq --argjson rid "$RID" 'del(.reservoir_index) + {reservoir_id: $rid}')
        RESULT=$(curl -sf -X POST "{{base}}/devices" \
            -H "Authorization: Bearer $TOKEN" \
            -H "Content-Type: application/json" \
            -d "$BODY")
        echo "$RESULT" | jq -r .api_key >> {{api_keys}}
        echo "  device: $(echo "$BODY" | jq -r .name)  id=$(echo "$RESULT" | jq .id)"
    done < <(jq -c '.[]' devices.json)

# Create alert rules from rules.json; reservoir_index resolved to saved IDs
rules:
    #!/usr/bin/env bash
    set -euo pipefail
    TOKEN=$(cat {{tok}})
    mapfile -t RES_IDS < {{res_ids}}
    while IFS= read -r payload; do
        IDX=$(echo "$payload" | jq -r '.reservoir_index')
        RID="${RES_IDS[$IDX]}"
        BODY=$(echo "$payload" | jq --argjson rid "$RID" 'del(.reservoir_index) + {reservoir_id: $rid}')
        curl -sf -X POST "{{base}}/reservoirs/$RID/rules" \
            -H "Authorization: Bearer $TOKEN" \
            -H "Content-Type: application/json" \
            -d "$BODY" | jq '{id, condition_type, threshold}'
    done < <(jq -c '.[]' rules.json)

# Submit 24 h of synthetic measurements for every device
measurements:
    #!/usr/bin/env bash
    set -euo pipefail
    mapfile -t API_KEYS < {{api_keys}}
    mapfile -t CAPACITIES < <(jq -r '.[].capacity' reservoirs.json)
    NOW=$(date +%s)
    for i in "${!API_KEYS[@]}"; do
        KEY="${API_KEYS[$i]}"
        CAP="${CAPACITIES[$i]:-2000}"
        for h in $(seq 23 -1 0); do
            TS=$(date -u -d "@$((NOW - h * 3600))" +"%Y-%m-%dT%H:%M:%SZ")
            VAL=$(awk -v cap="$CAP" -v h="$h" \
                'BEGIN { printf "%.1f", cap * (0.55 + 0.15 * sin(h / 3.8)) }')
            curl -sf -X POST "{{base}}/devices/measurements" \
                -H "x-api-key: $KEY" \
                -H "Content-Type: application/json" \
                -d "[{\"value\":$VAL,\"timestamp\":\"$TS\"}]"
        done
        echo "  device $((i+1)): 24 measurements sent  (cap=${CAP}L)"
    done

# Delete all reservoirs and devices for the demo user, then clear state files
wipe: login
    #!/usr/bin/env bash
    set -euo pipefail
    TOKEN=$(cat {{tok}})
    AUTH="Authorization: Bearer $TOKEN"
    curl -sf "{{base}}/devices" -H "$AUTH" | jq -r '.[].id' | while read -r id; do
        curl -sf -X DELETE "{{base}}/devices/$id" -H "$AUTH"
        echo "  deleted device $id"
    done
    curl -sf "{{base}}/reservoirs" -H "$AUTH" | jq -r '.[].id' | while read -r id; do
        curl -sf -X DELETE "{{base}}/reservoirs/$id" -H "$AUTH"
        echo "  deleted reservoir $id"
    done
    rm -f {{tok}} {{res_ids}} {{api_keys}}
    @echo "Wiped."

# Remove generated token/state files without touching the database
clean:
    rm -f {{tok}} {{res_ids}} {{api_keys}}
    @echo "Seed state cleared."

# Trigger an alert by sending one measurement below the threshold for device 0
# Usage: just seed::trigger-alert [value]   (default: 100 L — below all less_than rules)
trigger-alert value="100":
    #!/usr/bin/env bash
    set -euo pipefail
    KEY=$(head -1 {{api_keys}})
    TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    echo "Sending value={{value}} L at $TS …"
    curl -sf -X POST "{{base}}/devices/measurements" \
        -H "x-api-key: $KEY" \
        -H "Content-Type: application/json" \
        -d "[{\"value\":{{value}},\"timestamp\":\"$TS\"}]"
    echo "Done — check push notification on device."
