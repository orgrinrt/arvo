# Container fork, elementwise transform with no loop-carried value, declared-width sweep (8192 elements, 4 ops/element, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (2.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-plusone at 2.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 21% (significant)

warm-container-minimum is -612 ns (21%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is fastest but the noisiest (CV 109.8%)

warm-container-plusone wins on median (2.31 us) yet has the highest variance (CV 109.8%), while warm-container-headroom is the steadiest (CV 88.8%, 2.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-plusone, warm-container-minimum) are a dead heat (<1%)

warm-container-plusone (2.31 us) and warm-container-minimum (2.32 us) differ by 0.22%, inside the noise, even though the wider field spreads 27.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.74)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (632 ns) is smaller than the fastest variant's own run-to-run std-dev (2.54 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-plusone, warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-headroom} (27% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-headroom} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-plusone is inconsistent: worst-20% is 3.6x its best-20%

warm-container-plusone's best 20% of batches run at 2.31 us but its worst 20% at 8.41 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 2311.1 ns median (-21.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.27x (fastest 2311.1 ns, slowest 2943.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4472ns | 3025ns | 2950ns | 3235ns | 9705ns | base |
| warm-container-kernel | 4048ns | 2404ns | 2390ns | 3201ns | 8246ns | -9.48% |
| warm-container-minimum | 3856ns | 2398ns | 2388ns | 2869ns | 8282ns | -13.78% |
| warm-container-native | 4162ns | 2401ns | 2388ns | 3338ns | 8407ns | -6.94% |
| warm-container-plusone | 3953ns | 2394ns | 2391ns | 2871ns | 8764ns | -11.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4338ns | 2868ns | 9381ns | base | 9.442 |
| warm-container-kernel | 3898ns | 2309ns | 7904ns | -10.15% | 10.509 |
| warm-container-minimum | 3713ns | 2307ns | 7947ns | -14.41% | 11.031 |
| warm-container-native | 4010ns | 2306ns | 8071ns | -7.57% | 10.214 |
| warm-container-plusone | 3807ns | 2310ns | 8409ns | -12.24% | 10.759 |

## Performance model

- Peak throughput: **17.759 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 13.917 | 78.4% |
| warm-container-kernel | 17.638 | 99.3% |
| warm-container-minimum | 17.683 | 99.6% |
| warm-container-native | 17.660 | 99.4% |
| warm-container-plusone | 17.723 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4472ns | 4472ns | base |
| warm-container-kernel | 4048ns | 4048ns | -9.48% |
| warm-container-minimum | 3856ns | 3856ns | -13.78% |
| warm-container-native | 4162ns | 4162ns | -6.94% |
| warm-container-plusone | 3953ns | 3953ns | -11.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 2943ns | base | --- | [2919, 2947] | --- | --- | --- | --- |
| warm-container-kernel | 2322ns | -605.2ns (-20.6%) | [-633, -571]ns | [2309, 3030] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-minimum | 2316ns | -612.2ns (-20.8%) | [-637, -593]ns | [2308, 2324] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2319ns | -610.6ns (-20.7%) | [-635, -589]ns | [2308, 3353] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 2311ns | -609.1ns (-20.7%) | [-622, -578]ns | [2310, 2324] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 2944ns | -21.5% | -21.4% | -21.1% | -21.3% |
| 1 | 2 | 2922ns | -20.7% | -20.6% | -21.0% | -20.9% |
| 1 | 3 | 2923ns | -8.2% | -21.0% | -20.6% | -20.6% |
| 1 | 4 | 2947ns | +57.9% | -21.4% | -21.0% | -21.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.247 | moderate- |
| warm-container-kernel | 0.041 | ok |
| warm-container-minimum | -0.738 | HIGH- (thermal bounce) |
| warm-container-native | -0.202 | moderate- |
| warm-container-plusone | -0.735 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 37.0ns | 4338.2ns | 0.9% |  |
| warm-container-kernel | 41.7ns | 3897.7ns | 1.1% |  |
| warm-container-minimum | 39.7ns | 3713.2ns | 1.1% |  |
| warm-container-native | 42.9ns | 4010.0ns | 1.1% |  |
| warm-container-plusone | 40.8ns | 3807.1ns | 1.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 2868.0-9380.9 ns)
   2868.0 |########################################
   3193.6 |
   3519.3 |#
   3844.9 |
   4170.6 |
   4496.2 |#
   4821.8 |
   5147.5 |
   5473.1 |#
   5798.8 |
   6124.4 |
   6450.1 |
   6775.7 |
   7101.4 |
   7427.0 |
   7752.7 |#
   8078.3 |#
   8404.0 |
   8729.6 |#
   9055.3 |###
  (4 below, 3 above range)

warm-container-kernel (n=40, range 2308.5-7904.2 ns)
   2308.5 |########################################
   2588.3 |
   2868.1 |###
   3147.9 |#
   3427.6 |
   3707.4 |
   3987.2 |
   4267.0 |
   4546.8 |
   4826.6 |
   5106.3 |
   5386.1 |
   5665.9 |
   5945.7 |#
   6225.5 |###
   6505.2 |#
   6785.0 |###
   7064.8 |#
   7344.6 |#
   7624.4 |#
  (4 below, 3 above range)

warm-container-minimum (n=40, range 2306.8-7946.9 ns)
   2306.8 |########################################
   2588.8 |#
   2870.8 |
   3152.8 |
   3434.8 |
   3716.8 |#
   3998.8 |
   4280.8 |
   4562.8 |###
   4844.8 |
   5126.9 |
   5408.9 |
   5690.9 |
   5972.9 |
   6254.9 |
   6536.9 |#
   6818.9 |#
   7100.9 |###
   7382.9 |###
   7664.9 |#
  (3 below, 2 above range)

warm-container-native (n=40, range 2306.5-8071.2 ns)
   2306.5 |########################################
   2594.7 |
   2882.9 |
   3171.2 |
   3459.4 |
   3747.6 |
   4035.9 |
   4324.1 |#
   4612.3 |#
   4900.6 |
   5188.8 |
   5477.0 |
   5765.3 |
   6053.5 |#
   6341.7 |#
   6630.0 |
   6918.2 |###
   7206.5 |#
   7494.7 |###
   7782.9 |#
  (4 below, 4 above range)

warm-container-plusone (n=40, range 2309.8-8409.0 ns)
   2309.8 |########################################
   2614.8 |
   2919.8 |###
   3224.7 |
   3529.7 |
   3834.6 |
   4139.6 |
   4444.6 |
   4749.5 |
   5054.5 |
   5359.4 |
   5664.4 |
   5969.4 |
   6274.3 |
   6579.3 |
   6884.2 |###
   7189.2 |
   7494.2 |###
   7799.1 |
   8104.1 |#####
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=60.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-container-kernel**: CV=60.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-minimum**: CV=63.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-native**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=66.6% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.6x (possible bimodal distribution)
