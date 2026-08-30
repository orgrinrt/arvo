#!/usr/bin/env python3
# s1 (133, the signature check): the bit-drop/toward-zero difference count has an exact closed
# form, both committed sweeps match it, and the fabricated figure fits no cell of any natural
# sweep. Exact integers throughout.
#
# Predictions, stated before running:
#   1. At width W, fraction F, the count of W-bit two's complement values where bit-drop (floor)
#      differs from toward_zero is exactly 2^(W-1) - 2^(W-1-F): the negatives with a nonzero
#      dropped bit. Exhaustive recount at W = 8, F in 1..7 matches.
#   2. The committed sweeps match it: 125_probes/p5 reported 64/112/124 at F in {1,3,5}; 131's v2
#      reported 4, 6 (W=4), 16, 24 (W=6), 64, 96 (W=8) at F in {1,2}, and its figure-hunt reported
#      64, 96, 120, 127 at F in {1,2,4,7}. All are cells of the closed form.
#   3. The figure 21,204 is NOT of the form 2^b * (2^k - 1) for any b >= 0, k >= 1, so it cannot
#      be a cell of the closed form at ANY width and fraction. Search bound: all b, k with
#      2^b (2^k - 1) <= 2^60.
#   4. Control (the instrument can fail): an off-by-one formula 2^(W-1) - 2^(W-F) mismatches the
#      recount at every F tested.

def count_diff(W, F):
    lo, hi = -(1 << (W - 1)), (1 << (W - 1))
    c = 0
    for v in range(lo, hi):
        bitdrop = v >> F
        tz = v // (1 << F) if v >= 0 else -((-v) // (1 << F))
        if bitdrop != tz:
            c += 1
    return c

print("--- 1+2. exhaustive recount against the closed form and the committed sweeps ---")
ok = True
for W in (4, 6, 8):
    for F in range(1, W):
        got = count_diff(W, F)
        formula = (1 << (W - 1)) - (1 << (W - 1 - F))
        wrong_formula = (1 << (W - 1)) - (1 << (W - F))  # control, off by one in the exponent
        match = got == formula
        ctrl = got != wrong_formula
        ok = ok and match and ctrl
        print(f"W={W} F={F}: recount={got}, closed form={formula} (match={match}), "
              f"off-by-one control={wrong_formula} (differs={ctrl})")
committed = {(8, 1): 64, (8, 3): 112, (8, 5): 124,           # 125_probes/p5_output.txt
             (4, 1): 4, (4, 2): 6, (6, 1): 16, (6, 2): 24,   # 131_probes/v2 signed rows
             (8, 2): 96, (8, 4): 120, (8, 7): 127}           # 131 s5.2 figure hunt
for (W, F), n in sorted(committed.items()):
    formula = (1 << (W - 1)) - (1 << (W - 1 - F))
    print(f"committed W={W} F={F}: {n}, closed form {formula}, match={n == formula}")
    ok = ok and n == formula
print(f"all matches and controls: {ok} (must be True)")

print("--- 3. 21,204 fits no cell of the closed form at any width ---")
target = 21204
hits = []
b = 0
while (1 << b) <= (1 << 60):
    k = 1
    while (1 << b) * ((1 << k) - 1) <= (1 << 60):
        if (1 << b) * ((1 << k) - 1) == target:
            hits.append((b, k))
        k += 1
    b += 1
print(f"representations of {target} as 2^b*(2^k-1) up to 2^60: {hits} (must be [])")
# sanity: the instrument finds representations when they exist
probe = 112  # = 2^4 * 7
hits2 = [(b, k) for b in range(0, 20) for k in range(1, 20)
         if (1 << b) * ((1 << k) - 1) == probe]
print(f"instrument sanity: representations of {probe}: {hits2} (must be non-empty)")
