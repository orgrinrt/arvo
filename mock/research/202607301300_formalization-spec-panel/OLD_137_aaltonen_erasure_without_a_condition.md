# 137. Erasure without a condition, and what it is actually quantified over

**Persona:** Sebastian Aaltonen, codegen and memory-layout lens. Third pass in this panel; file 32 was
whether identity lowers well, file 57 the measurement debt, file 75 what bitpacked means, file 96 the
footprint bench.
**Date:** 2026-08-07
**Position:** answers the acceptance gate at `135b:12-16`. Reads `135b`, `132` sections 4.1 to 4.4, `133`,
`134`, `134c`, `131`, op's checkpoints `130b` and `127b`, and `110`'s container material.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, `-O`, aarch64-apple-darwin, LLVM
22.1.6. Every number below is an instruction count read off emitted assembly. Scratch tree outside the
repository at `/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/aal137/`, sources copied to
`137_probes/`. `mock/crates` read, never written.

## The verdict

**The condition as stated is not the condition, and the real one is closable.** `132` reported it as "the
operation body names a machine type". Measured, that is neither necessary nor sufficient: a body that names
`u64` as a **limb** and loops over eight of them costs 66 instructions where native costs 2, and a body that
names no machine type in its own text at all lowers to a single `add` when the limb it resolves to is one
register wide. The condition is a statement about the payload, not about the body: **a numeral erases to a
single machine instruction exactly when its payload is one limb of a type the hardware adds in one
instruction.** The ladder is the function that guarantees that, and I built one that is total, gate-free,
and enumerates no width anywhere.

**So the erasure is unconditional, in the only sense a machine can deliver it.** At and below 128 bits the
lowering is byte-identical to the native primitive, which the probe demonstrates by LLVM folding the two
into one symbol. Above 128 bits there is no native primitive to be identical to, and the honest bar is what
a bignum author writes by hand at that width. Against that bar the derived operation is **10 instructions
against 10 at 192 bits, 11 against 11 at 256, 21 against 21 at 512, and 43 against 41 at 1024**, with the
same instruction mix in every case. `132:294`'s thirty-three times is real and it is a property of a body
written over byte limbs, not a property of the wide rung. Over the right limb it is zero.

One thing remains, and it is a choice rather than a caveat. Section 5.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts op's own calls in the governing position. The calls that
bear here are the acceptance gate (`135b:12-16`), the container ruling (`130b:39-48`), the no-ceiling call
(`127b:118-126`), the enumeration refusal (`127b:36-50`) and D48's surface (`127b:56-59`). Nothing below
asks op to reopen any of them, and section 5 is the one place I hand a choice back rather than take it.

**Test gate.** Not run, and I name it rather than let it pass. `126:47-48` ran
`cargo test --offline --workspace` and got 672 passed, 0 failed, 9 ignored; `129` through `134` each
declined to re-run it on `108b:174-181`. `git status --porcelain` in the repository shows one untracked
file, this one, so the tree has not moved since. My deliverable touches no crate. The instrument here is the
compiler, and every construction below carries a negative control, because a construction whose assertions
cannot fail is not evidence.

**Toolchain.** `rustc +nightly-2026-05-28 --version --verbose` reports
`1.98.0-nightly (57d06900f 2026-05-27)`, `LLVM version: 22.1.6`, host `aarch64-apple-darwin`. Matches the
brief.

### The brief's factual claims

*"Sixty-six instructions against two at eight bytes (`132:308-318`)."* **Holds, reproduced.** I rebuilt
`q9`'s body verbatim in `p1_condition_shape.rs` and counted 66 static instructions for `a_ripple_b8` against
2 for the native `u64` add. I also rebuilt it with `u8::carrying_add` instead of `132`'s `|`-combined
overflow flags, in case the carry combination was the artifact: `b_carrying_b8` is also **66**, and LLVM
folded it with a third spelling into one symbol (`_d_limb_u8x8 = _b_carrying_b8`). So the figure is about
byte limbs and not about how the carry was spelled.

*"LLVM's identical-code-folding collapsing the native functions into the arvo ones so the object file
defines three symbols."* **Holds, reproduced independently.** Compiling `transp/t.rs` at `--crate-type=lib`
on the pin gives exactly three bodies and three aliases:

```
_native_vec = _arvo_vec
_native16   = _arvo16
_native64   = _arvo64
```

(The brief's invocation omits `--crate-type=lib`, which fails with `#[panic_handler] function required`.
Cosmetic, recorded so the next reader does not chase it.)

