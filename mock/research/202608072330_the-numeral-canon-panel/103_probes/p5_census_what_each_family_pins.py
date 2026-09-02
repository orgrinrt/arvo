#!/usr/bin/env python3
"""p5. Per-family census: does the family pin ONE answer, or pin EACH arm to
its own semantics, or pin nothing?

`102`'s instrument counted, per shared crate, whether `outputs_may_differ` was
declared and whether a `validate_output` exists, and read the presence of a
validator plus the default flag as establishing answer-equivalence. That
conflates three different things, and this census separates them.

The distinction that matters:

**Answer-pinning.** The validator computes a reference VALUE and refuses any
output not equal to it. Every arm that passes computes the same value, so the
family is answer-equivalent and its numbers compare cost at a fixed answer.

**Arm-pinning.** Each arm is checked against ITS OWN declared semantics, with
no requirement that the arms agree. The family is correct arm by arm and its
arms may compute different values.

**Property-pinning only.** The validator checks a range, a shape, or a bound.
It refuses a broken output and admits a family of different correct ones.

The classifier is structural rather than lexical wherever it can be. An
exact-value oracle shows up as a comparison of `output.<field>` against a value
the validator itself computed from the input, and this looks for that shape.
The result is reported per family with the evidence line, so a reader can check
the classification rather than trust it.

The `outputs_may_differ` and `max_relative_error` declarations are reported
alongside, because they are what the harness reads, and a family whose code
pins each arm separately while its bridge flags say "all arms must agree
byte-exactly" is a family whose declaration and whose intent disagree. That
disagreement is the thing worth finding.

Run from anywhere; paths are resolved relative to this file.
"""

import os
import re
import sys

FAMILIES = [
    "bitpack-carrier-shared",
    "bitpack-contend-shared",
    "bitpack-footprint-shared",
    "bitpack-plan-shared",
    "bitpack-shared",
    "bitpack-wide-shared",
    "bitpack-write-contend-shared",
    "quantiser-fadd-shared",
    "quantiser-radix-shared",
    "satfold-shared",
    "warm-clamp-shared",
    "warm-container-shared",
    "wide-rung-shared",
]


def read_sources(root, fam):
    src = os.path.join(root, fam, "src")
    out = {}
    if not os.path.isdir(src):
        return out
    for fn in sorted(os.listdir(src)):
        if fn.endswith(".rs"):
            with open(os.path.join(src, fn)) as fh:
                out[fn] = fh.read()
    return out


def validator_bodies(text):
    """Every `fn validate_output` body, by brace matching from the signature."""
    bodies = []
    for m in re.finditer(r"fn validate_output\s*\(", text):
        i = text.find("{", m.end())
        if i < 0:
            continue
        depth = 0
        j = i
        while j < len(text):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        bodies.append(text[i : j + 1])
    return bodies


# An exact-value refusal: a comparison between an `output.` field and something
# else, guarding a `return Err`. Both orders occur in the corpus.
EXACT_PATTERNS = [
    r"output\.\w+\s*!=\s*\w",
    r"\w[\w:().\[\]&\s]*!=\s*output\.\w+",
    r"output\.\w+\s*==\s*\w",
]

# A property refusal: a magnitude/shape bound on an output element, with no
# reference value anywhere in the body.
PROPERTY_PATTERNS = [
    r"output\.\w+\[\w+\]\s*[<>]=?\s*",
    r"output\.\w+\[\w+\]\.is_nan\(\)",
    r"\.abs\(\)\s*>\s*",
    r"seen\[",
]


def classify(bodies):
    if not bodies:
        return "NO VALIDATOR", ""
    joined = "\n".join(bodies)
    exact_hits = []
    for p in EXACT_PATTERNS:
        for m in re.finditer(p, joined):
            frag = m.group(0).strip()
            # a comparison against a literal 0 or a length is a shape check
            if re.search(r"!=\s*0\s*$|==\s*0\s*$", frag):
                continue
            exact_hits.append(frag)
    prop_hits = []
    for p in PROPERTY_PATTERNS:
        for m in re.finditer(p, joined):
            prop_hits.append(m.group(0).strip())

    if exact_hits:
        return "ANSWER-PINNING", exact_hits[0]
    if prop_hits:
        return "PROPERTY-PINNING ONLY", prop_hits[0]
    return "UNCLASSIFIED", ""


