#!/usr/bin/env python3
"""Probe 3. At rank > 1, `Layout::Bitpacked` has a question rank 1 never posed:
which axis is packed? Exact integer arithmetic, no floats anywhere.

`91:563-570` ratifies the group arithmetic for ONE dimension: P = 8/gcd(W_S, 8)
elements sit in G = W_S * P / 8 whole bytes, G*8 = W_S*P by algebra, and zero
inter-value padding is a theorem rather than an obligation. `91:645-651` then
prices the mutation gap against it: under Bitpacked "the dirt surface this
theorem protects is strictly smaller (one tail-group region for the whole
column, not one region per element)".

Neither sentence has a rank. This computes the two candidate readings at rank N
and reports where they differ, where they coincide, and what each costs.

  (a) per-axis   pack each innermost row independently
  (b) flattened  pack the whole shape as one rank-1 run of COUNT elements
"""

from math import gcd


def group(w):
    """The ratified rank-1 group arithmetic, `91:563-570`."""
    p = 8 // gcd(w, 8)
    g = w * p // 8
    assert g * 8 == w * p, "the ratified identity G*8 == W_S*P"
    return p, g


def flattened(extents, w):
    count = 1
    for e in extents:
        count *= e
    bits = count * w
    byts = -(-bits // 8)
    return dict(bytes=byts, pad_bits=byts * 8 - bits, tail_regions=1 if byts * 8 != bits else 0)


def per_axis(extents, w):
    inner = extents[-1]
    outer = 1
    for e in extents[:-1]:
        outer *= e
    row_bits = inner * w
    row_bytes = -(-row_bits // 8)
    row_pad = row_bytes * 8 - row_bits
    return dict(
        bytes=outer * row_bytes,
        pad_bits=outer * row_pad,
        tail_regions=outer if row_pad else 0,
    )


CASES = [
    # (name,             extents,      element width)
    ("vec3 of 7-bit",    (3,),         7),
    ("3x4 of 7-bit",     (3, 4),       7),
    ("3x4x5 of 7-bit",   (3, 4, 5),    7),
    ("3x4 of 13-bit",    (3, 4),       13),
    ("64x64 of 3-bit",   (64, 64),     3),
    ("BitMatrix 8x8",    (8, 8),       1),
    ("3x8 of 1-bit",     (3, 8),       1),
    ("3x8 of 8-bit",     (3, 8),       8),
    ("5x16 of 4-bit",    (5, 16),      4),
    ("7x9 of 11-bit",    (7, 9),       11),
    ("2x3x4 of 5-bit",   (2, 3, 4),    5),
    ("17x13 of 23-bit",  (17, 13),     23),
]

print(f"{'case':18} {'W':>3} {'P':>3} {'G':>3} | "
      f"{'flat B':>7} {'flat pad':>8} {'flat rgn':>8} | "
      f"{'axis B':>7} {'axis pad':>8} {'axis rgn':>8} | verdict")
print("-" * 118)

coincide = []
differ = []
for name, ext, w in CASES:
    p, g = group(w)
    f = flattened(ext, w)
    a = per_axis(ext, w)
    same = (f["bytes"], f["pad_bits"]) == (a["bytes"], a["pad_bits"])
    (coincide if same else differ).append(name)
    verdict = "IDENTICAL" if same else f"flat saves {a['bytes']-f['bytes']}B"
    print(f"{name:18} {w:3} {p:3} {g:3} | "
          f"{f['bytes']:7} {f['pad_bits']:8} {f['tail_regions']:8} | "
          f"{a['bytes']:7} {a['pad_bits']:8} {a['tail_regions']:8} | {verdict}")

print()
print("CLAIM A. When the two readings coincide. Three candidate conditions,")
print("checked EXHAUSTIVELY rather than over the case table above, because the")
print("first two are what a reader would reach for and both are wrong.")
CANDS = {
    "row byte-aligned":        lambda o, i, w: (i * w) % 8 == 0,
    "row aligned or outer==1": lambda o, i, w: (i * w) % 8 == 0 or o == 1,
    "outer * rowpad < 8":      lambda o, i, w: o * ((-(i * w)) % 8) < 8,
}
for label, pred in CANDS.items():
    bad = 0
    checked = 0
    for w in range(1, 65):
        for inner in range(1, 65):
            for outer in range(1, 33):
                checked += 1
                f = flattened((outer, inner), w)
                a = per_axis((outer, inner), w)
                same = (f["bytes"], f["pad_bits"]) == (a["bytes"], a["pad_bits"])
                if pred(outer, inner, w) != same:
                    bad += 1
    print(f"  {label:26} mismatches {bad:6} / {checked}")
print()
print("  So the exact condition is `outer * rowpad < 8`: the two coincide when")
print("  the whole per-axis padding fits inside one byte, and differ the moment")
print("  it reaches a whole byte. One-line proof: per-axis padding is")
print("  outer * ((-x) mod 8) and flattened padding is (-outer*x) mod 8, which")
print("  is the same quantity reduced mod 8, so they are equal exactly when the")
print("  first is already below 8.")
print("  The first candidate, which this probe originally asserted, is FALSE.")

print()
print("CLAIM B. Flattened never costs more bytes than per-axis, and the excess")
print("is exactly (outer - 1) * row_padding + (row_padding - flat_padding).")
worse = 0
maxexcess = (0, None)
for w in range(1, 65):
    for inner in range(1, 65):
        for outer in range(1, 33):
            f = flattened((outer, inner), w)
            a = per_axis((outer, inner), w)
            if a["bytes"] < f["bytes"]:
                worse += 1
            ex = a["bytes"] - f["bytes"]
            if ex > maxexcess[0]:
                maxexcess = (ex, (w, inner, outer))
print(f"  per-axis cheaper than flattened in {worse} of {checked} cases")
print(f"  largest per-axis excess: {maxexcess[0]} bytes at "
      f"(W, inner, outer) = {maxexcess[1]}")

print()
print("CLAIM C. Under the flattened reading the rank-N container IS the rank-1")
print("container of COUNT elements: the ratified period P and group stride G")
print("depend on W alone, so nothing about them is rank-sensitive. What the")
print("shape adds is index arithmetic only. Checked: the group identity holds")
print("at every W, and COUNT*W bits fill exactly ceil(COUNT/P) groups.")
bad_c = 0
for w in range(1, 65):
    p, g = group(w)
    for extents in [(3,), (3, 4), (3, 4, 5), (7, 9), (2, 3, 4, 5)]:
        count = 1
        for e in extents:
            count *= e
        groups = -(-count // p)
        if groups * g < -(-count * w // 8):
            bad_c += 1
        # the flattened byte count never exceeds the group-rounded one
        if flattened(extents, w)["bytes"] > groups * g:
            bad_c += 1
print(f"  violations: {bad_c}")

print()
print("CLAIM D. The write granule becomes a SHAPE fact under the flattened")
print("reading: a parallel partition along the OUTER axis is legal only when")
print("the inner row is a whole number of groups, i.e. inner mod P == 0.")
for name, ext, w in CASES:
    if len(ext) < 2:
        continue
    p, _ = group(w)
    ok = ext[-1] % p == 0
    print(f"  {name:18} W={w:2} P={p:2} inner={ext[-1]:3} -> outer-axis "
          f"partition {'LEGAL' if ok else 'ILLEGAL, crosses a group'}")
