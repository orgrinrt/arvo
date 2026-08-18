#!/usr/bin/env python3
"""172 P1. The deferral theorem checked at the generality of its hypotheses.

CLAIM UNDER TEST (the proof in 172 section 4)
  Let S be any nonempty finite set of representable values, and let the boundary
  resolution be ANY selection pi from the nearest-point correspondence onto S
  (ties broken by any fixed rule, adversarial included). For any chain of total
  steps and any placement of interior resolutions, the fully deferred form's
  output is at least as close to the exact composite as that placement's output,
  at every input. The proof needs only: (i) every placement's output lies in S,
  because the boundary resolution fires last in all of them; (ii) pi(x) attains
  min_{s in S} |s - x|.

  The unit's sweeps used grids and ranges for S. This probe draws S as RANDOM
  SUBSETS and breaks ties by a per-(S,x) hash, which is the adversarial-fixed
  selection O-170-2 asked about, and checks every placement, every input.

THE CASE THAT MUST FAIL, declared before the run
  The control resolution tau projects DOWNWARD onto the same random S (largest
  element <= x, else min S). It is a projection onto S and is not nearest-point.
  If the harness reports no placement win for tau over the whole sweep, the
  harness cannot detect a win and the zeros above it prove nothing.
"""
import random

W = 256  # value space 0..255 as exact integers; exact composite computed in Z

def mk_ops(rng):
    k = rng.randrange(1, 250)
    g = rng.randrange(1, 5)
    ops = [
        ("addk", lambda x, k=k: x + k),
        ("mulk", lambda x, m=rng.choice([2, 3, 5]): x * m),
        ("shr",  lambda x, g=g: x >> g),
        ("xork", lambda x, k=k: x ^ k),
        ("satsubk", lambda x, k=k: max(0, x - k)),
    ]
    return rng.choice(ops)

def nearest_selection(S, x, salt):
    # any fixed selection from the argmin correspondence; tie broken by hash
    best = min(abs(s - x) for s in S)
    cands = [s for s in S if abs(s - x) == best]
    if len(cands) == 1:
        return cands[0]
    return cands[hash((x, salt)) % len(cands)]

def down_projection(S, x):
    le = [s for s in S if s <= x]
    return max(le) if le else min(S)

def run_chain(x, steps, resolve, mask):
    v = x
    for i, (_, f) in enumerate(steps):
        v = f(v)
        if i < len(steps) - 1 and (mask >> i) & 1:
            v = resolve(v)
    return resolve(v)  # boundary resolution always fires

def exact_chain(x, steps):
    v = x
    for _, f in steps:
        v = f(v)
    return v

def sweep(n_chains, seed, use_nearest):
    rng = random.Random(seed)
    win_chains = 0
    win_inputs = 0
    exercised = 0
    for c in range(n_chains):
        depth = rng.randrange(2, 6)
        steps = [mk_ops(rng) for _ in range(depth)]
        # random representable set: size 2..64 drawn from 0..255
        size = rng.randrange(2, 65)
        S = sorted(rng.sample(range(W), size))
        salt = rng.randrange(1 << 30)
        if use_nearest:
            resolve = lambda v: nearest_selection(S, v, salt)
        else:
            resolve = lambda v: down_projection(S, v)
        chain_won = False
        chain_exercised = False
        n_int = depth - 1
        for x in range(W):
            e = exact_chain(x, steps)
            d = run_chain(x, steps, resolve, 0)
            derr = abs(d - e)
            for mask in range(1, 1 << n_int):
                y = run_chain(x, steps, resolve, mask)
                if y != d:
                    chain_exercised = True
                if abs(y - e) < derr:
                    win_inputs += 1
                    chain_won = True
        if chain_won:
            win_chains += 1
        if chain_exercised:
            exercised += 1
    return win_chains, win_inputs, exercised

def main():
    N = 400
    wc, wi, ex = sweep(N, 20260818, True)
    print(f"nearest (random S, adversarial ties): {wc} win-chains, {wi} win-inputs, {ex} exercised of {N}")
    cwc, cwi, cex = sweep(N, 20260818, False)
    print(f"CONTROL down-projection onto same S : {cwc} win-chains, {cwi} win-inputs, {cex} exercised of {N}")
    print(f"C1 theorem rows zero                : {'PASS' if wc == 0 and wi == 0 else 'FAIL'}")
    print(f"C2 control fires (harness can see a win): {'PASS' if cwc > 0 else 'FAIL'}")
    print(f"C3 placements genuinely move outputs: {'PASS' if ex > N // 2 else 'FAIL'}")

main()