*"`131:282-284` reports the shipped ladder is total because the wide rung is parameterised by a byte count,
and `133:651-653` reports the wide rung's array length is refused, which are in tension. Settle it."*
**They are not in tension, and the appearance of tension is the brief's, not the files'.** `131:282-284`
describes the **const-keyed** encoding, where `BYTES` is a `const` generic and `[u8; BYTES]` is a standalone
argument, which compiles with no gate. `133:651-653` describes the **structural** encoding, where the byte
count is a type and `[u8; <B as Nat>::V]` is a const operation on a type parameter, which rustc refuses in
as many words at `133:414-416`. Two different encodings, two correct reports, no conflict. `133:423-427`
already draws the table that says so; the brief compressed it into a contradiction.

What is real underneath is narrower and worth stating in one line, because it is what section 4 closes:
**the structural encoding, which is the route with no cap, could not express the wide rung's payload.** That
was a genuine hole. It is not a hole about totality.

*"`132:322-337` gives a gate-free ladder."* **Holds, with a citation correction.** The ladder source is at
`132:307-318`; `132:322-326` is its emitted assembly and `132:335-345` is the seam. I re-ran
`lat132/q15_op_dispatch.rs` and it still exits 0 with no gates.

*"Op has overturned five converged panel conclusions this session."* Not checked, and not load-bearing for
anything below.

---

## 1. What the condition actually is

I ran five bodies over the same eight-byte payload, from "no machine type anywhere in the source" to "the
ladder supplies one", plus a limb-generic body where the machine type appears one level down in a trait
impl rather than in the operation. `p1_condition_shape.rs`, no features, no flags, exit 0. Static
instruction counts off the emitted assembly, aarch64:

| Body | Machine type in the operation? | Insns | Native bar |
|---|---|---|---|
| `a_ripple_b8`, `132`'s ripple carry over `[u8; 8]` | no | 66 | 2 |
| `b_carrying_b8`, same with `u8::carrying_add` | no | 66 | 2 |
| `c_nocarry_b8`, per-byte adds, **no carry chain at all** | no | 31 | 2 |
| `d_limb_u16x4`, generic over `L: Limb`, at `L = u16`, 4 limbs | no, `u16` is in the `Limb` impl | 27 | 2 |
| `d_limb_u64x1`, the same generic body at `L = u64`, 1 limb | no, `u64` is in the `Limb` impl | **2** | 2 |
| `e_ladder_b8`, `132`'s ladder | yes, via `<Rung<B> as Machine>::M` | **2** | 2 |

Two rows break the reported condition, in opposite directions.

**`c_nocarry_b8` breaks the "it is the carry chain" reading.** That body has no carry chain: eight
independent `wrapping_add`s on eight bytes, semantically not a wide add, included only to isolate the
variable. It is still **31 instructions**, fifteen times native. So the carry chain is not what costs; the
byte-granular layout costs on its own, because every lane has to be extracted, added and re-inserted.

**`d_limb_u64x1` breaks the "the body must name a machine type" reading.** Its body is
`while i < N { let (s, c) = self.0[i].carrying(o.0[i], carry); ... }`, generic over `L: Limb` and
`const N: usize`, with no machine type in its text. At `L = u64, N = 1` it is **2 instructions**, and LLVM
folded it with the native add into one symbol:

```
_e_ladder_b8   = _d_limb_u64x1
_nat_64        = _d_limb_u64x1
_nat_16        = _e_ladder_b2
```

Three functions, one body. The ladder's output and the honestly-generic limb loop and the bare `u64` add are
literally the same machine code, and the object file says so.

So the condition, in the form that survives all six rows:

> **A numeral's operation lowers to the native instruction exactly when its payload is one limb of a type
> the hardware operates on in one instruction.** Where the machine type is *named* is irrelevant: it may be
> in the operation, in a trait impl one level down, or nowhere the author wrote it. What matters is that
> monomorphisation ends with a single register-width value.

That is a materially different statement from `132:299-300`'s, and it changes what the design has to
deliver. `132`'s form makes the condition sound like a discipline the operation author must observe, which
is the reading op rejected at `135b:65-68`. The measured form makes it a property of the **payload**, which
is derived, which is exactly the thing op says the typestate has or can be given everything to derive.

The vector case says the same thing in the shape a consumer would actually hit. 1024 elements of a 2-byte
numeral:

| Loop | Insns in the emitted function | Vectorised |
|---|---|---|
| `v_nat_16`, native `u16` | 16 | yes, `add.8h`, eight lanes |
| `v_ladder_b2`, the ladder | **16** | yes, `add.8h`, eight lanes |
| `v_carrying_b2`, byte limbs | 56 | deinterleaved byte lanes |
| `v_ripple_b2`, `132`'s spelling | 60 | deinterleaved byte lanes |

`v_ladder_b2` and `v_nat_16` are both 16. The ladder autovectorises identically.

