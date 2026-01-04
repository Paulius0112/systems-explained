#!/bin/bash

set -e

echo "=== TCP Congestion Demo Recorder ==="
echo ""

if ! command -v asciinema &> /dev/null; then
    echo "Error: asciinema not found"
    echo "Install with: brew install asciinema"
    exit 1
fi

if ! command -v agg &> /dev/null; then
    echo "Error: agg not found"
    echo "Install with: brew install agg"
    exit 1
fi

echo "Building release version..."
cargo build --release

echo ""
echo "Recording animation..."
echo ">>> The visualization will start. Wait ~30 seconds to show the sawtooth pattern."
echo ">>> Press 'q' to quit when ready."
echo ""
read -p "Press Enter to start recording..."

asciinema rec demo.cast \
    --cols 120 \
    --rows 35 \
    --command "./target/release/tcp-congestion" \
    --overwrite

echo ""
echo "Converting to GIF..."

agg demo.cast demo.gif \
    --cols 120 \
    --rows 35 \
    --font-size 14 \
    --theme monokai

echo ""
echo "=== Done! ==="
echo "Created: demo.gif"
echo ""

ls -lh demo.gif

if command -v ffmpeg &> /dev/null; then
    echo ""
    echo "Converting to MP4..."
    ffmpeg -y -i demo.gif -movflags faststart -pix_fmt yuv420p -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" demo.mp4 2>/dev/null
    echo "Created: demo.mp4"
    ls -lh demo.mp4
fi

echo ""
echo "You can now use demo.gif in your article!"
