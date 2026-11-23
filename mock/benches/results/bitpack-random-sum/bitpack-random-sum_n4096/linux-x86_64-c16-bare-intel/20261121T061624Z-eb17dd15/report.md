# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-aligned-rand) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-aligned-rand has the worst median (5.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-native-rand at 3.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-native-rand dominates: 27% faster than the next best (bitpack-zeropad-rand)

bitpack-native-rand (3.61 us) leads bitpack-zeropad-rand (4.60 us) by 27%, a clear separation rather than a photo finish. CV 27.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 21% (significant)

bitpack-native-rand is -1.08 us (21%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-native-rand shows alternating (throttle bounce) (autocorr -0.52)

bitpack-native-rand's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-aligned-rand is inconsistent: worst-20% is 3.7x its best-20%

bitpack-aligned-rand's best 20% of batches run at 1.88 us but its worst 20% at 6.94 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 3614.5 ns median (-28.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.40x (fastest 3614.5 ns, slowest 5067.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112835320 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 4321ns | 5205ns | 1931ns | 4171ns | 7163ns | base |
| bitpack-native-rand | 3717ns | 3792ns | 2150ns | 3812ns | 4999ns | -13.98% |
| bitpack-zeropad-rand | 5462ns | 4651ns | 4643ns | 4650ns | 8718ns | +26.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 4197ns | 1878ns | 6939ns | base | 0.976 |
| bitpack-native-rand | 3533ns | 2050ns | 4721ns | -15.82% | 1.159 |
| bitpack-zeropad-rand | 5394ns | 4591ns | 8587ns | +28.51% | 0.759 |

## Performance model

- Peak throughput: **2.181 Gops/s** (bitpack-aligned-rand; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 0.808 | 37.1% |
| bitpack-native-rand | 1.133 | 52.0% |
| bitpack-zeropad-rand | 0.891 | 40.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 4321ns | 4321ns | base |
| bitpack-native-rand | 3717ns | 3717ns | -13.98% |
| bitpack-zeropad-rand | 5462ns | 5462ns | +26.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 5068ns | base | --- | [1890, 5503] | --- | --- | --- | --- |
| bitpack-native-rand | 3614ns | -1078.4ns (-21.3%) | [-1565, -58]ns | [3369, 3852] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| bitpack-zeropad-rand | 4597ns | no significant difference | [-580, +2713]ns | [4593, 4602] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 4925ns | -11.2% | -6.7% |
| 1 | 2 | 3612ns | -7.9% | +27.2% |
| 1 | 3 | 3226ns | +19.4% | +42.6% |
| 1 | 4 | 5179ns | -30.2% | -11.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.271 | moderate- |
| bitpack-native-rand | -0.523 | HIGH- (thermal bounce) |
| bitpack-zeropad-rand | 0.214 | moderate+ |

**Consistency summary:**

- **bitpack-native-rand**: won 3/4, lost 1/4
- **bitpack-zeropad-rand**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 41.7ns | 4197.2ns | 1.0% |  |
| bitpack-native-rand | 62.9ns | 3533.1ns | 1.8% |  |
| bitpack-zeropad-rand | 23.3ns | 5394.0ns | 0.4% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 1878.4-6938.6 ns)
   1878.4 |########################################
   2131.4 |
   2384.4 |
   2637.4 |
   2890.4 |
   3143.4 |
   3396.4 |
   3649.5 |
   3902.5 |
   4155.5 |##
   4408.5 |##
   4661.5 |
   4914.5 |#####
   5167.5 |########
   5420.6 |#############
   5673.6 |#####
   5926.6 |##########
   6179.6 |##
   6432.6 |#####
   6685.6 |##
  (2 below, 1 above range)

bitpack-native-rand (n=40, range 2049.7-4721.1 ns)
   2049.7 |########
   2183.2 |
   2316.8 |
   2450.4 |
   2583.9 |
   2717.5 |
   2851.1 |
   2984.7 |################################
   3118.2 |########
   3251.8 |########################################
   3385.4 |################
   3519.0 |################################
   3652.6 |################################
   3786.1 |################
   3919.7 |########################
   4053.3 |
   4186.9 |########
   4320.4 |########################
   4454.0 |################
   4587.6 |########
  (4 below, 3 above range)

bitpack-zeropad-rand (n=40, range 4591.0-8587.0 ns)
   4591.0 |########################################
   4790.8 |
   4990.6 |
   5190.4 |
   5390.2 |
   5590.0 |
   5789.8 |
   5989.6 |
   6189.4 |
   6389.2 |#
   6589.0 |#
   6788.8 |
   6988.6 |
   7188.4 |
   7388.2 |
   7588.0 |
   7787.8 |
   7987.6 |
   8187.4 |
   8387.2 |
  (5 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: CV=52.7% (high variance, measurements may be unstable)
- **bitpack-aligned-rand**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-native-rand**: CV=27.6% (high variance, measurements may be unstable)
- **bitpack-zeropad-rand**: CV=45.9% (high variance, measurements may be unstable)
