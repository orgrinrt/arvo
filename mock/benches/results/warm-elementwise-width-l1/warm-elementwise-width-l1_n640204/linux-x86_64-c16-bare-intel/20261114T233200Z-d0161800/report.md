# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 63% (significant)

warm-container-minimum is -3.34 us (63%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.7x slower than the field

warm-container-plusone (5.31 us) is 2.7x the fastest (1.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum is fastest but the noisiest (CV 110.6%)

warm-container-minimum wins on median (1.97 us) yet has the highest variance (CV 110.6%), while warm-container-plusone is the steadiest (CV 27.9%, 5.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-minimum, warm-container-kernel) are a dead heat (<1%)

warm-container-minimum (1.97 us) and warm-container-kernel (1.98 us) differ by 0.50%, inside the noise, even though the wider field spreads 169.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.70)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-kernel} vs {warm-container-native, warm-container-headroom, warm-container-plusone} (153% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-native, warm-container-headroom, warm-container-plusone} with a 153% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 3.6x its best-20%

warm-container-minimum's best 20% of batches run at 1.95 us but its worst 20% at 6.92 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (14 ns, 0.27%)

warm-container-plusone differs from baseline warm-container-headroom by 14 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-minimum** at 1968.8 ns median (-62.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.70x (fastest 1968.8 ns, slowest 5313.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5872ns | 5389ns | 5377ns | 5388ns | 7821ns | base |
| warm-container-kernel | 3978ns | 2061ns | 2044ns | 3535ns | 7243ns | -32.25% |
| warm-container-minimum | 3905ns | 2051ns | 2027ns | 3410ns | 7268ns | -33.50% |
| warm-container-native | 4228ns | 5214ns | 2016ns | 4174ns | 6600ns | -28.01% |
| warm-container-plusone | 6315ns | 5398ns | 5382ns | 5780ns | 8852ns | +7.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5773ns | 5293ns | 7655ns | base | 7.095 |
| warm-container-kernel | 3804ns | 1962ns | 6882ns | -34.11% | 10.768 |
| warm-container-minimum | 3737ns | 1946ns | 6918ns | -35.27% | 10.961 |
| warm-container-native | 4047ns | 1935ns | 6280ns | -29.90% | 10.121 |
| warm-container-plusone | 6208ns | 5300ns | 8694ns | +7.53% | 6.598 |

## Performance model

- Peak throughput: **21.170 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.720 | 36.5% |
| warm-container-kernel | 20.703 | 97.8% |
| warm-container-minimum | 20.805 | 98.3% |
| warm-container-native | 8.185 | 38.7% |
| warm-container-plusone | 7.709 | 36.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5872ns | 5872ns | base |
| warm-container-kernel | 3978ns | 3978ns | -32.25% |
| warm-container-minimum | 3905ns | 3905ns | -33.50% |
| warm-container-native | 4228ns | 4228ns | -28.01% |
| warm-container-plusone | 6315ns | 6315ns | +7.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5305ns | base | --- | [5300, 5311] | --- | --- | --- | --- |
| warm-container-kernel | 1978ns | -3326.7ns (-62.7%) | [-3335, -1434]ns | [1967, 5683] | YES (adj: no) | 0.0513 | 0.0166 | 0 |
| warm-container-minimum | 1969ns | -3342.7ns (-63.0%) | [-3352, -1742]ns | [1953, 5510] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| warm-container-native | 5004ns | no significant difference | [-3312, +3]ns | [2038, 5503] | no | 0.0807 | 0.0807 | 0 |
| warm-container-plusone | 5314ns | +14.3ns (+0.3%) | [+6, +25]ns | [5308, 5332] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 5311ns | -62.9% | -25.3% | -5.5% | +58.9% |
| 1 | 2 | 5301ns | -21.7% | -58.2% | -2.0% | +0.3% |
| 1 | 3 | 5305ns | -62.7% | -26.2% | -31.7% | +0.1% |
| 1 | 4 | 5307ns | -55.5% | -63.2% | -35.1% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.421 | moderate- |
| warm-container-kernel | -0.557 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.699 | HIGH- (thermal bounce) |
| warm-container-native | 0.244 | moderate+ |
| warm-container-plusone | -0.081 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 0/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.3ns | 5773.0ns | 0.5% |  |
| warm-container-kernel | 48.4ns | 3804.0ns | 1.3% |  |
| warm-container-minimum | 54.0ns | 3736.7ns | 1.4% |  |
| warm-container-native | 52.1ns | 4047.2ns | 1.3% |  |
| warm-container-plusone | 30.1ns | 6207.8ns | 0.5% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5293.2-7655.2 ns)
   5293.2 |########################################
   5411.3 |
   5529.4 |
   5647.5 |
   5765.6 |
   5883.7 |
   6001.8 |
   6119.9 |
   6238.0 |
   6356.1 |
   6474.2 |
   6592.3 |
   6710.4 |
   6828.5 |
   6946.6 |
   7064.7 |
   7182.8 |
   7300.9 |
   7419.0 |#
   7537.1 |
  (4 below, 2 above range)

warm-container-kernel (n=40, range 1962.1-6881.8 ns)
   1962.1 |########################################
   2208.1 |
   2454.0 |##
   2700.0 |##
   2946.0 |
   3192.0 |##
   3438.0 |
   3684.0 |
   3930.0 |
   4176.0 |
   4421.9 |
   4667.9 |
   4913.9 |##
   5159.9 |
   5405.9 |##
   5651.9 |##
   5897.9 |####
   6143.9 |#######
   6389.8 |##
   6635.8 |####
  (4 below, 5 above range)

warm-container-minimum (n=40, range 1945.9-6917.9 ns)
   1945.9 |########################################
   2194.5 |####
   2443.1 |##
   2691.7 |
   2940.3 |
   3188.9 |
   3437.5 |
   3686.1 |
   3934.7 |
   4183.3 |
   4431.9 |##
   4680.5 |
   4929.1 |
   5177.7 |
   5426.3 |####
   5674.9 |####
   5923.5 |##
   6172.1 |#######
   6420.7 |####
   6669.3 |##
  (4 below, 4 above range)

warm-container-native (n=40, range 1934.8-6280.4 ns)
   1934.8 |########################################
   2152.1 |###
   2369.4 |
   2586.6 |
   2803.9 |
   3021.2 |
   3238.5 |
   3455.8 |
   3673.0 |
   3890.3 |
   4107.6 |
   4324.9 |
   4542.2 |
   4759.4 |######
   4976.7 |######
   5194.0 |######
   5411.3 |############
   5628.6 |######
   5845.9 |############
   6063.1 |###
  (4 below, 5 above range)

warm-container-plusone (n=40, range 5299.6-8693.9 ns)
   5299.6 |########################################
   5469.3 |#
   5639.0 |
   5808.7 |
   5978.4 |
   6148.2 |
   6317.9 |
   6487.6 |
   6657.3 |
   6827.0 |
   6996.8 |
   7166.5 |
   7336.2 |
   7505.9 |#
   7675.6 |
   7845.3 |
   8015.1 |
   8184.8 |
   8354.5 |###############
   8524.2 |
  (4 below, 1 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=31.7% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=56.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-minimum**: CV=58.3% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=46.8% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=23.9% (high variance, measurements may be unstable)
