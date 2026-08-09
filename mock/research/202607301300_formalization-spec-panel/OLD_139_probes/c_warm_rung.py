# 131:275-280's rule for Warm/Precise is "one rung of headroom": rung(rung_bits(W) + 1).
# Candidate replacement: rung(W + margin) with margin the operation's carry need in bits.
# Third candidate: no container headroom at all, margin covered by the spare bits the
# rung already has plus the hardware carry.
NATIVE = [8, 16, 32, 64, 128]
def rung_bits(w):
    for b in NATIVE:
        if w <= b: return b
    return None                                    # wide
def name(b):  return f"u{b}" if b else "Wide"

print(f"{'W':>4}  {'Hot':>5}  {'131 Warm':>9}  {'rung(W+1)':>10}  {'spare':>6}  {'no-headroom':>11}")
for W in [1,3,8,12,13,16,17,24,31,32,33,48,60,63,64,65,96,128,129]:
    hot = rung_bits(W)
    warm131 = rung_bits(hot + 1) if hot else None
    warm_m1 = rung_bits(W + 1)
    spare = (hot - W) if hot else None
    print(f"{W:4}  {name(hot):>5}  {name(warm131):>9}  {name(warm_m1):>10}  "
          f"{('' if spare is None else spare):>6}  {name(hot):>11}")

print()
print("widths at or below 64 that 131's rule places in a container wider than Hot's:")
bad = [W for W in range(1,65) if rung_bits(rung_bits(W)+1) != rung_bits(W)]
print(f"  {len(bad)} of 64, that is every one of them")
print()
print("widths at or below 64 that rung(W+1) places wider than Hot's:")
bad2 = [W for W in range(1,65) if rung_bits(W+1) != rung_bits(W)]
print(f"  {bad2}   ({len(bad2)} of 64)")
print("  these are exactly the widths that exactly fill their rung, where spare == 0")
print()
print("widths at or below 64 that the no-container-headroom rule places wider than Hot's:")
print("  none, by construction; the margin is not in the container")
