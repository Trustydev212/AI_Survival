#!/bin/bash
# Put the current slice of the world on the web.
#
# Builds a small static site (the viewer plus the most recent window of the world) and force
# pushes it as a single commit to the gh-pages branch. Single commit on purpose: a few megabytes
# published every quarter of an hour would otherwise grow the repository without end, and none of
# those old slices is worth keeping. The records that are worth keeping, the chronicle and the
# statistics, are small and are committed on the main branch by hand.
set -eu
cd "$(dirname "$0")/.."

WORLD_DIR="${1:-$PWD/world}"
# DRY=<dir> builds the site there and pushes nothing, which is how to see what would be published
# before pointing it at a real repository.
OUT="${DRY:-$(mktemp -d)}"
mkdir -p "$OUT"
[ -n "${DRY:-}" ] || trap 'rm -rf "$OUT"' EXIT

cp viewer/index.html "$OUT/"
# The drawing library ships with the repository on purpose: a published world that fetches it
# from somewhere else stops working the day that somewhere else does.
cp -r viewer/lib "$OUT/lib"
cp -r viewer/assets "$OUT/assets" 2>/dev/null || true
mkdir -p "$OUT/out"
# The viewer asks for a seed; the endless world publishes itself as seed 0.
cp "$WORLD_DIR/live.bin" "$OUT/out/snap_seed0.bin"
cp "$WORLD_DIR/live.json" "$OUT/out/meta_seed0.json"
newest_events=$(ls -t "$WORLD_DIR"/events_gen*.txt 2>/dev/null | head -1 || true)
[ -n "$newest_events" ] && cp "$newest_events" "$OUT/out/events_seed0.txt" || : > "$OUT/out/events_seed0.txt"
cp "$WORLD_DIR/chronicle.txt" "$OUT/chronicle.txt" 2>/dev/null || : > "$OUT/chronicle.txt"
cp "$WORLD_DIR/postmortem.txt" "$OUT/postmortem.txt" 2>/dev/null || : > "$OUT/postmortem.txt"
cp "$WORLD_DIR/status.json" "$OUT/status.json" 2>/dev/null || : > "$OUT/status.json"
echo '<meta http-equiv="refresh" content="0; url=index.html?seed=0&live=1">' > "$OUT/live.html"

if [ -n "${DRY:-}" ]; then
  echo "built into $OUT ($(du -sh "$OUT" | cut -f1)), nothing pushed"
  exit 0
fi

cd "$OUT"
git init -q
git checkout -q -b gh-pages
git add -A
git -c user.email=world@ai-survival -c user.name="the world" commit -q -m "the world at $(date -u +%FT%TZ)"
git push -q --force "$(cd - >/dev/null && git remote get-url origin)" gh-pages
echo "$(date -u +%FT%TZ) published $(du -sh "$OUT" | cut -f1)"
