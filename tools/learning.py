#!/usr/bin/env python3
"""Nghiên cứu siêu tham số cho bộ não học bằng gradient.

Đây là phần một sinh viên máy học cần nhìn thấy mà chạy sim không cho thấy: đường học đi xuống
hay không, và tốc độ học nào thì tốt. Nó quét một lưới tham số, mỗi cấu hình chạy vài thế giới,
rồi vẽ đường sai số dự báo theo thời gian và bảng kết quả cuối.

    python3 tools/learning.py                  # quét mặc định
    python3 tools/learning.py --seeds 1-5 --ticks 10000

Kết quả: docs/lab/learning-curves.svg và docs/lab/learning-curves.md

Đọc gì ở đó. Sai số dự báo (`td_error`) là |phần thưởng thực tế trừ điều nhà phê bình đoán|. Nếu
nó đi xuống thì nhà phê bình đang học được thế giới; nếu nó phẳng ở mức cao thì hoặc thế giới
không đoán được, hoặc bước học sai. Cột kết quả cuối cho thấy điều quan trọng hơn: sai số nhỏ
nhất **không** đồng nghĩa với xã hội khá nhất, và đó là bài học thật của máy học ứng dụng.
"""
import csv
import os
import statistics as st
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SIM = os.environ.get("AISV_SIM") or os.path.join(ROOT, "sim", "target", "release", "sim")
OUT = os.path.join(ROOT, "docs", "lab")
WORK = os.path.join(ROOT, "sim", "target", "learning")

# Một siêu tham số được quét, phần còn lại giữ nguyên: đó là cách duy nhất để biết cái gì gây ra cái gì.
GRID = [
    ("actor 0.003", ["--actor-rate", "0.003"]),
    ("actor 0.01", ["--actor-rate", "0.01"]),
    ("actor 0.03", ["--actor-rate", "0.03"]),
    ("actor 0.1", ["--actor-rate", "0.1"]),
    ("actor 0.3", ["--actor-rate", "0.3"]),
    ("hebb (đối chứng)", None),
]
COLOURS = ["#f2c14e", "#6bbf73", "#5aa9e6", "#d9534f", "#b07ad9", "#e6ebf5"]


def run(name, flags, seeds, ticks):
    """Chạy một cấu hình trên nhiều seed, trả về các đường và kết quả cuối."""
    curves, finals = [], []
    for seed in seeds:
        d = os.path.join(WORK, name.replace(" ", "_"), str(seed))
        os.makedirs(d, exist_ok=True)
        cmd = [SIM, "--seed", str(seed), "--ticks", str(ticks), "--quiet", "--out", d]
        if flags is not None:
            cmd += ["--gradient"] + flags
        r = subprocess.run(cmd, capture_output=True, text=True)
        if r.returncode != 0:
            print("  hỏng:", (r.stderr or r.stdout).strip().splitlines()[-1][:100])
            continue
        path = os.path.join(d, f"stats_seed{seed}.csv")
        rows = list(csv.DictReader(open(path))) if os.path.exists(path) else []
        if not rows:
            continue
        curves.append([(float(x["tick"]), float(x.get("td_error", 0) or 0)) for x in rows])
        last = rows[-1]
        finals.append((float(last["pop"]), float(last["mean_known"]), float(last["innovations"])))
    return curves, finals


def median_curve(curves):
    """Trung vị theo từng mốc thời gian, nên một thế giới chết không kéo cả đường xuống."""
    if not curves:
        return []
    n = min(len(c) for c in curves)
    return [(curves[0][i][0], st.median(c[i][1] for c in curves)) for i in range(n)]


def svg(series, path, title, ylab):
    W, H, L, R, T, B = 900, 380, 64, 210, 34, 44
    xs = [p[0] for s in series for p in s["points"]] or [0, 1]
    ys = [p[1] for s in series for p in s["points"]] or [0, 1]
    x0, x1, y1 = min(xs), max(xs) or 1, max(ys) or 1
    px = lambda x: L + (x - x0) / max(1e-9, x1 - x0) * (W - L - R)
    py = lambda y: H - B - y / max(1e-9, y1) * (H - T - B)
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="ui-monospace,monospace">',
         f'<rect width="{W}" height="{H}" fill="#0c1018"/>',
         f'<text x="{L}" y="21" fill="#e6ebf5" font-size="14">{title}</text>']
    for k in range(5):
        y = y1 * k / 4
        o.append(f'<line x1="{L}" y1="{py(y):.1f}" x2="{W-R}" y2="{py(y):.1f}" stroke="#222a38"/>')
        o.append(f'<text x="{L-8}" y="{py(y)+4:.1f}" fill="#7f8a9e" font-size="11" text-anchor="end">{y:.2f}</text>')
    o.append(f'<text x="{L}" y="{H-12}" fill="#7f8a9e" font-size="11">tick</text>')
    o.append(f'<text x="8" y="{T-14}" fill="#7f8a9e" font-size="11">{ylab}</text>')
    for i, s in enumerate(series):
        if not s["points"]:
            continue
        d = " ".join(("M" if j == 0 else "L") + f"{px(x):.1f},{py(y):.1f}" for j, (x, y) in enumerate(s["points"]))
        o.append(f'<path d="{d}" fill="none" stroke="{s["colour"]}" stroke-width="1.8"/>')
        o.append(f'<rect x="{W-R+10}" y="{T+i*20-8}" width="9" height="9" fill="{s["colour"]}"/>')
        o.append(f'<text x="{W-R+25}" y="{T+i*20}" fill="#c9d4e6" font-size="11">{s["name"]}</text>')
    o.append("</svg>")
    open(path, "w").write("\n".join(o))


