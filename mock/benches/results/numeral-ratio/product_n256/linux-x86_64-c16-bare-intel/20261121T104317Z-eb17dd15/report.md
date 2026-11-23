# The product of two 16-bit fixed points at exponent -8, 64 records

3 variants, 40 samples per variant.
Baseline: **numeral_ratio_product_hand**

## Highlights

Baseline for all deltas below: **numeral_ratio_product_hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### numeral_ratio_product_hand dominates: 54% faster than the next best (numeral_ratio_product_room)

numeral_ratio_product_hand (614 ns) leads numeral_ratio_product_room (944 ns) by 54%, a clear separation rather than a photo finish. CV 65.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### numeral_ratio_product_wide is an outlier: 16.6x slower than the field

numeral_ratio_product_wide (10.20 us) is 16.6x the fastest (614 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### numeral_ratio_product_hand is fastest but the noisiest (CV 65.6%)

numeral_ratio_product_hand wins on median (614 ns) yet has the highest variance (CV 65.6%), while numeral_ratio_product_wide is the steadiest (CV 10.1%, 10.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### numeral_ratio_product_wide shows alternating (throttle bounce) (autocorr -0.75)

numeral_ratio_product_wide's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (numeral_ratio_product_hand)

The baseline numeral_ratio_product_hand is the fastest (614 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.6x the fastest

Fastest numeral_ratio_product_hand (614 ns) to slowest numeral_ratio_product_wide (10.20 us): 16.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### numeral_ratio_product_room is inconsistent: worst-20% is 3.3x its best-20%

numeral_ratio_product_room's best 20% of batches run at 433 ns but its worst 20% at 1.44 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (numeral_ratio_product_hand) is the fastest** at 614.3 ns median
- 2 variants significantly slower than baseline
- Spread: 16.61x (fastest 614.3 ns, slowest 10202.5 ns)

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
| Counter frequency | 2112810317 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| numeral_ratio_product_hand | 1071ns | 917ns | 830ns | 916ns | 1780ns | base |
| numeral_ratio_product_room | 1220ns | 1270ns | 568ns | 1207ns | 1912ns | +13.86% |
| numeral_ratio_product_wide | 10282ns | 10290ns | 9212ns | 10261ns | 11413ns | +859.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| numeral_ratio_product_hand | 768ns | 582ns | 1350ns | base | 0.333 |
| numeral_ratio_product_room | 919ns | 433ns | 1438ns | +19.67% | 0.279 |
| numeral_ratio_product_wide | 10213ns | 9152ns | 11328ns | +1230.41% | 0.025 |

## Performance model

- Peak throughput: **0.591 Gops/s** (numeral_ratio_product_room; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| numeral_ratio_product_hand | 0.417 | 70.6% |
| numeral_ratio_product_room | 0.271 | 45.9% |
| numeral_ratio_product_wide | 0.025 | 4.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| numeral_ratio_product_hand | 1071ns | 1071ns | base |
| numeral_ratio_product_room | 1220ns | 1220ns | +13.86% |
| numeral_ratio_product_wide | 10282ns | 10282ns | +859.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| numeral_ratio_product_hand | 614ns | base | --- | [600, 669] | --- | --- | --- | --- |
| numeral_ratio_product_room | 944ns | +185.5ns (+30.2%) | [+51, +382]ns | [781, 1115] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| numeral_ratio_product_wide | 10202ns | +8824.1ns (+1436.4%) | [+8600, +10488]ns | [9213, 11168] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | numeral_ratio_product_hand | numeral_ratio_product_room | numeral_ratio_product_wide |
|---|---|---|---|---|
| 1 | 1 | 595ns | +77.4% | +1448.8% |
| 1 | 2 | 664ns | +17.6% | +1583.8% |
| 1 | 3 | 601ns | -27.0% | +1425.2% |
| 1 | 4 | 669ns | +95.0% | +1575.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| numeral_ratio_product_hand | -0.702 | HIGH- (thermal bounce) |
| numeral_ratio_product_room | -0.370 | moderate- |
| numeral_ratio_product_wide | -0.752 | HIGH- (thermal bounce) |

**Consistency summary:**

- **numeral_ratio_product_room**: won 1/4, lost 3/4
- **numeral_ratio_product_wide**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| numeral_ratio_product_hand | 101.3ns | 767.6ns | 13.2% | HIGH |
| numeral_ratio_product_room | 96.5ns | 918.7ns | 10.5% | HIGH |
| numeral_ratio_product_wide | 20.8ns | 10212.9ns | 0.2% |  |

## Distribution (algo ns)

```
numeral_ratio_product_hand (n=40, range 582.2-1350.2 ns)
    582.2 |########################################
    620.6 |####
    659.0 |####################
    697.4 |####
    735.8 |##
    774.2 |
    812.6 |##
    851.0 |
    889.4 |
    927.8 |
    966.2 |
   1004.6 |
   1043.0 |##
   1081.4 |
   1119.8 |##
   1158.2 |
   1196.6 |
   1235.0 |
   1273.4 |
   1311.8 |
  (2 below, 3 above range)

numeral_ratio_product_room (n=40, range 433.4-1438.2 ns)
    433.4 |########################################
    483.7 |
    533.9 |
    584.1 |#####
    634.4 |
    684.6 |#####
    734.9 |##################################
    785.1 |#####
    835.3 |#####
    885.6 |#####
    935.8 |######################
    986.1 |#####
   1036.3 |
   1086.5 |############################
   1136.8 |###########
   1187.0 |
   1237.2 |
   1287.5 |######################
   1337.7 |
   1388.0 |
  (2 below, 4 above range)

numeral_ratio_product_wide (n=40, range 9151.7-11328.2 ns)
   9151.7 |########################################
   9260.5 |##
   9369.3 |
   9478.1 |
   9587.0 |
   9695.8 |
   9804.6 |
   9913.4 |
  10022.3 |
  10131.1 |
  10239.9 |
  10348.7 |
  10457.6 |
  10566.4 |
  10675.2 |
  10784.0 |
  10892.9 |
  11001.7 |
  11110.5 |########################################
  11219.3 |##############
  (5 below, 1 above range)

```

## Diagnostics

- **numeral_ratio_product_hand**: CV=52.5% (high variance, measurements may be unstable)
- **numeral_ratio_product_hand**: bridge=15.3% of algo (FFI overhead may distort results)
- **numeral_ratio_product_room**: CV=38.6% (high variance, measurements may be unstable)
- **numeral_ratio_product_room**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **numeral_ratio_product_room**: bridge=10.4% of algo (FFI overhead may distort results)
