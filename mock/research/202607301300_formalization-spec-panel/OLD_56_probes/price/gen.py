import random, math

random.seed(56)

def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

N = 60

# Variant A: raw distinct u64-tagged types, no seal check at all (floor).
lines_a = ["#![allow(dead_code)]", "pub struct Tag<const V: u64>;", ""]
for i in range(N):
    v = random.randint(2, 1 << 20)
    lines_a.append(f"pub const _CHECK_{i}: u64 = {{ let t: Tag<{v}> = Tag; {v} }};")

# Variant B: const-struct face (adt_const_params) with the forced seal
# check via `.checked()`, one distinct composition per line, every
# num/den pair already reduced (the common, cheap case a real macro would
# emit, matching the profile file 53 found realistic rather than the
# worst case).
lines_b = [
    "#![feature(adt_const_params)]",
    "#![allow(dead_code)]",
    "use core::marker::ConstParamTy;",
    "use core::marker::PhantomData;",
    "const fn gcd(mut a: u64, mut b: u64) -> u64 { while b != 0 { let t = b; b = a % b; a = t; } a }",
    "#[derive(PartialEq, Eq, ConstParamTy)]",
    "pub struct Spec { pub precision: u16, pub bias_num: u64, pub bias_den: u64 }",
    "impl Spec { const fn is_reduced(&self) -> bool { gcd(self.bias_num, self.bias_den) == 1 } }",
    "pub struct NFace<const S: Spec>(PhantomData<()>);",
    "impl<const S: Spec> NFace<S> {",
    "    pub const REDUCED: () = assert!(S.is_reduced());",
    "    pub const fn checked() -> Self { let () = Self::REDUCED; NFace(PhantomData) }",
    "}",
    "",
]
for i in range(N):
    p = random.randint(1, 60)
    num = random.randint(1, 500)
    den = random.randint(1, 500)
    g = gcd(num, den)
    num //= g
    den //= g
    if den == 0:
        den = 1
    lines_b.append(
        f"pub const SPEC_{i}: Spec = Spec {{ precision: {p}, bias_num: {num}, bias_den: {den} }};"
    )
    lines_b.append(f"pub fn mk_{i}() -> NFace<SPEC_{i}> {{ NFace::<SPEC_{i}>::checked() }}")

with open("variant_a.rs", "w") as f:
    f.write("\n".join(lines_a) + "\n")
with open("variant_b.rs", "w") as f:
    f.write("\n".join(lines_b) + "\n")
print("generated", N, "items per variant")
