# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 234% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (1.67 us) leads bitpack-aligned-seq (5.57 us) by 234%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 70% (significant)

bitpack-native-seq is -3.90 us (70%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 4.6x slower than the field

bitpack-zeropad-seq (7.68 us) is 4.6x the fastest (1.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq shows warm-up / thermal drift (autocorr +0.83)

bitpack-native-seq's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-native-seq (1.67 us) to slowest bitpack-zeropad-seq (7.68 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-native-seq** at 1667.3 ns median (-70.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.61x (fastest 1667.3 ns, slowest 7678.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 5844ns | 5645ns | 5637ns | 5744ns | 6350ns | base |
| bitpack-native-seq | 1761ns | 1740ns | 1737ns | 1741ns | 1844ns | -69.87% |
| bitpack-zeropad-seq | 7902ns | 7749ns | 7742ns | 7787ns | 8408ns | +35.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 5761ns | 5563ns | 6243ns | base | 2.844 |
| bitpack-native-seq | 1686ns | 1665ns | 1766ns | -70.73% | 9.716 |
| bitpack-zeropad-seq | 7832ns | 7674ns | 8333ns | +35.95% | 2.092 |

## Performance model

- Peak throughput: **9.842 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.942 | 29.9% |
| bitpack-native-seq | 9.827 | 99.8% |
| bitpack-zeropad-seq | 2.134 | 21.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 5844ns | 5844ns | base |
| bitpack-native-seq | 1761ns | 1761ns | -69.87% |
| bitpack-zeropad-seq | 7902ns | 7902ns | +35.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 5570ns | base | --- | [5567, 5690] | --- | --- | --- | --- |
| bitpack-native-seq | 1667ns | -3903.1ns (-70.1%) | [-4016, -3900]ns | [1666, 1668] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 7679ns | +2111.4ns (+37.9%) | [+2107, +2116]ns | [7677, 7700] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|
| 1 | 6041ns | -72.4% | +37.8% |
| 2 | 6041ns | -72.4% | +37.9% |
| 3 | 6042ns | -72.4% | +37.9% |
| 4 | 6042ns | -72.4% | +27.1% |
| 5 | 5804ns | -71.3% | +32.3% |
| 6 | 5565ns | -70.0% | +38.0% |
| 7 | 5565ns | -70.1% | +37.9% |
| 8 | 5558ns | -70.0% | +39.1% |
| 9 | 5565ns | -70.1% | +38.0% |
| 10 | 5565ns | -70.1% | +47.3% |
| 11 | 5568ns | -67.6% | +37.8% |
| 12 | 5562ns | -67.4% | +38.0% |
| 13 | 5571ns | -67.5% | +37.8% |
| 14 | 5569ns | -67.5% | +37.9% |
| 15 | 5931ns | -69.5% | +29.4% |
| 16 | 5568ns | -68.7% | +37.9% |
| 17 | 5569ns | -70.1% | +38.0% |
| 18 | 5570ns | -70.1% | +38.2% |
| 19 | 5566ns | -70.1% | +37.9% |
| 20 | 5570ns | -70.1% | +37.8% |
| 21 | 5576ns | -70.1% | +38.5% |
| 22 | 5574ns | -70.1% | +37.7% |
| 23 | 5564ns | -70.0% | +38.0% |
| 24 | 5566ns | -70.1% | +38.0% |
| 25 | 5569ns | -70.0% | +37.8% |
| 26 | 5570ns | -70.1% | +37.8% |
| 27 | 5565ns | -70.1% | +37.9% |
| 28 | 5566ns | -70.1% | +39.0% |
| 29 | 5564ns | -70.1% | +38.0% |
| 30 | 5565ns | -70.1% | +37.9% |
| 31 | 6043ns | -72.4% | +37.9% |
| 32 | 6041ns | -72.4% | +37.9% |
| 33 | 6114ns | -72.7% | +36.5% |
| 34 | 6670ns | -75.0% | +24.9% |
| 35 | 6041ns | -72.3% | +37.9% |
| 36 | 6953ns | -76.0% | +13.4% |
| 37 | 6040ns | -72.4% | +27.5% |
| 38 | 5893ns | -71.7% | +30.6% |
| 39 | 5563ns | -70.0% | +38.4% |
| 40 | 5568ns | -70.1% | +37.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | 0.599 | HIGH+ (drift/warm-up) |
| bitpack-native-seq | 0.829 | HIGH+ (drift/warm-up) |
| bitpack-zeropad-seq | 0.656 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-native-seq**: won 40/40, lost 0/40
- **bitpack-zeropad-seq**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 2.7ns | 5760.9ns | 0.0% |  |
| bitpack-native-seq | 2.2ns | 1686.4ns | 0.1% |  |
| bitpack-zeropad-seq | 2.0ns | 7832.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 5563.1-6243.4 ns)
   5563.1 |########################################
   5597.1 |
   5631.2 |
   5665.2 |
   5699.2 |
   5733.2 |
   5767.2 |
   5801.2 |#
   5835.2 |
   5869.2 |#
   5903.3 |#
   5937.3 |
   5971.3 |
   6005.3 |
   6039.3 |#############
   6073.3 |
   6107.3 |#
   6141.3 |
   6175.4 |
   6209.4 |
  (3 below, 2 above range)

bitpack-native-seq (n=40, range 1664.7-1765.6 ns)
   1664.7 |########################################
   1669.7 |#
   1674.7 |
   1679.8 |
   1684.8 |
   1689.9 |
   1694.9 |
   1700.0 |
   1705.0 |
   1710.1 |
   1715.1 |
   1720.2 |
   1725.2 |
   1730.3 |
   1735.3 |
   1740.4 |#
   1745.4 |
   1750.5 |
   1755.5 |
   1760.6 |
  (4 below, 5 above range)

bitpack-zeropad-seq (n=40, range 7674.1-8332.8 ns)
   7674.1 |########################################
   7707.0 |#####
   7740.0 |
   7772.9 |
   7805.8 |
   7838.8 |
   7871.7 |#
   7904.6 |
   7937.6 |
   7970.5 |
   8003.4 |
   8036.4 |
   8069.3 |
   8102.2 |
   8135.2 |
   8168.1 |#
   8201.0 |
   8234.0 |
   8266.9 |
   8299.8 |########
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **bitpack-native-seq**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **bitpack-zeropad-seq**: autocorrelation=0.66 (measurement drift or warm-up artifact)
