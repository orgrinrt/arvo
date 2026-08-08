# Probes and commands for file 20

Everything file `20` asserts is either a committed harness artifact in `mock/benches/`, a probe here, or a
count produced by one of the commands below. Toolchain for every probe:
`nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, Apple M1.

Raw output of each probe is committed beside it as `pN_output.txt`. `p3` reproduces across runs: the two
captures in this directory differ in the real arms by the usual few percent and agree on the kernel arm at
0.10 ticks either way.

## The probes

**`p1_counter_nomem_is_not_a_barrier.rs`** is **flawed and kept deliberately**. Its four timing brackets
store into one output, so three of the four stores are dead and LLVM may delete the work behind them. Its
zero readings therefore prove nothing about the counter. It is here because it is why `p2` exists.

```
rustc +nightly-2026-05-28 -O p1_counter_nomem_is_not_a_barrier.rs -o p1 && ./p1
```

**`p2_counter_nomem_isolated.rs`** repairs it: one arm per loop, each with its own live accumulator.

```
rustc +nightly-2026-05-28 -O p2_counter_nomem_isolated.rs -o p2 && ./p2
```

**`p3_dylib_probe/`** loads the shipped variant cdylibs from `mock/target/release/` with `dlopen` and calls
`bench_entry` the way the harness does, timing from outside with a clock the variant cannot influence, and
printing the value the variant wrote. Requires the variants to have been built.

```
cd p3_dylib_probe && cargo run --release
```

**`p4_absorbing_fixpoint.rs`** is the mechanism behind the six void cells. Three predictions, stated in the
file before the run: the saturating fold collapses when its constants are visible, does not collapse behind
`black_box`, and its answer is insensitive to every input element past the third.

```
rustc +nightly-2026-05-28 -O p4_absorbing_fixpoint.rs -o p4 && ./p4
```

## The counts, each reproducible from one command

All run from `mock/benches/`.

Findings files, 147:

```
find . -name '*findings*' -type f | wc -l
```

Runs from a dirty working tree, 146 of 147:

```
python3 -c "
import glob,json
m=[json.load(open(f)) for f in glob.glob('*.meta.json')]
print(len(m), sum(1 for x in m if x['git_commit'].endswith('-dirty')))"
```

Rows carrying a digest or a score, 0 of 55280 for each:

```
python3 -c "
import csv,glob
t=d=s=0
for fn in glob.glob('*.csv'):
    for r in csv.DictReader(open(fn)):
        t+=1
        if r['digest']!='0': d+=1
        if r['score'].strip(): s+=1
print('rows',t,'digest',d,'score',s)"
```

Cells under 100 ns, 12 of 691:

```
python3 -c "
import csv,glob,statistics,collections
hits=tot=0
for fn in glob.glob('*.csv'):
    d=collections.defaultdict(list)
    for r in csv.DictReader(open(fn)):
        if r['mode']=='warm': d[r['variant']].append(float(r['algo_ns']))
    for v,xs in d.items():
        tot+=1
        if statistics.median(xs)<100: hits+=1; print(fn,v,round(statistics.median(xs),2))
print('under 100ns:',hits,'of',tot)"
```

Reported peak throughput per findings file, descending:

```
grep -h "Peak throughput" *_findings.md | sed -E 's/.*\*\*([0-9.]+) Gops.*/\1/' | sort -g -r | head
```

## The width tables

Every median table in file `20` comes from this shape, with the family name substituted:

```
python3 -c "
import csv,glob,statistics,collections,re
fam='warm-container-width-l1'
for fn in sorted(glob.glob(fam+'_n*.csv'), key=lambda s:int(re.search(r'_n(\d+)',s).group(1))):
    d=collections.defaultdict(list)
    for r in csv.DictReader(open(fn)):
        if r['mode']=='warm': d[r['variant']].append(float(r['algo_ns']))
    n=int(re.search(r'_n(\d+)',fn).group(1))
    print(n, {k:round(statistics.median(v)) for k,v in sorted(d.items())})"
```

Key decoding differs per family and is documented in each variant crate's own doc comment.
`warm-container-shared` uses `KEY = W*10000 + NC*1000 + OP*100 + D`; `warm-clamp-shared` uses
`KEY = W*10000 + NC*1000 + LOG2A*10 + OP`. The two schemes are not interchangeable and reading one with the
other is the easiest way to misreport these tables.
