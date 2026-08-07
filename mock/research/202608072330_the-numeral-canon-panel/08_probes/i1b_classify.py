#!/usr/bin/env python3
"""i1b: the classifier, with three defects in i1 repaired.

i1 is kept unchanged and its output kept as `i1.out`, because what it got wrong
is worth more than what it got right.

DEFECT 1, and it is the interesting one. i1 tested `is every value a multiple
of r^fexp(e)`, which is Flocq's generic_format and is NOT the design's value
map. The design's map is affine, `Adjustment * radix^exponent * k + Bias`, so a
numeral may sit at a phase. i1 therefore reported a half-unit-biased format as
OUTSIDE the concept when the design admits it and the droplist records the very
correction that admits it ("`Bias` as a plain signed integer: made a legal
MATLAB numerictype unrepresentable (slope 1, bias 1/2)"). i1b tests both, and
reports them as separate columns, because the gap between them is a finding
rather than a bug.

DEFECT 2. A binade holding one value pins no step, so i1's "take the value's
own valuation" branch invented a canonical exponent wherever a range was
truncated, and reported tapering that was an artifact of the top binade. This
is the same phenomenon `02_carried` section 1.6 reports for the inclusion
predicate one level up: a numeral carrying fewer than two values pins no grid.
i1b marks such binades UNCONSTRAINED and fits the family over the rest.

DEFECT 3. i1's double-double generator drew the low part from the same float
family as the high part, so no nonzero low part ever qualified and the set it
classified was the plain float. i1b draws the low part from an extended
exponent range.
"""

from fractions import Fraction
from math import gcd

from i1_classify import (is_r_adic, binade, is_power_of, fixed, hub, flt,
                         posit, decimal, fixed_slash, integer_interval,
                         negabinary, mixed_radix, stochastic,
                         lns_rational_fraction)


def per_binade_structure(mags, r):
    """For each binade, return (step, phase_representative, n_values) or
    ('UNCONSTRAINED', v, 1) where the binade holds a single value."""
    by = {}
    for v in mags:
        by.setdefault(binade(v, r), []).append(v)
    out = {}
    for e in sorted(by):
        xs = sorted(by[e])
        if len(xs) == 1:
            out[e] = ("UNCONSTRAINED", xs[0], 1)
            continue
        diffs = {xs[i + 1] - xs[i] for i in range(len(xs) - 1)}
        if len(diffs) != 1:
            out[e] = ("NONUNIFORM", len(diffs), len(xs))
            continue
        out[e] = (diffs.pop(), xs[0], len(xs))
    return out


def classify(name, values, r, note=""):
    vs = sorted(set(values))
    mags = sorted({abs(v) for v in vs if v != 0})

    res = {"name": name, "radix": r, "count": len(vs), "note": note}

    bad = [v for v in mags if not is_r_adic(v, r)]
    res["r_adic"] = not bad
    res["n_not_r_adic"] = len(bad)
    res["first_bad"] = str(bad[0]) if bad else None
    if bad:
        res["verdict"] = "OUTSIDE: values are not m * r^q"
        res["generic_format"] = False
        res["affine_grid"] = False
        res["fexp"] = None
        return res

    st = per_binade_structure(mags, r)
    res["n_binades"] = len(st)
    res["n_unconstrained"] = sum(1 for v in st.values() if v[0] == "UNCONSTRAINED")
    nonuni = [e for e, v in st.items() if v[0] == "NONUNIFORM"]
    res["n_nonuniform"] = len(nonuni)
    if nonuni:
        res["verdict"] = "OUTSIDE: a binade is not an arithmetic progression"
        res["generic_format"] = False
        res["affine_grid"] = False
        res["fexp"] = None
        res["nonuniform_binades"] = nonuni[:4]
        return res

    constrained = {e: v for e, v in st.items() if v[0] != "UNCONSTRAINED"}
    if not constrained:
        res["verdict"] = "DEGENERATE: no binade holds two values"
        res["generic_format"] = None
        res["affine_grid"] = None
        res["fexp"] = None
        return res

    # Is every step an integer power of the radix, times one common Adjustment?
    steps = {e: v[0] for e, v in constrained.items()}
    s_min = min(steps.values())
    ratios_are_powers = all(is_power_of(s / s_min, r) is not None for s in steps.values())
    res["one_adjustment"] = ratios_are_powers
    res["adjustment_witness"] = str(s_min)

    # phase: is there one Bias B with (v - B) a multiple of its binade's step?
    e_fine = min(constrained, key=lambda e: constrained[e][0])
    B_candidate = constrained[e_fine][1] % constrained[e_fine][0]
    ok_phase = True
    for v in mags:
        e = binade(v, r)
        if st[e][0] == "UNCONSTRAINED":
            continue
        if ((v - B_candidate) / st[e][0]).denominator != 1:
            ok_phase = False
            break
    res["affine_grid"] = bool(ratios_are_powers and ok_phase)
    res["bias"] = str(B_candidate)
    res["generic_format"] = bool(res["affine_grid"] and B_candidate == 0
                                 and is_power_of(s_min, r) is not None)

    if not res["affine_grid"]:
        res["verdict"] = ("OUTSIDE: steps are not one adjustment times powers of the radix"
                          if not ratios_are_powers else
                          "OUTSIDE: no single bias places every value on its binade's grid")
        res["fexp"] = None
        return res

    fexp = {}
    for e, (s, _, _) in constrained.items():
        fexp[e] = is_power_of(s / s_min, r)
    res["fexp"] = dict(sorted(fexp.items()))
    res["n_distinct_fexp"] = len(set(fexp.values()))
    res["verdict"] = "INSIDE"
    res["family"] = name_family(res["fexp"])
    return res


