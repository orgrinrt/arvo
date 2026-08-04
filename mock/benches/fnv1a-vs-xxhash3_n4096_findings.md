# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fnv1a has the worst median (5.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest xxhash3 at 1.96 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### xxhash3 dominates: 203% faster than the next best (fnv1a)

xxhash3 (1.96 us) leads fnv1a (5.94 us) by 203%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### xxhash3 beats baseline by 68% (significant)

xxhash3 is -4.03 us (68%) faster than baseline fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### xxhash3 is fastest but the noisiest (CV 6.4%)

xxhash3 wins on median (1.96 us) yet has the highest variance (CV 6.4%), while fnv1a is the steadiest (CV 4.7%, 5.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.0x the fastest

Fastest xxhash3 (1.96 us) to slowest fnv1a (5.94 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: xxhash3** at 1962.7 ns median (-67.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.03x (fastest 1962.7 ns, slowest 5941.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 6180ns | 6052ns | 5946ns | 6120ns | 6597ns | base |
| xxhash3 | 2000ns | 2037ns | 1851ns | 1996ns | 2158ns | -67.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 6064ns | 5839ns | 6462ns | base | 0.675 |
| xxhash3 | 1927ns | 1785ns | 2077ns | -68.22% | 2.125 |

## Performance model

- Peak throughput: **2.295 Gops/s** (xxhash3; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 0.689 | 30.0% |
| xxhash3 | 2.087 | 90.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 6180ns | 6180ns | base |
| xxhash3 | 2000ns | 2000ns | -67.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 5941ns | base | --- | [5840, 6200] | --- | --- | --- | --- |
| xxhash3 | 1963ns | -4105.6ns (-69.1%) | [-4274, -3997]ns | [1846, 1966] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 5839ns | -68.5% |
| 2 | 5838ns | -68.5% |
| 3 | 5840ns | -68.5% |
| 4 | 5840ns | -68.4% |
| 5 | 5841ns | -68.4% |
| 6 | 5840ns | -68.4% |
| 7 | 5839ns | -68.3% |
| 8 | 5840ns | -68.4% |
| 9 | 5840ns | -68.5% |
| 10 | 5841ns | -68.5% |
| 11 | 5841ns | -70.9% |
| 12 | 6069ns | -71.9% |
| 13 | 5841ns | -70.8% |
| 14 | 5840ns | -69.0% |
| 15 | 6820ns | -71.2% |
| 16 | 6400ns | -69.3% |
| 17 | 6342ns | -68.9% |
| 18 | 5840ns | -66.3% |
| 19 | 5838ns | -66.3% |
| 20 | 5841ns | -66.4% |
| 21 | 6245ns | -61.8% |
| 22 | 6142ns | -68.0% |
| 23 | 5840ns | -61.4% |
| 24 | 6148ns | -68.1% |
| 25 | 5839ns | -66.4% |
| 26 | 5840ns | -64.8% |
| 27 | 6041ns | -67.5% |
| 28 | 5840ns | -65.2% |
| 29 | 6160ns | -70.1% |
| 30 | 6059ns | -69.6% |
| 31 | 6244ns | -68.6% |
| 32 | 6240ns | -68.5% |
| 33 | 6243ns | -68.6% |
| 34 | 6246ns | -68.6% |
| 35 | 6245ns | -68.4% |
| 36 | 6243ns | -68.5% |
| 37 | 6247ns | -68.4% |
| 38 | 6370ns | -69.0% |
| 39 | 6246ns | -68.5% |
| 40 | 7026ns | -72.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.388 | moderate+ |
| xxhash3 | 0.475 | moderate+ |

**Consistency summary:**

- **xxhash3**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 3.0ns | 6064.4ns | 0.0% |  |
| xxhash3 | 2.2ns | 1927.3ns | 0.1% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 5838.9-6462.3 ns)
   5838.9 |########################################
   5870.1 |
   5901.2 |
   5932.4 |
   5963.6 |
   5994.8 |
   6025.9 |##
   6057.1 |#####
   6088.3 |
   6119.4 |#####
   6150.6 |##
   6181.8 |
   6213.0 |#######
   6244.1 |###############
   6275.3 |
   6306.5 |
   6337.6 |##
   6368.8 |##
   6400.0 |##
   6431.2 |
  (4 below, 2 above range)

xxhash3 (n=40, range 1785.0-2077.5 ns)
   1785.0 |
   1799.6 |##
   1814.2 |
   1828.9 |##############
   1843.5 |##########
   1858.1 |
   1872.7 |
   1887.4 |
   1902.0 |
   1916.6 |
   1931.2 |
   1945.9 |
   1960.5 |########################################
   1975.1 |##
   1989.7 |
   2004.4 |
   2019.0 |##
   2033.6 |
   2048.2 |##
   2062.9 |
  (3 below, 2 above range)

```
