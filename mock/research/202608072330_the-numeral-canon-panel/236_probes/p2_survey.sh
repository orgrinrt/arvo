#!/usr/bin/env nutshell
# What the registry says about the format topic's proposals, measured rather
# than read off.
#
# Every zero-returning check below carries a positive control immediately after
# it, because a zero from a pipeline is a claim about the pipeline until the
# pipeline has been shown able to return something else.
#
# Run from the panel directory's `236_probes`, or from anywhere: the registry
# path is resolved from this file's own location.

set -uo pipefail

# `$0` is the interpreter under a `#!/usr/bin/env nutshell` shebang, because
# nutshell sources the script rather than exec-ing it. Resolving the registry
# from `$0` therefore pointed every path at `~/.local/bin/../../../registry`,
# and check 5 reported all eight artifacts absent, its known-present arm
# included. `BASH_SOURCE` is the script under both invocations.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
REG="$HERE/../../../registry"
PANEL="$HERE/.."

[ -f "$REG/proposal.toml" ] || { echo "cannot find the registry at $REG" >&2; exit 2; }

echo "## 1. the format topic's proposal rows"
echo
printf '%-72s %-12s %-14s %s\n' "id" "standing" "sentence_kind" "distinct provenance files"
awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} /topic = "the_format"/{
    id=""; st=""; sk="";
    n=split($0, L, "\n");
    delete files;
    for (i=1;i<=n;i++) {
        if (L[i] ~ /^id = /)            { id=L[i]; sub(/^id = "/,"",id); sub(/"$/,"",id) }
        if (L[i] ~ /^standing = /)      { st=L[i]; sub(/^standing = "/,"",st); sub(/"$/,"",st) }
        if (L[i] ~ /^sentence_kind = /) { sk=L[i]; sub(/^sentence_kind = "/,"",sk); sub(/"$/,"",sk) }
        if (L[i] ~ /panel::.*::.*::/) {
            s=L[i]; sub(/^[^"]*"/,"",s); sub(/".*$/,"",s);
            k=split(s, P, "::"); if (k>=3) files[P[3]]=1
        }
    }
    c=0; for (x in files) c++;
    printf "%-72s %-12s %-14s %d\n", id, st, sk, c
}' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml

echo
echo "## 2. which of them a ratified ruling names under ratifies"
echo
awk 'BEGIN{RS="\\[\\[ruling\\]\\]"} /^ratifies = \[/ || /\nratifies = \[/ {
    id=""; rung=""; by=""; inr=0;
    n=split($0, L, "\n");
    for (i=1;i<=n;i++) {
        if (L[i] ~ /^id = /)           { id=L[i]; sub(/^id = "/,"",id); sub(/"$/,"",id) }
        if (L[i] ~ /^rung = /)         { rung=L[i]; sub(/^rung = "/,"",rung); sub(/"$/,"",rung) }
        if (L[i] ~ /^ratified_by = /)  { by=L[i]; sub(/^ratified_by = "/,"",by); sub(/"$/,"",by) }
    }
    # A `ratifies` array is written both ways in this file, one slug per line
    # and all on one line. Reading only the first shape made the one-line row
    # never close its block and spill every following field as a slug.
    for (i=1;i<=n;i++) {
        if (L[i] ~ /^ratifies = \[.*\]/) {
            s=L[i]; sub(/^ratifies = \[/,"",s); sub(/\].*$/,"",s);
            m=split(s, A, ",");
            for (j=1;j<=m;j++) { t=A[j]; gsub(/[ "]/,"",t);
                if (t != "") printf "%-56s <- %s (rung=%s, by=%s)\n", t, id, rung, (by==""?"unset":by) }
        }
        else if (L[i] ~ /^ratifies = \[/) inr=1;
        else if (inr && L[i] ~ /^\]/) inr=0;
        else if (inr) { s=L[i]; gsub(/[ ",]/,"",s); if (s != "") printf "%-56s <- %s (rung=%s, by=%s)\n", s, id, rung, (by==""?"unset":by) }
    }
}' "$REG"/ruling.toml

echo
echo "### control on check 2"
echo "Every line above must be a slug that resolves in the proposal namespace."
awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} NR>1{ n=split($0,L,"\n");
    for (i=1;i<=n;i++) if (L[i] ~ /^id = /) { s=L[i]; sub(/^id = "/,"",s); sub(/"$/,"",s); print s }
}' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml | sort -u > /tmp/236_all_proposals.txt
echo "proposal slugs in the registry: $(wc -l < /tmp/236_all_proposals.txt | tr -d ' ')"

echo
echo "## 3. multi-arrival standing resting on one cited file, whole registry"
echo "(the state a-standing-is-reachable-from-what-it-cites refuses; its"
echo " ceiling grandfathers the backlog, so the gate is green and the rows stand)"
echo
awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} NR>1{
    id=""; st=""; tp="";
    n=split($0, L, "\n"); delete files;
    for (i=1;i<=n;i++) {
        if (L[i] ~ /^id = /)       { id=L[i]; sub(/^id = "/,"",id); sub(/"$/,"",id) }
        if (L[i] ~ /^standing = /) { st=L[i]; sub(/^standing = "/,"",st); sub(/"$/,"",st) }
        if (L[i] ~ /^topic = /)    { tp=L[i]; sub(/^topic = "/,"",tp); sub(/"$/,"",tp) }
        if (L[i] ~ /panel::.*::.*::/) { s=L[i]; sub(/^[^"]*"/,"",s); sub(/".*$/,"",s);
            k=split(s, P, "::"); if (k>=3) files[P[3]]=1 }
    }
    if (st=="two_experts" || st=="three_or_more" || st=="cross_topic") {
        c=0; for (x in files) c++;
        if (c < 2) { printf "%-72s %-12s %s (files=%d)\n", id, st, tp, c; total++ }
    }
} END{ printf "\ncount: %d\n", total+0 }' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml

echo
echo "### control on check 3"
echo "A one_expert row must never appear above, whatever it cites, and a"
echo "two_experts row citing two files must not either. Both arms:"
awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} NR>1{
    id=""; st="";
    n=split($0, L, "\n"); delete files;
    for (i=1;i<=n;i++) {
        if (L[i] ~ /^id = /)       { id=L[i]; sub(/^id = "/,"",id); sub(/"$/,"",id) }
        if (L[i] ~ /^standing = /) { st=L[i]; sub(/^standing = "/,"",st); sub(/"$/,"",st) }
        if (L[i] ~ /panel::.*::.*::/) { s=L[i]; sub(/^[^"]*"/,"",s); sub(/".*$/,"",s);
            k=split(s, P, "::"); if (k>=3) files[P[3]]=1 }
    }
    c=0; for (x in files) c++;
    if (st=="one_expert") one++;
    if ((st=="two_experts"||st=="three_or_more"||st=="cross_topic") && c>=2) multi_ok++;
} END{
    printf "one_expert rows in the corpus (none of which check 3 may report): %d\n", one+0;
    printf "multi-arrival rows citing two or more files (the passing arm):    %d\n", multi_ok+0;
}' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml

echo
echo "## 4. imposed rows carrying a law edge and no evidence"
echo "(an-imposition-rests-on-no-instrument reads 'evidence' only, so a"
echo " normative row pointing at a measured law row walks past it)"
echo
awk 'BEGIN{RS="\\[\\[proposal\\]\\]"} NR>1{
    id=""; sk=""; law=0; ev=0; tp="";
    n=split($0, L, "\n");
    for (i=1;i<=n;i++) {
        if (L[i] ~ /^id = /)            { id=L[i]; sub(/^id = "/,"",id); sub(/"$/,"",id) }
        if (L[i] ~ /^sentence_kind = /) { sk=L[i]; sub(/^sentence_kind = "/,"",sk); sub(/"$/,"",sk) }
        if (L[i] ~ /^topic = /)         { tp=L[i]; sub(/^topic = "/,"",tp); sub(/"$/,"",tp) }
        if (L[i] ~ /^law = \[".*/)      law=1;
        if (L[i] ~ /^evidence = \[".*/) ev=1;
    }
    if ((sk=="normative" || sk=="definition") && law==1 && ev==0) {
        printf "%-72s %-11s %s\n", id, sk, tp; hit++
    }
    if ((sk=="normative" || sk=="definition") && ev==1) lintwould++;
} END{
    printf "\ncount through the hole: %d\n", hit+0;
    printf "count the lint itself would report (imposed + evidence): %d\n", lintwould+0;
}' "$REG"/proposal.toml "$REG"/proposal-the-later-topics.toml

echo
echo "### control on check 4"
echo "The second count is the lint's own predicate. The gate is green, so it"
echo "must be zero; if it is not, this script disagrees with the shipped lint"
echo "and this script is what is wrong."

echo
echo "## 5. do the cited evidence artifacts exist"
echo
for p in \
    55_probes/p3_encoding_is_a_separate_axis.rs \
    55_probes/p3_output.txt \
    56_probes/q2_affine_membership.rs \
    56_probes/q2_output.txt \
    56_probes/q3_signed_encoding_trade.rs \
    56_probes/q3_output.txt \
    56_probes/q1_two_law_families.rs \
    56_probes/this_file_does_not_exist.rs ; do
    if [ -f "$PANEL/$p" ]; then echo "PRESENT $p"; else echo "ABSENT  $p"; fi
done
echo
echo "### control on check 5"
echo "The last line must read ABSENT. A checker reporting every path present"
echo "has not looked at any of them."
