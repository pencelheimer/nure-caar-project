# ─── iot (ESP32 / esp-hal) ──────────────────────────────────────────────────

check:
    cargo check

build:
    cargo build --release

flash:
    cargo run --release

monitor port="/dev/ttyUSB0":
    espflash monitor {{ port }}
