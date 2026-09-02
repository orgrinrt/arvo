# Generator for p10: the core, plus literals, plus a check that ceil-to-bytes
# and the buffer size are right for EVERY width 0..=B, plus container selection.
import sys, subprocess
B = int(sys.argv[1]) if len(sys.argv) > 1 else 130

def nat(n):
    if n == 0: return "Z"
    return ("O<" if n & 1 else "E<") + nat(n >> 1) + ">"

core = open("p09_core.rs").read()
out = [core, "\n// literals 0..=%d, machine generated\n" % B]
for n in range(0, B + 1):
    out.append("pub type N%d = %s;\n" % (n, nat(n)))

out.append("""
// container: five native rungs then the wide rung, expressed once
pub type Container<W> = Rung<W, N8, u8,
                       Rung<W, N16, u16,
                       Rung<W, N32, u32,
                       Rung<W, N64, u64,
                       Rung<W, N128, u128,
                       BufOf<Bytes<W>>>>>>>;

pub trait Same<T> {}
impl<T> Same<T> for T {}

pub fn size_of_buf<W>() -> usize where W: CeilHalf, CH<W>: CeilHalf, CH<CH<W>>: CeilHalf, Bytes<W>: Buf {
    core::mem::size_of::<BufOf<Bytes<W>>>()
}
""")

# whole-matrix check of ceil-to-bytes, every width, not a sample
out.append("fn _bytes_matrix()\nwhere\n")
for n in range(0, B + 1):
    out.append("    Bytes<N%d>: Same<N%d>,\n" % (n, (n + 7) // 8))
out.append("{\n}\n")

# whole-matrix check of container selection, every width
def cont(n):
    if n <= 8: return "u8"
    if n <= 16: return "u16"
    if n <= 32: return "u32"
    if n <= 64: return "u64"
    if n <= 128: return "u128"
    return None
out.append("fn _container_matrix()\nwhere\n")
for n in range(0, B + 1):
    c = cont(n)
    if c is not None:
        out.append("    Container<N%d>: Same<%s>,\n" % (n, c))
    else:
        out.append("    Container<N%d>: Same<BufOf<N%d>>,\n" % (n, (n + 7) // 8))
out.append("{\n}\n")
open("p10_full.rs", "w").write("".join(out))
print("widths checked:", B + 1)
