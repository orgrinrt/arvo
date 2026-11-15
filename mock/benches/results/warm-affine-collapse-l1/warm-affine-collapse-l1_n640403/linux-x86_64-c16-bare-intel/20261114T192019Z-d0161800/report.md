# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 1.99 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 51% (significant)

warm-container-minimum is -2.11 us (51%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.1x slower than the field

warm-container-headroom (4.10 us) is 2.1x the fastest (1.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum is fastest but the noisiest (CV 115.4%)

warm-container-minimum wins on median (1.99 us) yet has the highest variance (CV 115.4%), while warm-container-plusone is the steadiest (CV 42.0%, 4.10 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-minimum, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-minimum (1.99 us) and warm-container-lanes-deferred (2.00 us) differ by 0.37%, inside the noise, even though the wider field spreads 106.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.76)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.11 us) is smaller than the fastest variant's own run-to-run std-dev (2.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {warm-container-minimum, warm-container-lanes-deferred, warm-container-kernel, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (78% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-lanes-deferred, warm-container-kernel, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-lanes-deferred is inconsistent: worst-20% is 4.1x its best-20%

warm-container-lanes-deferred's best 20% of batches run at 1.80 us but its worst 20% at 7.32 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1989.8 ns median (-51.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.06x (fastest 1989.8 ns, slowest 4100.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4686ns | 4184ns | 3427ns | 4188ns | 7441ns | base |
| warm-container-kernel | 4105ns | 2134ns | 1891ns | 3682ns | 7590ns | -12.40% |
| warm-container-lanes-deferred | 3831ns | 2078ns | 1876ns | 3205ns | 7664ns | -18.24% |
| warm-container-minimum | 4016ns | 2074ns | 2053ns | 3464ns | 7635ns | -14.30% |
| warm-container-native | 3809ns | 2450ns | 2050ns | 3236ns | 7285ns | -18.73% |
| warm-container-plusone | 4356ns | 4183ns | 3425ns | 4020ns | 6295ns | -7.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4583ns | 3357ns | 7249ns | base | 7.149 |
| warm-container-kernel | 3926ns | 1816ns | 7217ns | -14.34% | 8.346 |
| warm-container-lanes-deferred | 3669ns | 1802ns | 7321ns | -19.95% | 8.931 |
| warm-container-minimum | 3840ns | 1968ns | 7262ns | -16.22% | 8.533 |
| warm-container-native | 3626ns | 1966ns | 6912ns | -20.88% | 9.036 |
| warm-container-plusone | 4258ns | 3357ns | 6124ns | -7.09% | 7.695 |

## Performance model

- Peak throughput: **18.182 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.992 | 44.0% |
| warm-container-kernel | 15.985 | 87.9% |
| warm-container-lanes-deferred | 16.407 | 90.2% |
| warm-container-minimum | 16.468 | 90.6% |
| warm-container-native | 14.204 | 78.1% |
| warm-container-plusone | 7.994 | 44.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4686ns | 4686ns | base |
| warm-container-kernel | 4105ns | 4105ns | -12.40% |
| warm-container-lanes-deferred | 3831ns | 3831ns | -18.24% |
| warm-container-minimum | 4016ns | 4016ns | -14.30% |
| warm-container-native | 3809ns | 3809ns | -18.73% |
| warm-container-plusone | 4356ns | 4356ns | -7.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4100ns | base | --- | [4099, 4110] | --- | --- | --- | --- |
| warm-container-kernel | 2050ns | -1890.9ns (-46.1%) | [-2104, -388]ns | [1994, 5625] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-container-lanes-deferred | 1997ns | -2108.6ns (-51.4%) | [-2113, -1525]ns | [1987, 4536] | YES | 0.0276 | 0.0166 | 0 |
| warm-container-minimum | 1990ns | -2109.3ns (-51.4%) | [-2122, -833]ns | [1981, 5310] | YES | 0.0276 | 0.0166 | 0 |
| warm-container-native | 2307ns | -1782.9ns (-43.5%) | [-2083, -1377]ns | [1993, 4112] | YES | 0.0111 | 0.0022 | 0 |
| warm-container-plusone | 4099ns | no significant difference | [-369, +1]ns | [3740, 4101] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3355ns | +13.5% | +24.1% | -24.8% | -25.3% | +22.2% |
| 1 | 2 | 4101ns | -51.3% | -51.5% | +31.1% | -51.9% | -8.8% |
| 1 | 3 | 4107ns | +32.9% | -31.0% | -51.6% | -50.8% | -0.0% |
| 1 | 4 | 4100ns | -50.2% | -51.3% | -51.7% | -43.4% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.079 | ok |
| warm-container-kernel | -0.758 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.382 | moderate- |
| warm-container-minimum | -0.310 | moderate- |
| warm-container-native | -0.252 | moderate- |
| warm-container-plusone | -0.468 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 3/4, lost 1/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.6ns | 4583.4ns | 0.6% |  |
| warm-container-kernel | 49.6ns | 3926.0ns | 1.3% |  |
| warm-container-lanes-deferred | 44.9ns | 3669.0ns | 1.2% |  |
| warm-container-minimum | 47.6ns | 3839.9ns | 1.2% |  |
| warm-container-native | 48.2ns | 3626.4ns | 1.3% |  |
| warm-container-plusone | 26.2ns | 4258.4ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3357.2-7249.5 ns)
   3357.2 |##
   3551.8 |
   3746.4 |
   3941.0 |########################################
   4135.6 |
   4330.3 |
   4524.9 |
   4719.5 |
   4914.1 |
   5108.7 |#
   5303.3 |
   5497.9 |
   5692.6 |
   5887.2 |
   6081.8 |
   6276.4 |
   6471.0 |
   6665.6 |
   6860.2 |
   7054.9 |#
  (6 below, 3 above range)

