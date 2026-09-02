# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fnv1a has the worst median (6.73 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest xxhash3 at 2.14 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### xxhash3 dominates: 215% faster than the next best (fnv1a)

xxhash3 (2.14 us) leads fnv1a (6.73 us) by 215%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### xxhash3 beats baseline by 62% (significant)

xxhash3 is -4.14 us (62%) faster than baseline fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### xxhash3 shows warm-up / thermal drift (autocorr +0.57)

xxhash3's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest xxhash3 (2.14 us) to slowest fnv1a (6.73 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: xxhash3** at 2135.8 ns median (-68.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.15x (fastest 2135.8 ns, slowest 6732.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 7173ns | 6855ns | 6366ns | 6714ns | 9354ns | base |
| xxhash3 | 2200ns | 2216ns | 2066ns | 2216ns | 2285ns | -69.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 7034ns | 6251ns | 9143ns | base | 0.582 |
| xxhash3 | 2120ns | 1990ns | 2203ns | -69.86% | 1.932 |

## Performance model

- Peak throughput: **2.059 Gops/s** (xxhash3; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 0.608 | 29.6% |
| xxhash3 | 1.918 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 7173ns | 7173ns | base |
| xxhash3 | 2200ns | 2200ns | -69.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 6732ns | base | --- | [6294, 6782] | --- | --- | --- | --- |
| xxhash3 | 2136ns | -4629.8ns (-68.8%) | [-4651, -4169]ns | [2132, 2138] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 6781ns | -68.6% |
| 2 | 6778ns | -68.5% |
| 3 | 6785ns | -68.7% |
| 4 | 6780ns | -69.5% |
| 5 | 6782ns | -71.1% |
| 6 | 6780ns | -71.0% |
| 7 | 6784ns | -71.1% |
| 8 | 6686ns | -70.7% |
| 9 | 6497ns | -69.8% |
| 10 | 6274ns | -68.8% |
| 11 | 6791ns | -68.4% |
| 12 | 6783ns | -68.3% |
| 13 | 6258ns | -63.3% |
| 14 | 6278ns | -65.9% |
| 15 | 6296ns | -65.9% |
| 16 | 6293ns | -65.8% |
| 17 | 6293ns | -65.9% |
| 18 | 6285ns | -65.9% |
| 19 | 6306ns | -65.9% |
| 20 | 6256ns | -66.8% |
| 21 | 13662ns | -84.4% |
| 22 | 9186ns | -76.8% |
| 23 | 8173ns | -73.9% |
| 24 | 10869ns | -80.4% |
| 25 | 10345ns | -79.4% |
| 26 | 6682ns | -68.0% |
| 27 | 6258ns | -65.9% |
| 28 | 6262ns | -65.8% |
| 29 | 6243ns | -65.8% |
| 30 | 6247ns | -65.8% |
| 31 | 6248ns | -65.8% |
| 32 | 6248ns | -65.8% |
| 33 | 6249ns | -65.8% |
| 34 | 6660ns | -66.9% |
| 35 | 6782ns | -68.4% |
| 36 | 7004ns | -66.1% |
| 37 | 6843ns | -68.9% |
| 38 | 6782ns | -68.6% |
| 39 | 7065ns | -69.8% |
| 40 | 6782ns | -68.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.431 | moderate+ |
| xxhash3 | 0.569 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **xxhash3**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 6.2ns | 7033.9ns | 0.1% |  |
| xxhash3 | 2.8ns | 2119.8ns | 0.1% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 6250.9-9143.5 ns)
   6250.9 |#################################
   6395.5 |###
   6540.1 |######
   6684.8 |########################################
   6829.4 |###
   6974.0 |######
   7118.7 |
   7263.3 |
   7407.9 |
   7552.6 |
   7697.2 |
   7841.8 |
   7986.4 |
   8131.1 |###
   8275.7 |
   8420.3 |
   8565.0 |
   8709.6 |
   8854.2 |
   8998.9 |
  (5 below, 4 above range)

xxhash3 (n=40, range 1989.5-2202.9 ns)
   1989.5 |
   2000.2 |
   2010.8 |
   2021.5 |
   2032.2 |
   2042.9 |
   2053.5 |
   2064.2 |##
   2074.9 |##
   2085.5 |
   2096.2 |
   2106.9 |
   2117.5 |##
   2128.2 |########################################
   2138.9 |#################
   2149.5 |####
   2160.2 |
   2170.9 |
   2181.5 |
   2192.2 |
  (6 below, 3 above range)

```

## Diagnostics

- **fnv1a**: CV=20.9% (high variance, measurements may be unstable)
- **xxhash3**: autocorrelation=0.57 (measurement drift or warm-up artifact)
