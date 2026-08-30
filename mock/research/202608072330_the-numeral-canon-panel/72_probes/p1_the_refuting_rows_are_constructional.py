#!/usr/bin/env python3
"""p1. The two rows that refute `67`'s universal cannot fail, and the question that can.

`71` section 1 refutes `67:252` ("No crossing preserves operations at 100%") with two rows
`67` did not measure: at telescope index 4 (re-encoding) and index 5 (re-housing) the
value-level operation is preserved at 256/256. I accept the refutation. This probe is about
its evidence, not its conclusion.

CLAIM UNDER TEST, written before the run: those two rows are TRUE BY CONSTRUCTION and cannot
fail, because `71_probes/p1_five_crossings.py`'s `System.add` is
`rho(dom_add(a, b), q)` and reads neither the encoding nor the housing offset. With the value
map the identity and the two systems agreeing on (D, Q, rho), `measure_value_ops` compares a
computation with itself.

PREDICTION: give the index-4 target a deliberately BROKEN encoding, one that is not even
injective, and the row still reads 256/256. If it does, the check does not read the encoding
and its number is not a measurement.

That matters in two directions and neither of them is a quibble.

  It STRENGTHENS `71`'s X2 ("the encoding and the container can never change what a crossing
  computes"). A row that cannot fail licenses "never"; a 4-bit sweep would license only "does
  not, at four bits".

  It removes a number from circulation. "256/256" invites a later reader to quote a
  measurement where the honest citation is "by construction, at any width, in any language".

Then the constructive half. There IS a question at index 4 that can fail, and it is the one a
consumer feels: the operation you get by operating on the STORED BITS and reading the result
back as a value. That is `63` section 3.5's raw-adder property asked per encoding rather than
per crossing, and it is where a re-encoding's cost actually lands.

Run: python3 p1_the_refuting_rows_are_constructional.py
"""

W = 4
M = 1 << W
SIGNED4 = list(range(-8, 8))
UNSIGNED4 = list(range(0, 16))

out = []


def say(s=""):
    out.append(s)
    print(s)


def wrap(v, q):
    n = len(q)
    return ((v - q[0]) % n) + q[0]


def saturate(v, q):
    return q[0] if v < q[0] else (q[-1] if v > q[-1] else v)


# ---------------------------------------------------------------- encodings, index 4

def enc_twos(v):
    return v & (M - 1)


def dec_twos(p):
    return p - M if p >= (M >> 1) else p


def enc_excess(v):
    return v + 8


def dec_excess(p):
    return p - 8


def enc_broken(v):
    """Deliberately not an encoding at all: every value onto one pattern."""
    return 0


def dec_broken(p):
    return 0


ENCODINGS = {
    "twos_complement": (enc_twos, dec_twos),
    "excess_8": (enc_excess, dec_excess),
    "BROKEN_constant": (enc_broken, dec_broken),
}


class System:
    """A completed telescope term, in `71`'s shape so the comparison is like for like."""

    def __init__(self, q, rho, enc_name, offset=0):
        self.q = q
        self.rho = rho
        self.enc, self.dec = ENCODINGS[enc_name]
        self.enc_name = enc_name
        self.offset = offset

    def add(self, a, b):
        return self.rho(a + b, self.q)

    def store(self, v):
        return (self.enc(v) & (M - 1)) << self.offset

    def load(self, byte):
        return self.dec((byte >> self.offset) & (M - 1))


def measure_value_ops(src, dst, vmap):
    """`71`'s own check, transcribed: vmap(a op_src b) == vmap(a) op_dst vmap(b)."""
    agree = 0
    total = 0
    for a in src.q:
        for b in src.q:
            total += 1
            if vmap(src.add(a, b)) == dst.add(vmap(a), vmap(b)):
                agree += 1
    return agree, total


say("p1. the two refuting rows are constructional, and the question that can fail")
say("=" * 78)

# --------------------------------------------------------- part one: the mutant
say()
say("PART ONE. Mutate the index-4 target's encoding and watch the row not move.")
say()

src = System(SIGNED4, saturate, "twos_complement")
for enc_name in ("twos_complement", "excess_8", "BROKEN_constant"):
    dst = System(SIGNED4, saturate, enc_name)
    a, t = measure_value_ops(src, dst, lambda v: v)
    say(f"  index-4 VALUE ops, target encoding {enc_name:16s}: {a}/{t}")

say()
say("  The third row's target encoding maps all sixteen values onto one pattern and is not")
say("  injective, not surjective, and not an encoding of anything. The check does not notice,")
say("  because it never calls it.")

