# 167. The chain, derived cold

**Member:** Tiark Rompf persona. **Unit:** the chain topic, opened by `166`.

**Phase one: written blind.** Premises only, per the cold-derivation protocol in `RULES.md`. No numbered
panel file, no register, no other member's probe, no commit log, no commit subject was read before this
file was committed. What I did read is listed in section 0.3 and includes one item the blind list did not
anticipate and which leaks panel conclusions; I name it there rather than let it pass.

---

## 0. The two gates, and the coverage bound

### 0.1 The canon gate: PASSED

Checked against `INTENTS.md` read in full, including its "How to read an entry" section, and against
`RULES.md` read in full.

The question is licensed and the licence is direct. **I7** is op's, it is STATED, and its wording ranges
over compositions rather than over single operations:

> Precise on other hand is the one that sacrifices as much performance and efficiency as makes sense, to
> be the most precise possible answer, throwing out all cold or hot axis optimisations to be *accurate*
> and *precise*, especially within chains and ops, not only alone.

`mock/canon/` does not exist, nothing is ratified except I13, and I13 is about predicated arms rather
than about compositions. So there is no ratified text this unit could misalign with, and the unit is not
building on a ratified state that would have to be re-derived.

**A second intent bears on this question and the brief did not name it.** I11:

> our main selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo and
> project I have, downstream, use. As well as the contracts for things that compose to bigger units than
> just numerals alone.

"The contracts for things that compose to bigger units than just numerals alone" is a statement about
composition contracts being the point of the library, and it is op's own. It is at least as load-bearing
for this unit as I7 is, and reading the unit as an I7-only unit understates what op has said about it.
I take both as premises below.

### 0.2 The test gate: PASSED, and it reconciles two figures the record disagreed on

Run crate by crate at `--release` per the brief. Commands and raw output in `167_probes/gate/`.

| crate | tests | result |
|---|---|---|
| bitpack-carrier-shared | 9 | ok |
| bitpack-contend-shared | 12 | ok |
| bitpack-footprint-shared | 6 | ok |
| bitpack-plan-shared | 5 | ok |
| bitpack-shared | 3 | ok |
| bitpack-wide-shared | 6 | ok |
| quantiser-fadd-shared | 1 | ok |
| quantiser-radix-shared | 3 | ok |
| satfold-shared | 11 | ok |
| warm-clamp-shared | 7 | ok |
| warm-container-shared | 15 | ok |
| wide-rung-shared | 30 | ok |
| **subtotal, twelve crates** | **108** | ok |
| bitpack-write-contend-shared, `--test-threads=1` | 15 | ok, 2.25s |
| **total, thirteen crates** | **123** | ok |

`holds for: profile = release, threads = 1 for bitpack-write-contend-shared and default for the other
twelve, host = this machine, toolchain = the committed pin`

**This reconciles two counts that have been treated as competing.** 108 across twelve and 123 across
thirteen are both correct and they are not the same measurement: 108 is the twelve crates that run
unserialised, and 123 is all thirteen with the write-contention crate given `--test-threads=1`. The
thirteenth **does terminate** when serialised, in 2.25 seconds at `--release`, so a record saying it does
not is a record of an unserialised run rather than a property of the crate. I did not touch that crate.

Four other variant crates are reported to fail to build on a pre-existing cause. That is outside the
thirteen and I did not investigate it.

**Read rather than counted.** `satfold-shared`'s eleven bodies in full;
`bitpack-shared`'s three and their `check_size` helper; `wide-rung-shared`'s `per_width!` macro. I
scanned every `#[test]` in all thirteen crates mechanically for the tautology shapes: eighteen bodies
contain no `assert` or `panic` token, and every one of them delegates to a helper or a macro that does
assert; I opened four of the eighteen and confirmed this rather than inferring it.

**The suite is not decorative, and `satfold-shared`'s is the strongest I have read in this workspace.**
It carries four deliberately-wrong kernels as negative controls (`WrongOp`, `DropsALane`,
`DropsTheRemainder`, `DropsOneElement`), it asserts each defect exactly where that defect is
*expressible* and skips it where asserting would assert something false, it pins the instrument's own
sensitivity boundary as a two-sided assertion rather than deleting the case that failed, and it checks
the workload is non-degenerate with a range that can fail. `satfold-shared/src/lib.rs`'s
`saturating_addition_is_associative_at_eight_bits` closes the law over its whole domain,
`assert_eq!(total, 1 << 24)`, and its companion proves the false gate is genuinely false. That pair is
directly load-bearing for this unit and I use it in section 5.

### 0.3 Coverage bound, and one leak the blind list did not anticipate

**Read in full:** `INTENTS.md`, `RULES.md`, `mock/Cargo.toml`, `rust-toolchain.toml`, the repository's
`.claude/` rules and the workspace rules that load automatically, `satfold-shared/src/lib.rs`.

**Grepped or skimmed:** `mock/benches/bench.toml`, the variant directory listing, the thirteen shared
crates' test bodies.

**Not opened:** any numbered panel file, any register, any other member's probes, the git log, the
committed CSV rows, `mock/crates`.

**The leak.** The brief permits reading `mock/benches/` including the variant crates. `satfold-shared`'s
module documentation **cites panel files by number and quotes their conclusions**: it names `80` section
5.3 and `82` section 9, reproduces their instructions-per-element figures, and says which arm lost and
why. So a member told to read the bench crates and not the panel has been handed a panel conclusion
anyway. I read it before I understood what it was, and I cannot unread it.

What it contaminated, named precisely so the discount is applied to the right thing: my section 5 uses
the *existence* of a licensed reassociation and its per-operator asymmetry, and `satfold-shared` told me
that a prior file had studied it. It did not tell me the definition of a chain, the observation-boundary
argument, the residual argument, or anything in sections 1 through 4, all of which I derived before
opening that file. **The honest handling is to treat section 5's framing as contaminated and to hold the
rest at full cold rung**, and to say that the blind list needs a line about bench crate documentation,
because this is a general channel rather than one file's accident.

**Which sections move if something I leaned on is wrong.** Sections 1 through 4 rest on op's own words
and on my own probes and would move only if I misread I7 or I11. Section 5 rests additionally on
`satfold-shared`'s committed law tests, which I re-derived independently in `167_probes/assoc/` rather
than citing. Section 7's fork rests on I15's "never any runtime checks, ever" and would collapse if that
sentence admits a reading I have not found.

