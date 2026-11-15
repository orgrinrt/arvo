# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 20% faster than the next best (warm-container-minimum)

warm-container-kernel (6.97 us) leads warm-container-minimum (8.35 us) by 20%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 63% (significant)

warm-container-kernel is -11.93 us (63%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.7x slower than the field

warm-container-plusone (18.92 us) is 2.7x the fastest (6.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel, warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (124% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 124% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 2.1x its best-20%

warm-container-minimum's best 20% of batches run at 6.60 us but its worst 20% at 13.60 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 6966.6 ns median (-63.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.72x (fastest 6966.6 ns, slowest 18923.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 18958ns | 18894ns | 18886ns | 18931ns | 19111ns | base |
| warm-container-kernel | 7061ns | 7057ns | 6345ns | 7060ns | 7783ns | -62.75% |
| warm-container-minimum | 8974ns | 8475ns | 6673ns | 8149ns | 13749ns | -52.66% |
| warm-container-native | 9401ns | 8481ns | 7043ns | 8739ns | 13748ns | -50.41% |
| warm-container-plusone | 18172ns | 19019ns | 15564ns | 18749ns | 19049ns | -4.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 18871ns | 18805ns | 19019ns | base | 7.380 |
| warm-container-kernel | 6981ns | 6273ns | 7693ns | -63.01% | 19.950 |
| warm-container-minimum | 8875ns | 6601ns | 13601ns | -52.97% | 15.692 |
| warm-container-native | 9301ns | 6972ns | 13601ns | -50.71% | 14.973 |
| warm-container-plusone | 18083ns | 15487ns | 18943ns | -4.18% | 7.702 |

## Performance model

- Peak throughput: **22.201 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.403 | 33.3% |
| warm-container-kernel | 19.990 | 90.0% |
| warm-container-minimum | 16.675 | 75.1% |
| warm-container-native | 16.590 | 74.7% |
| warm-container-plusone | 7.359 | 33.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 18958ns | 18958ns | base |
| warm-container-kernel | 7061ns | 7061ns | -62.75% |
| warm-container-minimum | 8974ns | 8974ns | -52.66% |
| warm-container-native | 9401ns | 9401ns | -50.41% |
| warm-container-plusone | 18172ns | 18172ns | -4.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 18812ns | base | --- | [18811, 18854] | --- | --- | --- | --- |
| warm-container-kernel | 6967ns | -11932.9ns (-63.4%) | [-12520, -11229]ns | [6291, 7674] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8352ns | -10655.5ns (-56.6%) | [-12203, -10318]ns | [6610, 8529] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 8395ns | -10512.8ns (-55.9%) | [-10828, -10334]ns | [8088, 8518] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 18923ns | no significant difference | [-90, +97]ns | [18904, 18936] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 18812ns | -66.6% | -64.9% | -62.9% | -17.6% |
| 1 | 2 | 18832ns | -66.6% | -64.9% | -57.0% | +0.5% |
| 1 | 3 | 18810ns | -59.2% | -27.7% | -27.7% | +0.7% |
| 1 | 4 | 18872ns | -59.3% | -54.8% | -54.9% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.362 | moderate- |
| warm-container-kernel | 0.252 | moderate+ |
| warm-container-minimum | -0.217 | moderate- |
| warm-container-native | -0.221 | moderate- |
| warm-container-plusone | -0.081 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.6ns | 18870.5ns | 0.1% |  |
| warm-container-kernel | 23.5ns | 6980.5ns | 0.3% |  |
| warm-container-minimum | 29.8ns | 8875.1ns | 0.3% |  |
| warm-container-native | 30.8ns | 9301.0ns | 0.3% |  |
| warm-container-plusone | 25.2ns | 18082.6ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 18804.5-19019.0 ns)
  18804.5 |########################################
  18815.2 |##
  18826.0 |####
  18836.7 |####
  18847.4 |
  18858.1 |##
  18868.9 |
  18879.6 |####
  18890.3 |
  18901.0 |
  18911.8 |
  18922.5 |
  18933.2 |
  18943.9 |##
  18954.7 |##
  18965.4 |
  18976.1 |##
  18986.8 |##
  18997.6 |##
  19008.3 |##
  (2 below, 5 above range)

warm-container-kernel (n=40, range 6272.8-7692.9 ns)
   6272.8 |########################################
   6343.8 |
   6414.8 |
   6485.8 |
   6556.8 |
   6627.8 |
   6698.8 |
   6769.8 |
   6840.8 |
   6911.8 |
   6982.8 |
   7053.8 |
   7124.8 |
   7195.9 |
   7266.9 |
   7337.9 |
   7408.9 |
   7479.9 |
   7550.9 |
   7621.9 |#####################################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 6601.4-13600.7 ns)
   6601.4 |########################################
   6951.4 |
   7301.4 |
   7651.3 |
   8001.3 |##
   8351.2 |############################
   8701.2 |
   9051.2 |
   9401.1 |
   9751.1 |
  10101.1 |
  10451.0 |
  10801.0 |
  11151.0 |
  11500.9 |
  11850.9 |
  12200.9 |
  12550.8 |
  12900.8 |
  13250.7 |####################
  (5 below, 3 above range)

warm-container-native (n=40, range 6972.2-13601.0 ns)
   6972.2 |############################
   7303.6 |
   7635.1 |
   7966.5 |########################################
   8298.0 |########################################
   8629.4 |
   8960.8 |
   9292.3 |
   9623.7 |
   9955.2 |
  10286.6 |
  10618.1 |
  10949.5 |
  11280.9 |
  11612.4 |
  11943.8 |
  12275.3 |
  12606.7 |
  12938.2 |
  13269.6 |############################
  (3 below, 3 above range)

warm-container-plusone (n=40, range 15486.6-18943.1 ns)
  15486.6 |#######
  15659.5 |
  15832.3 |#
  16005.1 |
  16177.9 |
  16350.8 |
  16523.6 |
  16696.4 |
  16869.2 |
  17042.1 |
  17214.9 |
  17387.7 |
  17560.5 |
  17733.3 |
  17906.2 |
  18079.0 |
  18251.8 |
  18424.6 |
  18597.5 |
  18770.3 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=32.0% (high variance, measurements may be unstable)
- **warm-container-native**: CV=27.4% (high variance, measurements may be unstable)
