#!/usr/bin/env bash
# The 43 rows marked `normative` are the only ones the region check exempts.
# `shape.rs` says a normative sentence "imposes rather than establishes".
#
# A mechanical tell for one that establishes instead: its own `because` reports
# a measurement. A stipulation is adopted; it is not swept, counted or measured
# at a width. So a row whose justification cites a count, a percentage, a width
# or an exhaustive sweep is an established claim wearing the exempt mark.
#
# This does not settle any row. It ranks the 43 for a reader to judge, which is
# the most an instrument can do here.
#
# CASE THAT MUST FAIL: control 1 runs the identical detector over the 18
# `measured` rows, every one of which is by definition an established claim. If
# the detector does not fire on most of those, it does not detect measurement.
# Control 2 runs it over a string with no measurement vocabulary in it.
set -uo pipefail
cd "$(dirname "$0")/../../../registry"

# vocabulary that only a measurement uses
M='[0-9]+ of [0-9]+|[0-9]+\.[0-9]+%|[0-9]+%|exhaustive|sweep|swept|measured at|`W = |`W in |widths |triples|cells|of 254|of 256|per-crate|instrument|probe'

scan() { # $1 = sentence_kind to select
  awk -v want="$1" -v pat="$M" '
    function flush(){ if(k==want && id!=""){ hay=says" "because;
        n=gsub(pat,"&",hay); printf "%-4s %s\n", (n>0?"MEAS":"  . "), id } }
    /^\[\[proposal\]\]/{flush(); id="";k="";says="";because=""}
    /^id = /{s=$0;sub(/^id = "/,"",s);sub(/"$/,"",s);id=s}
    /^sentence_kind = /{s=$0;sub(/^sentence_kind = "/,"",s);sub(/"$/,"",s);k=s}
    /^says = /{says=$0}
    /^because = /{because=$0}
    END{flush()}' proposal.toml
}

echo "=== the 43 normative rows, flagged where their own says/because reports a measurement ==="
scan normative | sort
echo
echo "normative rows whose justification reports a measurement: $(scan normative | grep -c MEAS)"
echo "normative rows that read as pure imposition:              $(scan normative | grep -c '^  \. ')"

echo
echo "=== CONTROL 1: the same detector over the 18 rows already marked measured ==="
scan measured | sort
echo "fired on: $(scan measured | grep -c MEAS) of $(scan measured | wc -l | tr -d ' ')"

echo
echo "=== CONTROL 2: the detector on a string with no measurement vocabulary ==="
printf 'A format is identified by its ambient domain.\n' \
  | grep -cE "$M" || echo "0 hits, as required"
