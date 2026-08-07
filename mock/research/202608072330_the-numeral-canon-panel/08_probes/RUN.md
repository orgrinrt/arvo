# 08 probes: every command, its exit code, and what it is for

Toolchain, checked rather than assumed:

    $ cat ../../../../rust-toolchain.toml
    channel = "nightly-2026-05-28"
    $ rustc +nightly-2026-05-28 --version
    rustc 1.98.0-nightly (57d06900f 2026-05-27)

Nothing here is a benchmark. No bench harness ran. Every number below is a count
produced by the named command, and every magnitude in the file that cites these
is called **unpriced**.

## The instruments

| File | Language | What it decides |
|---|---|---|
| `i1_classify.py` | Python, exact rationals | first classifier; **carries three defects, kept** |
| `i1b_classify.py` | Python, exact rationals | the classifier with those defects repaired |
| `i2_lattice.py` | Python, exact rationals | closure of the general class under both operations |
| `i2b_valid_shapes.py` | Python, enumeration | how many exponent shapes there are; **carries a defect, kept** |
| `i2c_join_closure.py` | Python, enumeration | meet-closure against join-closure, on i2b's shape set |
| `i2d_per_family.py` | Python, enumeration | per family rather than the union; surfaced i2b's defect |
| `i2e_corrected_window.py` | Python, enumeration | the same counts with the defect repaired; supersedes i2b/i2c/i2d |
| `i3_encoding_and_denotation.py` | Python, enumeration | the two things a value-set test cannot see |
| `p3_segmented_typestate.rs` | Rust, pinned nightly | can a segmented format live in the typestate, gate-free, and erase |

## Commands

    $ python3 i1_classify.py > i1.out                       # exit 0
    $ python3 i1b_classify.py > i1b.out                     # exit 0
    $ python3 i2_lattice.py > i2.out                        # exit 0
    $ python3 i2b_valid_shapes.py > i2b.out                 # exit 0
    $ python3 i2c_join_closure.py > i2c.out                 # exit 0
    $ python3 i2d_per_family.py > i2d.out                   # exit 0
    $ python3 i2e_corrected_window.py > i2e.out             # exit 0
    $ python3 i3_encoding_and_denotation.py > i3.out         # exit 0

    $ rustc +nightly-2026-05-28 --crate-type=lib --edition 2021 -O \
        -C codegen-units=1 p3_segmented_typestate.rs -o p3.rlib
      # exit 0, after two repairs recorded below. The rlib is a build
      # artifact and is not kept; `p3.s` is.

    $ rustc +nightly-2026-05-28 --crate-type=lib --edition 2021 -O \
        -C codegen-units=1 --emit=asm p3_segmented_typestate.rs -o p3.s
      # exit 0; p3.s is 22 lines and is quoted whole in p3_asm.out

    $ rustc +nightly-2026-05-28 --crate-type=lib --edition 2021 \
        p3_negctl2.rs -o /dev/null
      # exit 1 by design; the negative control

## The defects, kept rather than repaired in place

**i1, defect one, and it is the one worth reading.** i1 tested whether every
value is a multiple of `r^fexp(e)`, which is Flocq's `generic_format` and is
**not** the design's value map. The design's map is affine,
`Adjustment * radix^exponent * k + Bias`, so a numeral may sit at a phase. i1
therefore called a half-unit-biased format OUTSIDE the concept when the design
admits it, and the droplist records the correction that admits it. i1b tests
both and reports them as separate columns. The gap between the two is a finding
and it would not have surfaced without the wrong test being run first.

**i1, defect two.** A binade holding one value pins no step, so i1's "take the
value's own valuation" branch invented a canonical exponent wherever a range was
truncated and reported tapering that was an artifact of the top binade. That is
the same phenomenon `02_carried` section 1.6 reports for the inclusion predicate
one level up. i1b marks such binades UNCONSTRAINED.

**i1, defect three.** i1's double-double generator drew the low part from the
same float family as the high part, so no nonzero low part ever qualified and
the set it classified was the plain float. i1b draws from an extended exponent
range, and the answer flips from INSIDE to OUTSIDE.

**i2b, and it is the enumeration-bound failure `RULES.md` warns about by name.**
i2b bounded the canonical exponent by `fexp(e) >= e - depth`, a bound that moves
with the binade, which silently forbids a constant at any window above four.
i2d then reported `fixed alone n=0` and I read it as a fact about fixed-point
formats when it was a fact about my own enumeration. i2e repairs it with a fixed
precision floor and its counts supersede i2b's, i2c's and i2d's. The shape of
the answer survives; the counts do not.

**p3, first build.** I asserted `Tapered: FinerThan<Underflow>`, which is false:
the tapered shape is (0,2,4) and gradual underflow is (1,1,2), so the tapered
one is coarser at the second binade. The compiler refused it and named the
offending pair, `S<Z>: AtMost<Z>`. That build is `p3_negctl2.out`'s sibling
`p3_negctl.out` and the false claim is kept in the source as a comment rather
than deleted.

**p3, second build.** `check_order` was declared `const fn` and called a
non-const helper, refused at `E0015`. Changed to a plain `fn`. That one is an
ordinary mistake with nothing to learn from and is recorded only for the count.

## What p3 established

Gate-free: `grep -n '^\s*#!\[feature' p3_segmented_typestate.rs` returns
nothing, so no forbidden feature is reachable from it.

The ordering refuses at type check rather than at monomorphisation, with a
diagnostic naming the binade where the ordering fails (`p3_negctl.out`).

The type-equality assertions are not vacuous: `p3_negctl2.out` claims the meet
of the fixed shape and the float shape is the float, and the compiler refuses it
while printing the meet it actually computed, `Cons<S<Z>, Cons<S<Z>, Cons<S<S<Z>>,
Nil>>>`, which is (1,1,2), gradual underflow.

It erases. `p3.s` is twenty-two lines. One symbol carries a body and the body is
a single `ret`; six further symbols, including the unguarded baseline
`widen_bare` and all three guarded call sites across a fixed, a float and a
tapered shape, are aliases onto it.

## One further check, run inline

    $ python3 -c "
    from i2e_corrected_window import shapes, families, close_under
    A=shapes(5,5); fx,fl,kn=families(5,5,A)
    M=close_under(max, fx|fl, A)
    print('meet closure of fixed+float:', len(M))
    print('is it inside the gradual-underflow family:', M <= kn)
    print('gradual-underflow family size:', len(kn))
    J=close_under(min, fx|fl, A)
    print('join closure of fixed+float:', len(J), 'inside knee family:', J <= kn)
    " > i2f_meet_lands_in_knee.out          # exit 0

It reports that the meet closure of the fixed and float shapes is twenty-four
shapes, all inside the gradual-underflow family, and that the join closure is a
different twenty-four which is not.

## What is not here

No bench harness ran, so nothing in `08` is priced and the word appears in the
file rather than a number. The compile-time cost of carrying a canonical
exponent as a type-level list at a realistic exponent span is the measurement
the file names as the one it would take first, and it was not taken.
