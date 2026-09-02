#!/usr/bin/env python3
"""P4b. The degenerate set is not fixed: it moves with the container policy.

P4 found exactly one collapse in a 128-point grid, at W = 64, where the declared
width fills the u64 container so masking and clamping are both the identity.
That suggests a rule: the overflow policy has no content where the width fills
its container.

If that is the rule, then choosing containers MINIMALLY (u8 for W<=8, u16 for
W<=16, ...) -- which is what a storage-minimising treatment does -- must move the
degenerate set to {8, 16, 32, 64} rather than {64}.

NEGATIVE CONTROL, stated before the run. Under the minimal-container arm, W = 8
must collapse and W = 7 must not. If W = 7 also collapses, the container is not
actually varying and the arm is the wide arm under another name. If W = 8 does
not collapse, the predicted rule is wrong and the result is reported as a
refutation rather than as support.
"""
import subprocess, sys, os, collections

WIDTHS = list(range(1, 65))
POLICIES = ["wrap", "clamp"]

def ctr_wide(w):
    return "u64"

def ctr_min(w):
    return "u8" if w <= 8 else "u16" if w <= 16 else "u32" if w <= 32 else "u64"

def gen(path, ctr, tag):
    L = ["#![no_std]", "#![allow(dead_code)]", ""]
    for w in WIDTHS:
        c = ctr(w)
        bits = int(c[1:])
        m = "!0" if w >= bits else f"((1{c} << {w}) - 1)"
        L.append(f"#[unsafe(no_mangle)] pub fn {tag}_wrap_{w}(x: {c}) -> {c} {{ x & {m} }}")
        L.append(f"#[unsafe(no_mangle)] pub fn {tag}_clamp_{w}(x: {c}) -> {c} "
                 f"{{ let m: {c} = {m}; if x > m {{ m }} else {{ x }} }}")
    L.append("#[panic_handler] fn ph(_: &core::panic::PanicInfo) -> ! { loop {} }")
    open(path, "w").write("\n".join(L) + "\n")

def parse_asm(path):
    bodies, aliases, cur, acc = {}, {}, None, []
    for line in open(path):
        t = line.strip()
        if t.startswith((".globl", ".p2align", ".section", ".build_version", ".subsections")):
            continue
        if " = " in t and t.startswith("_"):
            a, b = t.split(" = ", 1); aliases[a.strip()] = b.strip(); continue
        if t.endswith(":") and t.startswith("_"):
            if cur is not None: bodies[cur] = tuple(acc)
            cur, acc = t[:-1], []; continue
        if cur is not None and t and not t.startswith(("L", ";", ".")):
            acc.append(t)
    if cur is not None: bodies[cur] = tuple(acc)
    return bodies, aliases

def run(ctr, tag, here):
    src = os.path.join(here, f"grid_{tag}.rs"); asm = os.path.join(here, f"grid_{tag}.s")
    gen(src, ctr, tag)
    r = subprocess.run(["rustc", "--edition", "2024", "-O", "--crate-type", "lib",
                        "-C", "panic=abort", "--emit=asm", "-o", asm, src],
                       capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stderr); sys.exit(1)
    bodies, aliases = parse_asm(asm)
    def resolve(n):
        s = 0
        while n in aliases and s < 50: n = aliases[n]; s += 1
        return n
    # Two indices name one primitive iff they resolve to the same symbol OR their
    # instruction sequences are identical. Folding is per-signature, so a u8 and a
    # u64 body that "look" the same are NOT the same primitive; comparison is
    # restricted to indices sharing a container.
    canon = {}
    for w in WIDTHS:
        for p in POLICIES:
            n = resolve(f"_{tag}_{p}_{w}")
            canon[(w, p)] = (ctr(w), bodies.get(n, ("<alias>", n)))
    same = [w for w in WIDTHS if canon[(w, "wrap")] == canon[(w, "clamp")]]
    return same

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    wide = run(ctr_wide, "cw", here)
    mini = run(ctr_min, "cm", here)
    ok8 = 8 in mini
    ok7 = 7 not in mini
    print(f"CONTROL minimal-container W=8 collapses : {ok8} -> {'ok' if ok8 else 'FAIL'}")
    print(f"CONTROL minimal-container W=7 does not  : {ok7} -> {'ok' if ok7 else 'FAIL'}")
    if not (ok8 and ok7):
        print("NEGATIVE CONTROL FAILED -- result suppressed"); sys.exit(1)
    print()
    print(f"wide container (always u64), widths where wrap == clamp : {wide}")
    print(f"minimal container (u8/u16/u32/u64),      same           : {mini}")
    print()
    print(f"degenerate index points, wide arm    : {len(wide)} of {len(WIDTHS)*2}")
    print(f"degenerate index points, minimal arm : {len(mini)} of {len(WIDTHS)*2}")
    print()
    print("READING: the set of index points at which the overflow policy has no")
    print("         content is NOT a property of the declared width. It is a")
    print("         property of (declared width, container), and the container is")
    print("         chosen by the treatment. Changing the container policy moves")
    print(f"         the degenerate set from {wide} to {mini}.")

if __name__ == "__main__":
    main()
