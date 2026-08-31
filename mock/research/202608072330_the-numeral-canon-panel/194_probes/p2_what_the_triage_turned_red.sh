#!/usr/bin/env nutshell
# What the triage turned red, named rather than described.
#
# Moving a probe to `uncontrolled` makes every live `measured` or `enumeration`
# claim citing it report. That is the check working and is not to be silenced,
# so this records exactly which claims and leaves them.
#
# Required outcomes, written before the run:
#
#   C1  `the_committed_canon_rests_no_measurement_on_an_unusable_instrument`
#       must FAIL. If it passes, either the triage did not land or the edges
#       were never wired, and either way this file is describing nothing.
#   C2  every other test in the file must pass. A triage that broke a
#       neighbouring arm has done something besides what it says.
#   C3  each reported claim must actually carry an `evidence` entry naming a row
#       that is now `uncontrolled`, confirmed against the registry rather than
#       read out of the failure message, because the message is the thing under
#       test.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../.." && pwd)"
reg="$root/mock/registry"
out="$here/p2_what_the_triage_turned_red.out"

{
  printf '=== p2 what the triage turned red, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  run=$( cd "$root/mock" && cargo test -p arvo-checks --test what_one_field_obliges_another_to_carry 2>&1 || true )

  printf '## C1: the unusable-instrument arm must be red\n'
  if printf '%s' "$run" | grep -q 'the_committed_canon_rests_no_measurement_on_an_unusable_instrument ... FAILED'; then
    printf 'C1 PASS: it is red\n'
  else
    printf 'C1 FAIL: it is not red, so the triage did not reach a wired edge\n'
  fi
  printf '\n'

  printf '## C2: nothing else moved\n'
  nfail=$(printf '%s' "$run" | grep -cE '^test .* FAILED$' || true)
  printf 'failing tests in this file: %s\n' "$nfail"
  printf '%s' "$run" | grep -E '^test .* FAILED$' | sed 's/^/  /'
  if [ "$nfail" -eq 1 ]; then printf 'C2 PASS: exactly the one\n'; else printf 'C2 FAIL\n'; fi
  printf '\n'

  printf '## the probes reported, from the failure\n'
  cited=$(printf '%s' "$run" | grep -oE 'probe::[a-z_]+' | sed 's/probe:://' | sort -u)
  printf '%s\n' "$cited" | sed 's/^/  /'
  printf '\n'

  printf '## C3: each is really `uncontrolled`, and which claim cites it\n'
  bad=0
  while IFS= read -r pr; do
    [ -z "$pr" ] && continue
    st=$(awk -v want="$pr" '
      /^\[\[probe\]\]/{cur=""} /^id = /{cur=$0;sub(/id = "/,"",cur);sub(/"$/,"",cur)}
      /^standing = / && cur==want {v=$0;sub(/.*= "/,"",v);sub(/"$/,"",v);print v}
    ' "$reg/probe.toml")
    claim=$(awk -v want="\"$pr\"" '
      /^\[\[proposal\]\]/{cur=""} /^id = /{cur=$0;sub(/id = "/,"",cur);sub(/"$/,"",cur)}
      /^evidence = /{ if (index($0, want)) print cur }
    ' "$reg"/proposal*.toml)
    if [ "$st" = uncontrolled ]; then
      printf '  probe %s\n    standing: %s\n    cited by: %s\n' "$pr" "$st" "${claim:-NONE FOUND}"
      [ -z "$claim" ] && bad=$((bad+1))
    else
      printf '  C3 FAIL: probe %s reports as unusable but its standing is `%s`\n' "$pr" "$st"; bad=$((bad+1))
    fi
  done <<< "$cited"
  if [ "$bad" -eq 0 ]; then printf 'C3 PASS\n'; else printf 'C3 FAIL: %s\n' "$bad"; fi
  printf '\n'
  printf 'LEFT RED ON PURPOSE. The rows are honest and the edges are somebody\n'
  printf 'else s to move. The repair is on the two claims: either they cite a\n'
  printf 'controlled instrument or they stop being marked as something that ran.\n'
} > "$out" 2>&1
cat "$out"
