# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 380% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (952.88 us) leads bitpack-footprint-packed-naive (4.58 ms) by 380%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (952.88 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest bitpack-footprint-packed (952.88 us) to slowest bitpack-footprint-packed-naive (4.58 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 952878.7 ns median
- 1 variant significantly slower than baseline
- Spread: 4.80x (fastest 952878.7 ns, slowest 4577366.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 955187ns | 953572ns | 941561ns | 954537ns | 970763ns | base |
| bitpack-footprint-packed-naive | 4583173ns | 4578062ns | 4559970ns | 4577313ns | 4623958ns | +379.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 954582ns | 941122ns | 970287ns | base | 7.333 |
| bitpack-footprint-packed-naive | 4582530ns | 4559277ns | 4623300ns | +380.06% | 1.528 |

## Performance model

- Peak throughput: **7.438 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.346 | 98.8% |
| bitpack-footprint-packed-naive | 1.529 | 20.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 955187ns | 955187ns | base |
| bitpack-footprint-packed-naive | 4583173ns | 4583173ns | +379.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 952879ns | base | --- | [952153, 953954] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 4577367ns | +3621474.9ns (+380.1%) | [+3619622, +3625093]ns | [4574378, 4580764] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 953198ns | +380.9% |
| 1 | 2 | 952577ns | +380.7% |
| 1 | 3 | 941055ns | +384.6% |
| 1 | 4 | 964911ns | +374.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.485 | moderate- |
| bitpack-footprint-packed-naive | -0.312 | moderate- |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 81.0ns | 954581.5ns | 0.0% |  |
| bitpack-footprint-packed-naive | 105.8ns | 4582530.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 941122.5-970287.5 ns)
  941122.5 |########
  942580.7 |########
  944039.0 |
  945497.2 |
  946955.5 |
  948413.7 |
  949872.0 |####
  951330.2 |########################################
  952788.5 |########################################
  954246.7 |####
  955705.0 |
  957163.2 |
  958621.5 |
  960079.7 |
  961538.0 |
  962996.2 |####
  964454.5 |###################################
  965912.7 |
  967371.0 |
  968829.2 |
  (5 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 4559277.4-4623299.5 ns)
  4559277.4 |###############
  4562478.5 |#####
  4565679.6 |###############
  4568880.7 |#####
  4572081.8 |#########################
  4575282.9 |####################
  4578484.0 |########################################
  4581685.1 |###############
  4584886.2 |###############
  4588087.3 |#####
  4591288.4 |
  4594489.5 |#####
  4597690.6 |
  4600891.8 |
  4604092.9 |#####
  4607294.0 |
  4610495.1 |
  4613696.2 |
  4616897.3 |
  4620098.4 |
  (4 below, 2 above range)

```
