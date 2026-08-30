#!/usr/bin/env bash
# Generates the three negative cases from p4_crossing_contract.rs by enabling one commented
# block each, compiles every one on the pin, and commits both the generated source and the
# transcript so a later reader can diff rather than trust.
set -u
cd "$(dirname "$0")"
TC=+nightly-2026-05-28
mkdir -p out

gen () {
  local tag="$1" marker="$2"
  python3 - "$marker" > "p4_${tag}.rs" <<'PY'
import sys
marker = sys.argv[1]
lines = open("p4_crossing_contract.rs").read().splitlines()
out, live = [], False
for ln in lines:
    if ln.startswith(f"// {marker}."):
        live = True
        out.append(ln)
        continue
    if live and ln.startswith("// pub const"):
        out.append(ln[3:]); continue
    if live and ln.startswith("//     <Step"):
        out.append(ln[3:]); live = False; continue
    out.append(ln)
print("\n".join(out))
PY
  rustc $TC --edition 2024 --crate-type lib "p4_${tag}.rs" --out-dir out > "p4_${tag}.stderr" 2>&1
  echo "$tag EXIT=$?"
}

gen n1 N1
gen n2 N2
gen n3 N3

echo
for t in n1 n2 n3; do
  echo "===== p4_${t}.stderr ====="
  head -20 "p4_${t}.stderr"
  echo
done
