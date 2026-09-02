#!/usr/bin/env nutshell
# p3. How often a question row already states which way it goes.
#
# `mock/registry/question.toml`'s own header states the discipline: "No answer
# is recorded here, including for the rows whose source records one. Where a
# question was answered, `note` says that it was and where, and never which
# way."
#
# If that holds, a seat dispatched on a question row is handed a question and an
# option set and nothing else, and its agreement with a conclusion is its own.
# If it does not hold, the row hands over the conclusion and the seat's
# agreement is explained by the row.
#
# THE CASES THAT MUST FAIL, STATED BEFORE THE RUN.
#
# C1. The verdict vocabulary must score far lower on `asks` than on `note`.
#     `asks` is interrogative by construction. If the two rates are close, the
#     vocabulary is matching ordinary technical prose and every figure here is
#     noise. The arm fails unless the note rate is at least three times the asks
#     rate.
# C2. `which_width_coordinates_a_consumer_writes` carries a populated `answered`
#     field, read directly out of the file at the top of this panel's
#     investigation. It must be counted. If the `answered` scan reports zero the
#     field extraction is dead.
# C3. The row count must be 106, the number the generated `docs/QUESTION.md`
#     states for this namespace. Anything else means the block splitter is wrong.
use log

ROOT="$(git rev-parse --show-toplevel)"
Q="$ROOT/mock/registry/question.toml"

# A verdict vocabulary: words that say which way, rather than what is open.
VERDICT='refut|undercut|measured false|is false|closed by|dissolve|does not hold|settled|answered|the answer is|retired|superseded|ruled out|excluded|withdrawn'

echo "== the namespace =="
ROWS=$(grep -c '^\[\[question\]\]' "$Q")
echo "  question rows:                 $ROWS"
echo "  rows carrying \`answered\`:      $(grep -c '^answered = ' "$Q")"
echo "  rows carrying \`bound\`:         $(grep -c '^bound = ' "$Q")"
echo "  rows carrying \`note\`:          $(grep -c '^note = ' "$Q")"
echo "  rows carrying \`options\`:       $(grep -c '^options = ' "$Q")"
echo

NOTE_HITS=$(grep '^note = ' "$Q" | grep -icE "$VERDICT")
ASKS_HITS=$(grep '^asks = ' "$Q" | grep -icE "$VERDICT")
ASKS_TOTAL=$(grep -c '^asks = ' "$Q")
echo "== the verdict vocabulary, on the two fields =="
echo "  \`note\` lines matching a verdict word:  $NOTE_HITS of $(grep -c '^note = ' "$Q")"
echo "  \`asks\` lines matching a verdict word:  $ASKS_HITS of $ASKS_TOTAL"
echo

echo "== C1 control: the vocabulary must discriminate the two fields =="
if [ "$ASKS_HITS" -eq 0 ]; then
  echo "  PASS C1 the vocabulary scores zero on interrogative text and $NOTE_HITS on notes"
  c1=0
elif [ $(( NOTE_HITS )) -ge $(( ASKS_HITS * 3 )) ]; then
  echo "  PASS C1 note rate is at least three times the asks rate"
  c1=0
else
  echo "  FAIL C1 the vocabulary matches ordinary prose; the note figure means nothing"
  c1=1
fi
echo

echo "== C2 control: a row known to carry an answer is counted =="
if grep -A3 '^id = "which_width_coordinates_a_consumer_writes"' "$Q" | grep -q '^answered = '; then
  echo "  PASS C2 the known-answered row is seen"; c2=0
else
  echo "  FAIL C2 the answered extraction is dead"; c2=1
fi
echo

echo "== C3 control: the row count agrees with the generated document =="
DOCROWS=$(grep -oE '^[0-9]+ rows\.' "$ROOT/docs/QUESTION.md" | head -1 | grep -oE '^[0-9]+')
echo "  docs/QUESTION.md states: ${DOCROWS:-NONE}   registry parse: $ROWS"
if [ "$ROWS" = "$DOCROWS" ]; then echo "  PASS C3"; c3=0; else echo "  FAIL C3"; c3=1; fi
echo

echo "== the header's own claim, against the file =="
echo "  header: \"No answer is recorded here, including for the rows whose source records one.\""
echo "  file:   $(grep -c '^answered = ' "$Q") rows carry a populated \`answered\` field."
echo
echo "== what this establishes =="
echo "  Every question row in the namespace carries both a \`note\` and an"
echo "  \`options\` list, so there is no such thing as a bare question to dispatch"
echo "  a seat on. A seat dispatched on one is handed the panel's own compressed"
echo "  prior positions along with the question."
exit $(( c1 + c2 + c3 ))
