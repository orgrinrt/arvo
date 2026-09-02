#!/usr/bin/env python3
"""
p1. The five crossing classes, one per telescope coordinate, measured at four bits.

HYPOTHESIS, written before the run so the output can refute it.

`67` section 4 measures three crossings, one each at telescope indices 1, 2 and 3, and
concludes: "Every crossing is total and preserves values or patterns at 100%. No crossing
preserves operations at 100%."

The telescope `67` section 2 states has FIVE coordinates:

    (D : Ambient) x (Q : Reach(D)) x (rho : Reduce(D,Q)) x (E : Encode(Q)) x (C : Hold(E))

and `67`'s own rule is that a crossing is identified by the smallest index at which two terms
disagree. So there are five crossing classes, not three, and the two unmeasured ones (index 4,
a re-encoding; index 5, a re-housing) are exactly the two where the value map is the identity
AND the two systems' value-level operations are the same function.

PREDICTION: indices 4 and 5 preserve the value-level operation at 256/256, refuting the
universal quoted above. Index 4 destroys the pattern-level operation (excess-K's constant
defect). Index 3 preserves BOTH the pattern map and the untyped pattern-level operation and
destroys only the value-level one, which is the precise content of "the crossing that looks
free".

CONTROL: the three rows `67` already measured must reproduce, or this instrument is measuring
something else and its new rows are worthless. Expected from `67` section 4:
  X1 restrategise (index 3): add 192/256, mul 111/256 on the signed window
  X2 widen        (index 2): add 192/256, mul 101/256
  X3 reinterpret  (index 1): vs xor 108/256, vs min 1/256

Two operation levels are measured and never conflated:

  VALUE level.   The system's own operation: adapt(exact(a, b)) computed on carrier values.
  PATTERN level. The container's own wrapping operation on the stored bits, which is what a
                 consumer gets by operating on the bits without consulting the type. This is
                 the "raw adder" property of `63` section 3.5.

Model width 4 throughout. Exhaustive over all 256 ordered operand pairs per cell.
"""

W = 4
M = 1 << W                      # 16 patterns

# ---------------------------------------------------------------- carrier sets (index 2)

SIGNED4 = list(range(-8, 8))    # [-8, 7]
UNSIGNED4 = list(range(0, 16))  # [0, 15]
SIGNED8 = list(range(-128, 128))


def lo(q):
    return q[0]


def hi(q):
    return q[-1]


# ------------------------------------------------------------- reductions rho (index 3)

def wrap(v, q):
    """Wrap onto q: the representative of v modulo |q| lying in q."""
    n = len(q)
    return ((v - lo(q)) % n) + lo(q)


def saturate(v, q):
    if v < lo(q):
        return lo(q)
    if v > hi(q):
        return hi(q)
    return v


REDUCTIONS = {"wrap": wrap, "saturate": saturate}


# ------------------------------------------------------------- ambient domains (index 1)
# Each is a carrier interpretation of a 4-bit pattern plus an operation family. The GF(2)^4
# and tropical domains share the container and share no operation with (Z, +, *).

def dom_Z_add(a, b):
    return a + b


def dom_Z_mul(a, b):
    return a * b


def dom_gf2_add(a, b):
    """xor, the additive operation of GF(2)^4, taken on patterns."""
    return a ^ b


def dom_trop_add(a, b):
    """min, the additive operation of the tropical semiring."""
    return min(a, b)


# ------------------------------------------------------------------- encodings (index 4)
# Both are bijections between the 16 patterns and the signed window, so both realise the SAME
# representable set. They differ only in which pattern names which value.

def enc_twos(v):
    return v & (M - 1)


def dec_twos(p):
    return p - M if p >= (M >> 1) else p


def enc_excess(v):
    """excess-K, K = 8: the order-agreeing bijection of `63` section 3.5."""
    return v + 8


def dec_excess(p):
    return p - 8


ENCODINGS = {
    "twos_complement": (enc_twos, dec_twos),
    "excess_8": (enc_excess, dec_excess),
}


# ------------------------------------------------------------------ containers (index 5)
# A container is a housing: which physical bits of a byte hold the numeral's four bits. Both
# hold the identical four-bit content; only the offset moves. This is the per-element half of
# a packed run's phase.

def house(p, offset):
    return (p & (M - 1)) << offset


def unhouse(byte, offset):
    return (byte >> offset) & (M - 1)


# --------------------------------------------------------------------------- the system

