#!/usr/bin/env bash
# p4. Is the subject of each seed talking point anywhere in the registry?
#
# The registry cites SEED_TALKING_POINTS nowhere, which is established and is not
# what this asks. This asks the different and more useful question: did the
# CURRENT panel reach the same ground by its own route? A subject the registry
# already holds is not a loss whatever the seed says about it, and a subject it
# does not hold is a candidate only after the search has been shown to work.
#
# THE CASE THAT MUST FAIL, stated before the run. A grep that finds nothing
# proves nothing about the registry until it has been shown able to find
# something, and the closed panel's own sweep recorded six occasions where a
# first vocabulary returned zero and a second found the idea. So:
#   CARRIED  subjects known to be in the registry. Every one MUST return non-zero.
#            If any returns zero the vocabulary is broken and every ABSENT below
#            is an artifact of my spelling rather than a fact about the registry.
#   NONSENSE a term that cannot be there. MUST return zero. If it returns
#            non-zero the matcher is matching something other than what I typed.
#
# Every subject is searched on two or three INDEPENDENT phrasings, not synonyms
# of one word, because the failure being guarded against is a single vocabulary
# missing a present idea.

set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
reg="$(cd "$here/../../../registry" && pwd)"

hits() { grep -ihoE "$1" "$reg"/*.toml 2>/dev/null | wc -l | tr -d ' '; }

row() {
    local id="$1" label="$2"; shift 2
    local total=0 per="" n
    for pat in "$@"; do
        n=$(hits "$pat")
        total=$((total + n))
        per="$per $n"
    done
    local verdict
    if [ "$total" -eq 0 ]; then verdict="ABSENT"; else verdict="present"; fi
    printf '%-6s %-42s %-8s %s\n' "$id" "$label" "$verdict" "($(echo "$per" | tr -s ' '))"
}

echo "== p4. registry coverage of the seed's sections 2-8, by subject =="
echo "registry: $reg  ($(ls "$reg"/*.toml | wc -l | tr -d ' ') files)"
echo "each subject searched on independent phrasings; counts per phrasing in ()"
echo

echo "-- CONTROLS: these must be present, or the sweep means nothing --"
row "c1" "headroom (known carried)"            "headroom"
row "c2" "interior safety (known carried)"     "interior safet" "accumulator"
row "c3" "bitpack contention (known carried)"  "bitpack" "contention"
row "c4" "strategy presets (known carried)"    "preset" "Warm"
echo
echo "-- CONTROL: this must be absent --"
row "c5" "nonsense term"                       "zzqqxx_not_a_word"
echo

echo "-- 3.1 the width surface and the container derivation --"
row "T1"  "structural width-to-container ladder" "total ladder" "structural keying" "D0<T>|D1<T>|little-endian binary type"
row "T2"  "structurally derived backing array"   "structural array" "backing array" "layout-identical"
row "T3"  "const-to-type bridge table"           "bridge table" "macro_rules" "const parameter.{0,20}case split"
row "T4"  "binding time and the table's real job" "binding time" "Capacity precedent" "decimal notation converts"
row "T5"  "generic_const_args vetting"           "generic_const_args" "next-solver"
row "T6"  "diagnostics: bound not equality"      "on_unimplemented" "E0277|E0308" "bound rather than.{0,20}equality"
row "T7"  "erasure vs a competent author"        "erasure" "competent author" "one limb|one-limb"
echo

echo "-- 3.2 the strategy axis --"
row "T8"  "one preset name, two rows, exponent form" "exponent form" "two rows|per number kind" "E0119"
row "T9"  "the headroom thread"                  "headroom" "warm-container|warm-clamp"
row "T10" "only Hot folds signed"                "signed fold" "ℤ/2|Z/2|[^a-z]ring[^a-z]" "only.{0,20}Hot"
row "T11" "the strategy door and the environment" "FPCR" "flush-to-zero|subnormal" "strategy door"
row "T12" "preset divergence mechanism"          "generic parameter default" "projecting off|parent preset"
echo

echo "-- 3.3 quantiser, laws, folds --"
row "T13" "round first, classify second"         "round first" "classify" "quantiser order"
row "T14" "overflow band closed form"            "overflow band" "lattice clause|reachability clause"
row "T15" "finest view / grade monoid"           "finest view" "grade monoid|view homomorphism" "Kleene"
row "T16" "TotalOrd split precondition"          "totalOrder|TotalOrd" "canonicalise" "bit-comparator|bit comparator"
row "T17" "closure laws, bias and adjustment"    "AddClosed" "bias.{0,20}integer|adjustment divides" "mul_full"
row "T18" "interior vs total safety, quire"      "interior safet" "quire" "fold_compensated"
row "T19" "site count vs moved count, flags"     "site count" "moved count" "fetestexcept"
row "T20" "algo crates wrong under their presets" "upward_rank" "Monotone" "foldnum"
row "T21" "parse is the quantiser, print"        "round-to-odd|ToOdd" "shortest correctly-rounded" "rational-of-digits"
echo

echo "-- 3.4 order, conversion, the family question --"
row "T22" "inclusion order, four conditions"     "grid refinement" "phase alignment" "inclusion order"
row "T23" "the cardinality antichain"            "antichain" "equal.{0,20}cardinality"
row "T24" "lattice dissolution, the 81 vs 0"     "within-family|within family" "join failure" "cross-family|cross family"
row "T25" "conversion needs an adjudicating key" "adjudicat" "narrowing" "which strategy.{0,20}law"
row "T26" "From coherence walls"                 "reflexive impl" "TryFrom" "by-reference From|coherence wall"
row "T27" "membership, m dot r^q, Ostrowski"     "Ostrowski" "finest inhabited" "embedding signature"
echo

echo "-- 3.5 crossing, encoding, storage --"
row "T28" "the crossing contract and the leak"   "decode|encode" "E4M3" "Crosses"
row "T29" "three width levels, bitpacked group"  "stored width" "write granule" "gcd|period P"
row "T30" "digest, mutation perimeter, niches"   "digest" "mutation perimeter|byte-owner" "niche"
row "T31" "byte image, same-process"             "byte-image|byte image" "portability"
echo

echo "-- 3.6 operations --"
row "T32" "division as a failure classifier"     "div_or" "solution set|solution-set" "x/0|divide by zero|division by zero"
row "T33" "elementary functions, three classes"  "Lindemann" "hardness constant" "correctly rounded root|residue pair"
row "T34" "radix ten, cohorts, preferred exponent" "cohort" "preferred exponent" "non-canonical code"
echo

echo "-- 3.7 the peripheral subjects --"
row "T35" "the truth contract"                   "Boolean algebra" "lane mask|lane-wise|n-lane" "coordinate projection"
row "T36" "the bitfield"                         "bitfield" "placement map" "overlap"
row "T37" "capacity and the array grammar"       "Capacity" "last-index|predecessor" "AGREES"
row "T38" "shape and geometry"                   "rotor" "index-domain|index domain" "closed interval|purely fractional"
row "T39" "the platform crate and Bool's doors"  "naming door|six doors" "inline\\(always\\)" "route multiplicity"
row "T40" "the notation vehicle"                 "proc-macro|proc macro" "decimal literal" "swapped-argument|two entry points"
row "T41" "the cost model"                       "composition pricing|cost model" "dyadic" "Reduce"
row "T42" "the environment parameter"            "environment fact" "assumption, never a witness|cannot-check|cannot-provide" "FZ16"
echo

echo "-- 4, 5, 6: refuted, abandoned, blind spots --"
row "T43" "ratified-then-reversed as a category" "ratified-then-reversed|reversed" "canonicity"
row "T44" "claims that died of provenance"       "tree-meaning" "existed nowhere|five probe"
row "T45" "the uncommitted-bench era"            "gitignore" "bench artifact.{0,30}discard"
row "T46" "refutations of the archive instruments" "false negative|false-negative" "model-inadequacy|too narrow to see"
row "T47" "the three-relation ladder"            "weak.{0,10}Kleene.{0,10}graded|three-relation" "partial associativity"
row "T48" "Growth and Widening as axes"          "Widening" "Growth"
row "T49" "LogicalWidth and the ten-axis table"  "LogicalWidth" "ten-axis|ten axis"
row "T50" "the single Sign axis split"           "SignDomain" "SignIndexing" "SystemC"
row "T51" "the step-A/step-B seam"               "step-A|step A/step B|seam"
row "T52" "ambient and realisation"              "ambient" "realise|realisation"
row "T53" "the 29-item queue at close"           "twenty-nine|29-item" "W_MAX"
row "T54" "the owed-artifact list"               "owed artifact|Boolean-algebra law suite" "uqadd|carry-chain"
row "T55" "three instruments named, never run"   "restoration ledger" "member-file sweep|ninety-nine"
row "T56" "threads A, B, C"                      "ConstantTime" "sticky flag|absorbing bottom" "leaf truth"
row "T57" "questions never asked"                "strided" "one Apple M1|single target|other target"
echo

echo "-- 2. material bearing on the two questions in flight --"
row "S2.1" "the verb validate"                   "validate" "erasure"
row "S2.2" "the long-standing constraints"       "no_std" "unstable-features|forbidden-feature"

echo
echo "== how to read this =="
echo "ABSENT is the reliable verdict: the vocabulary appears nowhere across 12"
echo "registry files on 2-3 independent phrasings, with four carried controls"
echo "returning non-zero in the same run. It means nobody has written the subject"
echo "down, and it is a finding."
echo
echo "'present' is NOT a claim that the registry carries the talking point. It"
echo "says a word appears. The registry reached these subjects by its own route"
echo "and may hold a different claim about them, a narrower one, or the opposite"
echo "one. Every 'present' row is a pointer to go and read, never a verdict that"
echo "the material is carried. The catalogue beside this file records which of"
echo "them was actually opened and what was found."
