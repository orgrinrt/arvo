#!/usr/bin/env bash
# The control behind the loudest claim in the deliverable: that 966 API
# positions across the stack name an arvo crate or type that arvo's `dev` does
# not have.
#
# Two halves, and neither is enough alone. What arvo publishes today, and what
# the consumers pin and import. A consumer pinning an older revision would make
# the first half irrelevant, which is why the pin is read rather than assumed.
set -euo pipefail
W="${1:?usage: <workspace root>}"

echo "== the crates arvo has on dev =="
git -C "$W/arvo" ls-tree --name-only origin/dev mock/crates/

echo
echo "== the arvo crates the consumers depend on =="
for r in hilavitkutin:origin/dev vehje:origin/main kolli:origin/dev; do
  repo="${r%%:*}"; ref="${r##*:}"
  echo "-- $repo @ $ref"
  git -C "$W/$repo" grep -hE '^arvo[a-z-]* *=' "$ref" -- '*/Cargo.toml' \
    | sed 's/ \+/ /g' | sort -u
done

echo
echo "== what they import, by name =="
for r in hilavitkutin:origin/dev vehje:origin/main kolli:origin/dev; do
  repo="${r%%:*}"; ref="${r##*:}"
  echo "-- $repo @ $ref"
  git -C "$W/$repo" grep -h 'use arvo' "$ref" -- '*/src/*.rs' \
    | sed 's/^[^:]*://' | sort -u
done