def name_family(fexp):
    es = sorted(fexp)
    if len(es) < 2:
        return "UNDETERMINED (fewer than two constrained binades)"
    vals = [fexp[e] for e in es]
    if len(set(vals)) == 1:
        return "FIXED: one segment"
    # affine with slope one on every constrained step?
    affine = all(fexp[es[i + 1]] - fexp[es[i]] == es[i + 1] - es[i]
                 for i in range(len(es) - 1))
    if affine:
        return f"FLOAT: fexp(e) = e - {es[0] - fexp[es[0]]}"
    # constant then slope one
    knee = None
    ok = True
    for i in range(len(es) - 1):
        d = fexp[es[i + 1]] - fexp[es[i]]
        g = es[i + 1] - es[i]
        if d == 0 and knee is None:
            continue
        if d == g:
            knee = knee if knee is not None else es[i]
            continue
        ok = False
        break
    if ok and knee is not None:
        return f"FLOAT with gradual underflow: constant below e = {knee}, slope one above"
    slopes = sorted({(fexp[es[i + 1]] - fexp[es[i]], es[i + 1] - es[i])
                     for i in range(len(es) - 1)})
    return f"SEGMENTED: no named shape, {len(slopes)} distinct step ratios"


def double_double(p, emin, emax):
    hi = [v for v in flt(p, emin, emax, subnormals=False) if v > 0]
    lo = [v for v in flt(p, emin - 3 * p, emax, subnormals=False) if v > 0]
    out = {Fraction(0)}
    for a in hi:
        e = binade(a, 2)
        ulp = Fraction(2) ** (e - p + 1)
        out.add(a)
        for b in lo:
            if b <= ulp / 2:
                out.add(a + b)
                out.add(a - b)
    return sorted(out)


def ranged_fixed(step, n, lo=Fraction(0)):
    """A uniform grid with a FREE count, which is what a `Ranged` numeral is
    and what the anchored family cannot spell unless n is a power of the
    radix."""
    return [lo + k * step for k in range(n)]


def show(res):
    print(f"--- {res['name']}  (radix {res['radix']}, {res['count']} values)")
    if res["note"]:
        print(f"    note: {res['note']}")
    print(f"    verdict            : {res['verdict']}")
    if res.get("n_binades") is not None:
        print(f"    binades            : {res['n_binades']} "
              f"({res['n_unconstrained']} hold one value and pin no step)")
    if res["verdict"] == "INSIDE":
        print(f"    affine grid        : {res['affine_grid']}   "
              f"(adjustment {res['adjustment_witness']}, bias {res['bias']})")
        print(f"    Flocq generic_format (bias zero, step a power of the radix): "
              f"{res['generic_format']}")
        f = res["fexp"]
        ks = sorted(f)
        s = ", ".join(f"{e}:{f[e]}" for e in ks[:8])
        more = "" if len(ks) <= 8 else f", ... ({len(ks)} constrained binades)"
        print(f"    canonical exponent : {{{s}{more}}}  distinct={res['n_distinct_fexp']}")
        print(f"    family             : {res['family']}")
    print()


