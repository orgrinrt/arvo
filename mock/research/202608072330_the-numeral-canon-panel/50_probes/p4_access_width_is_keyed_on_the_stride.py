#!/usr/bin/env python3
# p4: is the packed access width a function of the declared width alone?
#
# 16:186-189 says yes, and dismisses the access width as an output on that basis:
#
#   "the maximum byte span of a W bit field at unknown phase is floor((W + 6) / 8) + 1, a function
#    of W alone"
#
# and 16:179-180 gives the reason it believes the phase is unknown:
#
#   "Its phase within a byte is 13k mod 8, and since thirteen and eight are coprime the phase
#    cycles through all eight residues."
#
# Thirteen and eight are coprime. Twelve and eight are not. The phase set of a packed run with
# stride s is exactly the multiples of gcd(s, 8), which is all eight residues only when s is odd.
# So the maximum span is a function of (W, gcd(stride, 8)), and the closed form is the worst case
# over a phase set that is often larger than the one that actually occurs.
#
# That does not make the closed form unsafe. It makes it an OVER-estimate, and it makes the exact
# quantity keyed on the stride, which is a strategy's choice. This file counts how often.
#
# Everything below is brute-forced over every phase that actually occurs, and the closed form is
# checked against the brute force rather than assumed.
#
#   python3 p4_access_width_is_keyed_on_the_stride.py

from math import gcd

WMAX = 128


def span_bytes(width, phase):
    """bytes touched by a field of `width` bits starting at bit offset `phase`."""
    return (phase + width - 1) // 8 - phase // 8 + 1


def worst_over(width, phases):
    return max(span_bytes(width, p) for p in phases)


def all_phases():
    return range(8)


def phases_for(stride):
    """the byte phases an element of a packed run at `stride` bits can actually land on."""
    g = gcd(stride, 8)
    return sorted({(stride * k) % 8 for k in range(8 // g)})


def closed_form(width):
    """16:187's closed form, in bytes."""
    return (width + 6) // 8 + 1


def pow2_up(n):
    p = 1
    while p < n:
        p *= 2
    return p


def main():
    # 0. the closed form IS the worst case over all eight phases. check it rather than trust it.
    mismatches = [w for w in range(1, 1025) if closed_form(w) != worst_over(w, all_phases())]
    print("closed form floor((W+6)/8)+1 against brute force over all 8 phases, W=1..1024:")
    print("  mismatches: %d" % len(mismatches))
    print("  so 16's arithmetic is right about what it computes. the question is what it quantifies over.")
    print()

    # 1. Cold: stride == W. how often is the actual phase set smaller than all eight?
    rows = []
    for w in range(1, WMAX + 1):
        ph = phases_for(w)
        true_span = worst_over(w, ph)
        cf = closed_form(w)
        rows.append((w, len(ph), true_span, cf))
    over = [r for r in rows if r[3] > r[2]]
    print("Cold (stride == W), widths 1..%d:" % WMAX)
    print("  widths whose packed run reaches all 8 byte phases : %d"
          % sum(1 for r in rows if r[1] == 8))
    print("  widths where the closed form OVER-estimates       : %d of %d" % (len(over), WMAX))
    print("  first ten of them (W, phases, true bytes, closed form):")
    for r in over[:10]:
        print("     W=%-4d phases=%-2d true=%-2d closed=%-2d" % r)
    print()

    # 2. the same, as load types (rounded up to a power of two), which is what 47_probes/p6 keys on
    over_t = [(w, pow2_up(t), pow2_up(c)) for (w, _, t, c) in rows if pow2_up(t) != pow2_up(c)]
    print("  as a LOAD TYPE (span rounded up to a power of two):")
    print("     widths where the closed form picks a wider load type than needed: %d of %d"
          % (len(over_t), WMAX))
    for r in over_t[:10]:
        print("        W=%-4d true=%d bytes  closed=%d bytes" % r)
    print()

    # 3. the same width under two packing disciplines, which is the point: same W, different answer
    def grid_stride(w, g):
        return -(-w // g) * g

    print("the exact access width is NOT a function of W alone once a second packer exists:")
    print("  W    Cold(stride=W)          Grid4(stride=ceil(W/4)*4)   16's closed form")
    disagree = 0
    for w in range(1, WMAX + 1):
        c = worst_over(w, phases_for(w))
        g = worst_over(w, phases_for(grid_stride(w, 4)))
        if c != g:
            disagree += 1
        if w in (5, 12, 13, 17, 23, 25, 47, 57):
            print("  %-4d %-24s %-27s %d"
                  % (w, "%d bytes (phases %d)" % (c, len(phases_for(w))),
                     "%d bytes (phases %d)" % (g, len(phases_for(grid_stride(w, 4)))),
                     closed_form(w)))
    print("  widths where Cold and Grid4 need different access spans: %d of %d" % (disagree, WMAX))
    print()

    # 4. what this does to 16's own 28-of-64 figure and to 47_probes/p6's jump points
    def native_rung_bytes(w):
        for b in (1, 2, 4, 8, 16):
            if w <= b * 8:
                return b
        return 32

    carrier_wrong_closed = sum(
        1 for w in range(1, 65) if native_rung_bytes(w) != pow2_up(closed_form(w)))
    carrier_wrong_true = sum(
        1 for w in range(1, 65)
        if native_rung_bytes(w) != pow2_up(worst_over(w, phases_for(w))))
    print("16:384-386 reports the carrier is the wrong load type at 28 of 64 widths.")
    print("  recomputed from the closed form                : %d of 64" % carrier_wrong_closed)
    print("  recomputed from the phases a Cold run reaches  : %d of 64" % carrier_wrong_true)
    print()

    def jumps(f):
        return [w for w in range(2, WMAX + 1) if f(w) != f(w - 1)]

    j_native = jumps(native_rung_bytes)
    j_closed = jumps(lambda w: pow2_up(closed_form(w)))
    j_true = jumps(lambda w: pow2_up(worst_over(w, phases_for(w))))
    print("47_probes/p6 reports zero shared jump points between the native and access ladders.")
    print("  native ladder jumps at              : %s" % j_native)
    print("  access ladder from the closed form  : %s" % j_closed)
    print("  access ladder from the true phases  : %s" % j_true)
    print("  shared with native, closed form     : %s" % sorted(set(j_native) & set(j_closed)))
    print("  shared with native, true phases     : %s" % sorted(set(j_native) & set(j_true)))
    print("  classes: native %d, closed %d, true %d"
          % (len(j_native) + 1, len(j_closed) + 1, len(j_true) + 1))


if __name__ == "__main__":
    main()