### 1.1 The premise this file was told to attack, and what is wrong with it

The brief points at the step A / step B seam and `133:72-73`'s one-sentence flag. Here is the defect, and it
is not the one either file names.

**Step B is not "rung to machine type". At the wide rung there is no machine type**, so the seam as drawn
says the derivation terminates in something that does not exist for every width above 128 bits (above 64
for Warm and Precise, per `131:270-280`). Four files have carried "step B is gate-free with
native-identical codegen" as settled, and it is true only on rungs 0 to 4. On rung 5 the sentence has no
referent.

Read that way, the seam does not explain the erasure, it hides where the erasure was never defined. And
that is the whole reason the caveat looked bounded to the panel and unacceptable to op: the panel was
comparing against a target that stops existing at 129 bits, and then reporting the region where the target
does not exist as a caveat on the mechanism.

The repair is one word. **Step B is payload to operation**, and the bar it is measured against is not "the
native instruction" but "what a competent author writes by hand at that width". At and below 128 bits those
two are the same thing, which is why nobody noticed. Above 128 bits they are not, and section 3 measures
the design against the right one.

---

## 2. Is the ladder total

Three separate questions live under this heading and the panel has been running them together.

**Step B is total over rungs, trivially, and that was never the question.** Six rungs, six impls, a finite
enumeration over a six-element set. `132`'s `q15` shows four of them; adding the other two is arithmetic.

**Step A is not total gate-free under const keying**, and `133` established this to a standard I am not
going to improve on: thirty compiled positions, one diagnostic, and the binding-time argument at
`133:239-251` that says why enumerating positions cannot find an exception. The gate-free const-keyed
`ToNat` bridge is one impl per width (`133:373-380`), which is an enumeration.

**Step A is total gate-free under structural keying, including the wide rung, and that is new.** `133 §3`
built the native half and left the wide half as its cost two (`133:410-417`), reporting that the byte count
is a type and a type cannot be an array length. That is correct about arrays. It is not correct that the
payload needs one.

`p5_total_ladder.rs`, no `#![feature]`, no `-Z` flag, `no_std`, **exit 0, 0.04 s**, and **not one width
enumerated anywhere in it**:

- a little-endian binary structural nat (`Term`, `D0<T>`, `D1<T>`), whose value is readable in value
  position through `Nat::V`
- `Dec`, two impls, so both selectors key on `W - 1`, which is what separates 8 from 9 and 128 from 129
- `Len`, three impls, the digit count, which is the native rung selector
- `Rung`, eight literal-tally impls for rungs 0 to 4 plus **one structural catch-all**
  `impl<T> Rung for S<S<S<S<S<S<S<S<T>>>>>>>>`, so there is no largest width and no ceiling
- `Shr6` through `Shr1`, sixteen impls, floor-division by 64
- `WordCount`, **one total impl**, computing `ceil(W/64)` as `floor((W-1)/64) + 1`
- `Build`, two impls, turning the word tally into the wide payload

The native ladder is checked as **type equalities**, not as assertions about them, so the file does not
build if any rung is wrong:

```rust
pub fn c3(x: <W3 as Container>::C) -> u8 { x }        // 3 bits   -> u8
pub fn c8(x: <W8 as Container>::C) -> u8 { x }        // 8 bits   -> u8
pub fn c9(x: <W9 as Container>::C) -> u16 { x }       // 9 bits   -> u16
pub fn c13(x: <W13 as Container>::C) -> u16 { x }     // 13 bits  -> u16
pub fn c16(x: <W16 as Container>::C) -> u16 { x }     // 16 bits  -> u16
pub fn c33(x: <W33 as Container>::C) -> u64 { x }     // 33 bits  -> u64
pub fn c64(x: <W64 as Container>::C) -> u64 { x }     // 64 bits  -> u64
pub fn c65(x: <W65 as Container>::C) -> u128 { x }    // 65 bits  -> u128
pub fn c128(x: <W128 as Container>::C) -> u128 { x }  // 128 bits -> u128
```

and the wide ladder as compile-time assertions on the derived word count and the derived payload's size:

```rust
const _: () = assert!(<<N64   as WordCount>::W as Tally>::N == 1);
const _: () = assert!(<<N65   as WordCount>::W as Tally>::N == 2);
const _: () = assert!(<<N128  as WordCount>::W as Tally>::N == 2);
const _: () = assert!(<<N129  as WordCount>::W as Tally>::N == 3);
const _: () = assert!(<<N200  as WordCount>::W as Tally>::N == 4);
const _: () = assert!(<<N1024 as WordCount>::W as Tally>::N == 16);
const _: () = assert!(core::mem::size_of::<P1024>() == 128);
```