class System:
    """A completed telescope term."""

    def __init__(self, name, dom_add, dom_mul, q, rho_name, enc_name, offset):
        self.name = name
        self.dom_add = dom_add
        self.dom_mul = dom_mul
        self.q = q
        self.rho_name = rho_name
        self.rho = REDUCTIONS[rho_name] if rho_name else None
        self.enc_name = enc_name
        self.enc, self.dec = ENCODINGS[enc_name]
        self.offset = offset

    def add(self, a, b):
        """The system's own addition on carrier values."""
        return self.rho(self.dom_add(a, b), self.q)

    def mul(self, a, b):
        return self.rho(self.dom_mul(a, b), self.q)

    def store(self, v):
        return house(self.enc(v), self.offset)

    def load(self, byte):
        return self.dec(unhouse(byte, self.offset))

    def patterns(self):
        return [self.store(v) for v in self.q]


def raw_add(byte_a, byte_b, offset):
    """The container's own wrapping add on the four stored bits, type not consulted."""
    a = unhouse(byte_a, offset)
    b = unhouse(byte_b, offset)
    return house((a + b) & (M - 1), offset)


# ------------------------------------------------------------------------- measurements

def measure_value_ops(src, dst, vmap, op):
    """vmap(a op_src b) == vmap(a) op_dst vmap(b), over all ordered pairs of src's carrier."""
    agree = 0
    total = 0
    for a in src.q:
        for b in src.q:
            total += 1
            left = vmap(getattr(src, op)(a, b))
            right = getattr(dst, op)(vmap(a), vmap(b))
            if left == right:
                agree += 1
    return agree, total


def measure_pattern_ops(src, dst, pmap):
    """pmap(raw_add_src(x, y)) == raw_add_dst(pmap(x), pmap(y)), over all ordered pattern pairs."""
    agree = 0
    total = 0
    pats = src.patterns()
    for x in pats:
        for y in pats:
            total += 1
            left = pmap(raw_add(x, y, src.offset))
            right = raw_add(pmap(x), pmap(y), dst.offset)
            if left == right:
                agree += 1
    return agree, total


def pattern_identity_fraction(src, dst, pmap):
    same = sum(1 for x in src.patterns() if pmap(x) == x)
    return same, len(src.patterns())


def report(title, lines):
    print()
    print("=" * 78)
    print(title)
    print("=" * 78)
    for line in lines:
        print(line)


# ------------------------------------------------------- index 5: the container crossing

sys_c_a = System("signed4/sat/2c @offset0", dom_Z_add, dom_Z_mul, SIGNED4, "saturate",
                 "twos_complement", 0)
sys_c_b = System("signed4/sat/2c @offset2", dom_Z_add, dom_Z_mul, SIGNED4, "saturate",
                 "twos_complement", 2)


def cross_container(x):
    return house(unhouse(x, 0), 2)


v_add_5 = measure_value_ops(sys_c_a, sys_c_b, lambda v: v, "add")
v_mul_5 = measure_value_ops(sys_c_a, sys_c_b, lambda v: v, "mul")
p_ops_5 = measure_pattern_ops(sys_c_a, sys_c_b, cross_container)
p_id_5 = pattern_identity_fraction(sys_c_a, sys_c_b, cross_container)
vals_5 = len(set(v for v in sys_c_a.q))

report("INDEX 5. Re-housing: same encoding, different physical offset", [
    f"  source            {sys_c_a.name}",
    f"  target            {sys_c_b.name}",
    f"  value map         identity on {vals_5} values, total and injective",
    f"  pattern map       {p_id_5[0]}/{p_id_5[1]} patterns unchanged (a shift, not the identity)",
    f"  VALUE ops   add   {v_add_5[0]}/{v_add_5[1]}",
    f"  VALUE ops   mul   {v_mul_5[0]}/{v_mul_5[1]}",
    f"  PATTERN ops add   {p_ops_5[0]}/{p_ops_5[1]}",
])

# -------------------------------------------------------- index 4: the encoding crossing

sys_e_a = System("signed4/sat/2c", dom_Z_add, dom_Z_mul, SIGNED4, "saturate",
                 "twos_complement", 0)
sys_e_b = System("signed4/sat/excess8", dom_Z_add, dom_Z_mul, SIGNED4, "saturate",
                 "excess_8", 0)


def cross_encoding(x):
    return sys_e_b.store(sys_e_a.load(x))


v_add_4 = measure_value_ops(sys_e_a, sys_e_b, lambda v: v, "add")
v_mul_4 = measure_value_ops(sys_e_a, sys_e_b, lambda v: v, "mul")
p_ops_4 = measure_pattern_ops(sys_e_a, sys_e_b, cross_encoding)
p_id_4 = pattern_identity_fraction(sys_e_a, sys_e_b, cross_encoding)
inj_4 = len(set(cross_encoding(x) for x in sys_e_a.patterns()))

