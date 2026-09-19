# Running an arm or a variant and judging its outcome, and the rows of the
# table. Sourced by `run.sh`, which holds the state these read and write.

# flag <pass>: one unexpected outcome, counted against the pass that found it.
flag() {
  unexpected=$((unexpected + 1))
  case $1 in
  arm) F_arm=$((F_arm + 1)) ;;
  structure) F_structure=$((F_structure + 1)) ;;
  flip) F_flip=$((F_flip + 1)) ;;
  premise) F_premise=$((F_premise + 1)) ;;
  spelling) F_spelling=$((F_spelling + 1)) ;;
  esac
}

# judge <pass> <status> <expected> <log>: the outcome against the expectation,
# REQ holding every text a refusal has to contain and ABS every text the
# output must not.
judge() {
  local pass=$1 status=$2 expected=$3 log=$4 missing=0 present=0 ok=0 t
  for t in "${REQ[@]}"; do
    grep -qF -- "$t" "$log" || missing=1
  done
  for t in "${ABS[@]}"; do
    grep -qF -- "$t" "$log" && present=1
  done
  echo "exit ${status}"
  if [ "$present" -eq 0 ]; then
    if [ "$expected" = builds ] && [ "$status" -eq 0 ]; then
      ok=1
    elif [ "$expected" = refused ] && [ "$status" -ne 0 ] && [ "$missing" -eq 0 ]; then
      ok=1
    fi
  fi
  if [ "$ok" -eq 1 ]; then
    VERDICT=$expected
    if [ "$expected" = builds ]; then
      echo "verdict: as expected, builds"
    else
      echo "verdict: as expected, refused with:"
      for t in "${REQ[@]}"; do echo "  ${t}"; done
    fi
    [ "${#ABS[@]}" -eq 0 ] || echo "  and without:"
    for t in "${ABS[@]}"; do echo "    ${t}"; done
  else
    VERDICT="UNEXPECTED, exit ${status}"
    echo "verdict: UNEXPECTED, expected ${expected}"
    for t in "${REQ[@]}"; do echo "  with: ${t}"; done
    for t in "${ABS[@]}"; do echo "  without: ${t}"; done
    flag "$pass"
  fi
  echo
}

# run_arm <pass> <kind> <arm> <expected>: one arm or variant, single-file or
# manifest, under whichever of the two tools it takes, judged against REQ and
# ABS.
run_arm() {
  local pass=$1 kind=$2 arm=$3 expected=$4
  local log="${OUT_DIR}/${arm//\//_}.log" status name=${arm##*/}
  if [ "$kind" = file ]; then
    echo "== ${arm}.rs"
    rustc --edition 2024 --crate-type lib --crate-name "${name%%.*}" --emit=metadata \
      --out-dir "$OUT_DIR" "${arm}.rs" </dev/null >"$log" 2>&1
  else
    echo "== manifest_rename/${arm}"
    (cd "manifest_rename/${arm}" && cargo check --quiet --locked) </dev/null >"$log" 2>&1
  fi
  status=$?
  cat "$log"
  judge "$pass" "$status" "$expected" "$log"
}

# unjudgeable <pass> <what is wrong> <row: arm> <row: what> <row: expected>:
# one reason an arm or a pass cannot be judged, flagged once.
unjudgeable() {
  echo "== ${2}"
  echo "verdict: UNEXPECTED"
  echo
  flag "$1"
  row "$3" "$4" "$5" "UNEXPECTED"
}

row() {
  ROWS+=("| $1 | $2 | $3 | $4 |")
}

# Code span that survives a backtick inside it, and a `|`, which would
# otherwise end the table cell.
code() {
  printf '`` %s ``' "${1//|/\\|}"
}

# list_text <items>: `1` as `item 1`, `1,2` as `items 1, 2`.
list_text() {
  case $1 in
  *,*) printf 'items %s' "${1//,/, }" ;;
  *) printf 'item %s' "$1" ;;
  esac
}

write_table() {
  {
    echo "<!-- Written by run.sh in the same run as run.out. Not edited by hand. -->"
    echo
    echo "| arm | what it checks | expected | observed |"
    echo "| --- | --- | --- | --- |"
    printf '%s\n' "${ROWS[@]}"
    echo
    echo "unexpected outcomes: ${unexpected}"
  } >arms.md
}
