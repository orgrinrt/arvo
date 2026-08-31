# 203. The catalogue for files 001 through 049

**Date:** 2026-08-30/31. **Persona:** Dolan. **Slice:** panel files 001 through 049 of three
disjoint slices covering the 217-file, 157-uncited gap. **Probes:** `203_probes/` (none needed;
every claim below is a direct citation, a direct read, or a stated confidence gap). **Catalogue:**
`203_catalogue_001_049.toml`, alongside this file.

## What changed mid-task, and why this file is not a porting report

The brief this seat started under asked for a porting pass: find the 157 files no registry row
cites, decide which of them are canon by op's own test ("if it's a real thing that constrains the
work and is needed to know, it's canon"), and add rows for the ones that qualify. Partway through,
the dispatching agent re-briefed with a sharper question underneath the porting one: **why does the
corpus look like this at all**, and is the shape a symptom of something lost, or of something
working as designed.

The answer for this slice is mostly the second. arvo ran an older panel discipline once, one long
panel with a new one opened only when something went wrong; the current shape is a panel per topic,
and a closed panel usually closed because its material had somewhere to go. Files 002 through 039
of this slice sit inside the pre-consolidation window for four topics (`the_format`, consolidated at
file 63; `the_number_system`, at 74; `algebraic_laws`, at 90; `the_strategy_axis`, at 106), and
files 044 through 049 sit inside a fifth, local consolidation at file 53. A raw exploration file
carrying zero direct registry citations is, for most of this slice, the expected shape of a file
that did its job and handed its content to a consolidation, which is what the registry actually
cites.

But not all of it. Two files in this slice, 020 and 022, measured something nobody else had
checked and found the corpus wrong in ways that never made it into a row, and a third, 041,
turned one of those local findings into a corpus-wide fact. Those three are this round's real find,
and they are catalogued as `lost`, not `superseded`, with the registry rows this file adds to close
that gap.

## The method

Every file in the slice got one of four verdicts, never asserted on my own say-so: `live` where a
registry row cites it directly, `superseded` where its content is carried by a later consolidation
file that the registry does cite, `lost` where a real, checkable claim reached no row, and
`exploration` where the file is process, an audit of somebody else's file, or a dead end that was
never meant to become canon. Every row carries a confidence score reflecting how directly I checked
it, not how strongly I believe the underlying claim: a citation count from the registry is high
confidence, a structural inference from a coverage statement I did read in full is medium, and an
inference from file position and naming alone, for the handful of files I did not open this round,
is capped lower and says so.

`git log --follow` confirms every file in this slice is single-commit: first date equals last date,
seven through nine August 2026, for all forty-nine. Nothing was edited in place. That matches this
panel's own discipline against rewriting a committed file rather than superseding it in a new one,
and it means the dating question this round asked for collapses to one date per file rather than a
first-versus-last comparison; the catalogue records that one date and says why the second is not
useful here.

## What is genuinely lost, and what closes it

**The bench corpus has never validated its own arms.** Across every committed bench (214 CSVs,
82,960 rows) the digest fidelity column is zero, everywhere, always. File 020 found the shape of
it by reading the harness against the CSV schema. File 022 found the root cause independently,
`run_orchestrator` never calls `validate`, and the function that would is not re-exported from the
crate root, then fixed it and demonstrated the fix catching a real injected one-character defect.
File 041 re-swept the whole corpus mechanically and confirmed the zero holds everywhere rather than
only where 020 and 022 happened to look. The cost of the gap is concrete rather than theoretical:
one bench family reports 12,365 to 17,022 Gops/s against a roofline of roughly 256, because LLVM
proves a saturating fixpoint absorbs after three of a thousand iterations, and the harness's own
findings text recommends the arm as dominant on the strength of it. This is now
`probe::bench_corpus_never_validates_its_own_arms`.

**A ratified figure from the predecessor panel is wrong, and was never retired.** `seed/OLD_SETTLED_container.md`
item 14 states the wide-rung ragged-versus-word-rounded trade at fourteen instructions and
twenty-five bytes against eleven and thirty-two, three instructions per operation against seven
bytes per value. File 022 measured it at six widths and multiple arms: the instruction gap is zero
at five of six swept widths, and where it is nonzero it is a per-element tail-residue effect, not a
per-operation one. The byte-count arithmetic is correct and never once converts into the throughput
difference the ratified claim used it to argue for. No existing retirement row cited into `seed/` at
all before this round; the grammar for it did not exist and is constructed here from the `panel::`
root's own rules, since `seed/` sits inside the frozen panel directory. This is now
`retirement::r22_r1_the_wide_rung_item_14_instruction_and_byte_claim`, and its `replacement` field
carries the one thing file 022 could not close: a roughly 2.1x width-dependent effect between W=129
and W=192 whose mechanism is still open.

## What I declined, and why declining is itself a result

Most of the slice declines a row. Files 010 through 018 (excluding the checkpoints), 023, 024, 026,
027, 040 and 042 carry real design content but nothing this round found that a later consolidation
does not already state at equal or greater generality: the packing findings in 025 through 027
converge with 106's own packing table, and the ring-homomorphism mechanism in 020's third section is
already stated more generally by a standing proposal. Files 021, 031 are outside checks on other
files by design and say so about themselves. Files 029, 033 are dispatcher process notes that
explicitly disclaim authority. The checkpoint files (009, 014, 019, 048) audit other files rather
than making claims of their own; 009 is read in full and confirmed, the other three are inferred
from the pattern and marked at lower confidence rather than asserted.

Nine files in this slice (010 to 013, 015, 018, 023, 024, 026, 027, 040, 042) were not opened
directly this round; their verdicts rest on the coverage statement in file 030 (which was read in
full and rebuilt the option register directly from files 02 through 27), on the topic table's own
unit ranges, and on file 053's explicit statement that it consolidates 44 through 52. That is a real
basis and it is a weaker one than a direct read, and the catalogue says so per file rather than
averaging it away.

## Where the catalogue lives, and why not in the registry

`203_catalogue_001_049.toml`, beside this file, one row per item, TOML in the registry's own
dialect. Not under `mock/registry/`: that directory is declared canon wholesale by `canon_paths` in
`mockspace.toml`, so anything placed there is treated as settled by the tooling regardless of what
it says about itself. A table that is explicitly uncertain and confidence-scored cannot sit where
the mechanism that reads it does not know how to read `verdict` or `confidence`. Kept in the
registry's own grammar anyway, so the same grep habits that work on `law.toml` or `probe.toml` work
here, and a reader checking one file's status does not need a second syntax.

## Coverage, and what I did not do

Twenty-two of forty-nine files were read in full or in the substantial part relevant to this
question; a further eleven were checked by grep against every registry file for direct citation;
the remaining sixteen rest on structural inference from files that were read in full, at
correspondingly lower confidence, stated per row. I did not open the seed directory's other three
`OLD_SETTLED` files, and I did not check whether any other item in them shares item 14's fate; that
is real remaining work and it is outside this slice's numbered range.

`cargo mock` and `cargo test -p arvo-checks` are both green: 98 tests, no regression, the two new
rows validate cleanly against the schema (`topic` resolves, `provenance` refs resolve, `standing`
and `kind` enum values are in range).
