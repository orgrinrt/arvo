# Independent read 2 of `63095320` (half_up reader), as returned

VERDICT: merge after fixes

Canon gate: passed against `ruling::half_up_denotes_ties_toward_positive_infinity` (ratified by experts): `half_up` is `floor(x + q/2)`, ties away from zero is the alias that Java, Python, IEEE 754 and MATLAB `fi` own. Nothing in the diff contradicts it. Finding 1 is where the reader works against it.

Test gate: failed, on setup that helps; no tautological tests. Suite on the checkout: 118 passed, 4 ignored (its pack copy); `--ignored` fails all 4 as it should. Undo checks, one reader change at a time in a scratch copy, were each caught by their own targeted test and by `every_neighbour_reads_right_on_both_sides` (copula `!identity()` at `escapes.rs:107`, the `it` rule at `words.rs:126-131`, `names_a_rule_in_words` at `reading.rs:367`, the noun-phrase check in `denied_since`). The walks in `shape_tests.rs` sample outside words that line up with the feature the implementation tests, so they cannot find the defects below: `:177` uses only foreign owners (Java's, Python's, MATLAB's, IEEE's), never the canon's own authorities; `:267-279` uses rules with an article and qualities without one, never a rule without an article or a quality with one; `:146` uses `it` only for the mode, never for the subject itself; `:163-172` puts only verbs after a seat number and only list words after a count. `the_neighbours_hold_twenty` and `the_catalogue_holds_four...` only pin counts: weak, not tautological.

## Findings

1. High, regression introduced by the second commit (`c4d665d0` fired correctly). `words.rs:426-436` (now `reading/words.rs:163-177` in the clone) and `reading.rs:367`: `names_a_rule_in_words` treats any owner's name before "rule" or "rounding" as another rule, including the owners of the ruled mode itself. New reader silent, old reader fired, on each of these false sentences, all following "`half_up` is the ruled mode.": "Under op's rule a tie goes away from zero." "By the registry's rule a tie goes away from zero." "Per the panel's rule, a tie goes away from zero." "Under arvo's rounding a tie goes away from zero." Fix: only an owner the ruling names as owning the alias (the Java, Python, IEEE and MATLAB set already in `OTHER_NAMES`) takes the clause. Add these four as neighbour pairs.
2. High, regressions against `126d0790` still open, arrived in the first commit and not closed by the second. Old reader fired, new reader silent, both sentences false: "`half_up` sends ties up. 5 go away from zero." (`words.rs:405-407`: a bare number followed by a verb is read as a seat). "`half_up` is not the slow one, which is why it sends a tie away from zero." (`escapes.rs:184-210`: any article counts as a denied rule). Fix: catalogue both as pairs under their own tickets, and change the doc claim (finding 5). The probe table at `/tmp/halfup-r2/table.txt` lines 11 and 18 shows old fired, mid and new silent for these two.
3. Medium, wrong in both readers, and the closed lists' residues are not catalogued. `COUNTED`, `MOTIONS`, `COPULAS`, `PERSONAL`, the gerund rule, the `it` rule and the noun-phrase check have no catalogue entry and no ticket. They appear only in prose (commit body, REPORT.md:85). Each of these is wrong in old and new. Old fired on the true sentences; on the false ones old and new were both silent.
   - True sentences refused: "`half_up` is not symmetric rounding, which sends a tie away from zero." (also "commercial rounding", "round-half-away", "isn't symmetric rounding"); "`half_up` is the ruled mode. Seat 270 first read it as ties away from zero."; "... The crate, as it ships, sends a tie away from zero."; "... 270 in its first answer read ties away from zero."; "... 270 or 271 answered ties away from zero."; "`half_up` is floor(x + q/2), and ties-away is what Java calls it."
   - False sentences let through: "... computed by adding half and then rounding toward zero, which sends a tie away from zero."; "For `half_up`, away from zero is right rather than toward positive infinity."; "... sends negative ties away from zero instead of drifting toward zero."; "... Two of them go away from zero."
   - The `c4d665d0` reader got every one of these right except "drifting" and "Two of them", so the second commit moved them back to wrong.
   - Fix: one catalogue pair and ticket per list, as `catalogue-edge-cases-as-tests` requires.
4. Medium. The pronoun call is not one call. `words.rs:115-131` resolves an in-clause `it` to the mode past a denied rule, outside the question ticket. "`half_up` differs from `HALF_UP`, and every test shows it sends a tie away from zero." fires, so `it` is read as the mode. "`half_up` differs from `floor`, and it sends a tie away from zero." is silent (CATALOGUE[3]), so `it` is read as the rule named last. Across a stop, "`half_up` is not `HALF_UP`. Every test shows it sends..." is silent. So the docs at `half_up_is_not_a_magnitude_rule.rs:36-42` and `reading.rs:285-287`, which say the reader takes "the rule standing last in both", are false. `PERSONAL` in `given` (`reading.rs:111`) and the "they/them never the mode" rule are further pronoun calls. Removing `past_denied` restores the old reading of the plain in-clause case; it is not the only change to pronoun behaviour. Fix: put the `it` rule and `PERSONAL` under ticket `arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi`, or make them defer to the last-named rule until that ticket is settled.
5. Medium, docs. `half_up_is_not_a_magnitude_rule.rs:23-26` ("one refused-true entry", "Four cases are known") and the `SEVERITY` doc (`:135-141`, "refuses one true one") are false given findings 1 to 3. Rewrite both after cataloguing.
6. Low. The two job tickets (`..._gets_eighteen_...`, `..._ledger_holds_18_...`) have no closed file yet. The pronoun ticket's note still says "the one entry left".

## Checked and sound

- Every earlier finding (1 to 8) now reads right on its own sentence and on the neighbours the author added.
- Ledger: three rows, one per remaining id, all resolving in the tickets file at lines 981-1001. The pronoun ticket is a question ticket, and the in-clause entry was added to it.
- The `FIXME` at `reading.rs:296` names its ticket.
- Every file is at most 468 lines.
- No em-dashes, en-dashes, bold, curly quotes or TODO in added lines.
- Commit subjects are 64 and 62 characters and in form.
- `SEVERITY` is still `ADVISORY`, and `mockspace.toml` is unchanged.

Probe sentences and old, mid and new tables: `/tmp/halfup-r2/sents_mine1.rs`, `/tmp/halfup-r2/sents.rs`, `/tmp/halfup-r2/table.txt` (scratch only).

## Dispositions

Answered in `9436c5eb`.

| finding | disposition |
| --- | --- |
| 1 | fixed: owners limited to `ALIAS_OWNERS` (java, python, ieee, 754, matlab); the four sentences are neighbour pairs with twins, and the owner walk also runs the mode's own authorities |
| 2 | one fixed (an article alone no longer names a rule in `denied_since`, which also makes the three `rounding` sentences of finding 3 right), one catalogued ("5 go away from zero", own ticket, plus an ignored walk) |
| 3 | catalogued: twelve entries in `sentences/known.rs`, each with its twin, one ticket per list or shape; "Two of them go away from zero" did not reproduce, all three readers fire on it |
| 4 | the `it` rule and `PERSONAL` carry `FIXME(arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi)` and sit under that ticket; the docs state what the reader does |
| 5 | module doc and `SEVERITY` doc list sixteen known cases by class, seven let through and nine refused |
| 6 | the pronoun ticket was bumped with a note naming its entries; the two job tickets are the coordinator's to close |