**Negative controls, both fire.** Changing `c64`'s return to `u128` gives
`expected u128, found u64` at that site and nowhere else. Changing the 200-bit word count to 3 gives
`error[E0080]: evaluation panicked: assertion failed`. So neither family of check is decorative.

**Answer: the ladder is total, gate-free, with nothing enumerated, and it is total including the wide rung.**
The quantifier is over every width the structural nat can express, which is every width, because the
catch-all is structural rather than a largest row.

### 2.1 A correction owed to `133 §1.2`

`133:161-180` reports that two blanket impls over the same self type, separated only by disjoint
where-clauses, are accepted, and calls it "the point". My first `WordCount` used exactly that shape, an
`AllZero` arm and an `AnyOne` arm over the peeled low digits, and rustc refused it:

```
error[E0119]: conflicting implementations of trait `WordCount`
215 | impl<T> WordCount for T where <T as Shr6>::R: AllZero, ...
    | ---------------------------------------------------- first implementation here
226 | impl<T> WordCount for T where <T as Shr6>::R: AnyOne, ...
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
```

(`p5_merged.rs`.) `133`'s result holds for a **constrained** self type, `Rung<N>`, which is what its `q9`
probes used. It does not hold for `impl<T> Tr for T`, because coherence does not reason negatively and
cannot see that `AllZero` and `AnyOne` are disjoint. The record should carry the narrower claim, since a
reader who finds `133 §1.2` will try the general form first, as I did.

The repair is better than the mechanism it replaces. `ceil(W/64) == floor((W-1)/64) + 1` for `W >= 1`, so
the round-up needs **no classifier at all** and there is nothing for coherence to resolve. `Dec` is already
on hand for the native rung, so the word count reuses it and `WordCount` is one total impl. This is the
same shape as every other working construction in this panel: the way past a refusal is not another
position, it is to not need the operation.

---

## 3. The wide rung's real price, measured

`132:652-655` flagged that nobody had priced how much of the thirty-three times is inherent above 128 bits
and how much a `u64` limb would recover. It is worth pricing because the shipped wide carrier is
`WideBits<BYTES, A>` over `pub bytes: [u8; BYTES]` (`arvo-strategy/src/widebits.rs:103-110`), so a body
written naively against that carrier is exactly `132`'s byte-limb loop.

**The bar.** Above 128 bits no hardware has a single-instruction add, so "native" is not available and
comparing to it is comparing to nothing. The bar is what a bignum author writes by hand: a chain of
`adds` / `adcs` over machine words. I wrote that out explicitly at each width and used it as the reference.

`p2_wide_rung.rs` and `p3_wide_alignment_and_tail.rs`, no features, no flags, exit 0. Static counts, plus
dynamic counts where the emitted code is a loop, because a fifteen-instruction loop body running
128 times is not a thirty-two-instruction function:

| Width | hand-written | `[u8; BYTES]`, byte limbs | `[u8; BYTES]`, word-chunked body | `[u64; W]` payload |
|---|---|---|---|---|
| 192 bits | 10 | 128 dynamic | 10 | 10 |
| 256 bits | 11 | **425 dynamic** (22 static, 32-iteration loop) | 11 | **11, byte-identical** |
| 512 bits | 21 | 127 dynamic | 127 dynamic | 21 |
| 1024 bits | 41 | **1683 dynamic** (32 static, 128-iteration loop) | 245 dynamic | 43 |

Read across the 256-bit row. The byte-limb body is **38.6 times** the hand-written form, which is
`132:294`'s thirty-three times reproduced at a different width and confirms that figure holds at the wide
rung. The same width over a `[u64; 4]` payload is 11 instructions and, checked line by line, **byte-identical
to the hand-written function**:

```
_bar_256_handwritten:            _w_w64_256:
  ldp x9, x10, [x0]                ldp x9, x10, [x0]
  ldp x11, x12, [x1]               ldp x11, x12, [x1]
  adds x9, x11, x9                 adds x9, x11, x9
  adcs x10, x12, x10               adcs x10, x12, x10
  ldp x11, x12, [x0, #16]          ldp x11, x12, [x0, #16]
  ldp x13, x14, [x1, #16]          ldp x13, x14, [x1, #16]
  adcs x11, x13, x11               adcs x11, x13, x11
  adc x12, x14, x12                adc x12, x14, x12
  stp x9, x10, [x8]                stp x9, x10, [x8]
  stp x11, x12, [x8, #16]          stp x11, x12, [x8, #16]
  ret                              ret
```

**So the answer to the brief's question three is: none of the thirty-three times is inherent at the wide
rung.** It is entirely a property of a body written over byte limbs, and it disappears over machine-word
limbs. The caveat `132` recorded is a claim about a badly written body, exactly as the brief suspected.