def main():
    seeds, ticks = list(range(1, 4)), 6000
    args = sys.argv[1:]
    while args:
        k = args.pop(0)
        if k == "--seeds":
            a, b = args.pop(0).split("-")
            seeds = list(range(int(a), int(b) + 1))
        elif k == "--ticks":
            ticks = int(args.pop(0))
    if not os.path.exists(SIM):
        sys.exit("dựng sim trước: cd sim && cargo build --release")
    os.makedirs(OUT, exist_ok=True)
    series, table = [], []
    for i, (name, flags) in enumerate(GRID):
        print("chạy", name, flush=True)
        curves, finals = run(name, flags, seeds, ticks)
        series.append({"name": name, "colour": COLOURS[i % len(COLOURS)], "points": median_curve(curves)})
        if finals:
            table.append((name, st.median(f[0] for f in finals), st.median(f[1] for f in finals),
                          st.median(f[2] for f in finals), st.median(p[1] for c in curves for p in c[len(c)//2:]), flags is not None))
    svg(series, os.path.join(OUT, "learning-curves.svg"), "Sai số dự báo của nhà phê bình theo thời gian", "td_error")
    md = ["# Quét siêu tham số cho bộ não học bằng gradient", "",
          f"{len(seeds)} thế giới mỗi cấu hình, {ticks:,} tick. Chỉ một siêu tham số thay đổi; phần còn lại giữ nguyên.", "",
          "![đường học](learning-curves.svg)", "",
          "| cấu hình | dân cuối | kiến thức | phát minh | sai số dự báo (nửa sau) |", "|---|---|---|---|---|"]
    for name, pop, known, innov, td, has_critic in table:
        cell = f"{td:.3f}" if has_critic else "– (không có nhà phê bình)"
        md.append(f"| {name} | {pop:.0f} | {known:.1f} | {innov:.0f} | {cell} |")
    # Nhánh Hebb không có nhà phê bình, nên sai số của nó bằng 0 theo cấu tạo chứ không phải vì
    # đoán giỏi. So một con số không tồn tại với một con số có tồn tại chính là kiểu sai lầm mà
    # trang này cảnh báo, nên nó bị loại khỏi phép so.
    scored = [r for r in table if r[5]]
    if scored:
        by_loss = min(scored, key=lambda r: r[4])
        by_life = max(scored, key=lambda r: r[2])
        md += ["", "## Kết quả lần chạy này", ""]
        if by_loss[0] != by_life[0]:
            md += [f"Cấu hình **đoán giỏi nhất** là `{by_loss[0]}`, sai số {by_loss[4]:.3f}, và nó kết thúc với",
                   f"{by_loss[1]:.0f} người và {by_loss[2]:.1f} thứ mỗi người biết.",
                   f"Cấu hình **sống tốt nhất** là `{by_life[0]}`, sai số {by_life[4]:.3f}, kết thúc với",
                   f"{by_life[1]:.0f} người và {by_life[2]:.1f} thứ mỗi người biết.", "",
                   "Hai cấu hình đó **không phải một**. Chọn theo hàm mất mát là chọn sai."]
        else:
            md += [f"Lần này cấu hình đoán giỏi nhất cũng là cấu hình sống tốt nhất (`{by_life[0]}`).",
                   "Điều đó không phải luật; chạy lại với seed khác thường thấy hai cái tách ra."]
    md += ["", "## Đọc bảng này thế nào", "",
           "Cột cuối là thứ một người mới học hay bám vào: sai số dự báo càng nhỏ càng tốt. Ba cột trước",
           "là thứ thật sự quan trọng. Nếu cấu hình có sai số nhỏ nhất lại không phải cấu hình cho xã hội",
           "khá nhất, thì bạn vừa gặp bài học trung tâm của máy học ứng dụng: **hàm mất mát không phải mục",
           "tiêu, nó chỉ là thứ đo được**. Chọn mô hình theo hàm mất mát mà không nhìn kết quả thật là cách",
           "hỏng việc phổ biến nhất trong ngành.", "",
           "_Sinh bởi `python3 tools/learning.py`._"]
    open(os.path.join(OUT, "learning-curves.md"), "w").write("\n".join(md) + "\n")
    print("viết -> docs/lab/learning-curves.md và learning-curves.svg")


if __name__ == "__main__":
    main()
