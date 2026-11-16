# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-aligned-seq is fastest but the noisiest (CV 120.2%)

bitpack-aligned-seq wins on median (2.04 us) yet has the highest variance (CV 120.2%), while bitpack-native-seq is the steadiest (CV 8.8%, 2.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-aligned-seq shows alternating (throttle bounce) (autocorr -0.76)

bitpack-aligned-seq's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (268 ns) is smaller than the fastest variant's own run-to-run std-dev (2.46 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-aligned-seq)

The baseline bitpack-aligned-seq is the fastest (2.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bitpack-aligned-seq is inconsistent: worst-20% is 4.0x its best-20%

bitpack-aligned-seq's best 20% of batches run at 1.87 us but its worst 20% at 7.43 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-aligned-seq) is the fastest** at 2044.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 2044.8 ns, slowest 2313.1 ns)

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
| bitpack-aligned-seq | 4213ns | 2101ns | 1924ns | 3822ns | 7677ns | base |
| bitpack-native-seq | 2534ns | 2509ns | 2231ns | 2508ns | 2916ns | -39.85% |
| bitpack-zeropad-seq | 4388ns | 2208ns | 2198ns | 3910ns | 8012ns | +4.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 4092ns | 1872ns | 7430ns | base | 1.001 |
| bitpack-native-seq | 2321ns | 2061ns | 2616ns | -43.28% | 1.765 |
| bitpack-zeropad-seq | 4273ns | 2144ns | 7778ns | +4.43% | 0.959 |

## Performance model

- Peak throughput: **2.188 Gops/s** (bitpack-aligned-seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.003 | 91.6% |
| bitpack-native-seq | 1.771 | 80.9% |
| bitpack-zeropad-seq | 1.902 | 86.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 4213ns | 4213ns | base |
| bitpack-native-seq | 2534ns | 2534ns | -39.85% |
| bitpack-zeropad-seq | 4388ns | 4388ns | +4.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 2045ns | base | --- | [1873, 6125] | --- | --- | --- | --- |
| bitpack-native-seq | 2313ns | no significant difference | [-3684, +262]ns | [2239, 2354] | no | 0.8746 | 0.8746 | 0 |
| bitpack-zeropad-seq | 2154ns | +271.9ns (+13.3%) | [+272, +276]ns | [2145, 6390] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 1951ns | +26.3% | +10.2% |
| 1 | 2 | 4013ns | -42.4% | -46.5% |
| 1 | 3 | 1971ns | +7.9% | +43.4% |
| 1 | 4 | 3865ns | -40.1% | +50.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.764 | HIGH- (thermal bounce) |
| bitpack-native-seq | -0.031 | ok |
| bitpack-zeropad-seq | 0.063 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 2/4, lost 2/4
- **bitpack-zeropad-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 41.5ns | 4091.7ns | 1.0% |  |
| bitpack-native-seq | 70.9ns | 2320.7ns | 3.1% |  |
| bitpack-zeropad-seq | 38.6ns | 4272.9ns | 0.9% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 1872.4-7430.1 ns)
   1872.4 |########################################
   2150.3 |##
   2428.2 |
   2706.1 |
   2983.9 |
   3261.8 |
   3539.7 |
   3817.6 |
   4095.5 |
   4373.3 |
   4651.2 |
   4929.1 |
   5207.0 |
   5484.9 |##
   5762.8 |#####
   6040.6 |#####
   6318.5 |##########
   6596.4 |##
   6874.3 |########
   7152.2 |##
  (6 below, 4 above range)

bitpack-native-seq (n=40, range 2061.2-2616.4 ns)
   2061.2 |
   2089.0 |
   2116.7 |########################
   2144.5 |########
   2172.3 |####
   2200.0 |####
   2227.8 |
   2255.5 |####
   2283.3 |
   2311.1 |########################################
   2338.8 |########
   2366.6 |####
   2394.3 |####
   2422.1 |
   2449.9 |
   2477.6 |####
   2505.4 |####################
   2533.2 |####
   2560.9 |####
   2588.7 |
  (4 below, 3 above range)

bitpack-zeropad-seq (n=40, range 2144.2-7777.8 ns)
   2144.2 |########################################
   2425.9 |
   2707.6 |
   2989.2 |
   3270.9 |##
   3552.6 |
   3834.3 |
   4116.0 |
   4397.6 |
   4679.3 |
   4961.0 |
   5242.7 |####
   5524.4 |
   5806.0 |
   6087.7 |##
   6369.4 |####
   6651.1 |####
   6932.8 |##
   7214.4 |######
   7496.1 |####
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: CV=60.1% (high variance, measurements may be unstable)
- **bitpack-aligned-seq**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **bitpack-zeropad-seq**: CV=57.9% (high variance, measurements may be unstable)
- **bitpack-zeropad-seq**: worst_20/best_20 = 3.6x (possible bimodal distribution)
