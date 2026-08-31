#!/usr/bin/env bash
# Does a `measured` proposal with no `evidence` actually fail the committed checks,
# and does a probe reference that names no row fail too?
#
# The question this settles: the brief says do not fill `evidence`, because no
# `probe` row exists. The schema says `evidence` is "required in spirit for a
# `measured` row and checked as such in mock/checks". If both are true, no
# `measured` row can be written at all right now, which is a blocker rather than
# a preference, and it is settled by planting rather than by reading the source.
#
# Three arms, each with the outcome that must occur written before the run:
#   A. measured + no evidence          MUST be reported (measured-claim-cites-no-probe)
#   B. measured + evidence naming no probe row  MUST be reported (unknown-row-reference)
#   C. argument + no evidence          MUST NOT be reported  (the negative control:
#                                      an arm reporting this too is reporting the
#                                      namespace rather than a breach)
set -euo pipefail
cd "$(dirname "$0")/../../../.."   # repo root

PLANT=mock/registry/zzz_probe_183_plant.toml
cleanup() { rm -f "$PLANT"; }
trap cleanup EXIT

cat > "$PLANT" <<'TOML'
[[proposal]]
id = "probe_183_a_measured_with_no_evidence"
kind = "finding"
sentence_kind = "measured"
standing = "one_expert"
topic = "the_chain"
says = "arm A"
because = "arm A"
predicate = ["threads: 1"]
provenance = ["panel::202608072330_the-numeral-canon-panel::173_leroy_the_canon_candidate_for_the_chain::521"]
keywords = ["probe", "arm a"]

[[proposal]]
id = "probe_183_b_measured_with_a_dangling_probe"
kind = "finding"
sentence_kind = "measured"
standing = "one_expert"
topic = "the_chain"
says = "arm B"
because = "arm B"
predicate = ["threads: 1"]
evidence = ["no_such_probe_row_exists"]
provenance = ["panel::202608072330_the-numeral-canon-panel::173_leroy_the_canon_candidate_for_the_chain::521"]
keywords = ["probe", "arm b"]

[[proposal]]
id = "probe_183_c_argument_with_no_evidence"
kind = "finding"
sentence_kind = "argument"
standing = "one_expert"
topic = "the_chain"
says = "arm C, the negative control"
because = "arm C"
predicate = ["threads: 1"]
provenance = ["panel::202608072330_the-numeral-canon-panel::173_leroy_the_canon_candidate_for_the_chain::521"]
keywords = ["probe", "arm c"]
TOML

echo "### planted rows: $(grep -c '^\[\[proposal\]\]' "$PLANT")"
echo
echo "### cargo test -p arvo-checks (expect: A and B reported, C silent)"
( cd mock && cargo test -p arvo-checks 2>&1 ) | grep -E 'probe_183|measured-claim-cites-no-probe|test result: FAILED|failures:' || true
echo
echo "### cargo mock --lint-only (expect: unknown-row-reference on arm B)"
( cargo mock --lint-only 2>&1 ) | grep -iE 'probe_183|unknown-row-reference|no_such_probe_row' || true
