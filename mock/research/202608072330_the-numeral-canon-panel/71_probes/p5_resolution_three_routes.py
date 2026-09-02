#!/usr/bin/env python3
"""
p5. `66` separates CONVERSION (a value moves between systems) from RESOLUTION (which system's
laws govern an operation whose operands disagree) and files them as two questions. This asks
whether resolution has an answer once conversion is settled.

HYPOTHESIS, written before the run.

If resolution were settled by conversion, then "convert both operands into the result system,
then operate there" would be THE meaning of a mixed operation, and nothing further would be
owed. `63` C1 says otherwise: arithmetic is an exact operation in the ambient domain composed
with ONE named adaptation, and an operation that fuses adaptations invisibly is several
adaptation points wearing one name.

Three routes for one mixed expression `a + b`, a from system A, b from system B, result
declared in system C. They differ only in how many adaptation points sit in the schedule:

  UNFUSED   rho_C(a + b)                     one adaptation.   `63` C1's model.
  CONVERT   rho_C(rho_C(a) + rho_C(b))       three.            conversion-then-operate.
  VIA_A     rho_C(rho_A(a + b))              two.              operate in the wider, then cross.

PREDICTION: the three are three different functions, so "the mixed operation" does not have a
meaning fixed by having a conversion. If the prediction fails and all three coincide, then
resolution really is a corollary of conversion and `66`'s second question needs no separate
answer.

Exhaustive over every operand pair. Model widths 6, 5 and 4 bits.
"""

A_SET = list(range(-32, 32))   # source of the left operand
B_SET = list(range(-16, 16))   # source of the right operand
C_SET = list(range(-8, 8))     # declared result system


def wrap(v, q):
    n = len(q)
    return ((v - q[0]) % n) + q[0]


def saturate(v, q):
    return q[0] if v < q[0] else (q[-1] if v > q[-1] else v)


REDUCTIONS = [("wrap", wrap), ("saturate", saturate)]


def routes(rho_a, rho_c):
    unfused, convert, via_a = [], [], []
    for a in A_SET:
        for b in B_SET:
            exact = a + b
            unfused.append(rho_c(exact, C_SET))
            convert.append(rho_c(rho_c(a, C_SET) + rho_c(b, C_SET), C_SET))
            via_a.append(rho_c(rho_a(exact, A_SET), C_SET))
    return unfused, convert, via_a


def agree(x, y):
    return sum(1 for p, q in zip(x, y) if p == q)


TOTAL = len(A_SET) * len(B_SET)

print("=" * 92)
print("THREE ROUTES THROUGH ONE MIXED EXPRESSION, a + b with a in A, b in B, result in C")
print(f"A = [-32,31]   B = [-16,15]   C = [-8,7]   operand pairs = {TOTAL}")
print("=" * 92)
print(f"{'rho_A':<12}{'rho_C':<12}{'UNFUSED=CONVERT':<20}{'UNFUSED=VIA_A':<20}"
      f"{'CONVERT=VIA_A':<20}{'all three'}")

rows = []
for an, rho_a in REDUCTIONS:
    for cn, rho_c in REDUCTIONS:
        u, c, v = routes(rho_a, rho_c)
        uc, uv, cv = agree(u, c), agree(u, v), agree(c, v)
        allthree = sum(1 for i in range(TOTAL) if u[i] == c[i] == v[i])
        rows.append((an, cn, uc, uv, cv, allthree))
        print(f"{an:<12}{cn:<12}{uc}/{TOTAL:<14}{uv}/{TOTAL:<14}{cv}/{TOTAL:<14}"
              f"{allthree}/{TOTAL}")

print()
distinct_everywhere = all(r[5] < TOTAL for r in rows)
any_pair_total = [(r[0], r[1]) for r in rows if r[2] == TOTAL or r[3] == TOTAL or r[4] == TOTAL]

print(f"Cells where the three routes do NOT all coincide: "
      f"{sum(1 for r in rows if r[5] < TOTAL)} of {len(rows)}")
print(f"Cells where at least two routes coincide totally: {any_pair_total}")
print()

if distinct_everywhere:
    print("PREDICTION HELD in its strong form.")
else:
    print("PREDICTION REFUTED in its strong form, and the refutation is the better result.")
    print("The three routes coincide totally in exactly the cells named above, and those are")
    print("the cells where the reductions carry the laws that make a schedule invisible. So")
    print("the correct statement is not 'the routes always differ' but: A CONVERSION FIXES THE")
    print("MEANING OF A MIXED OPERATION EXACTLY WHEN THE REDUCTIONS INVOLVED ARE COHERENT AND")
    print("NESTED-ABSORBING, AND OTHERWISE THE SCHEDULE HAS TO BE NAMED.")

