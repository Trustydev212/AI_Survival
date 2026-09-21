#!/usr/bin/env python3
"""Chấm điểm bộ não: đổi một thứ trong cách huấn luyện, đo trên thế giới chưa từng thấy.

Khác với tools/lab.py vốn đo xã hội, cái này đo **bộ não**. Mọi chỉ số xã hội đều trộn bộ não với
đất đai, hàng xóm và vận may; muốn biết cái mạng có giỏi lên không thì phải mang nó ra khỏi thế
giới đã sinh ra nó. Quy trình: tiến hoá theo một cấu hình, lấy ark, thả hai mươi bản sao của từng
bộ não vào một thế giới mới mà không bộ nào từng sống ở đó, đếm người còn lại.

    python3 tools/brainlab.py                      # các nhánh mặc định
    python3 tools/brainlab.py --ticks 40000 --eval-ticks 2500

Kết quả: docs/lab/brainlab.md
"""
import os
import re
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SIM = os.environ.get("AISV_SIM") or os.path.join(ROOT, "sim", "target", "release", "sim")
WORK = os.path.join(ROOT, "sim", "target", "brainlab")
BASE = ["--forever", "--width", "288", "--height", "288", "--finds", "40", "--save-every", "15000", "--quiet"]

ARMS = [
    ("mặc định (Hebb)", []),
    ("không học trong đời", ["--learn-scale", "0"]),
    ("gradient bước chậm", ["--gradient", "--actor-rate", "0.01"]),
    ("có lai tạo", ["--mates"]),
]


def evolve_and_score(name, flags, seed, ticks, eval_ticks):
    d = os.path.join(WORK, re.sub(r"\W+", "_", name))
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d, exist_ok=True)
    r = subprocess.run([SIM] + BASE + ["--seed", str(seed), "--ticks", str(ticks), "--out", d] + flags,
                       capture_output=True, text=True)
    if r.returncode != 0:
        return None, (r.stderr or r.stdout).strip().splitlines()[-1][:110]
    ark = os.path.join(d, "ark.bin")
    if not os.path.exists(ark):
        return None, "không có ark"
    r = subprocess.run([SIM, "--eval", ark, "--ticks", str(eval_ticks)], capture_output=True, text=True)
    out = r.stdout + r.stderr
    rand = re.search(r"^random\s+\d+\s+([\d.]+)", out, re.M)
    trained = re.search(r"^from the ark\s+\d+\s+([\d.]+)\s+([\d.]+)", out, re.M)
    if not (rand and trained):
        return None, "không đọc được điểm"
    return (float(rand.group(1)), float(trained.group(1)), float(trained.group(2))), None


def main():
    seeds, ticks, eval_ticks = [77], 30000, 2500
    args = sys.argv[1:]
    while args:
        k = args.pop(0)
        if k == "--ticks":
            ticks = int(args.pop(0))
        elif k == "--eval-ticks":
            eval_ticks = int(args.pop(0))
        elif k == "--seeds":
            a, b = args.pop(0).split("-")
            seeds = list(range(int(a), int(b) + 1))
    if not os.path.exists(SIM):
        sys.exit("dựng sim trước: cd sim && cargo build --release")
    rows = []
    for name, flags in ARMS:
        for seed in seeds:
            label = name if len(seeds) == 1 else f"{name} (seed {seed})"
            print("chạy", label, flush=True)
            got, err = evolve_and_score(label, flags, seed, ticks, eval_ticks)
            if err:
                print("  ", err)
                continue
            rand, trained, spread = got
            rows.append((label, rand, trained, spread, trained / max(rand, 0.01)))
    md = ["# Chấm điểm bộ não trên thế giới chưa từng thấy", "",
          f"Tiến hoá {ticks:,} tick theo từng cấu hình, rồi thả hai mươi bản sao của từng bộ não trong ark",
          f"vào một thế giới mới 64×64, năm lần thử, đếm người còn lại sau {eval_ticks:,} tick.", "",
          "| cách huấn luyện | não ngẫu nhiên | não đã huấn luyện | lệch | gấp mấy lần |", "|---|---|---|---|---|"]
    for name, rand, trained, spread, lift in rows:
        md.append(f"| {name} | {rand:.1f} | **{trained:.1f}** | {spread:.1f} | {lift:.1f}× |")
    md += ["", "## Đọc bảng này thế nào", "",
           "Cột cuối là thứ đáng nhìn: bộ não đã huấn luyện hơn bộ não ngẫu nhiên bao nhiêu lần. Cột lệch",
           "cho biết các bộ não trong cùng một ark khác nhau nhiều hay ít; lệch lớn nghĩa là ark đang giữ",
           "vài bộ giỏi lẫn vài bộ tầm thường, chứ không phải cả đàn đều khá.", "",
           "Một cảnh báo về so sánh: mỗi kiến trúc có đường cơ sở ngẫu nhiên của riêng nó, nên so tỉ lệ",
           "giữa các nhánh thì được, so điểm tuyệt đối giữa hai kiến trúc khác nhau thì không.", "",
           "_Sinh bởi `python3 tools/brainlab.py`._"]
    os.makedirs(os.path.join(ROOT, "docs", "lab"), exist_ok=True)
    path = os.path.join(ROOT, "docs", "lab", "brainlab.md")
    open(path, "w").write("\n".join(md) + "\n")
    print("viết ->", os.path.relpath(path, ROOT))


if __name__ == "__main__":
    main()
