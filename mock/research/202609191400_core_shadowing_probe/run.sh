#!/usr/bin/env bash
# Which ways of binding the name `core` reach the leading-:: path the
# platform-width aliases read, and which only shadow a bare `core::...` path.
#
# One command, from anywhere: `./run.sh` in this directory. Every arm in this
# directory is run, each with the outcome it is expected to have: `builds`, or
# `refused` together with every text the compiler's refusal has to contain, so
# an arm refused for some other reason counts as unexpected rather than as a
# pass. The single-file arms are checked with `rustc --emit=metadata`, the part
# of a build `cargo check` runs, and the manifest-rename fixtures under
# `manifest_rename/` with `cargo check --locked`. All of it runs at the host on
# the pinned toolchain (`rust-toolchain.toml`); this is a name-resolution
# question rather than a target one, so no cross target is needed.
#
# The checks. An arm checks a value by one of two shapes, each a free `const _`
# item evaluated when the crate is checked: `const _: () = assert!(<cond>,
# "<message>");`, or, where no `assert!` is in scope, `const _: [(); 1] =
# [(); (<cond>) as usize];`, which is a length mismatch when `<cond>` is false.
# Every such item's condition holds one `==` or `!=`.
#
# The controls, generated rather than written. For every arm expected to build,
# this script finds each of those items and derives one variant per item, the
# arm with that item's first `==` turned into `!=` or the reverse and nothing
# else changed. Each variant is expected refused, and refused by that item
# alone: a `const assert` variant with `error[E0080]: evaluation panicked:` and
# the item's own message, an `array length` variant with `error[E0308]:
# mismatched types` and the flipped line itself, and either with `due to 1
# previous error`. So a check that could never fail shows up as an unexpected
# `builds`, and an arm added later is covered by the same loop. An arm expected
# to build that holds no such item, an item with no comparison to flip, a
# `const assert` with no message, or two items in one arm sharing a message,
# are each counted as unexpected, since none of them could be told apart by
# its control. `harness_self_check` below feeds the loop three arms built to
# trip it and checks that each is flagged once.
#
# The whole output of every run, exit statuses and verdicts included, goes to
# `run.out` beside this script, so the committed file is the run rather than a
# transcription of it. The same run writes `arms.md`, one row per arm and per
# generated control: what the arm checks, what it is expected to do, and what
# it did. The script exits non-zero when any outcome is not the expected one,
# and the last line of `run.out` says how many were.
set -uo pipefail
cd "$(dirname "$0")"

OUT_DIR=$(mktemp -d)
# The generated variants sit beside the arms they come from, so a manifest
# variant's `../<stand-in>` path dependency and every rustc diagnostic path
# read the same as the arm's own. They are named `*.flipped_<n>`, which the
# `.gitignore` here names, and removed on exit.
trap 'rm -rf "$OUT_DIR" ./*.flipped_*.rs manifest_rename/*.flipped_*' EXIT

unexpected=0
ROWS=()
VERDICT=

# judge <status> <expected> <log> [<text the refusal contains>...]
judge() {
  local status=$1 expected=$2 log=$3
  shift 3
  local missing=0 t
  for t in "$@"; do
    grep -qF -- "$t" "$log" || missing=1
  done
  echo "exit ${status}"
  if [ "$expected" = builds ] && [ "$status" -eq 0 ]; then
    VERDICT="builds"
    echo "verdict: as expected, builds"
  elif [ "$expected" = refused ] && [ "$status" -ne 0 ] && [ "$missing" -eq 0 ]; then
    VERDICT="refused"
    echo "verdict: as expected, refused with:"
    for t in "$@"; do echo "  ${t}"; done
  else
    VERDICT="UNEXPECTED, exit ${status}"
    echo "verdict: UNEXPECTED, expected ${expected}"
    for t in "$@"; do echo "  with: ${t}"; done
    unexpected=$((unexpected + 1))
  fi
  echo
}

