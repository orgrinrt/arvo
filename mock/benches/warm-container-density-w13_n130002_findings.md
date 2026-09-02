# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 101% faster than the next best (warm-container-kernel)

warm-container-native (392 ns) leads warm-container-kernel (786 ns) by 101%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 97% (significant)

warm-container-native is -8.13 us (97%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 21.6x slower than the field

warm-container-minimum (8.49 us) is 21.6x the fastest (392 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 10.1%)

warm-container-native wins on median (392 ns) yet has the highest variance (CV 10.1%), while warm-container-lanes-deferred is the steadiest (CV 3.3%, 795 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows warm-up / thermal drift (autocorr +0.84)

warm-container-native's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-plusone, warm-container-minimum} (958% apart)

The field splits into a fast tier {warm-container-native, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-plusone, warm-container-minimum} with a 958% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 21.6x the fastest

Fastest warm-container-native (392 ns) to slowest warm-container-minimum (8.49 us): 21.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 392.1 ns median (-95.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 21.64x (fastest 392.1 ns, slowest 8486.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8596ns | 8471ns | 8346ns | 8492ns | 9158ns | base |
| warm-container-kernel | 855ns | 842ns | 840ns | 844ns | 903ns | -90.05% |
| warm-container-lanes-deferred | 862ns | 854ns | 839ns | 854ns | 907ns | -89.97% |
| warm-container-minimum | 8804ns | 8552ns | 8352ns | 8714ns | 9523ns | +2.42% |
| warm-container-native | 482ns | 458ns | 451ns | 466ns | 563ns | -94.39% |
| warm-container-plusone | 8777ns | 8510ns | 8392ns | 8543ns | 9863ns | +2.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8532ns | 8286ns | 9088ns | base | 2.880 |
| warm-container-kernel | 798ns | 784ns | 842ns | -90.65% | 30.808 |
| warm-container-lanes-deferred | 803ns | 782ns | 844ns | -90.59% | 30.617 |
| warm-container-minimum | 8736ns | 8288ns | 9454ns | +2.39% | 2.813 |
| warm-container-native | 413ns | 389ns | 483ns | -95.16% | 59.459 |
| warm-container-plusone | 8709ns | 8333ns | 9779ns | +2.07% | 2.822 |

## Performance model

- Peak throughput: **63.234 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.921 | 4.6% |
| warm-container-kernel | 31.259 | 49.4% |
| warm-container-lanes-deferred | 30.896 | 48.9% |
| warm-container-minimum | 2.896 | 4.6% |
| warm-container-native | 62.678 | 99.1% |
| warm-container-plusone | 2.910 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8596ns | 8596ns | base |
| warm-container-kernel | 855ns | 855ns | -90.05% |
| warm-container-lanes-deferred | 862ns | 862ns | -89.97% |
| warm-container-minimum | 8804ns | 8804ns | +2.42% |
| warm-container-native | 482ns | 482ns | -94.39% |
| warm-container-plusone | 8777ns | 8777ns | +2.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8412ns | base | --- | [8302, 8518] | --- | --- | --- | --- |
| warm-container-kernel | 786ns | -7596.5ns (-90.3%) | [-7711, -7509]ns | [785, 788] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 795ns | -7585.0ns (-90.2%) | [-7715, -7515]ns | [788, 801] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8487ns | no significant difference | [-0, +316]ns | [8351, 8964] | no | 0.1352 | 0.1081 | 1 |
| warm-container-native | 392ns | -7977.6ns (-94.8%) | [-8068, -7910]ns | [391, 394] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8446ns | no significant difference | [-36, +156]ns | [8420, 8488] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8286ns | -90.5% | -89.5% | +0.2% | -95.2% | +2.3% |
| 2 | 8287ns | -90.5% | -90.4% | +0.0% | -95.2% | +2.9% |
| 3 | 8289ns | -90.5% | -90.4% | +0.0% | -95.4% | +1.8% |
| 4 | 8290ns | -90.5% | -90.3% | -0.0% | -95.3% | +1.6% |
| 5 | 8806ns | -91.0% | -90.9% | -5.8% | -95.6% | -4.7% |
| 6 | 9398ns | -91.6% | -91.5% | -11.9% | -95.8% | -10.9% |
| 7 | 9235ns | -91.5% | -91.0% | -10.1% | -95.8% | -10.2% |
| 8 | 8285ns | -90.5% | -89.9% | +0.8% | -95.3% | +1.5% |
| 9 | 8286ns | -90.6% | -90.0% | +3.9% | -95.3% | +1.9% |
| 10 | 9175ns | -91.5% | -91.0% | -7.5% | -95.7% | -7.1% |
| 11 | 9891ns | -92.1% | -92.0% | -5.0% | -96.0% | -14.2% |
| 12 | 8672ns | -90.9% | -91.0% | +3.3% | -95.5% | +16.0% |
| 13 | 8438ns | -90.7% | -90.8% | -0.1% | -95.3% | +19.1% |
| 14 | 8305ns | -90.5% | -90.6% | +1.3% | -95.3% | +22.0% |
| 15 | 8284ns | -90.5% | -90.6% | +3.5% | -95.3% | +22.6% |
| 16 | 8305ns | -90.5% | -90.6% | +0.0% | -95.3% | +21.4% |
| 17 | 8373ns | -90.6% | -90.6% | -1.0% | -95.3% | +8.4% |
| 18 | 8412ns | -89.1% | -90.7% | -0.8% | -95.3% | -0.7% |
| 19 | 8454ns | -90.7% | -90.8% | +2.3% | -95.3% | -1.5% |
| 20 | 8462ns | -90.7% | -89.2% | +11.0% | -95.4% | +7.0% |
| 21 | 8322ns | -90.6% | -90.5% | +1.6% | -95.3% | +1.3% |
| 22 | 8289ns | -90.5% | -90.5% | +2.4% | -95.3% | +2.8% |
| 23 | 8291ns | -90.5% | -90.5% | -0.0% | -95.3% | +6.3% |
| 24 | 8286ns | -90.5% | -90.5% | +3.7% | -95.2% | +1.8% |
| 25 | 8413ns | -90.6% | -90.6% | +11.7% | -95.4% | -1.3% |
| 26 | 8289ns | -90.5% | -90.5% | +15.1% | -95.3% | +1.4% |
| 27 | 8298ns | -90.5% | -90.5% | +14.7% | -95.3% | +1.7% |
| 28 | 8330ns | -90.6% | -90.6% | +12.8% | -95.3% | +1.8% |
| 29 | 8284ns | -90.5% | -90.5% | +13.4% | -95.3% | +2.0% |
| 30 | 8287ns | -90.5% | -90.6% | +13.9% | -95.3% | +1.5% |
| 31 | 8701ns | -90.9% | -90.7% | -4.7% | -94.5% | -3.3% |
| 32 | 8580ns | -90.7% | -90.4% | -3.4% | -93.7% | -1.9% |
| 33 | 8722ns | -90.9% | -90.7% | -4.9% | -94.6% | -3.2% |
| 34 | 8718ns | -90.9% | -90.8% | -4.0% | -94.6% | -0.2% |
| 35 | 8574ns | -90.6% | -90.5% | +8.9% | -94.4% | +12.3% |
| 36 | 8414ns | -90.4% | -90.1% | +12.2% | -94.4% | +0.8% |
| 37 | 8441ns | -90.6% | -90.3% | +12.5% | -94.4% | +0.5% |
| 38 | 8640ns | -90.6% | -90.8% | +3.8% | -94.6% | -2.9% |
| 39 | 8749ns | -90.6% | -90.9% | +4.4% | -94.6% | -5.2% |
| 40 | 8725ns | -88.8% | -90.9% | +3.7% | -94.6% | -4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.504 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.120 | ok |
| warm-container-lanes-deferred | 0.151 | ok |
| warm-container-minimum | 0.634 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.843 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.712 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 11/40, lost 23/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 15/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 8532.1ns | 0.0% |  |
| warm-container-kernel | 2.4ns | 797.7ns | 0.3% |  |
| warm-container-lanes-deferred | 2.4ns | 802.7ns | 0.3% |  |
| warm-container-minimum | 2.9ns | 8735.9ns | 0.0% |  |
| warm-container-native | 3.0ns | 413.3ns | 0.7% |  |
| warm-container-plusone | 3.0ns | 8709.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8285.7-9087.5 ns)
   8285.7 |########################################
   8325.8 |##
   8365.9 |##
   8405.9 |##############
   8446.0 |#####
   8486.1 |
   8526.2 |
   8566.3 |#####
   8606.4 |##
   8646.5 |##
   8686.6 |###########
   8726.7 |##
   8766.8 |##
   8806.9 |
   8847.0 |
   8887.0 |
   8927.1 |
   8967.2 |
   9007.3 |
   9047.4 |
  (3 below, 4 above range)

warm-container-kernel (n=40, range 783.6-842.5 ns)
    783.6 |########################################
    786.6 |############
    789.5 |##
    792.5 |########
    795.4 |####
    798.3 |
    801.3 |
    804.2 |
    807.2 |####
    810.1 |##
    813.0 |
    816.0 |
    818.9 |
    821.9 |##
    824.8 |
    827.7 |
    830.7 |
    833.6 |
    836.6 |
    839.5 |
  (2 below, 2 above range)

warm-container-lanes-deferred (n=40, range 782.1-843.9 ns)
    782.1 |########################################
    785.2 |#################################
    788.3 |##########################
    791.4 |######
    794.5 |#############
    797.6 |####################
    800.7 |####################
    803.8 |######
    806.8 |######
    809.9 |
    813.0 |######
    816.1 |
    819.2 |####################
    822.3 |
    825.4 |
    828.5 |#############
    831.6 |#############
    834.6 |
    837.7 |
    840.8 |
  (4 below, 2 above range)

warm-container-minimum (n=40, range 8288.5-9453.6 ns)
   8288.5 |########################################
   8346.7 |##########
   8405.0 |##########
   8463.2 |#######
   8521.5 |###
   8579.7 |#######
   8638.0 |###
   8696.3 |
   8754.5 |
   8812.8 |
   8871.0 |
   8929.3 |#######
   8987.5 |
   9045.8 |###
   9104.1 |###
   9162.3 |
   9220.6 |
   9278.8 |###
   9337.1 |###
   9395.3 |#####################
  (2 below, 3 above range)

warm-container-native (n=40, range 388.7-482.5 ns)
    388.7 |########################################
    393.3 |#########
    398.0 |
    402.7 |
    407.4 |
    412.1 |
    416.8 |
    421.5 |
    426.2 |
    430.9 |
    435.6 |
    440.3 |
    445.0 |
    449.7 |
    454.4 |
    459.0 |
    463.7 |
    468.4 |#########
    473.1 |#####
    477.8 |#
  (3 below, 1 above range)

warm-container-plusone (n=40, range 8332.6-9779.1 ns)
   8332.6 |###########
   8404.9 |########################################
   8477.3 |####################
   8549.6 |
   8621.9 |
   8694.2 |##
   8766.6 |##
   8838.9 |
   8911.2 |
   8983.5 |##
   9055.9 |##
   9128.2 |
   9200.5 |
   9272.8 |
   9345.2 |
   9417.5 |
   9489.8 |
   9562.1 |##
   9634.5 |
   9706.8 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.71 (measurement drift or warm-up artifact)
