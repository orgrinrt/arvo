# One arm and its passes, and the self-check over them. Sourced by `run.sh`,
# whose header says what each pass expects and shows.

# check <arm> <expected> [--reads <i,j>] [--resists <k>] [--checks-nothing]
# [<text>...]: one single-file arm, and its passes.
check() {
  local arm=$1
  shift
  arm_and_passes file "$arm" "${arm}.rs" "$@"
}

# check_manifest <crate> <expected> [...]: one fixture under manifest_rename/,
# and its passes.
check_manifest() {
  local crate=$1
  shift
  arm_and_passes manifest "$crate" "manifest_rename/${crate}/src/lib.rs" "$@"
}

arm_and_passes() {
  local kind=$1 arm=$2 src=$3 expected=$4
  shift 4
  local reads= resists= nothing=0
  while [ $# -gt 0 ]; do
    case $1 in
    --reads) reads=$2; shift 2 ;;
    --resists) resists=$2; shift 2 ;;
    --checks-nothing) nothing=1; shift ;;
    *) break ;;
    esac
  done
  ARM_TEXTS=("$@")
  local shown checks expect_text
  if [ "$kind" = file ]; then shown="${arm}.rs"; else shown="manifest_rename/${arm}"; fi
  checks=$(describe "$src" "$reads" "$resists")

  REQ=("${ARM_TEXTS[@]}")
  ABS=()
  run_arm arm "$kind" "$arm" "$expected"
  if [ "$expected" = refused ]; then
    expect_text="refused with $(printf '%s; ' "${ARM_TEXTS[@]}")"
    expect_text="${expect_text%; }"
  elif [ "$nothing" -eq 1 ]; then
    expect_text="builds, and checks no value"
  else
    expect_text="builds"
  fi
  row "$(code "$shown")" "$checks" "$(code "$expect_text")" "${VERDICT}"

  if [ "$expected" = builds ]; then
    if [ "$nothing" -eq 1 ]; then
      load_items "$src"
      [ "$N_ITEMS" -eq 0 ] || unjudgeable structure \
        "${shown}: declared to check nothing, and holds ${N_ITEMS} item(s)" \
        "$(code "$shown")" "${N_ITEMS} item(s)" "no item"
      return 0
    fi
    if ! judgeable "$shown" "$src" "$reads" "$resists"; then
      echo "-- ${shown}: passes skipped, since the arm cannot be judged"
      echo
      return 0
    fi
    flip_pass "$kind" "$arm" "$src" "$shown"
  fi
  premise_pass "$kind" "$arm" "$src" "$shown" "$expected" "$reads" "$resists"
  if [ "$expected" = builds ] && [ -n "$resists" ]; then
    spelling_pass "$kind" "$arm" "$src" "$shown" "$reads" "$resists"
  fi
  return 0
}

# describe <source> <reads> <resists>: what one source file checks, as the
# table says it: the items with a comparison, counted by shape, and which the
# arm declares reading the fake width and reading past it.
describe() {
  local i asserts=0 arrays=0 out=
  load_items "$1"
  for ((i = 1; i <= N_ITEMS; i++)); do
    [ "${I_LINE[$i]}" -gt 0 ] || continue
    if [ "${I_SHAPE[$i]}" = "const assert" ]; then asserts=$((asserts + 1)); else arrays=$((arrays + 1)); fi
  done
  [ "$asserts" -gt 0 ] && out="${asserts} const assert"
  [ "$arrays" -gt 0 ] && out="${out:+${out}, }${arrays} array length"
  out=${out:-none}
  [ -n "$2" ] && out="${out}; reading the fake width: $(list_text "$2")"
  [ -n "$3" ] && out="${out}; reading past it: $(list_text "$3")"
  printf '%s' "$out"
}

