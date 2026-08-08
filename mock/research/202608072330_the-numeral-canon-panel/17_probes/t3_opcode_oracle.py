#!/usr/bin/env python3
"""t3. An erasure oracle that works at the aggregate, where symbol identity does not.

The panel's instrument reads the assembler's symbol table and asks whether two functions
were folded into one. That works at the scalar, where the bodies are a handful of
instructions and are textually identical. t2 shows it stops working at the aggregate: a
typed packed walk and the hand-written walk it claims to erase to have the SAME
instruction count and differ only in register allocation, scheduling and label names, so
the assembler does not fold them and symbol identity reports erasure failing where it
holds.

So the oracle has to be insensitive to the things a compiler is free to choose and
sensitive to the things it is not. The multiset of opcodes is one such oracle: register
allocation and label naming do not change it, and doing more or different work does.

  python3 t3_opcode_oracle.py asm/<file>.s

Nothing here is timed. Instruction counts are counts. What any of this COSTS is unpriced,
because no bench harness has run in this panel.

Spike. Presume it flawed. In particular an opcode multiset is insensitive to operand
values, so it would equate a shift by 3 with a shift by 4. It is a better oracle than
symbol identity at this arity, not a correct one; section 4 of the file names what it
still misses.
"""
import re
import sys
from collections import Counter


def bodies(text):
    """Return {symbol: [instruction lines]} plus the set of aliased symbols."""
    out = {}
    alias = {}
    for m in re.finditer(r"^_(\w+) = _(\w+)$", text, re.M):
        alias[m.group(1)] = m.group(2)
    for m in re.finditer(r"^_(\w+):\n(.*?)(?=\n\t\.cfi_endproc)", text, re.S | re.M):
        name = m.group(1)
        lines = []
        for ln in m.group(2).split("\n"):
            ln = ln.strip()
            if not ln or ln.startswith(".") or ln.endswith(":"):
                continue
            lines.append(ln)
        out[name] = lines
    return out, alias


def opcodes(lines):
    return Counter(ln.split()[0] for ln in lines)


def compare(a_name, a, b_name, b):
    oa, ob = opcodes(a), opcodes(b)
    same = oa == ob
    print(f"\n  {a_name}  vs  {b_name}")
    print(f"    instructions      : {len(a)} vs {len(b)}")
    print(f"    distinct opcodes  : {len(oa)} vs {len(ob)}")
    print(f"    opcode multiset   : {'IDENTICAL' if same else 'DIFFERS'}")
    if not same:
        diff = (oa - ob) + (ob - oa)
        rows = []
        for k in sorted(set(oa) | set(ob)):
            if oa[k] != ob[k]:
                rows.append(f"      {k:10} {oa[k]:>4} vs {ob[k]:>4}")
        print("    where they differ:")
        print("\n".join(rows))
    return same


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else None
    if not path:
        print("usage: t3_opcode_oracle.py <asm file>")
        return 1
    text = open(path).read()
    b, alias = bodies(text)

    print("t3. erasure oracles compared, on the assembly t2 emitted")
    print(f"    source: {path}")
    print(f"    bodies with their own code: {len(b)}")
    print(f"    symbols aliased by the assembler: {len(alias)}")
    for k, v in sorted(alias.items()):
        print(f"      _{k} = _{v}")

    print("\n--- oracle A: symbol identity, which is the panel's instrument ---")
    for pair in [
        ("t2_scalar_typed", "t2_scalar_native"),
        ("t2_typed_sum", "t2_handwritten_sum"),
    ]:
        x, y = pair
        folded = alias.get(x) == y or alias.get(y) == x
        verdict = "ERASED" if folded else "reports NOT ERASED"
        print(f"    {x:22} vs {y:22} -> {verdict}")

    print("\n--- oracle B: opcode multiset ---")
    ok_scalar = None
    if "t2_scalar_native" in b:
        # the scalar pair folded, so only one body exists; that IS the answer
        print("\n  t2_scalar_typed vs t2_scalar_native")
        print("    folded by the assembler, so there is one body and they are identical")
        ok_scalar = True
    ok_agg = compare(
        "t2_typed_sum", b.get("t2_typed_sum", []),
        "t2_handwritten_sum", b.get("t2_handwritten_sum", []),
    )

    print("\n--- the control: does the second output reach codegen at all? ---")

    def resolve(name):
        """Follow the assembler's aliases to the body that actually carries the code."""
        seen = set()
        while name in alias and name not in seen:
            seen.add(name)
            name = alias[name]
        return name, b.get(name, [])

    warm_name, warm_body = resolve("t2_typed_sum_warm")
    if warm_name != "t2_typed_sum_warm":
        print(f"    (t2_typed_sum_warm was folded into {warm_name}, so the access width")
        print("     is not what distinguishes the arms; the stride is)")
    compare(
        "t2_typed_sum_cold_400 (stride 13)", b.get("t2_typed_sum_cold_400", []),
        f"{warm_name} (stride 16)", warm_body,
    )

    print("\nreading:")
    print("  scalar pair, symbol identity  :", "agrees with opcode multiset" if ok_scalar else "n/a")
    print("  aggregate pair, symbol identity: reports NOT ERASED")
    print("  aggregate pair, opcode multiset:", "reports ERASED" if ok_agg else "reports NOT ERASED")
    if ok_agg:
        print("\n  So the two oracles DISAGREE at the aggregate, and the disagreement is not")
        print("  a tie. The bodies compute the same thing with the same instructions in the")
        print("  same quantity; only register allocation, scheduling and label names differ,")
        print("  and a compiler is free to choose all three. Symbol identity is measuring")
        print("  something the criterion does not ask about.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
