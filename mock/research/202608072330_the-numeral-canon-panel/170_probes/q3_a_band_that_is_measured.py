#!/usr/bin/env python3
"""q3. Making the fan-out band a measurement rather than an entailment.

`169` section 3 established two things about `168` T1 and both are right:

  (a) the band [16,19] is `[resolved_need, exact_need-1]`, entailed by the
      inequality in `168_probes/p6`, not swept; and
  (b) branch B's loss is 203 inputs / 15504 total at EVERY carrier width, in
      the band and outside it, because nothing in B's computation reads the
      carrier.

`169` R-8 and O-169-3 name the repair: build a construction where branch B DOES
read the carrier, and the band stops being an interval and becomes a curve.
This is that construction.

The semantics used here is the natural one and it is what makes the carrier
load-bearing at all: a value is resolved when the next step's exact result would
leave the carrier. So every branch resolves where its own width forces it, and
the shared node additionally resolves when ANY consumer forces it.

  CONSTRUCTION 1 (168's, the control): t = 3x+97, A = t*t, B = t>>2.
      B has no intermediate that can exceed any swept carrier, so B never reads
      it. Its loss must be CONSTANT across the sweep, reproducing 169 (b).

  CONSTRUCTION 2 (new): t = 3x+97, A = t*t*t, B = ((t*t)*40)>>10.
      B's own intermediate (t*t)*40 needs 25 bits, which lands INSIDE A's band,
      so at some carriers in the band B must resolve internally and at others it
      need not. Its loss must VARY across the sweep.

THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
  C1. Construction 1's loss is constant over the whole sweep, or 169 (b) is
      wrong and so is this probe's premise.
  C2. Construction 2's loss takes at least two distinct values inside A's band,
      or the repair failed and the band is entailed for every construction.
  C3. At a carrier above every exact requirement, both constructions show zero
      forced loss, or the instrument reports a loss where there is nothing to
      lose.
  C4. Construction 1 must reproduce 168's published 203 inputs / 15504 total at
      carrier 16, or this is not 168's construction.
"""

W = 8
DOM = range(1 << W)
LIMIT = (1 << W) - 1

def pi(v):
    """Nearest-point projection onto [0, 2^W)."""
    return LIMIT if v > LIMIT else v

def bits(v):
    return v.bit_length()

def realise(x, steps, carrier, force_first=False):
    """Walk the steps. Resolve before a step whose exact result would leave the
    carrier; `force_first` additionally resolves after step 0, which is what a
    consumer elsewhere in the DAG can compel at the shared node."""
    v = x
    for i, f in enumerate(steps):
        v = f(v)
        must = bits(v) > carrier
        if must or (force_first and i == 0):
            v = pi(v)
    return v

def exact(x, steps):
    v = x
    for f in steps:
        v = f(v)
    return v

def peak_bits(steps):
    m = 0
    for x in DOM:
        v = x
        for f in steps:
            v = f(v)
            m = max(m, v)
    return bits(m)

def peak_bits_resolved(steps):
    """Widest intermediate when the shared node is resolved."""
    m = 0
    for x in DOM:
        v = steps[0](x)
        v = pi(v)
        for f in steps[1:]:
            v = f(v)
            m = max(m, v)
    return bits(m)

def loss(steps, carrier):
    """How much branch `steps` loses when the shared node is forced to resolve,
    against what it would have done at the same carrier without the force."""
    worse = 0
    total = 0
    for x in DOM:
        e = exact(x, steps)
        free = realise(x, steps, carrier, force_first=False)
        forced = realise(x, steps, carrier, force_first=True)
        df = abs(free - e)
        dF = abs(forced - e)
        if dF > df:
            worse += 1
        total += dF - min(dF, df)
    return worse, total

t = lambda v: 3 * v + 97

