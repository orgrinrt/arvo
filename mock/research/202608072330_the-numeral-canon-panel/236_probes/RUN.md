# 236 probes, run record

Toolchain, passed explicitly on every command per the panel's convention:
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
which is the pin `rust-toolchain.toml` names.

Both probes are about one row, `proposal::a_nonzero_phase_leaves_the_representable_set_without_an
_additive_identity`, and specifically about whether its `says` holds where its predicate does not
reach. The predicate names `total_width`, `fraction_width`, `signedness`, `operation`, `arity` and
`rounding`. It names no quantum and no phase denominator, and those are the two coordinates the
sentence turns on.

## p1_the_phase_clause_over_three_geometries.rs

    rustc +nightly-2026-05-28 -O p1_the_phase_clause_over_three_geometries.rs -o p1 && ./p1
    exit 0, prints P1 WORKS

Output committed as `p1_output.txt`. Exact arithmetic in integer units of 1/24, so nothing rounds.
One affine membership predicate, written once and instantiated at three geometries, each compared
against an enumeration built without consulting the predicate, each with the bias-dropped mutant
detected.

| arm | step | phase | closed under exact addition | contains zero | contains one | every sum half a step away |
|---|---|---|---|---|---|---|
| A | 1/4 | 1/8 | no, 0 of 256 | no | no | yes |
| B | 2 | 1 | no, 0 of 256 | no | **yes** | yes |
| C | 1 | 1/3 | no, 0 of 256 | no | no | **no**, distance is a third of a step |

Arm A is the geometry `56_probes/q2_affine_membership.rs` ran at, `STEP = 8, BIAS = 4` on a scale
of 32, which is step 1/4 and bias 1/8. It reproduces q2's counts: sixteen grid points, zero of 256
sums on the grid, the distance a systematic half step, neither zero nor one on the grid.

Instrument-can-fail, stated before the run and asserted in the file:

1. `contains one` must differ between A and B, or the instrument cannot separate the two
   geometries and its report that a clause is geometry-dependent means nothing. It differs.
2. A phase-zero grid at each arm's own step must contain zero, or "contains no zero" would be a
   fact about the predicate rather than about the phase. All three do.
3. The half-step clause must hold at A and B and fail at C, or the distance arm is structurally
   green. It fails at C.

## p2_the_shipped_crate_admits_the_counterexample.rs

    RL=$(ls mock/target/debug/deps/libarvo_format-*.rlib | head -1)
    rustc +nightly-2026-05-28 -O --edition 2024 \
      --extern arvo_format=$RL -L mock/target/debug/deps \
      p2_the_shipped_crate_admits_the_counterexample.rs -o p2 && ./p2
    exit 0, prints P2 WORKS

Built against `arvo-format` as committed, after `cargo build -p arvo-format` in `mock/`. Output
committed as `p2_output.txt`.

p1 alone could be dismissed as a geometry nobody would declare. p2 closes that: every arm is an
instantiation of `arvo_format::points::Biased`, which is in the shipped inventory and pins
`PHASE_DEN = 2`, so it is the half-step family by construction and leaves the quantum exponent
free.

| arm | format | quantum | phase | contains zero | contains one | `has_additive_identity()` |
|---|---|---|---|---|---|---|
| A | `Biased<4, -2, 1>` | 1/4 | 1/8 | no | no | false |
| B | `Biased<4, 1, 1>` | 2 | 1 | no | **yes** | false |
| C | `Integer<4>` | 1 | 0 | yes | yes | true |
| D | `Biased<4, 0, 2>` | 1 | 1 | **yes** | yes | **false** |

Two separate findings, and D was not what the probe was built for.

Arm B is the counterexample the row's `says` denies: a half-step-biased grid carrying the value
one. The clause "the grid contains neither zero nor one" is true at the quantum the finding was
measured at and false at a quantum above one, and both are `points::Biased`.

Arm D is a defect in the shipped function rather than in the row. `has_additive_identity` reads
`PHASE_NUM == 0` where the property it names is whether the phase is a whole multiple of the
quantum. At `PHASE_NUM = 2` over `PHASE_DEN = 2` the phase is one whole step, the grid is shifted
onto itself, zero is at slot -1, and the function reports no additive identity anyway.
`PHASE_DEN` is read by no function in the crate: `grep -rn 'PHASE_DEN' mock/crates/arvo-format/src/`
returns its declaration, four impls, one test impl, and no use.

Instrument-can-fail, stated before the run and asserted in the file:

1. Arms A and B must disagree about one, or the instrument reports a property of the code rather
   than of the format.
2. Neither A nor B may have an additive identity, or the instrument is reading a different phase
   than the one declared.
3. `Integer<4>`, phase zero from the same inventory, must have one, or the false results above are
   facts about the function rather than about the phase.
4. Arm D must carry zero and the crate must say it does not. Both halves are asserted: if the crate
   agreed there would be nothing to report, and if the lattice had moved the arm would be wrong.

## Honesty notes

Both probes passed on their first complete run. No instrument defect was found and repaired during
this work, so there is no kept-defect trail; the can-fail demonstrations are the four controls and
the three mutants named above.

**What is argued and not probed.** That arm B's geometry is one a consumer would declare. Nothing
here says anybody wants a quantum of two with a half-step phase; what is established is that the
concept admits it, the shipped inventory expresses it, and the sentence is false there. Under the
predicate discipline that is enough, because an unqualified sentence claims every region.

**Width transfer is not probed either.** Both probes run at four slots' worth of bits in the crate
arms and sixteen grid points in p1. The clauses are arithmetic in the quantum and the phase and do
not read the width, which is an argument rather than a sweep, and it is the same argument
`56_probes/RUN.md` makes about its own counts.
