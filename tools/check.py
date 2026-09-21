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
import hashlib
import json
import os
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SIM = os.path.join(ROOT, "sim", "target", "release", "sim")
REF = os.path.join(ROOT, "tools", "fingerprints.json")
WORK = os.path.join(ROOT, "sim", "target", "check")
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
        with open(path, "rb") as f:
            h.update(f.read())
    return h.hexdigest()[:16]


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
    ref = json.load(open(REF)) if os.path.exists(REF) else {}
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
    for b in broken:
        print("HỎNG  ", b)
    for m in moved:
        print("ĐỔI   ", m)
    if bless:
        json.dump(now, open(REF, "w"), indent=2, sort_keys=True)
        print("đã ghi lại làm mốc:", os.path.relpath(REF, ROOT))
        return
    if broken:
        sys.exit(1)
    if moved:
        print("\nCác thế giới trên đã đổi hành vi. Nếu là cố ý: python3 tools/check.py --bless")
        sys.exit(2)
    print("\nKhông có gì đổi.")


if __name__ == "__main__":
    main()
