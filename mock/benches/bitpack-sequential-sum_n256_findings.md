# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 309% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (21 ns) leads bitpack-aligned-seq (86 ns) by 309%, a clear separation rather than a photo finish. CV 9.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 79% (significant)

bitpack-native-seq is -68 ns (79%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 5.5x slower than the field

bitpack-zeropad-seq (116 ns) is 5.5x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-seq is fastest but the noisiest (CV 9.1%)

bitpack-native-seq wins on median (21 ns) yet has the highest variance (CV 9.1%), while bitpack-zeropad-seq is the steadiest (CV 5.1%, 116 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-aligned-seq shows warm-up / thermal drift (autocorr +0.87)

bitpack-aligned-seq's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest bitpack-native-seq (21 ns) to slowest bitpack-zeropad-seq (116 ns): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-native-seq** at 21.0 ns median (-75.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.52x (fastest 21.0 ns, slowest 116.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 162ns | 161ns | 147ns | 162ns | 177ns | base |
| bitpack-native-seq | 88ns | 88ns | 83ns | 88ns | 94ns | -45.43% |
| bitpack-zeropad-seq | 179ns | 178ns | 170ns | 178ns | 191ns | +10.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 89ns | 84ns | 99ns | base | 2.881 |
| bitpack-native-seq | 21ns | 18ns | 23ns | -76.55% | 12.286 |
| bitpack-zeropad-seq | 117ns | 110ns | 126ns | +31.52% | 2.191 |

## Performance model

- Peak throughput: **14.242 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.982 | 20.9% |
| bitpack-native-seq | 12.190 | 85.6% |
| bitpack-zeropad-seq | 2.207 | 15.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 162ns | 162ns | base |
| bitpack-native-seq | 88ns | 88ns | -45.43% |
| bitpack-zeropad-seq | 179ns | 179ns | +10.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 86ns | base | --- | [84, 90] | --- | --- | --- | --- |
| bitpack-native-seq | 21ns | -67.9ns (-79.1%) | [-70, -64]ns | [20, 22] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 116ns | +27.9ns (+32.5%) | [+25, +31]ns | [114, 119] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|
| 1 | 85ns | -72.0% | +44.8% |
| 2 | 84ns | -78.1% | +41.3% |
| 3 | 83ns | -77.0% | +45.0% |
| 4 | 84ns | -75.6% | +47.9% |
| 5 | 84ns | -72.6% | +35.8% |
| 6 | 84ns | -74.2% | +33.1% |
| 7 | 84ns | -76.1% | +32.4% |
| 8 | 84ns | -71.9% | +31.6% |
| 9 | 85ns | -80.9% | +29.1% |
| 10 | 84ns | -72.6% | +33.9% |
| 11 | 83ns | -74.5% | +40.6% |
| 12 | 84ns | -75.6% | +38.8% |
| 13 | 83ns | -73.9% | +37.1% |
| 14 | 84ns | -74.2% | +41.6% |
| 15 | 84ns | -75.3% | +39.1% |
| 16 | 84ns | -74.2% | +43.9% |
| 17 | 85ns | -79.4% | +39.8% |
| 18 | 85ns | -73.5% | +40.2% |
| 19 | 84ns | -76.7% | +37.1% |
| 20 | 83ns | -78.5% | +36.5% |
| 21 | 99ns | -76.8% | +32.4% |
| 22 | 95ns | -80.2% | +36.0% |
| 23 | 98ns | -76.6% | +30.6% |
| 24 | 99ns | -78.5% | +28.2% |
| 25 | 101ns | -75.2% | +24.4% |
| 26 | 99ns | -76.8% | +24.0% |
| 27 | 101ns | -77.4% | +18.2% |
| 28 | 98ns | -78.7% | +19.7% |
| 29 | 97ns | -79.0% | +19.3% |
| 30 | 95ns | -77.2% | +26.7% |
| 31 | 91ns | -76.7% | +20.7% |
| 32 | 88ns | -78.7% | +25.0% |
| 33 | 88ns | -77.6% | +30.5% |
| 34 | 87ns | -80.3% | +29.8% |
| 35 | 89ns | -77.1% | +22.9% |
| 36 | 90ns | -77.4% | +26.3% |
| 37 | 90ns | -76.3% | +25.6% |
| 38 | 92ns | -79.1% | +19.1% |
| 39 | 90ns | -77.2% | +21.9% |
| 40 | 90ns | -75.8% | +24.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | 0.870 | HIGH+ (drift/warm-up) |
| bitpack-native-seq | -0.170 | ok |
| bitpack-zeropad-seq | 0.694 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-native-seq**: won 40/40, lost 0/40
- **bitpack-zeropad-seq**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 1.4ns | 88.9ns | 1.6% |  |
| bitpack-native-seq | 1.3ns | 20.8ns | 6.3% | HIGH |
| bitpack-zeropad-seq | 1.2ns | 116.9ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 83.5-98.9 ns)
     83.5 |########################################
     84.3 |#############
     85.0 |
     85.8 |
     86.6 |###
     87.3 |###
     88.1 |###
     88.9 |#############
     89.6 |###
     90.4 |###
     91.2 |###
     91.9 |
     92.7 |
     93.5 |
     94.3 |######
     95.0 |
     95.8 |
     96.6 |###
     97.3 |######
     98.1 |##########
  (4 below, 2 above range)

bitpack-native-seq (n=40, range 18.0-23.4 ns)
     18.0 |
     18.2 |######
     18.5 |
     18.8 |#############
     19.1 |#############
     19.3 |
     19.6 |#############
     19.9 |######
     20.1 |
     20.4 |########################################
     20.7 |#############
     20.9 |##########################
     21.2 |
     21.5 |########################################
     21.7 |
     22.0 |
     22.3 |######
     22.6 |
     22.8 |########################################
     23.1 |
  (4 below, 3 above range)

bitpack-zeropad-seq (n=40, range 109.8-126.1 ns)
    109.8 |#############
    110.7 |##########################
    111.5 |########################################
    112.3 |##########################
    113.1 |##########################
    113.9 |########################################
    114.7 |#############
    115.5 |##########################
    116.3 |########################################
    117.2 |
    118.0 |#############
    118.8 |########################################
    119.6 |##########################
    120.4 |##########################
    121.2 |
    122.0 |##########################
    122.8 |
    123.7 |#############
    124.5 |
    125.3 |#############
  (5 below, 4 above range)

```

## Diagnostics

- **bitpack-aligned-seq**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **bitpack-zeropad-seq**: autocorrelation=0.69 (measurement drift or warm-up artifact)
