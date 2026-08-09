import sys
def tow(n):
    if n == 0: return "Term"
    s = "Term"
    bits = []
    while n: bits.append(n & 1); n >>= 1
    for b in reversed(bits):
        s = ("D1<%s>" if b else "D0<%s>") % s
    return s
for n in sys.argv[1:]:
    n = int(n)
    print("pub type N%d = %s;" % (n, tow(n)))
