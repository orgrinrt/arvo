# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 32% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-packed (516.44 us) leads bitpack-footprint-dense (680.79 us) by 32%, a clear separation rather than a photo finish. CV 6.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed beats baseline by 24% (significant)

bitpack-footprint-packed is -165.63 us (24%) faster than baseline bitpack-footprint-dense, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-footprint-packed-naive is an outlier: 5.2x slower than the field

bitpack-footprint-packed-naive (2.70 ms) is 5.2x the fastest (516.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} vs {bitpack-footprint-packed-naive} (285% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} and a slow tier {bitpack-footprint-packed-naive} with a 285% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest bitpack-footprint-packed (516.44 us) to slowest bitpack-footprint-packed-naive (2.70 ms): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 516438.2 ns median (-24.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.23x (fastest 516438.2 ns, slowest 2701154.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 705396ns | 681142ns | 654428ns | 697075ns | 781327ns | base |
| bitpack-footprint-dense-alt | 729077ns | 701059ns | 638647ns | 699384ns | 908586ns | +3.36% |
| bitpack-footprint-packed | 538001ns | 516580ns | 515319ns | 525312ns | 598749ns | -23.73% |
| bitpack-footprint-packed-naive | 2720561ns | 2701354ns | 2695884ns | 2709536ns | 2778310ns | +285.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 705097ns | 654232ns | 780947ns | base | 5.949 |
| bitpack-footprint-dense-alt | 728777ns | 638250ns | 908317ns | +3.36% | 5.755 |
| bitpack-footprint-packed | 537786ns | 515148ns | 598423ns | -23.73% | 7.799 |
| bitpack-footprint-packed-naive | 2720318ns | 2695680ns | 2777992ns | +285.81% | 1.542 |

## Performance model

- Peak throughput: **8.142 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.161 | 75.7% |
| bitpack-footprint-dense-alt | 5.985 | 73.5% |
| bitpack-footprint-packed | 8.122 | 99.8% |
| bitpack-footprint-packed-naive | 1.553 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 705396ns | 705396ns | base |
| bitpack-footprint-dense-alt | 729077ns | 729077ns | +3.36% |
| bitpack-footprint-packed | 538001ns | 538001ns | -23.73% |
| bitpack-footprint-packed-naive | 2720561ns | 2720561ns | +285.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 680792ns | base | --- | [679524, 708736] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 700765ns | no significant difference | [-18022, +53181]ns | [690591, 712194] | no | 0.6358 | 0.6358 | 0 |
| bitpack-footprint-packed | 516438ns | -165634.5ns (-24.3%) | [-179859, -163781]ns | [515752, 525890] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 2701155ns | +2041897.9ns (+299.9%) | [+1988349, +2048256]ns | [2696946, 2727569] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 679658ns | -6.1% | -24.2% | +301.6% |
| 1 | 2 | 654230ns | +8.5% | -21.1% | +312.6% |
| 1 | 3 | 708640ns | -2.5% | -27.2% | +280.6% |
| 1 | 4 | 777148ns | +18.7% | -23.8% | +246.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.161 | ok |
| bitpack-footprint-dense-alt | -0.094 | ok |
| bitpack-footprint-packed | -0.082 | ok |
| bitpack-footprint-packed-naive | -0.014 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 2/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 67.5ns | 705096.8ns | 0.0% |  |
| bitpack-footprint-dense-alt | 74.5ns | 728777.4ns | 0.0% |  |
| bitpack-footprint-packed | 34.8ns | 537786.1ns | 0.0% |  |
| bitpack-footprint-packed-naive | 62.7ns | 2720318.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 654232.2-780947.5 ns)
  654232.2 |######################
  660567.9 |
  666903.7 |
  673239.5 |######################
  679575.2 |##########################
  685911.0 |
  692246.7 |
  698582.5 |
  704918.3 |###################################
  711254.0 |####
  717589.8 |
  723925.6 |
  730261.3 |
  736597.1 |
  742932.9 |
  749268.6 |
  755604.4 |
  761940.2 |
  768275.9 |
  774611.7 |########################################
  (5 below, 1 above range)

bitpack-footprint-dense-alt (n=40, range 638249.7-908317.0 ns)
  638249.7 |################
  651753.1 |########
  665256.5 |####
  678759.8 |########################
  692263.2 |####
  705766.6 |########################################
  719269.9 |
  732773.3 |
  746276.7 |
  759780.0 |########
  773283.4 |
  786786.8 |
  800290.1 |####
  813793.5 |
  827296.8 |
  840800.2 |
  854303.6 |
  867806.9 |
  881310.3 |
  894813.7 |
  (6 below, 7 above range)

bitpack-footprint-packed (n=40, range 515148.3-598423.3 ns)
  515148.3 |########################################
  519312.0 |##
  523475.8 |
  527639.5 |########
  531803.3 |
  535967.0 |
  540130.8 |
  544294.5 |
  548458.3 |
  552622.0 |
  556785.8 |
  560949.6 |
  565113.3 |
  569277.1 |
  573440.8 |
  577604.6 |
  581768.3 |
  585932.1 |
  590095.8 |##############
  594259.6 |##
  (5 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 2695679.7-2777992.0 ns)
  2695679.7 |########################################
  2699795.3 |#######
  2703910.9 |
  2708026.6 |##
  2712142.2 |
  2716257.8 |##
  2720373.4 |##
  2724489.0 |##########
  2728604.6 |#######
  2732720.2 |#####
  2736835.8 |
  2740951.5 |##
  2745067.1 |
  2749182.7 |
  2753298.3 |
  2757413.9 |
  2761529.5 |
  2765645.1 |
  2769760.8 |
  2773876.4 |
  (3 below, 5 above range)

```
