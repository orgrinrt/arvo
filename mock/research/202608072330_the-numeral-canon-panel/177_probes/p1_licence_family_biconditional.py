#!/usr/bin/env python3
"""
177 P1. Testing 176 section 1's structural claim, the load-bearing reason the B3 repair is
said to COMPOSE rather than patch.

THE CLAIM UNDER TEST, quoted from 176 (lines 74-80):
  "175's chains with boundary-function families are exactly chains where clause 6's
   licences refuse deletion: `*3 >>1 *5` under a rounding resolution has no algebra
   licence (rounding does not commute with the shift) and no range licence (intermediates
   off the grid). Under the wrap resolution, where the algebra licence holds, 175's own
   identity column and 168 4.1's degeneracy table agree the family has one member at every
   depth. So the definite description in clause 2 denotes precisely where a deletion
   licence holds or an exact grade is declared."

  Two words are biconditional: "exactly" and "precisely". Decomposed:
    (i)   licence holds  =>  family = 1          [needed for the repair to compose]
    (ii)  family > 1     =>  licence refuses     [same statement, contrapositive]
    (iii) family = 1     =>  licence holds       [asserted by "exactly"/"precisely";
                                                  NOT needed by the repair]

  This probe measures all three over 175's own four chains, at three resolutions,
  reproducing 175's table as its first output so the extension is anchored to it.

NEGATIVE CONTROLS, declared before the run
  K1. The family counter must be able to report > 1. If every cell is 1 the counter is
      dead and no verdict below means anything.
  K2. The identity resolution must give family = 1 everywhere. That is 175's own C-A and
      it re-arms the same control here.
  K3. A chain whose algebra licence structurally HOLDS must show family = 1. If it does
      not, my licence predicate is not tracking what it claims and direction (i) is
      untested rather than confirmed.
  K4. The structural licence predicate must REFUSE somewhere and HOLD somewhere. A
      predicate that is constant carries no information.
"""
import itertools

W = 8
MOD = 1 << W

def clamp(v):  return 0 if v < 0 else (255 if v > 255 else v)
def rnd8(v):   v = clamp(v); return min(248, ((v + 4) // 8) * 8)
def ident(v):  return v
def wrap(v):   return v % MOD

RES = {"identity": ident, "round-to-8": rnd8, "wrap": wrap}

OPS = {
    '+97': lambda v: v + 97,
    '*3':  lambda v: v * 3,
    '>>1': lambda v: v >> 1,
    '+13': lambda v: v + 13,
    '*5':  lambda v: v * 5,
}
CHAINS = [
    ['+97', '*3', '+13'],
    ['*3', '>>1', '*5'],
    ['+97', '*5', '>>1', '+13'],
    ['*3', '*3', '+97'],
]

def run(chain, placement, pi):
    def f(x):
        v = x
        for i, name in enumerate(chain):
            v = OPS[name](v)
            if placement[i]:
                v = pi(v)
        return pi(v)
    return tuple(f(x) for x in range(256))

def family(chain, pi):
    n = len(chain)
    seen = set()
    for pl in itertools.product([False, True], repeat=n - 1):
        seen.add(run(chain, tuple(list(pl) + [False]), pi))
    return len(seen)

# ---- the two licences of clause 6, as clause 6 words them ----

def algebra_licence(chain, resname):
    """'the resolution commutes with or is absorbed by the composition; reads the
    operations, not any bound.' Checked structurally per operation, exhaustively over the
    reachable intermediates rather than asserted."""
    pi = RES[resname]
    reach = set(range(256))
    for name in chain:
        op = OPS[name]
        # does pi commute with this op on the reachable set?
        if any(pi(op(v)) != pi(op(pi(v))) for v in sorted(reach)):
            return False
        reach = {op(v) for v in reach} | {pi(op(v)) for v in reach}
        if len(reach) > 200000:
            reach = set(sorted(reach)[:200000])
    return True

def range_licence(chain, resname):
    """'every intermediate provably where the resolution is the identity.'"""
    pi = RES[resname]
    vals = set(range(256))
    for name in chain[:-1]:
        vals = {OPS[name](v) for v in vals}
        if any(pi(v) != v for v in vals):
            return False
    return True

print("PART 1. Reproducing 175's clause23 table, plus the wrap column 176 appeals to.")
print(f"{'chain':>26} {'identity':>9} {'round-to-8':>11} {'wrap':>6}")
tbl = {}
for ch in CHAINS:
    nm = " ".join(ch)
    row = {r: family(ch, RES[r]) for r in RES}
    tbl[nm] = row
    print(f"{nm:>26} {row['identity']:>9} {row['round-to-8']:>11} {row['wrap']:>6}")
print()

print("PART 2. The two licences of clause 6, per chain and resolution.")
print(f"{'chain':>26} {'resolution':>11} {'algebra':>8} {'range':>6} {'family':>7}")
rows = []
for ch in CHAINS:
    nm = " ".join(ch)
    for r in ("round-to-8", "wrap"):
        a = algebra_licence(ch, r)
        g = range_licence(ch, r)
        f = tbl[nm][r]
        rows.append((nm, r, a, g, f))
        print(f"{nm:>26} {r:>11} {str(a):>8} {str(g):>6} {f:>7}")
print()

print("PART 3. The three directions.")
i_ok  = [(n, r, a, g, f) for (n, r, a, g, f) in rows if (a or g) and f != 1]
ii_ok = [(n, r, a, g, f) for (n, r, a, g, f) in rows if f > 1 and (a or g)]
iii_x = [(n, r, a, g, f) for (n, r, a, g, f) in rows if f == 1 and not (a or g)]
print(f"  (i)   licence holds => family = 1 : {'HOLDS' if not i_ok else 'FAILS'}"
      + (f"  counterexamples {i_ok}" if i_ok else "  (no counterexample)"))
print(f"  (ii)  family > 1 => licence refuses: {'HOLDS' if not ii_ok else 'FAILS'}"
      + (f"  counterexamples {ii_ok}" if ii_ok else "  (no counterexample)"))
print(f"  (iii) family = 1 => licence holds  : {'HOLDS' if not iii_x else 'FAILS'}")
for c in iii_x:
    print(f"        COUNTEREXAMPLE  chain={c[0]!r}  resolution={c[1]}  "
          f"algebra={c[2]} range={c[3]} family={c[4]}")
print()

print("CONTROLS")
k1 = any(f > 1 for row in tbl.values() for f in row.values())
k2 = all(row['identity'] == 1 for row in tbl.values())
holds = [(n, r, f) for (n, r, a, g, f) in rows if (a or g)]
k3 = all(f == 1 for (_, _, f) in holds) and len(holds) > 0
preds = {(a or g) for (_, _, a, g, _) in rows}
k4 = len(preds) == 2
print(f"  K1 the family counter can report > 1        : {'PASS' if k1 else 'FAIL'}")
print(f"  K2 identity resolution gives 1 everywhere   : {'PASS' if k2 else 'FAIL'}")
print(f"  K3 licence holds => family 1 on every such  : {'PASS' if k3 else 'FAIL'}"
      f"   ({len(holds)} cells where a licence holds)")
print(f"  K4 the licence predicate is not constant    : {'PASS' if k4 else 'FAIL'}")
print()
print("READING")
print("  The direction the repair needs is (i)/(ii). The word 'exactly' in 176 and the word")
print("  'precisely' in its conclusion assert (iii) as well.")
