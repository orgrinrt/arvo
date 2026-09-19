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
# question rather than a target one, so no cross target is needed. The
# functions live in `harness/`; this file holds the arms and what each is
# expected to do.
#
# The checks. An arm checks a value by one of two shapes, each a free `const _`
# item evaluated when the crate is checked: `const _: () = assert!(<cond>,
# "<message>");`, or, where no `assert!` is in scope, `const _: [(); 1] =
# [(); (<cond>) as usize];`, which is a length mismatch when `<cond>` is false.
# Every such item's condition holds one `==` or `!=` outside a string literal.
# Each arm's premise is a fake width, a shadow's or a stand-in's `const BITS:
# u32 = <n>;`, and the arm declares, in `main` below, which of its items read
# that fake width (`--reads`) and which read past it through the leading `::`
# (`--resists`).
#
# Three passes of generated variants follow the arm itself, each variant the
# arm with one thing changed and nothing else.
#
# The flip pass, for every arm expected to build: one variant per item, that
# item's first `==` or `!=` outside a string literal turned into the other,
# expected refused by that item alone. A `const assert` variant has to be
# refused with `error[E0080]: evaluation panicked:` and the item's own message,
# an `array length` variant with `error[E0308]: mismatched types` and the
# flipped line itself, and either with `due to 1 previous error`. This shows
# the item is evaluated and its comparison decides whether the arm builds. It
# does not show the comparison could come out the other way on the arm as
# written: one whose two sides cannot differ, `W == W`, is refused once
# flipped and so passes this pass.
#
# The premise pass, for every arm: one variant with every fake width set to
# the host's pointer width, in the arm and, for a manifest arm, in a copy of
# its stand-in. At the host's width a read of the fake width and a read of the
# real one agree, so an item that tells the two apart has to change outcome,
# `W == 8` and `W != usize::BITS` alike. An arm expected to build is expected
# refused by exactly its `--reads` items, each with its own message or line and
# `due to <n> previous error(s)` for their count, with no `--resists` item's
# message or line in the output, and expected to build when it has no
# `--reads` item. A refused arm with `--reads` items, the negative control, is
# expected to build; any other refused arm is expected refused with the same
# texts. This shows a `--reads` item's outcome depends on the fake width. It
# shows nothing about a `--resists` item, which is expected to hold either
# way, so one whose two sides cannot differ passes this pass.
#
# The spelling pass, for every arm expected to build that holds a `--resists`
# item: one variant with the leading `::` dropped from every `::core::` path
# outside a comment. Expected refused by exactly the `--resists` items, the
# same way as above, and with no `--reads` item's message or line in the
# output. This shows a `--resists` item's outcome depends on the leading `::`.
# An arm holding a `--resists` item and no leading-`::` `core` path is flagged.
#
# What no pass shows. Each pass judges an item whole, so a comparison whose two
# sides cannot differ, joined by `&&` to one that responds, is flagged by none
# of the three. And a pass compares an item's outcome with the item's
# declaration, so a wrong declaration is flagged only where the two disagree.
#
# Flagged before any pass, and the arm's passes then skipped: an arm expected
# to build that holds no item; an item with no comparison; a `const assert`
# with no message, or whose message holds an escape other than `\"` and `\\`,
# a brace or a line break, none of which the harness matches against rustc's
# panic text; two items sharing a message; an item declared in neither list or
# in both. An arm declared `--checks-nothing` is expected to build and to hold
# no item, and has no passes.
#
# `harness_self_check` runs the arms in `selfcheck/`, each written to be
# flagged by a stated set of passes, and checks each is flagged by exactly that
# set, the shapes a pass misses included.
#
# The whole output of every run, exit statuses and verdicts included, goes to
# `run.out` beside this script, so the committed file is the run rather than a
# transcription of it. The same run writes `arms.md`, one row per arm and per
# generated variant: what the arm checks, what it is expected to do, and what
# it did. The script exits non-zero when any outcome is not the expected one,
# and the last line of `run.out` says how many were.
set -uo pipefail
cd "$(dirname "$0")"

OUT_DIR=$(mktemp -d)
# The generated variants sit beside the arms they come from, so a manifest
# variant's `../<stand-in>` path dependency and every rustc diagnostic path
# read the same as the arm's own. They are named `*.generated_*`, which the
# `.gitignore` here names, and removed on exit.
trap 'rm -rf "$OUT_DIR" ./*.generated_*.rs selfcheck/*.generated_*.rs manifest_rename/*.generated_*' EXIT

# The host's pointer width, which the premise pass sets every fake width to.
HOST_WIDTH=
# The count of unexpected outcomes, and the same count per pass that found it.
unexpected=0
F_arm=0 F_structure=0 F_flip=0 F_premise=0 F_spelling=0
# The rows of `arms.md`, and the verdict of the last arm or variant run.
ROWS=()
VERDICT=
# What the next run's refusal has to contain, and what its output must not.
REQ=()
ABS=()
# The refusal texts of the arm being run, for its premise pass.
ARM_TEXTS=()

