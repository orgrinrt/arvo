# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 1.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 34% (significant)

warm-container-lanes-deferred is -1.38 us (34%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.1x slower than the field

warm-container-headroom (4.10 us) is 2.1x the fastest (1.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-minimum (1.98 us) and warm-container-lanes-deferred (1.98 us) differ by 0.09%, inside the noise, even though the wider field spreads 106.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.64)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.11 us) is smaller than the fastest variant's own run-to-run std-dev (2.15 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-lanes-deferred} vs {warm-container-native, warm-container-plusone, warm-container-kernel, warm-container-headroom} (42% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-lanes-deferred} and a slow tier {warm-container-native, warm-container-plusone, warm-container-kernel, warm-container-headroom} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.1x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.71 us but its worst 20% at 7.07 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1982.7 ns median (-51.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.07x (fastest 1982.7 ns, slowest 4097.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112700793 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4492ns | 4180ns | 3418ns | 3964ns | 7151ns | base |
| warm-container-kernel | 3771ns | 3640ns | 1692ns | 3570ns | 6450ns | -16.06% |
| warm-container-lanes-deferred | 3755ns | 2066ns | 1780ns | 3192ns | 7421ns | -16.40% |
| warm-container-minimum | 3603ns | 2068ns | 1825ns | 2930ns | 7399ns | -19.80% |
| warm-container-native | 4030ns | 2936ns | 1919ns | 3644ns | 7299ns | -10.28% |
| warm-container-plusone | 4581ns | 3434ns | 3422ns | 3714ns | 8340ns | +1.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4397ns | 3351ns | 6974ns | base | 7.453 |
| warm-container-kernel | 3612ns | 1625ns | 6150ns | -17.85% | 9.073 |
| warm-container-lanes-deferred | 3594ns | 1709ns | 7070ns | -18.26% | 9.118 |
| warm-container-minimum | 3448ns | 1753ns | 7049ns | -21.57% | 9.502 |
| warm-container-native | 3858ns | 1843ns | 6967ns | -12.24% | 8.493 |
| warm-container-plusone | 4480ns | 3355ns | 8126ns | +1.90% | 7.314 |

## Performance model

