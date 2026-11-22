# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 198% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (2.51 us) leads bitpack-aligned-seq (7.48 us) by 198%, a clear separation rather than a photo finish. CV 94.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 66% (significant)

bitpack-native-seq is -4.97 us (66%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 3.4x slower than the field

bitpack-zeropad-seq (8.55 us) is 3.4x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq is fastest but the noisiest (CV 94.2%)

bitpack-native-seq wins on median (2.51 us) yet has the highest variance (CV 94.2%), while bitpack-zeropad-seq is the steadiest (CV 0.1%, 8.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.4x the fastest

Fastest bitpack-native-seq (2.51 us) to slowest bitpack-zeropad-seq (8.55 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-seq is inconsistent: worst-20% is 3.1x its best-20%

bitpack-native-seq's best 20% of batches run at 2.50 us but its worst 20% at 7.79 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 2506.6 ns median (-66.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.41x (fastest 2506.6 ns, slowest 8554.6 ns)

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
| bitpack-aligned-seq | 7590ns | 7538ns | 7528ns | 7537ns | 7810ns | base |
| bitpack-native-seq | 4338ns | 2561ns | 2549ns | 3716ns | 7992ns | -42.85% |
| bitpack-zeropad-seq | 8617ns | 8615ns | 8602ns | 8615ns | 8639ns | +13.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 7525ns | 7468ns | 7724ns | base | 2.177 |
| bitpack-native-seq | 4239ns | 2496ns | 7790ns | -43.66% | 3.865 |
| bitpack-zeropad-seq | 8555ns | 8547ns | 8563ns | +13.69% | 1.915 |

## Performance model

- Peak throughput: **6.565 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.191 | 33.4% |
| bitpack-native-seq | 6.536 | 99.6% |
| bitpack-zeropad-seq | 1.915 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 7590ns | 7590ns | base |
| bitpack-native-seq | 4338ns | 4338ns | -42.85% |
| bitpack-zeropad-seq | 8617ns | 8617ns | +13.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 7478ns | base | --- | [7478, 7478] | --- | --- | --- | --- |
| bitpack-native-seq | 2507ns | -4970.1ns (-66.5%) | [-4972, -2904]ns | [2498, 4718] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 8555ns | +1077.0ns (+14.4%) | [+1076, +1079]ns | [8554, 8556] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 7478ns | -66.5% | +14.4% |
| 1 | 2 | 7478ns | -64.3% | +14.4% |
| 1 | 3 | 7478ns | -66.5% | +14.4% |
| 1 | 4 | 7478ns | -66.6% | +14.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | -0.492 | moderate- |
| bitpack-native-seq | -0.386 | moderate- |
| bitpack-zeropad-seq | -0.064 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 19.2ns | 7524.7ns | 0.3% |  |
| bitpack-native-seq | 32.7ns | 4239.5ns | 0.8% |  |
| bitpack-zeropad-seq | 19.7ns | 8555.1ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 7468.2-7724.5 ns)
   7468.2 |########################################
   7481.0 |##
   7493.8 |#
   7506.6 |
   7519.4 |
   7532.2 |
   7545.1 |
   7557.9 |
   7570.7 |
   7583.5 |
   7596.3 |
   7609.1 |
   7621.9 |
   7634.8 |
   7647.6 |
   7660.4 |
   7673.2 |
   7686.0 |
   7698.8 |
   7711.6 |
  (3 below, 2 above range)

bitpack-native-seq (n=40, range 2495.8-7790.1 ns)
   2495.8 |########################################
   2760.6 |####
   3025.3 |
   3290.0 |
   3554.7 |
   3819.4 |
   4084.1 |
   4348.8 |
   4613.5 |
   4878.3 |
   5143.0 |
   5407.7 |
   5672.4 |
   5937.1 |
   6201.8 |
   6466.5 |##
   6731.2 |######
   6996.0 |####
   7260.7 |######
   7525.4 |####
  (4 below, 3 above range)

bitpack-zeropad-seq (n=40, range 8547.0-8563.0 ns)
   8547.0 |
   8547.8 |
   8548.6 |
   8549.4 |
   8550.2 |
   8551.0 |
   8551.8 |####
   8552.6 |####
   8553.4 |################
   8554.2 |########################################
   8555.0 |########
   8555.8 |########################
   8556.6 |
   8557.4 |####
   8558.2 |####
   8559.0 |################
   8559.8 |####
   8560.6 |####
   8561.4 |####
   8562.2 |
  (6 below, 1 above range)

```

## Diagnostics

- **bitpack-native-seq**: CV=55.7% (high variance, measurements may be unstable)
- **bitpack-native-seq**: worst_20/best_20 = 3.1x (possible bimodal distribution)
