#!/usr/bin/env python3
"""PROBE 3: the whole verifier. One .ll file in, two directions out.

Input is LLVM IR from a CHECK BUILD: the ordinary build plus two flags
(`-Cno-prepopulate-passes -Zinline-mir=no`), measured in probe 1 to be the
cheapest artifact in which every monomorphisation is still a named function
with its body attached. Probe 1 also measures that the shipping artifact
contains none of them, with or without LTO.

  under-claim  a body contains an operation the composition's declared axes do
               not license. The unsound direction. Checked per operation,
               because one operation doing an unlicensed thing is the defect
               regardless of what its siblings do.
  over-claim   a composition declares an axis instance and nothing anywhere in
               it corresponds. The empty-promise direction `19_ringer` asked to
               be made visible. Checked per COMPOSITION, not per operation:
               probe 2 measures a 100 percent false-positive rate per
               operation, because `dot` has no reason to contain a packed load.

RULES is the entire semantic content of this program. One row per axis instance
that is supposed to be visible in generated code. Adding an axis is adding
rows, and a row nobody writes is a check nobody gets, silently. That is the
maintenance cost, stated here rather than discovered later.
"""
import re
import sys

# (marker type name, regex over the body text, one line of English)
RULES = [
    ("Relaxed",   r"@llvm\.fma|@llvm\.fmuladd|7mul_add",  "fused multiply-add"),
    ("Relaxed",   r"f(add|mul|sub) [a-z ]*(fast|reassoc|contract)", "a relaxed float flag"),
    ("Bitpacked", r"\blshr\b",                            "a shift, so a packed load"),
]

DEFINE = re.compile(r"^define[^@]*@([\w$.]+)\([^\n]*\{\n(.*?)\n\}", re.S | re.M)
CALLEE = re.compile(r"@([\w$.]+)\(")


def ident(s, i):
    """Read one v0 <len><name> at i. A leading `_` is a marker, not counted."""
    m = re.compile(r"(\d+)(_?)").match(s, i)
    if not m:
        return None, i
    n, j = int(m.group(1)), m.end()
    return s[j:j + n], j + n


def decode(sym):
    """(operation, [(defining crate, marker name)], const values) from a symbol.

    The defining crate of every generic type argument is in the symbol, measured
    in `04_the_marker_carries_its_crate.sh`. That is what scopes the verifier:
    a composition is a monomorphisation with an argument from an axis crate, and
    nothing else in the program qualifies. Adding an axis INSTANCE therefore
    costs nothing here, because the new marker is in the same crate.
    """
    markers = []
    for m in re.finditer(r"Nt(?:(?:B[0-9a-zA-Z]*_)|(?:Cs[0-9A-Za-z]+_))?(?=\d)", sym):
        end = m.end()
        first, j = ident(sym, end)
        if m.group(0).startswith("NtCs"):        # cross-crate: crate then item
            second, _ = ident(sym, j)
            markers.append((first, second))
        else:                                    # same-crate backref
            markers.append((None, first))
    consts = [int(h, 16) if h else 0 for h in re.findall(r"K[a-z]([0-9a-f]*)_", sym)]
    h = re.match(r"_RIN[a-z]Cs[0-9A-Za-z]+_(?=\d)", sym)
    op = "?"
    if h:
        _, i = ident(sym, h.end())          # crate name
        op = ident(sym, i)[0] or "?"
    return op, markers, consts


def main():
    text = sys.stdin.read()
    depth = int(sys.argv[sys.argv.index("--depth") + 1]) if "--depth" in sys.argv else 1
    scope = sys.argv[sys.argv.index("--scope") + 1] if "--scope" in sys.argv else "arvo_"
    bodies = {m.group(1): m.group(2) for m in DEFINE.finditer(text)}

    def reach(sym):
        seen, frontier = {sym}, [sym]
        for _ in range(depth):
            frontier = [c for f in frontier for c in CALLEE.findall(bodies.get(f, ""))
                        if c in bodies and not (c in seen or seen.add(c))]
        return "\n".join(bodies.get(s, "") for s in seen)

    under, comps, ops = [], {}, 0
    for sym in bodies:
        if "_RIN" not in sym:
            continue                        # not a generic monomorphisation
        op, pairs, consts = decode(sym)
        pairs = [(c, n) for c, n in pairs if n]
        consts = (consts + [0, 0])[:8]
        # SCOPE: at least one generic argument comes from an axis crate.
        if not any(c is None or c.startswith(scope) for c, _ in pairs):
            continue
        markers = [n for _, n in pairs]
        if not markers:
            continue
        ops += 1
        body, key = reach(sym), (tuple(consts), tuple(markers))
        comps.setdefault(key, []).append(body)
        who = f"{consts[0]:>3}/{consts[1]:<3} {'/'.join(markers)[:40]:<40} {op}"
        for marker, pattern, english in RULES:
            if re.search(pattern, body) and marker not in markers:
                under.append(f"  {who}: has {english}, {marker} not declared")

    # OR the rules per marker: one marker with several witnesses is satisfied
    # by any of them, and probe 2 measures that treating them separately is
    # pure noise on a clean build.
    witness = {}
    for marker, pattern, english in RULES:
        witness.setdefault(marker, []).append((pattern, english))
    over = [f"  {c[0]:>3}/{c[1]:<3} {'/'.join(mk)[:40]:<40}: declares {marker}, no "
            + " or ".join(e for _, e in ws) + " anywhere in it"
            for (c, mk), bs in comps.items() for marker, ws in witness.items()
            if marker in mk and not any(re.search(p, b) for p, _ in ws for b in bs)]

    print(f"{ops} monomorphisations in {len(comps)} compositions, "
          f"{len(RULES)} rules, call depth {depth}\n")
    print(f"UNDER-CLAIM ({len(under)}), per operation, the unsound direction:")
    print("\n".join(sorted(set(under))) or "  none")
    print(f"\nOVER-CLAIM ({len(over)}), per composition, the empty-promise direction:")
    print("\n".join(sorted(set(over))) or "  none")
    return 1 if under else 0


if __name__ == "__main__":
    sys.exit(main())
