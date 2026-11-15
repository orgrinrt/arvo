# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 108% faster than the next best (warm-container-kernel)

warm-container-native (397 ns) leads warm-container-kernel (826 ns) by 108%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 96% (significant)

warm-container-native is -8.05 us (96%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 21.7x slower than the field

warm-container-plusone (8.63 us) is 21.7x the fastest (397 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native shows warm-up / thermal drift (autocorr +0.64)

warm-container-native's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-minimum, warm-container-plusone} (911% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-plusone} with a 911% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 21.7x the fastest

Fastest warm-container-native (397 ns) to slowest warm-container-plusone (8.63 us): 21.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.3x its best-20%

warm-container-kernel's best 20% of batches run at 783 ns but its worst 20% at 1.81 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 396.9 ns median (-95.3% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 21.74x (fastest 396.9 ns, slowest 8629.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8491ns | 8455ns | 8356ns | 8467ns | 8697ns | base |
| warm-container-kernel | 1098ns | 889ns | 841ns | 902ns | 1942ns | -87.07% |
| warm-container-lanes-deferred | 885ns | 892ns | 855ns | 887ns | 908ns | -89.58% |
| warm-container-minimum | 8762ns | 8656ns | 8360ns | 8656ns | 9483ns | +3.19% |
| warm-container-native | 461ns | 459ns | 451ns | 460ns | 477ns | -94.57% |
| warm-container-plusone | 8925ns | 8701ns | 8388ns | 8704ns | 10126ns | +5.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8423ns | 8297ns | 8621ns | base | 3.890 |
| warm-container-kernel | 1023ns | 783ns | 1812ns | -87.85% | 32.024 |
| warm-container-lanes-deferred | 822ns | 796ns | 844ns | -90.24% | 39.842 |
| warm-container-minimum | 8693ns | 8297ns | 9404ns | +3.20% | 3.770 |
| warm-container-native | 398ns | 389ns | 411ns | -95.27% | 82.234 |
| warm-container-plusone | 8845ns | 8324ns | 10001ns | +5.01% | 3.705 |

## Performance model

- Peak throughput: **84.201 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.907 | 4.6% |
| warm-container-kernel | 39.649 | 47.1% |
| warm-container-lanes-deferred | 39.518 | 46.9% |
| warm-container-minimum | 3.814 | 4.5% |
| warm-container-native | 82.560 | 98.1% |
| warm-container-plusone | 3.797 | 4.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8491ns | 8491ns | base |
| warm-container-kernel | 1098ns | 1098ns | -87.07% |
| warm-container-lanes-deferred | 885ns | 885ns | -89.58% |
| warm-container-minimum | 8762ns | 8762ns | +3.19% |
| warm-container-native | 461ns | 461ns | -94.57% |
| warm-container-plusone | 8925ns | 8925ns | +5.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8387ns | base | --- | [8344, 8446] | --- | --- | --- | --- |
| warm-container-kernel | 826ns | -7531.6ns (-89.8%) | [-7558, -7509]ns | [802, 890] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 829ns | -7538.2ns (-89.9%) | [-7641, -7514]ns | [820, 831] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8591ns | +214.2ns (+2.6%) | [+15, +311]ns | [8446, 8718] | YES | 0.0385 | 0.0385 | 0 |
| warm-container-native | 397ns | -7997.0ns (-95.3%) | [-8036, -7945]ns | [393, 400] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8630ns | +177.5ns (+2.1%) | [+55, +343]ns | [8468, 8730] | YES | 0.0080 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8343ns | -90.6% | -90.0% | +20.3% | -95.3% | -0.3% |
| 2 | 8279ns | -90.5% | -90.1% | +10.0% | -95.3% | +4.7% |
| 3 | 8295ns | -90.5% | -89.9% | +6.0% | -95.3% | -0.1% |
| 4 | 8345ns | -90.6% | -90.0% | -0.9% | -95.2% | +3.1% |
| 5 | 8284ns | -90.3% | -90.0% | +0.6% | -95.2% | +3.5% |
| 6 | 8324ns | -90.3% | -90.0% | -0.4% | -95.3% | +8.8% |
| 7 | 8328ns | -90.1% | -90.0% | +3.2% | -95.3% | +1.4% |
| 8 | 8332ns | -90.1% | -90.0% | +12.5% | -95.3% | +2.5% |
| 9 | 8275ns | -90.1% | -89.9% | +13.8% | -95.3% | +1.6% |
| 10 | 8371ns | -90.4% | -90.1% | +17.4% | -95.4% | +3.7% |
| 11 | 8579ns | -68.6% | -90.7% | -2.0% | -95.4% | +1.8% |
| 12 | 8685ns | -70.3% | -90.8% | -2.4% | -95.4% | +0.6% |
| 13 | 8588ns | -71.4% | -90.7% | -1.5% | -95.4% | +9.5% |
| 14 | 8690ns | -90.4% | -90.3% | -2.9% | -95.5% | +23.5% |
| 15 | 8403ns | -85.4% | -89.5% | -0.0% | -95.4% | +4.5% |
| 16 | 8322ns | -67.8% | -90.4% | +0.1% | -95.3% | +1.4% |
| 17 | 8325ns | -90.0% | -90.0% | -0.7% | -95.3% | +1.3% |
| 18 | 8402ns | -90.1% | -90.2% | -1.3% | -95.3% | +0.7% |
| 19 | 8372ns | -90.1% | -89.9% | -1.0% | -95.3% | +3.8% |
| 20 | 8642ns | -90.4% | -90.6% | -3.9% | -95.5% | -0.6% |
| 21 | 8319ns | -90.6% | -90.0% | +0.6% | -95.0% | +12.0% |
| 22 | 8346ns | -90.6% | -90.1% | +3.2% | -95.0% | +5.0% |
| 23 | 8287ns | -90.5% | -90.0% | +5.0% | -95.2% | +13.3% |
| 24 | 8312ns | -90.6% | -89.9% | +4.8% | -95.2% | +16.8% |
| 25 | 8343ns | -90.3% | -90.0% | +4.4% | -95.1% | +9.1% |
| 26 | 8368ns | -90.6% | -90.0% | +5.4% | -95.2% | +54.2% |
| 27 | 8470ns | -90.7% | -90.2% | +3.1% | -95.3% | +9.2% |
| 28 | 8602ns | -90.9% | -90.3% | +6.4% | -95.4% | +8.1% |
| 29 | 8525ns | -90.8% | -90.3% | +3.1% | -95.3% | +2.4% |
| 30 | 8366ns | -90.6% | -90.0% | +4.1% | -95.2% | +4.4% |
| 31 | 8480ns | -88.8% | -90.6% | +3.6% | -95.3% | -2.3% |
| 32 | 8647ns | -89.0% | -90.7% | +9.6% | -95.4% | +0.1% |
| 33 | 8493ns | -88.8% | -90.3% | -1.7% | -95.3% | -0.2% |
| 34 | 8430ns | -88.7% | -90.5% | +1.5% | -95.2% | +0.6% |
| 35 | 8433ns | -88.7% | -90.6% | +0.5% | -95.1% | -0.2% |
| 36 | 8455ns | -89.0% | -90.6% | +3.2% | -95.1% | -0.4% |
| 37 | 8473ns | -89.5% | -90.6% | +0.3% | -95.2% | -0.9% |
| 38 | 8438ns | -89.5% | -90.6% | -0.5% | -95.2% | -1.8% |
| 39 | 8424ns | -89.4% | -90.5% | +2.0% | -95.1% | -1.5% |
| 40 | 8532ns | -89.6% | -90.4% | +3.7% | -95.2% | -2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.521 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.471 | moderate+ |
| warm-container-lanes-deferred | 0.339 | moderate+ |
| warm-container-minimum | 0.414 | moderate+ |
| warm-container-native | 0.638 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.269 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 12/40, lost 26/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 11/40, lost 29/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 4.2ns | 8423.2ns | 0.1% |  |
| warm-container-kernel | 3.4ns | 1023.2ns | 0.3% |  |
| warm-container-lanes-deferred | 2.5ns | 822.5ns | 0.3% |  |
| warm-container-minimum | 2.7ns | 8692.7ns | 0.0% |  |
| warm-container-native | 2.5ns | 398.5ns | 0.6% |  |
| warm-container-plusone | 3.9ns | 8845.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8296.5-8620.8 ns)
   8296.5 |########
   8312.8 |########################################
   8329.0 |########################
   8345.2 |################
   8361.4 |################################
   8377.6 |
   8393.8 |################
   8410.0 |########
   8426.2 |########################
   8442.4 |########
   8458.7 |################
   8474.9 |########
   8491.1 |########
   8507.3 |
   8523.5 |################
   8539.7 |
   8555.9 |
   8572.1 |################
   8588.4 |########
   8604.6 |
  (5 below, 4 above range)

warm-container-kernel (n=40, range 783.5-1812.3 ns)
    783.5 |########################################
    834.9 |
    886.3 |#########
    937.8 |#########
    989.2 |
   1040.7 |
   1092.1 |
   1143.6 |
   1195.0 |#
   1246.4 |
   1297.9 |
   1349.3 |
   1400.8 |
   1452.2 |
   1503.6 |
   1555.1 |
   1606.5 |
   1658.0 |
   1709.4 |
   1760.8 |
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 795.7-844.2 ns)
    795.7 |##################################
    798.1 |
    800.6 |
    803.0 |###########
    805.4 |#####
    807.8 |
    810.3 |
    812.7 |#####
    815.1 |#####
    817.5 |
    820.0 |
    822.4 |#####
    824.8 |#################
    827.2 |######################
    829.7 |######################
    832.1 |########################################
    834.5 |#################
    836.9 |#####
    839.4 |
    841.8 |#####
  (3 below, 2 above range)

