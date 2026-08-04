# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fnv1a has the worst median (309 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest xxhash3 at 128 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### xxhash3 dominates: 141% faster than the next best (fnv1a)

xxhash3 (128 ns) leads fnv1a (309 ns) by 141%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### xxhash3 beats baseline by 58% (significant)

xxhash3 is -180 ns (58%) faster than baseline fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### xxhash3 is fastest but the noisiest (CV 5.6%)

xxhash3 wins on median (128 ns) yet has the highest variance (CV 5.6%), while fnv1a is the steadiest (CV 3.8%, 309 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### xxhash3 shows warm-up / thermal drift (autocorr +0.90)

xxhash3's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: xxhash3** at 128.3 ns median (-58.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.41x (fastest 128.3 ns, slowest 309.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 422ns | 424ns | 407ns | 422ns | 439ns | base |
| xxhash3 | 199ns | 199ns | 184ns | 198ns | 215ns | -52.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 309ns | 296ns | 323ns | base | 0.827 |
| xxhash3 | 129ns | 119ns | 140ns | -58.42% | 1.990 |

## Performance model

- Peak throughput: **2.153 Gops/s** (xxhash3; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 0.827 | 38.4% |
| xxhash3 | 1.995 | 92.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 422ns | 422ns | base |
| xxhash3 | 199ns | 199ns | -52.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 309ns | base | --- | [299, 319] | --- | --- | --- | --- |
| xxhash3 | 128ns | -180.6ns (-58.4%) | [-182, -179]ns | [127, 129] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 317ns | -59.5% |
| 2 | 319ns | -60.5% |
| 3 | 320ns | -59.8% |
| 4 | 319ns | -59.9% |
| 5 | 319ns | -59.6% |
| 6 | 320ns | -60.2% |
| 7 | 319ns | -59.8% |
| 8 | 318ns | -60.1% |
| 9 | 319ns | -59.9% |
| 10 | 321ns | -60.1% |
| 11 | 298ns | -60.0% |
| 12 | 299ns | -60.7% |
| 13 | 299ns | -60.2% |
| 14 | 302ns | -60.4% |
| 15 | 300ns | -60.1% |
| 16 | 301ns | -60.6% |
| 17 | 300ns | -59.5% |
| 18 | 300ns | -59.4% |
| 19 | 299ns | -59.7% |
| 20 | 300ns | -60.8% |
| 21 | 299ns | -58.0% |
| 22 | 297ns | -56.8% |
| 23 | 298ns | -56.0% |
| 24 | 297ns | -56.5% |
| 25 | 299ns | -57.1% |
| 26 | 295ns | -56.5% |
| 27 | 295ns | -56.8% |
| 28 | 295ns | -57.1% |
| 29 | 296ns | -56.5% |
| 30 | 298ns | -57.0% |
| 31 | 318ns | -56.3% |
| 32 | 323ns | -56.1% |
| 33 | 320ns | -56.7% |
| 34 | 320ns | -56.4% |
| 35 | 320ns | -56.1% |
| 36 | 320ns | -55.7% |
| 37 | 342ns | -59.2% |
| 38 | 319ns | -56.1% |
| 39 | 321ns | -57.3% |
| 40 | 319ns | -57.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.800 | HIGH+ (drift/warm-up) |
| xxhash3 | 0.902 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **xxhash3**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 4.3ns | 309.5ns | 1.4% |  |
| xxhash3 | 2.4ns | 128.7ns | 1.9% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 296.4-323.4 ns)
    296.4 |###########
    297.8 |########################################
    299.1 |############################
    300.5 |###########
    301.8 |
    303.2 |
    304.5 |
    305.9 |
    307.2 |
    308.6 |
    309.9 |
    311.3 |
    312.6 |
    314.0 |
    315.3 |
    316.7 |#################
    318.0 |########################################
    319.4 |##################################
    320.7 |###########
    322.1 |#####
  (4 below, 1 above range)

xxhash3 (n=40, range 118.9-140.0 ns)
    118.9 |############
    120.0 |####
    121.0 |########
    122.1 |
    123.1 |
    124.2 |
    125.3 |########
    126.3 |############
    127.4 |########################################
    128.4 |################
    129.5 |
    130.5 |####
    131.6 |
    132.6 |
    133.7 |
    134.7 |####
    135.8 |
    136.8 |####
    137.9 |########
    139.0 |############
  (4 below, 3 above range)

```

## Diagnostics

- **fnv1a**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **xxhash3**: autocorrelation=0.90 (measurement drift or warm-up artifact)
