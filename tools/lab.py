#!/usr/bin/env python3
"""The lab: run controlled experiments on the world and write a report.

An experiment is a question with two or more arms that differ in one setting, run over
the same seeds, compared on the outcome table the sim writes (out/experiment_A_B.csv).
Nothing here needs more than the standard library.

    python3 tools/lab.py list                      # experiments defined in tools/experiments.json
    python3 tools/lab.py run orders --seeds 1-16   # run every arm, then report
    python3 tools/lab.py report orders             # report again from the files on disk
    python3 tools/lab.py run all --seeds 1-8 --ticks 20000
    python3 tools/lab.py run all --seeds 1-16 --reuse   # skip arms already run on these seeds
    python3 tools/lab.py index                     # docs/lab/README.md: every experiment at a glance
    python3 tools/lab.py screen brains             # cheap first look: 8 seeds, 8000 ticks

Reports land in docs/lab/<name>.md: per-arm outcome counts, medians of every metric, and
for each treatment arm the difference of means against the control with a bootstrap 95%
interval. Eight seeds tell you the direction; sixteen or more start to tell you the size.
"""
import csv
import json
import os
import random
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SIM = os.path.join(ROOT, "sim", "target", "release", "sim")
LAB = os.path.join(ROOT, "docs", "lab")
DEFS = os.path.join(ROOT, "tools", "experiments.json")
METRICS = ["peak_pop", "final_pop", "innovations", "mean_known", "soil_health", "lived_soil", "settled_share",
           "obedience", "breed_rate", "swing", "final_level", "plastic", "signal_mi", "signal_meaning", "things_per_head", "equipped_share", "crafts", "learn_rate", "loudness", "hunts", "division_of_labour"]
GOOD = {"flourishing", "surviving"}


def load_defs():
    with open(DEFS) as f:
        return json.load(f)


def arm_dir(name, arm, flags=None, screen=False):
    """Arms with no flags are the same world everywhere, so they share one run directory."""
    if screen:
        return os.path.join(LAB, "runs", "_screen", name, arm)
    if flags is not None and not flags:
        return os.path.join(LAB, "runs", "_default")
    return os.path.join(LAB, "runs", name, arm)


def run_arm(name, arm, flags, seeds, ticks, reuse=False, screen=False):
    out = arm_dir(name, arm, flags, screen)
    os.makedirs(out, exist_ok=True)
    if reuse and os.path.exists(os.path.join(out, f"experiment_{seeds.replace('-', '_')}.csv")):
        print("   (reusing", os.path.relpath(out, ROOT) + ")", flush=True)
        return
    for f in os.listdir(out):  # a fresh run, never a mix of old and new seeds
        if f.startswith(("experiment_", "stats_seed", "events_seed", "meta_seed")):
            os.remove(os.path.join(out, f))
    cmd = [SIM, "--seeds", seeds, "--ticks", str(ticks), "--quiet", "--out", out] + flags
    print("  ", " ".join(os.path.relpath(c, ROOT) if c.startswith(ROOT) else c for c in cmd), flush=True)
    log = open(os.path.join(out, "log.txt"), "w")
    subprocess.run(cmd, check=True, stdout=log, stderr=subprocess.STDOUT)


def read_arm(name, arm, flags=None, screen=False):
    out = arm_dir(name, arm, flags, screen)
    files = [f for f in os.listdir(out) if f.startswith("experiment_") and f.endswith(".csv")] if os.path.isdir(out) else []
    rows = []
    for f in sorted(files):
        with open(os.path.join(out, f)) as fh:
            rows.extend(csv.DictReader(fh))
    for r in rows:
        for k in METRICS:
            if k in r:
                try:
                    r[k] = float(r[k])
                except ValueError:
                    r[k] = float("nan")
    return rows


def mean(xs):
    xs = [x for x in xs if x == x]
    return sum(xs) / len(xs) if xs else float("nan")


def median(xs):
    xs = sorted(x for x in xs if x == x)
    if not xs:
        return float("nan")
    m = len(xs) // 2
    return xs[m] if len(xs) % 2 else (xs[m - 1] + xs[m]) / 2


