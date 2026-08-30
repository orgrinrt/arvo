import sys
k = int(sys.argv[1]); out = sys.argv[2]
L = ["#![feature(const_trait_impl)]", "use union::*;", "use core::hint::black_box;",
     "pub type C<const I: u16, const F: u16, P, L> = Number<Fix<I, F, Unsigned>, P, L>;", "fn main() {"]
presets = ["Warm", "Hot", "Cold", "Precise"]
mods = ["{p}", "OverRangeOf<{p}, ReduceModulo>", "LayoutOf<{p}, Bitpacked>",
        "DeliveredAs<{p}, AsBottom>", "UnderRangeOf<{p}, TowardNegative>"]
i = 0
for w in range(1, 200):
    for p in presets:
        for m in mods:
            if i >= k: break
            s = m.format(p=p)
            L.append(f"    black_box(add::<C<{w%60+1}, {w%7}, {s}, {s}>>(black_box(3), black_box(4), 0, 100));")
            i += 1
        if i >= k: break
    if i >= k: break
L.append("}")
open(out,"w").write("\n".join(L)+"\n")
