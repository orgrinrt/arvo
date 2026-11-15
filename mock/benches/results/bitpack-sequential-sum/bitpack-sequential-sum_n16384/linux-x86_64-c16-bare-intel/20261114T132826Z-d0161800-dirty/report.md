# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 198% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (2.51 us) leads bitpack-aligned-seq (7.48 us) by 198%, a clear separation rather than a photo finish. CV 99.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 66% (significant)

bitpack-native-seq is -4.97 us (66%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 3.4x slower than the field

bitpack-zeropad-seq (8.56 us) is 3.4x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq is fastest but the noisiest (CV 99.3%)

bitpack-native-seq wins on median (2.51 us) yet has the highest variance (CV 99.3%), while bitpack-aligned-seq is the steadiest (CV 6.7%, 7.48 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.4x the fastest

Fastest bitpack-native-seq (2.51 us) to slowest bitpack-zeropad-seq (8.56 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-seq is inconsistent: worst-20% is 3.3x its best-20%

bitpack-native-seq's best 20% of batches run at 2.50 us but its worst 20% at 8.17 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 2505.7 ns median (-66.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.41x (fastest 2505.7 ns, slowest 8555.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 7621ns | 7537ns | 7528ns | 7537ns | 7967ns | base |
| bitpack-native-seq | 4443ns | 2557ns | 2547ns | 3760ns | 8389ns | -41.70% |
| bitpack-zeropad-seq | 9123ns | 8619ns | 8606ns | 8826ns | 10534ns | +19.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 7556ns | 7468ns | 7880ns | base | 2.168 |
| bitpack-native-seq | 4344ns | 2496ns | 8174ns | -42.52% | 3.772 |
| bitpack-zeropad-seq | 9056ns | 8549ns | 10458ns | +19.86% | 1.809 |

## Performance model

- Peak throughput: **6.565 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.191 | 33.4% |
| bitpack-native-seq | 6.539 | 99.6% |
| bitpack-zeropad-seq | 1.915 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 7621ns | 7621ns | base |
| bitpack-native-seq | 4443ns | 4443ns | -41.70% |
| bitpack-zeropad-seq | 9123ns | 9123ns | +19.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 7478ns | base | --- | [7478, 7478] | --- | --- | --- | --- |
| bitpack-native-seq | 2506ns | -4972.0ns (-66.5%) | [-4973, -2796]ns | [2496, 4689] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 8556ns | +1078.4ns (+14.4%) | [+1076, +1087]ns | [8555, 8557] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 7478ns | -55.5% | +39.8% |
| 1 | 2 | 7478ns | -66.5% | +14.4% |
| 1 | 3 | 7478ns | -66.5% | +14.4% |
| 1 | 4 | 7478ns | -66.6% | +14.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.339 | moderate- |
| bitpack-native-seq | -0.080 | ok |
| bitpack-zeropad-seq | -0.083 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 19.4ns | 7556.0ns | 0.3% |  |
| bitpack-native-seq | 33.6ns | 4343.5ns | 0.8% |  |
| bitpack-zeropad-seq | 20.9ns | 9056.3ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 7468.0-7880.4 ns)
   7468.0 |########################################
   7488.6 |#
   7509.2 |
   7529.9 |
   7550.5 |
   7571.1 |
   7591.7 |
   7612.3 |
   7633.0 |
   7653.6 |
   7674.2 |
   7694.8 |
   7715.4 |
   7736.1 |
   7756.7 |
   7777.3 |
   7797.9 |
   7818.6 |
   7839.2 |
   7859.8 |
  (4 below, 1 above range)

bitpack-native-seq (n=40, range 2495.7-8173.5 ns)
   2495.7 |########################################
   2779.5 |
   3063.4 |
   3347.3 |
   3631.2 |
   3915.1 |#
   4199.0 |
   4482.9 |
   4766.8 |
   5050.7 |#
   5334.6 |
   5618.5 |
   5902.4 |
   6186.3 |
   6470.2 |
   6754.1 |
   7038.0 |#####
   7321.8 |###
   7605.7 |#
   7889.6 |#####
  (4 below, 4 above range)

bitpack-zeropad-seq (n=40, range 8548.5-10457.7 ns)
   8548.5 |########################################
   8644.0 |
   8739.4 |
   8834.9 |
   8930.4 |
   9025.8 |
   9121.3 |
   9216.7 |
   9312.2 |
   9407.7 |
   9503.1 |
   9598.6 |#
   9694.0 |
   9789.5 |
   9885.0 |
   9980.4 |
  10075.9 |
  10171.3 |
  10266.8 |
  10362.3 |############
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-native-seq**: CV=57.3% (high variance, measurements may be unstable)
- **bitpack-native-seq**: worst_20/best_20 = 3.3x (possible bimodal distribution)