# judgeable <shown> <source> <reads> <resists>: every reason an arm expected
# to build cannot be judged, each flagged once; false when there is one.
judgeable() {
  local shown=$1 src=$2 reads=$3 resists=$4 i n dups ok=0 why
  load_items "$src"
  [ "$N_ITEMS" -gt 0 ] || {
    unjudgeable structure "${shown}: expected to build and checks nothing, so its build shows nothing" \
      "$(code "$shown")" "none" "a value check"
    ok=1
  }
  for ((i = 1; i <= N_ITEMS; i++)); do
    why=
    [ "${I_LINE[$i]}" -gt 0 ] || why="no comparison"
    if [ -z "$why" ] && [ "${I_SHAPE[$i]}" = "const assert" ]; then
      [ -n "${I_MSG[$i]}" ] || why="no message"
      [ "${I_BAD[$i]}" = - ] || why="a message holding ${I_BAD[$i]}, which the harness does not match"
    fi
    [ -z "$why" ] || {
      unjudgeable structure "${shown}, item ${i} (${I_SHAPE[$i]}): ${why}" \
        "$(code "$shown"), item ${i}" "${I_SHAPE[$i]}, ${why}" "a comparison and a matchable message"
      ok=1
    }
    n=0
    case ",${reads}," in *",${i},"*) n=$((n + 1)) ;; esac
    case ",${resists}," in *",${i},"*) n=$((n + 1)) ;; esac
    [ "$n" -eq 1 ] || {
      unjudgeable structure "${shown}, item ${i}: declared in ${n} of --reads and --resists, not in exactly one" \
        "$(code "$shown"), item ${i}" "declared in ${n} lists" "declared in one"
      ok=1
    }
  done
  for i in ${reads//,/ } ${resists//,/ }; do
    [ "$i" -le "$N_ITEMS" ] || {
      unjudgeable structure "${shown}: declares item ${i} and holds ${N_ITEMS}" \
        "$(code "$shown")" "${N_ITEMS} item(s)" "item ${i} declared"
      ok=1
    }
  done
  dups=$(for ((i = 1; i <= N_ITEMS; i++)); do
    [ "${I_SHAPE[$i]}" = "const assert" ] && [ -n "${I_MSG[$i]}" ] && printf '%s\n' "${I_MSG[$i]}"
  done | sort | uniq -d)
  [ -z "$dups" ] || {
    unjudgeable structure "${shown}: two items share a message, so their refusals cannot be told apart" \
      "$(code "$shown")" "shared message" "distinct messages"
    ok=1
  }
  return "$ok"
}

flip_pass() {
  local kind=$1 arm=$2 src=$3 shown=$4 i vname vsrc line count
  load_items "$src"
  count=$N_ITEMS
  for ((i = 1; i <= count; i++)); do
    vname="${arm}.generated_flip_${i}"
    if [ "$kind" = file ]; then
      vsrc="${vname}.rs"
    else
      vsrc="$(manifest_variant "$arm" "generated_flip_${i}")/src/lib.rs"
    fi
    load_items "$src"
    flip "$src" "${I_LINE[$i]}" "${I_COL[$i]}" "$vsrc"
    line=$(sed -n "${I_LINE[$i]}p" "$vsrc" | sed 's/^[[:space:]]*//')
    echo "-- flip pass: ${shown}, item ${i} (${I_SHAPE[$i]}), line ${I_LINE[$i]} flipped to: ${line}"
    expect_refused_by "$vsrc" "$i" ""
    run_arm flip "$kind" "$vname" refused
    row "$(code "$shown"), item ${i} flipped" "${I_SHAPE[$i]}: $(code "$line")" \
      "refused by that item alone" "${VERDICT}"
  done
}

premise_pass() {
  local kind=$1 arm=$2 src=$3 shown=$4 expected=$5 reads=$6 resists=$7
  local vname="${arm}.generated_premise" vsrc dir stand changed=0 want label
  label="$(code "$shown"), fake width set to ${HOST_WIDTH}"
  if [ "$kind" = file ]; then
    vsrc="${vname}.rs"
    premise_of "$src" "$vsrc"
    cmp -s "$src" "$vsrc" || changed=1
  else
    dir=$(manifest_variant "$arm" generated_premise)
    vsrc="${dir}/src/lib.rs"
    premise_of "$src" "$vsrc"
    cmp -s "$src" "$vsrc" || changed=1
    stand=$(sed -n 's/.*path = "\.\.\/\([^"]*\)".*/\1/p' "manifest_rename/${arm}/Cargo.toml")
    if [ -n "$stand" ]; then
      rm -rf "manifest_rename/${stand}.generated_premise"
      mkdir -p "manifest_rename/${stand}.generated_premise/src"
      cp "manifest_rename/${stand}/Cargo.toml" "manifest_rename/${stand}.generated_premise/"
      premise_of "manifest_rename/${stand}/src/lib.rs" \
        "manifest_rename/${stand}.generated_premise/src/lib.rs"
      cmp -s "manifest_rename/${stand}/src/lib.rs" \
        "manifest_rename/${stand}.generated_premise/src/lib.rs" || changed=1
      awk -v from="\"../${stand}\"" -v to="\"../${stand}.generated_premise\"" '
        { i = index($0, from); if (i) $0 = substr($0, 1, i - 1) to substr($0, i + length(from)); print }
      ' "manifest_rename/${arm}/Cargo.toml" >"${dir}/Cargo.toml"
    fi
  fi
  if [ "$changed" -eq 0 ]; then
    unjudgeable premise "${shown}: premise pass, no fake width to set" \
      "$label" "no fake width" "a fake width to set"
    return 0
  fi
  echo "-- premise pass: ${shown}, every fake width set to ${HOST_WIDTH}, the host's pointer width"
  if [ "$expected" = builds ] && [ -n "$reads" ]; then
    expect_refused_by "$vsrc" "$reads" "$resists"
    want="refused by $(list_text "$reads") alone"
    run_arm premise "$kind" "$vname" refused
  elif [ "$expected" = builds ]; then
    expect_holding "$vsrc" "$resists"
    want="builds"
    run_arm premise "$kind" "$vname" builds
  elif [ -n "$reads" ]; then
    REQ=()
    ABS=()
    want="builds"
    run_arm premise "$kind" "$vname" builds
  else
    REQ=("${ARM_TEXTS[@]}")
    ABS=()
    want="refused with the same texts"
    run_arm premise "$kind" "$vname" refused
  fi
  row "$label" "$([ -n "$reads" ] && list_text "$reads" || printf 'no item') reading the fake width" \
    "$want" "${VERDICT}"
}