def bootstrap_diff(a, b, n=4000, seed=1):
    """Mean(b) - mean(a) with a percentile 95% interval."""
    rng = random.Random(seed)
    a = [x for x in a if x == x]
    b = [x for x in b if x == x]
    if not a or not b:
        return float("nan"), float("nan"), float("nan")
    diffs = []
    for _ in range(n):
        sa = [rng.choice(a) for _ in a]
        sb = [rng.choice(b) for _ in b]
        diffs.append(sum(sb) / len(sb) - sum(sa) / len(sa))
    diffs.sort()
    return mean(b) - mean(a), diffs[int(0.025 * n)], diffs[int(0.975 * n)]


def fmt(v):
    if v != v:
        return "–"
    return f"{v:.0f}" if abs(v) >= 100 else f"{v:.2f}"


TRAJ = ["pop", "mean_known", "plastic", "signal_mi", "signal_meaning", "things_per_head", "soil_health", "settled_share", "learn_rate", "loudness", "division_of_labour"]


def read_trajectories(name, arm, flags=None, ticks=(2500, 5000, 10000, 15000, 20000)):
    """Median across seeds of a few series at fixed ticks, from stats_seed*.csv."""
    out = arm_dir(name, arm, flags)
    if not os.path.isdir(out):
        return {}
    series = {t: {m: [] for m in TRAJ} for t in ticks}
    for f in sorted(os.listdir(out)):
        if not (f.startswith("stats_seed") and f.endswith(".csv")):
            continue
        with open(os.path.join(out, f)) as fh:
            rows = list(csv.DictReader(fh))
        by_tick = {int(float(r["tick"])): r for r in rows}
        for t in ticks:
            r = by_tick.get(t)
            for m in TRAJ:
                if r is not None and m in r:
                    series[t][m].append(float(r[m]))
                else:
                    series[t][m].append(0.0 if m == "pop" else float("nan"))  # a dead world
    return {t: {m: median(v) for m, v in ms.items()} for t, ms in series.items()}


def report(name, exp):
    arms = list(exp["arms"].keys())
    data = {arm: read_arm(name, arm, exp["arms"][arm]) for arm in arms}
    control = exp.get("control", arms[0])
    lines = [f"# {exp['title']}", "", exp["question"], ""]
    lines.append("## Cách chạy")
    lines.append("")
    for arm in arms:
        lines.append(f"- **{arm}**: `{' '.join(exp['arms'][arm]) or '(mặc định)'}` · {len(data[arm])} thế giới")
    lines.append("")
    lines.append("## Kết cục")
    lines.append("")
    labels = sorted({r["outcome"] for rows in data.values() for r in rows})
    lines.append("| nhánh | " + " | ".join(labels) + " | tốt |")
    lines.append("|---|" + "---|" * (len(labels) + 1))
    for arm in arms:
        counts = {l: sum(1 for r in data[arm] if r["outcome"] == l) for l in labels}
        good = sum(1 for r in data[arm] if r["outcome"] in GOOD)
        lines.append(f"| {arm} | " + " | ".join(str(counts[l]) for l in labels) + f" | {good}/{len(data[arm])} |")
    lines.append("")
    lines.append("## Trung vị mỗi chỉ số")
    lines.append("")
    lines.append("| chỉ số | " + " | ".join(arms) + " |")
    lines.append("|---|" + "---|" * len(arms))
    for m in METRICS:
        if not any(m in r for rows in data.values() for r in rows):
            continue
        lines.append(f"| {m} | " + " | ".join(fmt(median([r.get(m, float('nan')) for r in data[arm]])) for arm in arms) + " |")
    lines.append("")
    for arm in arms:
        if arm == control:
            continue
        lines.append(f"## {arm} so với {control}: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)")
        lines.append("")
        lines.append("| chỉ số | hiệu số | KTC 95% | đọc |")
        lines.append("|---|---|---|---|")
        for m in METRICS:
            a = [r.get(m, float("nan")) for r in data[control]]
            b = [r.get(m, float("nan")) for r in data[arm]]
            d, lo, hi = bootstrap_diff(a, b)
            if d != d:
                continue
            verdict = "khác 0" if (lo > 0 or hi < 0) else "chưa rõ"
            lines.append(f"| {m} | {fmt(d)} | [{fmt(lo)}, {fmt(hi)}] | {verdict} |")
        lines.append("")
    lines.append("## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)")
    lines.append("")
    traj = {arm: read_trajectories(name, arm, exp["arms"][arm]) for arm in arms}
    ticks = sorted(next(iter(traj.values())).keys()) if traj else []
    for m in TRAJ:
        if not ticks:
            break
        lines.append(f"**{m}**")
        lines.append("")
        lines.append("| nhánh | " + " | ".join(f"tick {t}" for t in ticks) + " |")
        lines.append("|---|" + "---|" * len(ticks))
        for arm in arms:
            lines.append(f"| {arm} | " + " | ".join(fmt(traj[arm].get(t, {}).get(m, float("nan"))) for t in ticks) + " |")
        lines.append("")
    lines.append("## Từng thế giới")
    lines.append("")
    lines.append("| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | hiểu (bit) | nghĩa (bit) |")
    lines.append("|---|---|---|---|---|---|---|---|---|---|---|")
    for arm in arms:
        for r in sorted(data[arm], key=lambda r: int(r["seed"])):
            lines.append(f"| {arm} | {r['seed']} | {r['outcome']} | {fmt(r['peak_pop'])} | {fmt(r['final_pop'])} | {fmt(r['mean_known'])} | {fmt(r['soil_health'])} | {fmt(r['settled_share'])} | {fmt(r.get('things_per_head', float('nan')))} | {fmt(r.get('signal_mi', float('nan')))} | {fmt(r.get('signal_meaning', float('nan')))} |")
    lines.append("")
    lines.append("_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._")
    os.makedirs(LAB, exist_ok=True)
    path = os.path.join(LAB, f"{name}.md")
    with open(path, "w") as f:
        f.write("\n".join(lines) + "\n")
    print("report ->", os.path.relpath(path, ROOT))
    return path