# The value checks in one source file, one line per item:
# <index> TAB <shape> TAB <line of the comparison, 0 if none> TAB <message>.
checks_in() {
  awk '
    /^const _: \(\) = assert!\(/ { n++; shape[n] = "const assert"; open = 1 }
    /^const _: \[\(\); 1\] = \[\(\); / { n++; shape[n] = "array length"; open = 1 }
    open {
      if (!cmp[n] && match($0, /[!=]=/)) cmp[n] = NR
      rest = $0
      while (match(rest, /"[^"]*"/)) {
        msg[n] = substr(rest, RSTART + 1, RLENGTH - 2)
        rest = substr(rest, RSTART + RLENGTH)
      }
      if ($0 ~ /;[[:space:]]*$/) open = 0
    }
    END { for (i = 1; i <= n; i++) printf "%d\t%s\t%d\t%s\n", i, shape[i], cmp[i] + 0, msg[i] }
  ' "$1"
}

# What one source file checks, as the table says it: the items with a
# comparison, counted by shape, or `none`.
describe() {
  local asserts arrays out=
  asserts=$(checks_in "$1" | awk -F'\t' '$2 == "const assert" && $3 > 0' | wc -l | tr -d ' ')
  arrays=$(checks_in "$1" | awk -F'\t' '$2 == "array length" && $3 > 0' | wc -l | tr -d ' ')
  [ "$asserts" -gt 0 ] && out="${asserts} const assert"
  [ "$arrays" -gt 0 ] && out="${out:+${out}, }${arrays} array length"
  echo "${out:-none}"
}

# flip <source> <line> <out>: the source with the first `==` or `!=` on that
# line turned into the other, every other byte unchanged.
flip() {
  awk -v at="$2" '
    NR == at && match($0, /[!=]=/) {
      op = (substr($0, RSTART, 1) == "=") ? "!=" : "=="
      $0 = substr($0, 1, RSTART - 1) op substr($0, RSTART + 2)
    }
    { print }
  ' "$1" >"$3"
}

row() {
  ROWS+=("| $1 | $2 | $3 | $4 |")
}

# Code span that survives a backtick inside it.
code() {
  printf '`` %s ``' "$1"
}

# run_arm <kind> <arm> <source> <expected> [<text>...]: one arm, single-file
# or manifest, under whichever of the two tools it takes.
run_arm() {
  local kind=$1 arm=$2 src=$3 expected=$4
  shift 4
  local log="${OUT_DIR}/${arm//\//_}.log" status
  if [ "$kind" = file ]; then
    echo "== ${arm}.rs"
    rustc --edition 2024 --crate-type lib --crate-name "${arm%%.*}" --emit=metadata \
      --out-dir "$OUT_DIR" "${arm}.rs" </dev/null >"$log" 2>&1
  else
    echo "== manifest_rename/${arm}"
    (cd "manifest_rename/${arm}" && cargo check --quiet --locked) </dev/null >"$log" 2>&1
  fi
  status=$?
  cat "$log"
  judge "$status" "$expected" "$log" "$@"
}

# check <arm> <expected> [<text>...]: one single-file arm, and its controls.
check() {
  local arm=$1 expected=$2
  shift 2
  arm_and_controls file "$arm" "${arm}.rs" "$expected" "$@"
}

# check_manifest <crate> <expected> [<text>...]: one fixture under
# manifest_rename/, and its controls.
check_manifest() {
  local crate=$1 expected=$2
  shift 2
  arm_and_controls manifest "$crate" "manifest_rename/${crate}/src/lib.rs" "$expected" "$@"
}

arm_and_controls() {
  local kind=$1 arm=$2 src=$3 expected=$4
  shift 4
  local shown checks expect_text
  if [ "$kind" = file ]; then shown="${arm}.rs"; else shown="manifest_rename/${arm}"; fi
  checks=$(describe "$src")
  run_arm "$kind" "$arm" "$src" "$expected" "$@"
  if [ "$expected" = refused ]; then
    expect_text="refused with $(printf '%s; ' "$@")"
    expect_text="${expect_text%; }"
  else
    expect_text="builds"
  fi
  row "$(code "$shown")" "$checks" "$(code "$expect_text")" "${VERDICT}"
  [ "$expected" = builds ] || return 0

  local items n=0 idx shape at msg line variant vsrc vname dups
  items=$(checks_in "$src")
  dups=$(printf '%s\n' "$items" | awk -F'\t' '$2 == "const assert" && $4 != ""' \
    | cut -f4 | sort | uniq -d)
  if [ -n "$dups" ]; then
    echo "== ${shown}: two items share a message, so their controls cannot be told apart"
    echo "verdict: UNEXPECTED"
    echo
    unexpected=$((unexpected + 1))
    row "$(code "$shown")" "shared message" "distinct messages" "UNEXPECTED"
  fi
  while IFS=$'\t' read -r idx shape at msg; do
    [ -n "$idx" ] || continue
    n=$((n + 1))
    vname="${arm}.flipped_${idx}"
    if [ "$at" -eq 0 ] || { [ "$shape" = "const assert" ] && [ -z "$msg" ]; }; then
      echo "== ${shown}, item ${idx} (${shape}): no comparison to flip, or no message"
      echo "verdict: UNEXPECTED"
      echo
      unexpected=$((unexpected + 1))
      row "$(code "$shown"), item ${idx}" "${shape}, not flippable" "refused" "UNEXPECTED"
      continue
    fi
    if [ "$kind" = file ]; then
      vsrc="${vname}.rs"
    else
      rm -rf "manifest_rename/${vname}"
      mkdir -p "manifest_rename/${vname}"
      cp -R "manifest_rename/${arm}/Cargo.toml" "manifest_rename/${arm}/Cargo.lock" \
        "manifest_rename/${vname}/"
      mkdir -p "manifest_rename/${vname}/src"
      vsrc="manifest_rename/${vname}/src/lib.rs"
    fi
    flip "$src" "$at" "$vsrc"
    line=$(sed -n "${at}p" "$vsrc" | sed 's/^[[:space:]]*//')
    echo "-- generated control: ${shown}, item ${idx} (${shape}), line ${at} flipped to: ${line}"
    if [ "$shape" = "const assert" ]; then
      run_arm "$kind" "$vname" "$vsrc" refused \
        "error[E0080]: evaluation panicked: ${msg}" "due to 1 previous error"
    else
      run_arm "$kind" "$vname" "$vsrc" refused \
        "error[E0308]: mismatched types" "$line" "due to 1 previous error"
    fi
    row "$(code "$shown"), item ${idx} flipped" "${shape}: $(code "$line")" \
      "refused by that item alone" "${VERDICT}"
  done <<<"$items"
  if [ "$n" -eq 0 ]; then
    echo "== ${shown}: expected to build and checks nothing, so its build shows nothing"
    echo "verdict: UNEXPECTED"
    echo
    unexpected=$((unexpected + 1))
    row "$(code "$shown")" "none" "a value check" "UNEXPECTED"
  fi
}

# The control loop's own control. Three arms written here to be flagged, each
# exactly once: one whose assertion holds with its comparison either way round,
# so its generated control builds; one expected to build that checks nothing;
# and one whose assertion carries no message. Their output is shown with a
# `| ` prefix, their flags are taken back out of the count, and only a flag
# count other than one per arm is counted as unexpected.
harness_self_check() {
  local before=$unexpected rows_before=${#ROWS[@]} arm b got bad=0
  printf '#![no_std]\nconst _: () = assert!(1 == 1 || true, "holds either way round");\n' \
    >selfcheck_vacuous.flipped_0.rs
  printf '#![no_std]\npub const W: u32 = 8;\n' >selfcheck_unchecked.flipped_0.rs
  printf '#![no_std]\nconst _: () = assert!(1 == 1);\n' >selfcheck_unmessaged.flipped_0.rs
  echo "== harness self-check: three arms built to be flagged, each exactly once"
  for arm in selfcheck_vacuous.flipped_0 selfcheck_unchecked.flipped_0 \
    selfcheck_unmessaged.flipped_0; do
    b=$unexpected
    check "$arm" builds >"${OUT_DIR}/selfcheck.log" 2>&1
    sed 's/^/  | /' "${OUT_DIR}/selfcheck.log"
    got=$((unexpected - b))
    echo "${arm%%.*}: flagged ${got} time(s), want 1"
    [ "$got" -eq 1 ] || bad=1
  done
  unexpected=$before
  ROWS=("${ROWS[@]:0:$rows_before}")
  if [ "$bad" -eq 0 ]; then
    echo "verdict: as expected, each flagged once"
    row "harness self-check" "a vacuous check, no check, an unmessaged check" \
      "each flagged once" "each flagged once"
  else
    echo "verdict: UNEXPECTED, the control loop did not flag each once"
    unexpected=$((unexpected + 1))
    row "harness self-check" "a vacuous check, no check, an unmessaged check" \
      "each flagged once" "UNEXPECTED"
  fi
  echo
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

main() {
  echo "tool: $(rustc --version)"
  echo "tool: $(cargo --version)"
  echo

  # The bare spelling is shadowed by a module and by a `use` alias.
  check a_mod_core_shadows_the_bare_path builds
  check a_use_alias_shadows_the_bare_path builds
  check a_bare_mod_usize_confirms_the_original_premise builds

  # The leading `::` resists a module and each shape of `use` that binds
  # `core`: an alias, an unaliased last segment, and a `self` under `core`.
  check the_leading_double_colon_resists_both_shadows builds
  check the_use_alias_does_not_reach_the_leading_colon_path builds
  check the_leading_double_colon_resists_an_unaliased_use_of_core builds
  check the_leading_double_colon_resists_a_self_under_core builds

  # An `extern crate self as core` at the crate root reaches it, plain or raw.
  check extern_crate_self_as_core_hijacks_the_absolute_path builds
  check the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path builds

  # The same alias out of a macro is refused by the compiler. Its one assertion
  # is never evaluated, since the refusal comes first; the arm checks the
  # refusal, not a value.
  check a_macro_expanded_extern_crate_self_as_core_is_refused refused \
    'macro-expanded `extern crate` items cannot shadow names passed with `--extern`'

  # A dependency renamed to `core` in the manifest reaches it for a `no_std`
  # dependent whenever the renamed crate carries the path the edition's
  # prelude import names: the real `core` re-exported whole, the real
  # `prelude` module alone, or an empty hand-written `prelude::rust_2024`.
  check_manifest user builds
  check_manifest user_of_the_crate_with_only_the_prelude builds
  check_manifest user_of_the_crate_with_an_empty_prelude builds

  # Without that path a `no_std` dependent stops at the prelude import. These
  # two check no value: each is refused before any item is evaluated, and the
  # refusal is what they show.
  check_manifest user_of_the_crate_without_the_glob refused \
    'cannot resolve a prelude import'
  check_manifest user_of_the_crate_with_a_bare_prelude refused \
    'cannot resolve a prelude import'

  # Whether `assert!` is in scope under the empty and the bare stand-in
  # prelude. Each names `assert!(true)` once, which checks no value; the name
  # lookup failing is what they show.
  check_manifest user_of_the_crate_with_an_empty_prelude_naming_assert refused \
    'cannot find macro `assert` in this scope'
  check_manifest user_of_the_crate_with_a_bare_prelude_naming_assert refused \
    'cannot resolve a prelude import' 'cannot find macro `assert` in this scope'

  # A std dependent, carrying no `#![no_std]`, against the stand-in that
  # refuses the `no_std` dependent above.
  check_manifest a_std_dependent_of_the_crate_without_the_glob builds

  # The positive control and the hand-written negative control. The second is
  # refused by its own assertion, the one refused arm here whose refusal is a
  # value check.
  check the_shipped_spelling_alone_is_silent builds
  check the_bare_spelling_without_the_leading_colon_is_shadowed refused \
    "the bare spelling read the \`mod core\` shadow's 8 rather than the host's pointer width"

  harness_self_check

  write_table
  echo "unexpected outcomes: ${unexpected}"
  [ "$unexpected" -eq 0 ]
}

main 2>&1 | tee run.out
exit "${PIPESTATUS[0]}"
