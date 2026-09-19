# Reading an arm's checks out of its source, and writing the variants the
# passes build. Sourced by `run.sh`, which says what each pass expects.

# The value checks in one source file, one line per item:
# <index> TAB <shape> TAB <line of the comparison, 0 if none> TAB <its column>
# TAB <what is wrong with the message, `-` if nothing> TAB <message>.
# String literals are read with their escapes, so an `==` or a `"` inside one
# is neither the comparison nor the end of the message, and the message is the
# item's last string literal, which is what `assert!` takes as its message.
checks_in() {
  awk '
    /^const _: \(\) = assert!\(/ { n++; shape[n] = "const assert"; open = 1; has[n] = 0; bad[n] = "" }
    /^const _: \[\(\); 1\] = \[\(\); / { n++; shape[n] = "array length"; open = 1; has[n] = 0; bad[n] = "" }
    open {
      line = $0
      for (i = 1; i <= length(line); i++) {
        c = substr(line, i, 1)
        if (instr) {
          if (c == "\\") {
            e = substr(line, i + 1, 1)
            if (e == "\"" || e == "\\") cur = cur e
            else if (!bad[n]) bad[n] = "an escape other than \\\" or \\\\"
            i++
            continue
          }
          if (c == "\"") { instr = 0; has[n] = 1; msg[n] = cur; continue }
          if ((c == "{" || c == "}") && !bad[n]) bad[n] = "a brace"
          cur = cur c
          continue
        }
        if (c == "\"") { instr = 1; cur = ""; continue }
        if (c == "'"'"'" && substr(line, i + 2, 1) == "'"'"'") { i += 2; continue }
        if (substr(line, i, 2) == "//") break
        two = substr(line, i, 2)
        if (!cmp[n] && (two == "==" || two == "!=")) { cmp[n] = NR; col[n] = i }
      }
      if (instr && !bad[n]) bad[n] = "a line break"
      if (!instr && $0 ~ /;[[:space:]]*$/) open = 0
    }
    END {
      for (i = 1; i <= n; i++) {
        m = has[i] ? msg[i] : ""
        printf "%d\t%s\t%d\t%d\t%s\t%s\n", i, shape[i], cmp[i] + 0, col[i] + 0, (bad[i] ? bad[i] : "-"), m
      }
    }
  ' "$1"
}

# load_items <source>: the items of one source file into I_SHAPE, I_LINE,
# I_COL, I_BAD and I_MSG, indexed from 1, and their count into N_ITEMS.
load_items() {
  local idx shape at col bad msg
  I_SHAPE=() I_LINE=() I_COL=() I_BAD=() I_MSG=()
  N_ITEMS=0
  while IFS=$'\t' read -r idx shape at col bad msg; do
    [ -n "$idx" ] || continue
    I_SHAPE[idx]=$shape
    I_LINE[idx]=$at
    I_COL[idx]=$col
    I_BAD[idx]=$bad
    I_MSG[idx]=$msg
    N_ITEMS=$idx
  done < <(checks_in "$1")
}

# item_text <index> <source>: the text a refusal by that item carries, read
# from the source the variant was built from.
item_text() {
  if [ "${I_SHAPE[$1]}" = "const assert" ]; then
    printf 'error[E0080]: evaluation panicked: %s' "${I_MSG[$1]}"
  else
    sed -n "${I_LINE[$1]}p" "$2" | sed 's/^[[:space:]]*//'
  fi
}

# expect_refused_by <source> <refusing items> <holding items>: REQ and ABS for
# a variant refused by exactly the first list, the items read out of that
# variant's source.
expect_refused_by() {
  local src=$1 fail=$2 hold=$3 i n=0 arr=0
  load_items "$src"
  REQ=()
  ABS=()
  for i in ${fail//,/ }; do
    REQ+=("$(item_text "$i" "$src")")
    [ "${I_SHAPE[$i]}" = "array length" ] && arr=1
    n=$((n + 1))
  done
  [ "$arr" -eq 1 ] && REQ+=("error[E0308]: mismatched types")
  if [ "$n" -eq 1 ]; then REQ+=("due to 1 previous error"); else REQ+=("due to ${n} previous errors"); fi
  for i in ${hold//,/ }; do
    ABS+=("$(item_text "$i" "$src")")
  done
}

# expect_holding <source> <holding items>: REQ and ABS for a variant that
# builds, none of the listed items refusing it.
expect_holding() {
  local src=$1 hold=$2 i
  load_items "$src"
  REQ=()
  ABS=()
  for i in ${hold//,/ }; do
    ABS+=("$(item_text "$i" "$src")")
  done
}

# flip <source> <line> <column> <out>: the source with the `==` or `!=` at
# that line and column turned into the other, every other byte unchanged.
flip() {
  awk -v at="$2" -v c="$3" '
    NR == at {
      op = (substr($0, c, 1) == "=") ? "!=" : "=="
      $0 = substr($0, 1, c - 1) op substr($0, c + 2)
    }
    { print }
  ' "$1" >"$4"
}

# premise_of <source> <out>: every `const BITS: u32 = <n>;` outside a comment
# set to the host's pointer width.
premise_of() {
  awk -v w="$HOST_WIDTH" '
    !/^[[:space:]]*\/\// && match($0, /const BITS: u32 = [0-9]+;/) {
      $0 = substr($0, 1, RSTART - 1) "const BITS: u32 = " w ";" substr($0, RSTART + RLENGTH)
    }
    { print }
  ' "$1" >"$2"
}

# spelling_of <source> <out>: every `::core::` outside a comment that is not
# itself a later segment of a path, with its leading `::` dropped.
spelling_of() {
  awk '
    /^[[:space:]]*\/\// { print; next }
    {
      out = ""
      rest = $0
      while (match(rest, /::core::/)) {
        prev = (RSTART > 1) ? substr(rest, RSTART - 1, 1) : substr(out, length(out), 1)
        if (prev ~ /[A-Za-z0-9_:#]/) out = out substr(rest, 1, RSTART - 1) "::core::"
        else out = out substr(rest, 1, RSTART - 1) "core::"
        rest = substr(rest, RSTART + RLENGTH)
      }
      print out rest
    }
  ' "$1" >"$2"
}

# manifest_variant <arm> <suffix>: manifest_rename/<arm>.<suffix>/ holding the
# arm's manifest and lockfile and an empty `src/`, its path printed.
manifest_variant() {
  local dir="manifest_rename/${1}.${2}"
  rm -rf "$dir"
  mkdir -p "${dir}/src"
  cp "manifest_rename/${1}/Cargo.toml" "manifest_rename/${1}/Cargo.lock" "${dir}/"
  printf '%s' "$dir"
}