def index(defs):
    """One page over every experiment that has a report: arms, good outcomes, and which
    differences against the control came out clearly different from zero."""
    lines = ["# Phòng thí nghiệm: mọi thí nghiệm ở một trang", "",
             "Mỗi dòng là một câu hỏi; mỗi nhánh chỉ khác đối chứng đúng một cờ và chạy trên cùng dãy seed.",
             "\"Tốt\" là số thế giới flourishing hoặc surviving. \"Khác 0\" là các chỉ số mà hiệu số so với đối",
             "chứng có khoảng tin cậy 95% không chứa 0 (dấu là chiều của nhánh so với đối chứng).", "",
             "| thí nghiệm | câu hỏi | nhánh: tốt | khác 0 |", "|---|---|---|---|"]
    for name, exp in defs.items():
        if not os.path.exists(os.path.join(LAB, f"{name}.md")):
            continue
        arms = list(exp["arms"].keys())
        data = {arm: read_arm(name, arm, exp["arms"][arm]) for arm in arms}
        control = exp.get("control", arms[0])
        good = "; ".join(f"{arm} {sum(1 for r in data[arm] if r['outcome'] in GOOD)}/{len(data[arm])}" for arm in arms)
        clear = []
        for arm in arms:
            if arm == control:
                continue
            for m in METRICS:
                a = [r.get(m, float("nan")) for r in data[control]]
                b = [r.get(m, float("nan")) for r in data[arm]]
                d, lo, hi = bootstrap_diff(a, b)
                if d == d and (lo > 0 or hi < 0):
                    clear.append(f"{arm}: {m} {'+' if d > 0 else '−'}{fmt(abs(d))}")
        lines.append(f"| [{name}]({name}.md) | {exp['title']} | {good} | {'; '.join(clear) or '–'} |")
    lines += ["", "_Sinh bởi `python3 tools/lab.py index`. Báo cáo đầy đủ của từng thí nghiệm nằm trong file cùng tên._"]
    path = os.path.join(LAB, "README.md")
    with open(path, "w") as f:
        f.write("\n".join(lines) + "\n")
    print("index ->", os.path.relpath(path, ROOT))


