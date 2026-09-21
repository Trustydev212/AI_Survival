#!/bin/bash
# Chạy một chương trình dài không cần ai trông.
#
# Mỗi thí nghiệm mất từ nửa tiếng tới một tiếng, và nếu đợi từng cái xong rồi mới xếp cái
# tiếp thì phần lớn thời gian máy nằm không còn người thì ngồi chờ. Script này chạy cả một
# danh sách liền mạch, viết báo cáo sau mỗi bước, và dựng lại danh mục đồ vật mỗi lần xong
# một thí nghiệm, nên bỏ đi vài tiếng quay lại là có kết quả.
#
#   nohup ./tools/programme.sh > /tmp/programme.log 2>&1 &
#   tail -f /tmp/programme.log
#
# Thêm hoặc bớt dòng trong PLAN. Mỗi dòng: <tên thí nghiệm> <dãy seed> <số tick>.
set -u
cd "$(dirname "$0")/.."

PLAN=(
  "food 1-4 60000"
  "long 1-8 60000"
  "brains 1-8 60000"
)

cd sim && cargo build --release 2>&1 | tail -1 && cd ..
for step in "${PLAN[@]}"; do
  set -- $step
  echo "=== $(date -u +%H:%M) bắt đầu $1 (seed $2, $3 tick) ==="
  python3 tools/lab.py run "$1" --seeds "$2" --ticks "$3" || echo "!! $1 hỏng, đi tiếp"
  python3 tools/things.py
  python3 tools/lab.py index
  echo "=== $(date -u +%H:%M) xong $1 ==="
done
echo CHUONGTRINH_XONG
