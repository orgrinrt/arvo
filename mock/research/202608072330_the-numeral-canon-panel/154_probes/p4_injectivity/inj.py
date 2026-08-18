#!/usr/bin/env python3
"""P4. Is the map from index (width, policy) to primitive injective?

If a canon says "a primitive is a point of the index space", that is only right
if two distinct indices always name two distinct primitives. This measures it:
emit the whole grid, fold, and count how many index points survive as distinct
machine bodies.

NEGATIVE CONTROL, stated before the run. The grid MUST contain at least one
pair that stays distinct and at least one pair that collapses, and both must be
predictable in advance:
  - distinct: (w=13, wrap) vs (w=47, wrap). Different masks, cannot fold.
  - collapse: (w=64, wrap) vs (w=64, clamp). At full container width the mask is
    all-ones, so masking is identity and clamping to the maximum is identity.
If the predicted-distinct pair folds, folding is over-eager and every count is
suspect. If the predicted-collapse pair stays distinct, the compiler is not
canonicalising and the measurement understates collapse rather than measuring
it; either way the result is suppressed.
"""
import subprocess, sys, os, collections

WIDTHS = list(range(1, 65))
POLICIES = ["wrap", "clamp"]

def body(w, p):
    if p == "wrap":
        return f"x & mask({w})"
    return f"{{ let m = mask({w}); if x > m {{ m }} else {{ x }} }}"

def gen(path):
    L = ["#![no_std]", "#![allow(dead_code)]", "",
         "pub const fn mask(w: u32) -> u64 { if w >= 64 { u64::MAX } else { (1u64 << w) - 1 } }", ""]
    for w in WIDTHS:
        for p in POLICIES:
            L.append(f"#[unsafe(no_mangle)] pub fn g_{p}_{w}(x: u64) -> u64 {{ {body(w,p)} }}")
    L.append("#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }")
    open(path, "w").write("\n".join(L) + "\n")

def parse_asm(path):
    """Return (bodies: name -> instruction tuple, aliases: name -> target)."""
    bodies, aliases, cur, acc = {}, {}, None, []
    for line in open(path):
        t = line.strip()
        if t.startswith(".globl") or t.startswith(".p2align") or t.startswith(".section") \
           or t.startswith(".build_version") or t.startswith(".subsections"):
            continue
        if " = " in t and t.startswith("_"):
            a, b = t.split(" = ", 1)
            aliases[a.strip()] = b.strip()
            continue
        if t.endswith(":") and t.startswith("_"):
            if cur is not None:
                bodies[cur] = tuple(acc)
            cur, acc = t[:-1], []
            continue
        if cur is not None and t and not t.startswith(("L", ";", ".")):
            acc.append(t)
    if cur is not None:
        bodies[cur] = tuple(acc)
    return bodies, aliases

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    src, asm = os.path.join(here, "grid64.rs"), os.path.join(here, "grid64.s")
    gen(src)
    r = subprocess.run(["rustc", "--edition", "2024", "-O", "--crate-type", "lib",
                        "-C", "panic=abort", "--emit=asm", "-o", asm, src],
                       capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stderr); sys.exit(1)
    bodies, aliases = parse_asm(asm)

    def resolve(n):
        seen = 0
        while n in aliases and seen < 50:
            n = aliases[n]; seen += 1
        return n

    grid = [(w, p) for w in WIDTHS for p in POLICIES]
    sym = {(w, p): f"_g_{p}_{w}" for (w, p) in grid}
    canon = {k: resolve(v) for k, v in sym.items()}

    # controls
    c_distinct = canon[(13, "wrap")] != canon[(47, "wrap")]
    c_collapse = canon[(64, "wrap")] == canon[(64, "clamp")]
    print(f"CONTROL predicted-distinct (13,wrap) vs (47,wrap): {'distinct' if c_distinct else 'FOLDED'} -> {'ok' if c_distinct else 'FAIL'}")
    print(f"CONTROL predicted-collapse (64,wrap) vs (64,clamp): {'same' if c_collapse else 'DISTINCT'} -> {'ok' if c_collapse else 'FAIL'}")
    if not (c_distinct and c_collapse):
        print("NEGATIVE CONTROL FAILED -- result suppressed"); sys.exit(1)
    print()

    classes = collections.defaultdict(list)
    for k, v in canon.items():
        classes[v].append(k)
    print(f"index points in the grid : {len(grid)}  (widths 1..64 x 2 policies)")
    print(f"distinct primitives      : {len(classes)}")
    print(f"index points that are NOT their own primitive: {len(grid) - len(classes)}")
    print()
    collapsed = {v: ks for v, ks in classes.items() if len(ks) > 1}
    print(f"collapse classes: {len(collapsed)}")
    for v, ks in sorted(collapsed.items(), key=lambda kv: (-len(kv[1]), kv[0]))[:12]:
        print(f"  {len(ks):3d} indices -> one primitive : {sorted(ks)[:6]}{' ...' if len(ks)>6 else ''}")
    print()
    per_w = [w for w in WIDTHS if canon[(w, "wrap")] == canon[(w, "clamp")]]
    print(f"widths at which wrap and clamp are the SAME primitive: {per_w}")
    print(f"widths at which they differ: {len([w for w in WIDTHS if w not in per_w])} of {len(WIDTHS)}")

if __name__ == "__main__":
    main()