SCREEN_SEEDS = "1-8"
SCREEN_TICKS = 8000


def screen(name, exp, seeds=SCREEN_SEEDS, ticks=SCREEN_TICKS):
    """A cheap first look before committing to a full run.

    Cost is dominated by the late, crowded ticks: over sixteen worlds of the default, the first
    8000 ticks are only 23% of the work of 20000, and half the seeds halve it again. So a screen
    costs about a ninth of a full arm.

    What it can and cannot tell you, measured against the experiments already run in full: where
    an effect is large (crafting, the brains experiment) the ranking of the arms is already the
    final one by tick 2500 to 8000. Where the ranking is still moving at 8000, the full run turned
    out to have no effect worth reporting either. So a screen that separates is worth pursuing,
    and a screen that does not is a reason to stop, not a reason to run longer.
    """
    arms = list(exp["arms"].keys())
    if not os.path.exists(SIM):
        sys.exit("build the sim first: cd sim && cargo build --release")
    print(f"== sàng lọc {name}: {exp['title']}  ({seeds}, {ticks} tick)")
    for arm, flags in exp["arms"].items():
        run_arm(name, arm, flags, seeds, ticks, screen=True)
    data = {arm: read_arm(name, arm, exp["arms"][arm], screen=True) for arm in arms}
    control = exp.get("control", arms[0])
    print()
    print(f"{'nhánh':<18}{'sống':>7}{'tốt':>7}{'dân đỉnh':>11}{'biết':>9}{'đồ/người':>11}{'phát minh':>11}")
    for arm in arms:
        rows = data[arm]
        if not rows:
            continue
        alive = sum(1 for r in rows if float(r["final_pop"]) > 0)
        good = sum(1 for r in rows if r["outcome"] in GOOD)
        med = lambda k: median([r.get(k, float("nan")) for r in rows])
        print(f"{arm:<18}{alive:>4}/{len(rows):<2}{good:>4}/{len(rows):<2}"
              f"{fmt(med('peak_pop')):>11}{fmt(med('mean_known')):>9}{fmt(med('things_per_head')):>11}{fmt(med('innovations')):>11}")
    print()
    for arm in arms:
        if arm == control:
            continue
        clear = []
        for m in METRICS:
            a = [r.get(m, float("nan")) for r in data[control]]
            b = [r.get(m, float("nan")) for r in data[arm]]
            d, lo, hi = bootstrap_diff(a, b)
            if d == d and (lo > 0 or hi < 0):
                clear.append(f"{m} {'+' if d > 0 else '−'}{fmt(abs(d))}")
        print(f"{arm} so với {control}: " + ("; ".join(clear) if clear else "chưa tách được nhánh nào"))
    print("\nTách được thì chạy đầy đủ: python3 tools/lab.py run " + name + " --seeds 1-16")


def main():
    if len(sys.argv) < 2 or sys.argv[1] not in ("list", "run", "report", "index", "screen"):
        sys.exit(__doc__)
    defs = load_defs()
    cmd = sys.argv[1]
    if cmd == "list":
        for name, exp in defs.items():
            print(f"{name:<14} {exp['title']}  [{', '.join(exp['arms'])}]")
        return
    if cmd == "index":
        index(defs)
        return
    names = list(defs) if sys.argv[2] == "all" else [sys.argv[2]]
    seeds = "1-8"
    ticks = 20000
    reuse = False
    args = sys.argv[3:]
    while args:
        k = args.pop(0)
        if k == "--seeds":
            seeds = args.pop(0)
        elif k == "--ticks":
            ticks = int(args.pop(0))
        elif k == "--reuse":
            reuse = True
    for name in names:
        exp = defs[name]
        if cmd == "screen":
            screen(name, exp, seeds, SCREEN_TICKS if ticks == 20000 else ticks)
            continue
        if cmd == "run":
            if not os.path.exists(SIM):
                sys.exit("build the sim first: cd sim && cargo build --release")
            print(f"== {name}: {exp['title']}")
            for arm, flags in exp["arms"].items():
                run_arm(name, arm, flags, seeds, ticks, reuse)
        report(name, exp)


if __name__ == "__main__":
    main()
