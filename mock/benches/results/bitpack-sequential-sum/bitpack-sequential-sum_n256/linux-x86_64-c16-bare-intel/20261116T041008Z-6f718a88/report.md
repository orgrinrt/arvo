# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 217% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (191 ns) leads bitpack-aligned-seq (605 ns) by 217%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 68% (significant)

bitpack-native-seq is -414 ns (68%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 3.3x slower than the field

bitpack-zeropad-seq (626 ns) is 3.3x the fastest (191 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.3x the fastest

Fastest bitpack-native-seq (191 ns) to slowest bitpack-zeropad-seq (626 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-aligned-seq is inconsistent: worst-20% is 1.9x its best-20%

bitpack-aligned-seq's best 20% of batches run at 476 ns but its worst 20% at 883 ns (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 191.0 ns median (-68.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.28x (fastest 191.0 ns, slowest 626.2 ns)

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
| Counter frequency | 2112815261 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 924ns | 857ns | 673ns | 884ns | 1296ns | base |
| bitpack-native-seq | 411ns | 399ns | 367ns | 406ns | 473ns | -55.48% |
| bitpack-zeropad-seq | 853ns | 854ns | 735ns | 838ns | 1014ns | -7.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 643ns | 476ns | 883ns | base | 0.398 |
| bitpack-native-seq | 192ns | 174ns | 210ns | -70.10% | 1.332 |
| bitpack-zeropad-seq | 623ns | 539ns | 733ns | -3.06% | 0.411 |

## Performance model

- Peak throughput: **1.469 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 0.423 | 28.8% |
| bitpack-native-seq | 1.340 | 91.2% |
| bitpack-zeropad-seq | 0.409 | 27.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 924ns | 924ns | base |
| bitpack-native-seq | 411ns | 411ns | -55.48% |
| bitpack-zeropad-seq | 853ns | 853ns | -7.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 605ns | base | --- | [574, 664] | --- | --- | --- | --- |
| bitpack-native-seq | 191ns | -414.1ns (-68.4%) | [-473, -384]ns | [190, 192] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 626ns | no significant difference | [-34, +82]ns | [588, 631] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 575ns | -66.9% | +9.7% |
| 1 | 2 | 494ns | -64.8% | +32.7% |
| 1 | 3 | 605ns | -68.4% | +1.1% |
| 1 | 4 | 906ns | -76.8% | -40.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | 0.064 | ok |
| bitpack-native-seq | 0.032 | ok |
| bitpack-zeropad-seq | 0.122 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 96.8ns | 642.5ns | 15.1% | HIGH |
| bitpack-native-seq | 76.7ns | 192.1ns | 39.9% | HIGH |
| bitpack-zeropad-seq | 86.2ns | 622.9ns | 13.8% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 475.8-883.3 ns)
    475.8 |##################
    496.1 |
    516.5 |
    536.9 |##################
    557.3 |
    577.7 |
    598.0 |########################################
    618.4 |
    638.8 |###
    659.2 |###
    679.5 |#######
    699.9 |###
    720.3 |
    740.7 |
    761.1 |##########
    781.4 |
    801.8 |
    822.2 |
    842.6 |###
    862.9 |
  (4 below, 6 above range)

bitpack-native-seq (n=40, range 174.3-210.0 ns)
    174.3 |#########
    176.0 |
    177.8 |
    179.6 |
    181.4 |
    183.2 |
    185.0 |
    186.8 |
    188.6 |###########################
    190.3 |########################################
    192.1 |
    193.9 |
    195.7 |
    197.5 |
    199.3 |
    201.1 |
    202.9 |
    204.7 |
    206.4 |
    208.2 |###############
  (5 below, 5 above range)

bitpack-zeropad-seq (n=40, range 539.5-733.5 ns)
    539.5 |
    549.2 |###
    558.9 |
    568.6 |#########################
    578.3 |###
    588.0 |
    597.7 |###
    607.4 |
    617.1 |#########################
    626.8 |########################################
    636.5 |
    646.2 |
    655.9 |
    665.6 |
    675.3 |###
    685.0 |
    694.7 |##############
    704.4 |###
    714.1 |
    723.8 |
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: CV=22.5% (high variance, measurements may be unstable)
- **bitpack-aligned-seq**: bridge=15.3% of algo (FFI overhead may distort results)
- **bitpack-native-seq**: bridge=39.9% of algo (FFI overhead may distort results)
- **bitpack-zeropad-seq**: bridge=13.2% of algo (FFI overhead may distort results)
