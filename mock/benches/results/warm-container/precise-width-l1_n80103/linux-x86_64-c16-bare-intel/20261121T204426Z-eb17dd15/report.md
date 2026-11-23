# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 5299% faster than the next best (warm-container-native)

warm-container-kernel (54 ns) leads warm-container-native (2.93 us) by 5299%, a clear separation rather than a photo finish. CV 61.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -5.00 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 100.8x slower than the field

warm-container-plusone (5.48 us) is 100.8x the fastest (54 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.69)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} (5299% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 5299% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 100.8x the fastest

Fastest warm-container-kernel (54 ns) to slowest warm-container-plusone (5.48 us): 100.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 3.5x its best-20%

warm-container-minimum's best 20% of batches run at 2.51 us but its worst 20% at 8.82 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 54.4 ns median (-98.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 100.82x (fastest 54.4 ns, slowest 5479.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112691662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6033ns | 5131ns | 5016ns | 5556ns | 8482ns | base |
| warm-container-kernel | 376ns | 322ns | 265ns | 341ns | 592ns | -93.77% |
| warm-container-minimum | 4153ns | 3125ns | 2556ns | 3052ns | 9050ns | -31.17% |
| warm-container-native | 4283ns | 2998ns | 2654ns | 3249ns | 9012ns | -29.01% |
| warm-container-plusone | 5628ns | 5538ns | 5459ns | 5531ns | 6089ns | -6.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5964ns | 4958ns | 8387ns | base | 5.494 |
| warm-container-kernel | 70ns | 49ns | 120ns | -98.83% | 468.114 |
| warm-container-minimum | 4059ns | 2505ns | 8815ns | -31.94% | 8.073 |
| warm-container-native | 4182ns | 2598ns | 8771ns | -29.87% | 7.835 |
| warm-container-plusone | 5561ns | 5400ns | 5997ns | -6.76% | 5.892 |

## Performance model

- Peak throughput: **664.834 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.460 | 1.0% |
| warm-container-kernel | 602.907 | 90.7% |
| warm-container-minimum | 10.702 | 1.6% |
| warm-container-native | 11.167 | 1.7% |
| warm-container-plusone | 5.980 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6033ns | 6033ns | base |
| warm-container-kernel | 376ns | 376ns | -93.77% |
| warm-container-minimum | 4153ns | 4153ns | -31.17% |
| warm-container-native | 4283ns | 4283ns | -29.01% |
| warm-container-plusone | 5628ns | 5628ns | -6.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5073ns | base | --- | [5051, 5916] | --- | --- | --- | --- |
| warm-container-kernel | 54ns | -4999.8ns (-98.6%) | [-5864, -4948]ns | [52, 65] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3062ns | -2482.1ns (-48.9%) | [-2549, -2027]ns | [2795, 3074] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-native | 2934ns | -2086.1ns (-41.1%) | [-2170, -2022]ns | [2882, 3028] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 5479ns | no significant difference | [-513, +412]ns | [5455, 5489] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 5059ns | -98.0% | -39.1% | -41.7% | +7.9% |
| 1 | 2 | 8122ns | -99.4% | -62.2% | -67.4% | -32.4% |
| 1 | 3 | 5089ns | -99.0% | -39.6% | -41.5% | +6.1% |
| 1 | 4 | 4994ns | -98.7% | -49.8% | -40.8% | +10.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.398 | moderate- |
| warm-container-kernel | -0.141 | ok |
| warm-container-minimum | -0.087 | ok |
| warm-container-native | -0.428 | moderate- |
| warm-container-plusone | -0.692 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 21.5ns | 5963.9ns | 0.4% |  |
| warm-container-kernel | 110.2ns | 70.0ns | 157.5% | HIGH |
| warm-container-minimum | 31.7ns | 4058.8ns | 0.8% |  |
| warm-container-native | 34.0ns | 4182.4ns | 0.8% |  |
| warm-container-plusone | 20.3ns | 5561.0ns | 0.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 4958.2-8386.7 ns)
   4958.2 |########################################
   5129.6 |##
   5301.0 |
   5472.5 |##
   5643.9 |
   5815.3 |
   5986.7 |
   6158.2 |######
   6329.6 |##
   6501.0 |
   6672.4 |
   6843.9 |##
   7015.3 |
   7186.7 |
   7358.1 |
   7529.6 |
   7701.0 |
   7872.4 |
   8043.8 |##############
   8215.3 |
  (4 below, 2 above range)

warm-container-kernel (n=40, range 49.3-119.7 ns)
     49.3 |########################################
     52.8 |
     56.3 |#######
     59.9 |##
     63.4 |###############
     66.9 |
     70.4 |
     73.9 |
     77.5 |
     81.0 |
     84.5 |
     88.0 |
     91.6 |
     95.1 |
     98.6 |
    102.1 |######################
    105.6 |
    109.2 |
    112.7 |
    116.2 |
  (4 below, 1 above range)

warm-container-minimum (n=40, range 2505.1-8815.5 ns)
   2505.1 |######################
   2820.6 |########################################
   3136.2 |
   3451.7 |##
   3767.2 |##
   4082.7 |
   4398.2 |
   4713.7 |
   5029.3 |
   5344.8 |
   5660.3 |
   5975.8 |
   6291.3 |
   6606.8 |
   6922.4 |##
   7237.9 |##
   7553.4 |
   7868.9 |##
   8184.4 |##
   8499.9 |
  (5 below, 4 above range)

warm-container-native (n=40, range 2598.1-8770.7 ns)
   2598.1 |########################################
   2906.8 |##############################
   3215.4 |
   3524.0 |######
   3832.7 |
   4141.3 |###
   4449.9 |
   4758.5 |
   5067.2 |
   5375.8 |
   5684.4 |
   5993.1 |###
   6301.7 |
   6610.3 |
   6919.0 |
   7227.6 |######
   7536.2 |###
   7844.8 |
   8153.5 |
   8462.1 |
  (5 below, 5 above range)

warm-container-plusone (n=40, range 5399.9-5996.6 ns)
   5399.9 |################
   5429.7 |#######################
   5459.6 |##############################
   5489.4 |########################################
   5519.2 |
   5549.1 |
   5578.9 |
   5608.7 |
   5638.6 |
   5668.4 |
   5698.2 |
   5728.1 |
   5757.9 |
   5787.8 |
   5817.6 |
   5847.4 |
   5877.3 |
   5907.1 |
   5936.9 |
   5966.8 |
  (5 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=23.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=47.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=165.8% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=60.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-native**: CV=58.0% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.4x (possible bimodal distribution)
