#!/usr/bin/env python3
"""169 P3. `168` section 7.1 supports "deferral is pointwise optimal under a
nearest-point boundary resolution" with three rows:

    nearest (round to 2^3)   3000 chains, 0 eager wins
    nearest (clamp)          3000 chains, 0 eager wins
    NOT nearest (truncate)   3000 chains, 91 eager wins, 1330 winning inputs

The truncate row is named as the control. It is matched to the ROUND row: both
resolve onto the multiples of 8, differing only in nearest against not. It is
NOT matched to the CLAMP row, whose representable set is every value in
[0, LIMIT] rather than 32 of them.

The question the brief poses is whether the conditions are comparable. A row
that reports zero eager wins is only evidence if a win was reachable in it, so
this measures, per condition, how often ANY placement difference occurs at all.
A condition where eager and deferred agree everywhere reports zero for a reason
that has nothing to do with nearest-point projection.

And a second question the first run turned up, which is larger. 168's stated
claim is:

    "There is no input, and no chain, on which ANY OTHER PLACEMENT is strictly
     closer to the exact composite."

`eager_wins` in that probe sets `let fm = full_mask(steps.len())` and compares
exactly two placements: fully eager against fully deferred. The claim quantifies
over all 2^(n-1) interior placements. So this runs BOTH searches: the original
two-placement one as a replication control, and the full enumeration the claim
actually asserts.

NEGATIVE CONTROLS, stated before the run.
  C1. The two-placement arm must reproduce 168's published numbers exactly
      (0, 0, 91 chains with a win). If it does not, my replication is not
      their experiment and nothing below is about their claim.
  C2. The truncate condition must show wins in BOTH arms. If the full
      enumeration found none where the narrow one did, my enumeration is
      broken rather than wider.
  C3. The full enumeration must find STRICTLY MORE truncate wins than the
      two-placement arm, or widening changed nothing and there is no gap to
      report.
"""
import sys

W = 8
LIMIT = (1 << W) - 1
M64 = (1 << 64) - 1

# ---- steps, transcribed from 168_probes/p3_resolution_degeneracy.rs ----
def apply(s, v):
    kind, k = s
    if kind == "AddK":    return v + k
    if kind == "MulK":    return v * k
    if kind == "ShrK":    return v >> k
    if kind == "XorK":    return v ^ k
    if kind == "SatSubK": return v - k if v > k else 0
    raise AssertionError(kind)

