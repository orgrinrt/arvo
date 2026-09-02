#!/usr/bin/env python3
"""
p7. Fills the one blank cell p1 left: the pattern-level operation at the index-2 crossing.

WHY IT WAS BLANK, AND WHY THAT WAS THE WRONG ANSWER.

p1 left the cell unmeasured because a widening changes the container width, so "the container's
own operation" is a different function on each side and the comparison needs a convention. That
was a reason to STATE a convention, not a reason to leave a hole, and leaving it produced a
table with a gap exactly where the question "is the crossing one relation or several" is decided.

THE CONVENTION, stated so it can be disagreed with rather than assumed: each side's pattern-level
operation is the operation its OWN container provides, which for a width-w container is addition
modulo 2^w. That is the only convention under which the comparison is about the crossing rather
than about a chosen arithmetic, because it is what a consumer gets by operating on the bits of
each side without consulting the type.

PREDICTION, in three parts, and the third was refuted. Kept as written with the refutation
below rather than rewritten to match the output.

  (a) under two's complement the pattern-level number equals the value-level number (192/256
      under wrap), because two's complement's raw adder IS the value operation for a wrapping
      system;
  (b) under excess-K it does not, because excess-K's raw adder carries the constant defect
      `63` section 3.5 measured;
  (c) under a saturating system the two levels part company, because saturation has no
      raw-adder realisation at all.

(a) and (b) held. (c) is FALSE and the reason is the better finding: the pattern-level operation
is the container's modular addition whatever policy the type carries, so it does not depend on
the reduction at all. The probe asserts that independence below rather than reporting it.
"""

W_SRC, W_DST = 4, 8
M_SRC, M_DST = 1 << W_SRC, 1 << W_DST

SIGNED_SRC = list(range(-8, 8))
UNSIGNED_SRC = list(range(0, 16))


def wrap(v, lo, n):
    return ((v - lo) % n) + lo


def saturate(v, lo, hi):
    return lo if v < lo else (hi if v > hi else v)


def enc_twos(v, w):
    return v & ((1 << w) - 1)


def dec_twos(p, w):
    return p - (1 << w) if p >= (1 << (w - 1)) else p


def enc_excess(v, w):
    return v + (1 << (w - 1))


def dec_excess(p, w):
    return p - (1 << (w - 1))


ENCODINGS = {"twos_complement": (enc_twos, dec_twos), "excess_K": (enc_excess, dec_excess)}


def raw_add(x, y, w):
    """The container's own operation: addition modulo 2^w. The stated convention."""
    return (x + y) & ((1 << w) - 1)


def measure(enc_name, policy):
    enc, dec = ENCODINGS[enc_name]
    lo, hi = -8, 7
    lo_d, hi_d = -128, 127

    def op_src(a, b):
        return wrap(a + b, lo, 16) if policy == "wrap" else saturate(a + b, lo, hi)

    def op_dst(a, b):
        return wrap(a + b, lo_d, 256) if policy == "wrap" else saturate(a + b, lo_d, hi_d)

    # value level: the inclusion, which is the identity on the carrier
    v_agree = sum(1 for a in SIGNED_SRC for b in SIGNED_SRC if op_src(a, b) == op_dst(a, b))

    # pattern level: the crossing is the widening of the encoded pattern, each side operating
    # with its own container's modular addition
    def cross(x):
        return enc(dec(x, W_SRC), W_DST)

    p_agree = 0
    for a in SIGNED_SRC:
        for b in SIGNED_SRC:
            xa, xb = enc(a, W_SRC), enc(b, W_SRC)
            left = cross(raw_add(xa, xb, W_SRC))
            right = raw_add(cross(xa), cross(xb), W_DST)
            if left == right:
                p_agree += 1
    return v_agree, p_agree


print("=" * 90)
print("INDEX 2, THE WIDENING CROSSING: value level against pattern level")
print("Convention: each side's pattern operation is addition modulo its own container width.")
print("=" * 90)
print(f"{'encoding':<20}{'policy':<12}{'VALUE ops':<16}{'PATTERN ops':<16}{'levels agree'}")

rows = []
for enc_name in ENCODINGS:
    for policy in ("wrap", "saturate"):
        v, p = measure(enc_name, policy)
        rows.append((enc_name, policy, v, p))
        print(f"{enc_name:<20}{policy:<12}{f'{v}/256':<16}{f'{p}/256':<16}"
              f"{'yes' if v == p else 'NO'}")

print()
tw_wrap = next(r for r in rows if r[0] == "twos_complement" and r[1] == "wrap")
ex_wrap = next(r for r in rows if r[0] == "excess_K" and r[1] == "wrap")
tw_sat = next(r for r in rows if r[0] == "twos_complement" and r[1] == "saturate")

print("READING")
print("-" * 90)

by_enc = {}
for enc_name, policy, v, p in rows:
    by_enc.setdefault(enc_name, set()).add(p)
policy_blind = all(len(v) == 1 for v in by_enc.values())
assert policy_blind, "the pattern level turned out to depend on the policy after all"

tw = next(r for r in rows if r[0] == "twos_complement" and r[1] == "wrap")
ex = next(r for r in rows if r[0] == "excess_K" and r[1] == "wrap")

print(f"  The blank cell has a value: {tw[3]}/256 under two's complement, {ex[3]}/256 under")
print("  excess-K, and it is the same under either policy in both encodings.")
print()
print("  PART (c) OF THE PREDICTION IS REFUTED, and this is why the cell was worth filling.")
print("  The pattern-level number is IDENTICAL across wrap and saturate for a fixed encoding,")
print("  asserted above rather than eyeballed. The container's own operation is modular")
print("  addition whatever the type says, so the pattern level CANNOT SEE THE REDUCTION.")
print()
print("  That is the same fact p1's index-3 row reports from the other side: restrategising")
print(f"  preserves the pattern-level operation at 256/256 precisely because the pattern level")
print("  is blind to the coordinate being moved. Two instruments, one structural fact.")
print()
print(f"  PARTS (a) AND (b) HELD. Under two's complement the levels coincide at {tw[2]}/256")
print(f"  against {tw[3]}/256; under excess-K they part at {ex[2]}/256 against {ex[3]}/256.")
print("  Nothing about the crossing changed between those two rows. The ENCODING did, and the")
print("  encoding is a coordinate this crossing does not move.")
print()
print("  So the correction to p1's table is not one number. The PATTERN column is not")
print("  determined by the index at all: it is determined by the coordinates both sides carry,")
print("  and it is blind to the reduction. p1's table read as though the index fixed both")
print("  columns, and it fixes only the first.")
