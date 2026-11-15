# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 37% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (450 ns) leads bitpack-aligned-rand (614 ns) by 37%, a clear separation rather than a photo finish. CV 23.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 33% (significant)

bitpack-native-rand is -202 ns (33%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 2.9x slower than the field

bitpack-zeropad-rand (1.31 us) is 2.9x the fastest (450 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-rand shows alternating (throttle bounce) (autocorr -0.76)

bitpack-native-rand's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-aligned-rand is inconsistent: worst-20% is 2.1x its best-20%

bitpack-aligned-rand's best 20% of batches run at 438 ns but its worst 20% at 923 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 449.8 ns median (-26.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.92x (fastest 449.8 ns, slowest 1311.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 933ns | 866ns | 618ns | 910ns | 1316ns | base |
| bitpack-native-rand | 725ns | 768ns | 509ns | 720ns | 953ns | -22.36% |
| bitpack-zeropad-rand | 1596ns | 1538ns | 1241ns | 1538ns | 2123ns | +70.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 646ns | 438ns | 923ns | base | 0.396 |
| bitpack-native-rand | 422ns | 299ns | 558ns | -34.75% | 0.607 |
| bitpack-zeropad-rand | 1355ns | 1055ns | 1809ns | +109.64% | 0.189 |

## Performance model

- Peak throughput: **0.857 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 0.417 | 48.7% |
| bitpack-native-rand | 0.569 | 66.5% |
| bitpack-zeropad-rand | 0.195 | 22.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 933ns | 933ns | base |
| bitpack-native-rand | 725ns | 725ns | -22.36% |
| bitpack-zeropad-rand | 1596ns | 1596ns | +70.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 614ns | base | --- | [613, 616] | --- | --- | --- | --- |
| bitpack-native-rand | 450ns | -201.8ns (-32.9%) | [-259, -155]ns | [325, 462] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 1312ns | +766.8ns (+124.9%) | [+479, +851]ns | [1192, 1360] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 614ns | -25.1% | +78.1% |
| 1 | 2 | 614ns | -47.6% | +141.5% |
| 1 | 3 | 921ns | -41.9% | +42.0% |
| 1 | 4 | 443ns | -33.5% | +196.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.540 | HIGH- (thermal bounce) |
| bitpack-native-rand | -0.761 | HIGH- (thermal bounce) |
| bitpack-zeropad-rand | -0.472 | moderate- |

**Consistency summary:**

- **bitpack-native-rand**: won 4/4, lost 0/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 103.5ns | 646.2ns | 16.0% | HIGH |
| bitpack-native-rand | 108.2ns | 421.7ns | 25.7% | HIGH |
| bitpack-zeropad-rand | 83.1ns | 1354.7ns | 6.1% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 438.4-922.6 ns)
    438.4 |########
    462.6 |#####
    486.8 |
    511.0 |
    535.2 |########
    559.5 |
    583.7 |
    607.9 |########################################
    632.1 |##
    656.3 |
    680.5 |##
    704.7 |
    728.9 |
    753.1 |
    777.3 |
    801.6 |
    825.8 |
    850.0 |
    874.2 |
    898.4 |################
  (5 below, 4 above range)

bitpack-native-rand (n=40, range 298.9-557.7 ns)
    298.9 |###
    311.8 |#########################
    324.8 |###
    337.7 |
    350.6 |##############
    363.6 |
    376.5 |
    389.5 |
    402.4 |
    415.4 |
    428.3 |
    441.2 |###
    454.2 |#########################
    467.1 |
    480.1 |
    493.0 |
    506.0 |
    518.9 |
    531.9 |########################################
    544.8 |
  (6 below, 2 above range)

bitpack-zeropad-rand (n=40, range 1055.2-1808.8 ns)
   1055.2 |
   1092.8 |####################
   1130.5 |
   1168.2 |########################
   1205.9 |
   1243.6 |####
   1281.3 |########################################
   1319.0 |
   1356.6 |
   1394.3 |####
   1432.0 |####################
   1469.7 |####
   1507.4 |
   1545.1 |####
   1582.7 |####
   1620.4 |########
   1658.1 |
   1695.8 |
   1733.5 |
   1771.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: CV=26.9% (high variance, measurements may be unstable)
- **bitpack-aligned-rand**: bridge=15.2% of algo (FFI overhead may distort results)
- **bitpack-native-rand**: CV=24.9% (high variance, measurements may be unstable)
- **bitpack-native-rand**: bridge=23.2% of algo (FFI overhead may distort results)
- **bitpack-zeropad-rand**: CV=22.8% (high variance, measurements may be unstable)
