#!/usr/bin/env nutshell
# Probe: does the ratified count of ten coordinates fix the slot arity at one?
#
# `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
# is ratified, says the door carries out "the coordinate set of the ratified
# parameterisation", and its reasoning counts "the ten associated constants" and
# partitions them: six declared in types a `u32` bit count cannot hold, of which
# three ship values it cannot hold, naming `Quantum::SLOPE`, `Format::PHASE_DEN`,
# `BASE` and `MIN`.
#
# If that ten reconstructs from the shipped traits, and the slot coordinate inside
# it is a scalar triple, then the ratified count fixes the slot arity at one, and
# a vector-valued affine predicate is refused by a ratified count rather than by a
# reading of a spelling.
#
# THE FIRST VERSION OF THIS PROBE WAS WRONG AND IS KEPT AS THE REASON FOR THE
# CONTROLS. Its declaration regex matched impl bodies as well as trait
# declarations, so it reported 22 coordinates while its own summary asserted the
# ten reconstructed. It did not. Two controls now stand between the count and the
# claim, and the claim is printed only if both hold.
#
# The cases that must fail, stated before the run:
#   control A: the partition must come out at six, which is the ruling's own figure.
#   control B: the reconstructed coordinate count must be ten. If it is anything
#              else the reconstruction is not the one the ruling counted, and the
#              arity argument does not run.

CRATE="../../../../crates/arvo-format/src"
FILES="$CRATE/ambient.rs $CRATE/quantum.rs $CRATE/slots.rs $CRATE/format.rs"

# A trait DECLARATION is `const NAME: Type;`. An impl is `const NAME: Type = ...`.
# Requiring the semicolon and forbidding `=` is what separates them, and getting
# this wrong is what broke the first version.
decls() { grep -hE '^    const [A-Z_]+: [^=]+;$' $FILES; }

echo "=== associated const DECLARATIONS on the four coordinate traits ==="
decls | sed 's/^    //'
echo
echo "=== excluded, and why ==="
grep -hE '^    const ADMITTED' $FILES | sed 's/^    /  /'
echo '  ^ the admission obligation, not a coordinate: it is a unit, it carries no'
echo "    value, and it is what every use site forces rather than something a"
echo "    candidate chooses."
grep -hcE '^    const [A-Z_]+: [^;]+= ' $FILES | paste -sd+ - | bc \
    | xargs -I{} echo "  {} impl-side definitions, which are values of coordinates, not coordinates"
echo

coords=$(decls | wc -l | tr -d ' ')
signed_or_wide=$(decls | grep -cE ': (i64|i32);$')
rest=$((coords - signed_or_wide))

echo "coordinates                    : $coords"
echo "  declared i64 or i32          : $signed_or_wide"
echo "  the rest (u32, bool, Width)  : $rest"
echo

fail=0
if [ "$signed_or_wide" -ne 6 ]; then
    echo "CONTROL A FAILED: the ruling's six does not reproduce ($signed_or_wide)."
    fail=1
else
    echo "control A passed: the ruling's six reproduces."
fi
if [ "$coords" -ne 10 ]; then
    echo "CONTROL B FAILED: the ratified ten does not reconstruct ($coords)."
    fail=1
else
    echo "control B passed: the ratified ten reconstructs."
fi
echo
if [ "$fail" -ne 0 ]; then
    echo "NO FINDING. The reconstruction is not the ruling's, so the arity argument"
    echo "does not run and nothing below would mean anything."
    exit 1
fi

echo "=== the slot coordinate, and what an arity of n would cost ==="
echo "slot axis      : MIN, MAX, WIDTH                                    -> 3 per axis"
echo "everything else: RADIX, SIGNED, BASE, SLOPE, MAGNITUDES, PHASE_NUM, PHASE_DEN -> 7"
for n in 1 2 3 4; do
    echo "  slot arity $n -> $((3 * n + 7)) coordinates"
done
echo

echo "FINDING: the ratified ten reconstructs exactly, as 3n + 7 at n = 1, and the"
echo "partition into six and four is the ruling's own. An affine predicate of slot"
echo "arity 2 would need thirteen coordinates and of arity 3 sixteen, so the door"
echo "would not carry ten. The slot arity is therefore fixed at one by a ratified"
echo "COUNT rather than by a reading of the membership sentence's spelling, and the"
echo "open question of whether 'one parameterisation' meant one scalar does not have"
echo "to be settled on the wording: the count settles it."