warm-container-minimum (n=40, range 8296.7-9404.1 ns)
   8296.7 |##########################
   8352.0 |##########################
   8407.4 |#############
   8462.8 |####################
   8518.1 |######
   8573.5 |####################
   8628.9 |
   8684.3 |########################################
   8739.6 |####################
   8795.0 |#############
   8850.4 |
   8905.7 |
   8961.1 |
   9016.5 |
   9071.8 |######
   9127.2 |######
   9182.6 |
   9238.0 |
   9293.3 |
   9348.7 |######
  (5 below, 4 above range)

warm-container-native (n=40, range 389.2-411.4 ns)
    389.2 |####################
    390.3 |##########
    391.4 |##############################
    392.5 |########################################
    393.6 |##########
    394.7 |####################
    395.8 |##############################
    396.9 |########################################
    398.0 |####################
    399.1 |
    400.3 |##############################
    401.4 |##########
    402.5 |
    403.6 |
    404.7 |##########
    405.8 |
    406.9 |
    408.0 |
    409.1 |##############################
    410.2 |####################
  (4 below, 4 above range)

warm-container-plusone (n=40, range 8324.1-10000.8 ns)
   8324.1 |####
   8407.9 |########################################
   8491.8 |########
   8575.6 |#############
   8659.4 |###############################
   8743.3 |########
   8827.1 |
   8910.9 |
   8994.8 |####
   9078.6 |####
   9162.4 |
   9246.3 |#############
   9330.1 |########
   9413.9 |
   9497.8 |
   9581.6 |
   9665.5 |####
   9749.3 |
   9833.1 |
   9917.0 |
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=52.2% (high variance, measurements may be unstable)
- **warm-container-native**: autocorrelation=0.64 (measurement drift or warm-up artifact)