- Peak throughput: **20.165 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.998 | 39.7% |
| warm-container-kernel | 9.377 | 46.5% |
| warm-container-lanes-deferred | 16.512 | 81.9% |
| warm-container-minimum | 16.527 | 82.0% |
| warm-container-native | 11.619 | 57.6% |
| warm-container-plusone | 9.735 | 48.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4492ns | 4492ns | base |
| warm-container-kernel | 3771ns | 3771ns | -16.06% |
| warm-container-lanes-deferred | 3755ns | 3755ns | -16.40% |
| warm-container-minimum | 3603ns | 3603ns | -19.80% |
| warm-container-native | 4030ns | 4030ns | -10.28% |
| warm-container-plusone | 4581ns | 4581ns | +1.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4097ns | base | --- | [3364, 4104] | --- | --- | --- | --- |
| warm-container-kernel | 3494ns | no significant difference | [-1737, +918]ns | [1989, 4863] | no | 0.1923 | 0.1539 | 0 |
| warm-container-lanes-deferred | 1984ns | -1381.1ns (-33.7%) | [-2100, -1360]ns | [1983, 3828] | YES | 0.0161 | 0.0064 | 0 |
| warm-container-minimum | 1983ns | -1374.8ns (-33.6%) | [-1802, -1369]ns | [1981, 3678] | YES | 0.0111 | 0.0022 | 0 |
| warm-container-native | 2820ns | no significant difference | [-1380, +197]ns | [1991, 4995] | no | 0.1345 | 0.0807 | 0 |
| warm-container-plusone | 3366ns | no significant difference | [-9, +6]ns | [3357, 4101] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4099ns | -9.0% | -60.3% | -51.5% | +13.8% | +0.1% |
| 1 | 2 | 4104ns | -51.3% | -51.2% | -22.3% | -31.3% | -18.2% |
| 1 | 3 | 3363ns | +35.8% | -40.8% | -40.9% | -40.9% | -0.1% |
| 1 | 4 | 3355ns | -1.6% | -41.1% | -40.9% | -40.5% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.251 | moderate+ |
| warm-container-kernel | -0.642 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.122 | ok |
| warm-container-minimum | -0.413 | moderate- |
| warm-container-native | 0.152 | ok |
| warm-container-plusone | -0.086 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 26.5ns | 4396.5ns | 0.6% |  |
| warm-container-kernel | 44.4ns | 3611.7ns | 1.2% |  |
| warm-container-lanes-deferred | 45.8ns | 3593.8ns | 1.3% |  |
| warm-container-minimum | 42.6ns | 3448.4ns | 1.2% |  |
| warm-container-native | 47.4ns | 3858.4ns | 1.2% |  |
| warm-container-plusone | 27.2ns | 4480.2ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3351.2-6973.8 ns)
   3351.2 |####################
   3532.3 |
   3713.5 |
   3894.6 |
   4075.7 |########################################
   4256.9 |
   4438.0 |
   4619.1 |
   4800.3 |
   4981.4 |
   5162.5 |##
   5343.7 |
   5524.8 |
   5705.9 |
   5887.0 |
   6068.2 |
   6249.3 |
   6430.4 |
   6611.6 |
   6792.7 |##
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1625.0-6150.1 ns)
   1625.0 |########################################
   1851.2 |##################################
   2077.5 |
   2303.7 |#####
   2530.0 |#####
   2756.3 |
   2982.5 |
   3208.8 |
   3435.0 |
   3661.3 |
   3887.5 |
   4113.8 |
   4340.0 |###########
   4566.3 |#################
   4792.5 |###########
   5018.8 |#################
   5245.0 |#################
   5471.3 |###########
   5697.5 |###########
   5923.8 |
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1708.5-7070.3 ns)
   1708.5 |####################
   1976.6 |########################################
   2244.7 |
   2512.8 |
   2780.9 |
   3049.0 |###
   3317.1 |
   3585.1 |###
   3853.2 |###
   4121.3 |
   4389.4 |
   4657.5 |
   4925.6 |
   5193.7 |###
   5461.8 |
   5729.8 |###
   5997.9 |
   6266.0 |######
   6534.1 |###
   6802.2 |##########
  (6 below, 5 above range)

warm-container-minimum (n=40, range 1753.0-7049.3 ns)
   1753.0 |########################################
   2017.8 |
   2282.6 |
   2547.4 |##
   2812.2 |
   3077.1 |
   3341.9 |
   3606.7 |
   3871.5 |
   4136.3 |
   4401.1 |
   4666.0 |####
   4930.8 |##
   5195.6 |##
   5460.4 |####
   5725.2 |##
   5990.1 |
   6254.9 |
   6519.7 |
   6784.5 |####
  (5 below, 5 above range)

warm-container-native (n=40, range 1842.6-6967.5 ns)
   1842.6 |########################################
   2098.9 |
   2355.1 |
   2611.4 |
   2867.6 |
   3123.8 |
   3380.1 |##
   3636.3 |
   3892.6 |####
   4148.8 |
   4405.1 |##
   4661.3 |####
   4917.5 |##
   5173.8 |####
   5430.0 |##
   5686.3 |
   5942.5 |
   6198.7 |##
   6455.0 |####
   6711.2 |#######
  (3 below, 4 above range)

warm-container-plusone (n=40, range 3355.1-8126.5 ns)
   3355.1 |########################################
   3593.7 |
   3832.2 |
   4070.8 |##################
   4309.4 |
   4547.9 |##
   4786.5 |##
   5025.1 |
   5263.6 |
   5502.2 |##
   5740.8 |
   5979.3 |
   6217.9 |
   6456.5 |
   6695.0 |
   6933.6 |
   7172.2 |
   7410.7 |
   7649.3 |
   7887.9 |
  (3 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=41.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=52.1% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=62.7% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=62.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-native**: CV=54.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=48.5% (high variance, measurements may be unstable)
