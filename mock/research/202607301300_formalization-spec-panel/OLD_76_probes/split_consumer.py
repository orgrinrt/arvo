#!/usr/bin/env python3
"""Split a generated arm into a machinery crate and a consumer crate.

The split point is the first per-numeral declaration. Everything above it is
what would live in arvo; everything below is what a consumer writes. The
consumer crate names the machinery through `mach::`, so the measurement is of
instantiation against an already-built rlib rather than of compiling the
library again.
"""
import re
import sys

whole, mach_path, user_path = sys.argv[1:4]
src = open(whole).read().split("\n")

# The consumer half starts at the first item the generator emits per numeral.
cut = next(i for i, l in enumerate(src)
           if re.match(r"pub (type (Ib0|N0)|const W0)\b", l))
mach_lines, user_lines = src[:cut], src[cut:]

attrs = [l for l in mach_lines if l.startswith("#!")]
mach_body = [l for l in mach_lines if not l.startswith("#!")]

open(mach_path, "w").write("\n".join(attrs + mach_body) + "\n")

# The consumer imports everything the machinery exports by name. A glob keeps
# the consumer text identical to the single-file arm, so the two measurements
# differ only in where the machinery was compiled.
head = attrs + ["use mach::*;"]
open(user_path, "w").write("\n".join(head + user_lines) + "\n")
