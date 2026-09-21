#!/bin/bash
# The world that does not stop.
#
# Runs the simulation forever on a small machine, putting the world down at intervals so a
# reboot costs minutes rather than a civilisation, and publishing a slice of it to the web every
# so often. When everyone dies the simulation writes down what that civilisation reached and
# starts another on fresh ground; this script does not need to know that happened.
#
#   ./deploy/run.sh                 # runs until stopped
#
# Settings come from the environment so systemd can override them without editing the file:
#   WORLD_DIR   where the world and its records live (default ./world)
#   SHIFT       ticks per shift before the world is put down (default 200000)
#   PUBLISH     seconds between publishes to the web (0 = never)
set -u
cd "$(dirname "$0")/.."

WORLD_DIR="${WORLD_DIR:-$PWD/world}"
SHIFT="${SHIFT:-200000}"
PUBLISH="${PUBLISH:-900}"
SIM="$PWD/sim/target/release/sim"

mkdir -p "$WORLD_DIR"
[ -x "$SIM" ] || { echo "build first: cd sim && cargo build --release"; exit 1; }

last_publish=0
while true; do
  "$SIM" --forever \
      --ticks "$SHIFT" \
      --save-every 5000 \
      --snapshot-every 200 \
      --snapshot-window 20000 \
      --quiet \
      --out "$WORLD_DIR" \
      >> "$WORLD_DIR/run.log" 2>&1
  code=$?
  if [ $code -ne 0 ]; then
    # A crash must not stop the world for good: the last save is at most 5000 ticks old.
    echo "$(date -u +%FT%TZ) sim exited $code, picking the world back up in 10s" >> "$WORLD_DIR/run.log"
    sleep 10
  fi
  now=$(date +%s)
  if [ "$PUBLISH" -gt 0 ] && [ $((now - last_publish)) -ge "$PUBLISH" ]; then
    ./deploy/publish.sh "$WORLD_DIR" >> "$WORLD_DIR/publish.log" 2>&1 || true
    last_publish=$now
  fi
done