def main():
    C = []
    C.append(classify("fixed U<3,3>", fixed(3, 3), 2))
    C.append(classify("fixed U<0,4>", fixed(0, 4), 2))
    C.append(classify("HUB fixed I=2 F=2", hub(2, 2), 2,
                      "half-unit-biased; i1 called this OUTSIDE"))
    C.append(classify("ranged grid, step 1/2, 7 values (03's join witness)",
                      ranged_fixed(Fraction(1, 2), 7), 2,
                      "count is not a power of the radix"))
    C.append(classify("float p=3 e=-2..3, no subnormals",
                      flt(3, -2, 3, subnormals=False), 2))
    C.append(classify("float p=3 e=-2..3, subnormals",
                      flt(3, -2, 3, subnormals=True), 2))
    C.append(classify("fp8 E4M3-shaped p=4 e=-6..8, subnormals",
                      flt(4, -6, 8, subnormals=True), 2))
    C.append(classify("bfloat16-shaped p=8 e=-6..7, subnormals",
                      flt(8, -6, 7, subnormals=True), 2))
    C.append(classify("posit<8,0>", posit(8, 0), 2))
    C.append(classify("posit<8,1>", posit(8, 1), 2))
    C.append(classify("posit<10,2>", posit(10, 2), 2))
    C.append(classify("decimal p=2 e=-2..2 at radix 10", decimal(2, -2, 2), 10))
    C.append(classify("decimal p=2 e=-2..2 at radix 2", decimal(2, -2, 2), 2,
                      "same set, asked at the binary radix"))
    C.append(classify("fixed-slash P=7 Q=7", fixed_slash(7, 7), 2))
    C.append(classify("residue number system (3,5,7), value set",
                      integer_interval(0, 104), 2,
                      "the encoding is the whole point and no value-set test can see it"))
    C.append(classify("thermometer 16 levels, value set",
                      integer_interval(0, 16), 2, "encoding-only difference"))
    C.append(classify("carry-save 6-bit, value set",
                      integer_interval(0, 63), 2,
                      "encoding-only difference, and the encoding is not injective"))
    C.append(classify("negabinary 6 digits", negabinary(6), 2))
    C.append(classify("mixed radix (factorial base, 5!)", mixed_radix([2, 3, 4, 5]), 2))
    C.append(classify("stochastic stream N=16, value set", stochastic(16), 2,
                      "a datum denotes a distribution, not a point"))
    C.append(classify("double-double p=3 e=-1..2", double_double(3, -1, 2), 2,
                      "i1's generator produced the plain float here"))

    for c in C:
        show(c)

    tot, rat = lns_rational_fraction(3, 1, 64)
    print("--- logarithmic number system, base 2, three fraction bits")
    print(f"    2^(k/8) for k = 1..64, rational at all: {rat} of {tot}")
    print("    verdict            : OUTSIDE: values are not m * r^q\n")

    inside = [c for c in C if c["verdict"] == "INSIDE"]
    print(f"SUMMARY over {len(C)} value sets plus the logarithmic case:")
    print(f"  inside the affine-grid-per-binade concept : {len(inside)} of {len(C)}")
    gf = sum(1 for c in inside if c["generic_format"])
    print(f"    of those, at bias zero (Flocq generic_format): {gf}")
    print(f"    of those, needing a nonzero bias            : {len(inside) - gf}")
    fam = {}
    for c in inside:
        fam[c["family"].split(":")[0]] = fam.get(c["family"].split(":")[0], 0) + 1
    for k in sorted(fam):
        print(f"    {k}: {fam[k]}")
    print("  outside, with the clause that fails:")
    for c in C:
        if c["verdict"] != "INSIDE":
            print(f"    {c['name']}: {c['verdict']}")
    print("    logarithmic number system: OUTSIDE: values are not m * r^q")


if __name__ == "__main__":
    main()
