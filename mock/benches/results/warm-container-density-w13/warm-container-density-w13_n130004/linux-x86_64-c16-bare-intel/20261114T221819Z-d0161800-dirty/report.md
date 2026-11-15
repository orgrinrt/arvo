# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 1.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 58% (significant)

warm-container-native is -2.31 us (58%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.2x slower than the field

warm-container-headroom (3.94 us) is 2.2x the fastest (1.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum is fastest but the noisiest (CV 124.0%)

warm-container-minimum wins on median (1.76 us) yet has the highest variance (CV 124.0%), while warm-container-native is the steadiest (CV 16.8%, 1.77 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (1.76 us) and warm-container-native (1.77 us) differ by 0.24%, inside the noise, even though the wider field spreads 123.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.74)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.18 us) is smaller than the fastest variant's own run-to-run std-dev (2.19 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-plusone, warm-container-kernel, warm-container-lanes-deferred, warm-container-headroom} (60% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-plusone, warm-container-kernel, warm-container-lanes-deferred, warm-container-headroom} with a 60% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+0% speed for 7.4x steadier)

warm-container-minimum is fastest (1.76 us, CV 124.0%); warm-container-native gives up 0.2% median for 7.4x lower variance (CV 16.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 4.4x its best-20%

warm-container-plusone's best 20% of batches run at 1.43 us but its worst 20% at 6.37 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1763.5 ns median (-55.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.24x (fastest 1763.5 ns, slowest 3942.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4147ns | 4075ns | 1925ns | 3958ns | 6938ns | base |
| warm-container-kernel | 3591ns | 3835ns | 2091ns | 3778ns | 4531ns | -13.41% |
| warm-container-lanes-deferred | 3542ns | 3894ns | 1190ns | 3828ns | 5033ns | -14.60% |
| warm-container-minimum | 3643ns | 1827ns | 1609ns | 3216ns | 6957ns | -12.17% |
| warm-container-native | 2023ns | 1998ns | 1538ns | 2019ns | 2521ns | -51.22% |
| warm-container-plusone | 3803ns | 2926ns | 1484ns | 3628ns | 6646ns | -8.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4000ns | 1862ns | 6652ns | base | 10.239 |
| warm-container-kernel | 3361ns | 1955ns | 4174ns | -15.99% | 12.189 |
| warm-container-lanes-deferred | 3339ns | 1126ns | 4721ns | -16.54% | 12.269 |
| warm-container-minimum | 3503ns | 1553ns | 6657ns | -12.42% | 11.692 |
| warm-container-native | 1779ns | 1363ns | 2203ns | -55.52% | 23.021 |
| warm-container-plusone | 3658ns | 1433ns | 6369ns | -8.55% | 11.197 |

## Performance model

- Peak throughput: **36.381 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.389 | 28.6% |
| warm-container-kernel | 11.351 | 31.2% |
| warm-container-lanes-deferred | 11.131 | 30.6% |
| warm-container-minimum | 23.227 | 63.8% |
| warm-container-native | 23.170 | 63.7% |
| warm-container-plusone | 14.494 | 39.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4147ns | 4147ns | base |
| warm-container-kernel | 3591ns | 3591ns | -13.41% |
| warm-container-lanes-deferred | 3542ns | 3542ns | -14.60% |
| warm-container-minimum | 3643ns | 3643ns | -12.17% |
| warm-container-native | 2023ns | 2023ns | -51.22% |
| warm-container-plusone | 3803ns | 3803ns | -8.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3943ns | base | --- | [1880, 5541] | --- | --- | --- | --- |
| warm-container-kernel | 3609ns | no significant difference | [-1586, +302]ns | [3345, 3811] | no | 0.2682 | 0.2682 | 0 |
| warm-container-lanes-deferred | 3680ns | -980.1ns (-24.9%) | [-1691, -773]ns | [3308, 3972] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-minimum | 1764ns | -128.2ns (-3.3%) | [-432, -109]ns | [1754, 5098] | YES | 0.0003 | 0.0002 | 0 |
| warm-container-native | 1768ns | -2305.5ns (-58.5%) | [-3644, -502]ns | [1687, 1905] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2826ns | -411.7ns (-10.4%) | [-436, -130]ns | [1752, 5401] | YES | 0.0003 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2430ns | +27.3% | -4.4% | -27.7% | -43.7% | +99.8% |
| 1 | 2 | 5150ns | -25.4% | -34.9% | -60.3% | -63.3% | -65.8% |
| 1 | 3 | 5076ns | -23.9% | -24.1% | -44.7% | -65.6% | +0.4% |
| 1 | 4 | 1884ns | +47.3% | +113.9% | +41.8% | +10.6% | -23.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.244 | moderate- |
| warm-container-kernel | -0.239 | moderate- |
| warm-container-lanes-deferred | 0.181 | ok |
| warm-container-minimum | 0.255 | moderate+ |
| warm-container-native | -0.218 | moderate- |
| warm-container-plusone | -0.740 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 3/4, lost 1/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 52.5ns | 4000.2ns | 1.3% |  |
| warm-container-kernel | 81.0ns | 3360.5ns | 2.4% |  |
| warm-container-lanes-deferred | 75.3ns | 3338.6ns | 2.3% |  |
| warm-container-minimum | 50.9ns | 3503.3ns | 1.5% |  |
| warm-container-native | 91.1ns | 1779.2ns | 5.1% | HIGH |
| warm-container-plusone | 51.5ns | 3658.2ns | 1.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1862.1-6651.8 ns)
   1862.1 |########################################
   2101.6 |
   2341.1 |
   2580.6 |
   2820.0 |##
   3059.5 |##
   3299.0 |
   3538.5 |
   3778.0 |
   4017.5 |
   4256.9 |
   4496.4 |##
   4735.9 |
   4975.4 |##
   5214.9 |###########
   5454.3 |#####
   5693.8 |##
   5933.3 |#####
   6172.8 |#####
   6412.3 |#####
  (4 below, 5 above range)

warm-container-kernel (n=40, range 1954.8-4173.9 ns)
   1954.8 |######
   2065.8 |
   2176.7 |
   2287.7 |
   2398.7 |
   2509.6 |
   2620.6 |
   2731.5 |#############
   2842.5 |#############
   2953.4 |######
   3064.4 |######
   3175.3 |######
   3286.3 |########################################
   3397.3 |
   3508.2 |####################
   3619.2 |####################
   3730.1 |####################
   3841.1 |######
   3952.0 |########################################
   4063.0 |#############
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1125.8-4720.6 ns)
   1125.8 |
   1305.6 |
   1485.3 |
   1665.1 |
   1844.8 |
   2024.5 |
   2204.3 |################
   2384.0 |
   2563.8 |
   2743.5 |
   2923.2 |################
   3103.0 |################
   3282.7 |########################
   3462.5 |################
   3642.2 |########################################
   3821.9 |########################################
   4001.7 |########################
   4181.4 |
   4361.2 |########################################
   4540.9 |
  (7 below, 4 above range)

