#!/usr/bin/env bash
# Seat q31a. Q31's own `note` names the distinguisher:
#
#   "whether the canon ever says something true of a system arvo cannot host,
#    and it already does, since the bounded windows it admits are defined as
#    bounded windows of systems it cannot host."
#
# The row asserts it. This checks it, against the registry rather than against a
# member file, and asks specifically whether the sentences carrying it are
# ratified, because an unratified one loses to a ratified one and settles nothing.
#
# Read through `cargo mock query`, the harness's own reader of `canon_paths`,
# rather than by grepping the TOML, so a field this file names is a field the
# engine resolves.
#
# The negative control is the last section: the same instrument asked for a
# domain the canon does not name must come back empty, or its zeroes mean
# nothing.
set -uo pipefail
cd "$(dirname "$0")/../../.." || exit 1
q() { cargo mock query "$1" 2>/dev/null | grep -v '^ *Finished' | grep -v '^ *Compiling'; }

echo "=============================================================="
echo " 1. The ratification chain the argument rests on"
echo "=============================================================="
echo "ruling::the_format_spine_is_canon rung  : $(q 'ruling::the_format_spine_is_canon::rung')"
echo "what it ratifies:"
q 'ruling::the_format_spine_is_canon::ratifies'
echo

echo "=============================================================="
echo " 2. Does a ratified sentence quantify over an ambient domain?"
echo "=============================================================="
echo "-- proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation"
q 'proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation::says' | head -6
echo
echo "-- proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set"
q 'proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set::says' | head -6
echo

echo "=============================================================="
echo " 3. What values does the canon's own ambient_domain axis take?"
echo "=============================================================="
q 'dimension::ambient_domain::grammar'
echo
echo "-- and what a live committed predicate actually writes there:"
grep -n 'ambient domain in {' mock/registry/*.toml
echo

echo "=============================================================="
echo " 4. Is 'the rationals' a system arvo can host?"
echo "=============================================================="
echo "-- ruling::the_operating_constraints_are_intents_and_rules"
echo "   rung : $(q 'ruling::the_operating_constraints_are_intents_and_rules::rung')"
q 'ruling::the_operating_constraints_are_intents_and_rules::says' | head -6
echo
echo "   An exact rational needs unbounded terms, so a representable set equal to"
echo "   the rationals needs runtime growth. The row above forbids it and is in"
echo "   force. So the rationals are named by the canon and are not hostable."
echo

echo "=============================================================="
echo " 5. Q31 option one's stated cost, against the row above"
echo "=============================================================="
q 'question::one_word_or_two_for_is_a_number_system::options' | head -12
echo
echo "   Option one prices itself on 'unratified constraints'. Section 4 shows"
echo "   the constraints carry rung in_force, and the row's own words are that"
echo "   they are 'not to be questioned'. The price is stale."
echo

echo "=============================================================="
echo " 6. NEGATIVE CONTROL: a domain the canon does not name"
echo "=============================================================="
for d in "the surreals" "the hyperreals" "the p-adics"; do
  n=$(grep -ric "$d" mock/registry/ | awk -F: '{s+=$2} END{print s+0}')
  echo "  '$d' : $n hit(s) across the registry"
done
echo "-- POSITIVE CONTROL, the same instrument on a domain it should find:"
for d in "the rationals" "the reals"; do
  n=$(grep -ric "$d" mock/registry/ | awk -F: '{s+=$2} END{print s+0}')
  echo "  '$d' : $n hit(s) across the registry"
done
echo
echo "If the negative rows are zero and the positive rows are not, the zeroes in"
echo "this file are facts about the registry rather than about the grep."