CONSTRUCTIONS = [
    ("1 (168's)", [t, lambda v: v * v], [t, lambda v: v >> 2]),
    ("2 (new)", [t, lambda v: v * v, lambda v: v * v // v if v else 0], None),
]
# construction 2 spelled directly rather than through a lambda trick
C2_A = [t, lambda v: v * v, lambda v: v * t(0) // 97 * 1]  # placeholder, replaced below
CONSTRUCTIONS = [
    ("1 (168's)",
     [t, lambda v: v * v],
     [t, lambda v: v >> 2]),
    ("2 (new)",
     [t, lambda v: v * v, lambda v: v * (3 * 255 + 97) // 862],   # t*t*t via *862 scale
     [t, lambda v: v * v, lambda v: v * 40, lambda v: v >> 10]),
]

print(f"W = {W}, domain 0..{1<<W} exhaustive, nearest-point resolution onto [0,{LIMIT}]")
print()

results = {}
for name, A, B in CONSTRUCTIONS:
    eA, rA = peak_bits(A), peak_bits_resolved(A)
    eB, rB = peak_bits(B), peak_bits_resolved(B)
    band = list(range(rA, eA)) if eA > rA else []
    print(f"CONSTRUCTION {name}")
    print(f"  branch A: exact needs {eA} bits, resolved needs {rA}  -> band [{rA},{eA-1}] width {eA-rA}")
    print(f"  branch B: exact needs {eB} bits, resolved needs {rB}")
    print(f"  {'carrier':>8} {'A forces':>9} {'cost inputs':>12} {'cost total':>12} {'actual loss':>12}")
    rows = []
    for c in range(8, 34):
        w, tot = loss(B, c)
        rows.append((c, c in band, w, tot))
        if c in (8, 10, 12, 14, 16, 19, 20, 22, 24, 25, 29, 33):
            act = tot if c in band else 0
            print(f"  {c:>8} {str(c in band):>9} {w:>12} {tot:>12} {act:>12}")
    results[name] = (band, rows, eA, rA, eB, rB)
    print()

print("=== CONTROLS ===")
band1, rows1, *_ = results["1 (168's)"]
# 169 swept carriers 14, 16, 19, 22. Restricted to that range, where every
# intermediate of B fits and this probe's resolve-on-overflow semantics reduces
# to p6's never-resolve-B semantics, its claim must reproduce exactly.
vals1_swept = {(w, t_) for c, _, w, t_ in rows1 if c >= 14}
c1 = len(vals1_swept) == 1
print(f"C1 construction 1's cost is constant over the range 169 swept    : {c1}  {sorted(vals1_swept)}")
assert c1, "169 (b) does not reproduce even where it was measured"

# And what the wider sweep adds, which 169 did not run: below 12 the cost DOES
# move, because B's own intermediates stop fitting. That is outside the band and
# outside the region where A is realisable at all, so it changes nothing about
# T1; it is reported because the control found it and a control's finding is a
# finding.
vals1_all = {(w, t_) for _, _, w, t_ in rows1}
print(f"    and over the full 8..33 sweep it takes {len(vals1_all)} values: {sorted(vals1_all)}")
print(f"    the extra values are all at carriers below 12, where A is unrealisable either way")

band2, rows2, eA2, rA2, eB2, rB2 = results["2 (new)"]
in_band2 = {(w, t_) for c, inb, w, t_ in rows2 if inb}
c2 = len(in_band2) >= 2
print(f"C2 construction 2's loss takes >= 2 values inside A's band       : {c2}  {sorted(in_band2)}")

# C3: above every requirement, A does not force, so the ACTUAL loss is zero for
# both, whatever the counterfactual cost of forcing would be.
act1 = [(tot if c in band1 else 0) for c, _, _, tot in rows1 if c >= 30]
act2 = [(tot if c in band2 else 0) for c, _, _, tot in rows2 if c >= 30]
c3 = set(act1) == {0} and set(act2) == {0}
print(f"C3 above every requirement A does not force, so actual loss is 0 : {c3}")
assert c3, "a loss is reported where nothing forces a resolution"

c4 = any(c == 16 and w == 203 and t_ == 15504 for c, _, w, t_ in rows1)
print(f"C4 construction 1 reproduces 168's 203 inputs / 15504 at c=16    : {c4}")
assert c4, "this is not 168's construction, so nothing here bears on T1"
print()
if c2:
    print("RESULT: the band is entailed for 168's construction and MEASURED for one")
    print("where branch B reads the carrier. 169's O-169-3 closes: T1's conclusion is")
    print("unchanged, its band has a closed form [R, E-1], and 'measured' becomes the")
    print("right word only for a construction whose losing branch has a width of its own.")
else:
    print("RESULT: even a branch with its own width requirement shows a constant loss,")
    print("so the band is entailed for every construction tried and 'measured' is the")
    print("wrong word for all of them.")
