# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 194% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (851 ns) leads bitpack-aligned-seq (2.50 us) by 194%, a clear separation rather than a photo finish. CV 14.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 67% (significant)

bitpack-native-seq is -1.68 us (67%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 3.4x slower than the field

bitpack-zeropad-seq (2.87 us) is 3.4x the fastest (851 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest bitpack-native-seq (851 ns) to slowest bitpack-zeropad-seq (2.87 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-aligned-seq is inconsistent: worst-20% is 4.7x its best-20%

bitpack-aligned-seq's best 20% of batches run at 1.99 us but its worst 20% at 9.34 us (4.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 851.1 ns median (-66.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.37x (fastest 851.1 ns, slowest 2868.2 ns)

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
| Counter frequency | 2100219680 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 3886ns | 2609ns | 2077ns | 2623ns | 9486ns | base |
| bitpack-native-seq | 991ns | 958ns | 862ns | 962ns | 1206ns | -74.50% |
| bitpack-zeropad-seq | 2912ns | 2973ns | 2376ns | 2893ns | 3503ns | -25.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 3775ns | 1993ns | 9342ns | base | 1.085 |
| bitpack-native-seq | 875ns | 763ns | 1070ns | -76.81% | 4.679 |
| bitpack-zeropad-seq | 2795ns | 2283ns | 3361ns | -25.96% | 1.466 |

## Performance model

- Peak throughput: **5.366 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 1.636 | 30.5% |
| bitpack-native-seq | 4.813 | 89.7% |
| bitpack-zeropad-seq | 1.428 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 3886ns | 3886ns | base |
| bitpack-native-seq | 991ns | 991ns | -74.50% |
| bitpack-zeropad-seq | 2912ns | 2912ns | -25.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 2503ns | base | --- | [2503, 2561] | --- | --- | --- | --- |
| bitpack-native-seq | 851ns | -1677.0ns (-67.0%) | [-1744, -1562]ns | [843, 852] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 2868ns | no significant difference | [-7, +368]ns | [2604, 2949] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 2554ns | -70.3% | -10.1% |
| 1 | 2 | 2534ns | -67.2% | +4.3% |
| 1 | 3 | 1998ns | -57.4% | +58.7% |
| 1 | 4 | 2567ns | -66.8% | +14.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.421 | moderate- |
| bitpack-native-seq | 0.082 | ok |
| bitpack-zeropad-seq | 0.190 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 41.1ns | 3774.6ns | 1.1% |  |
| bitpack-native-seq | 42.0ns | 875.5ns | 4.8% |  |
| bitpack-zeropad-seq | 40.4ns | 2794.6ns | 1.4% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 1993.1-9341.6 ns)
   1993.1 |#########
   2360.5 |########################################
   2727.9 |###
   3095.3 |
   3462.8 |
   3830.2 |
   4197.6 |
   4565.1 |
   4932.5 |
   5299.9 |
   5667.3 |
   6034.8 |
   6402.2 |
   6769.6 |
   7137.1 |
   7504.5 |
   7871.9 |
   8239.4 |
   8606.8 |
   8974.2 |
  (4 below, 2 above range)

bitpack-native-seq (n=40, range 763.4-1069.6 ns)
    763.4 |
    778.7 |##
    794.0 |
    809.3 |
    824.6 |###############
    839.9 |########################################
    855.2 |##
    870.5 |
    885.9 |
    901.2 |#####
    916.5 |##
    931.8 |##
    947.1 |
    962.4 |
    977.7 |
    993.0 |##
   1008.3 |
   1023.7 |
   1039.0 |
   1054.3 |#####
  (7 below, 2 above range)

bitpack-zeropad-seq (n=40, range 2283.4-3361.1 ns)
   2283.4 |########################
   2337.3 |################
   2391.2 |
   2445.0 |########
   2498.9 |########################
   2552.8 |########
   2606.7 |########
   2660.6 |################################
   2714.5 |
   2768.3 |
   2822.2 |########################################
   2876.1 |########
   2930.0 |########################
   2983.9 |########################
   3037.8 |########
   3091.6 |########
   3145.5 |################
   3199.4 |################
   3253.3 |
   3307.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: CV=175.5% (high variance, measurements may be unstable)
- **bitpack-aligned-seq**: worst_20/best_20 = 4.7x (possible bimodal distribution)
