#!/usr/bin/env bash
# Can the citation machinery fail? Four arms, each outcome written before the run.
#
#   A. a line citation into a numbered member file      MUST pass  (negative control)
#   B. a citation naming a file that is not there       MUST be reported
#   C. a heading anchor naming no heading               MUST be reported
#   D. a line citation into a living ledger             MUST be reported
#
# Arm A is the one that matters: an instrument where every arm is red proves only
# that it reports everything. Arm D is the guard `mockspace.toml:313-317` says is
# "checked rather than declared", and two prior ports found it unbuilt; it exists
# now at mock/checks/tests/no_line_citation_into_a_living_ledger.rs and this run
# is a third party confirming it fires.
set -euo pipefail
cd "$(dirname "$0")/../../../.."   # repo root

PLANT=mock/registry/zzz_probe_183_cites.toml
cleanup() { rm -f "$PLANT"; }
trap cleanup EXIT

P=panel::202608072330_the-numeral-canon-panel

cat > "$PLANT" <<TOML
[[proposal]]
id = "probe_183_cite_a_line_into_a_member_file"
kind = "finding"
sentence_kind = "argument"
standing = "one_expert"
topic = "the_chain"
says = "arm A"
because = "arm A"
predicate = ["threads: 1"]
provenance = ["${P}::173_leroy_the_canon_candidate_for_the_chain::521"]
keywords = ["probe"]

[[proposal]]
id = "probe_183_cite_b_no_such_file"
kind = "finding"
sentence_kind = "argument"
standing = "one_expert"
topic = "the_chain"
says = "arm B"
because = "arm B"
predicate = ["threads: 1"]
provenance = ["${P}::999_no_such_file_anywhere::1"]
keywords = ["probe"]

[[proposal]]
id = "probe_183_cite_c_no_such_heading"
kind = "finding"
sentence_kind = "argument"
standing = "one_expert"
topic = "the_chain"
says = "arm C"
because = "arm C"
predicate = ["threads: 1"]
provenance = ["${P}::173_leroy_the_canon_candidate_for_the_chain::#there-is-no-such-heading"]
keywords = ["probe"]

[[proposal]]
id = "probe_183_cite_d_line_into_a_living_ledger"
kind = "finding"
sentence_kind = "argument"
standing = "one_expert"
topic = "the_chain"
says = "arm D"
because = "arm D"
predicate = ["threads: 1"]
provenance = ["${P}::AGREEMENTS::754"]
keywords = ["probe"]
TOML

echo "### cargo mock --lint-only (expect B and C reported, A silent)"
( cargo mock --lint-only 2>&1 ) | grep -iE 'probe_183_cite' || echo "(nothing reported)"
echo
echo "### cargo test -p arvo-checks (expect D reported by the living-ledger arm)"
( cd mock && cargo test -p arvo-checks 2>&1 ) | grep -iE 'probe_183_cite|AGREEMENTS|test result: FAILED' || echo "(nothing reported)"
