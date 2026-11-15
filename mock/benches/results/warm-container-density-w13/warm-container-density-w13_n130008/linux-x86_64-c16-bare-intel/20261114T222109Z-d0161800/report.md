# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.34 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 1.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred dominates: 70% faster than the next best (warm-container-native)

warm-container-lanes-deferred (1.87 us) leads warm-container-native (3.18 us) by 70%, a clear separation rather than a photo finish. CV 103.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-lanes-deferred beats baseline by 65% (significant)

warm-container-lanes-deferred is -2.83 us (65%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.3x slower than the field

warm-container-headroom (4.34 us) is 2.3x the fastest (1.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred is fastest but the noisiest (CV 103.7%)

warm-container-lanes-deferred wins on median (1.87 us) yet has the highest variance (CV 103.7%), while warm-container-native is the steadiest (CV 21.8%, 3.18 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.53)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred} vs {warm-container-native, warm-container-kernel, warm-container-plusone, warm-container-minimum, warm-container-headroom} (70% apart)

The field splits into a fast tier {warm-container-lanes-deferred} and a slow tier {warm-container-native, warm-container-kernel, warm-container-plusone, warm-container-minimum, warm-container-headroom} with a 70% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 4.1x its best-20%

warm-container-kernel's best 20% of batches run at 1.50 us but its worst 20% at 6.12 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-30 ns, 0.70%)

warm-container-plusone differs from baseline warm-container-headroom by -30 ns (0.70%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 1872.8 ns median (-56.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.32x (fastest 1872.8 ns, slowest 4337.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5097ns | 4401ns | 4392ns | 4401ns | 7886ns | base |
| warm-container-kernel | 3647ns | 3482ns | 1561ns | 3411ns | 6441ns | -28.44% |
| warm-container-lanes-deferred | 3439ns | 1952ns | 1558ns | 3154ns | 6174ns | -32.53% |
| warm-container-minimum | 5367ns | 4376ns | 3579ns | 4705ns | 9139ns | +5.30% |
| warm-container-native | 3262ns | 3408ns | 2121ns | 3414ns | 3946ns | -36.00% |
| warm-container-plusone | 4952ns | 4374ns | 3578ns | 4372ns | 8063ns | -2.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5012ns | 4329ns | 7715ns | base | 14.711 |
| warm-container-kernel | 3486ns | 1497ns | 6117ns | -30.44% | 21.147 |
| warm-container-lanes-deferred | 3287ns | 1494ns | 5866ns | -34.41% | 22.429 |
| warm-container-minimum | 5272ns | 3525ns | 8947ns | +5.19% | 13.985 |
| warm-container-native | 3033ns | 1979ns | 3633ns | -39.49% | 24.312 |
| warm-container-plusone | 4867ns | 3527ns | 7878ns | -2.90% | 15.150 |

## Performance model

- Peak throughput: **49.333 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 16.999 | 34.5% |
| warm-container-kernel | 22.064 | 44.7% |
| warm-container-lanes-deferred | 39.369 | 79.8% |
| warm-container-minimum | 17.095 | 34.7% |
| warm-container-native | 23.190 | 47.0% |
| warm-container-plusone | 17.103 | 34.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5097ns | 5097ns | base |
| warm-container-kernel | 3647ns | 3647ns | -28.44% |
| warm-container-lanes-deferred | 3439ns | 3439ns | -32.53% |
| warm-container-minimum | 5367ns | 5367ns | +5.30% |
| warm-container-native | 3262ns | 3262ns | -36.00% |
| warm-container-plusone | 4952ns | 4952ns | -2.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4337ns | base | --- | [4332, 4344] | --- | --- | --- | --- |
| warm-container-kernel | 3342ns | no significant difference | [-2833, +129]ns | [1508, 4908] | no | 0.1539 | 0.1539 | 0 |
| warm-container-lanes-deferred | 1873ns | -2828.4ns (-65.2%) | [-2836, -1190]ns | [1499, 4917] | YES (adj: no) | 0.0641 | 0.0385 | 0 |
| warm-container-minimum | 4313ns | no significant difference | [-35, +28]ns | [4307, 5499] | no | 0.1009 | 0.0807 | 0 |
| warm-container-native | 3179ns | -1156.1ns (-26.7%) | [-1311, -1153]ns | [3178, 3231] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4311ns | -30.2ns (-0.7%) | [-33, -24]ns | [4305, 4314] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4338ns | -65.5% | -23.9% | -18.7% | -25.5% | -0.6% |
| 1 | 2 | 4344ns | -64.3% | -65.5% | +26.9% | -26.2% | -18.8% |
| 1 | 3 | 4333ns | +6.0% | -60.9% | -0.4% | -29.3% | -0.5% |
| 1 | 4 | 4337ns | -9.4% | +11.9% | -0.6% | -26.7% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.379 | moderate- |
| warm-container-kernel | 0.176 | ok |
| warm-container-lanes-deferred | -0.189 | ok |
| warm-container-minimum | -0.533 | HIGH- (thermal bounce) |
| warm-container-native | -0.147 | ok |
| warm-container-plusone | -0.417 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 3/4, lost 1/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.1ns | 5011.9ns | 0.6% |  |
| warm-container-kernel | 58.9ns | 3486.4ns | 1.7% |  |
| warm-container-lanes-deferred | 53.6ns | 3287.1ns | 1.6% |  |
| warm-container-minimum | 32.0ns | 5272.0ns | 0.6% |  |
| warm-container-native | 84.5ns | 3032.6ns | 2.8% |  |
| warm-container-plusone | 27.9ns | 4866.7ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 4329.1-7715.2 ns)
   4329.1 |########################################
   4498.5 |
   4667.8 |
   4837.1 |
   5006.4 |
   5175.7 |
   5345.0 |
   5514.3 |
   5683.6 |
   5852.9 |
   6022.2 |
   6191.5 |
   6360.8 |
   6530.1 |
   6699.4 |
   6868.7 |
   7038.0 |
   7207.3 |
   7376.6 |
   7545.9 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1497.5-6116.6 ns)
   1497.5 |########################################
   1728.4 |
   1959.4 |
   2190.3 |
   2421.3 |
   2652.3 |
   2883.2 |##
   3114.2 |
   3345.1 |
   3576.1 |##
   3807.0 |
   4038.0 |
   4268.9 |#####
   4499.9 |##
   4730.9 |########
   4961.8 |
   5192.8 |#####
   5423.7 |################
   5654.7 |
   5885.6 |#####
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1494.5-5865.6 ns)
   1494.5 |########################################
   1713.1 |#####
   1931.6 |
   2150.2 |
   2368.7 |
   2587.3 |
   2805.8 |##
   3024.4 |
   3242.9 |##
   3461.5 |
   3680.0 |
   3898.6 |
   4117.1 |
   4335.7 |
   4554.2 |#####
   4772.8 |#####
   4991.4 |##
   5209.9 |########
   5428.5 |#############
   5647.0 |##
  (4 below, 3 above range)

warm-container-minimum (n=40, range 3525.1-8947.0 ns)
   3525.1 |##########
   3796.2 |
   4067.3 |########################################
   4338.4 |##
   4609.5 |
   4880.6 |
   5151.6 |
   5422.7 |##########################
   5693.8 |
   5964.9 |
   6236.0 |
   6507.1 |
   6778.2 |
   7049.3 |
   7320.4 |
   7591.5 |
   7862.6 |##
   8133.7 |
   8404.8 |
   8675.9 |
  (5 below, 4 above range)

warm-container-native (n=40, range 1979.2-3632.9 ns)
   1979.2 |
   2061.9 |
   2144.6 |
   2227.2 |
   2309.9 |
   2392.6 |
   2475.3 |
   2558.0 |
   2640.7 |###
   2723.3 |###
   2806.0 |
   2888.7 |############
   2971.4 |
   3054.1 |###
   3136.8 |########################################
   3219.5 |#####################
   3302.1 |###
   3384.8 |###
   3467.5 |#########
   3550.2 |######
  (4 below, 2 above range)

warm-container-plusone (n=40, range 3526.7-7878.0 ns)
   3526.7 |##
   3744.2 |
   3961.8 |
   4179.4 |########################################
   4396.9 |
   4614.5 |
   4832.1 |#
   5049.6 |
   5267.2 |
   5484.8 |
   5702.4 |
   5919.9 |
   6137.5 |
   6355.1 |
   6572.6 |
   6790.2 |
   7007.8 |
   7225.3 |
   7442.9 |
   7660.5 |
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=42.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=57.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=59.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-minimum**: CV=45.0% (high variance, measurements may be unstable)
- **warm-container-native**: CV=22.8% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=46.0% (high variance, measurements may be unstable)
