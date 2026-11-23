# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (6.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 3.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum dominates: 13% faster than the next best (warm-container-plusone)

warm-container-minimum (3.08 us) leads warm-container-plusone (3.49 us) by 13%, a clear separation rather than a photo finish. CV 79.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 55% (significant)

warm-container-minimum is -3.74 us (55%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.2x slower than the field

warm-container-headroom (6.82 us) is 2.2x the fastest (3.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum is fastest but the noisiest (CV 79.3%)

warm-container-minimum wins on median (3.08 us) yet has the highest variance (CV 79.3%), while warm-container-headroom is the steadiest (CV 0.1%, 6.82 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native is inconsistent: worst-20% is 3.4x its best-20%

warm-container-native's best 20% of batches run at 3.08 us but its worst 20% at 10.56 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 3081.8 ns median (-54.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.21x (fastest 3081.8 ns, slowest 6821.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6894ns | 6891ns | 6882ns | 6891ns | 6917ns | base |
| warm-container-kernel | 5027ns | 4979ns | 3742ns | 4504ns | 7879ns | -27.09% |
| warm-container-minimum | 4425ns | 3150ns | 2912ns | 3381ns | 9073ns | -35.81% |
| warm-container-native | 5750ns | 3849ns | 3145ns | 4917ns | 10853ns | -16.60% |
| warm-container-plusone | 4558ns | 3569ns | 2994ns | 3650ns | 8845ns | -33.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6820ns | 6813ns | 6827ns | base | 6.005 |
| warm-container-kernel | 4934ns | 3676ns | 7710ns | -27.66% | 8.302 |
| warm-container-minimum | 4320ns | 2845ns | 8829ns | -36.67% | 9.482 |
| warm-container-native | 5617ns | 3078ns | 10562ns | -17.64% | 7.292 |
| warm-container-plusone | 4447ns | 2926ns | 8613ns | -34.79% | 9.210 |

## Performance model

- Peak throughput: **14.395 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.005 | 41.7% |
| warm-container-kernel | 8.363 | 58.1% |
| warm-container-minimum | 13.291 | 92.3% |
| warm-container-native | 10.873 | 75.5% |
| warm-container-plusone | 11.743 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6894ns | 6894ns | base |
| warm-container-kernel | 5027ns | 5027ns | -27.09% |
| warm-container-minimum | 4425ns | 4425ns | -35.81% |
| warm-container-native | 5750ns | 5750ns | -16.60% |
| warm-container-plusone | 4558ns | 4558ns | -33.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6821ns | base | --- | [6820, 6823] | --- | --- | --- | --- |
| warm-container-kernel | 4898ns | -1924.0ns (-28.2%) | [-3133, -1912]ns | [3690, 4908] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3082ns | -3740.2ns (-54.8%) | [-3754, -3068]ns | [3071, 3753] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 3767ns | no significant difference | [-3064, +294]ns | [3756, 7116] | no | 0.1539 | 0.1539 | 0 |
| warm-container-plusone | 3488ns | -3332.3ns (-48.9%) | [-3347, -3081]ns | [3476, 3743] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 6822ns | -28.0% | -54.9% | -44.8% | -49.0% |
| 1 | 2 | 6825ns | -46.1% | -55.0% | -54.9% | -49.1% |
| 1 | 3 | 6821ns | -46.0% | -58.3% | +4.6% | -44.8% |
| 1 | 4 | 6821ns | -28.0% | -44.9% | -44.8% | -58.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.236 | moderate- |
| warm-container-kernel | -0.250 | moderate- |
| warm-container-minimum | -0.306 | moderate- |
| warm-container-native | -0.456 | moderate- |
| warm-container-plusone | -0.366 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 20.1ns | 6820.5ns | 0.3% |  |
| warm-container-kernel | 25.3ns | 4933.9ns | 0.5% |  |
| warm-container-minimum | 28.4ns | 4319.6ns | 0.7% |  |
| warm-container-native | 35.6ns | 5617.1ns | 0.6% |  |
| warm-container-plusone | 30.2ns | 4447.5ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6812.5-6826.6 ns)
   6812.5 |########
   6813.2 |################################
   6813.9 |########
   6814.6 |
   6815.3 |
   6816.0 |################
   6816.8 |
   6817.5 |
   6818.2 |########
   6818.9 |
   6819.6 |
   6820.3 |########################
   6821.0 |########################################
   6821.7 |########################
   6822.4 |################
   6823.1 |
   6823.8 |################
   6824.5 |########################
   6825.2 |########################################
   6825.9 |########
  (4 below, 3 above range)

warm-container-kernel (n=40, range 3675.6-7710.4 ns)
   3675.6 |############################
   3877.4 |##
   4079.1 |##
   4280.8 |
   4482.6 |
   4684.3 |
   4886.1 |########################################
   5087.8 |
   5289.5 |
   5491.3 |
   5693.0 |
   5894.8 |
   6096.5 |
   6298.2 |
   6500.0 |
   6701.7 |
   6903.5 |
   7105.2 |
   7306.9 |
   7508.7 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 2845.5-8828.9 ns)
   2845.5 |########################################
   3144.6 |##
   3443.8 |
   3743.0 |##################
   4042.2 |
   4341.3 |
   4640.5 |
   4939.7 |
   5238.8 |##
   5538.0 |
   5837.2 |
   6136.4 |
   6435.5 |
   6734.7 |##
   7033.9 |
   7333.0 |
   7632.2 |
   7931.4 |
   8230.5 |####
   8529.7 |
  (6 below, 4 above range)

warm-container-native (n=40, range 3078.4-10561.7 ns)
   3078.4 |########
   3452.6 |########################################
   3826.7 |
   4200.9 |##
   4575.1 |##
   4949.2 |
   5323.4 |
   5697.5 |
   6071.7 |
   6445.9 |
   6820.0 |########################
   7194.2 |
   7568.4 |
   7942.5 |
   8316.7 |
   8690.8 |
   9065.0 |
   9439.2 |
   9813.3 |
  10187.5 |#####
  (5 below, 4 above range)

warm-container-plusone (n=40, range 2925.8-8612.7 ns)
   2925.8 |
   3210.1 |########################################
   3494.5 |#############################
   3778.8 |
   4063.1 |
   4347.5 |
   4631.8 |
   4916.2 |
   5200.5 |
   5484.9 |
   5769.2 |##
   6053.6 |
   6337.9 |
   6622.3 |
   6906.6 |##
   7191.0 |
   7475.3 |
   7759.7 |
   8044.0 |
   8328.4 |##
  (7 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=41.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=56.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-container-native**: CV=53.0% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-plusone**: CV=55.0% (high variance, measurements may be unstable)
