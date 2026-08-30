#!/bin/sh
# Build and run r2 at both settings of the flag clause 1 does not name.
set -e
for m in off on; do
  rustc -Copt-level=1 -Cdebug-assertions=$m -o /tmp/r2_$m r2_clause1_needs_the_profile_bound.rs 2>/dev/null
  /tmp/r2_$m
  echo
done