# index 5, the same demonstration on the housing
say()
say("  And at index 5, the same, with the housing offset swept:")
for off in (0, 1, 2, 4):
    dst = System(SIGNED4, saturate, "twos_complement", offset=off)
    a, t = measure_value_ops(src, dst, lambda v: v)
    say(f"    index-5 VALUE ops, target offset {off}: {a}/{t}")

say()
say("  VERDICT: both rows are true by construction. `71`'s refutation of `67:252` STANDS,")
say("  and its evidence is a structural fact rather than a 4-bit measurement. Cite it as")
say("  'by construction, at any width' and the word 'never' in `71`'s X2 is licensed.")

# ------------------------------------------------ part two: the question that can fail
say()
say("=" * 78)
say("PART TWO. The index-4 question a consumer feels: operate on the stored bits, read back.")
say()
say("  via_patterns(a, b) := load(raw_add(store a, store b)), where raw_add is the")
say("  container's own wrapping add on the four stored bits. Compared against the system's")
say("  own value-level add. This CAN fail and does.")
say()


def raw_add(pa, pb):
    return (pa + pb) & (M - 1)


# Encodings are per-window, because an encoding is a realisation of ONE representable set.
# A FIRST VERSION OF THIS PART GOT THAT WRONG and is kept here as the record: it applied the
# signed two's-complement decoder to the unsigned window, which is not an encoding of that
# window at all, and produced 128/256 and 36/256 rows that meant nothing. Same class of defect
# as a setup that helps, pointing the other way.

PER_WINDOW = {
    "signed": {
        "natural (two's complement)": (lambda v: v & (M - 1),
                                       lambda p: p - M if p >= (M >> 1) else p),
        "biased (excess-8)": (lambda v: v + 8, lambda p: p - 8),
    },
    "unsigned": {
        "natural (identity)": (lambda v: v, lambda p: p),
        "biased (rotate-8)": (lambda v: (v + 8) & (M - 1), lambda p: (p - 8) & (M - 1)),
    },
}


class Sys2:
    def __init__(self, q, rho, enc, dec):
        self.q, self.rho, self.enc, self.dec = q, rho, enc, dec

    def add(self, a, b):
        return self.rho(a + b, self.q)

    def store(self, v):
        return self.enc(v) & (M - 1)

    def load(self, p):
        return self.dec(p & (M - 1))


hdr = f"  {'window':10s} {'policy':10s} {'encoding':26s} {'via-patterns == value add':>26s}"
say(hdr)
say("  " + "-" * (len(hdr) - 2))

rows = []
for wname, q in (("signed", SIGNED4), ("unsigned", UNSIGNED4)):
    for pname, rho in (("wrap", wrap), ("saturate", saturate)):
        for enc_name, (e, d) in PER_WINDOW[wname].items():
            s2 = Sys2(q, rho, e, d)
            agree = 0
            total = 0
            for a in q:
                for b in q:
                    total += 1
                    if s2.load(raw_add(s2.store(a), s2.store(b))) == s2.add(a, b):
                        agree += 1
            rows.append((wname, pname, enc_name, agree, total))
            say(f"  {wname:10s} {pname:10s} {enc_name:26s} {agree:>15d}/{total}")

say()
say("  So a re-encoding cannot change what the crossing computes on VALUES, and it can and")
say("  does change what the same bits compute when a consumer adds them without consulting")
say("  the type. Those are two different sentences about index 4 and the panel has been")
say("  writing one of them.")

nat = [r for r in rows if r[2].startswith("natural") and r[1] == "wrap"]
bia = [r for r in rows if r[2].startswith("biased") and r[1] == "wrap"]
say()
say(f"  under wrapping the natural encoding agrees at {nat[0][3]}/{nat[0][4]} signed and "
    f"{nat[1][3]}/{nat[1][4]} unsigned;")
say(f"  the biased one at {bia[0][3]}/{bia[0][4]} and {bia[1][3]}/{bia[1][4]}.")
say("  That is `63` section 3.5's constant defect, arriving as a number a consumer can feel")
say("  rather than as a property of a crossing square: a biased encoding forfeits the raw")
say("  adder totally, at every operand pair, in both sign domains.")

say()
say("  What this does not establish: any magnitude, anything past four bits, and nothing")
say("  about whether either encoding should be preferred. It replaces one vacuous row with")
say("  one that can fail.")

with open("p1_the_refuting_rows_are_constructional.out", "w") as f:
    f.write("\n".join(out) + "\n")
