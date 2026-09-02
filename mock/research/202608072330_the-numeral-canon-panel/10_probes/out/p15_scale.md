# p15 scale arms: how they were generated, and what they measured

The 513-row source is committed as `../p15_dense_bridge.rs`. The 2049-row and 8193-row arms are the
same file with the row list regenerated; they are not committed because they are 120K and 532K of
mechanically generated `N => tower` lines carrying no information the generator does not.

Regenerate either arm with, from `10_probes/`:

```python
def tower(n):
    if n == 0: return "Term"
    b = []
    while n: b.append(n & 1); n >>= 1
    t = "Term"
    for x in reversed(b): t = ("D1<" if x else "D0<") + t + ">"
    return t
N = 2048                      # or 8192
s = open("p15_dense_bridge.rs").read()
head, tail = s.split("bridge! {")[0], s[s.index("\n}\n") + 3:]
rows = "\n".join("    %d => %s," % (i, tower(i)) for i in range(0, N + 1))
open("out/p15_%d.rs" % N, "w").write(head + "bridge! {\n" + rows + "\n}\n" + tail)
```

then

```
/usr/bin/time -p rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
    --emit=metadata -o out/p15_$N.meta out/p15_$N.rs
```

Observed, and this is an ad-hoc quick spike with no substance rather than a benchmark: 513 rows 0.10 s,
2049 rows 0.61 s, 8193 rows 3.11 s, all exit 0. The claim it supports is existence, that a dense bridge
compiles gate-free at these row counts. It prices nothing, because no harness ran.
