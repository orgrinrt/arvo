#!/usr/bin/env bash
# Swaps the four mutant copies in, runs every mutant and the control, writes
# out/runs.txt, and puts the shipped files back, touched so the next build is
# not the mutant one. Run from the repository root.
set -euo pipefail

here=mock/research/sketches/202609241413_the-half-up-reader-attribution-mutants
lint=mock/lints/half_up_is_not_a_magnitude_rule
history=mock/lints/no_project_history_in_published_prose.rs
manifest=mock/target/mockspace-lints/Cargo.toml
keep=$(mktemp -d)

shipped=("$lint/reading.rs" "$lint/reading/cutting.rs" "$lint/reading/escapes.rs" "$history")
mutants=("$here/reading.mutant.rs.txt" "$here/cutting.mutant.rs.txt" "$here/escapes.mutant.rs.txt" \
    "$here/no_project_history_in_published_prose.mutant.rs.txt")

restore() {
    for i in "${!shipped[@]}"; do
        cp "$keep/$i" "${shipped[$i]}"
        touch "${shipped[$i]}"
    done
}

for i in "${!shipped[@]}"; do
    cp "${shipped[$i]}" "$keep/$i"
    cp "${mutants[$i]}" "${shipped[$i]}"
    touch "${shipped[$i]}"
done
trap restore EXIT

reader=(none backtick noother nocarry nopass pronounonly pronounany codespan nocap nomask nodenote
    readafter nameafter nobetween betweenboth noconj twoseg nobare neighbour noqual nomention
    mentionspan noattrib noattribclause noattribrow segmine segnamed segback lastany negatedante
    relonly pronounsonly noconjskip nodenoteconj denotewhole emptyneighbour mentionreading)

out=$here/out/runs.txt
: > "$out"
for m in "${reader[@]}"; do
    echo "== $m" >> "$out"
    HALF_UP_MUTANT=$m cargo test --manifest-path "$manifest" half_up 2>&1 \
        | grep -E '\.\.\. FAILED|^test result' >> "$out" || true
done
for m in none unboundedphrases; do
    echo "== $m, filter no_project_history" >> "$out"
    HALF_UP_MUTANT=$m cargo test --manifest-path "$manifest" no_project_history 2>&1 \
        | grep -E '\.\.\. FAILED|^test result' >> "$out" || true
done
