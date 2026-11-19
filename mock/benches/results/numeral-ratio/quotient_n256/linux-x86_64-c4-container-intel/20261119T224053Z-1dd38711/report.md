# The quotient of a 16-bit fixed point by a value read at run time, 64 records

3 variants, 40 samples per variant.
Baseline: **numeral_ratio_quotient_hand**

## Highlights

Baseline for all deltas below: **numeral_ratio_quotient_hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### numeral_ratio_quotient_hand dominates: 801% faster than the next best (numeral_ratio_quotient_room)

numeral_ratio_quotient_hand (153 ns) leads numeral_ratio_quotient_room (1.37 us) by 801%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### numeral_ratio_quotient_wide is an outlier: 79.3x slower than the field

numeral_ratio_quotient_wide (12.10 us) is 79.3x the fastest (153 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (numeral_ratio_quotient_hand)

The baseline numeral_ratio_quotient_hand is the fastest (153 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 79.3x the fastest

Fastest numeral_ratio_quotient_hand (153 ns) to slowest numeral_ratio_quotient_wide (12.10 us): 79.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### numeral_ratio_quotient_wide is inconsistent: worst-20% is 2.0x its best-20%

numeral_ratio_quotient_wide's best 20% of batches run at 11.77 us but its worst 20% at 23.05 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (numeral_ratio_quotient_hand) is the fastest** at 152.6 ns median
- 2 variants significantly slower than baseline
- Spread: 79.31x (fastest 152.6 ns, slowest 12098.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The quotient of a 16-bit fixed point by a value read at run time, 64 records |
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
| numeral_ratio_quotient_hand | 252ns | 241ns | 229ns | 241ns | 308ns | base |
| numeral_ratio_quotient_room | 1441ns | 1465ns | 1318ns | 1437ns | 1574ns | +471.98% |
| numeral_ratio_quotient_wide | 14384ns | 12193ns | 11868ns | 12295ns | 23168ns | +5610.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| numeral_ratio_quotient_hand | 154ns | 145ns | 166ns | base | 1.666 |
| numeral_ratio_quotient_room | 1342ns | 1223ns | 1459ns | +773.05% | 0.191 |
| numeral_ratio_quotient_wide | 14277ns | 11769ns | 23049ns | +9190.62% | 0.018 |

## Performance model

- Peak throughput: **1.761 Gops/s** (numeral_ratio_quotient_hand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| numeral_ratio_quotient_hand | 1.678 | 95.3% |
| numeral_ratio_quotient_room | 0.186 | 10.6% |
| numeral_ratio_quotient_wide | 0.021 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| numeral_ratio_quotient_hand | 252ns | 252ns | base |
| numeral_ratio_quotient_room | 1441ns | 1441ns | +471.98% |
| numeral_ratio_quotient_wide | 14384ns | 14384ns | +5610.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| numeral_ratio_quotient_hand | 153ns | base | --- | [152, 153] | --- | --- | --- | --- |
| numeral_ratio_quotient_room | 1374ns | +1222.1ns (+801.1%) | [+1109, +1241]ns | [1265, 1394] | YES | 0.0000 | 0.0000 | 0 |
| numeral_ratio_quotient_wide | 12099ns | +11946.0ns (+7830.9%) | [+11880, +12183]ns | [12033, 12359] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | numeral_ratio_quotient_hand | numeral_ratio_quotient_room | numeral_ratio_quotient_wide |
|---|---|---|---|---|
| 1 | 1 | 153ns | +802.5% | +7816.4% |
| 1 | 2 | 146ns | +739.8% | +8001.2% |
| 1 | 3 | 153ns | +722.9% | +7795.5% |
| 1 | 4 | 155ns | +810.0% | +8049.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| numeral_ratio_quotient_hand | -0.218 | moderate- |
| numeral_ratio_quotient_room | -0.229 | moderate- |
| numeral_ratio_quotient_wide | 0.034 | ok |

**Consistency summary:**

- **numeral_ratio_quotient_room**: won 0/4, lost 4/4
- **numeral_ratio_quotient_wide**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| numeral_ratio_quotient_hand | 49.3ns | 153.7ns | 32.1% | HIGH |
| numeral_ratio_quotient_room | 39.3ns | 1341.6ns | 2.9% |  |
| numeral_ratio_quotient_wide | 37.2ns | 14276.9ns | 0.3% |  |

## Distribution (algo ns)

```
numeral_ratio_quotient_hand (n=40, range 145.3-165.7 ns)
    145.3 |########################
    146.4 |
    147.4 |
    148.4 |
    149.4 |
    150.4 |
    151.4 |########################################
    152.5 |########################################
    153.5 |########
    154.5 |####################
    155.5 |########
    156.5 |
    157.5 |
    158.5 |
    159.6 |
    160.6 |
    161.6 |
    162.6 |
    163.6 |
    164.6 |
  (3 below, 2 above range)

numeral_ratio_quotient_room (n=40, range 1222.6-1459.4 ns)
   1222.6 |
   1234.4 |#####
   1246.2 |##################################
   1258.1 |###########
   1269.9 |#####
   1281.8 |
   1293.6 |#####
   1305.4 |
   1317.3 |
   1329.1 |
   1341.0 |
   1352.8 |
   1364.7 |##################################
   1376.5 |#################
   1388.3 |
   1400.2 |########################################
   1412.0 |#################
   1423.9 |
   1435.7 |
   1447.6 |
  (6 below, 4 above range)

numeral_ratio_quotient_wide (n=40, range 11769.2-23048.9 ns)
  11769.2 |########################################
  12333.2 |###################
  12897.2 |#
  13461.2 |
  14025.2 |
  14589.1 |
  15153.1 |#
  15717.1 |
  16281.1 |
  16845.1 |
  17409.1 |
  17973.0 |
  18537.0 |
  19101.0 |
  19665.0 |
  20229.0 |
  20792.9 |
  21356.9 |
  21920.9 |
  22484.9 |
  (5 below, 2 above range)

```

## Diagnostics

- **numeral_ratio_quotient_hand**: bridge=23.5% of algo (FFI overhead may distort results)
- **numeral_ratio_quotient_wide**: CV=61.8% (high variance, measurements may be unstable)
