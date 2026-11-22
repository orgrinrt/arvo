# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 43% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (352 ns) leads bitpack-aligned-rand (502 ns) by 43%, a clear separation rather than a photo finish. CV 18.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 29% (significant)

bitpack-native-rand is -146 ns (29%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 3.4x slower than the field

bitpack-zeropad-rand (1.19 us) is 3.4x the fastest (352 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-rand is fastest but the noisiest (CV 18.8%)

bitpack-native-rand wins on median (352 ns) yet has the highest variance (CV 18.8%), while bitpack-aligned-rand is the steadiest (CV 6.5%, 502 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-aligned-rand shows alternating (throttle bounce) (autocorr -0.51)

bitpack-aligned-rand's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest bitpack-native-rand (352 ns) to slowest bitpack-zeropad-rand (1.19 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-rand is inconsistent: worst-20% is 1.5x its best-20%

bitpack-native-rand's best 20% of batches run at 293 ns but its worst 20% at 443 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 351.8 ns median (-30.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.39x (fastest 351.8 ns, slowest 1193.6 ns)

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
| bitpack-aligned-rand | 718ns | 709ns | 649ns | 703ns | 832ns | base |
| bitpack-native-rand | 652ns | 608ns | 499ns | 596ns | 971ns | -9.25% |
| bitpack-zeropad-rand | 1439ns | 1399ns | 1297ns | 1414ns | 1657ns | +100.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 498ns | 460ns | 552ns | base | 0.514 |
| bitpack-native-rand | 353ns | 293ns | 443ns | -29.26% | 0.726 |
| bitpack-zeropad-rand | 1222ns | 1109ns | 1384ns | +145.21% | 0.209 |

## Performance model

- Peak throughput: **0.873 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 0.510 | 58.4% |
| bitpack-native-rand | 0.728 | 83.4% |
| bitpack-zeropad-rand | 0.214 | 24.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 718ns | 718ns | base |
| bitpack-native-rand | 652ns | 652ns | -9.25% |
| bitpack-zeropad-rand | 1439ns | 1439ns | +100.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 502ns | base | --- | [490, 503] | --- | --- | --- | --- |
| bitpack-native-rand | 352ns | -145.7ns (-29.0%) | [-169, -138]ns | [323, 358] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 1194ns | +713.6ns (+142.1%) | [+683, +735]ns | [1193, 1197] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 502ns | -29.9% | +143.4% |
| 1 | 2 | 502ns | -31.1% | +137.5% |
| 1 | 3 | 460ns | -13.8% | +159.3% |
| 1 | 4 | 543ns | -46.1% | +119.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.505 | HIGH- (thermal bounce) |
| bitpack-native-rand | -0.505 | HIGH- (thermal bounce) |
| bitpack-zeropad-rand | -0.095 | ok |

**Consistency summary:**

- **bitpack-native-rand**: won 4/4, lost 0/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 76.9ns | 498.5ns | 15.4% | HIGH |
| bitpack-native-rand | 89.5ns | 352.6ns | 25.4% | HIGH |
| bitpack-zeropad-rand | 77.8ns | 1222.4ns | 6.4% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 460.2-552.0 ns)
    460.2 |##################
    464.8 |
    469.4 |
    474.0 |
    478.6 |##
    483.1 |
    487.7 |
    492.3 |
    496.9 |
    501.5 |########################################
    506.1 |##
    510.7 |
    515.3 |
    519.9 |
    524.5 |
    529.1 |##
    533.6 |
    538.2 |
    542.8 |##
    547.4 |
  (5 below, 6 above range)

bitpack-native-rand (n=40, range 293.2-443.3 ns)
    293.2 |########
    300.7 |####
    308.2 |
    315.7 |########################
    323.2 |########
    330.7 |########
    338.2 |
    345.7 |####
    353.2 |########################################
    360.7 |####
    368.2 |####
    375.7 |
    383.2 |
    390.7 |####
    398.3 |####################
    405.8 |
    413.3 |
    420.8 |
    428.3 |
    435.8 |
  (6 below, 2 above range)

bitpack-zeropad-rand (n=40, range 1108.9-1384.2 ns)
   1108.9 |
   1122.6 |##
   1136.4 |
   1150.2 |
   1163.9 |##
   1177.7 |####
   1191.5 |########################################
   1205.2 |####
   1219.0 |##
   1232.8 |
   1246.5 |
   1260.3 |
   1274.1 |
   1287.8 |
   1301.6 |#######
   1315.4 |#######
   1329.1 |
   1342.9 |
   1356.7 |##
   1370.4 |
  (6 below, 3 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: bridge=15.1% of algo (FFI overhead may distort results)
- **bitpack-native-rand**: bridge=25.8% of algo (FFI overhead may distort results)