def flags(srcs):
    joined = "\n".join(srcs.values())
    omd = "false (default)"
    m = re.search(r"fn outputs_may_differ\s*\([^)]*\)\s*->\s*bool\s*\{([^}]*)\}", joined)
    if m:
        v = m.group(1).strip()
        omd = v + " (declared)"
    mre = "None (default)"
    m = re.search(
        r"fn max_relative_error\s*\([^)]*\)\s*->\s*Option<f64>\s*\{([^}]*)\}", joined
    )
    if m:
        mre = m.group(1).strip() + " (declared)"
    return omd, mre


def per_arm_oracles(srcs):
    """Tests that check ONE arm against an oracle that is not the other arm.

    The tell is a test whose body names exactly one of the family's run
    functions and compares against something computed independently.
    """
    joined = "\n".join(srcs.values())
    names = re.findall(r"fn (\w+)\s*\([^)]*\)\s*\{", joined)
    return sorted(
        set(
            n
            for n in names
            if re.search(r"matches_the_silicon|nearest_grid|matches_hardware|delivers_the", n)
        )
    )


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    root = os.path.abspath(os.path.join(here, "..", "..", "..", "benches", "variants"))
    if not os.path.isdir(root):
        print("variants directory not found at", root)
        return 1

    print("p5. per-family census: what does each committed family actually pin?")
    print()
    print("root:", root)
    print()

    hdr = f"{'family':<30} {'validator kind':<24} {'outputs_may_differ':<20} {'max_rel_err':<18}"
    print(hdr)
    print("-" * len(hdr))

    kinds = {}
    for fam in FAMILIES:
        srcs = read_sources(root, fam)
        bodies = []
        for text in srcs.values():
            bodies.extend(validator_bodies(text))
        kind, ev = classify(bodies)
        omd, mre = flags(srcs)
        kinds[fam] = (kind, ev, omd, mre, srcs)
        print(f"{fam:<30} {kind:<24} {omd:<20} {mre:<18}")

    print()
    print("EVIDENCE LINE PER FAMILY (the fragment the classification rests on)")
    for fam, (kind, ev, _, _, _) in kinds.items():
        print(f"  {fam:<30} {kind:<24} {ev!r}")

    print()
    print("FAMILIES WHOSE ARMS ARE NOT REQUIRED TO AGREE ON A VALUE")
    loose = [f for f, (k, _, _, _, _) in kinds.items() if k != "ANSWER-PINNING"]
    for f in loose:
        print("   ", f)
    print(f"  count: {len(loose)} of {len(FAMILIES)}")

    print()
    print("PER-ARM ORACLES: tests validating ONE arm against its own semantics")
    for fam, (_, _, _, _, srcs) in kinds.items():
        names = per_arm_oracles(srcs)
        if names:
            print(f"  {fam:<30} {names}")

    print()
    print("THE DECLARATION MISMATCH")
    print("  A family classified PROPERTY-PINNING ONLY whose bridge declares")
    print("  outputs_may_differ = false is telling the harness its arms must")
    print("  agree byte-for-byte while its own validator does not require it.")
    for fam in loose:
        kind, _, omd, mre, _ = kinds[fam]
        if omd.startswith("false"):
            print(f"    {fam:<30} {kind}  with outputs_may_differ {omd}")

    print()
    print("READING")
    print("  ANSWER-PINNING is the only classification that supports the claim")
    print("  that a family's numbers compare cost at a fixed answer. A family")
    print("  that pins only a property admits arms computing different values,")
    print("  and whether they do is a separate measurement (p2, p3, p4).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
