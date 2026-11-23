# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 417% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (131.59 us) leads bitpack-footprint-packed-naive (679.80 us) by 417%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (131.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.2x the fastest

Fastest bitpack-footprint-packed (131.59 us) to slowest bitpack-footprint-packed-naive (679.80 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 131590.5 ns median
- 1 variant significantly slower than baseline
- Spread: 5.17x (fastest 131590.5 ns, slowest 679801.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112746100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 130871ns | 131744ns | 127889ns | 131460ns | 132084ns | base |
| bitpack-footprint-packed-naive | 677838ns | 679972ns | 668044ns | 678080ns | 686906ns | +417.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 130713ns | 127726ns | 131937ns | base | 8.022 |
| bitpack-footprint-packed-naive | 677658ns | 667872ns | 686730ns | +418.43% | 1.547 |

## Performance model

- Peak throughput: **8.210 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.968 | 97.1% |
| bitpack-footprint-packed-naive | 1.542 | 18.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 130871ns | 130871ns | base |
| bitpack-footprint-packed-naive | 677838ns | 677838ns | +417.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 131590ns | base | --- | [131548, 131663] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 679801ns | +548046.5ns (+416.5%) | [+537123, +554647]ns | [668693, 686187] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 131557ns | +407.7% |
| 1 | 2 | 131643ns | +407.6% |
| 1 | 3 | 127740ns | +437.3% |
| 1 | 4 | 131780ns | +420.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.452 | moderate- |
| bitpack-footprint-packed-naive | 0.254 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 28.3ns | 130712.9ns | 0.0% |  |
| bitpack-footprint-packed-naive | 31.5ns | 677658.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 127726.3-131937.4 ns)
  127726.3 |#############
  127936.8 |
  128147.4 |
  128358.0 |
  128568.5 |
  128779.1 |
  128989.6 |
  129200.2 |
  129410.7 |
  129621.3 |
  129831.9 |
  130042.4 |
  130253.0 |
  130463.5 |
  130674.1 |
  130884.7 |
  131095.2 |
  131305.8 |##
  131516.3 |########################################
  131726.9 |#################
  (4 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 667871.7-686730.3 ns)
  667871.7 |#######################
  668814.6 |#######
  669757.6 |
  670700.5 |##
  671643.4 |
  672586.3 |##
  673529.3 |##
  674472.2 |
  675415.1 |
  676358.1 |
  677301.0 |
  678243.9 |
  679186.9 |
  680129.8 |
  681072.7 |
  682015.7 |
  682958.6 |
  683901.5 |
  684844.5 |
  685787.4 |########################################
  (4 below, 3 above range)

```
