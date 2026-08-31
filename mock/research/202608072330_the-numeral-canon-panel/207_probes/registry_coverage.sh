#!/usr/bin/env bash
# Does op's material from the closed panel appear anywhere in the registry.
#
# One line per item, with the distinctive phrases that item would be written in.
# A zero means no row in any namespace uses any of them.
#
# CONTROL, and it is the one this class of check always needs. A grep's
# vocabulary is what fails, not the corpus: the closed panel's own sweep recorded
# six occasions where a first vocabulary returned nothing and a second found the
# idea under another name. So the table below carries CARRIED rows alongside the
# ABSENT ones. A CARRIED row returning zero means the instrument is broken and the
# ABSENT rows mean nothing; the script says so and exits non-zero rather than
# printing a table that reads as a result.
set -uo pipefail

reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

# item | expectation | phrases, pipe-separated, matched case-insensitively
items='
1.1a standard: optimal and ideal|ABSENT|representative of the math|optimal, ideal|the abstractions are what truly matter
1.1c standard: express the conventions|CARRIED|alias over|MATLAB|IEEE 754|SystemC
1.3a stopping condition: op says when|CARRIED|clears the goal|until the full canon is ratified
1.3b the fresh read, transcripts withheld|ABSENT|transcripts withheld|fresh read|deep dive
1.4 post-canon four-phase sequence|ABSENT|source stub|taxonomy round|piece by piece into the stub
1.5 end state: invisible, lowers optimally|ABSENT|lowering transparently|optimal instructions|efficient and ergonomic
1.6 pricing pillar: compile time is nothing|ABSENT|compile time is nothing|long compile times|amortiz
1.7 D54 the axis-sorting test|ABSENT|sorting test|set of representable values changed|the axis is identity
1.8 D56 no gratuitous abbreviation|ABSENT|gratuitous|abbreviation only where
1.9 D53 one numeric type, families alias it|ABSENT|one numeric type|semantic alias|convenience alias
1.10a no enumerations, generally|PARTIAL|no enumeration|enumeration
1.10b container is never consumer-named|ABSENT|strategy guides|user writes strategy|container selection
1.14 downstream evidence is not a need|ABSENT|no better existed|what consumers would ideally
1.15 licence to argue a ratified call|ABSENT|argument is made rather than asserted|free to argue
135b the four-part erasure gate|ABSENT|no caveats left|unacceptable for this design
143b a constant is a function, granular|ABSENT|constant is a function|act granularly|settled canon
138b the aliases are aliases|ABSENT|aliases are aliases|only as aliases to the real
CTL warm behaves as a rust primitive|CARRIED|native primitive|behave like native
CTL the D-numbered decisions are dead|CARRIED|D-number|are dead and are not to be mined
CTL prior calls are a historical log|CARRIED|historical log|failed lineage|lineage that failed
'

fail=0
printf '%-42s %-8s %-6s %s\n' ITEM EXPECT HITS "WHERE"
printf '%-42s %-8s %-6s %s\n' "------------------------------------------" "--------" "------" "-----"
echo "$items" | while IFS='|' read -r name expect rest; do
    [ -z "${name:-}" ] && continue
    hits=0
    where=""
    IFS='|'
    for phrase in $rest; do
        [ -z "$phrase" ] && continue
        c=$(grep -ohi "$phrase" "$reg"/*.toml 2>/dev/null | wc -l | tr -d ' ')
        hits=$((hits + c))
        if [ "$c" -gt 0 ]; then
            w=$(grep -oil "$phrase" "$reg"/*.toml 2>/dev/null | xargs -n1 basename | tr '\n' ' ')
            where="$where$w"
        fi
    done
    unset IFS
    printf '%-42s %-8s %-6s %s\n' "$name" "$expect" "$hits" "$(echo "$where" | tr ' ' '\n' | sort -u | tr '\n' ' ')"
    if [ "$expect" = "CARRIED" ] && [ "$hits" -eq 0 ]; then
        echo "  ^^ CONTROL FAILED: an item known to be carried returned zero; the vocabulary is broken" >&2
        echo BROKEN >>/tmp/207_ctl
    fi
done

if [ -f /tmp/207_ctl ]; then
    rm -f /tmp/207_ctl
    echo >&2
    echo "instrument rejected: at least one CARRIED control returned zero." >&2
    exit 1
fi
echo
echo "controls held: every CARRIED row returned a non-zero count, so a zero above is"
echo "the registry's silence rather than the grep's."
