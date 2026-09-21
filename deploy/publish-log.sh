#!/bin/bash
# Đẩy nhật ký của thế giới lên repo, để đọc được từ bất cứ đâu.
#
# Bốn file, tất cả đều nhỏ trừ ark: biên niên sử của các nền văn minh, lý do từng nền chết, dòng
# trạng thái hiện tại, và hai mươi bộ não tốt nhất. Chúng đi lên nhánh riêng `world-log` bằng một
# commit duy nhất bị force push đè: nếu giữ lịch sử thì một năm chạy sẽ thành vài GB, mà lịch sử
# thật đã nằm sẵn trong chính nội dung biên niên sử rồi.
#
#   ./deploy/publish-log.sh [world_dir]
#
# Cần quyền đẩy: deploy key có quyền ghi, hoặc token trong remote.
set -eu
cd "$(dirname "$0")/.."

WORLD_DIR="${1:-$PWD/world}"
REMOTE=$(git remote get-url origin)
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT

for f in chronicle.txt postmortem.txt status.json ark.bin; do
  [ -f "$WORLD_DIR/$f" ] && cp "$WORLD_DIR/$f" "$OUT/$f" || true
done
[ -f "$OUT/status.json" ] || { echo "chưa có status.json, thế giới chưa chạy đủ lâu"; exit 0; }

cat > "$OUT/README.md" <<'MD'
# Nhật ký của thế giới đang chạy

Nhánh này do máy chạy sim tự đẩy lên, một commit duy nhất bị ghi đè mỗi lần.

| file | là gì |
|---|---|
| `status.json` | thế giới đang ở tick nào, bao nhiêu người, nhanh bao nhiêu |
| `chronicle.txt` | mỗi dòng một nền văn minh: sống bao lâu, vươn tới đâu, chết vì gì |
| `postmortem.txt` | khám nghiệm từng nền: đỉnh ở đâu, suy tàn bao lâu, thứ sâu nhất từng làm |
| `ark.bin` | hai mươi bộ não tốt nhất, chấm điểm được bằng `sim --eval ark.bin` |

Chấm điểm bộ não trên một thế giới chưa từng thấy:

```bash
sim --eval ark.bin --ticks 2500
```
MD

cd "$OUT"
git init -q
git checkout -q -b world-log
git add -A
git -c user.email=world@ai-survival -c user.name="the world" commit -q -m "nhật ký thế giới $(date -u +%FT%TZ)"
git push -q --force "$REMOTE" world-log
echo "$(date -u +%FT%TZ) đã đẩy nhật ký ($(du -sh "$OUT" | cut -f1))"