warm-container-kernel (n=40, range 1815.5-7217.1 ns)
   1815.5 |########################################
   2085.6 |
   2355.7 |##
   2625.8 |
   2895.9 |
   3165.9 |
   3436.0 |
   3706.1 |
   3976.2 |##
   4246.3 |
   4516.3 |
   4786.4 |
   5056.5 |####
   5326.6 |##
   5596.7 |####
   5866.7 |####
   6136.8 |####
   6406.9 |####
   6677.0 |####
   6947.1 |##
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 1802.3-7321.5 ns)
   1802.3 |########################################
   2078.2 |
   2354.2 |
   2630.1 |##
   2906.1 |
   3182.1 |
   3458.0 |####
   3734.0 |
   4009.9 |
   4285.9 |##
   4561.9 |####
   4837.8 |####
   5113.8 |
   5389.7 |
   5665.7 |##
   5941.7 |
   6217.6 |##
   6493.6 |##
   6769.5 |##
   7045.5 |########
  (4 below, 2 above range)

warm-container-minimum (n=40, range 1968.4-7261.5 ns)
   1968.4 |########################################
   2233.1 |
   2497.7 |
   2762.4 |
   3027.0 |##
   3291.7 |
   3556.3 |
   3821.0 |
   4085.6 |
   4350.3 |
   4615.0 |####
   4879.6 |
   5144.3 |
   5408.9 |
   5673.6 |####
   5938.2 |##
   6202.9 |##
   6467.5 |####
   6732.2 |##
   6996.8 |##########
  (4 below, 2 above range)

warm-container-native (n=40, range 1965.9-6912.3 ns)
   1965.9 |########################################
   2213.2 |######################
   2460.6 |
   2707.9 |
   2955.2 |##
   3202.5 |
   3449.8 |
   3697.2 |
   3944.5 |
   4191.8 |##
   4439.1 |
   4686.4 |
   4933.8 |
   5181.1 |
   5428.4 |##
   5675.7 |##
   5923.0 |##
   6170.4 |########
   6417.7 |##
   6665.0 |########
  (3 below, 3 above range)

warm-container-plusone (n=40, range 3357.0-6124.2 ns)
   3357.0 |######
   3495.4 |
   3633.7 |####################
   3772.1 |
   3910.4 |
   4048.8 |########################################
   4187.1 |##
   4325.5 |
   4463.9 |
   4602.2 |
   4740.6 |
   4878.9 |
   5017.3 |
   5155.7 |
   5294.0 |
   5432.4 |##
   5570.7 |
   5709.1 |
   5847.4 |
   5985.8 |
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=42.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=58.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.2% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-container-minimum**: CV=59.8% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-native**: CV=58.1% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-container-plusone**: CV=40.4% (high variance, measurements may be unstable)