. harness/judge.sh
. harness/source.sh
. harness/passes.sh

harness_self_check() {
  echo "== harness self-check: the arms in selfcheck/, each flagged by exactly the passes it names"
  selfcheck holds_either_way_round "$(code 'W == 8 || true')" "flip, premise" --reads 1
  selfcheck checks_nothing "no item" "structure"
  selfcheck no_message "$(code 'assert!(W == 8)'), no message" "structure" --reads 1
  selfcheck an_escape_the_harness_cannot_match "a message holding \`\\n\`" "structure" --reads 1
  selfcheck an_item_declared_in_neither_list "item 2 in neither list" "structure" --reads 1
  selfcheck an_item_declared_in_both_lists "item 1 in both lists" "structure" --reads 1 --resists 1
  selfcheck two_items_sharing_a_message "two items, one message" "structure" --reads 1,2
  selfcheck declared_to_check_nothing_and_checks "declared \`--checks-nothing\`, holds an item" "structure" --checks-nothing
  selfcheck compares_the_width_with_itself "$(code 'W == W')" "premise" --reads 1
  selfcheck a_true_comparison_before_the_width "$(code '1 == 1 && W == W')" "premise" --reads 1
  selfcheck compares_the_width_with_another_constant "$(code 'W != 7'), the fake width 8" "premise" --reads 1
  selfcheck a_resisted_read_compared_with_itself "$(code 'W == W'), W read past the fake width" "spelling" --resists 1
  selfcheck the_shipped_spelling_old_assertion "two reads of the real width, no shadow" "premise, spelling" --resists 1
  selfcheck a_read_of_the_fake_width_declared_as_read_past_it \
    "$(code 'W == 8'), W read through the shadow, declared \`--resists\`" "premise, spelling" --resists 1
  selfcheck a_vacuous_conjunct_beside_a_real_one "$(code 'W == 8 && W == W')" "none" --reads 1
  selfcheck an_escaped_quote_in_the_message "a message holding \`\\\"\`" "none" --reads 1
  selfcheck an_equals_sign_in_a_string_before_the_comparison "a string holding \`==\` before the comparison" "none" --reads 1
}

main() {
  echo "tool: $(rustc --version)"
  echo "tool: $(cargo --version)"
  HOST_WIDTH=$(rustc --print cfg | sed -n 's/^target_pointer_width="\([0-9]*\)"$/\1/p')
  if [ -z "$HOST_WIDTH" ]; then
    echo "no target_pointer_width in rustc --print cfg"
    return 1
  fi
  echo "host pointer width, the premise pass's fake width: ${HOST_WIDTH}"
  echo

  # The bare spelling is shadowed by a module and by a `use` alias.
  check a_mod_core_shadows_the_bare_path builds --reads 1
  check a_use_alias_shadows_the_bare_path builds --reads 1
  check a_bare_mod_usize_confirms_the_original_premise builds --reads 1

  # The leading `::` resists a module and each shape of `use` that binds
  # `core`: an alias, an unaliased last segment, and a `self` under `core`.
  check the_leading_double_colon_resists_both_shadows builds --resists 1
  check the_use_alias_does_not_reach_the_leading_colon_path builds --resists 1
  check the_leading_double_colon_resists_an_unaliased_use_of_core builds --reads 1 --resists 2
  check the_leading_double_colon_resists_a_self_under_core builds --reads 1 --resists 2

  # An `extern crate self as core` at the crate root reaches it, plain or raw.
  check extern_crate_self_as_core_hijacks_the_absolute_path builds --reads 1
  check the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path builds --reads 1,2

  # The same alias out of a macro is refused by the compiler. Its one assertion
  # is never evaluated, since the refusal comes first; the arm checks the
  # refusal, not a value.
  check a_macro_expanded_extern_crate_self_as_core_is_refused refused \
    'macro-expanded `extern crate` items cannot shadow names passed with `--extern`'

  # A dependency renamed to `core` in the manifest reaches it for a `no_std`
  # dependent whenever the renamed crate carries the path the edition's
  # prelude import names: the real `core` re-exported whole, the real
  # `prelude` module alone, or an empty hand-written `prelude::rust_2024`.
  check_manifest user builds --reads 1,2
  check_manifest user_of_the_crate_with_only_the_prelude builds --reads 1,2
  check_manifest user_of_the_crate_with_an_empty_prelude builds --reads 1,2

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
  check_manifest a_std_dependent_of_the_crate_without_the_glob builds --reads 1,2

  # The positive control, which builds and checks no value, and the
  # hand-written negative control, refused by its own assertion, the one
  # refused arm here whose refusal is a value check.
  check the_shipped_spelling_alone_is_silent builds --checks-nothing
  check the_bare_spelling_without_the_leading_colon_is_shadowed refused --reads 1 \
    "the bare spelling read the \`mod core\` shadow's 8 rather than the host's pointer width"

  harness_self_check

  write_table
  echo "unexpected outcomes: ${unexpected}"
  [ "$unexpected" -eq 0 ]
}

main 2>&1 | tee run.out
exit "${PIPESTATUS[0]}"
