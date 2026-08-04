# Layout::Bitpacked reading, sequential column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-seq**

## Highlights

Baseline for all deltas below: **bitpack-aligned-seq**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-seq dominates: 243% faster than the next best (bitpack-aligned-seq)

bitpack-native-seq (407 ns) leads bitpack-aligned-seq (1.39 us) by 243%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-seq beats baseline by 71% (significant)

bitpack-native-seq is -987 ns (71%) faster than baseline bitpack-aligned-seq, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-seq is an outlier: 4.8x slower than the field

bitpack-zeropad-seq (1.93 us) is 4.8x the fastest (407 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest bitpack-native-seq (407 ns) to slowest bitpack-zeropad-seq (1.93 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-native-seq** at 406.7 ns median (-70.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.76x (fastest 406.7 ns, slowest 1934.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 1488ns | 1468ns | 1464ns | 1468ns | 1573ns | base |
| bitpack-native-seq | 486ns | 480ns | 477ns | 480ns | 514ns | -67.35% |
| bitpack-zeropad-seq | 2149ns | 2002ns | 1998ns | 2032ns | 2653ns | +44.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-seq | 1411ns | 1392ns | 1479ns | base | 2.903 |
| bitpack-native-seq | 413ns | 405ns | 440ns | -70.73% | 9.917 |
| bitpack-zeropad-seq | 2073ns | 1930ns | 2549ns | +46.94% | 1.976 |

## Performance model

- Peak throughput: **10.120 Gops/s** (bitpack-native-seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-seq | 2.937 | 29.0% |
| bitpack-native-seq | 10.071 | 99.5% |
| bitpack-zeropad-seq | 2.118 | 20.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-seq | 1488ns | 1488ns | base |
| bitpack-native-seq | 486ns | 486ns | -67.35% |
| bitpack-zeropad-seq | 2149ns | 2149ns | +44.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-seq | 1395ns | base | --- | [1394, 1396] | --- | --- | --- | --- |
| bitpack-native-seq | 407ns | -988.1ns (-70.9%) | [-989, -986]ns | [406, 408] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-seq | 1934ns | +538.8ns (+38.6%) | [+536, +546]ns | [1932, 1971] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-aligned-seq | bitpack-native-seq | bitpack-zeropad-seq |
|---|---|---|---|
| 1 | 1393ns | -70.9% | +85.3% |
| 2 | 1395ns | -70.9% | +45.8% |
| 3 | 1397ns | -68.9% | +44.0% |
| 4 | 1604ns | -74.6% | +28.4% |
| 5 | 1512ns | -73.1% | +28.0% |
| 6 | 1480ns | -72.7% | +30.6% |
| 7 | 1395ns | -70.7% | +38.4% |
| 8 | 1392ns | -70.7% | +38.7% |
| 9 | 1482ns | -72.6% | +30.3% |
| 10 | 1393ns | -70.9% | +38.5% |
| 11 | 1509ns | -73.0% | +28.2% |
| 12 | 1395ns | -70.9% | +84.3% |
| 13 | 1393ns | -70.9% | +38.7% |
| 14 | 1396ns | -70.9% | +38.6% |
| 15 | 1394ns | -70.8% | +144.4% |
| 16 | 1395ns | -70.9% | +141.8% |
| 17 | 1394ns | -70.9% | +38.5% |
| 18 | 1392ns | -71.1% | +38.6% |
| 19 | 1394ns | -66.1% | +38.5% |
| 20 | 1393ns | -64.2% | +38.9% |
| 21 | 1390ns | -70.6% | +51.0% |
| 22 | 1395ns | -70.8% | +50.4% |
| 23 | 1394ns | -70.7% | +50.3% |
| 24 | 1392ns | -70.6% | +51.3% |
| 25 | 1393ns | -70.9% | +50.8% |
| 26 | 1392ns | -70.8% | +55.1% |
| 27 | 1393ns | -70.9% | +38.6% |
| 28 | 1398ns | -70.9% | +38.0% |
| 29 | 1394ns | -70.6% | +38.6% |
| 30 | 1396ns | -70.8% | +38.6% |
| 31 | 1395ns | -70.5% | +38.4% |
| 32 | 1400ns | -65.9% | +38.0% |
| 33 | 1396ns | -70.9% | +43.3% |
| 34 | 1392ns | -70.8% | +39.5% |
| 35 | 1399ns | -71.0% | +38.2% |
| 36 | 1393ns | -70.8% | +38.6% |
| 37 | 1448ns | -71.9% | +33.6% |
| 38 | 1392ns | -70.8% | +38.9% |
| 39 | 1398ns | -71.0% | +38.2% |
| 40 | 1395ns | -70.9% | +38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-seq | 0.298 | moderate+ |
| bitpack-native-seq | 0.270 | moderate+ |
| bitpack-zeropad-seq | 0.363 | moderate+ |

**Consistency summary:**

- **bitpack-native-seq**: won 40/40, lost 0/40
- **bitpack-zeropad-seq**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-seq | 2.4ns | 1411.0ns | 0.2% |  |
| bitpack-native-seq | 1.9ns | 413.0ns | 0.5% |  |
| bitpack-zeropad-seq | 2.0ns | 2073.4ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-aligned-seq (n=40, range 1391.8-1479.0 ns)
   1391.8 |########################################
   1396.1 |##########
   1400.5 |
   1404.9 |
   1409.2 |
   1413.6 |
   1418.0 |
   1422.3 |
   1426.7 |
   1431.0 |
   1435.4 |
   1439.8 |
   1444.1 |#
   1448.5 |
   1452.9 |
   1457.2 |
   1461.6 |
   1465.9 |
   1470.3 |
   1474.7 |
  (5 below, 5 above range)

bitpack-native-seq (n=40, range 404.7-440.3 ns)
    404.7 |########################################
    406.5 |################################
    408.3 |##########
    410.1 |##
    411.8 |
    413.6 |
    415.4 |
    417.2 |
    418.9 |
    420.7 |
    422.5 |
    424.3 |
    426.0 |
    427.8 |
    429.6 |
    431.4 |
    433.2 |##
    434.9 |
    436.7 |
    438.5 |
  (2 below, 3 above range)

bitpack-zeropad-seq (n=40, range 1930.1-2549.4 ns)
   1930.1 |########################################
   1961.1 |
   1992.0 |###
   2023.0 |#
   2054.0 |#
   2084.9 |#########
   2115.9 |
   2146.9 |#
   2177.8 |
   2208.8 |
   2239.8 |
   2270.7 |
   2301.7 |
   2332.7 |
   2363.6 |
   2394.6 |
   2425.6 |
   2456.5 |
   2487.5 |
   2518.5 |
  (4 below, 4 above range)

```
