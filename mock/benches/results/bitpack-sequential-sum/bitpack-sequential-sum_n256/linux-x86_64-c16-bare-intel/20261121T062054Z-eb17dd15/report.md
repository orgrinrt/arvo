# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 154% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (215 ns) leads bitpack-aligned-seq (546 ns) by 154%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 61% (significant)

bitpack-native-seq is -333 ns (61%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 2.6x slower than the field

bitpack-zeropad-seq (551 ns) is 2.6x the fastest (215 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq shows alternating (throttle bounce) (autocorr -0.75)

bitpack-native-seq's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-native-seq** at 215.2 ns median (-60.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.56x (fastest 215.2 ns, slowest 551.4 ns)

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
| bitpack-aligned-seq | 780ns | 773ns | 710ns | 771ns | 879ns | base |
| bitpack-native-seq | 466ns | 483ns | 389ns | 455ns | 578ns | -40.24% |
| bitpack-zeropad-seq | 764ns | 777ns | 660ns | 751ns | 907ns | -2.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 543ns | 500ns | 580ns | base | 0.471 |
| bitpack-native-seq | 213ns | 188ns | 235ns | -60.80% | 1.203 |
| bitpack-zeropad-seq | 555ns | 485ns | 644ns | +2.30% | 0.461 |

## Performance model

- Peak throughput: **1.361 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 0.469 | 34.5% |
| bitpack-native-seq | 1.190 | 87.4% |
| bitpack-zeropad-seq | 0.464 | 34.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 780ns | 780ns | base |
| bitpack-native-seq | 466ns | 466ns | -40.24% |
| bitpack-zeropad-seq | 764ns | 764ns | -2.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 546ns | base | --- | [545, 546] | --- | --- | --- | --- |
| bitpack-native-seq | 215ns | -333.2ns (-61.1%) | [-354, -311]ns | [192, 234] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 551ns | no significant difference | [-19, +27]ns | [526, 573] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 546ns | -64.8% | +15.5% |
| 1 | 2 | 545ns | -57.0% | -9.4% |
| 1 | 3 | 545ns | -64.8% | -3.5% |
| 1 | 4 | 556ns | -57.9% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.086 | ok |
| bitpack-native-seq | -0.751 | HIGH- (thermal bounce) |
| bitpack-zeropad-seq | -0.309 | moderate- |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 80.4ns | 543.0ns | 14.8% | HIGH |
| bitpack-native-seq | 93.0ns | 212.8ns | 43.7% | HIGH |
| bitpack-zeropad-seq | 103.6ns | 555.5ns | 18.7% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 500.4-580.5 ns)
    500.4 |
    504.4 |
    508.4 |
    512.4 |###
    516.4 |
    520.4 |
    524.4 |#
    528.4 |
    532.4 |
    536.4 |
    540.4 |
    544.4 |########################################
    548.4 |
    552.4 |
    556.5 |#
    560.5 |
    564.5 |#
    568.5 |
    572.5 |
    576.5 |
  (6 below, 4 above range)

bitpack-native-seq (n=40, range 188.1-234.8 ns)
    188.1 |
    190.4 |########################################
    192.8 |
    195.1 |
    197.4 |
    199.8 |
    202.1 |##
    204.5 |##
    206.8 |
    209.1 |
    211.5 |
    213.8 |
    216.1 |
    218.5 |
    220.8 |
    223.1 |##
    225.5 |
    227.8 |
    230.1 |
    232.5 |###################################
  (2 below, 5 above range)

bitpack-zeropad-seq (n=40, range 485.3-643.5 ns)
    485.3 |############
    493.2 |
    501.1 |####
    509.0 |
    516.9 |########
    524.9 |################################
    532.8 |
    540.7 |####
    548.6 |
    556.5 |####
    564.4 |
    572.3 |########################################
    580.2 |####
    588.2 |
    596.1 |
    604.0 |
    611.9 |####
    619.8 |
    627.7 |####################
    635.6 |####
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: bridge=14.8% of algo (FFI overhead may distort results)
- **bitpack-native-seq**: bridge=38.1% of algo (FFI overhead may distort results)
- **bitpack-zeropad-seq**: bridge=12.8% of algo (FFI overhead may distort results)
