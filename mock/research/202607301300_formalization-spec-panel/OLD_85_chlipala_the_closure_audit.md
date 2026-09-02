# 85. The closure audit: six real closures, one miscounted, one mislabelled, and the four owed second reads

Adam Chlipala, file 85. I wrote file 41 (the rational bias) and file 64 (the owed second reads bundle
this file's discipline descends from). Nothing in either survives unexamined here; where file 64
recurs below it is as a citation checked fresh, not carried on trust.

**What I read.** `78_consolidation_eight.md` in full, the standing base, and its own owed-item lists
(section 2's numbered list, section 4's "what is open," the droplist) read against the tree rather than
against the compressed forward-carry every later file gives them. The nine deliverables since:
`79_dolan_what_capacity_is.md`, `79b_op_the_verification_mandate.md`, `80_leroy_the_verification_
bundle.md`, `81_fog_is_the_bitpack_cost_inherent.md`, `82_pesce_the_stretch_assembled.md`,
`82b_op_checkpoint_twenty.md`, `83_lattner_how_many_widths.md`, `84_leijen_failure_that_is_not_a_
range_event.md`, each in full. One `ls` of the panel directory, current through `84_probes`. The
shipped tree I touched for exactly four things: the standing canon-gate greps, the tautological test
at `arvo-tensor/tests/capacity.rs`, the current state of `mock/Cargo.toml` / `mock/Cargo.lock`, and
`sysctl` on this host. All four are checks of a claim before reasoning from it; no conclusion below
survives the deletion of any tree citation.

**Gates.** Canon gate, fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
--include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty. Test gate:
`cargo test --offline --workspace` from `mock/`, summed per binary from every `test result:` line,
**666 passed, 0 failed, 9 ignored**, matching files 81 through 84, and I ran it myself rather than
trusting the printed headline. The tautological test at `arvo-tensor/tests/capacity.rs:14-18`
(`dim_cap_is_typed_and_exact`) is confirmed by reading its body: the impl at `capacity.rs:48` is
`const CAP: Cap = cap(N)`, so `assert_eq!(<Dim<3> as Capacity>::CAP, cap(3))` is `cap(3) == cap(3)`
after monomorphisation on all three of its lines. It stands exactly as `78:874-876` and files 82
through 84 carry it, flagged for deletion, outside this panel's scope to touch. Toolchain
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml`, confirmed inside the tree this session.

**What is compiled, what is reasoned, and what I mean by "recompiled."** Where a probe already exists
in `79_probes/` through `84_probes/`, I did not merely read its `OUTCOMES.md`. I ran `rustc` on the
source file myself this session, on the pinned toolchain, and report the diagnostic I got, not the
one the file quotes. Three probes recompiled this way below (`80_probes/probe_4b_safe_impl_refused.
rs`, `82_probes/probe_1_allones_at_real_precisions.rs`, `82_probes/probe_2_foldexact_without_allones.
rs`); each result matches its file's claim exactly. Everything else traces to `OUTCOMES.md`, cited as
such, or to a cheap independent measurement (`sysctl`, `grep`, `git log`, `cargo test`) I ran myself
and report as measured. Sections 2.2 and 2.3 include algebraic derivations that are mine, not
compiled, and are marked reasoned.

---

## 1. The closure audit

### 1.0 Which seven, and the arithmetic behind the number

Op's directive names "the seven just closed" without listing them; the list has to be reconstructed
from `78` section 4 (ten-plus owed items, no closing artifact named for any) against what `79`
through `84` actually did. Walking `78` section 4 item by item against the nine deliverables:

| item, as `78` states it | closed by | artifact |
|---|---|---|
| A. `Specials` primary-source check on the E4M3 exponent figure (pending since `68b`) | `80` §5 | `62_probes/primary_sources.md`, re-confirmed |
| C. IEEE 754's overflow-tie sentence (§4.3.1, corollary only) | `80` §5 | verbatim quotation, no probe file |
| D. OCP mode-split facts behind file 71's declined ground 4 | `80` §5 | verbatim quotation, no probe file |
| E. `Crosses`'s own second read, now carrying statement P | `80` §1 | `80_probes/probe_4`, `probe_4b` |
| F. Statement 0 against `quantize` and `roundToIntegralExact` | `80` §2 | `80_probes/probe_2` |
| I. The nine-bit `u16`-class companion model | `80` §6 | `80_probes/probe_3` |
| K. The `Layout::Bitpacked` follow-up (op's own item 4, `78:853-854`) | `81` in full | `81_probes/`, `mock/benches/bitpack-*` |

Six from file 80, one from file 81, seven total, and this is the set I audit. Three items file 78
lists as owed (the §5.12 citation, `foldnum` against the real four-member contract with `Exponent`
fixed, the non-default `Canonical` compile, the constructive-extensibility compile) remain genuinely
open; I checked each is still absent from every deliverable through file 84 and found no closure
claimed for any of them. That is correct restraint, not a gap in this audit.

One thing worth stating before the item-by-item pass: **file 82's own sentence naming this set is
internally inconsistent, and the inconsistency does not propagate, by luck rather than by check.**
`82:577-578`: "File 80 closed five owed items in one file: the `Crosses` second read, statement 0
against both operations, the E4M3 primary source, the overflow tie, the OCP mode split, and the
nine-bit companion." That is six items in a list introduced by the word "five." The paragraph's own
final figure, "seven items ride at once" (`82:580`), is arithmetically right only if the reader counts
the enumeration (six) rather than the label (five), plus file 81's one. Op's `82b` repeats "seven" and
inherits the correct number, but nothing between file 82 and `82b` reconciled the off-by-one inside
the sentence that produced it. This is precisely the discipline gap file 83 later names against the
same file's "container" count (`83:320-335`, addressed in section 1.5 below): a number was published
that its own neighbouring text does not support, and the fact that the *final* figure survived is not
evidence the *arithmetic* was checked.

### 1.1 Confirmed real closures on their own terms: E, F, K

**E, the `Crosses` second read.** `80_probes/probe_4_crosses_unsafe_marking.rs` compiles clean;
`80_probes/probe_4b_safe_impl_refused.rs` refuses. I recompiled `probe_4b` myself this session
(`rustc --edition 2021 --crate-type=lib --emit=metadata`) and got exactly the diagnostic file 80
quotes:

```
error[E0200]: the trait `Crosses<SomeNumeral>` requires an `unsafe impl` declaration
```

The claim ("the impl is blanket and safe," `68:271`, is unspellable) is exactly what this refusal
shows, on the literal sentence rather than a paraphrase of it. Closed, on its own evidence, compiled
by me and not merely read.

**F, statement 0 against `quantize` and `roundToIntegralExact`.** `80_probes/probe_2_statement0_vs_
quantize_rtie.rs`'s `OUTCOMES.md` entry states exhaustive closure over 16,000,000 pairs at file 66's
decimal model, with the specific counts (5,679,000 refusals, 2,889 of 4,000 x's cohort-affected) that
`80`'s prose reports. I did not recompile this one (it is a 16M-cell exhaustive run, not a cheap
recompile), but file 84 independently reproduces the identical count (5,679,000) through a separately
written construction (`84_probes/probe_1_refusal_is_overrange.rs`, confirmed below in section 2.2),
which is the strongest available corroboration short of rerunning it myself: two independent authors,
two independent programs, one number. Closed.

**K, the `Layout::Bitpacked` follow-up.** `81`'s bench artifacts exist (`mock/benches/bitpack-decoder-
shape_*`, `bitpack-kernel-amortisation_*`, confirmed present) and `81_probes/` holds the disassembly
probes. But this closure was not self-contained at the moment it was claimed: file 82 found the
crates were not registered in `mock/Cargo.toml` or `mock/Cargo.lock` at HEAD (`82:598-616`), so a
fresh clone could not reproduce the 666-test count or rebuild the benches file 81's headline rests on.
I confirmed the fix independently: `git log` shows commit `2e2b423` ("bench: register the plan-driven
and mac bitpack variants") landing between files 81 and 82, and the current `mock/Cargo.toml` /
`mock/Cargo.lock` carry ten and twenty hits respectively for `bitpack-plan`/`bitpack-mac`, matching
file 83's confirmation (`83:31-34`) exactly. `cargo test --offline --workspace`, run by me from a
clean invocation, reports 666, matching. Closed, but only as of `2e2b423`; for the window between
file 81's commit and file 82's fix, the closure did not exist as claimed. Worth stating plainly for
the next reader: a closure that depends on a manifest edit is not closed until the manifest edit
lands, and "the artifact exists in my working tree" is not the same claim as "the artifact exists at
HEAD."

### 1.2 Closures that are real but structurally different from the rest: C and D

**C, the overflow tie**, and **D, the OCP mode split**, both close on a verbatim quotation from a
primary source, reproduced in file 80's own prose (`80:319-345` for C, `80:310-317` for D). These are
not compiled artifacts and there is no probe file for either. I checked both anyway, since a quotation
is exactly the kind of factual claim this dispatch is instructed to verify cheaply. The IEEE 754-2008
clause 4.3.1 quotation ("an infinitely precise result with magnitude at least b^emax(b - 1/2 b^(1-p))
shall round to infinity") is not independently re-fetchable by me inside this session (I have no
access to a fresh copy of the standard), so I cannot re-verify the primary text itself; what I can and
did verify is that the arithmetic file 80 performs on the quotation (the threshold equals the maximum
finite plus half a top-binade ulp, and "at least" includes the tie) is correct algebra given the
quoted sentence, and that it is consistent with the design's own extended-grid boundary at
`78:288-293`. The E4M3 Table 3 citation I could check more directly, since `62_probes/primary_
sources.md` is in this repository and I read it: it does not carry a Table 3 entry (that file's own
E4M3 content is Table 1, Table 2, and the §4.2 transposition, confirmed in section 1.4 below), so file
80's Table 3 read genuinely is new sourcing this session rather than a re-read of an existing probe,
exactly as `80` claims ("Table 3... is hereby primary-sourced," `80:310`).

**The structural point worth naming.** Op's adopted fix ("an owed item names the artifact whose
existence would close it," `82b:52-58`) reads naturally as "name a file." C and D show the fix needs
one more clause: a closure grounded in a primary-source quotation closes on the quotation being
reproduced verbatim, with position, inside the deliverable that closes it, and a future audit of such
an item has to re-fetch the source rather than `grep` a probes directory. That is a real, different
verification cost from a compiled closure, and the discipline as stated does not distinguish them. I
suggest the "names the artifact" rule gain a second clause for this shape: a primary-source closure
names the document, edition, and position, in the same sentence that closes it (both C and D already
do this, so nothing needs fixing in the record; the rule text should just say so, so a future closure
of this shape does not skip the position citation and become unauditable).

### 1.3 A real closure with a corrected label: I, the nine-bit companion

`80_probes/probe_3_nine_bit_companion.rs` exists, and its `OUTCOMES.md` entry matches file 80's prose
exactly: 65,024 same-value-different-padding pairs, every one misordered under a raw-carrier compare,
Equal under the canonical one. The computation is not in dispute. What is in dispute, correctly, is
what file 80 said the computation proved: "it is the first point at which the padding half of the
crossing contract (statement P) has observable content" (`80:377-380`, restated at `80:457-459`).
File 83 found this holds only under the container reading of `StoredWidth`, which its own section 2
forecloses on three independent grounds (`83:95-129`, checked by me in section 2.1 below): under the
forced reading, the nine-bit model's seven padding bits sit at `[W_S, W_C)`, where statement P is
vacuous, and what the probe actually measured is the ungoverned third level, not statement P's
content (`83:157-166`).

**So this closure is real at the artifact level and wrong at the interpretation level, and nothing in
the record between file 80 and file 84 has formally corrected it.** `78`'s section 4 is what a future
consolidation will absorb; there is no `83b` or `84b` yet folding file 83's relabelling into the
open-items list, and the correction currently lives only inside file 83's prose. If the ninth
consolidation absorbs file 80's headline for item I without also absorbing file 83's correction of it,
the record will state a false thing with a real probe behind it, which is exactly the "confidently
wrong because the citation is real" shape this review has flagged before at the strategy-door table
(`78` section 1.19). I flag it explicitly here so the next consolidation does not have to rediscover
it: **item I closes on the artifact, not on the sentence file 80 attached to it; the sentence is file
83's, not file 80's.**

### 1.4 A closure that was true before it was ever marked owed: A, the E4M3 primary source

I re-read `62_probes/primary_sources.md` myself this session rather than trusting file 80's quotation
of it. It carries, verbatim: Table 1 (E4M3 bias 7, emax 8, emin -6), Table 2 (max normal ±448, no
infinities), and the §4.2 contradiction (E4M3 stated with "an exponent bias of 15," transposed against
Table 1 and the value formula). All three match file 80's re-quotation exactly, position for position.
`63:757` ("witnesses now primary-sourced") predates `68b:36-37` marking the same check pending, and
both predate `78:850-852` carrying it forward as "not performed this stretch." Op's own `82b` calls
this "one had already been performed two stretches before it was marked owed"; the timeline I traced
independently confirms it: file 62 (which built the probe) is earlier in the corpus than `68b` (which
marked it pending), and no deliverable between `68b` and `80` grepped `62_probes/` to check. **The
defect was not that the check went undone; it was that the open-items list had no mechanism for
noticing it had already been done.** That is exactly the defect op's adopted fix targets, and I have
nothing to add to file 80's or file 82's diagnosis of it beyond confirming the primary text matches.

### 1.5 Two counts caught mid-audit, worth naming precisely rather than gesturing at

Two count discrepancies surface across files 79 through 83, and both are worth pinning down exactly
because the panel's own running theme is "a count in a member file names the command that produced
it" (`82b`, adopted; `83:333-335`). Neither of the two below is a new finding; both are already caught
in the record. I re-derived both independently and can say precisely why the numbers differ, which
neither correcting file states.

**The arity count.** File 80: "25 occurrences in file 64 and 14 in file 55" (`80:392-393`). File 82:
"27 and 15 on my count this session" (`82:199-203`). I ran both counting methods myself:

```
grep -c "[Aa]rity" 64_chlipala_the_owed_second_reads.md   -> 25   (lines matching)
grep -o "[Aa]rity" 64_chlipala_the_owed_second_reads.md | wc -l -> 27   (total occurrences)
grep -c "[Aa]rity" 55_mcsherry_typing_the_algorithm_crates.md   -> 14   (lines matching)
grep -o "[Aa]rity" 55_mcsherry_typing_the_algorithm_crates.md | wc -l -> 15   (total occurrences)
```

**Both numbers are correct, under two different, unstated counting methods.** File 80 counted
matching lines; file 82 counted occurrences (two files have one line each carrying the word twice).
Neither states which. This is not a new instance of the count-discipline gap; it is the same
instance, more precisely diagnosed: the fix op adopted ("names the command that produced it") already
covers this, because the command, not just the number, disambiguates `-c` from `-o`. File 84 complies
with the fix (`84:541-543`, "every count in this file names the artifact that produced it"); this
paragraph is the retrospective proof that the fix was necessary and sufficient, not merely tidy.

**The "container" count.** File 82: "the word 'container' appears in the consolidation exactly three
times" (`82:361-363`). File 83 recounted: eight (`83:320-335`), and I reran it myself: `grep -c
"\bcontainer\b" 78_consolidation_eight.md` gives 8, exact word, no substring inflation from
"containers" or "arvo-container." File 83's correction is right and file 82's original count is wrong,
not merely differently-counted (there is no alternate method under which "three" is defensible; the
word occurs at lines 128, 656, 680, 869, 900, 903, 946, 956). File 83 already names this the third
count-in-a-row published without re-derivation; I have nothing to add except that my own recount
agrees with file 83's, not file 82's, exactly.

### 1.6 What remains open despite reading like it might be closed

Three items on `78`'s owed list are untouched by name through file 84, and I checked each explicitly
rather than assuming the silence means closure: the IEEE 754-2019 §5.12 citation (file 82 itself
checks and confirms this is genuinely distinct from the §5.2 citation file 62 already carries,
`82:568-573`, correctly declining to claim a false second instance); `foldnum` compiled against the
real four-member `Numeral` contract with `Exponent` held fixed (mentioned nowhere in files 79 through
84 by that description); the non-default `Canonical` compile (same, absent); and the
constructive-extensibility compile, which file 83 explicitly names as "still owed" rather than
performed (`83:391-394`, the only one of the four to be mentioned at all this stretch, and it is
mentioned as open). None of these four is claimed closed by anyone; I checked to be certain the audit
was not missing a silent closure, and it was not.

### 1.7 The discipline's own gap, stated once

Op's adopted fix is sound and the audit above shows it working: six of the seven items this stretch
closed have a real, checkable artifact, and the one whose artifact was temporarily unreproducible
(K) had that fact caught and fixed inside the same stretch. The two gaps worth carrying forward are
process gaps, not correctness gaps: primary-source closures (C, D) need a stated position-citation
convention rather than a probe-file convention, and a closure's *characterisation* (item I) is not the
same claim as its *artifact*, and the two can drift apart with nobody noticing until the next member
happens to re-derive the interpretation from first principles rather than trusting the label.

---

## 2. The four owed second reads

Full blindness was not available to any of these; each was required reading before I could reach it,
and each file's own conclusions are already visible in the consolidation-shaped summaries I read
first. What follows is my own independent derivation of the load-bearing claim in each, checked
against sources the first reader also had, with disagreement stated where I found grounds for it and
brevity where I did not, per the standing instruction that a confirmation is a sentence and a
disagreement is the file.

### 2.1 File 83: three width levels, one axis. Confirmed, with the theorem claim independently checked

File 83's central move is that `StoredWidth`'s ratified "minimum" cannot denote the container's
rounded-up width, on three grounds (`83:99-118`): it would contradict `Cold`'s ratified zero-padding
meaning of `Layout::Bitpacked`; file 75's own "minimum" is logical-relative by its own argument; and
the axis's original instance names ("Minimum," "DoubleLogical") are logical-relative by construction.

I worked through the first ground independently rather than taking it as given, because it is the one
doing the real work (the other two are corroborating, not load-bearing). If "minimum" meant the
dispatched primitive's width (16 for a 13-bit value), then under `Cold` (minimum, bitpacked), every
stored value would occupy a fixed 16-bit slot with 3 bits that never vary between values, which is
inter-value padding in the plainest sense: the same three bit positions are unused in every value,
repeating with the stride of the type. That is precisely what `Layout::Bitpacked`'s ratified single
meaning, zero inter-value padding (`78:552-556`), forbids. The contradiction is not subtle once
stated this way, and I could not find a reading of "container width" for `StoredWidth` under `Cold`
that survives it. Confirmed independently.

The "theorem, not obligation" claim about zero-padding under `Bitpacked` (`83:216-218`) is arithmetic
I checked directly rather than trusted: with `P = 8/gcd(W,8)` and `G = W*P/8`, the identity `G*8 =
W*P` is a restatement of `G`'s own definition once `W*P` is shown divisible by 8, and it is, because
`W*P = W * 8/gcd(W,8) = 8 * (W/gcd(W,8))`. There is no separate fact to discharge here; the claim
that the group is always whole bytes follows from the definition of the group by algebra alone, which
is exactly file 83's point in calling it a theorem rather than an obligation.

I read the two sections file 83 itself names as the halves to attack (`83:380-382`) and found no
counter-argument in either. Confirmed, one sentence per the standing discipline: agreement, no
disagreement to elevate.

### 2.2 File 84: `quantize`'s hard failure is `OverRange` on the target numeral. Confirmed, with an independent mathematical grounding for the load-bearing claim

I recomputed the central premise-check myself rather than accepting file 84's framing of file 82's two
premises. Op's own dispatch (via `82b`) hands this file the fork already narrowed; file 84's own
section 1 tests two of file 82's sentences against a decimal oracle before building anything on them,
and finds one false ("rounding to `p` digits changes the value" is wrong; `quantize` is defined to
*change* the value to match the target quantum, and often does, delivering `inexact` rather than
refusing). I ran no oracle myself, but the CPython `decimal` transcript in `84:59-64` is checkable
prose (anyone can run `Decimal('1.234').quantize(Decimal('0.01'))` and get `1.23` with the `Inexact`
flag set), and it agrees with what the General Decimal Arithmetic specification says `quantize` is
for. I accept the premise correction.

The load-bearing claim is that `At<N, Q>`, a numeral sharing `N`'s radix, precision and domain with
its exponent fixed at `q`, is "not a new kind of object" (`84:88-90`) and is an ordinary member of the
ratified `Numeral` vocabulary. File 84 itself names this the half to attack (`84:551-554`). I attacked
it, from a direction none of the compiled probes reach: the design's own founding identity, that fixed
point and float are one formalisation differing in the exponent form (unchanged since file 40 through
every consolidation, restated without argument at `78:186`, and traceable to Flocq's `generic_format`
construction in the formal floating-point literature, where a fixed-point number and a floating-point
number are the same object, `m * beta^e`, differing only in whether the exponent function is constant
or a step function of the mantissa). Under that reading, "the exponent fixed at `q`" is not a new
shape bolted onto `Numeral`; it is the *constant* exponent function, which is exactly what a
fixed-point numeral's exponent already is in this formalisation. `At<N, Q>` is not `N` with something
added; it is what "fixed-point" already denotes, instantiated at a specific constant. That is a
stronger ground for file 84's claim than the ratified trait signature alone gives (`78:618-623` says
`Numeral` has an `Exponent: ExponentForm` member and does not by itself settle whether a constant
counts as an ordinary instance of that member's sealed vocabulary), and I offer it because it closes
the gap file 84 itself left open rather than merely restating the gap.

The `roundToIntegralExact`-versus-`quantize` decomposition (file 80's finding, confirmed by file 84's
own independently-written probe reproducing 5,679,000 refusals cell for cell against three predicates,
`84_probes/probe_1_refusal_is_overrange.rs`) is the second load-bearing piece, and I checked its
`OUTCOMES.md` against file 84's prose directly: the three counts (16,000,000 examined, 5,679,000 A,
5,679,000 B, 5,679,000 C, zero disagreements pairwise) match exactly, and the negative control (15,984
cells differ, as required) is present and asserted. I did not rerun the 16M-cell probe myself; the
independent reproduction against file 80's number, by a separately written program, is the strongest
corroboration available short of that, and I take it. Confirmed, with the added grounding above.

### 2.3 File 82's fold-width rebuild: confirmed by independent recompilation and independent algebraic derivation

This is the one item where I did substantive independent work rather than confirming what was already
shown, because file 82 states plainly that it "has not been read by anyone" (`82:731-735`), and it is
one pass.

**Recompiled, both halves.** `82_probes/probe_1_allones_at_real_precisions.rs`'s binary256 case is
behind `#[cfg(feature = "p237")]`; compiled bare it is silent (the flag is off), which is not what the
file claims, so I recompiled with `--cfg 'feature="p237"'` explicitly and got:

```
error[E0275]: overflow evaluating the requirement `O<I<I<I<O<I<H>>>>>>: AllOnes`
    = help: consider increasing the recursion limit ... `#![recursion_limit = "256"]`
```

matching file 82's quoted diagnostic exactly. `82_probes/probe_2_foldexact_without_allones.rs`
compiles clean under the same toolchain with no flags at all, including its own binary256 assertions
(`foldexact(237,3)=239`, `(237,257)=246`, `(237,4096)=249`, `(237,256)=245`, all four present in the
source and none gated), which I confirmed by grepping the file for its `const _: () = assert!` lines
and running the compile myself rather than trusting `OUTCOMES.md`. Both recompiles reproduce file 82's
claims exactly.

**Derived, independently, algebraically, not merely spot-checked.** File 82 verifies the replacement
formula on 1,225,601 cells and does not include a proof; I worked one. With `L = bitlen(A)`, `R = A -
2^(L-1)`, the claim is `foldexact(P, A) = P + L - 1 + bit`. I checked each of the three branches
algebraically rather than numerically:

- `R = 0` (`A` a power of two): `A*(2^P-1) = 2^(L-1)*(2^P-1)`, whose bit length is `P + (L-1)` exactly,
  since `2^P - 1` has bit length `P` and multiplying by `2^(L-1)` shifts it left by `L-1`. Matches
  `bit = 0`.
- `R >= 1, P >= L`: I showed `2^(L+P-1) <= A*(2^P-1) < 2^(L+P)` directly. The upper bound is immediate
  (`A < 2^L`). The lower bound needs `A >= 2^(L-1)+1` (true since `R >= 1`) and `2^P >= 2^(L-1)+1`
  (true since `P >= L` gives `2^P >= 2^L = 2 \cdot 2^{L-1} \geq 2^{L-1}+1`); combining gives
  `A(2^P-1) \geq 2^{L+P-1} - 2^{L-1} + 2^P - 1 \geq 2^{L+P-1}`. So bit length is exactly `L+P`, matching
  `bit = 1`.
- `R >= 1, P < L`: I hand-checked four cells spanning both sub-cases (`A=5, P=1`: predicted 3, actual
  `bitlen(5)=3`; `A=5, P=2`: predicted 4, actual `bitlen(15)=4`; `A=13, P=1`: predicted 4, actual
  `bitlen(13)=4`; `A=13, P=3`: predicted 7, actual `bitlen(91)=7`), all agreeing. I did not complete a
  general algebraic proof of this branch (the `(R<<P) >= A` test is a direct restatement of whether
  `A*(2^P-1)` crosses the `2^{L+P-1}` boundary, and I am confident it is correct by the same shape of
  argument as the other two branches, but I did not write out the inequality chain the way I did
  above, and I say so rather than round up).

**The suggestion, seconded.** File 80's recommendation to retire `foldnum` to prose stands; file 82's
correction (the replacement construction, not file 80's, because file 80's does not exist above
`p=128`) is the one to adopt. I have nothing to add to file 82's own honest residual (the `CmpP`
mixed-constructor refinements depend on `Pos`'s encoding staying canonical, `82:733-735`) beyond
agreeing it is the right thing to flag rather than assume.

### 2.4 File 79's search claim: independently confirmed false, and the composite-closure argument holds on the grounds file 80 gave it

I grepped for the artifact myself rather than accepting either file 79's negative or file 80's
positive claim on the record's word. `[Aa]rity` hits 55, 62, and 64 substantively (not incidentally):
`62_knuth_the_verification_backlog.md:218-224` names the sealed `Arity` fix directly ("the tower's own
idiom, a sealed `Arity` kind with the finite constructor wrapping a `Pos`... `Fin<P>`"); `55_mcsherry_
typing_the_algorithm_crates.md:127,260` shows the unsealed `InteriorSafety<ArityMinusOne>` hazard the
fix responds to; `64_chlipala_the_owed_second_reads.md:142-193` (my own earlier file) compiles the
forgery and the fix in `64_probes/probe_2_arity_seal/`, with the exact sequence file 80 describes: an
unsealed marker forged for a downstream type, then a sealed `Arity: sealed::Sealed` closing it,
confirmed by a compiled `E0277` on the attack. File 79's stated search ("the hits are all fold-arity,"
`79:139-140`) did not find this, and I confirm file 80's and file 82's finding that it should have.

The substantive question is whether file 80's replacement argument (`Capacity: Nat` has no generic
parameter slot, so the uncovered-type-parameter forgery route the `Arity` carrier needed sealing
against does not exist, `80:404-408`) actually holds, rather than merely being a plausible-sounding
correction. It does, and the reasoning is short: any foreign impl of `Capacity` for a type `T` requires
`T: Nat` first, since `Capacity: Nat` is a supertrait bound, and `Nat` is already sealed by the tower's
own carrier-at-birth mechanism (no external crate can implement a sealed supertrait). The forgery route
that broke the unsealed `InteriorSafety<ArityMinusOne>` (a downstream crate minting its own marker type
and implementing the *unsealed* trait against it directly) has no analogue here, because the
supertrait itself, not just the derived trait, is closed to outside implementation. This is ordinary
sealed-trait transitivity and I see no gap in it. Confirmed, on the grounds file 80 gave rather than
the grounds file 79 gave, exactly as file 82 already flagged (`82:222-225`, "should be the sentence
that ships, not file 79's ordinal-versus-cardinal reasoning").

---

## 3. What a consolidation could take, close to verbatim

*Six of the seven items this stretch's two files closed have a checkable, named artifact matching
what closed them: the `Crosses` second read and statement 0's closure over `quantize` and
`roundToIntegralExact` (both `80_probes/`, one recompiled and reconfirmed this session), the E4M3
primary source (already performed at file 62, two stretches before it was marked owed, confirmed by a
fresh read of `62_probes/primary_sources.md` matching position for position), the overflow tie and the
OCP mode split (primary-source quotations rather than probes, both checked), and the
`Layout::Bitpacked` follow-up (closed at file 81, its manifest gap fixed at commit `2e2b423`, confirmed
reproducible from HEAD). The seventh, the nine-bit companion, closes on its artifact (the probe and
its misordering matrix are correct) but not on its label: file 80's own characterisation of what the
artifact proves is superseded by file 83's, and the record should carry file 83's sentence, not file
80's, as the closing statement for this item. Primary-source closures need their own convention
alongside the artifact-naming one: a document, edition, and position, quoted in the file that closes
them, since there is no probes-directory grep that substitutes for re-fetching the source.*

*File 83's three-level, one-axis reading of the lowering charter is confirmed on independent
derivation of its load-bearing step (the container reading of `minimum` contradicts `Cold`'s ratified
zero-padding meaning of `Bitpacked` directly, not merely by analogy) and its zero-padding theorem is
confirmed by direct algebra (`G*8 = W*P` follows from `G`'s own definition once `W*P`'s divisibility
by eight is shown, which it is, from `P`'s own definition). File 84's identification of `quantize`'s
hard failure with a range event on a fixed-exponent target numeral is confirmed, and the design's own
founding identity (fixed-point and float as one formalisation differing in the exponent function) is
independent grounding that `At<N, Q>` is the ordinary constant-exponent instance of that formalisation
rather than new vocabulary, which is stronger support for the claim than the bare trait signature
alone provides. File 82's fold-width replacement is confirmed by independent recompilation at
binary256 (both the old construction's `E0275` refusal and the new construction's clean compile,
reproduced fresh) and by an independent algebraic proof of two of its three branches, with the third
checked numerically rather than proved. File 79's arity-search claim is independently confirmed false,
and file 80's composite-closure argument for capacity needing no new seal is confirmed sound by
ordinary sealed-supertrait transitivity: any foreign `Capacity` impl requires `Nat` first, and `Nat`
is already closed to outside implementation, so the forgery route the old `Arity` vocabulary needed
sealing against has no analogue on `Capacity`.*

---

## 4. What this leaves open

- **The primary-source citation convention** (section 1.2) is a suggestion, not a ruling: state the
  document, edition, and position inside the closing file, which both C and D already do; the rule
  text at `82b` should say so explicitly for the next primary-source closure that might not.
- **Item I's record needs its sentence swapped**, from file 80's to file 83's, before the ninth
  consolidation compresses it; I state this as a finding rather than doing it, since I am not
  authoring the next consolidation.
- **The `(R \geq 1, P < L)` branch of the fold-width formula** (section 2.3) is checked numerically at
  four cells and not proved algebraically by me; a reader who wants full certainty on this branch
  should complete the inequality chain the way I completed the other two, which I expect to go
  through by the identical shape of argument but did not write out.
- **The `At<N, Q>`-versus-`ExponentForm` question** (section 2.2) still wants the review's own
  formal check, since my grounding is from the external formalism the design is built on rather than
  from a compile against the tower's actual sealed `Exponent` vocabulary; nobody has yet built `At<N,
  Q>`'s exponent as a literal instance of `EZero | EPos<P> | ENeg<P>` and watched it typecheck.
- **The three items still genuinely open** (section 1.6, the §5.12 citation, `foldnum` against the
  real contract, the non-default `Canonical` compile) remain untouched; naming them here is not a
  claim about their priority, only a confirmation that this audit did not silently drop them.

Only op's calls are final, and even those go stale. Everything above is evidence, correction, and
suggestion offered in that spirit, not a ruling.

*Grounded on: ratified (`78` in full, `79b`, `82b`), settled shapes (`79` through `84` in full,
`62_probes/primary_sources.md`, `64_probes/probe_2_arity_seal/`), compiled (`80_probes/probe_4b`,
`82_probes/probe_1_allones_at_real_precisions.rs`, `82_probes/probe_2_foldexact_without_allones.rs`,
all three recompiled fresh this session on the pinned toolchain), measured (`sysctl` confirmation of
files 81 and 82's host figures, `cargo test --offline --workspace` run fresh, `git log` confirming
commit `2e2b423`, `grep` recounts of the arity and container discrepancies), tree-fact
(`arvo-tensor/tests/capacity.rs:14-18,48`, `mock/Cargo.toml`, `mock/Cargo.lock`, existence and current
state only), reasoned (the seven-item reconstruction in section 1.0, the algebraic derivations in
sections 2.1 and 2.3, the `generic_format` grounding in section 2.2, the composite-closure confirmation
in section 2.4, all mine).*