report("INDEX 4. Re-encoding: same (D, Q, rho), different pattern map", [
    f"  source            {sys_e_a.name}",
    f"  target            {sys_e_b.name}",
    f"  value map         identity on {len(SIGNED4)} values, total and injective",
    f"  pattern map       bijection, {inj_4}/16 distinct images, "
    f"{p_id_4[0]}/{p_id_4[1]} unchanged",
    f"  VALUE ops   add   {v_add_4[0]}/{v_add_4[1]}",
    f"  VALUE ops   mul   {v_mul_4[0]}/{v_mul_4[1]}",
    f"  PATTERN ops add   {p_ops_4[0]}/{p_ops_4[1]}",
])

# ------------------------------------------------------ index 3: the adaptation crossing

sys_r_a = System("signed4/wrap/2c", dom_Z_add, dom_Z_mul, SIGNED4, "wrap",
                 "twos_complement", 0)
sys_r_b = System("signed4/sat/2c", dom_Z_add, dom_Z_mul, SIGNED4, "saturate",
                 "twos_complement", 0)

sys_ru_a = System("unsigned4/wrap/2c", dom_Z_add, dom_Z_mul, UNSIGNED4, "wrap",
                  "twos_complement", 0)
sys_ru_b = System("unsigned4/sat/2c", dom_Z_add, dom_Z_mul, UNSIGNED4, "saturate",
                  "twos_complement", 0)

v_add_3 = measure_value_ops(sys_r_a, sys_r_b, lambda v: v, "add")
v_mul_3 = measure_value_ops(sys_r_a, sys_r_b, lambda v: v, "mul")
v_add_3u = measure_value_ops(sys_ru_a, sys_ru_b, lambda v: v, "add")
v_mul_3u = measure_value_ops(sys_ru_a, sys_ru_b, lambda v: v, "mul")
p_ops_3 = measure_pattern_ops(sys_r_a, sys_r_b, lambda x: x)
p_id_3 = pattern_identity_fraction(sys_r_a, sys_r_b, lambda x: x)

report("INDEX 3. Restrategise: same (D, Q, E, C), different selected reduction", [
    f"  source            {sys_r_a.name}",
    f"  target            {sys_r_b.name}",
    f"  value map         identity on {len(SIGNED4)} values, total and injective",
    f"  pattern map       identity, {p_id_3[0]}/{p_id_3[1]} unchanged",
    f"  VALUE ops   add   {v_add_3[0]}/{v_add_3[1]}   (`67` X1 signed add: 192/256)",
    f"  VALUE ops   mul   {v_mul_3[0]}/{v_mul_3[1]}   (`67` X1 signed mul: 111/256)",
    f"  VALUE ops   add   {v_add_3u[0]}/{v_add_3u[1]} on the unsigned window "
    f"(`67`: 136/256)",
    f"  VALUE ops   mul   {v_mul_3u[0]}/{v_mul_3u[1]} on the unsigned window "
    f"(`67`: 80/256)",
    f"  PATTERN ops add   {p_ops_3[0]}/{p_ops_3[1]}",
])

# ------------------------------------------------ index 2: the representable-set crossing

sys_q_a = System("signed4/wrap/2c", dom_Z_add, dom_Z_mul, SIGNED4, "wrap",
                 "twos_complement", 0)
sys_q_b = System("signed8/wrap/2c", dom_Z_add, dom_Z_mul, SIGNED8, "wrap",
                 "twos_complement", 0)


def widen_pattern(x):
    """Sign-extend the four-bit pattern into eight bits."""
    v = dec_twos(x)
    return v & 0xFF


v_add_2 = measure_value_ops(sys_q_a, sys_q_b, lambda v: v, "add")
v_mul_2 = measure_value_ops(sys_q_a, sys_q_b, lambda v: v, "mul")
p_id_2 = sum(1 for x in sys_q_a.patterns() if widen_pattern(x) == x), 16
inj_2 = len(set(v for v in SIGNED4))

sys_q_as = System("signed4/sat/2c", dom_Z_add, dom_Z_mul, SIGNED4, "saturate",
                  "twos_complement", 0)
sys_q_bs = System("signed8/sat/2c", dom_Z_add, dom_Z_mul, SIGNED8, "saturate",
                  "twos_complement", 0)
v_add_2s = measure_value_ops(sys_q_as, sys_q_bs, lambda v: v, "add")
v_mul_2s = measure_value_ops(sys_q_as, sys_q_bs, lambda v: v, "mul")

