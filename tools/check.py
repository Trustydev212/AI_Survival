#!/usr/bin/env python3
"""The fast check: does the code still work, and did the worlds change?

Experiments cost a quarter of an hour an arm, so a mistake found by an experiment is a mistake
found far too late. This runs in a few seconds and answers the two questions that actually go
wrong: did anything break, and did a change alter behaviour somewhere it was not meant to.

    python3 tools/check.py          # run the checks
    python3 tools/check.py --bless  # accept the current behaviour as the new reference

Each configuration runs a small world and its whole statistics table is hashed. Because the sim
is deterministic, an unchanged fingerprint proves the worlds are identical down to the last
decimal; a changed one names the configuration that moved. Change behaviour on purpose, bless it,
and the commit then carries the record of exactly which worlds moved.
"""
import csv
import hashlib
import json
import os
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SIM = os.environ.get("AISV_SIM") or os.path.join(ROOT, "sim", "target", "release", "sim")
REF = os.path.join(ROOT, "tools", "fingerprints.json")
WORK = os.path.join(ROOT, "sim", "target", "check")
# Hashing the whole statistics file confused two different things: adding a column changed the
# fingerprint of every world although not one of them had behaved differently, and a run of false
# alarms is how a real one gets ignored. These columns are behaviour; the rest is reporting.
WATCHED = ["tick", "pop", "mean_energy", "food", "soil_health", "innovations", "mean_known",
           "births", "starved", "killed", "plague_deaths", "cultivated_cells", "settled_share"]
SMALL = ["--width", "96", "--height", "96", "--agents", "400", "--ticks", "4000", "--quiet"]

CONFIGS = {
    "default": [],
    "gradient": ["--gradient"],
    "gradient-social": ["--gradient", "--know-rate", "0.3"],
    "no-crafting": ["--no-crafting"],
    "no-orders": ["--no-orders"],
    "deaf": ["--hear-scale", "0"],
    "herds": ["--herd-density", "3.5"],
}
SEEDS = [1, 4]


def fingerprint(name, flags):
    h = hashlib.sha256()
    for seed in SEEDS:
        out = os.path.join(WORK, name, str(seed))
        shutil.rmtree(out, ignore_errors=True)
        os.makedirs(out, exist_ok=True)
        r = subprocess.run([SIM, "--seed", str(seed), "--out", out] + SMALL + flags, capture_output=True, text=True)
        if r.returncode != 0:
            return f"CRASH: {(r.stderr or r.stdout).strip().splitlines()[-1][:120]}"
        path = os.path.join(out, f"stats_seed{seed}.csv")
        if not os.path.exists(path):
            return "NO OUTPUT"
        with open(path) as f:
            rows = list(csv.DictReader(f))
        for r in rows:
            h.update("|".join(r.get(c, "") for c in WATCHED).encode())
    return h.hexdigest()[:16]


def world_version():
    """The rules version the built sim carries, read straight from its own header."""
    r = subprocess.run([SIM, "--seed", "1", "--ticks", "1", "--quiet", "--out", WORK], capture_output=True, text=True)
    for word in (r.stderr + r.stdout).split():
        if word.startswith("world=v"):
            return int(word[7:])
    return 0


def continuity():
    """A world put down and picked up must carry on as if nothing happened.

    Getting this right took four attempts: the random stream was being nudged on the way back in,
    the recipe table was left behind so known things looked new, the id counter restarted, and the
    map of where the good land is only rebuilds on a schedule so it came back blank. Each one made
    the world drift instead of continue, and none of them was visible without this comparison.
    """
    a, b, c = (os.path.join(WORK, k) for k in ("cont_a", "cont_b", "cont_c"))
    for d in (a, b, c):
        shutil.rmtree(d, ignore_errors=True)
        os.makedirs(d, exist_ok=True)
    state = os.path.join(WORK, "cont.bin")
    small = ["--width", "96", "--height", "96", "--agents", "400", "--quiet"]
    runs = [
        [SIM, "--seed", "5", "--ticks", "3000", "--out", a],
        [SIM, "--seed", "5", "--ticks", "1500", "--save", state, "--out", b],
        [SIM, "--seed", "5", "--ticks", "1500", "--load", state, "--out", c],
    ]
    for cmd in runs:
        r = subprocess.run(cmd + small, capture_output=True, text=True)
        if r.returncode != 0:
            return f"CRASH: {(r.stderr or r.stdout).strip().splitlines()[-1][:120]}"
    rows = {}
    for d in (a, c):
        with open(os.path.join(d, "stats_seed5.csv")) as fh:
            rows[d] = {r["tick"]: r for r in csv.DictReader(fh)}
    for tick, row in rows[c].items():
        straight = rows[a].get(tick)
        if straight and (straight["pop"] != row["pop"] or straight["mean_known"] != row["mean_known"]):
            return f"lệch tại tick {tick}: dân {straight['pop']} -> {row['pop']}"
    return "ok"


def main():
    bless = "--bless" in sys.argv
    if not os.path.exists(SIM):
        sys.exit("build the sim first: cd sim && cargo build --release")
    print("unit tests ...", flush=True)
    r = subprocess.run(["cargo", "test", "--release", "--quiet"], cwd=os.path.join(ROOT, "sim"), capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stdout[-3000:])
        sys.exit("unit tests failed")
    print("  ok")
    print("nối lại thế giới ...", flush=True)
    cont = continuity()
    print("  " + cont)
    world = world_version()
    stored = json.load(open(REF)) if os.path.exists(REF) else {}
    ref = stored.get("fingerprints", {})
    was_world = stored.get("world")
    if was_world is not None and was_world != world:
        print(f"thế giới đã lên v{was_world} -> v{world}: mọi vân tay đổi là điều đương nhiên")
    now = {name: fingerprint(name, flags) for name, flags in CONFIGS.items()}
    moved, broken = [], []
    for name, fp in now.items():
        was = ref.get(name)
        if fp.startswith(("CRASH", "NO OUTPUT")):
            broken.append(f"{name}: {fp}")
        elif was is None:
            print(f"  {name:<16} {fp}  (mới)")
        elif was != fp:
            moved.append(f"{name}: {was} -> {fp}")
        else:
            print(f"  {name:<16} {fp}  giữ nguyên")
    shutil.rmtree(WORK, ignore_errors=True)
    if cont != "ok":
        broken.append("nối lại thế giới: " + cont)
    for b in broken:
        print("HỎNG  ", b)
    for m in moved:
        print("ĐỔI   ", m)
    if bless:
        json.dump({"world": world, "fingerprints": now}, open(REF, "w"), indent=2, sort_keys=True)
        print("đã ghi lại làm mốc:", os.path.relpath(REF, ROOT))
        return
    if broken:
        sys.exit(1)
    if moved:
        print("\nCác thế giới trên đã đổi hành vi.")
        if was_world == world:
            print(f"Hành vi đổi mà phiên bản thế giới vẫn là v{world}. Tăng WORLD trong sim/src/version.rs,")
            print("ghi một dòng vào NOTE và HISTORY, rồi: python3 tools/check.py --bless")
        else:
            print("Nếu là cố ý: python3 tools/check.py --bless")
        sys.exit(2)
    print("\nKhông có gì đổi.")


if __name__ == "__main__":
    main()
