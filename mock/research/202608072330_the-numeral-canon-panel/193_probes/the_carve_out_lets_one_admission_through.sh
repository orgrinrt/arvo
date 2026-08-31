#!/usr/bin/env bash
# Two probes open their `control` with the same sentence. The gate catches one.
#
# `names_no_control` in `shape.rs` reads a `control` field beginning "None" as
# an admission unless the field somewhere reports a case that came out one way,
# the carve-out existing because "None of the arms disagreed, and that is the
# control firing" begins identically and means the opposite. The carve-out is a
# word list read over the whole field, and its own doc comment says an
# opening-word rule is blunt enough that a second reader is owed.
#
# This is that second read, and the rule loses. Both probes below open with
# "None was run as a case that had to fail." One of them later uses the word
# "reported" inside a hypothetical about what a different outcome would have
# meant, and that hypothetical is enough to clear the carve-out.
#
# The control on this control: the two `control` fields are printed whole, so a
# reader can see the shared opening rather than take it on trust.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"

for p in chain_accuracy_needs_an_intermediate_wider_than_the_operand_type \
         the_four_const_available_constructions_bind_at_four_times; do
  echo "######## probe::$p"
  "$here/row.sh" probe "$p" | sed -n '/^control = /,/^standing = /p' | sed '$d' | fold -s -w 96 | sed 's/^/  /'
  echo "  --- FIRED words present:"
  txt=$("$here/row.sh" probe "$p" | sed -n 's/^control = "\(.*\)"$/\1/p')
  for w in fired disagreed refused failed reported caught flagged; do
    case "$txt" in *"$w"*) echo "      $w" ;; esac
  done
  echo
done
cat <<'TXT'
Measured by writing each edge and running the gate, one at a time:

  chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type
    -> FAILED, kind = measurement-rests-on-an-uncontrolled-instrument

  the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times
    -> ok, 28 passed

Both probes stand at `sound` and both say in their first sentence that no case
that had to fail was run. The second is let through by the word `reported`,
which appears in "four constructions reported as binding at one time would say
the instrument cannot distinguish them": a sentence about what the instrument
would have shown had it come out differently, which is the opposite of a report
that something did.
TXT