spelling_pass() {
  local kind=$1 arm=$2 src=$3 shown=$4 reads=$5 resists=$6
  local vname="${arm}.generated_spelling" vsrc label
  label="$(code "$shown"), leading \`::\` dropped"
  if [ "$kind" = file ]; then
    vsrc="${vname}.rs"
  else
    vsrc="$(manifest_variant "$arm" generated_spelling)/src/lib.rs"
  fi
  spelling_of "$src" "$vsrc"
  if cmp -s "$src" "$vsrc"; then
    unjudgeable spelling \
      "${shown}: spelling pass, $(list_text "$resists") read past the fake width and no leading-:: core path to drop" \
      "$label" "no leading-\`::\` \`core\` path" "a path to drop"
    return 0
  fi
  echo "-- spelling pass: ${shown}, the leading :: dropped from every ::core:: path"
  expect_refused_by "$vsrc" "$resists" "$reads"
  run_arm spelling "$kind" "$vname" refused
  row "$label" "$(list_text "$resists") reading past the fake width" \
    "refused by $(list_text "$resists") alone" "${VERDICT}"
}

# The harness's own control. Each arm in `selfcheck/` is written to be flagged
# by a stated set of passes, `none` included; its output is shown with a `| `
# prefix, its flags are taken back out of the count, and only a set other than
# the stated one is counted as unexpected.
selfcheck() {
  local arm=$1 what=$2 want=$3
  shift 3
  local before=$unexpected rows_before=${#ROWS[@]} got= p c k
  local s_arm=$F_arm s_structure=$F_structure s_flip=$F_flip s_premise=$F_premise s_spelling=$F_spelling
  check "selfcheck/${arm}" builds "$@" >"${OUT_DIR}/selfcheck.log" 2>&1
  sed 's/^/  | /' "${OUT_DIR}/selfcheck.log"
  for p in arm structure flip premise spelling; do
    case $p in
    arm) c=$((F_arm - s_arm)) ;;
    structure) c=$((F_structure - s_structure)) ;;
    flip) c=$((F_flip - s_flip)) ;;
    premise) c=$((F_premise - s_premise)) ;;
    spelling) c=$((F_spelling - s_spelling)) ;;
    esac
    for ((k = 0; k < c; k++)); do got="${got:+${got}, }${p}"; done
  done
  got=${got:-none}
  unexpected=$before
  F_arm=$s_arm F_structure=$s_structure F_flip=$s_flip F_premise=$s_premise F_spelling=$s_spelling
  ROWS=("${ROWS[@]:0:$rows_before}")
  echo "selfcheck/${arm}.rs: flagged by ${got}, want ${want}"
  if [ "$got" = "$want" ]; then
    echo "verdict: as expected"
    row "harness self-check, $(code "selfcheck/${arm}.rs")" "$what" "flagged by ${want}" "flagged by ${got}"
  else
    echo "verdict: UNEXPECTED"
    unexpected=$((unexpected + 1))
    row "harness self-check, $(code "selfcheck/${arm}.rs")" "$what" "flagged by ${want}" "UNEXPECTED, flagged by ${got}"
  fi
  echo
}
