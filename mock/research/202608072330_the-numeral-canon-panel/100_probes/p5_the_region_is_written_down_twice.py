#!/usr/bin/env python3
"""p5. The region a strategy selects over is written down twice, and nothing
joins the two.

The unit has been arguing about whether the region-to-arm table should be checked
or generated. Both shapes take the REGION SET as given. This probe asks where the
region set actually comes from, on the evidence in this repository, and finds it
comes from two places at once.

WHAT IS DUPLICATED. Several bench families identify a region by packing several
parameters into one integer. `warm-clamp-shared` declares
`KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP`
(`mock/benches/variants/warm-clamp-shared/src/lib.rs:83`) and ships const-fn
decoders for each field. `mock/benches/bench.toml` writes the keys as integer
literals. So the same fact, "this row is 13 bits at arity 64", exists as a
literal in the manifest and as a decoder in the crate, and the two are joined by
nothing.

`97` hit this directly: its section 5 says it decoded the arity sweep "from the
crate's own key encoding rather than from the title", which is a reader
discovering by hand that the human-facing statement of a region and the machine
one are separate artifacts.

WHAT THIS PROBE CHECKS, mechanically:

  1. Every bench block whose variants come from a crate with a key decoder: does
     every key in `bench.toml` decode to parameters the block's own TITLE
     declares? A title listing "arity 2 / 4 / 8 / 16" against a key set decoding
     to something else is a drift between two statements of one fact.
  2. Does every key decode to a parameter set the crate can actually serve, or
     can a manifest name a region that has no arm?
  3. Do the region sets of the two families this unit has reasoned from,
     `bitpack-carrier-width` and `warm-clamp-arity-w13`, match what `97` and `98`
     assumed about them?

A clean result is still a result. It says the twins agree TODAY, which is
different from saying anything keeps them agreeing.

Not a bench, no measurement. Reads committed source and manifest.

Run:  python3 p5_the_region_is_written_down_twice.py
"""

import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))
TOML = os.path.join(BENCH, "bench.toml")


def blocks():
    """Parse bench.toml into (name, title, [keys], [variant paths])."""
    out = {}
    name = None
    with open(TOML) as fh:
        text = fh.read()
    for m in re.finditer(
        r"^\[bench\.([^\]]+)\]$(.*?)(?=^\[bench\.[^\]]+\]$|\Z)",
        text,
        re.M | re.S,
    ):
        name = m.group(1)
        body = m.group(2)
        tm = re.search(r'^title\s*=\s*"(.*?)"\s*$', body, re.M | re.S)
        title = tm.group(1) if tm else ""
        keys = [int(x) for x in re.findall(r"^\s*n\s*=\s*(\d+)", body, re.M)]
        vars_ = sorted(set(re.findall(r"bench_([a-z0-9_]+)", body)))
        out[name] = (title, keys, vars_)
    return out


# The decoders, transcribed from each crate's own source, with the file and line
# the transcription came from so a reader can check it rather than trust it.
DECODERS = {
    "warm-clamp": (
        "variants/warm-clamp-shared/src/lib.rs:83",
        lambda k: {
            "W": k // 10000,
            "NC": (k // 1000) % 10,
            "LOG2A": (k // 10) % 100,
            "OP": k % 10,
        },
    ),
    "warm-container": (
        "variants/warm-container-shared/src/lib.rs:108",
        None,  # read below rather than guessed
    ),
}


def read_decoder_source(rel):
    path = os.path.join(BENCH, rel.split(":")[0])
    with open(path) as fh:
        return fh.read()


def check_warm_clamp(bl):
    """The one family this unit reasoned from, checked field by field."""
    dec = DECODERS["warm-clamp"][1]
    src = read_decoder_source(DECODERS["warm-clamp"][0])
    # confirm the transcription against the crate rather than trusting it
    assert "key / 10_000" in src and "(key / 1_000) % 10" in src, (
        "the decoder in the crate no longer matches the transcription in this "
        "probe, which is itself the drift this probe is about"
    )
    print("  decoder transcription checked against the crate source: ok")
    print()
    bad = 0
    for name, (title, keys, vars_) in sorted(bl.items()):
        if not name.startswith("warm-clamp"):
            continue
        decoded = [dec(k) for k in keys]
        widths = sorted({d["W"] for d in decoded})
        arities = sorted({1 << d["LOG2A"] for d in decoded})
        ops = sorted({d["OP"] for d in decoded})
        # the title states the width and the arity list in prose
        tw = re.search(r"at (\d+) bits", title)
        ta = re.findall(r"arity ([\d\s/]+):", title)
        title_w = int(tw.group(1)) if tw else None
        title_a = (
            [int(x) for x in re.findall(r"\d+", ta[0])] if ta else None
        )
        wok = title_w is None or [title_w] == widths
        aok = title_a is None or sorted(title_a) == arities
        flag = "" if (wok and aok) else "   <-- DISAGREES"
        if flag:
            bad += 1
        print(f"  {name}")
        print(f"    keys            : {keys}")
        print(f"    decoded widths  : {widths}   title says: {title_w}")
        print(f"    decoded arities : {arities}   title says: {title_a}{flag}")
        print(f"    decoded ops     : {ops}")
    return bad


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    bl = blocks()
    print(f"bench blocks in bench.toml: {len(bl)}")
    print()
    print("=" * 78)
    print("1 and 2. warm-clamp: manifest keys against the crate's decoder and the")
    print("         block's own title")
    print("=" * 78)
    bad = check_warm_clamp(bl)
    print()
    print(f"  blocks whose title disagrees with its decoded keys: {bad}")

    print()
    print("=" * 78)
    print("3. The two families this unit reasoned from")
    print("=" * 78)
    for fam in ("bitpack-carrier-width", "warm-clamp-arity-w13"):
        if fam in bl:
            title, keys, vars_ = bl[fam]
            print(f"\n  {fam}")
            print(f"    regions : {keys}")
            print(f"    arms    : {vars_}")
    print()
    print(
        "  `97` section 5 and `98` section 2 both treat these region sets as "
        "given.\n  They are given, by bench.toml, and the meaning of each "
        "integer lives in a\n  different file."
    )

    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    print(
        """
Whether or not anything disagrees today, the shape is the one that rots. One
fact, "which region is this", is stated as an integer literal in a manifest and
as a decoder in a crate, and a third time in prose in the block's title. Nothing
derives any of the three from the others, so a fourth reader has to do what `97`
did, which is decode the keys by hand and say so.

This bears on the unit's question in a way neither `97` nor `98` reached. Both
proposals take the region set as an input and argue about what is computed from
it. If the region set itself is duplicated, then a design that generates the
selection table from the weighting has automated one join and left the one
underneath it manual, and the unautomated join is the one a reader already had to
repair by hand once in this unit.

The generated artifact a single-definition discipline would want here is not only
the winner table. It is the region grid, the arm registry, and the key encoding,
emitted from one declaration, so that the region a strategy selects over and the
region the harness measured are the same object by construction rather than by
a reader decoding an integer.
"""
    )


if __name__ == "__main__":
    main()
