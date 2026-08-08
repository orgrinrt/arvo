# Generator for p07: the WHOLE matrix of Sum<Na,Nb> == N(a+b) for a,b in 0..=B.
# Not a sample. If any pair fails to normalise, the file does not compile.
import sys
B = int(sys.argv[1]) if len(sys.argv) > 1 else 64

def nat(n):
    if n == 0:
        return "Z"
    return ("O<" if n & 1 else "E<") + nat(n >> 1) + ">"

head = open("p06_nat_algebra.rs").read()
head = head.split("// literals, LSB first")[0]

out = [head]
out.append("// literals for 0..=%d and 0..=%d, machine generated\n" % (B, 2 * B))
for n in range(0, 2 * B + 1):
    out.append("pub type N%d = %s;\n" % (n, nat(n)))
out.append("\npub trait Same<T> {}\nimpl<T> Same<T> for T {}\n\n")
out.append("fn _whole_matrix()\nwhere\n")
for a in range(0, B + 1):
    for b in range(0, B + 1):
        out.append("    Sum<N%d, N%d>: Same<N%d>,\n" % (a, b, a + b))
out.append("{\n}\n")
open("p07_matrix.rs", "w").write("".join(out))
print("pairs:", (B + 1) * (B + 1))
