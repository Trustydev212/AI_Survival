#!/bin/bash
# Não to hơn có giỏi hơn không?
#
# Số nơ-ron ẩn là hằng số lúc biên dịch, nên quét nó phải dựng nhiều bản. Việc đó làm trên **bản
# sao** của mã nguồn, không bao giờ sửa cây làm việc: một script chạy nửa tiếng mà để repo ở trạng
# thái dở dang là cách tốt nhất để commit nhầm một thứ chưa ai định commit.
#
#   ./tools/architecture.sh 20 40 64
set -u
cd "$(dirname "$0")/.."
SIZES=${*:-"20 40 64"}
WORK=${WORK:-/tmp/ai-survival-arch}
TICKS=${TICKS:-30000}
EVAL_TICKS=${EVAL_TICKS:-2500}
SEEDS=${SEEDS:-"77 78 79"}

rm -rf "$WORK"; mkdir -p "$WORK"
cp -r sim "$WORK/sim"
rm -rf "$WORK/sim/target"

for H in $SIZES; do
  sed -i "s/^pub const N_HID: usize = .*/pub const N_HID: usize = $H;/" "$WORK/sim/src/brain.rs"
  CARGO_TARGET_DIR="$WORK/t$H" cargo build --release --manifest-path "$WORK/sim/Cargo.toml" 2>&1 | tail -1
  # Nhiều seed là bắt buộc, không phải xa xỉ: một seed cho mỗi kiến trúc đã từng cho 15x, 4.7x và
  # 53x, trong khi độ lệch bên trong từng ark còn lớn hơn khoảng cách giữa các kiến trúc.
  for SEED in $SEEDS; do
    D="$WORK/w${H}_$SEED"; rm -rf "$D"; mkdir -p "$D"
    s=$(date +%s)
    "$WORK/t$H/release/sim" --forever --seed "$SEED" --ticks "$TICKS" --width 288 --height 288 \
        --finds 40 --save-every 15000 --quiet --out "$D" >/dev/null 2>&1
    e=$(date +%s)
    echo "=== $H nơ-ron ẩn · seed $SEED · tiến hoá $TICKS tick trong $((e-s))s ==="
    "$WORK/t$H/release/sim" --eval "$D/ark.bin" --ticks "$EVAL_TICKS" 2>&1 | awk '/^random|^from the ark/'
  done
done
echo ARCHDONE
