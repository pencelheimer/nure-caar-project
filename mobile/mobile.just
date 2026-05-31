# ─── mobile (Android / Gradle) ──────────────────────────────────────────────

host := `ip route get 1 | awk '{print $7; exit}'`

check:
    ./gradlew lint

build:
    ./gradlew assembleDebug -PapiHost={{ host }}

install:
    ./gradlew installDebug -PapiHost={{ host }}
    adb shell am start -n com.smarttank.app/.MainActivity

release:
    ./gradlew assembleRelease -PapiHost={{ host }}

logs:
    #!/usr/bin/env bash
    PID=$(adb shell pidof -s com.smarttank.app 2>/dev/null)
    if [ -n "$PID" ]; then
        adb logcat --pid="$PID" -v brief
    else
        echo "App not running — last crash:"
        adb logcat -d -v brief | grep -A 30 "FATAL EXCEPTION"
    fi

# Generate mipmap PNGs from app/ic_launcher_source.png (requires ImageMagick).
# Source must be PNG ≥1024×1024 with transparent background (logo only).
icon:
    #!/usr/bin/env bash
    set -euo pipefail
    SRC="app/ic_launcher_source.png"
    RES="app/src/main/res"
    if [ ! -f "$SRC" ]; then
        echo "Missing $SRC — place your 1024×1024 PNG there first."
        exit 1
    fi
    declare -A SIZES=([mdpi]=48 [hdpi]=72 [xhdpi]=96 [xxhdpi]=144 [xxxhdpi]=192)
    declare -A FG=([mdpi]=108 [hdpi]=162 [xhdpi]=216 [xxhdpi]=324 [xxxhdpi]=432)
    BG="#1565C0"
    for DPI in mdpi hdpi xhdpi xxhdpi xxxhdpi; do
        SZ=${SIZES[$DPI]}
        FG_SZ=${FG[$DPI]}
        DIR="$RES/mipmap-$DPI"
        # ic_launcher — logo composited on solid blue background
        magick "$SRC" \
            -resize "${SZ}x${SZ}" \
            \( -clone 0 -alpha off -fill "$BG" -colorize 100 \) \
            -swap 0,1 -composite \
            "$DIR/ic_launcher.png"
        # ic_launcher_round — same but circular crop
        magick "$SRC" \
            -resize "${SZ}x${SZ}" \
            \( -clone 0 -alpha off -fill "$BG" -colorize 100 \) \
            -swap 0,1 -composite \
            \( +clone -threshold 100% -fill white -draw "circle $((SZ/2)),$((SZ/2)) $((SZ/2)),0" \) \
            -alpha set -compose DstIn -composite \
            "$DIR/ic_launcher_round.png"
        # ic_launcher_foreground — logo only, scaled to adaptive safe zone (66% of FG_SZ)
        LOGO_SZ=$(echo "$FG_SZ * 66 / 100" | bc)
        PAD=$(( (FG_SZ - LOGO_SZ) / 2 ))
        magick "$SRC" \
            -resize "${LOGO_SZ}x${LOGO_SZ}" \
            -gravity center \
            -background none \
            -extent "${FG_SZ}x${FG_SZ}" \
            "$DIR/ic_launcher_foreground.png"
    done
    echo "Icons generated."

# Generate icons from a Nerd Fonts glyph (requires ImageMagick + font file).
# Usage: just icon-glyph glyph='󰆫' font='/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Regular.ttf'
icon-glyph glyph='󰆫' font='/usr/share/fonts/TTF/IosevkaTermNerdFontMono-Regular.ttf':
    #!/usr/bin/env bash
    set -euo pipefail
    GLYPH="{{ glyph }}"
    FONT="{{ font }}"
    RES="app/src/main/res"
    BG="#1565C0"
    if [ ! -f "$FONT" ]; then
        echo "Font not found: $FONT"
        exit 1
    fi
    declare -A SIZES=([mdpi]=48 [hdpi]=72 [xhdpi]=96 [xxhdpi]=144 [xxxhdpi]=192)
    declare -A FG=([mdpi]=108 [hdpi]=162 [xhdpi]=216 [xxhdpi]=324 [xxxhdpi]=432)
    for DPI in mdpi hdpi xhdpi xxhdpi xxxhdpi; do
        SZ=${SIZES[$DPI]}
        FG_SZ=${FG[$DPI]}
        DIR="$RES/mipmap-$DPI"
        PT=$(( SZ * 60 / 100 ))
        # ic_launcher + ic_launcher_round — glyph on blue square/circle
        for NAME in ic_launcher ic_launcher_round; do
            magick -size "${SZ}x${SZ}" "xc:$BG" \
                -font "$FONT" -pointsize "$PT" \
                -fill white -gravity center \
                -annotate 0 "$GLYPH" \
                "$DIR/${NAME}.png"
        done
        # ic_launcher_foreground — glyph on transparent background for adaptive icon
        PT_FG=$(( FG_SZ * 50 / 100 ))
        magick -size "${FG_SZ}x${FG_SZ}" xc:none \
            -font "$FONT" -pointsize "$PT_FG" \
            -fill white -gravity center \
            -annotate 0 "$GLYPH" \
            "$DIR/ic_launcher_foreground.png"
    done
    echo "Icons generated from glyph '$GLYPH'."
