# 16. Second read on how many outputs the derivation has

**Date:** 2026-08-08. **Register:** breadth pass. Nothing here settles, including where I agree with
`15`. **Dispatch:** derive independently first, then read `15` and say where I land differently.

## The order I actually worked in, and one contamination I have to declare first

The dispatch inverts the panel's default reading order for one question. I followed it, and the
sequence is checkable from this file's own git history and from the mtimes under `16_probes/`:

1. `RULES.md`, `01_op_answers.md`, `04_op_no_settlements_tonight.md`, `SETTLED.md`.
2. The acceptance criterion in full, and then its establishing source, because `SETTLED.md` warns
   about its own compressions and this criterion is one of them (see the next section).
3. Derived my own answer, built the probes, and wrote sections 1 through 9 of this file.
4. Only then opened `15`, `13` and `12`.

**The contamination.** My first orientation command was `git log --oneline -8` in the arvo repo, run
before I had read anything. One of the eight subjects it printed is:

```
c85cfe2 research: build the three-input map and find it needs two outputs
```

So **the number two was in front of me before I derived anything.** I am not going to pretend
otherwise, and the consequence is specific rather than total:

- My agreement with `15` **on the count** is worth close to nothing. It is an anchored guess, not an
  independent derivation, and if the dispatcher records a provenance rung for the count, it should
  record ONE EXPERT and not two.
- My derivation of **which two, what the second is keyed on, what fails without it, and which check
  is blind** was done before opening `15`, and the commit subject says nothing about any of those.
  Those are the parts where a second read has content, and they are where I would want the rung
  read.

A second contamination, smaller. Listing `15_probes/` to find the panel's probe conventions printed
48 filenames including `q07_three_input_map` and `q13_cold_packed`. Filenames are a weak leak but
they are a leak: they told me `15` had touched packed storage. I did not open any of them until step
4.

Both are worth recording as a fact about the dispatch shape rather than about `15` or me. `RULES.md`
already says the default panel shape makes the TWO EXPERTS rung unreachable by construction. It is
worth adding that **an orientation command can leak a predecessor's conclusion through a commit
subject**, and that a dispatch which wants a genuinely independent second read should say "do not run
`git log` in the panel repo before you have written your answer down". That instruction costs one
line and would have saved this one.

## The criterion, in op's own words, and the noun the compressions drop

`SETTLED.md:63-74` carries it, with op's words quoted at `67-71`. The establishing source is
`seed/SETTLED_container.md:34-37`, which quotes the same passage:

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the
> matching container and numeral representations, then validate, and erase on lowering to be exactly
> what you describe before that caveat.
>
> Anything less than that, no caveats left, is unacceptable for this design and canon.

Now the chain of restatements. I got this wrong on a first pass and the corrected version is more
interesting than the version I had, so it is worth stating carefully: **the quotes are intact
everywhere and it is the glosses that drift.**

| Where | Kind | How it renders the derived thing |
|---|---|---|
| `SETTLED.md:68` | quote | "the matching container **and numeral representations**" |
| `seed/SETTLED_container.md:35` | quote | "the matching container **and numeral representations**" |
| `seed/SETTLED_container.md:29` | the sweep's own claim | "the matching container and numeral **representation**" |
| `SETTLED.md:73-74` | the four-part gloss | "the typestate derives the container **and representation**" |

Both files preserve op's plural verbatim. Both then paraphrase it, and both paraphrases lose the
plural; the second loses "numeral" as well, leaving "container and representation", which reads as a
hendiadys, two words for one thing, and is exactly how a reader who already believes the answer is
one output will parse it.

That the quote survives and the gloss does not is the worse failure mode, not the milder one. The
gloss is what a hurried reader takes, because it is the sentence that says "four parts" and is
therefore the one that looks like the operational form of the criterion. A reader who reads only the
numbered gloss gets a four-part criterion whose second part has one noun in it.

I want to be careful about how much weight one plural can carry, because it can also be read as a
plain distributive plural over the many numerals a design admits, rather than as a claim that one
numeral has several representations. Both readings are available from the sentence alone. What I can
say without overreading it: **op's sentence names two things joined by "and", the compressions
collapse them to one, and my derivation below reaches two by a route that never used the sentence.**
That is the useful shape. The sentence is corroboration for a result, not the source of it.

## 1. What I take the question to be

A declared numeral is the consumer's entire input: widths, sign, strategy. Something like
`UFixed<13, 3, Warm>`, with logical width `W = I + F` unsigned or `1 + I + F` signed. The consumer
never writes the container; that is ratified at `SETTLED.md:93` from `130b:37`.

The derivation runs at compile time, must erase, and must produce enough that the machine can hold
the value and operate on it.

"How many outputs" needs a criterion for what counts as an output, or the answer is unfalsifiable:
everything downstream is a function of the declaration, so you can always claim one output and call
the rest recomputation. I use this criterion throughout, and I state it up front so it can be
attacked:

**A component is an output of the derivation when the consumer did not write it, the machine needs
it, and a downstream site that holds the other components cannot recover it.**

Three clauses. The first excludes the declared data: `I`, `F`, sign and strategy are inputs, so the
fixed-point encoding (raw = value times two to the F, two's complement for signed) is a restatement
of the input rather than a derived output. The second excludes decoration. The third is the load
bearing one and it is what the rest of this file is about.

## 2. The derivation for four strategies and both signs, done by hand first

Before any probe, the arithmetic. I take the four strategies at their documented intent from
`arvo-toolbox-not-policer.md` and the crate instructions: `Hot` picks the fastest native container,
`Warm` is what Rust does, `Cold` is bitpacked contiguous storage, `Precise` keeps the most precision.
I am deliberately not reading the shipped ladder, since `mock/crates` is being nuked and is not
evidence about what is correct.

For the three unpacked strategies the derivation is the familiar one. `W = 13` unsigned goes to a
sixteen bit container. Thirteen bits are used, three are not. An array of a thousand of them is two
thousand bytes, because a Rust array's stride is the element's size and nothing else.

For `Cold` at `W = 13` the intent is that a thousand values occupy thirteen thousand bits, which is
1625 bytes. Element `k` begins at bit `13k`. There is no Rust type whose size is thirteen bits, so
whatever the derivation emits for `Cold`, **it cannot be a type whose `size_of` is the answer.**

That is the whole finding in one line, and everything below is me trying to break it.

### The collapse, stated as a failure of injectivity

Take the map from a declared numeral to a container type. Restrict it to unsigned, `Cold`, widths
nine through sixteen. Every one of those maps to the same sixteen bit container, because that is the
smallest native container that holds them. Their strides are nine, ten, eleven, twelve, thirteen,
fourteen, fifteen and sixteen bits respectively, and those are eight different memory layouts.

So the map is not injective, and the information it drops is exactly the information `Cold` exists to
carry. A derivation whose codomain is the container type alone has, at the moment it returns,
destroyed the distinction between `UFixed<13,0,Cold>` and `UFixed<16,0,Cold>`. No downstream site can
recover it, because the only thing it was handed is a type that both share.

This is not an exotic corner. Under `arvo-toolbox-not-policer.md` the packed contiguous case is the
workload arvo exists for, and the widths that are not powers of two are the entire reason a bitpacked
strategy is on the menu. A derivation that is correct exactly on the power-of-two widths is correct
exactly where the strategy buys nothing.

## 3. So the second output, named, and what it is keyed on

I will call the two outputs the **carrier** and the **extent**, and I am naming them for this file
rather than proposing vocabulary. Any two names would do; what matters is that there are two slots.

**The carrier** is the machine type an operation lowers to. It is what a register holds, what a
function argument is passed as, what an add instruction operates on. This is the output everyone
already agrees exists.

**The extent** is what one value occupies in memory: its bit width, and the stride at which
consecutive values repeat. For the three unpacked strategies the extent is `W` bits inside a slot of
`8 * size_of(carrier)` bits, so the stride exceeds the width and the difference is padding. For
`Cold` the extent is `W` bits inside a slot of `W` bits, so the stride equals the width and there is
no padding at all.

Two properties of this shape are worth stating because they are what make it a good answer rather
than a bookkeeping device.

**The arity does not change with the strategy.** A derivation that returned one output for `Hot` and
two for `Cold` would be a case split wearing a signature, and every downstream site would carry the
split. Under the pair, `Hot` returns `(u16, extent 13 in stride 16)` and `Cold` returns
`(u16, extent 13 in stride 13)`. Same shape, different value. The strategies differ in what they
compute, not in what they return.

**The second output is keyed on the declaration, not on the carrier.** This is the part that decides
whether it is genuinely a second output or a derived convenience. Stride is a function of `(W,
strategy)`. Carrier is a function of `(W, strategy, sign)`. Both read the same input; neither reads
the other. And critically, `(W, strategy) -> stride` cannot be factored through the carrier, by the
non-injectivity in section 2.

## 4. Is a third output needed? I looked for one and it is recoverable

This is where I expected to land on three, because access into a packed array is not the same size as
either the carrier or the stride, and it is the number that actually decides the load instruction.

Take `W = 13`, packed, element `k`. It occupies bits `[13k, 13k + 13)`. Its phase within a byte is
`13k mod 8`, and since thirteen and eight are coprime the phase cycles through all eight residues.
At phase seven the field runs from bit seven to bit nineteen, which touches three bytes. A sixteen
bit load cannot cover it. So the load a compiler must emit for the general element of a packed
thirteen bit array is thirty-two bits wide, while the carrier is sixteen and the stride is thirteen.
Three different numbers, all needed.

That looked like a third output. It is not, and the reason is precise: the maximum byte span of a
`W` bit field at unknown phase is `floor((W + 6) / 8) + 1`, a function of `W` alone, and `W` is
already recoverable from the extent. So a site holding `(carrier, extent)` can compute the access
width without re-entering the derivation. A site holding only the carrier cannot, because `W` is
exactly what the carrier lost.

I checked this rather than asserting it, in `16_probes/p4_access_width.rs`. The interesting row is
that the access width and the carrier disagree for most widths, so anyone who tried to use the
carrier as the access type would generate a load that truncates the field.

So: **two, and the third candidate is a projection of the pair.** Both directions matter. Section 2
says one is too few. This section says three is one too many, at least for every layout quantity I
was able to name.

## 5. The check that is blind, which is the part worth the most

The dispatch asks which obvious correctness check a too-narrow derivation still passes. There is one,
it is the panel's own, and it is blind for a structural reason rather than by oversight.

**The blind check is the erasure and codegen-equality check itself.**
`seed/SETTLED_container.md:45-62` records it as what closed the gate: an operation on a derived
numeral lowers byte-identically to the same operation on the native primitive, with LLVM folding the
two into one symbol and the native function left with no body. `137b:10-26` is op recording the gate
as met on the strength of it.

That check is correct and I am not attacking it. I am pointing at what it quantifies over.

**It is per-value by construction.** It takes one numeral, applies one operation, and compares the
emitted instructions against one native instruction. There is no array anywhere in it. There cannot
be, because its entire method is comparing against a native primitive, and a native primitive has no
packed-array form to compare against. The check's instrument is a scalar, so its resolution is a
scalar.

Which means a derivation that emits only the carrier passes it **at full marks, for every strategy,
including `Cold`.** For `UFixed<13,0,Cold>` the carrier is sixteen bits, the operation lowers to the
native sixteen bit instruction, the symbols fold, the check is green, and the storage is silently
23 percent larger than the strategy promised.

Three further checks a reasonable person writes, all of which are also blind:

- **Round trip.** `raw(from_raw(r)) == r` for all representable `r`. Green. It never leaves the
  carrier.
- **Arithmetic agreement.** Derived operations agree with a reference implementation across the value
  range. Green. Same reason.
- **`size_of` is what the ladder says.** `size_of::<UFixed<13,0,Cold>>() == 2`. Green, and worse than
  useless, because it is a tautology: it asks the derivation to agree with itself. This one is the
  fabricated-diligence shape. It looks like a layout test and it is a restatement of the thing under
  test.

The check that catches it has to name a quantity the carrier does not have, and there are only two
shapes of it I can find:

- **`bits_of::<[N]>() == N * W`**, an array-extent assertion. Nobody writes this unless they already
  believe the second output exists, which is the trap.
- **A packed round trip at a non-zero phase.** Write value `v` at index `k` for `k` covering a full
  phase cycle, read it back, and require equality. This fails immediately against a carrier-only
  derivation, because there is no phase in a carrier-only world and index `k` lands at byte `2k`.

I built the green-and-blind suite and its two catching counterparts in
`16_probes/p3_blind_suite.rs`, because a claim that a suite is blind should be a suite you can run.

**Why this matters more than an ordinary coverage gap.** The blind check is the one the acceptance
criterion's fourth clause is measured by, and the criterion is the one statement everything answers
to. If the instrument that certified erasure is structurally incapable of observing the second
output, then the gate being met is evidence about the carrier and about nothing else. That is not a
reason to doubt `137`. It is a reason to say the gate has been met **on one of its two nouns**, which
is what the plural in op's sentence was carrying and what every compression since has dropped.

## 6. Why both must be emitted even where one is computable from the other

There is a version of the objection that survives everything above, so I want to meet it head on:
maybe the carrier is recoverable from the extent, so emit the extent and derive the carrier at each
use.

For most rungs the carrier is `next_pow2(max(W, 8))` clamped to the native ladder, which is a
function of `W`, which the extent carries. So the objection has teeth in the arithmetic.

It fails for two independent reasons.

**First, `Precise` breaks the recovery if it widens compute past storage.** If a `Precise` numeral
stores thirteen bits and computes in a wider container to keep intermediates exact, then two
declarations with the same extent have different carriers, and the map from extent to carrier is not
a function. I do not know the shipped `Precise` semantics and I am not going to reason from
`mock/crates`, so I state this conditionally and mark it as a question rather than a result. It is a
cheap question to settle and it is worth settling, because if `Precise` does widen, the pair is
irreducible in both directions and the argument is closed.

**Second, and this one does not depend on `Precise`: recoverable by arithmetic is not the same as
available at the type level.** This design's derivation is typestate. A downstream site that wants to
write an operation needs the carrier **as a type**. A site that wants to compute an address needs the
extent **as constants**. Recovering a type from a constant is precisely the const-to-type problem
that `seed/SETTLED_container.md:74-89` records as the expensive part of the whole thread, the one
that is irreducible under const keying without a forbidden feature and only became gate-free by
keying the magnitude structurally.

So emitting the extent and recomputing the carrier at each use would re-enter, at every use site, the
problem the derivation exists to solve once. That is a strong reason to name both that has nothing to
do with whether the arithmetic happens to work out.

## 7. What goes wrong, in numbers, each with the command that produced it

Everything in this section is a count or a compile-time size. **No bench harness has run in this
panel**, so nothing here is a timing, nothing is a throughput, and every magnitude question about
what packed access *costs* is unpriced. I do not claim packed storage is faster. I claim it is
smaller by a stated amount, and what that costs in cycles is unmeasured.

Rerun the lot with `./16_probes/verify.sh`, which rebuilds from source on the pinned toolchain.

### How much the carrier alone loses

`16_probes/p1_fibre_count.rs`, output in `p1_fibre_count.out`:

```
rustc +nightly-2026-05-28 --edition 2021 -O p1_fibre_count.rs -o bin/p1 && ./bin/p1
```

| Quantity, over widths 1 to 128, two signs, four strategies | Value |
|---|---|
| distinct declarations | 1024 |
| distinct carriers | 10 |
| distinct (carrier, stride) pairs | 256 |
| declarations behind one carrier, average | 102.4 |

Ten values in the codomain against 1024 in the domain. The pair separates 256. The fibres are not
evenly spread and the large ones are the expensive ones: the 128-bit carrier stands behind 64
distinct unsigned `Cold` widths, strides 65 through 128, which the carrier alone renders identical.

### What the collapse costs, as bytes

Same probe. A contiguous run of one million values, unsigned `Cold`:

| W | carrier-only | two-output | carrier-only overhead |
|---|---|---|---|
| 3 | 1 000 000 | 375 000 | 166.7% |
| 5 | 1 000 000 | 625 000 | 60.0% |
| 9 | 2 000 000 | 1 125 000 | 77.8% |
| 13 | 2 000 000 | 1 625 000 | 23.1% |
| 17 | 4 000 000 | 2 125 000 | 88.2% |
| 23 | 4 000 000 | 2 875 000 | 39.1% |
| 47 | 8 000 000 | 5 875 000 | 36.2% |

The worst ratio in 1 to 128 is at `W = 1`, where carrier-only storage is 8.00 times the packed
size. That is a boolean column, which is not a contrived case.

Two things about this table that I want to be honest about. The overheads are not monotone in `W`,
because they track the distance to the next native rung, so a table sampled at 31 and 63 looks
nearly free (3.2% and 1.6%) and a table sampled at 17 and 9 looks catastrophic. **Anyone
demonstrating either position can pick a sample that proves it.** That is a reason to state the
whole range rather than a headline. And these are storage figures. Whether the packed form is
faster to walk is a bandwidth-against-shift-and-mask question that no measurement in this panel
touches.

### That an array cannot express the packed form

`16_probes/p2_stride_is_not_size_of.rs`. Every layout claim in it is a `const` assertion, so the
compiler checked them and the binary only prints what it already proved:

```
rustc +nightly-2026-05-28 --edition 2021 -O p2_stride_is_not_size_of.rs -o bin/p2 && ./bin/p2
```

The three declarations `UFixed<13,0,Cold>`, `UFixed<16,0,Cold>` and `UFixed<13,0,Warm>` all have
`size_of` 2 and the same alignment. `size_of::<[Num13Cold; 1000]>()` is 2000 and the strategy
promised 1625. The 16-bit rows agree between the two derivations and the 13-bit rows do not, which
is the mechanical reason a test matrix built on power-of-two widths reports green.

### The suite that is green and blind

`16_probes/p3_blind_suite.rs`, the central result:

```
rustc +nightly-2026-05-28 --edition 2021 -O p3_blind_suite.rs -o bin/p3 && ./bin/p3
```

Against a carrier-only derivation of `UFixed<13,0,Cold>`, four of four green:

| Check | Result | Why it cannot see the problem |
|---|---|---|
| round trip over all 8192 representable raws | PASS | never leaves the carrier |
| arithmetic agreement over 163 840 pairs | PASS | never leaves the carrier |
| erasure and codegen equality, per value | PASS | its instrument is a scalar |
| `size_of` matches the ladder | PASS | asks the derivation to agree with itself |

Then the two catching checks:

- **array extent equals `N * W`**: FAIL, `[Num; 1000]` occupies 16 000 bits where the declaration
  promised 13 000, 2000 bytes against 1625, 23.1% over.
- **packed round trip with the access width taken from the carrier**: FAIL, 24 of 64 values wrong,
  failing bit-phases 4, 5, 6 and 7, first at index 3 which wrote 1852 and read 316.
- **packed round trip with the access width taken from the extent**: PASS, all 64.

### The access width, and why it is not a third output

`16_probes/p4_access_width.rs`:

```
rustc +nightly-2026-05-28 --edition 2021 -O p4_access_width.rs -o bin/p4 && ./bin/p4
```

Across widths 1 to 64 the carrier and the access width **disagree for 28 of 64 widths**. So a site
that reaches for the carrier as its load type is wrong on nearly half the range, which is what the
24-of-64 failure above is.

Recoverability, checked by asking whether every width behind a carrier shares an access width:

| From | Widths behind it | Access widths needed | Verdict |
|---|---|---|---|
| carrier u8 | 8 | 8, 16 | not recoverable |
| carrier u16 | 8 | 16, 32 | not recoverable |
| carrier u32 | 16 | 32, 64 | not recoverable |
| carrier u64 | 32 | 64, 128 | not recoverable |
| the extent | carries W directly | one per W | recoverable |

The closed form I used in section 4, `floor((W + 6) / 8) + 1`, was checked against an exhaustive
scan over all eight bit-phases for every width 1 to 1024: **0 mismatches**.

### A second-order blindness I did not expect

The same probe, part (b), and this is the finding I would not have predicted. The truncation
failure is **data-dependent**: a too-narrow load returns the right answer whenever the bits it
truncated happened to be zero.

| Test data, 64 elements at 13 bits, 2-byte access | Wrong |
|---|---|
| values 0 to 63, which is what a hand-written test uses | 0 of 64 |
| values with the top bit set | 32 of 64 |
| all bits set | 32 of 64 |

So **the catching check is itself blind if its test data is small**, and small test data is the
default a person reaches for. The check only bites when the data fills the declared width. Anyone
adding the packed round trip to a suite has to also decide to fill the width, and that is a second
decision that is easy not to make.

This is worth more than the first-order finding, because it explains how a design could adopt the
right check and still not learn anything. It also generalises past this question: any check on a
packed representation needs data that exercises the high bits, and a test written with counters
and small literals exercises none of them.

### Whether the pair is irreducible in both directions

`16_probes/p5_recovery_direction.rs` models both readings of `Precise`, because I do not know which
the design takes and will not read the shipped source:

| Reading | Extents mapping to more than one carrier | Consequence |
|---|---|---|
| `Precise` does not widen compute past storage | 0 of 251 | extent to carrier is a function, so this direction of the objection survives the arithmetic |
| `Precise` widens compute past storage | 64 of 251 | not a function; the pair is irreducible in both directions |

Under the first reading the objection is answered only by the type-level argument, which is the
next probe.

### That "recoverable by arithmetic" is not "available at the type level"

`16_probes/p5b_const_to_type.rs` **does not compile, on purpose**, and its diagnostic is committed
beside it in `p5b_const_to_type.err`:

```
rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p5b_const_to_type.rs
```

Four refusals, in three syntactic positions (return type, type alias, where-clause), all with the
same text:

```
error: generic parameters may not be used in const operations
   = help: const parameters may only be used as standalone arguments here, i.e. `W`
   = help: add `#![feature(generic_const_exprs)]` to allow generic const expressions
```

`generic_const_exprs` is forbidden. So a use site handed the extent as a const cannot write the
arithmetic that recovers the carrier as a type, even though the arithmetic is right there and
const-evaluable.

Note on the count: `verify.sh` reports 5 by grepping `^error`, which includes the "aborting due to
4 previous errors" line. The refusals are four.

### Attacking that refusal instead of reporting it

The workspace rule for a refused bound is that it wants a trait rather than a feature, so I built
the trait form. `16_probes/p6_trait_form_recovers_both.rs`, **with no `#![feature]` gate anywhere
in the probe directory**, which `verify.sh` checks by grep and reports as none:

```
declaration          carrier  extent  stride  access  bytes/1e6   phase of elem 3
UFixed<5,0,Warm>     u8      5       8       2       1000000     0
UFixed<5,0,Cold>     u8      5       5       2       625000      7
UFixed<8,0,Cold>     u8      8       8       2       1000000     0
UFixed<13,0,Warm>    u16     13      16      3       2000000     0
UFixed<13,0,Cold>    u16     13      13      3       1625000     7
UFixed<16,0,Cold>    u16     16      16      3       2000000     0
UFixed<31,0,Cold>    u32     31      31      5       3875000     5
UFixed<47,0,Cold>    u64     47      47      7       5875000     5
```

One trait, one blanket impl per strategy, an associated type and two associated consts. Same arity
in every strategy. The four right-hand columns are computed from the pair by plain const fns, which
is the constructive form of section 4's claim that no third component is needed.

**Scaffolding flag, because this probe would be easy to misread.** Its width-to-carrier ladder is
one impl per width, which the design has ratified against (`SETTLED.md:97`, `:110`, "no
enumerations", refused four times). That is not a proposal. The non-enumerating ladder is the closed
panel's `137` result and I did not re-derive it. What this probe checks is what sits on top of a
ladder: whether one trait can emit two components at once, gate-free. The blanket impls are the part
to read; the `widths!` macro is scaffolding to reach them.

**And one result came out of the failed first attempt.** I first tried to select the native rung
with a `where M: InRange<LO, HI>` bound, and rustc refused it:

```
error[E0119]: conflicting implementations of trait `Derive<Warm>` for type `(_, Warm)`
```

A where-clause does not disambiguate impls, so two rungs overlap at the head constructor. The rung
has to be carried **in the type**, not in a bound. That is the same conclusion as the closed panel's
structural-keying result, reached here from the opposite direction, by trying the bound form and
being told no. Recorded in `16_probes/p6_first_attempt_note.md`.

## 8. Where I land, before reading anything else

**Two outputs.** The carrier, which is the machine type an operation lowers to. The extent, which is
what one value occupies and the stride at which consecutive values repeat.

**The second is keyed on the declaration**, specifically on `(W, strategy)`, and it cannot be
factored through the carrier because the carrier is not injective: 1024 declarations, 10 carriers,
102.4 declarations behind each on average.

**What goes wrong without it** is that `Cold` stops meaning anything at every width that is not a
native rung, which is every width the strategy exists for. The cost runs from 1.6% to 700% of extra
storage across the range, and is 23.1% at the thirteen-bit case I worked. Separately, a site handed
only the carrier picks the carrier as its access type and generates a load too narrow to cover the
field, wrong on 28 of 64 widths.

**The blind check is the erasure and codegen-equality check**, the one the acceptance criterion's
fourth clause is measured by. It is blind for a structural reason: its method is comparing one
operation against one native instruction, so its instrument is a scalar and it has no array in it
and cannot have one. A carrier-only derivation passes it at full marks for every strategy.

**And the check that catches it has a second blindness underneath**, which is the part I would put
in front of op first: the packed round trip only fails on data that fills the declared width, so a
test written with small values is green against a broken access. Adopting the right check is not
enough; the data has to be chosen too.

None of this settles anything. It is one derivation, with its probes committed, offered for the
morning.

## 9. Coverage before the read, and what I did not do

Stated before I read `15`, so it is a description of my own work rather than of the difference.

I did not read any panel file except `RULES.md`, `01`, `04`, `SETTLED.md`,
`seed/SETTLED_container.md` up to line 120, before writing sections 1 through 7. I did not read
`CANON_CANDIDATE.md`, `MORNING.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, or any expert file. I did not
read `mock/crates` and I did not read the shipped ladder, on the standing instruction that it is
being nuked and is not evidence.

I did not derive the wide rung above 128 bits in any detail. The extent argument should carry there
unchanged, since a wide payload has a size and an alignment like anything else, but the access-width
arithmetic in section 4 assumes a byte-addressed field inside a machine word and I did not check it
against a multi-limb payload.

I did not consider the signed packed case at all, and it is the largest hole in this file. Sign
extension on read out of a packed field is a real cost and a real correctness hazard: an unsigned
packed read is a shift and a mask, and a signed one needs the sign bit replicated from an arbitrary
bit position, which is a different instruction sequence and may want a different access width. Every
width in every probe here is unsigned. Anyone taking this further should start there.

I have not touched the `Precise` question of section 6 beyond naming it.

## 10. Having now read `15`: where I land, corrected

`15` reaches two outputs and names them **container** and **stride**. I reached two and named them
carrier and extent. On the count we agree, and per the contamination declared at the
top of this file that agreement is worth little, because a commit subject put the number in front of
me first.

On the content we agree more closely than I expected, and on one point **`15` is tighter than I was
and I was over-counting.** Taking that first, because a second read that only adds is not doing its
job.

### 10.1 My own criterion refutes my third component

Section 1 states the criterion: a component is an output when the consumer did not write it, the
machine needs it, and a site holding the other components cannot recover it.

My `p6` emits three associated items: `type Carrier`, `const EXTENT_BITS`, `const STRIDE_BITS`.
`15`'s `q07_three_input_map.rs:130-131` emits two: `type Container; type Stride;`.

`EXTENT_BITS` is the declared total width. **The consumer wrote it.** It fails the first clause of my
own criterion, and it is present in the numeral type as an input regardless of what the derivation
returns. I checked that against `15` rather than assuming it: `15`'s numeral is
`Numeral<W, F, Tag>` with `W` as the first structural parameter (visible in the diagnostic quoted at
`15:446` and `15:508`, where `E<E<E<E<O<Z>>>>>` is 16 and `O<O<Z>>` is 3), so the width is carried by
the type and does not need re-emitting.

So the honest count of *derivation outputs* is two, and my extent was a pair of which one half is an
input travelling under a new name. `15` had this right and I did not. The corrected statement:

> The derivation has two outputs, the carrier and the stride. Every layout quantity a consumer needs
> is a function of those two together with the declared width, which the numeral already carries.

That change is not cosmetic. It matters for the canon, because a canon sentence that says the
derivation emits an extent invites an implementer to store the width twice, and two copies of one
fact drift.

### 10.2 Where my derivation confirms `15` on something specific rather than on the count

Three sub-claims, each reached by me before reading `15` and each matching:

**The carrier is the same for `Cold` and `Warm`.** `15:317-319` says "a lone `UFixed<13,3>` is a
`u16` whatever strategy you asked for". My `p6` encodes exactly that (`Derive<Cold>::Carrier =
W::Native`), and my section 2 reached it from the other end: a lone `Cold` value has to have a size,
so `Cold` cannot be a statement about the standalone type and must be a statement about composition.

**The stride for a non-packed strategy is the container's width, not the value's rounded-up bytes.**
`15:345-349` reports this as its first defect, having computed `8 * ceil(W / 8)` and found it wrong at
`W = 24`. I never wrote that form: my `p6` defines the non-packed stride as
`8 * size_of::<W::Native>()`, which is `15`'s repair, because I derived stride from the carrier's size
rather than from the value's bytes. That is an independent arrival at the same repair from a different
starting point, which is worth more than agreeing with the finished statement.

**Alignment is not a third gap.** `15:530-532` asserts it falls out and `15:553-556` then names its own
residual doubt: whether alignment is a fifth axis is "not decided by anything I built". I built
something. `16_probes/p7_alignment_is_not_a_third.rs` constructs the adversarial pair, two wide
payloads of **identical size 32 and identical stride 256 bits, at align 1 and align 16**, and the
const assertions confirm size equal and alignment unequal. So alignment is genuinely not recoverable
from the stride, which is the doubt `15` had. It is recoverable from the **carrier**, because
`align_of` is a property of a type, so it rides on output 1 and is not a third output.

That result has a consequence worth stating: **it is a reason the carrier must be a type rather than a
width.** A width cannot carry an alignment. Anyone tempted to collapse the carrier to a bit count
loses the align-1 against align-16 distinction that `15:537-539` measures at seven bytes per element
at `W = 200`.

The one case with no carrier holding the run is a packed column's base, and there `p7` shows every
byte offset is reachable at a 13-bit stride, so no base alignment makes any element aligned and align
1 is sufficient for correctness. What a higher base alignment buys is a load-crossing question and is
**unpriced**.

## 11. What I have that `15` does not, and it is the part I would put in front of op first

Three things. The first is the dispatch's actual question about blindness, which `15` approaches and
then passes.

### 11.1 The certifying check is blind, and `15` runs it as evidence

`15:400-416` runs `q12_erasure_asm.rs`, reads the assembly, and reports that the assembler equated
the symbols: `q12_native_u16` aliased to `q12_arvo_cold`. It offers this as evidence that erasure
survives all three inputs. It does survive, and the reading is correct.

**That same check would be equally green over a derivation that emits only the container**, and `15`
does not say so. Its instrument is one operation on one value compared against one native
instruction. There is no array in it and there cannot be, because a native primitive has no packed
form to compare against. `q12` is measuring output 1 and is structurally incapable of measuring
output 2, and it is the check the acceptance criterion's fourth clause is certified by.

My `p3_blind_suite.rs` is that claim made runnable: four of four green over a carrier-only derivation
of `UFixed<13,0,Cold>`, including the per-value erasure check and including a `size_of` check that is
a tautology, while the stored form is 23.1% larger than the strategy promised.

`15:375-379` gets closest, and its generalisation is a different one: "a derivation that produces a
container **and** a layout has an internal consistency condition, and the two can disagree silently."
That is about a two-output map's own bugs, and it is a good finding, and `15` found two of them that
way. Mine is about a one-output map passing the certification. They compose: **the second output is
both harder to get right and impossible to notice missing**, and the second half is not in `15`.

### 11.2 The catching check has a blindness underneath it, which is the finding I did not expect

`16_probes/p4_access_width.rs` part (b). A too-narrow load returns the correct value whenever the bits
it truncated were zero, so the packed round trip fails on 0 of 64 elements with test data of 0 to 63,
and on 32 of 64 with data that fills the width.

Small values are what a person reaches for when writing a test by hand. So adopting the right check is
not sufficient; the data has to be chosen to fill the declared width, and that is a second decision
nobody is prompted to make. This generalises past the present question to any check on a packed
representation.

I would put this in front of op before the count, because the count is a design statement and this is
the reason a design statement about it would not have been reached by testing.

### 11.3 The access width, and the trap of reaching for the carrier

Neither `15` nor anything else in the panel names the width of the load that reads one element out of
a packed run. It is a third quantity, distinct from both outputs: at `W = 13` the carrier is 16 bits,
the stride is 13 bits, and the load must be 32 bits, because a 13-bit field at bit-phase 7 spans
three bytes.

It is **not a third output**, because it is `floor((W + 6) / 8) + 1` bytes rounded up to a power of
two, a function of the declared width, which the numeral carries. I checked the closed form against an
exhaustive scan of all eight phases for every width 1 to 1024: zero mismatches.

What makes it worth naming anyway is that **the carrier is the wrong answer for it at 28 of 64
widths**, and the carrier is the nearest thing to hand. An implementer building `15`'s column from
`15`'s two outputs has the stride for the offset and then needs a load type, and the obvious reach is
the container. That produces the 24-of-64 truncation in `p3`. This is an implementation trap rather
than a hole in `15`'s design, and it is the sort of thing that belongs in a canon sentence as a
relation rather than being rediscovered.

## 12. Where I read the material differently from `15`

**On what `Cold` is.** `15:337` calls its packed column "the whole of `Cold`'s reason for
existing, expressed as one associated type". I would put it one notch stronger and it changes what the
canon has to say. `Cold` is not a container choice with an extra field attached. It is **a statement
about composition rather than about the value**, and that is why it has no standalone type. Under that
reading the two outputs are not two coordinates of one answer, they are answers to two different
questions: what does an operation lower to, and what does a run of these look like in memory. A canon
that says "the derivation produces a container and a stride" invites the first reading. One that says
the derivation answers a per-value question and a per-aggregate question carries the reason.

**On the `Precise` question.** `15:669-672` says it did not build `Precise` as anything but `Warm`
with a different name, and raises whether `Precise` widens intermediates rather than containers, in
which case "it is not a container-map input at all and belongs on a different axis". I modelled both
readings and there is a consequence `15` does not name.
`16_probes/p5_recovery_direction.rs`: if `Precise` does not widen, then over widths 1 to 128 there are
251 distinct extents and **0** map to more than one carrier, so the carrier is a function of the rest
and the pair's irreducibility rests only on the type-level argument. If `Precise` does widen, **64 of
251** extents map to two carriers, and the pair is irreducible in both directions as a matter of
arithmetic.

So `Precise`'s semantics is not only a gap in the strategy table. **It decides whether the two-output
shape is forced by arithmetic or only by the type system.** That is a reason to settle it earlier than
its apparent size suggests, and it is a sharper statement of the same gap `15` names.

**On what is op's.** `15:697-707` puts one question to him: does a consumer write the integer width or
the total width. I have no independent view on that; it is downstream of work I did not do, and I am
not adding a second question to compete with it.

What I would add is not a question but a caveat on the criterion. The acceptance criterion is
certified met at `137b:10-26` on the strength of a per-value check. If the derivation has two outputs,
then **the gate has been met on one of its two nouns**, and the second has never been through the
gate at all. Whether that reopens anything is his call and not mine. It is stated here so that the
morning's reader does not carry "the erasure gate is met" forward as though it covered both.

## 13. Attacks I made on my own answer that did not land, recorded so nobody repeats them

**Could the second output be folded into the carrier by making the carrier a packed type?** No. There
is no Rust type of size 13 bits, and `p2`'s const assertions confirm `size_of::<[T; N]>() ==
size_of::<T>() * N` with no exception. A packed run is not an array of anything.

**Could the derivation return only the stride and recompute the carrier?** Not as a use-site move.
`p5b_const_to_type.rs` does not compile, in three syntactic positions, all four refusals naming
`generic_const_exprs`, which is forbidden. And the trait form that answers it (`p6`) is the derivation
again, so recomputing at each site means re-entering the problem the derivation exists to solve once.

**Is the access width a third output?** No. Recoverable in closed form from the declared width, which
`p4` verifies against an exhaustive phase scan over 1024 widths, 0 mismatches.

**Is alignment a third output?** No. `p7` builds the adversarial same-size same-stride pair and finds
alignment rides on the carrier type.

**Could rung selection be done by a where-clause bound, which would have simplified `p6`?** No.
`E0119`, conflicting implementations, because where-clauses do not disambiguate impls. The rung has to
be in the type. Recorded in `p6_first_attempt_note.md`, and it is an independent arrival at the same
place as the closed panel's structural-keying result.

**Route I did not take.** The wide rung above 128 bits. `15:351-354` reports its second defect there,
that the stride belongs to the `(strategy, rung)` pair because `Hot` pads to align 16, and I did not
build a wide rung so I cannot confirm or contest it. My `p7` touches the alignment half of it and not
the stride half. Anyone picking this up should start from `15`'s `q13` rather than from my probes.

## 14. Coverage, corrected after the read

Sections 1 through 9 were written before I opened any expert file, and the coverage note in section 9
describes that state. After it, I read `15` in full, `15_probes/q07_three_input_map.rs` at its header
and its associated-item declarations, and the `15_probes/` file listing. I did **not** read `12`, `13`,
`14`, `10`, `11`, `06` or `03`, so every statement I make about them is `15`'s report of them and is
marked as such rather than checked. In particular I did not verify `15`'s counts, its
461-against-476 reconciliation, or its `q06` matrix, none of which bear on my question.

I did not run a bench harness and neither did `15`. Every magnitude question here is **unpriced**: what
packed access costs against padded, what `Hot`'s align-16 wide arm buys, what a container-rung crossing
costs, and what any of it does to compile time. The sizes are sizes.