Three further results from the same probes, all of which bear on how the wide payload should be shaped.

**Alignment does not matter, and that is worth knowing because the shipped design spends a design decision
on it.** At 192 bits the align-1 carrier and the align-16 carrier emit **identical code, 10 instructions
each**, both matching the bar (`p3`, `a1_192` and `a16_192`). aarch64's `ldur` handles the unaligned case at
no instruction cost. The `A16` alignment for Hot at `container.rs:24-27` is justified as the SSE2 and NEON
baseline, which is about vector loads and is a different claim; it buys nothing for the scalar carry chain,
and nothing here argues against it.

**The ragged tail costs three to four instructions, once, not per limb.** 136 bits is 17 bytes, two words
plus one byte, and the derived add is **10 instructions**: two `adds`/`adcs` on `x` registers and one `adc`
on a `w` register (`p3`, and `p4`'s `st_136`). 200 bits is 25 bytes and costs 14. Against that, rounding the
payload up to whole words costs 11 at 200 bits and 7 bytes of footprint. That is a real trade, it is small
in both directions, and it belongs to op rather than to me, because the footprint side of it is exactly
what `arvo-toolbox-not-policer.md` says Cold exists to protect.

**The limb type has to be in the carrier, not only in the body.** At 512 bits, a `[u8; 64]` carrier with a
word-chunked body stops unrolling and emits a 14-instruction loop over 8 iterations, 127 dynamic, against
21 for a `[u64; 8]` payload. The `from_le_bytes` reads at byte offsets defeat the unroller
(`w_chunk_1024` emits `ldur x15, [x10, #-3]` inside a loop). So the recommendation that falls out of the
measurement is not "keep `[u8; BYTES]` and write a smarter body". It is **make the wide payload machine
words plus a byte tail**, which is what section 4 builds.

---

## 4. The structural wide payload, which closes `133`'s cost two

`133:410-417` reported that the structural encoding cannot reach `[u8; <B as Nat>::V]`, sketched a
`#[repr(C)]` byte cons as a possible repair, and explicitly marked it unbuilt
(`133:433-434`, and again at `133:651-653`). I built it, and it is better than the sketch because the cells
should be words rather than bytes.

`p4_structural_wide.rs`, no features, no flags, exit 0:

```rust
#[repr(C)] pub struct WNil;
#[repr(C)] pub struct WCons<T> { pub w: u64, pub rest: T }

pub trait WAdd: Copy { fn add_c(self, o: Self, carry: bool) -> (Self, bool); }
impl WAdd for WNil { fn add_c(self, _o: Self, c: bool) -> (Self, bool) { (WNil, c) } }
impl<T: WAdd> WAdd for WCons<T> {
    #[inline] fn add_c(self, o: Self, carry: bool) -> (Self, bool) {
        let (s, c) = self.w.carrying_add(o.w, carry);
        let (rest, c2) = self.rest.add_c(o.rest, c);
        (WCons { w: s, rest }, c2)
    }
}
```

**There is no array length anywhere.** The size comes from `#[repr(C)]` layout of the cons chain, so the
type-in-array-length refusal never arises. The byte-exact footprint comes back with a `#[repr(C, packed)]`
byte cons for the tail, and the whole thing is asserted rather than argued:

```rust
const _: () = assert!(core::mem::size_of::<R136>() == 17);   // 136 bits, byte-exact
const _: () = assert!(core::mem::align_of::<R136>() == 1);   // matches WideBits<17, A1>
const _: () = assert!(core::mem::size_of::<R200>() == 25);
const _: () = assert!(core::mem::size_of::<W16>() == 128);
```

`WideBits<17, A1>` is exactly what `arvo-storage/src/layout_assertions.rs:114` asserts the shipped Cold wide
bucket to be at 129 bits, so the structural payload reproduces the shipped footprint, not an approximation
of it.

Codegen against the hand-written bar, at four widths:

| Width | hand-written | structural cons | delta |
|---|---|---|---|
| 192 | 10 | 10 | 0 |
| 256 | 11 | 11 | 0 |
| 512 | 21 | 21 | 0 |
| 1024 | 41 | 43 | +2 |
| 136, ragged, 17 bytes | 10 | 10 | 0 |
| 200, ragged, 25 bytes | 11 (word-rounded) | 14 | +3 |

The three zero-delta rows are equal in count and in instruction mix but not byte-identical: the structural
version hoists all its loads before the carry chain rather than interleaving them. Same `ldp` count, same
`adds` / `adcs` / `adc` count, same `stp` count, different schedule. At 1024 bits that hoisting spills one
register pair, which is the whole of the +2 (`stp x20, x19, [sp, #-16]!` and its restore). Under
`arvo-compile-time-last.md`'s asymmetry that is a 4.9% static cost at 1024 bits and zero below 512, against
a mechanism with no gate, no flag and no cap.

**And the byte count is still readable in value position**, which is what anything downstream needs it as:

```rust
pub trait WLen { const BYTES: usize; }
impl WLen for WNil { const BYTES: usize = 0; }
impl<T: WLen> WLen for WCons<T> { const BYTES: usize = 8 + T::BYTES; }
const _: () = assert!(<R200 as WLen>::BYTES == 25);
```

So the structural encoding does not lose the number. It loses only the ability to put the number in an
array length, and the array was never necessary.

Combined with section 2, that is the whole ladder: `p5_total_ladder.rs` derives the word count from the
width by structural ceil-division and feeds it to `Build`, which emits this payload, and the resulting
`derived_192`, `derived_256` and `derived_1024` are the 10, 11 and 43 in the table above. **One
construction, gate-free, from a written width to an operation that lowers at the hand-written bar, at every
width, with nothing enumerated.**

---

## 5. All four parts of the gate, on one crate

Sections 1 through 4 are pieces. This is them assembled, because the gate is "all four at once" and pieces
that each work are not that. `p6_surface_end_to_end.rs`, one crate, no `#![feature]`, no `-Z` flag,
`no_std`, **exit 0, 0.04 s**.

**Part one, the consumer expresses usage in bits and bytes.** The written surface is D48's, unchanged:

```rust
pub fn arvo16(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> { a.add(b) }
```

Three parameters, literal widths, literal spelling (`127b:56-59`).

**Part two, the typestate derives the container.** The numeral is `#[repr(transparent)]` with one real field
whose type is a projection, and nothing in the source says what it is:

```rust
#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, S, M = Arvo>
where /* .. */
{
    raw: Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>,
    _m: PhantomData<(S, M)>,
}
```

The bridge turns each written coordinate into a structural nat, the nats add structurally, and section 2's
ladder picks the container from the sum. Nothing is written by the consumer and no width is enumerated in
the ladder.

**Part three, it validates.** Asserted, with negative controls that fire:

```rust
const _: () = assert!(size_of::<UFixed<13, 3, Hot>>()  == size_of::<u16>());
const _: () = assert!(align_of::<UFixed<13, 3, Hot>>() == align_of::<u16>());
const _: () = assert!(size_of::<UFixed<40, 24, Hot>>() == size_of::<u64>());
const _: () = assert!(size_of::<UFixed<3, 0, Hot>>()   == 1);
const _: () = assert!(size_of::<UFixed<100, 100, Hot>>() == 32);   // 200 bits, four words
```

Changing the first to `u32` gives `E0080: evaluation panicked`. A width the bridge does not carry refuses at
the type, before any body runs:

```
error[E0277]: the trait bound `Idx<7>: ToNat<Arvo>` is not satisfied
151 | pub fn unbridged(a: UFixed<7, 1, Hot>) -> UFixed<7, 1, Hot> { a }
    |                     ^^^^^^^^^^^^^^^^^ unsatisfied trait bound
```

That is an unimplemented bound, so `#[diagnostic::on_unimplemented]` reaches it, which is the mechanism op
adopted at `130b:70-80`.

**Part four, it erases.** The object file:

```
_arvo16:    add w0, w1, w0 ; ret          _native16   = _arvo16
_arvo64:    add x0, x1, x0 ; ret          _native64   = _arvo64
_arvo_vec:  16-insn loop, add.8h, x8      _native_vec = _arvo_vec
```

Three bodies, three aliases. `native16`, `native64` and `native_vec` have no bodies because LLVM found
nothing to distinguish them from. This is op's own verified result from `135b:42-46`, reproduced over a
**derived** ladder rather than a hand-keyed `Rung<B>`, which is what `135b:65-68` asks for.

And at the wide rung, the same crate, the same operation, one derived numeral against the hand-written bar:

```
_arvo_wide200:                       _bar_wide256:
  ldp x9, x10, [x0]                    ldp x9, x10, [x0]
  ldp x11, x12, [x0, #16]              ldp x11, x12, [x1]
  ldp x13, x14, [x1]                   adds x9, x11, x9
  ldp x15, x16, [x1, #16]              adcs x10, x12, x10
  adds x9, x13, x9                     ldp x11, x12, [x0, #16]
  adcs x10, x14, x10                   ldp x13, x14, [x1, #16]
  adcs x11, x15, x11                   adcs x11, x13, x11
  adc  x12, x16, x12                   adc x12, x14, x12
  stp x9, x10, [x8]                    stp x9, x10, [x8]
  stp x11, x12, [x8, #16]              stp x11, x12, [x8, #16]
  ret                                  ret
```

Eleven against eleven, same mix, different schedule.

### 5.1 The consumer extension, and the law, both hold on the same crate

`134c` showed that a marker-carrying bridge is populated downstream with no `E0117`, and marked itself as
one probe by the dispatcher with no second read. **I am that second read and it reproduces**, here on the
full construction rather than a reduced one. The same crate carries a second marker and two widths arvo does
not list, and the derived numeral works through the same generic operation:

```rust
pub struct Mine;
impl ToNat<Mine> for Idx<777> { type N = T777; }
impl ToNat<Mine> for Idx<41>  { type N = T41; }

pub fn consumer_818(a: Fixed<777, 41, Hot, Mine>, b: Fixed<777, 41, Hot, Mine>)
    -> Fixed<777, 41, Hot, Mine> { a.add(b) }
const _: () = assert!(size_of::<Fixed<777, 41, Hot, Mine>>() == 104);   // 818 bits, 13 words
```

818 bits, thirteen words, 104 bytes, and a 48-instruction add, which is thirteen `adcs` plus the loads and
stores. No cap anywhere.

The width-generic law also stays on the const coordinates, which nobody had tried and which matters for
section 5.2. The output coordinates are const parameters pinned to the structural sum by an
associated-type equality bound, so no const arithmetic appears (`p7_law_site.rs`, exit 0):

```rust
pub fn mul<const I: u32, const F: u32, const J: u32, const K: u32,
           const OI: u32, const OF: u32, S, M>(..) -> Fixed<OI, OF, S, M>
where <Idx<I> as ToNat<M>>::N: Add<<Idx<J> as ToNat<M>>::N, O = <Idx<OI> as ToNat<M>>::N>,
      <Idx<F> as ToNat<M>>::N: Add<<Idx<K> as ToNat<M>>::N, O = <Idx<OF> as ToNat<M>>::N>, /* .. */

pub fn law_site(a: UFixed<13, 3, Hot>, b: UFixed<13, 3, Hot>) -> Fixed<26, 6, Hot> { mul(a, b) }
```

rustc infers `OI = 26` and `OF = 6` from the relation. That is `133:356-362`'s width-generic law with the
surface's own coordinates instead of structural ones.

### 5.2 `133`'s cost three is smaller than reported, and `134`'s repair applies to less than it thought

`133:436-450` calls the digit-tower diagnostic the serious cost, and `134` spent a whole file on a base-ten
repair for it. Measured on this construction, the tower appears at one site class out of two, because the
**surface keeps the const coordinates** and only the derivation is structural.

An ordinary coordinate mismatch is numeric:

```
error[E0308]: mismatched types
151 | pub fn wrong(a: UFixed<13, 3, Hot>) -> UFixed<16, 3, Hot> { a }
    |                                        ------------------   ^ expected `16`, found `13`
    = note: expected struct `Fixed<16, _, _>`
               found struct `Fixed<13, _, _>`
```

A law-relation mismatch is half towered, and it does name the written coordinate and point at the bridge row:

```
error[E0271]: type mismatch resolving `<Idx<16> as ToNat<Arvo>>::N == D0<D1<D0<D1<D1<Term>>>>>`
note: expected this to be `D0<D1<D0<D1<D1<Term>>>>>`
 23 |     0 => T0, 3 => T3, 8 => T8, 13 => T13, 16 => T16, 24 => T24,
    |                                                 ^^^
```

Against `131:476-481`'s GCA form, `type mismatch resolving 17 == 16`, this route is better at the first
site class and worse at the second. `134`'s base-ten encoding would fix the second and it is worth having
for that; it is not the whole-file cost `133` priced.

---

## 6. The answer to the gate, and what remains for op

**Is the erasure unconditional?** Yes, once the comparison target is the right one, and the panel's target
was wrong above 128 bits. The claim that holds, quantified precisely:

> **For every numeral the design admits, the operation lowers to what a competent author writes by hand at
> that width.** At and below 128 bits the hand-written form is the native primitive, and the lowering is
> byte-identical to it, demonstrated by LLVM emitting one symbol for both. Above 128 bits the hand-written
> form is a multi-limb carry chain over machine words, and the lowering is within 0 to 2 instructions of it
> with the same instruction mix.

There is no condition on the operation author. The body is written once, generically, and the ladder
supplies the payload. There is no condition on the width: the ladder is total and has no ceiling, by
structural catch-all rather than by a largest row. There is no gate and no flag.

**Is the ladder total?** Yes, and this is the part the panel did not have. `p5_total_ladder.rs` is a
complete gate-free derivation from a width to a container, native rungs and wide rung alike, with no width
enumerated anywhere and no `generic_const_args`. `133`'s cost two, the one hole it left, is closed by
section 4: the wide payload does not need an array length because a `#[repr(C)]` word cons has the size by
construction and the byte count is still readable in value position.

**The wide rung's price** is nothing, given the right limb, and 15 to 44 times, given byte limbs. The design
should not ship a body over byte limbs, and the current carrier
(`arvo-strategy/src/widebits.rs:103-110`, `pub bytes: [u8; BYTES]`) invites one.

### What is left, and it is a choice rather than a caveat

**One. The bridge, and whether extending it is acceptable to ask of a consumer.** The only enumeration
anywhere in this construction is `impl ToNat<Arvo> for Idx<13>`, one per written coordinate. It is not a
cap, since `134c`'s marker makes it consumer-extensible and section 5.1 reproduces that on the full
construction. It is not a ceiling, since a consumer's widths are unbounded. It is a **population question**:
arvo ships some set, and a consumer wanting a width outside it writes one line. Whether that is a
reasonable thing to ask, what the shipped set should be, and whether the refusal diagnostic should say
"add `impl ToNat<YourMarker> for Idx<7>`" in those words, are op's. My read, offered as a read: the refusal
is an unimplemented bound, so `#[diagnostic::on_unimplemented]` reaches it, and a one-line fix behind a
clear message is a better position than any cap.

**Two. The wide payload's shape, ragged against word-rounded.** Byte-exact costs 14 instructions at 200
bits and 25 bytes. Word-rounded costs 11 and 32 bytes. Seven bytes of padding at the worst case, shrinking
as a fraction as widths grow, against three instructions per operation. The footprint side is exactly what
`arvo-toolbox-not-policer.md` says Cold exists to protect, and the instruction side is exactly what Hot
exists to buy, so the honest answer may be that this is a **strategy axis** rather than one choice: Cold
ragged, Hot word-rounded. I have not built that and I am not proposing it as settled, only noting that the
trade falls on an axis the design already has.

**Three. The wide rung starts earlier than anyone has said out loud.** `131:277-280` gives Warm and Precise
one rung of headroom and therefore no rung 4, so they cross into the wide rung at **65 logical bits**, not
129. Since `Warm` is the design's default, the common consumer path reaches the multi-limb rung at 65 bits.
Everything above still holds there, since the multi-limb add is at the hand-written bar. But it means the
wide rung is not an exotic corner and its body deserves the same attention as the native rungs, which is
the opposite of how the panel has been treating it.

### Where I am the first read

Sections 1, 2, 3, 4 and 5 are all first reads and none of it should enter the canon on one expert's word.
The premise a second read should attack in my file: **that "what a competent author writes by hand" is the
right bar above 128 bits.** I chose it because comparing to a native instruction that does not exist is
comparing to nothing, and I wrote the hand-written forms myself, which means I chose the bar and then met
it. A second read should write its own bar, ideally by comparing against a shipped bignum library's inner
loop rather than against mine, and should check whether a SIMD formulation beats the scalar carry chain at
512 bits and above, which I did not test.

Section 2.1's correction to `133:161-180` is also a first read and it is a narrowing of a published result,
so it deserves the same treatment.

## 7. What I did not check

- **Multiply, divide and shift at the wide rung.** I measured addition only. Multiplication at the wide
  rung is quadratic in limbs and its limb choice matters more than addition's, not less. Nobody has priced
  it and the thirty-three times figure should not be assumed to carry over in either direction.
- **x86-64.** Every number here is aarch64. The alignment result in particular is architecture-dependent:
  aarch64's `ldur` makes unaligned loads free, and I would not assume that on every target the stack ships
  to. The instruction-count ratios should hold; the absolute counts will not.
- **Whether the total ladder's compile time holds at scale.** 0.04 s on this crate, which carries eleven
  bridge rows and a handful of sites. `133:400-408` measured the structural encoding at 0.11 s on
  sixty-four compositions; I did not repeat that on the ladder with the wide arm attached.
- **Whether `Cold`'s bitpacked access path survives.** `132:656-657` reduced this to a source reading and it
  is still not a compiled result. My word-rounding proposal in section 6 makes it more urgent rather than
  less.
- **The vectorised wide rung.** `arvo_vec` at the native rung autovectorises to `add.8h`. I did not test
  whether an array of wide numerals vectorises at all, and the carry chain suggests it does not, which
  would be a real finding about `Hot`'s `A16` alignment buying nothing.
- **The real law count**, still not in `110`, flagged at `131:663-666`, `132:658` and `133:665`, still open.
