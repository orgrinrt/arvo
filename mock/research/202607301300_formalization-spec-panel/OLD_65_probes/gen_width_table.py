#!/usr/bin/env python3
"""Emit probe_8: the drop-in shape of probe 7 with a FULL per-width impl table
up to MAX, so the table's compile cost can be measured as a curve.

The shape is arvo-strategy's shipped `BitsContainerFor<const N: Width, Sign>`
signature, unchanged, with the bucket selection moved from a const fn into a
`WidthFor<Family>` associated-type table (the 2026-07-28 sketch's move).
"""
import sys
MAX = int(sys.argv[1])
def bkt_hc(n):
    for lim, b in ((8,"B8"),(16,"B16"),(32,"B32"),(64,"B64"),(128,"B128")):
        if n <= lim: return b
    return "BWide<%d>" % -(-n // 8)
def bkt_wp(n):
    for lim, b in ((8,"B16"),(16,"B32"),(32,"B64"),(64,"B128")):
        if n <= lim: return b
    return "BWide<%d>" % -(-n // 8)
head = open("probe_7_strategy_only_dropin.rs").read().split("macro_rules! widths")[0]
head = head.replace("//! Probe 7. Can `arvo-strategy`'s gate come off WITHOUT touching the facade?",
                    "//! Probe 8 (generated). Probe 7's shape with a full per-width table to %d." % MAX)
rows = []
for n in range(1, MAX + 1):
    rows.append("impl WidthFor<HotCold> for Wid<{ Width(%d) }> { type Bkt = %s; }" % (n, bkt_hc(n)))
    rows.append("impl WidthFor<WarmPrecise> for Wid<{ Width(%d) }> { type Bkt = %s; }" % (n, bkt_wp(n)))
tail = open("probe_7_strategy_only_dropin.rs").read().split("pub trait Project<B: Bucket")[1]
out = head + "\n".join(rows) + "\n\npub trait Project<B: Bucket" + tail
open("probe_8_width_table_%d.rs" % MAX, "w").write(out)
print("probe_8_width_table_%d.rs: %d impls" % (MAX, 2 * MAX))
