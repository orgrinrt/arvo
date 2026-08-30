#!/usr/bin/env nutshell
# Does every `quote` in mock/registry/ruling.toml appear verbatim in the panel
# file its `provenance` names?
#
# A row in that namespace claims op's authority. The claim is worth exactly the
# fidelity of the quotation, and nothing in the registry checks it: the schema
# checks that the citation resolves to a file, never that the file says this.
#
# What it does. For each row, take the `quote` block and the files named in
# `provenance`, strip the blockquote markers from the source, collapse every run
# of whitespace to one space on both sides, and test substring containment.
# Whitespace is collapsed because several rows quote one clause of a longer
# paragraph, so the line breaks differ while the words do not.
#
# The negative control runs first and must fail. A verifier that has never
# rejected anything has not been shown to work.
#
# Run from the repository root:
#     mock/research/202608072330_the-numeral-canon-panel/179_probes/verify_quotes.sh

use log

readonly REG="mock/registry/ruling.toml"
readonly PANEL="mock/research/202608072330_the-numeral-canon-panel"

if [[ ! -f "$REG" ]]; then
    log_error "run me from the repository root: $REG not found"
    exit 2
fi

# Collapse a file to one line, blockquote markers gone, whitespace squeezed.
normalise_file() {
    sed -e 's/^[[:space:]]*>[[:space:]]\{0,1\}//' "$1" | tr '\n' ' ' | tr -s '[:space:]' ' '
}

normalise_text() {
    tr '\n' ' ' | tr -s '[:space:]' ' ' | sed -e 's/^ //' -e 's/ $//'
}

# id <TAB> space-separated provenance files <TAB> normalised quote, one per row.
extract_rows() {
    awk '
        /^\[\[ruling\]\]/          { flush(); id=""; files=""; q=""; inq=0; next }
        /^id = /                   { id=$0; sub(/^id = "/,"",id); sub(/"$/,"",id); next }
        /^provenance = /           {
                                     line=$0
                                     while (match(line, /::[^:"]+::/)) {
                                         f=substr(line, RSTART+2, RLENGTH-4)
                                         if (f !~ /^202608072330/) files = files " " f
                                         line=substr(line, RSTART+RLENGTH-2)
                                     }
                                     next
                                   }
        /^quote = .../ && inq==0   { inq=1; next }
        inq==1 && /^...$/          { inq=0; next }
        inq==1                     { q = q " " $0; next }
        END                        { flush() }
        function flush() {
            if (id != "" && q != "") { gsub(/^ +| +$/,"",q); print id "\t" files "\t" q }
        }
    ' "$REG"
}

fail=0
checked=0

# --- negative control: this must be reported as a mismatch ------------------
control_src="$(normalise_file "$PANEL/39_op_the_strategy_set_is_not_closed.md")"
control_bad="the strategy set is closed at exactly four and nobody may reopen it"
if [[ "$control_src" == *"$control_bad"* ]]; then
    log_error "NEGATIVE CONTROL DID NOT FIRE: the verifier accepts text the source does not contain"
    exit 2
fi
log_info "negative control fired: a sentence the source does not contain is rejected"

# --- the real check ---------------------------------------------------------
while IFS=$'\t' read -r id files quote; do
    [[ -z "$id" ]] && continue
    checked=$((checked + 1))
    q="$(printf '%s' "$quote" | normalise_text)"
    hit=0
    for f in $files; do
        path="$PANEL/$f.md"
        [[ -f "$path" ]] || continue
        src="$(normalise_file "$path")"
        if [[ "$src" == *"$q"* ]]; then hit=1; break; fi
    done
    if [[ "$hit" -eq 0 ]]; then
        log_error "MISMATCH $id"
        log_error "  searched:$files"
        log_error "  quote:   ${q:0:140}..."
        fail=$((fail + 1))
    fi
done < <(extract_rows)

log_info "checked $checked quoted rows, $fail did not match their cited source"
[[ "$fail" -eq 0 ]] || exit 1
