# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 280% faster than the next best (warm-container-headroom)

warm-container-kernel (1.06 us) leads warm-container-headroom (4.01 us) by 280%, a clear separation rather than a photo finish. CV 18.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 77% (significant)

warm-container-kernel is -3.10 us (77%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 7.3x slower than the field

warm-container-plusone (7.75 us) is 7.3x the fastest (1.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.75)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-headroom, warm-container-minimum, warm-container-native, warm-container-plusone} (280% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-native, warm-container-plusone} with a 280% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest warm-container-kernel (1.06 us) to slowest warm-container-plusone (7.75 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 2.7x its best-20%

warm-container-headroom's best 20% of batches run at 3.28 us but its worst 20% at 8.92 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1056.4 ns median (-73.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.34x (fastest 1056.4 ns, slowest 7753.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112691662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4807ns | 4076ns | 3334ns | 3865ns | 9108ns | base |
| warm-container-kernel | 1325ns | 1304ns | 1041ns | 1288ns | 1718ns | -72.44% |
| warm-container-minimum | 7394ns | 7532ns | 5480ns | 7364ns | 9400ns | +53.81% |
| warm-container-native | 7330ns | 7811ns | 6405ns | 7212ns | 8609ns | +52.48% |
| warm-container-plusone | 7832ns | 7837ns | 7770ns | 7832ns | 7891ns | +62.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4723ns | 3282ns | 8915ns | base | 6.939 |
| warm-container-kernel | 1083ns | 865ns | 1363ns | -77.07% | 30.254 |
| warm-container-minimum | 7323ns | 5422ns | 9315ns | +55.07% | 4.474 |
| warm-container-native | 7257ns | 6347ns | 8515ns | +53.67% | 4.515 |
| warm-container-plusone | 7755ns | 7694ns | 7807ns | +64.20% | 4.226 |

## Performance model

- Peak throughput: **37.869 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.167 | 21.6% |
| warm-container-kernel | 31.019 | 81.9% |
| warm-container-minimum | 4.394 | 11.6% |
| warm-container-native | 4.235 | 11.2% |
| warm-container-plusone | 4.226 | 11.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4807ns | 4807ns | base |
| warm-container-kernel | 1325ns | 1325ns | -72.44% |
| warm-container-minimum | 7394ns | 7394ns | +53.81% |
| warm-container-native | 7330ns | 7330ns | +52.48% |
| warm-container-plusone | 7832ns | 7832ns | +62.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4012ns | base | --- | [3293, 4018] | --- | --- | --- | --- |
| warm-container-kernel | 1056ns | -3095.6ns (-77.2%) | [-3141, -2183]ns | [922, 1230] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 7457ns | +3508.2ns (+87.4%) | [+2202, +4136]ns | [6952, 7725] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 7737ns | +3055.3ns (+76.2%) | [+2620, +3727]ns | [6370, 7754] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 7754ns | +3780.7ns (+94.2%) | [+3731, +4398]ns | [7742, 7780] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 4012ns | -77.0% | +37.8% | +58.8% | +94.5% |
| 1 | 2 | 3292ns | -62.6% | +134.7% | +92.9% | +136.3% |
| 1 | 3 | 4018ns | -77.3% | +112.5% | +92.9% | +92.7% |
| 1 | 4 | 3288ns | -59.5% | +119.7% | +136.8% | +134.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.750 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.662 | HIGH- (thermal bounce) |
| warm-container-minimum | -0.051 | ok |
| warm-container-native | 0.253 | moderate+ |
| warm-container-plusone | 0.249 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 0/4, lost 4/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 28.5ns | 4722.5ns | 0.6% |  |
| warm-container-kernel | 84.0ns | 1083.1ns | 7.8% | HIGH |
| warm-container-minimum | 21.9ns | 7323.3ns | 0.3% |  |
| warm-container-native | 22.6ns | 7257.1ns | 0.3% |  |
| warm-container-plusone | 24.3ns | 7754.5ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3282.5-8915.0 ns)
   3282.5 |############################
   3564.1 |
   3845.7 |########################################
   4127.4 |
   4409.0 |
   4690.6 |####
   4972.3 |##
   5253.9 |
   5535.5 |
   5817.1 |
   6098.8 |
   6380.4 |
   6662.0 |
   6943.7 |
   7225.3 |
   7506.9 |
   7788.5 |
   8070.2 |
   8351.8 |
   8633.4 |
  (3 below, 5 above range)

warm-container-kernel (n=40, range 865.3-1362.6 ns)
    865.3 |########
    890.2 |########
    915.0 |########################################
    939.9 |
    964.8 |
    989.6 |####
   1014.5 |
   1039.4 |
   1064.2 |
   1089.1 |############
   1114.0 |
   1138.8 |
   1163.7 |
   1188.6 |####
   1213.4 |################################
   1238.3 |####
   1263.2 |
   1288.0 |
   1312.9 |########
   1337.8 |####
  (5 below, 4 above range)

warm-container-minimum (n=40, range 5421.7-9315.0 ns)
   5421.7 |######
   5616.3 |##########################
   5811.0 |######
   6005.7 |######
   6200.3 |
   6395.0 |######
   6589.7 |######
   6784.3 |
   6979.0 |
   7173.7 |##########################
   7368.3 |####################
   7563.0 |########################################
   7757.7 |######
   7952.3 |
   8147.0 |#############
   8341.7 |####################
   8536.3 |#############
   8731.0 |
   8925.6 |
   9120.3 |######
  (5 below, 4 above range)

warm-container-native (n=40, range 6346.5-8515.1 ns)
   6346.5 |########################################
   6455.0 |##
   6563.4 |
   6671.8 |
   6780.3 |##
   6888.7 |
   6997.1 |
   7105.5 |
   7214.0 |
   7322.4 |
   7430.8 |
   7539.2 |
   7647.7 |######################
   7756.1 |##################################
   7864.5 |
   7972.9 |
   8081.4 |
   8189.8 |
   8298.2 |
   8406.6 |
  (3 below, 1 above range)

warm-container-plusone (n=40, range 7694.1-7807.2 ns)
   7694.1 |###############
   7699.8 |####################
   7705.4 |
   7711.1 |
   7716.7 |
   7722.4 |#####
   7728.0 |#####
   7733.7 |
   7739.3 |########################################
   7745.0 |
   7750.6 |
   7756.3 |
   7762.0 |#####
   7767.6 |#####
   7773.3 |###############
   7778.9 |###############
   7784.6 |###############
   7790.2 |##########
   7795.9 |
   7801.5 |#########################
  (3 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=53.9% (high variance, measurements may be unstable)
