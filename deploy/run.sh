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
# The world is put down every 50,000 ticks rather than every 5,000: the file is around fifteen
# megabytes, and at roughly 250 ticks a second a save every 5,000 would write it three times a
# minute, which is tens of gigabytes a day for nothing. Fifty thousand costs at most a few minutes
# of world if the machine dies.
#
#   WORLD_DIR   where the world and its records live (default ./world)
#   SHIFT       ticks per shift before the world is put down (default 200000)
#   PUBLISH     seconds between publishes to the web (0 = never)
set -u
cd "$(dirname "$0")/.."

WORLD_DIR="${WORLD_DIR:-$PWD/world}"
SHIFT="${SHIFT:-200000}"
# Section 17: the map was capping everything. Four times the land gave four times the people,
# five times the knowledge and three times the inventions, at a third of the tick rate. For a
# machine that runs anyway, that is a trade worth making.
MAP="${MAP:-384}"
# Section 18: rich spots nobody can see from a distance are the first thing in this world worth
# telling somebody about, and whether that ever turns into speech is a question for millions of
# ticks rather than for a screen. This is the run that can answer it, so they are on.
FINDS="${FINDS:-60}"
PUBLISH="${PUBLISH:-900}"
SIM="$PWD/sim/target/release/sim"

mkdir -p "$WORLD_DIR"
[ -x "$SIM" ] || { echo "build first: cd sim && cargo build --release"; exit 1; }

last_publish=0
while true; do
  "$SIM" --forever \
      --ticks "$SHIFT" \
      --save-every 50000 \
      --snapshot-every 200 \
      --snapshot-window 20000 \
      --quiet \
      --width "$MAP" --height "$MAP" \
      --finds "$FINDS" \
      --out "$WORLD_DIR" \
      >> "$WORLD_DIR/run.log" 2>&1
  code=$?
  if [ $code -ne 0 ]; then
    # A crash must not stop the world for good: the last save is at most 5000 ticks old.
    echo "$(date -u +%FT%TZ) sim exited $code, picking the world back up in 10s" >> "$WORLD_DIR/run.log"
    sleep 10
  fi
  # Each civilisation leaves a statistics file and a history file behind. A machine left running
  # for weeks would otherwise fill a directory with thousands of them, so only the last twenty
  # generations are kept in full; the chronicle keeps the one line that matters about the rest.
  ls -t "$WORLD_DIR"/stats_gen*.csv 2>/dev/null | tail -n +21 | xargs -r rm -f
  ls -t "$WORLD_DIR"/events_gen*.txt 2>/dev/null | tail -n +21 | xargs -r rm -f

  now=$(date +%s)
  if [ "$PUBLISH" -gt 0 ] && [ $((now - last_publish)) -ge "$PUBLISH" ]; then
    ./deploy/publish.sh "$WORLD_DIR" >> "$WORLD_DIR/publish.log" 2>&1 || true
    # Nhật ký đi lên nhánh riêng, nhỏ và đọc được từ bất cứ đâu. Hỏng thì bỏ qua: thế giới
    # không được dừng chỉ vì mạng trục trặc.
    ./deploy/publish-log.sh "$WORLD_DIR" >> "$WORLD_DIR/publish.log" 2>&1 || true
    last_publish=$now
  fi
done
