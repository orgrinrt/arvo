# The product of two 16-bit fixed points at exponent -8, 64 records

3 variants, 40 samples per variant.
Baseline: **numeral_ratio_product_hand**

## Highlights

Baseline for all deltas below: **numeral_ratio_product_hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### numeral_ratio_product_hand dominates: 12% faster than the next best (numeral_ratio_product_room)

numeral_ratio_product_hand (153 ns) leads numeral_ratio_product_room (172 ns) by 12%, a clear separation rather than a photo finish. CV 16.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### numeral_ratio_product_wide is an outlier: 71.4x slower than the field

numeral_ratio_product_wide (10.92 us) is 71.4x the fastest (153 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (numeral_ratio_product_hand)

The baseline numeral_ratio_product_hand is the fastest (153 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 71.4x the fastest

Fastest numeral_ratio_product_hand (153 ns) to slowest numeral_ratio_product_wide (10.92 us): 71.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### numeral_ratio_product_room is inconsistent: worst-20% is 1.7x its best-20%

numeral_ratio_product_room's best 20% of batches run at 169 ns but its worst 20% at 281 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (numeral_ratio_product_hand) is the fastest** at 153.0 ns median
- 2 variants significantly slower than baseline
- Spread: 71.37x (fastest 153.0 ns, slowest 10920.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The product of two 16-bit fixed points at exponent -8, 64 records |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2300262086 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| numeral_ratio_product_hand | 259ns | 241ns | 239ns | 241ns | 331ns | base |
| numeral_ratio_product_room | 290ns | 263ns | 257ns | 263ns | 406ns | +12.25% |
| numeral_ratio_product_wide | 11173ns | 11018ns | 10727ns | 11064ns | 11947ns | +4217.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| numeral_ratio_product_hand | 159ns | 151ns | 186ns | base | 1.607 |
| numeral_ratio_product_room | 193ns | 169ns | 281ns | +21.16% | 1.327 |
| numeral_ratio_product_wide | 11065ns | 10616ns | 11822ns | +6847.70% | 0.023 |

## Performance model

- Peak throughput: **1.695 Gops/s** (numeral_ratio_product_hand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| numeral_ratio_product_hand | 1.673 | 98.7% |
| numeral_ratio_product_room | 1.493 | 88.1% |
| numeral_ratio_product_wide | 0.023 | 1.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| numeral_ratio_product_hand | 259ns | 259ns | base |
| numeral_ratio_product_room | 290ns | 290ns | +12.25% |
| numeral_ratio_product_wide | 11173ns | 11173ns | +4217.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| numeral_ratio_product_hand | 153ns | base | --- | [153, 154] | --- | --- | --- | --- |
| numeral_ratio_product_room | 172ns | +18.6ns (+12.2%) | [+18, +20]ns | [171, 172] | YES | 0.0000 | 0.0000 | 0 |
| numeral_ratio_product_wide | 10920ns | +10762.9ns (+7034.6%) | [+10705, +10864]ns | [10858, 11042] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | numeral_ratio_product_hand | numeral_ratio_product_room | numeral_ratio_product_wide |
|---|---|---|---|---|
| 1 | 1 | 154ns | +11.5% | +6886.6% |
| 1 | 2 | 151ns | +14.3% | +6989.3% |
| 1 | 3 | 153ns | +12.2% | +7436.7% |
| 1 | 4 | 154ns | +9.4% | +7016.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| numeral_ratio_product_hand | -0.182 | ok |
| numeral_ratio_product_room | -0.004 | ok |
| numeral_ratio_product_wide | -0.181 | ok |

**Consistency summary:**

- **numeral_ratio_product_room**: won 0/4, lost 4/4
- **numeral_ratio_product_wide**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| numeral_ratio_product_hand | 36.1ns | 159.3ns | 22.7% | HIGH |
| numeral_ratio_product_room | 35.7ns | 193.0ns | 18.5% | HIGH |
| numeral_ratio_product_wide | 36.9ns | 11064.9ns | 0.3% |  |

## Distribution (algo ns)

```
numeral_ratio_product_hand (n=40, range 151.0-185.9 ns)
    151.0 |############
    152.8 |########################################
    154.5 |###
    156.3 |
    158.0 |
    159.8 |
    161.5 |
    163.2 |
    165.0 |
    166.7 |
    168.5 |
    170.2 |
    172.0 |
    173.7 |
    175.5 |
    177.2 |
    178.9 |
    180.7 |
    182.4 |
    184.2 |
  (5 below, 3 above range)

numeral_ratio_product_room (n=40, range 168.7-281.0 ns)
    168.7 |########################################
    174.3 |
    179.9 |
    185.6 |
    191.2 |
    196.8 |
    202.4 |
    208.0 |
    213.6 |
    219.2 |
    224.8 |
    230.5 |
    236.1 |
    241.7 |
    247.3 |
    252.9 |
    258.5 |
    264.1 |
    269.8 |
    275.4 |
  (5 below, 4 above range)

numeral_ratio_product_wide (n=40, range 10615.6-11821.7 ns)
  10615.6 |######
  10675.9 |##########################
  10736.2 |######
  10796.5 |####################
  10856.8 |########################################
  10917.1 |##########################
  10977.4 |#############
  11037.7 |#############
  11098.0 |
  11158.3 |#############
  11218.7 |
  11279.0 |
  11339.3 |######
  11399.6 |#############
  11459.9 |#############
  11520.2 |######
  11580.5 |######
  11640.8 |######
  11701.1 |
  11761.4 |
  (5 below, 2 above range)

```

## Diagnostics

- **numeral_ratio_product_hand**: bridge=23.4% of algo (FFI overhead may distort results)
- **numeral_ratio_product_room**: CV=34.0% (high variance, measurements may be unstable)
- **numeral_ratio_product_room**: bridge=20.9% of algo (FFI overhead may distort results)
