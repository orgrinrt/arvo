#!/usr/bin/env bash
# Extracts the passages seat 269 cites from the documents it fetched, so the
# quotations in 269 are checkable from this repository. The documents are in
# the workspace store, `.data/fetched/<sha256>.html`, fetched through
# `.shared/scripts/fetch/fetch --store`; the hash is the file's sha256.
#
# Run from the workspace root: bash <this file> > <this dir>/sources.out
set -u
store=.data/fetched
text() { sed -e 's/<[^>]*>/\n/g' "$store/$1.html" | grep -v '^[[:space:]]*$' | tr -s ' '; }
show() { # hash, url, pattern, lines after
    printf '\n=== %s\n    %s\n' "$2" "$1"
    [ -f "$store/$1.html" ] || { echo "MISSING"; return; }
    text "$1" | grep -n -i -A"$4" -- "$3" | head -40
}
absent() { # hash, phrase: prints the count, which is the claim
    printf '  count of "%s" in %s: ' "$2" "$1"
    grep -c -- "$2" "$store/$1.html"
}

FIMATH=c3201caf126426b5bdbdfaf6209a4fbb5defe43c5abde902cef87c4bcf07dfd5
NEAREST=8fed35442f8089dc1853ea33ed1e1c622d6100a38d244c574d465690dca364f4
ROUND=b0ffda2a3bb28bac7a7f47fa2510d45c1566468a25bb70a22c35dbb65175cb9b
IEEE_WP=0bd303ba9f316ed279c1e6632a977e729e6dfc61857cf5ef77b96d675f4bb193
PYDEC=2250296fa1776a289b866d9d315d1a28cad1452b08801553af6d13f1b841b17a
JAVA_RM=dc2b8fa7078c0581090bb1a0e485666657cccc7657fc05c91b94803f82bdf6b6
ROUNDING_WP=601151a2cc03878bfd59c07c06b814efcbf0a6c3cac35b9cb9b29943d11b411f

show $FIMATH https://www.mathworks.com/help/fixedpoint/ref/embedded.fimath.html 'Rounding method to use, specified' 17
show $FIMATH 'fimath default display' 'RoundingMethod: Nearest' 1
echo '  seat 267 quotes this page; its wording, tested:'
absent $FIMATH 'greater absolute value'
absent $FIMATH 'closest representable'
absent $FIMATH 'rounds ties to the nearest integer'
show $NEAREST https://www.mathworks.com/help/fixedpoint/ref/embedded.fi.nearest.html 'differ in the way they treat' 12
show $ROUND https://www.mathworks.com/help/fixedpoint/ref/embedded.fi.round.html 'rounds values to the nearest integer with greater' 3
echo '  the one phrase of 267 found nowhere fetched:'
absent $NEAREST 'closest representable'
absent $ROUND 'closest representable'
show $IEEE_WP 'https://en.wikipedia.org/wiki/IEEE_754 (secondary; the standard is not open access)' 'The standard defines five rounding rules' 14
show $IEEE_WP 'same' 'only required for decimal' 0
show $PYDEC https://docs.python.org/3/library/decimal.html 'ties going away from zero' 0
show $JAVA_RM https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/math/RoundingMode.html '^HALF_UP$' 2
show $JAVA_RM 'same' 'Rounding mode to round away from zero. Always' 0
text $JAVA_RM | tr '\n' ' ' | grep -o 'This mode corresponds to the IEEE 754 rounding-direction  attribute roundTiesToAway. Example: Rounding mode HALF_UP Examples.\{0,200\}' | head -1
show $ROUNDING_WP https://en.wikipedia.org/wiki/Rounding 'round half toward positive infinity$' 3
text $ROUNDING_WP | tr '\n' ' ' | grep -o 'Some programming languages (such as Java and Python) use "half up" to refer to  round half away from zero  rather than  round half toward positive infinity' | head -1