warm-container-minimum (n=40, range 1553.2-6656.6 ns)
   1553.2 |########################################
   1808.4 |
   2063.5 |
   2318.7 |##
   2573.9 |
   2829.1 |
   3084.2 |
   3339.4 |
   3594.6 |
   3849.7 |####
   4104.9 |
   4360.1 |##
   4615.2 |
   4870.4 |
   5125.6 |
   5380.8 |##
   5635.9 |#######
   5891.1 |####
   6146.3 |###########
   6401.4 |
  (5 below, 3 above range)

warm-container-native (n=40, range 1363.5-2202.7 ns)
   1363.5 |##########
   1405.4 |
   1447.4 |##########
   1489.4 |
   1531.3 |#####
   1573.3 |###############
   1615.3 |#####
   1657.2 |
   1699.2 |#####
   1741.1 |########################################
   1783.1 |#####
   1825.1 |##########
   1867.0 |
   1909.0 |
   1950.9 |##############################
   1992.9 |##########
   2034.9 |
   2076.8 |#####
   2118.8 |#####
   2160.7 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 1433.2-6369.5 ns)
   1433.2 |###################################
   1680.0 |########################################
   1926.8 |
   2173.6 |
   2420.4 |####
   2667.2 |
   2914.1 |
   3160.9 |####
   3407.7 |
   3654.5 |
   3901.3 |
   4148.1 |
   4394.9 |####
   4641.8 |
   4888.6 |####
   5135.4 |#############
   5382.2 |####
   5629.0 |######################
   5875.8 |########
   6122.7 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=51.9% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-kernel**: CV=25.2% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: CV=37.7% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=62.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-plusone**: CV=57.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.4x (possible bimodal distribution)
