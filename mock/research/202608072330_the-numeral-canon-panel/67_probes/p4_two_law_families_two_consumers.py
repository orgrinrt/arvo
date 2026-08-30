#!/usr/bin/env python3
"""p4. The two law families are two consumer classes, and one cell serves both.

`63` section 3.4 records that a total reduction carries two independent law
families: the ADAPTATION laws (monotone, distance-minimising), which "face the
source", and the COHERENCE law, which "faces the target". All four combinations
are inhabited, so neither subsumes the other.

What nobody has said is WHO NEEDS WHICH. This probe answers that, and extends
`57b`'s H1/H2 frame to a third operation, which `63` section 9 lists among the
experts' own outstanding work ("Subtraction, shifts and mixed-operation chains
against absorption and the congruence conditions").

The third operation is `min`, and it matters because the algorithm crates op
names as arvo's selling point (I11) are semiring computations over the TROPICAL
semiring: min as the additive operation, the numeral's own + as the
multiplicative one. `35` measured this from the algorithm side (a wrapping
min-plus fold returns wrong answers on 45.4% and 48.9% of instances at two
widths; saturating returns 0). This probe derives the same fact from the
reduction's laws instead, which is a mechanism rather than a measurement.

The derivation, stated before the run so the probe can refute it:

  min is CLOSED on Q, so it needs no adaptation at all and its ambient laws are
  inherited unconditionally. H1 and H2 are vacuous for it.

  The tropical structure needs + to distribute over min:
      rho(min(a,b) + c) == min(rho(a+c), rho(b+c))
  In Z, min(a,b) + c == min(a+c, b+c) exactly (translation invariance of the
  order). So the left side is rho(min(u,v)) and the right is min(rho u, rho v),
  and distributivity holds for all a,b,c EXACTLY WHEN rho commutes with binary
  min on the reachable set, which is exactly when rho is MONOTONE there.

  So: the tropical consumer needs the ADAPTATION laws and does not care about
  coherence. The reassociating-fold consumer needs COHERENCE and does not care
  about monotonicity. They are the two law families, and the two-by-two says
  no policy is forced to serve both.

Exhaustive at the 4-bit model width, both sign domains, three reductions
including `56`'s opposite-bound mutant as the neither-cell control.

Run: python3 p4_two_law_families_two_consumers.py
"""

out = []


def say(s=""):
    out.append(s)
    print(s)


def wrap(lo, hi):
    n = hi - lo + 1
    return lambda x: (x - lo) % n + lo


def sat(lo, hi):
    return lambda x: lo if x < lo else (hi if x > hi else x)


def mutant(lo, hi):
    """`56`'s opposite-bound mutant: clamp to the wrong end."""
    return lambda x: hi if x < lo else (lo if x > hi else x)


WINDOWS = {"unsigned [0,15]": (0, 15), "signed [-8,7]": (-8, 7)}
POLICIES = {"wrap": wrap, "saturate": sat, "opposite-bound mutant": mutant}

say("p4. two law families, two consumer classes, exhaustive at 4 bits")
say("=" * 76)

rows = []

for wname, (lo, hi) in WINDOWS.items():
    Q = list(range(lo, hi + 1))
    # the reachable exact set for addition of two representable values
    reach_add = list(range(2 * lo, 2 * hi + 1))

    for pname, mk in POLICIES.items():
        rho = mk(lo, hi)

        # --- adaptation law: monotone on the reachable set
        mono_bad = sum(
            1
            for i, x in enumerate(reach_add)
            for y in reach_add[i:]
            if not (rho(x) <= rho(y))
        )

        # --- min is closed on Q: how many min results need adapting at all
        min_needs_adapt = sum(1 for a in Q for b in Q if min(a, b) not in Q)

        # --- tropical distributivity: + over min
        trop_bad = sum(
            1
            for a in Q
            for b in Q
            for c in Q
            if rho(min(a, b) + c) != min(rho(a + c), rho(b + c))
        )
        trop_tot = len(Q) ** 3

        # --- induced additive associativity (the reassociating-fold consumer)
        add_bad = sum(
            1
            for a in Q
            for b in Q
            for c in Q
            if rho(rho(a + b) + c) != rho(a + rho(b + c))
        )

        # --- induced multiplicative associativity, for the cube's sake
        mul_bad = sum(
            1
            for a in Q
            for b in Q
            for c in Q
            if rho(rho(a * b) * c) != rho(a * rho(b * c))
        )

        rows.append(
            dict(
                window=wname,
                policy=pname,
                mono_bad=mono_bad,
                min_needs_adapt=min_needs_adapt,
                trop_bad=trop_bad,
                trop_tot=trop_tot,
                add_bad=add_bad,
                mul_bad=mul_bad,
            )
        )

