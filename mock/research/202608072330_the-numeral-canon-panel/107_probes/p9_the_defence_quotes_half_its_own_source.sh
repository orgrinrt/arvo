#!/usr/bin/env bash
# p9: 106 section 15 defends its 177 dropped anchors by citing 87. The sentence
# in 87 it cites has two halves and 106 carries one of them.
#
# Also: the precise predicate audit of 106 section 3.1, since the law bullet's
# missing `holds for:` is the file's most consequential single defect and an
# overstatement of how unusual it is would be a defect of mine.
set -u
cd "$(dirname "$0")/.." || exit 1
T=106_giesen_consolidation_the_strategy_axis.md

echo "=== 106's defence, verbatim ==="
sed -n '1232,1236p' "$T"

echo
echo "=== the sentence in 87 it rests on, whole ==="
sed -n '26,29p' 87_op_the_canon_is_written_once_at_the_end.md

echo
echo "-- the half 106 carries --"
echo '   "recoverable at the end from the file it came from"'
echo "-- the half of the same sentence 106 does not carry --"
echo '   "which is why the droplist and the compression checks matter and why a'
echo '    dropped item is a defect rather than a closed question"'
echo
echo "-- does any 'defect' in 106 refer to ITS OWN drops? --"
echo "   (checked by hand: every occurrence is listed, none is about its drops)"
grep -n -i 'defect' "$T" | sed 's/^/     /'
echo "   The one line matching both words is a predicate listing generator"
echo "   defect classes, which is a false positive of the naive grep."

echo
echo "-- and the provenance of that bullet: op's verbatim, or the coordinator's? --"
echo "   87 marks op's words with '>' blockquotes. The bullet is under"
echo "   'Two consequences worth stating', outside any blockquote:"
sed -n '24,25p' 87_op_the_canon_is_written_once_at_the_end.md
echo "   So BOTH halves are agent-authored. 106 may not take one and leave the other."

echo
echo "=== predicate audit of 106 section 3.1, stated exactly ==="
awk '/^### 3\.1/{f=1;n=0;next} /^### 3\.2/{f=0} f{n++; if(/^\*\*/) printf "  LEAD        %s\n", substr($0,1,66); if(/holds for/) printf "  PREDICATE   ^\n"}' "$T"
echo
echo "  Five bold leads. Two carry a predicate. Of the three that do not, one is"
echo "  a sub-note on the lead above it and one is a test count. The law bullet is"
echo "  the only claim about arvo's arithmetic in the section, and it is"
echo "  unpredicated, while 93's F1 which it compresses reads:"
grep -A2 '^\*\*F1\. ' 93_orchard_the_strategy_axis_derived_cold.md | tail -2
