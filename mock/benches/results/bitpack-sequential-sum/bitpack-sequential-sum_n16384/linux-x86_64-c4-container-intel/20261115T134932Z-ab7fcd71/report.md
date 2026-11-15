# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-aligned-seq) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-aligned-seq has the worst median (10.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-native-seq at 3.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-native-seq dominates: 239% faster than the next best (bitpack-zeropad-seq)

bitpack-native-seq (3.11 us) leads bitpack-zeropad-seq (10.57 us) by 239%, a clear separation rather than a photo finish. CV 207.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 70% (significant)

bitpack-native-seq is -7.43 us (70%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-aligned-seq is an outlier: 3.4x slower than the field

bitpack-aligned-seq (10.57 us) is 3.4x the fastest (3.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq is fastest but the noisiest (CV 207.0%)

bitpack-native-seq wins on median (3.11 us) yet has the highest variance (CV 207.0%), while bitpack-aligned-seq is the steadiest (CV 9.6%, 10.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-native-seq shows alternating (throttle bounce) (autocorr -0.64)

bitpack-native-seq's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest bitpack-native-seq (3.11 us) to slowest bitpack-aligned-seq (10.57 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-seq is inconsistent: worst-20% is 3.5x its best-20%

bitpack-native-seq's best 20% of batches run at 2.68 us but its worst 20% at 9.29 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-seq** at 3114.0 ns median (-70.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.39x (fastest 3114.0 ns, slowest 10568.5 ns)

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
| bitpack-aligned-seq | 11088ns | 10740ns | 10260ns | 10901ns | 12476ns | base |
| bitpack-native-seq | 4429ns | 3258ns | 2771ns | 3300ns | 9475ns | -60.06% |
| bitpack-zeropad-seq | 11509ns | 10684ns | 9191ns | 10943ns | 15522ns | +3.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 10914ns | 10123ns | 12263ns | base | 1.501 |
| bitpack-native-seq | 4293ns | 2680ns | 9294ns | -60.67% | 3.816 |
| bitpack-zeropad-seq | 11371ns | 9093ns | 15329ns | +4.18% | 1.441 |

## Performance model

- Peak throughput: **6.113 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 1.550 | 25.4% |
| bitpack-native-seq | 5.261 | 86.1% |
| bitpack-zeropad-seq | 1.551 | 25.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 11088ns | 11088ns | base |
| bitpack-native-seq | 4429ns | 4429ns | -60.06% |
| bitpack-zeropad-seq | 11509ns | 11509ns | +3.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 10568ns | base | --- | [10271, 11175] | --- | --- | --- | --- |
| bitpack-native-seq | 3114ns | -7429.7ns (-70.3%) | [-7705, -7185]ns | [2978, 3379] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 10566ns | no significant difference | [-2001, +1771]ns | [9200, 11923] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|---|
| 1 | 1 | 10158ns | -69.2% | +16.2% |
| 1 | 2 | 10271ns | -73.5% | +47.6% |
| 1 | 3 | 11608ns | -69.0% | -21.4% |
| 1 | 4 | 11239ns | -73.6% | -18.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | 0.170 | ok |
| bitpack-native-seq | -0.645 | HIGH- (thermal bounce) |
| bitpack-zeropad-seq | -0.071 | ok |

**Consistency summary:**

- **bitpack-native-seq**: won 4/4, lost 0/4
- **bitpack-zeropad-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 57.7ns | 10914.3ns | 0.5% |  |
| bitpack-native-seq | 48.6ns | 4292.9ns | 1.1% |  |
| bitpack-zeropad-seq | 54.3ns | 11370.8ns | 0.5% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 10122.8-12263.2 ns)
  10122.8 |########################################
  10229.8 |########################################
  10336.8 |#############
  10443.8 |######
  10550.9 |
  10657.9 |######
  10764.9 |
  10871.9 |######
  10978.9 |######
  11086.0 |####################
  11193.0 |####################
  11300.0 |
  11407.0 |##########################
  11514.0 |######
  11621.1 |#############
  11728.1 |######
  11835.1 |######
  11942.1 |
  12049.1 |######
  12156.2 |
  (5 below, 1 above range)

bitpack-native-seq (n=40, range 2680.3-9294.5 ns)
   2680.3 |#################################
   3011.0 |########################################
   3341.7 |####################
   3672.4 |##########
   4003.1 |###
   4333.8 |###
   4664.5 |###
   4995.2 |###
   5326.0 |
   5656.7 |
   5987.4 |
   6318.1 |
   6648.8 |
   6979.5 |
   7310.2 |
   7640.9 |
   7971.6 |
   8302.3 |
   8633.0 |
   8963.8 |
  (4 below, 1 above range)

bitpack-zeropad-seq (n=40, range 9093.4-15328.8 ns)
   9093.4 |########################################
   9405.1 |##
   9716.9 |
  10028.7 |
  10340.5 |
  10652.2 |
  10964.0 |
  11275.8 |
  11587.5 |#################
  11899.3 |########
  12211.1 |##
  12522.8 |
  12834.6 |
  13146.4 |
  13458.1 |
  13769.9 |
  14081.7 |
  14393.4 |
  14705.2 |########
  15017.0 |##############
  (5 below, 2 above range)

```

## Diagnostics

- **bitpack-native-seq**: CV=150.2% (high variance, measurements may be unstable)
- **bitpack-native-seq**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bitpack-zeropad-seq**: CV=22.0% (high variance, measurements may be unstable)
