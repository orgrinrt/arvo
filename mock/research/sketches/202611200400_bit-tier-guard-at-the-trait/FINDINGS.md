# The index guard of the bit tier, owned by the tier

The second review of the bit tier (`mock/research/202611200325_review-arvo-bits-tier/PASS_2.md`,
finding 2) says the refusal of an index or an amount at or past the width can be enforced by the
tier rather than owed by each implementor, as `src/tier.rs` and the crate design said it could not.
This sketch asks two things: does the shape the reviewer built hold, and does a sealed form of it
hold, where no implementor can skip the refusal by writing the caller's method itself.

**Outcome:** WORKS. Both shapes refuse at build an index past the width of a container written with no
assertion of its own, in a call at run time, through a generic function and through a `[const]` bound
in a constant. The sealed shape also refuses an impl of the caller's trait by hand and a proof built by
hand.

## The two shapes

`reviewer/` is the reviewer's spike as it was handed back, moved here with its paths made relative. A
token `At<B, I>`, private field, whose `NEW` asserts `I < B::WIDTH.count()`; the trait's required
method takes the token, and a provided method `bit::<I>` forces it. The provided method can be
overridden, so an implementor can still write a `bit` with no guard.

`guard/` seals it. The proof is `Within<B, I>`, made only by `NEW`, and two traits split the sides:
`BitAccessAt`, which an implementor writes and whose methods take the proof, and `BitAccess`, which a
caller names and whose methods take the index alone. `BitAccess` has `BitAccessAt` as its supertrait and
one impl, `const impl<B: [const] BitAccessAt> BitAccess for B`, which makes the proof. Any other impl of
`BitAccess` is either for a type that implements `BitAccessAt`, which overlaps the blanket impl, or for
one that does not, which the supertrait refuses.

## What the runs show

`run.sh` builds and runs every example of both crates; `outputs/reviewer.txt` and `outputs/guard.txt`
hold one line per example: the exit status and the first error or the run's last line.

| Case (`guard/`) | Exit | What |
|---|---|---|
| `wide_last_builds` | 0 | a 256-bit set with no assertion of its own, index 255 written and read |
| `wide_past_refused` | 101 | index 256 at run time, E0080 from `Within::NEW` |
| `wide_generic_past_refused` | 101 | index 256 through `fn top<B: BitAccess>`, E0080 |
| `in_a_constant_builds` | 0 | `[const] BitAccess` in a constant, index 7 of eight bits, 255 of 256 |
| `in_a_constant_past_refused` | 101 | index 8 of eight bits there, E0080 |
| `caller_trait_by_hand_refused` | 101 | `impl BitAccess` by hand beside `BitAccessAt`, E0119 |
| `caller_trait_without_implementor_refused` | 101 | `impl BitAccess` with no `BitAccessAt`, E0277 |
| `proof_by_hand_refused` | 101 | `Within(PhantomData)` outside the crate, E0423 |

The reviewer's `reviewer/` runs as its own results said: the nine edge refusals of the shipped
`Bits<N>` (one bit at index 1, sixty-four bits at 64 for each of the six methods, `bit::<{u32::MAX}>`,
`shift_down::<9>` at eight bits) exit 101 with the shipped message, and its 256-bit set is refused
past the width and builds at 255.

## What it decides, and what it does not

holds for: the toolchain `nightly-2026-05-28`, `const_trait_impl`, `arvo-bits` and `arvo-format` at
`28a85372`, `cargo build` (examples, debug profile), the read and the write of one bit. The amounts of
`BitSequence` take the same proof and are not built here.

It decides that the tier can own the guard once, and that the sealed form is the one where nothing can
go round it. It is one instance and says nothing of what the split costs a consumer who only names
`BitAccess`, beyond that the generic and the `[const]` calls above build unchanged.
