#!/usr/bin/env python3
"""The catalogue of everything the worlds have ever made.

Every run leaves an events file naming each thing invented, its recipe, what it turned out to be
and what it does. Those files had piled up for hundreds of worlds and nobody had ever read them
together. This does: it walks every events file under docs/lab/runs and writes docs/THINGS.md.

    python3 tools/things.py

Nothing here is a designed object. The recipes are what the materials allowed, found by agents
trying combinations, so the catalogue is a record of what this physics affords and what the
societies actually reached for.
"""
import collections
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RUNS = os.path.join(ROOT, "docs", "lab", "runs")
LINE = re.compile(r"^(\d+)\tcraft\tcrafted: (\w+) = (.+?) -> (\w+) \(t(\d+): (.*?)\) by lineage (\d+)")
SLOT_VI = {"weapon": "vũ khí", "shelter": "nơi trú", "vessel": "đồ đựng", "boat": "thuyền",
           "tool": "công cụ", "fire": "lửa", "armour": "giáp"}
MATS = ["wood", "stone", "fibre", "clay", "ore", "bone"]


def depth(recipe):
    """How many processes deep the recipe goes: hollow(sharpen(wood)) is two."""
    return recipe.count("(")


def walk():
    for base, _, files in os.walk(RUNS):
        for f in files:
            if f.startswith("events_seed") and f.endswith(".txt"):
                yield os.path.join(base, f)


def main():
    things, by_slot, by_recipe, by_process, by_mat = [], collections.Counter(), collections.Counter(), collections.Counter(), collections.Counter()
    worlds = first_tick = 0
    deepest = {}
    for path in walk():
        worlds += 1
        with open(path, errors="ignore") as fh:
            for line in fh:
                m = LINE.match(line)
                if not m:
                    continue
                tick, name, recipe, slot, tier, effects, lineage = m.groups()
                d = depth(recipe)
                things.append((int(tick), name, recipe, slot, int(tier), d))
                by_slot[slot] += 1
                by_recipe[recipe] += 1
                for p in re.findall(r"(\w+)\(", recipe):
                    by_process[p] += 1
                for mat in MATS:
                    if re.search(rf"\b{mat}\b", recipe):
                        by_mat[mat] += 1
                if d > deepest.get(slot, (0,))[0]:
                    deepest[slot] = (d, name, recipe, int(tick))
    if not things:
        sys.exit("không tìm thấy sự kiện chế tác nào trong " + RUNS)
    n = len(things)
    out = ["# Mọi thứ các thế giới từng làm ra", "",
           f"Tổng hợp từ **{worlds} thế giới** đã chạy, **{n:,} lần phát minh**. Không món nào được viết sẵn:",
           "công thức là thứ vật liệu cho phép, do các agent thử ra. Sinh bằng `python3 tools/things.py`.", "",
           "## Họ làm ra cái gì", "", "| loại | số lần được phát minh | phần |", "|---|---|---|"]
    for slot, c in by_slot.most_common():
        if slot not in SLOT_VI:
            continue
        out.append(f"| {SLOT_VI[slot]} | {c:,} | {100 * c / n:.0f}% |")
    out += ["", "## Công thức hay gặp nhất", "",
            "Cùng một công thức được tìm lại ở nhiều thế giới khác nhau, mỗi nơi một cái tên khác.", "",
            "| công thức | số thế giới tìm ra |", "|---|---|"]
    for recipe, c in by_recipe.most_common(12):
        out.append(f"| `{recipe}` | {c:,} |")
    out += ["", "## Sâu nhất mỗi loại", "",
            "Số tầng là số lần một thứ đã làm lại bị đem đi chế tiếp.", "",
            "| loại | tầng | công thức | tick |", "|---|---|---|---|"]
    for slot, (d, name, recipe, tick) in sorted(deepest.items(), key=lambda kv: -kv[1][0]):
        if slot not in SLOT_VI:
            continue
        out.append(f"| {SLOT_VI[slot]} | {d} | `{recipe}` | {tick:,} |")
    out += ["", "## Thao tác và vật liệu", "", "| thao tác | lần dùng | | vật liệu | lần dùng |", "|---|---|---|---|---|"]
    procs, mats = by_process.most_common(), by_mat.most_common()
    for i in range(max(len(procs), len(mats))):
        p = f"`{procs[i][0]}` | {procs[i][1]:,}" if i < len(procs) else " | "
        m = f"`{mats[i][0]}` | {mats[i][1]:,}" if i < len(mats) else " | "
        out.append(f"| {p} | | {m} |")
    depths = collections.Counter(t[5] for t in things)
    out += ["", "## Bao nhiêu tầng", "", "| tầng | số phát minh |", "|---|---|"]
    for d in sorted(depths):
        out.append(f"| {d} | {depths[d]:,} |")
    early = sorted(things, key=lambda t: t[0])[:1]
    out += ["", f"_Phát minh sớm nhất từng ghi được: `{early[0][2]}` ở tick {early[0][0]}._"]
    path = os.path.join(ROOT, "docs", "THINGS.md")
    with open(path, "w") as f:
        f.write("\n".join(out) + "\n")
    print("viết ->", os.path.relpath(path, ROOT), f"({worlds} thế giới, {n:,} phát minh)")


if __name__ == "__main__":
    main()
