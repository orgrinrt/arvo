#!/usr/bin/env python3
"""PROBE 3: the build layer's first function, built.

The contract designed in `20_wingo...md` section 6 claims a downstream target
can recover a composition's full intent from a compiled arvo with no
cooperation from the type system, no metadata section, and no source access.
This is that reader, so the claim is a program rather than a paragraph.

It decodes the subset of the v0 mangling grammar the contract needs: nested
paths, generic-argument lists, and const arguments with their type tag. A real
implementation calls `rustc-demangle` and walks the parsed tree; this exists to
show the information is there and recoverable, and to be checkable by hand
against `llvm-nm` output.

Usage:  ./02_run.sh producing /tmp/p2v0.o, then
        llvm-nm /tmp/p2v0.o | ./03_the_build_layer_reader.py
"""
import re
import sys

# v0 const-argument type tags that arvo's axes can use as const generics.
INT_TAG = {'h': 'u8', 't': 'u16', 'm': 'u32', 'y': 'u64', 'a': 'i8',
           's': 'i16', 'l': 'i32', 'x': 'i64', 'j': 'usize', 'b': 'bool'}


def decode_args(s):
    """Decode one v0 generic-argument list into (const values, type names)."""
    consts, types, i = [], [], 0
    while i < len(s):
        c = s[i]
        if c == 'K':                                   # const argument
            tag = s[i + 1]
            j = s.index('_', i + 2)
            raw = s[i + 2:j]
            consts.append((INT_TAG.get(tag, tag), int(raw, 16) if raw else 0))
            i = j + 1
        elif c == 'N':                                 # nested path -> a name
            m = re.match(r'N[a-z](?:B[0-9a-zA-Z]*_)?(\d+)([A-Za-z_][A-Za-z0-9_]*)',
                         s[i:])
            if not m:
                i += 1
                continue
            n = int(m.group(1))
            types.append(m.group(2)[:n])
            i += m.end() - len(m.group(2)) + n
        else:
            i += 1
    return consts, types


def main():
    rows = []
    for line in sys.stdin:
        m = re.search(r'(__?RI[A-Za-z0-9_$.]+)', line)
        if not m:
            continue
        sym = m.group(1)
        fn = re.search(r'(\d+)([a-z_][a-z0-9_]*)E?', sym)
        body = sym[sym.index('4sum4') + 5:] if '4sum4' in sym else sym
        consts, types = decode_args(body.split('E')[0])
        rows.append((consts, types, sym))

    if not rows:
        print("no v0 generic symbols on stdin")
        return
    print(f"{'I':>4}{'F':>5}   {'Policy':<10}{'Lowering':<12}")
    for consts, types, sym in sorted(rows):
        vals = [v for _, v in consts]
        i = vals[0] if len(vals) > 0 else '?'
        f = vals[1] if len(vals) > 1 else '?'
        p = types[0] if len(types) > 0 else '?'
        l = types[1] if len(types) > 1 else '?'
        print(f"{i:>4}{f:>5}   {p:<10}{l:<12}")
    print()
    print(f"{len(rows)} instantiations recovered from the symbol table alone.")


if __name__ == '__main__':
    main()
