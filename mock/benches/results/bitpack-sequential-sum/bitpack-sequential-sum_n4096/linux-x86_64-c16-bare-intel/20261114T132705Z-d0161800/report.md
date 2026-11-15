# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-aligned-seq) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-aligned-seq has the worst median (5.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-native-seq at 2.31 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-native-seq dominates: 117% faster than the next best (bitpack-zeropad-seq)

bitpack-native-seq (2.31 us) leads bitpack-zeropad-seq (5.03 us) by 117%, a clear separation rather than a photo finish. CV 10.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 57% (significant)

bitpack-native-seq is -3.04 us (57%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-aligned-seq is an outlier: 2.3x slower than the field

bitpack-aligned-seq (5.29 us) is 2.3x the fastest (2.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-zeropad-seq shows alternating (throttle bounce) (autocorr -0.64)

bitpack-zeropad-seq's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-aligned-seq is inconsistent: worst-20% is 4.3x its best-20%

bitpack-aligned-seq's best 20% of batches run at 1.87 us but its worst 20% at 7.96 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 2313.1 ns median (-56.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.29x (fastest 2313.1 ns, slowest 5292.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 4923ns | 5438ns | 1925ns | 4813ns | 8250ns | base |
| bitpack-native-seq | 2603ns | 2546ns | 2254ns | 2566ns | 3061ns | -47.13% |
| bitpack-zeropad-seq | 4646ns | 5151ns | 2198ns | 4514ns | 7492ns | -5.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 4773ns | 1874ns | 7965ns | base | 0.858 |
| bitpack-native-seq | 2370ns | 2078ns | 2740ns | -50.33% | 1.728 |
| bitpack-zeropad-seq | 4525ns | 2144ns | 7268ns | -5.19% | 0.905 |

## Performance model

- Peak throughput: **2.186 Gops/s** (bitpack-aligned-seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 0.774 | 35.4% |
| bitpack-native-seq | 1.771 | 81.0% |
| bitpack-zeropad-seq | 0.815 | 37.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 4923ns | 4923ns | base |
| bitpack-native-seq | 2603ns | 2603ns | -47.13% |
| bitpack-zeropad-seq | 4646ns | 4646ns | -5.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 5293ns | base | --- | [2380, 6121] | --- | --- | --- | --- |
| bitpack-native-seq | 2313ns | -3038.7ns (-57.4%) | [-3590, -307]ns | [2280, 2437] | YES | 0.0014 | 0.0007 | 0 |
| bitpack-zeropad-seq | 5026ns | no significant difference | [-140, +325]ns | [2154, 6399] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 5004ns | -53.8% | -27.2% |
| 1 | 2 | 3764ns | -43.3% | +57.4% |
| 1 | 3 | 4031ns | -38.9% | -12.3% |
| 1 | 4 | 7221ns | -65.8% | -40.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.128 | ok |
| bitpack-native-seq | -0.043 | ok |
| bitpack-zeropad-seq | -0.636 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 49.9ns | 4772.8ns | 1.0% |  |
| bitpack-native-seq | 77.9ns | 2370.5ns | 3.3% |  |
| bitpack-zeropad-seq | 40.7ns | 4525.1ns | 0.9% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 1873.8-7964.9 ns)
   1873.8 |#####
   2178.3 |########################################
   2482.9 |
   2787.4 |
   3092.0 |
   3396.5 |#####
   3701.1 |
   4005.7 |
   4310.2 |#####
   4614.8 |#####
   4919.3 |###########
   5223.9 |###########
   5528.4 |#####
   5833.0 |#################
   6137.5 |######################
   6442.1 |#####
   6746.6 |###########
   7051.2 |#####
   7355.8 |###########
   7660.3 |
  (7 below, 4 above range)

bitpack-native-seq (n=40, range 2078.0-2739.7 ns)
   2078.0 |
   2111.1 |############################
   2144.2 |#####
   2177.2 |###########
   2210.3 |#####
   2243.4 |#####
   2276.5 |###########
   2309.6 |########################################
   2342.7 |#################
   2375.8 |#####
   2408.8 |
   2441.9 |#################
   2475.0 |
   2508.1 |###########
   2541.2 |###########
   2574.3 |
   2607.4 |#####
   2640.4 |
   2673.5 |###########
   2706.6 |
  (3 below, 4 above range)

bitpack-zeropad-seq (n=40, range 2144.2-7267.8 ns)
   2144.2 |########################################
   2400.3 |
   2656.5 |
   2912.7 |
   3168.9 |
   3425.1 |##
   3681.2 |
   3937.4 |
   4193.6 |
   4449.8 |
   4705.9 |##
   4962.1 |##
   5218.3 |
   5474.5 |
   5730.7 |#####
   5986.9 |
   6243.0 |#############
   6499.2 |#############
   6755.4 |##
   7011.6 |##
  (3 below, 5 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: CV=49.4% (high variance, measurements may be unstable)
- **bitpack-aligned-seq**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **bitpack-zeropad-seq**: CV=49.9% (high variance, measurements may be unstable)
- **bitpack-zeropad-seq**: worst_20/best_20 = 3.4x (possible bimodal distribution)
