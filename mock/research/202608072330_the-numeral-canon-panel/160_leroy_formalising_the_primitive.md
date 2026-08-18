# 160. Formalising the primitive

My job here is construction rather than attack. The ninth unit converged, in its reply round, on a
shape nobody has stated exactly: **types as the degenerate case of lenses** (`157:358-362`, S-8,
conceded and adopted outright by `159:169-175`), and **adequacy as two obligations of different
kinds** (`157` sections 3.2 to 3.4, its two most consequential probes rebuilt byte-for-byte at source
by `158`). This file plugs the holes in that shape, determines the bounds each clause holds in, and
states the thing exactly with its predicates.

Two of the holes turned out to be real defects in the converged text, and both are repaired here with
a compiled instance rather than reported: S-8's degeneracy condition is insufficient as worded
(section 2), and S-14's completeness obligation, taken as written, rejects the refinement parameters
the realisation-map topic requires (section 3). Neither defect touches the conclusion it sits inside;
both would have shipped into a canon candidate as stated.

**This file is not the canon candidate.** That is a later dispatch. Everything below is a suggestion;
op decides, and per I12 an opinion given before the experts converge is an ack.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read in full, including its normative "How to read an entry" section,
and against `RULES.md` read in full.

The assignment is licensed: I14 (IN FORCE, `INTENTS.md:268-297`) requires that public API positions
use the stack's own primitives, which presupposes a determinate account of what one is, and I11
(`INTENTS.md:190-197`) makes the base and the contracts above it the library's stated purpose.
Nothing formalised below touches the RATIFIED rung; I13 (`INTENTS.md:214`) is used as an instrument
throughout and argued with nowhere. Everything I formalise is agent output on the presumed-wrong
rung, which is exactly what a formalisation dispatch exists to work over.

**One thing named rather than resolved, because it is op's.** The container premise, `156` item 1:
whether footprint is in the operation set the design ships. `157` F157-4 measured that a container
observation splits every identity class it swept, `159` section 3 mapped which of `154`'s options
each branch closes, and nothing below closes it. Every clause of the statement that the premise
reaches is written conditionally and marked, per the brief's instruction.

### 0.2 Test gate: passed, at 123 across 13, and it is the tenth count

The suite-bearing surface is `mock/benches/variants/`'s thirteen `-shared` crates; `mock/crates/` is
empty by design. Run crate by crate at `--release`, with `bitpack-write-contend-shared` serialised
per the standing instruction, and that crate otherwise untouched: its hang and its soundness bug are
handled elsewhere.

```
12 crates:  9+12+6+5+3+6+1+3+11+7+15+30 = 108
bitpack-write-contend-shared (serial)  =  15
total                                  = 123, all passing
```

`160_probes/run_test_gate.sh`, output at `160_probes/gate_release.out`. The script carries its own
negative control, declared in its header before the run: a crate whose invocation produces no
parseable pass count prints `MISSING OR ZERO` rather than reading as green, and the control line at
the bottom (a nonexistent crate) fires. That control exists because `157` section 0.2 found its own
first sweep measuring nothing and exiting 0, and the class does not deserve a third instance.

**What I read and what I did not.** My surface is the panel's own claims rather than new crate code,
so I read no new test bodies; `154` scanned all 123 mechanically, `155` read
`warm-container-shared`'s fifteen in full, and `157` read `bitpack-write-contend-shared`'s fifteen.
Where a claim below leans on a crate's source, the lines were reread at source by `158` section 1.1
and I reopened the two that carry weight myself: `warm-container-shared/src/lib.rs:187` (`pub trait
Carrier: Copy + 'static`) and `:279-283` (the five native impls). That is a bounded reliance and I
name it: if the three mechanical scans were wrong, my gate inherits the error.

Proceeding.

---

## 1. The statement, exactly

Written to compose with `112` section 9 (`112:898-945`) rather than to replace it: clauses this unit
did not touch are carried by reference, clauses it changed are rewritten here, and clauses it
produced are new. The propagation clauses belong to the realisation-map topic, whose authoritative
ledger is `122`'s per `AGREEMENTS.md` section 9; they are not restated here, because restating a
ledger I did not read would be a fresh compression with no checker.

Each clause below is tagged **[argument]** or **[sweep]**, which is the distinction Q65 records the
notation as lacking a marker for. I am not settling Q65; I am saying which kind each claim is, in
prose, so that a later pass can translate once op rules on the marker.

