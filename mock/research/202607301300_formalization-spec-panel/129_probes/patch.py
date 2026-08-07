import io
p = "/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202607301300_formalization-spec-panel/129_jhala_canonicity_settled.md"
s = io.open(p, encoding="utf-8").read()

old_time = """**Compile cost, unmeasured and marked as such.** I did not time the capstone at scale against `126`'s
0.04 s figure or `125:245-250`'s table. The construction is structurally lighter than either (five impls,
no recursion, no trait-level arithmetic), so I expect it to be at least as fast, but expectation is not a
measurement and the panel has been bitten by exactly that. If the number matters to the decision, it is one
`/usr/bin/time -p` run away.
"""

new_time = """**Compile cost, measured.** `q15_scale.rs`, 64 distinct compositions at four-digit widths, each a generic
`mul` call through the macro surface with its own compile-time assertion, `/usr/bin/time -p`, three runs:
**0.13 s cold, then 0.05 s, 0.05 s.** Set that against `125:245-250`'s own table for the same workload:
0.06 s through a 4096-row table, 5.87 s through use-site realisation, and `126:229-232`'s 0.04 s for the
structural numeral. The gate-free construction is level with the fastest thing the panel has measured, at
widths no table holds, and it is doing more than `126`'s numeral was because the width law is checked as
well as computed. The negative control fires
(`error[E0080]: evaluation panicked: assertion failed: 1000 + 1000 == 2001`, `q15_neg.rs`).
"""
assert old_time in s
s = s.replace(old_time, new_time)

old_s9 = """- **Compile time at scale.** Section 7 says why and what it would take.
- **Whether the `Fx<P, C, S>` shape survives contact with the rest of the standing base**, specifically
  `S: Policy + Lowering` (`127b:97-102`) and the preset key on the exponent form. The capstone carries `S`
  as an unbounded parameter, which is a placeholder for the real bound, and I did not test whether adding
  it perturbs anything. I do not expect it to, since nothing in the mechanism touches `S`.
- **`mock/research/sketches/202607282100_container-projection-without-gce`.** Named in the brief, not
  opened. It enumerates widths per the brief; the mechanism here does not, at any layer.
- **Whether the post-monomorphisation hole is reachable in practice in arvo's own crates.** `q6` proves it
  is reachable in principle. How often a generic wrapper over a width law appears in real consumer code is
  a question about code I did not read.
- **Signed numerals.** Everything above is `UFixed`-shaped. `IFixed<I, F, S>` carries `1 + I + F`, which is
  one more addition at a concrete site and should behave identically, but I did not compile it.
"""

new_s9 = """- **`mock/research/sketches/202607282100_container-projection-without-gce`.** Named in the brief, not
  opened. It enumerates widths per the brief; the mechanism here does not, at any layer.
- **Whether the post-monomorphisation hole is reachable in practice in arvo's own crates.** `q6` proves it
  is reachable in principle. How often a generic wrapper over a width law appears in real consumer code is
  a question about code I did not read, and it is the input that would settle section 4's fork on evidence
  rather than on taste.
- **The preset key on the exponent form** (`127b:99-102`). Section 10 puts `S: Policy + Lowering` on the
  capstone and nothing moves, but the preset key is a separate piece and I did not reach it.
- **Whether `Fx<P, C, Sign, S>` is the right parameter order or the right factoring**, as against carrying
  the sign inside the container marker. `q16_signed.rs` proves the mechanism is indifferent to which; it is
  a surface question and it belongs with section 5's.
- **The next-solver open bug list**, which `128:78-81` also left unchecked. It bears only on the GCA column
  of section 4's table, and the gate-free column does not touch the solver at all.
"""
assert old_s9 in s
s = s.replace(old_s9, new_s9)

# append section 10 before nothing; insert after section 8 heading block end, i.e. before "## 9."
marker = "## 9. What I did not check"
assert marker in s
sec10 = """## 10. Two checks the file was written without, closed afterwards

Both were listed as outstanding when this file was first written, per the brief's instruction to write
early and extend rather than hold a finished investigation. Both now hold.

**The signed numeral.** `q16_signed.rs`, exit 0, gate-free. `IFixed!(12, 3, i16, Warm)` and
`IFixed!(7, 8, i16, Warm)` are the same type, both precision 16, because the surface macro expands
`1 + $i + $f` at the concrete site exactly as the unsigned one expands `$i + $f`:

```rust
macro_rules! IFixed { ($i:literal, $f:literal, $c:ty, $s:ty) => { Fx<{ 1 + $i + $f }, $c, Signed, $s> }; }
```

The sign marker keeps the two families apart as types while the precision parameter keeps each family
canonical within itself. One extra addition, at a place where addition was never restricted.

**The strategy bound.** `q17_bounded_s.rs`, exit 0, gate-free: the whole capstone with `S: Policy +
Lowering` on every item, which is the bound `127b:97-102` records as converged and never stated anywhere
in six thousand lines. Nothing perturbs, which is the expected result and is worth having compiled rather
than expected, because the panel's record of expected results is not good.

---

"""
s = s.replace(marker, sec10 + marker)
io.open(p, "w", encoding="utf-8").write(s)
print("ok")
