# 138 computes a "slack" in 3.2 (exponent codes / span) and an "overshoot" in 3.1
# (per-field sum vs joint). Claim under test: they are the same quantity, and 3.2's
# is its one-field restriction. Exact rationals, no floats.
from fractions import Fraction
def cl2(n): return (n - 1).bit_length()

F = [("binary16",   2, 11,  -14,    15, 1),
     ("binary32",   2, 24, -126,   127, 1),
     ("binary64",   2, 53,-1022,  1023, 1),
     ("binary128",  2,113,-16382,16383, 1),
     ("binary256",  2,237,-262142,262143,1),
     ("bfloat16",   2,  8, -126,   127, 1),
     ("E4M3(OCP)",  2,  4,   -6,     8, 1),
     ("E5M2(OCP)",  2,  3,  -14,    15, 1),
     ("decimal32", 10,  7,  -95,    96, 0),
     ("decimal64", 10, 16, -383,   384, 0),
     ("decimal128",10, 34,-6143,  6144, 0)]

print(f"{'format':11} {'slack_sig':>10} {'slack_exp':>10} {'slack_all':>10}  {'138 3.2 slack':>13}  bits_lost")
for name, R, P, EMIN, EMAX, h in F:
    span = EMAX - EMIN + 1
    sig_card = R**(P - h)
    s_sig = Fraction(2**cl2(sig_card), sig_card)
    s_exp = Fraction(2**cl2(span), span)
    s_all = s_sig * s_exp
    # 138's 3.2 figure is an absolute code count, not a ratio: codes - span
    codes_minus_span = 2**cl2(span) - span
    lost = cl2(sig_card) + cl2(span) - cl2(sig_card * span)
    print(f"{name:11} {float(s_sig):10.4f} {float(s_exp):10.4f} {float(s_all):10.4f}  "
          f"{codes_minus_span:13}  {lost}")

print()
print("check: s_all >= 2  <=>  a whole bit is lost  <=>  the per-field sum overshoots")
for name, R, P, EMIN, EMAX, h in F:
    span = EMAX - EMIN + 1; sig_card = R**(P-h)
    s_all = Fraction(2**cl2(sig_card), sig_card) * Fraction(2**cl2(span), span)
    lost = cl2(sig_card) + cl2(span) - cl2(sig_card*span)
    assert (s_all >= 2) == (lost >= 1), name
print("  holds at all 11")
print()
print("check: at radix 2 the significand slack is exactly 1, so s_all == s_exp,")
print("       which is why 138's one-field figure was sufficient there and only there")
for name, R, P, EMIN, EMAX, h in F:
    sig_card = R**(P-h); s_sig = Fraction(2**cl2(sig_card), sig_card)
    print(f"  {name:11} radix {R:2}  slack_sig = {s_sig}  == 1: {s_sig == 1}")
