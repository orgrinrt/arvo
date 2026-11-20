#!/usr/bin/env bash
# The red run of `arvo_mutant_room`, through the crate's mutant runner, into
# this sketch's own outputs.
here=$(cd "$(dirname "$0")" && pwd)
MUTANT_OUT="$here/outputs" exec "$here/../202611140253-gl7r_value-mutants/run.sh" arvo_mutant_room
