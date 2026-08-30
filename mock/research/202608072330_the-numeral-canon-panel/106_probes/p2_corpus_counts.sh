#!/bin/sh
# Independent verification of the corpus counts the strategy-axis unit rests on.
# Run from arvo/mock/benches. Every number here is cited in 106 sections 7 and 16.
#
# --exclude-dir=target is LOAD-BEARING and is the finding this probe carries as
# much as the counts are. Running the test gate first creates variants/*/target,
# and `outputs_may_differ` appears as a literal inside target/debug/**/*.rmeta,
# so a grep over variants/ taken AFTER the suite has run returns 133 files where
# the source has 1. Both numbers reproduce; only one is about the source.
# Same class as 102's own catch, where a grep matched its file's own sentence
# claiming a thing was absent.
set -e
G="grep -rl --exclude-dir=target"
echo "variant crates                       : $(ls -d variants/*/ | wc -l | tr -d ' ')"
echo "score_output impls                   : $($G 'fn score_output' variants/ 2>/dev/null | wc -l | tr -d ' ')"
echo "score_dimensions impls               : $($G 'fn score_dimensions' variants/ 2>/dev/null | wc -l | tr -d ' ')"
echo "max_relative_error impls             : $($G 'fn max_relative_error' variants/ 2>/dev/null | wc -l | tr -d ' ')"
echo "crates defining validate_output      : $($G 'fn validate_output' variants/ | sed 's|variants/\([^/]*\)/.*|\1|' | sort -u | wc -l | tr -d ' ')"
echo "files mentioning outputs_may_differ  : $($G 'outputs_may_differ' variants/ 2>/dev/null | wc -l | tr -d ' ')"
echo "committed CSVs                       : $(ls *.csv 2>/dev/null | wc -l | tr -d ' ')"
echo "committed meta files                 : $(ls *.meta.json 2>/dev/null | wc -l | tr -d ' ')"
echo
echo "--- the contamination, shown rather than asserted ---"
echo "without --exclude-dir=target, outputs_may_differ : $(grep -rl 'outputs_may_differ' variants/ 2>/dev/null | wc -l | tr -d ' ')"
