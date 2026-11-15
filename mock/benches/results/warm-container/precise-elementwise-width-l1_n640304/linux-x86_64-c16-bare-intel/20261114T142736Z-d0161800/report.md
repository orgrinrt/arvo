# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (7.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 35% (significant)

warm-container-native is -2.72 us (35%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (5.06 us) and warm-container-native (5.06 us) differ by 0.03%, inside the noise, even though the wider field spreads 53.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.51)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (54% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 54% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 1.9x its best-20%

warm-container-minimum's best 20% of batches run at 5.05 us but its worst 20% at 9.67 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 5063.0 ns median (-34.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.54x (fastest 5063.0 ns, slowest 7781.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 9463ns | 7870ns | 7851ns | 8407ns | 14244ns | base |
| warm-container-kernel | 5920ns | 5146ns | 5109ns | 5144ns | 9058ns | -37.45% |
| warm-container-minimum | 6093ns | 5148ns | 5136ns | 5146ns | 9894ns | -35.61% |
| warm-container-native | 5695ns | 5146ns | 5135ns | 5144ns | 7908ns | -39.82% |
| warm-container-plusone | 7719ns | 7869ns | 7233ns | 7820ns | 7902ns | -18.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 9357ns | 7760ns | 14097ns | base | 4.378 |
| warm-container-kernel | 5814ns | 5027ns | 8859ns | -37.86% | 7.045 |
| warm-container-minimum | 5983ns | 5053ns | 9674ns | -36.06% | 6.846 |
| warm-container-native | 5593ns | 5053ns | 7725ns | -40.23% | 7.324 |
| warm-container-plusone | 7626ns | 7146ns | 7795ns | -18.49% | 5.371 |

## Performance model

- Peak throughput: **8.149 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.264 | 64.6% |
| warm-container-kernel | 8.090 | 99.3% |
| warm-container-minimum | 8.087 | 99.2% |
| warm-container-native | 8.088 | 99.3% |
| warm-container-plusone | 5.265 | 64.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 9463ns | 9463ns | base |
| warm-container-kernel | 5920ns | 5920ns | -37.45% |
| warm-container-minimum | 6093ns | 6093ns | -35.61% |
| warm-container-native | 5695ns | 5695ns | -39.82% |
| warm-container-plusone | 7719ns | 7719ns | -18.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7781ns | base | --- | [7775, 7787] | --- | --- | --- | --- |
| warm-container-kernel | 5063ns | -2714.0ns (-34.9%) | [-2726, -2710]ns | [5055, 5066] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5065ns | -2718.6ns (-34.9%) | [-2727, -2711]ns | [5056, 5067] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 5065ns | -2719.4ns (-35.0%) | [-2730, -2712]ns | [5057, 5066] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 7780ns | no significant difference | [-629, +10]ns | [7775, 7787] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 7778ns | -34.9% | -34.9% | -34.9% | +0.1% |
| 1 | 2 | 7766ns | -34.8% | -34.8% | -34.8% | +0.3% |
| 1 | 3 | 7776ns | -34.8% | -34.9% | -34.9% | -8.1% |
| 1 | 4 | 14092ns | -64.1% | -64.0% | -64.1% | -44.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.083 | ok |
| warm-container-kernel | -0.271 | moderate- |
| warm-container-minimum | -0.513 | HIGH- (thermal bounce) |
| warm-container-native | -0.172 | ok |
| warm-container-plusone | -0.410 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 30.3ns | 9356.6ns | 0.3% |  |
| warm-container-kernel | 28.6ns | 5813.9ns | 0.5% |  |
| warm-container-minimum | 29.5ns | 5982.8ns | 0.5% |  |
| warm-container-native | 27.6ns | 5592.6ns | 0.5% |  |
| warm-container-plusone | 25.6ns | 7626.2ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7760.4-14096.9 ns)
   7760.4 |########################################
   8077.3 |
   8394.1 |
   8710.9 |
   9027.7 |
   9344.6 |
   9661.4 |
   9978.2 |
  10295.0 |
  10611.8 |
  10928.7 |
  11245.5 |
  11562.3 |
  11879.1 |
  12195.9 |
  12512.8 |
  12829.6 |
  13146.4 |
  13463.2 |
  13780.0 |########
  (3 below, 4 above range)

warm-container-kernel (n=40, range 5026.7-8858.9 ns)
   5026.7 |########################################
   5218.3 |
   5409.9 |
   5601.5 |
   5793.1 |
   5984.7 |
   6176.4 |
   6368.0 |
   6559.6 |
   6751.2 |
   6942.8 |
   7134.4 |
   7326.0 |
   7517.6 |
   7709.2 |
   7900.8 |
   8092.5 |
   8284.1 |
   8475.7 |
   8667.3 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 5053.0-9674.0 ns)
   5053.0 |########################################
   5284.0 |
   5515.1 |
   5746.1 |
   5977.2 |
   6208.2 |
   6439.3 |
   6670.3 |
   6901.4 |
   7132.4 |
   7363.5 |
   7594.5 |
   7825.6 |
   8056.6 |
   8287.7 |
   8518.7 |
   8749.8 |
   8980.8 |
   9211.9 |
   9442.9 |
  (2 below, 5 above range)

warm-container-native (n=40, range 5052.7-7725.5 ns)
   5052.7 |########################################
   5186.3 |
   5320.0 |
   5453.6 |
   5587.2 |
   5720.9 |
   5854.5 |
   5988.2 |
   6121.8 |
   6255.4 |
   6389.1 |
   6522.7 |
   6656.4 |
   6790.0 |
   6923.6 |
   7057.3 |
   7190.9 |
   7324.6 |
   7458.2 |
   7591.8 |
  (2 below, 4 above range)

warm-container-plusone (n=40, range 7145.7-7795.2 ns)
   7145.7 |##########
   7178.2 |#
   7210.7 |
   7243.2 |
   7275.6 |
   7308.1 |
   7340.6 |
   7373.1 |
   7405.5 |
   7438.0 |
   7470.5 |
   7503.0 |
   7535.4 |
   7567.9 |
   7600.4 |
   7632.9 |
   7665.3 |
   7697.8 |
   7730.3 |
   7762.8 |########################################
  (2 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=29.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=40.0% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=41.9% (high variance, measurements may be unstable)
- **warm-container-native**: CV=29.4% (high variance, measurements may be unstable)
