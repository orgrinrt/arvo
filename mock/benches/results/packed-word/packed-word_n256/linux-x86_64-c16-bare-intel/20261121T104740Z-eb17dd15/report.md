# A packed word of named fields, through Word and by hand on the host integer

2 variants, 40 samples per variant.
Baseline: **packed_word_host**

## Highlights

Baseline for all deltas below: **packed_word_host**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (packed_word_host) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline packed_word_host has the worst median (171 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest packed_word_word at 132 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### packed_word_word is fastest but the noisiest (CV 63.5%)

packed_word_word wins on median (132 ns) yet has the highest variance (CV 63.5%), while packed_word_host is the steadiest (CV 52.5%, 171 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (39 ns) is smaller than the fastest variant's own run-to-run std-dev (84 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### packed_word_word is inconsistent: worst-20% is 2.5x its best-20%

packed_word_word's best 20% of batches run at 120 ns but its worst 20% at 295 ns (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: packed_word_word** at 132.1 ns median (-22.8% vs baseline)
- Spread: 1.29x (fastest 132.1 ns, slowest 170.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A packed word of named fields, through Word and by hand on the host integer |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18ea09bf3f51ebfb |
| Counter frequency | 2112871953 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| packed_word_host | 546ns | 568ns | 361ns | 523ns | 799ns | base |
| packed_word_word | 507ns | 427ns | 350ns | 470ns | 772ns | -7.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| packed_word_host | 186ns | 122ns | 291ns | base | 1.377 |
| packed_word_word | 175ns | 120ns | 295ns | -5.95% | 1.464 |

## Performance model

- Peak throughput: **2.140 Gops/s** (packed_word_word; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| packed_word_host | 1.498 | 70.0% |
| packed_word_word | 1.939 | 90.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| packed_word_host | 546ns | 546ns | base |
| packed_word_word | 507ns | 507ns | -7.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| packed_word_host | 171ns | base | --- | [146, 196] | --- | --- | --- | --- |
| packed_word_word | 132ns | no significant difference | [-27, +1]ns | [131, 147] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | packed_word_host | packed_word_word |
|---|---|---|---|
| 1 | 1 | 196ns | -32.9% |
| 1 | 2 | 235ns | +0.2% |
| 1 | 3 | 122ns | +20.3% |
| 1 | 4 | 147ns | -19.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| packed_word_host | -0.055 | ok |
| packed_word_word | -0.295 | moderate- |

**Consistency summary:**

- **packed_word_word**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| packed_word_host | 134.5ns | 185.9ns | 72.3% | HIGH |
| packed_word_word | 138.4ns | 174.9ns | 79.2% | HIGH |

## Distribution (algo ns)

```
packed_word_host (n=40, range 121.9-290.6 ns)
    121.9 |########
    130.3 |################
    138.8 |################################
    147.2 |####
    155.6 |
    164.1 |
    172.5 |
    180.9 |
    189.4 |########################################
    197.8 |
    206.2 |
    214.7 |
    223.1 |
    231.6 |####################################
    240.0 |
    248.4 |
    256.9 |
    265.3 |
    273.7 |
    282.2 |
  (5 below, 1 above range)

packed_word_word (n=40, range 119.6-295.4 ns)
    119.6 |##
    128.4 |########################################
    137.2 |
    146.0 |####################
    154.8 |
    163.6 |
    172.4 |
    181.1 |
    189.9 |
    198.7 |
    207.5 |
    216.3 |
    225.1 |
    233.9 |############################
    242.7 |
    251.4 |
    260.2 |
    269.0 |
    277.8 |
    286.6 |
  (6 below, 2 above range)

```

## Diagnostics

- **packed_word_host**: CV=48.2% (high variance, measurements may be unstable)
- **packed_word_host**: bridge=77.6% of algo (FFI overhead may distort results)
- **packed_word_word**: CV=48.0% (high variance, measurements may be unstable)
- **packed_word_word**: bridge=70.2% of algo (FFI overhead may distort results)
