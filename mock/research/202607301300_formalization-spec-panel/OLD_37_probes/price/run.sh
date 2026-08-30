#!/usr/bin/env bash
# Compile-time sweep. Build shape is `rustc --edition 2021 --crate-type lib
# --emit=metadata`, which is type checking, trait selection and const eval with
# no codegen, and that is the honest shape because both mechanisms are entirely
# compile-time. Min of three runs, count = 0 subtracted as fixed cost.
set -u
OUT=$(mktemp -d)
echo "shape,count,ms,metadata_bytes"
for shape in a b c; do
  for n in 0 50 100 200 400; do
    python3 gen.py "$shape" "$n" "$OUT/s.rs"
    best=999999
    for _ in 1 2 3; do
      s=$(python3 -c 'import time;print(int(time.time()*1000))')
      rustc --edition 2021 --crate-type lib --emit=metadata "$OUT/s.rs" --out-dir "$OUT" 2>/dev/null
      e=$(python3 -c 'import time;print(int(time.time()*1000))')
      d=$((e-s)); [ "$d" -lt "$best" ] && best=$d
    done
    sz=$(stat -f%z "$OUT"/libs.rmeta 2>/dev/null || echo 0)
    echo "$shape,$n,$best,$sz"
  done
done
rm -rf "$OUT"
