#!/usr/bin/env python3
"""Generator for the identity-side monomorphisation sweep, mirroring
`08_probes/i_gen_monomorphisation_sweep.py`'s exact methodology so the
resulting numbers are directly comparable to the policy/lowering side's
already-measured 5.2ms/composition, zero-symbols result.

Sweeps K distinct `Number<FixNumeral<...>, S>` compositions across width,
sign domain, sign indexing and lowering (Hot vs Cold), calling `add`
through the full settled identity contract (Numeral incl. Radix,
Precision, Exponent nesting Adjustment/Bias, Domain; Encoding nested in
Lowering incl. SignIndexing, Fields, Canonical) for each.
"""
import sys

k = int(sys.argv[1])
out = sys.argv[2]

L = [
    "use identity_model::*;",
    "use core::hint::black_box;",
    "fn main() {",
]

domains = ["NonNegative", "Symmetric", "AsymmetricLow"]
sign_idx = ["UnsignedIdx", "TwosComplement", "SignMagnitude", "OnesComplement"]
lowerings = ["HotLowering", "ColdLowering"]

i = 0
for w in range(1, 400):
    for d in domains:
        for si in sign_idx:
            for lw in lowerings:
                if i >= k:
                    break
                intw = w % 60 + 1
                fracw = w % 13
                pbits = intw + fracw
                negf = -(fracw)
                numeral = f"FixNumeral<{intw}, {fracw}, {pbits}, {negf}, {d}>"
                lowering = f"{lw}<{si}>"
                L.append(
                    f"    black_box(add::<{numeral}, {lowering}>("
                    f"black_box(3i64), black_box(4i64)));"
                )
                i += 1
            if i >= k:
                break
        if i >= k:
            break
    if i >= k:
        break

L.append("}")
open(out, "w").write("\n".join(L) + "\n")
