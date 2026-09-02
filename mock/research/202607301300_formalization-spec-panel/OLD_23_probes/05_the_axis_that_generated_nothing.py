#!/usr/bin/env python3
"""PROBE 5: the cruder check, which has no rules at all.

`03_the_whole_verifier.py` needs one hand-written rule per axis instance saying
what instruction witnesses it. Probe 3's own run over the real tree measures
what that costs in precision: `lshr` is a fine witness for nothing, because a
shift is what division by 64 and every hash and every capacity calculation also
compile to. An instruction is a good witness only when the instruction IS the
liberty, which is true of `llvm.fmuladd` and of a fast-math flag and of very
little else.

This program checks a different thing and knows nothing:

    for each axis, does varying that axis, with everything else held fixed,
    ever change one single instruction anywhere in the program

An axis that never changes anything is a declaration with no consequence. That
is the strongest form of the over-claim `19_ringer` asked to be made visible,
it is the defect file 16 found by hand when it noticed two float types
compiling identically, and finding it needs no knowledge of what any axis
means. Adding an axis to the design costs this program nothing.

It cannot see an under-claim. Two axes that both generate code, one of which
generates the wrong code, are indistinguishable to it. It is half a check, and
the cheap half.
"""
import hashlib
import re
import sys

DEFINE = re.compile(r"^define[^@]*@([\w$.]+)\([^\n]*\{\n(.*?)\n\}", re.S | re.M)


def ident(s, i):
    m = re.compile(r"(\d+)(_?)").match(s, i)
    if not m:
        return None, i
    n, j = int(m.group(1)), m.end()
    return s[j:j + n], j + n


def decode(sym):
    markers = []
    for m in re.finditer(r"Nt(?:(?:B[0-9a-zA-Z]*_)|(?:Cs[0-9A-Za-z]+_))?(?=\d)", sym):
        first, j = ident(sym, m.end())
        markers.append(ident(sym, j)[0] if m.group(0).startswith("NtCs") else first)
    consts = tuple(int(h, 16) if h else 0 for h in re.findall(r"K[a-z]([0-9a-f]*)_", sym))
    h = re.match(r"_RIN[a-z]Cs[0-9A-Za-z]+_(?=\d)", sym)
    op = "?"
    if h:
        _, i = ident(sym, h.end())
        op = ident(sym, i)[0] or "?"
    return op, [m for m in markers if m], consts


def shape(body):
    """Body with everything that is not an instruction shape removed."""
    b = re.sub(r"%[\w.]+", "%", body)          # SSA names
    b = re.sub(r"![\w.]+ ![0-9]+", "", b)      # metadata attachments
    b = re.sub(r"@_R[\w$.]+", "@F", b)         # callee symbol names
    b = re.sub(r"\s+", " ", b)
    return hashlib.blake2b(b.encode(), digest_size=8).hexdigest()


def main():
    scope = sys.argv[sys.argv.index("--scope") + 1] if "--scope" in sys.argv else ""
    rows = {}
    for m in DEFINE.finditer(sys.stdin.read()):
        sym, body = m.group(1), m.group(2)
        if "_RIN" not in sym:
            continue
        op, markers, consts = decode(sym)
        if not markers or (scope and scope not in sym):
            continue
        rows.setdefault((op, consts, len(markers)), {})[tuple(markers)] = shape(body)

    # For each marker position, does varying it alone ever change the shape?
    speaks, silent, groups = {}, {}, 0
    for (op, _, arity), by_markers in rows.items():
        if len(by_markers) < 2:
            continue
        groups += 1
        for pos in range(arity):
            fixed = {}
            for mk, h in by_markers.items():
                fixed.setdefault(mk[:pos] + mk[pos + 1:], set()).add((mk[pos], h))
            for others, seen in fixed.items():
                if len({m for m, _ in seen}) < 2:
                    continue          # only one instance present, nothing to compare
                names = "|".join(sorted({m for m, _ in seen}))
                key = (pos, names)
                if len({h for _, h in seen}) > 1:
                    speaks.setdefault(key, set()).add(op)
                else:
                    silent.setdefault(key, set()).add(op)

    print(f"{len(rows)} instantiation groups, {groups} comparable\n")
    print("axis position and instances        operations it changes / leaves identical")
    for key in sorted(set(speaks) | set(silent)):
        pos, names = key
        s, q = speaks.get(key, set()), silent.get(key, set())
        verdict = "" if s else "   <-- CHANGES NOTHING, ANYWHERE"
        print(f"  [{pos}] {names:<28} {len(s):>4} / {len(q):<4}{verdict}")
        if s and len(s) <= 6:
            print(f"        changes: {', '.join(sorted(s))}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