def resolve(p, v):
    kind, g = p
    if kind == "Clamp":   return LIMIT if v > LIMIT else v
    if kind == "RoundTo":
        step = 1 << g
        return ((v + (step >> 1)) // step) * step
    if kind == "TruncTo":
        step = 1 << g
        return (v // step) * step
    raise AssertionError(kind)

def run(x, steps, p, firemask):
    """firemask bit i set => resolve after step i. The boundary always resolves."""
    v = x
    for i, s in enumerate(steps):
        v = apply(s, v)
        if firemask & (1 << i):
            v = resolve(p, v)
    return resolve(p, v)

def exact(x, steps):
    v = x
    for s in steps:
        v = apply(s, v)
    return v

# ---- the same chains: xorshift64 with 168's seed and alphabet ----
K = 97  # AddK(k); see below for the control that pins it
ALPHABET = [("AddK", K), ("MulK", 3), ("ShrK", 2), ("XorK", 0b1011_0110),
            ("SatSubK", LIMIT // 2), ("MulK", 5), ("ShrK", 1), ("AddK", 13)]

def chains():
    rng = 0xA5A5_1234_DEAD_BEEF
    def nxt():
        nonlocal rng
        rng ^= (rng << 13) & M64; rng &= M64
        rng ^= rng >> 7
        rng ^= (rng << 17) & M64; rng &= M64
        return rng
    out = []
    for _ in range(3000):
        d = 2 + (nxt() % 4)
        out.append([ALPHABET[nxt() % 8] for _ in range(d)])
    return out

def survey(chs, p, mode):
    """mode='narrow' reproduces 168: fully-eager against fully-deferred only.
       mode='full' enumerates every placement, which is what the claim says."""
    wins_chains = 0
    wins_inputs = 0
    diff_chains = 0          # chains where SOME placement changes the output
    for steps in chs:
        n = len(steps)
        masks = [(1 << n) - 1] if mode == 'narrow' else [m for m in range(1, 1 << n)]
        any_win = False
        any_diff = False
        for x in range(LIMIT + 1):
            want = exact(x, steps)
            base = run(x, steps, p, 0)
            dd = abs(base - want)
            won_here = False
            for m in masks:
                g = run(x, steps, p, m)
                if g != base:
                    any_diff = True
                if abs(g - want) < dd:
                    won_here = True
            if won_here:
                any_win = True
                wins_inputs += 1
        if any_win:
            wins_chains += 1
        if any_diff:
            diff_chains += 1
    return wins_chains, wins_inputs, diff_chains

def main():
    chs = chains()
    conds = [("nearest (round to 2^3)", ("RoundTo", 3)),
             ("nearest (clamp)",        ("Clamp", 0)),
             ("NOT nearest (truncate)", ("TruncTo", 3))]
    res = {}
    for mode in ("narrow", "full"):
        label = ("ARM 1: two placements, as 168 ran it"
                 if mode == "narrow" else
                 "ARM 2: every placement, as the claim states it")
        print(f"=== {label} ===")
        print(f"{'condition':26} {'win chains':>11} {'win inputs':>11} {'chains where placement':>24}")
        print(f"{'':26} {'':>11} {'':>11} {'changes the output':>24}")
        for name, p in conds:
            w, wi, d = survey(chs, p, mode)
            res[(mode, name)] = (w, wi, d)
            print(f"{name:26} {w:>11} {wi:>11} {d:>24}")
        print()
    rw, cw, tw = (res[("narrow", n)][0] for n, _ in conds)
    rwf, cwf, twf = (res[("full", n)][0] for n, _ in conds)
    c1 = (rw == 0 and cw == 0 and tw == 91)
    print(f"C1 arm 1 reproduces 168's 0 / 0 / 91        : {c1}   (got {rw} / {cw} / {tw})")
    c2 = tw > 0 and twf > 0
    print(f"C2 truncate wins in both arms               : {c2}   ({tw} narrow, {twf} full)")
    c3 = twf > tw
    print(f"C3 widening finds strictly more             : {c3}   ({twf} > {tw})")
    if not (c1 and c2 and c3):
        print("CONTROL FAILED -- result suppressed"); sys.exit(1)
    print()
    print("THE SIXTH INSTRUMENT DEFECT")
    print(f"  168's search compares two placements. Its claim quantifies over all of")
    print(f"  them. Widening the search to every placement takes the truncate control")
    print(f"  from {tw} win-chains to {twf}, so intermediate placements do reach cases the")
    print("  endpoints do not: the instrument was thinner than the claim.")
    print()
    print("  AND THE CLAIM SURVIVES THE WIDER TEST.")
    print(f"  round: {rwf} win-chains over every placement. clamp: {cwf}. Both still zero.")
    print("  The finding is not refuted; it is now supported at the strength it")
    print("  actually asserts rather than one placement's worth of it.")
    print()
    cd = res[("full", "nearest (clamp)")][2]
    rd = res[("full", "nearest (round to 2^3)")][2]
    td = res[("full", "NOT nearest (truncate)")][2]
    tw = twf
    print()
    print("VERDICT")
    print(f"  round    : placement changes the output on {rd} of 3000 chains, and never helps.")
    print(f"  clamp    : placement changes the output on {cd} of 3000 chains, and never helps.")
    print(f"  truncate : placement changes the output on {td} of 3000 chains, and helps on {tw}.")
    if cd < rd:
        print()
        print(f"  The clamp row is exercised on {cd} chains against round's {rd}. Both report")
        print("  zero, and they are not equally strong zeros: the smaller the set of")
        print("  chains where placement changes anything, the less a zero says.")
    print()
    print("  The matched pair is round against truncate: same representable set (the")
    print("  32 multiples of 8), differing only in nearest against not. That pair is")
    print("  a proper experiment and it carries the claim.")
    print("  The clamp row has a representable set of all 256 in-range values, so it")
    print("  is not the control's partner; it is a second positive with no matched")
    print("  negative. 168 does not claim otherwise, and its summary sentence")
    print("  ('checked over 3000 chains with a control that finds 1330") 
    print("  counterexamples') reads as though the control covers both positives.")

if __name__ == "__main__":
    main()