say()
say("Per cell. 'mono' is violations of monotonicity over the reachable set for")
say("addition; 'trop' is violations of + distributing over min; 'add' and 'mul'")
say("are violations of associativity of the induced operation. All over Q^3.")
say()
hdr = f"{'window':16s} {'policy':22s} {'mono':>6s} {'trop':>7s} {'add':>7s} {'mul':>7s}"
say(hdr)
say("-" * len(hdr))
for r in rows:
    say(
        f"{r['window']:16s} {r['policy']:22s} {r['mono_bad']:6d} "
        f"{r['trop_bad']:7d} {r['add_bad']:7d} {r['mul_bad']:7d}"
    )

say()
say("min is closed on Q in every cell (adaptations needed for min over Q^2): "
    + ", ".join(f"{r['policy'][:4]}/{r['window'][:4]}={r['min_needs_adapt']}" for r in rows))

# ---------------------------------------------------------------------------
say()
say("=" * 76)
say("THE DERIVED CLAIM, checked rather than asserted:")
say("  tropical distributivity holds  <==>  the reduction is monotone")
say()
ok = all((r["trop_bad"] == 0) == (r["mono_bad"] == 0) for r in rows)
say(f"  biconditional over all {len(rows)} cells: {ok}")
for r in rows:
    say(
        f"    {r['window']:16s} {r['policy']:22s} monotone={r['mono_bad'] == 0!s:5s} "
        f"distributes={r['trop_bad'] == 0!s:5s}"
    )
say()
say("  and the two hypotheses observed both true and false across the cells, so")
say("  the biconditional is not holding by accident:")
say(f"    monotone true in {sum(1 for r in rows if r['mono_bad'] == 0)} of {len(rows)} cells, "
    f"false in {sum(1 for r in rows if r['mono_bad'] != 0)}")

# ---------------------------------------------------------------------------
say()
say("=" * 76)
say("THE TWO CONSUMERS, per cell.")
say("  TROPICAL consumer  (shortest path, min-plus, widest-path, any semiring")
say("                      computation whose additive operation is the order):")
say("                      needs the ADAPTATION laws. Coherence is irrelevant.")
say("  REASSOCIATING fold (splitting, threading, algebraic rewriting):")
say("                      needs COHERENCE. Monotonicity is irrelevant.")
say()
hdr2 = f"{'window':16s} {'policy':22s} {'tropical ok':>12s} {'fold ok':>9s} {'both':>6s}"
say(hdr2)
say("-" * len(hdr2))
both_cells = []
for r in rows:
    trop_ok = r["trop_bad"] == 0
    fold_ok = r["add_bad"] == 0
    if trop_ok and fold_ok:
        both_cells.append((r["window"], r["policy"]))
    say(
        f"{r['window']:16s} {r['policy']:22s} {str(trop_ok):>12s} "
        f"{str(fold_ok):>9s} {str(trop_ok and fold_ok):>6s}"
    )

say()
say(f"  cells serving BOTH consumers for addition: {len(both_cells)} of {len(rows)}")
for w, p in both_cells:
    say(f"    {w}, {p}")

say()
say("  This reproduces `63` section 3.4's inhabited two-by-two from the consumer")
say("  side: the cell holding both law families is unsigned saturation over a")
say("  nonnegative window, and that is exactly the workload shape of nonnegative")
say("  shortest path. The default the imitation intent points at (I3, signed) is")
say("  in a cell that serves one consumer or the other and never both.")
say()
say("  What this does not establish: anything about multiplication under the")
say("  tropical reading (the tropical product is the numeral's add, already")
say("  covered by the 'add' column); anything at nonzero fraction width; any")
say("  magnitude whatsoever. One model width, exhaustive within it.")

with open("p4_two_law_families_two_consumers.out", "w") as f:
    f.write("\n".join(out) + "\n")
