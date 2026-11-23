# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 55% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (356 ns) leads bitpack-aligned-rand (552 ns) by 55%, a clear separation rather than a photo finish. CV 23.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 36% (significant)

bitpack-native-rand is -198 ns (36%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 4.0x slower than the field

bitpack-zeropad-rand (1.44 us) is 4.0x the fastest (356 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.0x the fastest

Fastest bitpack-native-rand (356 ns) to slowest bitpack-zeropad-rand (1.44 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-rand is inconsistent: worst-20% is 1.7x its best-20%

bitpack-native-rand's best 20% of batches run at 327 ns but its worst 20% at 548 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 356.4 ns median (-35.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.04x (fastest 356.4 ns, slowest 1438.2 ns)

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
| bitpack-aligned-rand | 890ns | 777ns | 706ns | 833ns | 1246ns | base |
| bitpack-native-rand | 668ns | 588ns | 538ns | 633ns | 903ns | -24.99% |
| bitpack-zeropad-rand | 1667ns | 1689ns | 1400ns | 1651ns | 1984ns | +87.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 614ns | 501ns | 835ns | base | 0.417 |
| bitpack-native-rand | 400ns | 327ns | 548ns | -34.79% | 0.639 |
| bitpack-zeropad-rand | 1412ns | 1196ns | 1657ns | +130.00% | 0.181 |

## Performance model

- Peak throughput: **0.784 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 0.464 | 59.2% |
| bitpack-native-rand | 0.718 | 91.6% |
| bitpack-zeropad-rand | 0.178 | 22.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 890ns | 890ns | base |
| bitpack-native-rand | 668ns | 668ns | -24.99% |
| bitpack-zeropad-rand | 1667ns | 1667ns | +87.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 552ns | base | --- | [551, 558] | --- | --- | --- | --- |
| bitpack-native-rand | 356ns | -198.4ns (-35.9%) | [-229, -186]ns | [355, 379] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 1438ns | +775.6ns (+140.5%) | [+704, +812]ns | [1314, 1461] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 551ns | -35.6% | +182.7% |
| 1 | 2 | 552ns | -41.2% | +116.5% |
| 1 | 3 | 502ns | -28.3% | +161.9% |
| 1 | 4 | 775ns | -34.2% | +94.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.241 | moderate- |
| bitpack-native-rand | 0.019 | ok |
| bitpack-zeropad-rand | -0.304 | moderate- |

**Consistency summary:**

- **bitpack-native-rand**: won 4/4, lost 0/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 94.3ns | 614.1ns | 15.4% | HIGH |
| bitpack-native-rand | 92.6ns | 400.4ns | 23.1% | HIGH |
| bitpack-zeropad-rand | 92.0ns | 1412.3ns | 6.5% | HIGH |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 501.2-835.5 ns)
    501.2 |#####################
    517.9 |###
    534.6 |##################
    551.4 |########################################
    568.1 |
    584.8 |
    601.5 |
    618.2 |
    634.9 |
    651.6 |
    668.3 |###
    685.1 |
    701.8 |
    718.5 |
    735.2 |
    751.9 |##################
    768.6 |###
    785.3 |
    802.0 |###
    818.8 |
  (5 below, 4 above range)

bitpack-native-rand (n=40, range 326.6-547.5 ns)
    326.6 |##
    337.6 |#####
    348.7 |########################################
    359.7 |##
    370.8 |
    381.8 |
    392.9 |#######
    403.9 |
    414.9 |
    426.0 |##
    437.0 |
    448.1 |
    459.1 |
    470.2 |
    481.2 |
    492.3 |
    503.3 |###############
    514.4 |
    525.4 |
    536.5 |
  (6 below, 4 above range)

bitpack-zeropad-rand (n=40, range 1196.2-1657.1 ns)
   1196.2 |####
   1219.3 |########
   1242.3 |
   1265.3 |
   1288.4 |####
   1311.4 |##########################
   1334.5 |
   1357.5 |
   1380.6 |####
   1403.6 |########
   1426.7 |
   1449.7 |########################################
   1472.8 |
   1495.8 |####
   1518.8 |
   1541.9 |####
   1564.9 |
   1588.0 |####
   1611.0 |
   1634.1 |###############################
  (7 below, 1 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: CV=21.5% (high variance, measurements may be unstable)
- **bitpack-aligned-rand**: bridge=14.9% of algo (FFI overhead may distort results)
- **bitpack-native-rand**: CV=21.3% (high variance, measurements may be unstable)
- **bitpack-native-rand**: bridge=23.3% of algo (FFI overhead may distort results)
