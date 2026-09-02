#!/usr/bin/env bash
# The run behind every number in the deliverable. From arvo's repo root, with
# the stack cloned as siblings of it.
#
# Each tree names a ref rather than a path alone, so the run does not depend on
# what anybody has checked out. The refs are resolved and printed by the tool
# itself, and the oids are in `all.out` beside this.
set -euo pipefail
W="${1:?usage: run.sh <workspace root holding the clones>}"
cargo mock the-positions \
  "$W/notko@origin/dev" \
  "$W/arvo@origin/dev" \
  "$W/hilavitkutin@origin/dev" \
  "$W/vehje@origin/main" \
  "$W/kolli@origin/dev" \
  "$W/tarina@origin/main" "${@:2}"
