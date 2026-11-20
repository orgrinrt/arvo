# A const bit trait refuses an index past the width at build, through every path

Round `202611200155` (the bit tier on `arvo-bits`) puts `BitWidth`, `BitAccess`, `BitLogic` and
`BitSequence` in `arvo-bits` as `const` traits whose impls call inherent `const fn`s on `Bits<N>`, and
says an index or amount at or past the width is refused at build through the inherent method and
through the trait. This sketch asks whether that shape compiles on the pinned nightly and whether the
refusal survives the trait.

**Outcome:** WORKS

## What it holds

`src/lib.rs` declares a cut-down `Bits<N>`, a private `Within<N, I>` whose `ADMITTED` asserts `I < N`,
an inherent `bit::<I>` forcing it, and three `const` traits with `const` impls delegating to the
inherent methods. A `const fn` generic over `[const] BitAccess` and one over `[const] BitLogic` are
evaluated in constants, and `<Bits<8> as BitWidth>::WIDTH` is read. Results are typed in
`arvo_format::Bool` and `arvo_format::Width`.

## The runs

`run.sh` builds each target with `cargo build` under the repository's toolchain and keeps the output.

- `outputs/lib.txt`: the library and its four constants build, exit 0.
- `outputs/past_inherent.txt`: `Bits::<8>::masked(1).bit::<8>()` in a constant is `E0080`, "a bit
  index at or past the width", exit 101.
- `outputs/past_generic.txt`: the same index through a `const fn` generic over `[const] BitAccess`
  is refused the same way, exit 101.
- `outputs/past_runtime_generic.txt`: through an ordinary generic function called at run time, the
  refusal comes at codegen of the monomorphised call, exit 101.

So the refusal sits in the inherent method and the trait carries it by delegation, at every path a
caller has, and the crate generic over the tier needs `const_trait_impl` only where it evaluates the
tier in a constant.