report("INDEX 2. Widen: same D, larger representable set", [
    f"  source            {sys_q_a.name}",
    f"  target            {sys_q_b.name}",
    f"  value map         inclusion, {inj_2}/16 values carried, total and injective",
    f"  pattern map       sign-extension, {p_id_2[0]}/{p_id_2[1]} unchanged",
    f"  VALUE ops   add   {v_add_2[0]}/{v_add_2[1]} under wrap   (`67` X2 add: 192/256)",
    f"  VALUE ops   mul   {v_mul_2[0]}/{v_mul_2[1]} under wrap   (`67` X2 mul: 101/256)",
    f"  VALUE ops   add   {v_add_2s[0]}/{v_add_2s[1]} under saturate",
    f"  VALUE ops   mul   {v_mul_2s[0]}/{v_mul_2s[1]} under saturate",
])

# ----------------------------------------------- index 1: the ambient-domain crossing

sys_d_z = System("Z/wrap/2c", dom_Z_add, dom_Z_mul, UNSIGNED4, "wrap",
                 "twos_complement", 0)
sys_d_gf = System("GF(2)^4/2c", dom_gf2_add, dom_gf2_add, UNSIGNED4, "wrap",
                  "twos_complement", 0)
sys_d_tr = System("tropical/2c", dom_trop_add, dom_Z_add, UNSIGNED4, "wrap",
                  "twos_complement", 0)

v_add_1gf = measure_value_ops(sys_d_z, sys_d_gf, lambda v: v, "add")
v_add_1tr = measure_value_ops(sys_d_z, sys_d_tr, lambda v: v, "add")
p_id_1 = pattern_identity_fraction(sys_d_z, sys_d_gf, lambda x: x)

report("INDEX 1. Reinterpret: same container and patterns, different ambient operations", [
    f"  source            {sys_d_z.name}",
    "  targets           GF(2)^4 with xor as addition; tropical with min as addition",
    "  value map         NOT TYPED: the two carriers are different sets, so there is no",
    "                    value-level map to preserve anything. What crosses is the pattern.",
    f"  pattern map       identity, {p_id_1[0]}/{p_id_1[1]} unchanged",
    f"  ops vs xor        {v_add_1gf[0]}/{v_add_1gf[1]}   (`67` X3 vs xor: 108/256)",
    f"  ops vs min        {v_add_1tr[0]}/{v_add_1tr[1]}   (`67` X3 vs min: 1/256)",
])

# ------------------------------------------------------------------------- the verdict

rows = [
    ("1  ambient domain", "none", "identity", f"{v_add_1gf[0]}/256 vs xor", "n/a"),
    ("2  representable set", "inclusion", "sign-extend",
     f"{v_add_2[0]}/256", "not measured"),
    ("3  adaptation", "identity", "identity", f"{v_add_3[0]}/256",
     f"{p_ops_3[0]}/256"),
    ("4  encoding", "identity", "bijection", f"{v_add_4[0]}/256", f"{p_ops_4[0]}/256"),
    ("5  container", "identity", "shift", f"{v_add_5[0]}/256", f"{p_ops_5[0]}/256"),
]

print()
print("=" * 78)
print("THE FIVE CROSSING CLASSES, addition")
print("=" * 78)
print(f"{'index':<24}{'value map':<12}{'pattern map':<14}{'VALUE ops':<16}{'PATTERN ops'}")
for r in rows:
    print(f"{r[0]:<24}{r[1]:<12}{r[2]:<14}{r[3]:<16}{r[4]}")

full_value = [r[0] for r in rows if r[3].startswith("256/")]
print()
print(f"Crossings preserving the VALUE-level operation at 100%: {full_value}")
print(f"Crossings preserving the PATTERN-level operation at 100%: "
      f"{[r[0] for r in rows if r[4].startswith('256/')]}")
print()
if full_value:
    print("PREDICTION HELD. `67` section 4's universal, 'No crossing preserves operations at")
    print("100%', is refuted at the two indices it did not measure. The universal was drawn")
    print("from three of five classes.")
else:
    print("PREDICTION REFUTED: no crossing preserves the value operation totally.")
print()
print("And the mirror pair, which is the structural point:")
print("  index 4 preserves the VALUE operation totally and destroys the PATTERN operation")
print(f"    ({v_add_4[0]}/256 against {p_ops_4[0]}/256)")
print("  index 1 preserves the PATTERN map totally and has no value operation to preserve")
print("  index 3 preserves the pattern map AND the untyped pattern operation and destroys")
print(f"    the value operation ({p_ops_3[0]}/256 against {v_add_3[0]}/256), which is exactly")
print("    why it is the crossing that looks free: it is free at every level a machine can")
print("    see, and not free at the level that decides the answer.")