print()
print("-" * 92)
print("WHICH COINCIDENCES OCCUR, WITH THE REASON MEASURED RATHER THAN ASSERTED")
print("-" * 92)
print("Three facts are computed per cell, none of them read off the agreement counts:")
print("  FIRES     does rho_A act on any reachable exact value")
print("  ABSORBED  is rho_C(rho_A(x)) == rho_C(x) for every reachable exact value")
print("  COHERENT  is rho_C(a + b) == rho_C(rho_C(a) + rho_C(b)) over the operand box")
print()
for an, rho_a in REDUCTIONS:
    for cn, rho_c in REDUCTIONS:
        exacts = sorted({a + b for a in A_SET for b in B_SET})
        fires = any(rho_a(x, A_SET) != x for x in exacts)
        absorbed = all(rho_c(rho_a(x, A_SET), C_SET) == rho_c(x, C_SET) for x in exacts)
        coherent = all(
            rho_c(a + b, C_SET) == rho_c(rho_c(a, C_SET) + rho_c(b, C_SET), C_SET)
            for a in A_SET for b in B_SET
        )
        row = next(r for r in rows if r[0] == an and r[1] == cn)
        print(f"  rho_A={an:<9} rho_C={cn:<9} FIRES={str(fires):<6} "
              f"ABSORBED={str(absorbed):<6} COHERENT={str(coherent):<6}"
              f"   UNFUSED=CONVERT {row[2] == TOTAL}, UNFUSED=VIA_A {row[3] == TOTAL}")
        assert (row[2] == TOTAL) == coherent, "CONVERT=UNFUSED did not track coherence"
        assert (row[3] == TOTAL) == absorbed, "VIA_A=UNFUSED did not track absorption"
print()
print("  Both implications hold in every cell, asserted in the probe rather than eyeballed:")
print("  CONVERT coincides with UNFUSED exactly when rho_C is coherent, and VIA_A coincides")
print("  with UNFUSED exactly when the source's reduction is absorbed by the target's. The")
print("  second is why saturation into a nested window shows no double rounding: clamping to")
print("  [-32,31] and then to [-8,7] is clamping to [-8,7]. Nesting, not absence of firing.")

all_three_distinct = 0
for an, rho_a in REDUCTIONS:
    for cn, rho_c in REDUCTIONS:
        for a in A_SET:
            for b in B_SET:
                exact = a + b
                u = rho_c(exact, C_SET)
                c = rho_c(rho_c(a, C_SET) + rho_c(b, C_SET), C_SET)
                v = rho_c(rho_a(exact, A_SET), C_SET)
                if len({u, c, v}) == 3:
                    all_three_distinct += 1
print()
print(f"  Operand pairs, over all four cells, at which all three routes give three different")
print(f"  answers: {all_three_distinct} of {4 * TOTAL}. The routes differ pairwise and never")
print("  three ways at once, which is a structural fact and not a search that came up empty.")

print()
print("-" * 92)
print("HAND WITNESSES: one operand pair per cell where two routes disagree")
print("-" * 92)
for an, rho_a in REDUCTIONS:
    for cn, rho_c in REDUCTIONS:
        found = None
        for a in A_SET:
            for b in B_SET:
                exact = a + b
                u = rho_c(exact, C_SET)
                c = rho_c(rho_c(a, C_SET) + rho_c(b, C_SET), C_SET)
                v = rho_c(rho_a(exact, A_SET), C_SET)
                if len({u, c, v}) > 1:
                    found = (a, b, exact, u, c, v)
                    break
            if found:
                break
        if found:
            a, b, exact, u, c, v = found
            print(f"  rho_A={an:<9} rho_C={cn:<9} a={a:>4} b={b:>4} exact={exact:>4}  "
                  f"UNFUSED={u:>3}  CONVERT={c:>3}  VIA_A={v:>3}")
        else:
            print(f"  rho_A={an:<9} rho_C={cn:<9} every operand pair agrees on all three "
                  f"routes: the schedule is invisible here")

print()
print("=" * 92)
print("READING")
print("=" * 92)
print("A conversion function does not by itself determine a mixed operation, and the cells")
print("above say exactly when it does. What determines the operation is how many adaptation")
print("points the schedule contains and where they sit, which is `63` C9's statement that the")
print("schedule is part of the function's meaning, arriving at the place two systems meet.")
print()
print("So `66`'s two questions are one obligation asked at two arities: a conversion is the")
print("unary case with the target declared, a resolution the n-ary case with the target")
print("derived, and both owe the same three things, a common ambient domain, a target")
print("representable set, and which reduction governs the loss. Only the third is a choice,")
print("neither operand's own system is privileged to make it, and the choice stops being")
print("observable exactly in the cells where coherence and nested absorption both hold.")