> **1. A primitive is a value set together with one realisation map taking an exact result back into
> it, over a declared operation set.** Its identity is that structure up to denotation-preserving
> isomorphism. A law is read off it and never declared. [Carried unchanged from `112:904-906`,
> resting on `110`, `63` C1, `90` R1, `109` P2; three instruments on the law half.]
>
> **2. The realisation is a lens: a placement `(carrier, offset, width)` of the value's bits within
> a carrier allocation.** The lens **degenerates to a value exactly where its focus is the sole
> logical occupant of its carrier allocation**: padding is permitted, sharing is not. At a degenerate
> point the language supplies a standalone `Sized` type; everywhere else the primitive is reached
> through its carrier, and no `Sized`-bounded contract ranges over it. Whether a placement has a
> standalone name is a property of the target's addressing, never of the primitive, so the canon
> states the reason and not the arity. [S-8 with its condition repaired, section 2; the
> addressing-not-primitive clause is `157` S-6 carried unchanged. Argument plus compiled instances.]
>
> **3. Weakening a declared refinement is free at every point of the declared range, including the
> packed end, and tightening is a compile-time refusal naming the instantiation.** [The dense half
> is `111` F111-12; the packed half is section 4.3's probe, new here. Sweep at the widths named in
> the predicates.]
>
> **4. Adequacy is the obligation the type owes the denotation, and it is two obligations of
> different kinds, plus an order.**
>
> **Soundness**: the denotation factors through what the type carries, **over every build**. It is
> structural, needs no enumeration, and is not enforceable by a signature, nor by anything that
> inspects one build; the residual obligation is a restriction on what the realisation map may read,
> checkable as a property of a call graph. [`157` sections 3.2 and 3.7, `159` F159-2's sharpening,
> `109:649-651` widened per S-21. Argument, with the compiled violation at
> `157_probes/p8_soundness_is_not_enforced/`.]
>
> **Completeness, up to weakening**: every pair of distinct shipped instantiations is either
> **separated by one witness**, or **connected by a weakening in exactly one direction**. A pair
> with neither, connected both ways and separated by nothing, is a spurious split, and the
> certificate's job there is to refuse. [The repair of S-14, section 3. Sweep at the model width for
> the direction count; the witness half is closed-form over `W in 1..=64`.]
>
> **5. The axis classification and the adequacy obligation are one thing at two granularities**, and
> the per-pair form is the one that discharges, because an axis can be read at some instantiations
> and not at others. [`157` sections 3.4 and 3.6, second-read by `158` section 2; F157-11 carried.]
>
> **6. The three verdicts of the classification age differently as the operation set grows.** A
> separation, once reachable, never becomes unreachable, so **declared semantics is stable**. A
> refinement pair shares its realisation map, so no growth of the operation set ever separates it,
> and **refinement is stable**. Only **spurious is provisional**: growth can convert it to declared
> semantics and nothing can convert it back. Eliminating a spurious axis from the parameter list,
> as opposed to gating arms on its inertness, is therefore licensed only where the verdict holds at
> the **largest operation set the design will ever admit**, which with a full literal is the
> realisation map's whole domain. At the shipped set, a two-direction verdict is a licence the
> resolver may take under a predicate, not a reclassification of the axis. [New as a statement;
> assembled from `110` F5/F6, `111` section 6, and `108:827`'s licence clause. Argument, section 4.1.]
>
> **7. The identity an operation set induces is determined by the reach of its terms into the
> realisation map's domain; it is monotone in that reach, saturates when the reach is the whole
> domain, and a full literal reaches saturation at depth one.** This holds **over operation sets
> whose members are functions of the value set and the realisation map**. An observation of the
> container is outside that class and splits every class it touches, so whether identity saturates
> at the literal is decided by `156` item 1 and is not decided here. [S-10 and S-11 carried
> unchanged with the premise inline; F157-4's measurement behind the premise. Argument, with the
> sweep at `157_probes/p4_output.txt`.]

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a
container width, a marker, a type parameter, a crate, or a count. Clause 2 names `Sized`, which is a
statement about what the clause is *about*, the boundary between a language's addressable values and
everything else, and the sentence survives translation to any language with the same boundary.

**Equivalence.** Two teams implementing this produce units that behave the same on what matters: a
sole-occupant placement is an ordinary value and a shared placement is reached through its carrier;
weakening never costs and tightening never compiles; no pair of shipped types is connected both ways
and separated by nothing; and no axis is deleted from the surface on the evidence of the shipped
operation set alone. They differ on the lens's spelling, the sugar at the degenerate point, and the
boundary shape at the wall, which is exactly the residue O-C still holds (section 5.1).

