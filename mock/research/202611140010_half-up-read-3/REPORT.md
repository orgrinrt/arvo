# Independent read 3 of `9436c5eb` (half_up reader), as returned

VERDICT: merge after fixes

Canon gate: passed against `ruling::half_up_denotes_ties_toward_positive_infinity` (ratified, experts; `mock/registry/ruling.toml`). The lint refuses prose that gives `half_up` ties away from zero, and that is the ruling defended. One doc misstates the ruling (finding 3). This is the first read of that point; a second is owed.

Test gate: failed on setup that helps and on a sampled walk. No tautological test. Runs were in a scratch pack built from `git archive` of each commit:
- Normal run on `9436c5eb`: 139 passed, 19 ignored.
- `--ignored`: all 19 red, 16 `catalogue_case!` tests and 3 shape walks.
- Undoing the owner restriction alone turns 2 tests red (`every_neighbour_reads_right_on_both_sides` and the owner walk).
- Undoing the article rule alone turns 2 tests red (the same neighbour test and the denial walk).
- The walks fit their inputs to the line the code draws. The owner walk (`shape_tests.rs:177-205`) uses only in-list owners and hand-picked "own authorities". It never walks `ALIAS_OWNERS` itself (`754` is never walked) and never uses an owner outside both sets. The denial walk (`:294-300`) never puts a rule noun after a preposition. That is why finding 1 survives a green suite.

## Findings

1. High, new reader wrong and old reader right (from `9436c5eb`). `reading/escapes.rs:173-176`: `described` looks for `RULE_NOUNS` or `rounding` anywhere between the negator and the relative, so "a rule" or "a mode" inside a phrase counts as a denied rule. All of these are false. Old and middle readers fire on each; the new reader is silent:
   - "`half_up` is not, as a rule, slow, which means a tie goes away from zero."
   - "`half_up` does not, as a rule, truncate, which sends a tie away from zero."
   - "`half_up` is not slow as a rule, which means a tie goes away from zero."
   - "`half_up` is not costly as a mode, which means a tie goes away from zero."
   - "`half_up` is not slow in any mode, which means a tie goes away from zero."
   - "`half_up` does not drift under this rule, which means a tie goes away from zero."
   - "`half_up` is not exact for this operation, which means a tie goes away from zero."
   - "`half_up` is not biased in rounding, which means a tie goes away from zero."

   Fix: stop the scan at the first `as|in|under|like|for|with|at|on`. Tried in scratch: suite 140/140 green, and all eight fire again. Add two or three of these as neighbour pairs.
2. High, new reader wrong and old reader right (from `c4d665d0`, still open). A clause that names an alias owner's rule in words is let through even when it also names the mode by a description. Old fires; middle and new are silent:
   - "`half_up` is the ruled mode. Like Java's rule, the mode sends a tie away from zero."
   - "... Unlike Java's rule, the mode sends a tie away from zero."
   - "... Like Python's rounding, this mode sends a tie away from zero."
   - "... Under Java's rule and under this one, a tie goes away from zero."

   The cause is `names_a_rule_in_words` feeding `names_nothing` (`reading.rs:366-371`). The old reader already let the code-span form through ("Unlike `HALF_UP`, the mode sends ..."). Fix: catalogue it as a let-through pair under its own ticket, twin "`half_up` is the ruled mode. Under Java's rule a tie goes away from zero.", or fire where the clause also names the mode by a `RULE_NOUNS` description.
3. Medium. The new closed list `ALIAS_OWNERS` (`reading/vocabulary.rs:356-359`, used at `reading/words.rs:177`) has no catalogued residue, against the commit's own claim. These are all true and all refused; old fires, middle silent, new fires. Following "`half_up` is the ruled mode.": "Under Rust's rule ...", "Under C's rounding ...", "Under Excel's rounding ...", "Under the JDK's rule ...", "Under BigDecimal's rule ...", each ending "a tie goes away from zero." The same with "Java 8's" and "Python 3's". "Under MATLAB fi's rounding ...": the ruling's own words are "MATLAB `fi`'s `Round`", and the code-span `fi` form fires in middle and new. The doc says the ruling "gives the alias to" these owners. It does not: it lists who computes ties away, and the alias is `toward_zero(x + sign(x) q/2)`. Fix: add `fi`; catalogue the outside-owner pair under a ticket; walk `for owner in ALIAS_OWNERS`; reword the doc to "owners the ruling names as computing ties away".
4. Medium. The article-rule residue is not catalogued. Entry 4 in `known.rs` covers only the hyphenated case. These are all true and all refused; middle silent, new fires (old fires too): "`half_up` is not the one Java uses, which sends a tie away from zero." "... is not the default in Java, which ..." "... is not the JDK's, which ..." "... is not the round function of C, which ..." "... is not the variant Excel ships, which ..." Fix: add one pair, for example "is not the one Java uses, which" against "is not the slow one, which", under its own ticket.
5. Medium, docs. These counts are true only of what was catalogued, and findings 2 to 4 are known cases outside it: `half_up_is_not_a_magnitude_rule.rs:26` ("Sixteen cases are known, seven let through and nine refused"); the `SEVERITY` doc at `:151-156` ("those nine"); `:94` says "described with a rule noun", but the code also takes `rounding`, and takes the noun anywhere up to the relative. Fix: recount after cataloguing findings 2 to 4.
6. Low. `REPORT.md` at `9436c5eb` describes `63095320`. Line 56 says the catalogue was left with no refused-true entry, and line 72 says `--ignored` gives 4 failed (it gives 19). Add a section for the last commit. Merge `4de50da7` reuses the `fix:` subject of `63095320`.

## Checked and sound

- Second read's finding 1: each own-authority owner (op, registry, panel, arvo, ruling, canon, the crate, the vocabulary) fires. Java, Python, IEEE and IEEE 754 stay silent.
- Second read's finding 2: "the slow one" fires, and "symmetric rounding" and "commercial rounding" are silent.
- Catalogue: 12 new entries (indices 4 to 15). Each is red under `--ignored`, and its twin reads right (`every_twin_in_the_catalogue_reads_right` passes). All 9 ticket ids are in `tickets-arvo.toml` and each has one ledger row. Directions are 7 let through (0, 1, 3, 12 to 15) and 9 refused, which matches the docs and the count test. The second read's residues ("5 go", drifting, the copula, seats) are now catalogued.
- Pronoun ticket: all three `FIXME`s name it (`reading/words.rs:126`, `reading.rs:298` and `:452`). Entries 0, 3 and 5 to 8 and the ignored walk sit under it, the across-stop and in-clause cases both. `reading.rs:279-283` matches what the reader does.
- Files and prose: the largest file is 473 lines. Added lines have no em-dash, en-dash, curly quote, bold or TODO.
- Commits: subjects are 48 to 65 characters and in form, with no bylines.
- Unchanged: `SEVERITY` is `ADVISORY`, and `mockspace.toml` and the registry are untouched.
