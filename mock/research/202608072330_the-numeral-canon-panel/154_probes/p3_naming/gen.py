#!/usr/bin/env python3
"""P3. Generate a crate with W widths x P policies of named primitive, and count
what survives to the object file.

P1/F4 found that two distinct named primitives with identical lowered bodies are
merged into one symbol by the linker. This asks the scale question: over a grid,
how many named instantiations produce how many distinct machine bodies?

NEGATIVE CONTROL, built into the generated crate. Two families are emitted:
  - `d_*`  : DISTINCT by construction. Width w's mask is different for every w,
             so no two may fold. If the survivor count for this family is not
             exactly the grid size, the counting instrument is broken.
  - `s_*`  : IDENTICAL by construction. Every one of them computes `x & MASK_13`
             under a different NAME. Every one must be an alias, and all of them
             must alias the SAME target. If any of them carries its own body,
             folding is not happening and the F4 reading is wrong.

The first version of this control expected the identical family to keep exactly
one body of its own. It does not: all 20 fold onto `_d_wrap_13`, a member of the
OTHER family, because `x & mask(13)` is what that function already computes.
The control refused to print, which is what it is for, and the corrected
expectation below is stronger than the one it replaced -- the identical family
costs zero bodies, not one.
Both controls must pass or the numbers are not reported.
"""
import subprocess, sys, os, collections

WIDTHS = [3, 7, 11, 13, 14, 23, 27, 31, 47, 61]
POLICIES = ["wrap", "clamp"]

def gen(path):
    L = []
    L.append("#![no_std]")
    L.append("#![allow(dead_code)]")
    L.append("")
    L.append("#[repr(transparent)] pub struct P<const W: u32, S>(pub u64, pub core::marker::PhantomData<S>);")
    L.append("pub struct Wrap; pub struct Clamp;")
    L.append("pub const fn mask(w: u32) -> u64 { if w >= 64 { u64::MAX } else { (1u64 << w) - 1 } }")
    L.append("")
    L.append("// --- family d: distinct by construction, one per (width, policy) ---")
    for w in WIDTHS:
        for p in POLICIES:
            if p == "wrap":
                body = f"x & mask({w})"
            else:
                body = f"{{ let m = mask({w}); if x > m {{ m }} else {{ x }} }}"
            L.append(f"#[unsafe(no_mangle)] pub fn d_{p}_{w}(x: u64) -> u64 {{ {body} }}")
    L.append("")
    L.append("// --- family s: identical by construction, distinct names only ---")
    for i in range(len(WIDTHS) * len(POLICIES)):
        L.append(f"#[unsafe(no_mangle)] pub fn s_alias_{i}(x: u64) -> u64 {{ x & mask(13) }}")
    L.append("")
    L.append("#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }")
    open(path, "w").write("\n".join(L) + "\n")

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    src = os.path.join(here, "grid.rs")
    obj = os.path.join(here, "grid.o")
    asm = os.path.join(here, "grid.s")
    gen(src)
    grid = len(WIDTHS) * len(POLICIES)
    for emit, out in (("obj", obj), ("asm", asm)):
        r = subprocess.run(["rustc", "--edition", "2024", "-O", "--crate-type", "lib",
                            "-C", "panic=abort", f"--emit={emit}", "-o", out, src],
                           capture_output=True, text=True)
        if r.returncode != 0:
            print(r.stderr); sys.exit(1)

    # Count distinct machine bodies by parsing the emitted assembly: a label that
    # is defined as `X = Y` is an alias, not a body.
    bodies, aliases = set(), {}
    cur = None
    for line in open(asm):
        t = line.strip()
        if t.startswith(".globl"):
            continue
        if " = " in t and t.startswith("_"):
            a, b = t.split(" = ", 1)
            aliases[a.strip()] = b.strip()
            continue
        if t.endswith(":") and t.startswith("_"):
            cur = t[:-1]
            bodies.add(cur)
    d_bodies = {b for b in bodies if b.startswith("_d_")}
    s_bodies = {b for b in bodies if b.startswith("_s_")}
    d_alias = {a for a in aliases if a.startswith("_d_")}
    s_alias = {a for a in aliases if a.startswith("_s_")}

    d_survivors = len(d_bodies)
    s_survivors = len(s_bodies)

    print(f"grid: {len(WIDTHS)} widths x {len(POLICIES)} policies = {grid} named primitives per family")
    print()
    print(f"CONTROL distinct-family : {d_survivors} bodies + {len(d_alias)} aliases (want {grid} bodies, 0 aliases)")
    targets_pre = {aliases[a] for a in s_alias}
    print(f"CONTROL identical-family: {s_survivors} bodies + {len(s_alias)} aliases "
          f"(want 0 bodies, {grid} aliases, 1 distinct target); targets = {sorted(targets_pre)}")
    targets = {aliases[a] for a in s_alias}
    ok = (d_survivors == grid and len(d_alias) == 0
          and s_survivors == 0 and len(s_alias) == grid and len(targets) == 1)
    print(f"controls: {'ok' if ok else 'FAILED'}")
    if not ok:
        print("NEGATIVE CONTROL FAILED -- result suppressed")
        sys.exit(1)
    print()
    print(f"object file size: {os.path.getsize(obj)} bytes")
    print(f"asm lines       : {sum(1 for _ in open(asm))}")
    print()
    print("READING: naming N genuinely-different primitives costs N machine bodies.")
    print("         naming N primitives that compute what some existing body already")
    print("         computes costs ZERO additional bodies: all N become aliases of it,")
    print("         across family boundaries. Here 40 named primitives -> 20 bodies.")
    print("         The cost of a name tracks what its body computes, never the name,")
    print("         and never the number of type parameters that produced it.")

if __name__ == "__main__":
    main()
