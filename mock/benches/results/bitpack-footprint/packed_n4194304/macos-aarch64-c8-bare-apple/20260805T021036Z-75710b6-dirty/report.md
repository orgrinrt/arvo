# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 334% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (744.88 us) leads bitpack-footprint-packed-naive (3.23 ms) by 334%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (744.88 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.3x the fastest

Fastest bitpack-footprint-packed (744.88 us) to slowest bitpack-footprint-packed-naive (3.23 ms): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 744880.2 ns median
- 1 variant significantly slower than baseline
- Spread: 4.34x (fastest 744880.2 ns, slowest 3231173.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 745085ns | 745556ns | 723839ns | 742860ns | 773006ns | base |
| bitpack-footprint-packed-naive | 3238910ns | 3232822ns | 3169233ns | 3236315ns | 3316369ns | +334.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 744293ns | 723146ns | 772131ns | base | 5.635 |
| bitpack-footprint-packed-naive | 3237153ns | 3167669ns | 3314401ns | +334.93% | 1.296 |

## Performance model

- Peak throughput: **5.800 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 5.631 | 97.1% |
| bitpack-footprint-packed-naive | 1.298 | 22.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 745085ns | 745085ns | base |
| bitpack-footprint-packed-naive | 3238910ns | 3238910ns | +334.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 744880ns | base | --- | [736002, 748151] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 3231174ns | +2496399.2ns (+335.1%) | [+2462609, +2518248]ns | [3213211, 3254481] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|
| 1 | 747008ns | +363.5% |
| 2 | 760563ns | +321.6% |
| 3 | 747645ns | +326.9% |
| 4 | 747030ns | +335.9% |
| 5 | 755913ns | +326.8% |
| 6 | 733540ns | +333.7% |
| 7 | 744113ns | +333.5% |
| 8 | 748855ns | +328.6% |
| 9 | 742040ns | +344.0% |
| 10 | 738463ns | +343.9% |
| 11 | 745648ns | +337.2% |
| 12 | 716420ns | +353.5% |
| 13 | 754681ns | +334.5% |
| 14 | 721857ns | +347.9% |
| 15 | 785092ns | +307.3% |
| 16 | 741122ns | +322.6% |
| 17 | 739414ns | +341.3% |
| 18 | 745841ns | +330.4% |
| 19 | 720509ns | +336.9% |
| 20 | 748656ns | +334.5% |
| 21 | 725159ns | +338.1% |
| 22 | 750589ns | +319.0% |
| 23 | 766171ns | +321.2% |
| 24 | 761520ns | +321.2% |
| 25 | 729510ns | +347.5% |
| 26 | 728208ns | +340.2% |
| 27 | 725600ns | +341.7% |
| 28 | 739545ns | +328.6% |
| 29 | 726460ns | +347.2% |
| 30 | 758282ns | +333.3% |
| 31 | 724417ns | +350.3% |
| 32 | 726323ns | +346.2% |
| 33 | 761691ns | +322.4% |
| 34 | 725563ns | +348.2% |
| 35 | 746810ns | +342.6% |
| 36 | 726283ns | +349.8% |
| 37 | 725645ns | +345.0% |
| 38 | 755812ns | +342.2% |
| 39 | 810873ns | +296.7% |
| 40 | 772857ns | +322.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.065 | ok |
| bitpack-footprint-packed-naive | 0.050 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 49.4ns | 744293.3ns | 0.0% |  |
| bitpack-footprint-packed-naive | 161.9ns | 3237152.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 723146.4-772131.1 ns)
  723146.4 |########################
  725595.6 |########################################
  728044.8 |################
  730494.1 |
  732943.3 |########
  735392.6 |
  737841.8 |########################
  740291.0 |################
  742740.3 |########
  745189.5 |########################################
  747638.8 |########################
  750088.0 |########
  752537.2 |########
  754986.5 |################
  757435.7 |########
  759884.9 |########################
  762334.2 |
  764783.4 |########
  767232.7 |
  769681.9 |
  (3 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 3167668.6-3314400.9 ns)
  3167668.6 |######
  3175005.2 |#############
  3182341.8 |
  3189678.5 |######
  3197015.1 |######
  3204351.7 |########################################
  3211688.3 |#############
  3219024.9 |#############
  3226361.5 |####################
  3233698.2 |######
  3241034.8 |
  3248371.4 |##########################
  3255708.0 |##########################
  3263044.6 |####################
  3270381.2 |
  3277717.9 |#############
  3285054.5 |######
  3292391.1 |######
  3299727.7 |######
  3307064.3 |
  (3 below, 2 above range)

```
