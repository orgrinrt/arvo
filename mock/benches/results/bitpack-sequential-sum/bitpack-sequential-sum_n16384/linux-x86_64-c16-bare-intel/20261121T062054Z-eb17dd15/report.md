# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 198% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (2.51 us) leads bitpack-aligned-seq (7.48 us) by 198%, a clear separation rather than a photo finish. CV 96.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 66% (significant)

bitpack-native-seq is -4.97 us (66%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 3.4x slower than the field

bitpack-zeropad-seq (8.55 us) is 3.4x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq is fastest but the noisiest (CV 96.9%)

bitpack-native-seq wins on median (2.51 us) yet has the highest variance (CV 96.9%), while bitpack-aligned-seq is the steadiest (CV 0.1%, 7.48 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.4x the fastest

Fastest bitpack-native-seq (2.51 us) to slowest bitpack-zeropad-seq (8.55 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-seq is inconsistent: worst-20% is 3.2x its best-20%

bitpack-native-seq's best 20% of batches run at 2.50 us but its worst 20% at 7.96 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 2506.8 ns median (-66.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.41x (fastest 2506.8 ns, slowest 8553.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112768523 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 7539ns | 7530ns | 7520ns | 7530ns | 7586ns | base |
| bitpack-native-seq | 4690ns | 2559ns | 2548ns | 4247ns | 8163ns | -37.79% |
| bitpack-zeropad-seq | 8618ns | 8608ns | 8601ns | 8608ns | 8666ns | +14.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 7477ns | 7468ns | 7486ns | base | 2.191 |
| bitpack-native-seq | 4587ns | 2496ns | 7958ns | -38.65% | 3.572 |
| bitpack-zeropad-seq | 8560ns | 8546ns | 8593ns | +14.48% | 1.914 |

## Performance model

- Peak throughput: **6.564 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.191 | 33.4% |
| bitpack-native-seq | 6.536 | 99.6% |
| bitpack-zeropad-seq | 1.916 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 7539ns | 7539ns | base |
| bitpack-native-seq | 4690ns | 4690ns | -37.79% |
| bitpack-zeropad-seq | 8618ns | 8618ns | +14.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 7478ns | base | --- | [7478, 7478] | --- | --- | --- | --- |
| bitpack-native-seq | 2507ns | -4971.1ns (-66.5%) | [-4972, -720]ns | [2497, 6770] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 8553ns | +1075.9ns (+14.4%) | [+1074, +1079]ns | [8552, 8554] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 7478ns | -63.2% | +14.4% |
| 1 | 2 | 7478ns | -8.1% | +14.4% |
| 1 | 3 | 7478ns | -62.5% | +14.4% |
| 1 | 4 | 7478ns | -66.5% | +14.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.326 | moderate- |
| bitpack-native-seq | -0.367 | moderate- |
| bitpack-zeropad-seq | 0.113 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 18.6ns | 7477.1ns | 0.2% |  |
| bitpack-native-seq | 36.0ns | 4587.1ns | 0.8% |  |
| bitpack-zeropad-seq | 19.3ns | 8560.0ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 7468.4-7485.9 ns)
   7468.4 |##################
   7469.3 |
   7470.1 |
   7471.0 |
   7471.9 |
   7472.8 |
   7473.6 |
   7474.5 |
   7475.4 |
   7476.3 |
   7477.1 |########################################
   7478.0 |##############################
   7478.9 |###############
   7479.8 |
   7480.6 |
   7481.5 |
   7482.4 |
   7483.3 |
   7484.1 |
   7485.0 |
  (4 below, 2 above range)

bitpack-native-seq (n=40, range 2496.1-7958.5 ns)
   2496.1 |########################################
   2769.2 |##
   3042.3 |##
   3315.4 |
   3588.6 |
   3861.7 |
   4134.8 |
   4407.9 |
   4681.0 |
   4954.1 |
   5227.3 |
   5500.4 |##
   5773.5 |
   6046.6 |
   6319.7 |##
   6592.9 |#######
   6866.0 |####
   7139.1 |#########
   7412.2 |##
   7685.3 |####
  (4 below, 3 above range)

bitpack-zeropad-seq (n=40, range 8546.3-8593.4 ns)
   8546.3 |
   8548.7 |##
   8551.1 |########################################
   8553.4 |######################
   8555.8 |##
   8558.1 |##########
   8560.5 |
   8562.8 |##
   8565.2 |#####
   8567.5 |
   8569.9 |
   8572.2 |
   8574.6 |
   8576.9 |
   8579.3 |
   8581.6 |
   8584.0 |
   8586.4 |
   8588.7 |
   8591.1 |
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-native-seq**: CV=52.9% (high variance, measurements may be unstable)
- **bitpack-native-seq**: worst_20/best_20 = 3.2x (possible bimodal distribution)
