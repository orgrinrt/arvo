# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-aligned-rand dominates: 89% faster than the next best (bitpack-native-rand)

bitpack-aligned-rand (1.89 us) leads bitpack-native-rand (3.56 us) by 89%, a clear separation rather than a photo finish. CV 131.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-zeropad-rand is an outlier: 2.4x slower than the field

bitpack-zeropad-rand (4.60 us) is 2.4x the fastest (1.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-aligned-rand is fastest but the noisiest (CV 131.3%)

bitpack-aligned-rand wins on median (1.89 us) yet has the highest variance (CV 131.3%), while bitpack-native-rand is the steadiest (CV 12.6%, 3.56 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-aligned-rand)

The baseline bitpack-aligned-rand is the fastest (1.89 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bitpack-aligned-rand is inconsistent: worst-20% is 4.0x its best-20%

bitpack-aligned-rand's best 20% of batches run at 1.87 us but its worst 20% at 7.46 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-aligned-rand) is the fastest** at 1885.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.44x (fastest 1885.2 ns, slowest 4599.4 ns)

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
| Counter frequency | 2112826911 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 4111ns | 1938ns | 1927ns | 3635ns | 7722ns | base |
| bitpack-native-rand | 3787ns | 3739ns | 3174ns | 3771ns | 4446ns | -7.88% |
| bitpack-zeropad-rand | 5747ns | 4651ns | 4642ns | 4650ns | 10146ns | +39.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 3988ns | 1874ns | 7463ns | base | 1.027 |
| bitpack-native-rand | 3591ns | 3018ns | 4185ns | -9.95% | 1.141 |
| bitpack-zeropad-rand | 5675ns | 4591ns | 9992ns | +42.30% | 0.722 |

## Performance model

- Peak throughput: **2.186 Gops/s** (bitpack-aligned-rand; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 2.173 | 99.4% |
| bitpack-native-rand | 1.151 | 52.7% |
| bitpack-zeropad-rand | 0.891 | 40.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 4111ns | 4111ns | base |
| bitpack-native-rand | 3787ns | 3787ns | -7.88% |
| bitpack-zeropad-rand | 5747ns | 5747ns | +39.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 1885ns | base | --- | [1875, 6221] | --- | --- | --- | --- |
| bitpack-native-rand | 3559ns | no significant difference | [-2346, +1365]ns | [3466, 3760] | no | 0.4296 | 0.4296 | 0 |
| bitpack-zeropad-rand | 4599ns | +2716.8ns (+144.1%) | [+2431, +2722]ns | [4593, 4602] | YES | 0.0129 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 1874ns | +88.3% | +145.3% |
| 1 | 2 | 2514ns | +37.9% | +83.0% |
| 1 | 3 | 2902ns | +20.7% | +58.4% |
| 1 | 4 | 1881ns | +101.2% | +144.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.273 | moderate- |
| bitpack-native-rand | -0.046 | ok |
| bitpack-zeropad-rand | -0.388 | moderate- |

**Consistency summary:**

- **bitpack-native-rand**: won 0/4, lost 4/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 41.5ns | 3988.0ns | 1.0% |  |
| bitpack-native-rand | 68.6ns | 3591.2ns | 1.9% |  |
| bitpack-zeropad-rand | 24.7ns | 5674.9ns | 0.4% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 1874.1-7463.4 ns)
   1874.1 |########################################
   2153.6 |
   2433.0 |
   2712.5 |
   2991.9 |##
   3271.4 |
   3550.9 |
   3830.3 |##
   4109.8 |
   4389.3 |
   4668.7 |
   4948.2 |
   5227.7 |
   5507.1 |##
   5786.6 |
   6066.0 |######
   6345.5 |##
   6625.0 |####
   6904.4 |######
   7183.9 |####
  (2 below, 4 above range)

bitpack-native-rand (n=40, range 3017.6-4185.1 ns)
   3017.6 |########
   3076.0 |
   3134.4 |########################
   3192.8 |################################
   3251.1 |################
   3309.5 |########
   3367.9 |
   3426.3 |########################
   3484.6 |################################
   3543.0 |########################
   3601.4 |########
   3659.8 |
   3718.1 |########################################
   3776.5 |################
   3834.9 |########
   3893.3 |
   3951.6 |
   4010.0 |################################
   4068.4 |################
   4126.8 |########
  (1 below, 2 above range)

bitpack-zeropad-rand (n=40, range 4590.6-9992.2 ns)
   4590.6 |########################################
   4860.7 |
   5130.8 |
   5400.8 |
   5670.9 |
   5941.0 |
   6211.1 |#
   6481.2 |
   6751.2 |
   7021.3 |
   7291.4 |
   7561.5 |
   7831.6 |
   8101.6 |
   8371.7 |
   8641.8 |
   8911.9 |
   9181.9 |
   9452.0 |#
   9722.1 |
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: CV=62.1% (high variance, measurements may be unstable)
- **bitpack-aligned-rand**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **bitpack-zeropad-rand**: CV=49.2% (high variance, measurements may be unstable)
