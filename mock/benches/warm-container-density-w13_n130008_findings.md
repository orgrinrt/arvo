# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 156% faster than the next best (warm-container-lanes-deferred)

warm-container-native (416 ns) leads warm-container-lanes-deferred (1.06 us) by 156%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 95% (significant)

warm-container-native is -6.44 us (95%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 16.4x slower than the field

warm-container-plusone (6.83 us) is 16.4x the fastest (416 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.69)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.69, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-lanes-deferred, warm-container-kernel} vs {warm-container-minimum, warm-container-headroom, warm-container-plusone} (525% apart)

The field splits into a fast tier {warm-container-native, warm-container-lanes-deferred, warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 525% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 16.4x the fastest

Fastest warm-container-native (416 ns) to slowest warm-container-plusone (6.83 us): 16.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 415.8 ns median (-93.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 16.43x (fastest 415.8 ns, slowest 6833.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6913ns | 6819ns | 6662ns | 6841ns | 7380ns | base |
| warm-container-kernel | 1154ns | 1126ns | 1121ns | 1132ns | 1257ns | -83.30% |
| warm-container-lanes-deferred | 1128ns | 1122ns | 1117ns | 1124ns | 1151ns | -83.68% |
| warm-container-minimum | 6801ns | 6737ns | 6664ns | 6750ns | 7092ns | -1.62% |
| warm-container-native | 480ns | 477ns | 473ns | 478ns | 493ns | -93.05% |
| warm-container-plusone | 7005ns | 6897ns | 6744ns | 6910ns | 7552ns | +1.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6844ns | 6594ns | 7295ns | base | 10.773 |
| warm-container-kernel | 1096ns | 1064ns | 1193ns | -83.99% | 67.284 |
| warm-container-lanes-deferred | 1070ns | 1060ns | 1093ns | -84.37% | 68.912 |
| warm-container-minimum | 6739ns | 6605ns | 7027ns | -1.52% | 10.940 |
| warm-container-native | 419ns | 415ns | 427ns | -93.88% | 176.092 |
| warm-container-plusone | 6940ns | 6686ns | 7482ns | +1.42% | 10.623 |

## Performance model

- Peak throughput: **177.722 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.912 | 6.1% |
| warm-container-kernel | 69.001 | 38.8% |
| warm-container-lanes-deferred | 69.339 | 39.0% |
| warm-container-minimum | 11.035 | 6.2% |
| warm-container-native | 177.316 | 99.8% |
| warm-container-plusone | 10.790 | 6.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6913ns | 6913ns | base |
| warm-container-kernel | 1154ns | 1154ns | -83.30% |
| warm-container-lanes-deferred | 1128ns | 1128ns | -83.68% |
| warm-container-minimum | 6801ns | 6801ns | -1.62% |
| warm-container-native | 480ns | 480ns | -93.05% |
| warm-container-plusone | 7005ns | 7005ns | +1.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 6756ns | base | --- | [6713, 6804] | --- | --- | --- | --- |
| warm-container-kernel | 1068ns | -5680.6ns (-84.1%) | [-5718, -5608]ns | [1067, 1076] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 1063ns | -5687.9ns (-84.2%) | [-5729, -5644]ns | [1062, 1068] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 6681ns | -70.6ns (-1.0%) | [-151, -28]ns | [6635, 6742] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-native | 416ns | -6341.1ns (-93.9%) | [-6389, -6285]ns | [416, 417] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6833ns | no significant difference | [-68, +163]ns | [6790, 6894] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 6744ns | -84.2% | -84.2% | -1.4% | -93.6% | +0.2% |
| 2 | 7087ns | -84.9% | -84.9% | -6.5% | -93.9% | -4.4% |
| 3 | 6785ns | -84.3% | -84.4% | -2.4% | -93.8% | +0.5% |
| 4 | 6748ns | -84.2% | -84.3% | -2.0% | -93.8% | +0.6% |
| 5 | 6650ns | -84.0% | -84.0% | -0.4% | -93.6% | +3.9% |
| 6 | 6665ns | -84.0% | -84.0% | -0.4% | -93.7% | +2.5% |
| 7 | 6695ns | -84.1% | -84.1% | -0.6% | -93.7% | +1.5% |
| 8 | 6775ns | -84.3% | -84.3% | -1.8% | -93.8% | -1.5% |
| 9 | 6625ns | -83.9% | -83.9% | +3.7% | -93.5% | +4.3% |
| 10 | 6730ns | -84.2% | -84.2% | +1.1% | -93.6% | -0.6% |
| 11 | 7140ns | -82.1% | -85.1% | -5.8% | -94.2% | -5.2% |
| 12 | 7227ns | -85.2% | -85.3% | -6.4% | -94.2% | -5.0% |
| 13 | 6639ns | -83.9% | -84.0% | +1.1% | -93.7% | +2.9% |
| 14 | 6750ns | -84.2% | -84.3% | -2.4% | -93.8% | +1.9% |
| 15 | 6793ns | -84.2% | -84.4% | -2.9% | -93.9% | +3.8% |
| 16 | 6770ns | -83.7% | -84.3% | -2.5% | -93.9% | +1.2% |
| 17 | 6762ns | -84.1% | -84.3% | -1.8% | -93.9% | +0.4% |
| 18 | 6750ns | -84.2% | -84.3% | +9.1% | -93.8% | +0.7% |
| 19 | 7074ns | -84.9% | -85.0% | +6.2% | -94.1% | -3.0% |
| 20 | 6650ns | -83.7% | -84.1% | +7.8% | -93.7% | +2.4% |
| 21 | 6574ns | -83.8% | -83.6% | +0.5% | -93.7% | +15.0% |
| 22 | 6569ns | -83.8% | -83.6% | +3.2% | -93.7% | +15.4% |
| 23 | 6570ns | -83.8% | -83.7% | +0.6% | -93.7% | +15.3% |
| 24 | 6584ns | -83.6% | -83.6% | +0.5% | -93.7% | +14.8% |
| 25 | 6661ns | -83.7% | -83.8% | -0.4% | -93.8% | +13.8% |
| 26 | 6600ns | -83.8% | -83.7% | +1.3% | -93.7% | +8.5% |
| 27 | 6735ns | -82.2% | -84.0% | +1.4% | -93.8% | +4.5% |
| 28 | 6923ns | -84.4% | -84.2% | -0.7% | -93.9% | +0.1% |
| 29 | 6920ns | -84.6% | -84.3% | -2.3% | -93.9% | -2.5% |
| 30 | 6992ns | -84.8% | -84.6% | -2.8% | -94.1% | -4.4% |
| 31 | 6664ns | -81.8% | -84.0% | -0.8% | -93.7% | +13.4% |
| 32 | 7236ns | -83.3% | -85.3% | -8.4% | -94.2% | +0.7% |
| 33 | 6589ns | -81.6% | -83.9% | +0.4% | -93.7% | +4.9% |
| 34 | 6816ns | -82.4% | -83.4% | -1.0% | -93.9% | -1.8% |
| 35 | 6860ns | -84.5% | -84.5% | -2.1% | -93.9% | -2.6% |
| 36 | 6783ns | -84.2% | -83.4% | -1.1% | -93.9% | -1.5% |
| 37 | 7009ns | -84.7% | -84.9% | -4.7% | -94.1% | -4.4% |
| 38 | 7541ns | -85.4% | -85.9% | -10.7% | -94.5% | -11.4% |
| 39 | 7513ns | -85.1% | -85.8% | -10.1% | -94.5% | -10.6% |
| 40 | 7546ns | -85.1% | -85.9% | -10.4% | -94.5% | -9.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.477 | moderate+ |
| warm-container-kernel | 0.277 | moderate+ |
| warm-container-lanes-deferred | 0.068 | ok |
| warm-container-minimum | 0.592 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.617 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.690 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 27/40, lost 13/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 15/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.4ns | 6843.6ns | 0.0% |  |
| warm-container-kernel | 2.5ns | 1095.8ns | 0.2% |  |
| warm-container-lanes-deferred | 2.4ns | 1069.9ns | 0.2% |  |
| warm-container-minimum | 3.1ns | 6739.4ns | 0.0% |  |
| warm-container-native | 2.7ns | 418.7ns | 0.6% |  |
| warm-container-plusone | 3.1ns | 6940.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 6593.7-7295.5 ns)
   6593.7 |#############
   6628.8 |##########################
   6663.9 |####################
   6699.0 |######
   6734.1 |########################################
   6769.1 |#################################
   6804.2 |######
   6839.3 |######
   6874.4 |
   6909.5 |#############
   6944.6 |
   6979.7 |#############
   7014.8 |
   7049.8 |######
   7084.9 |######
   7120.0 |######
   7155.1 |
   7190.2 |
   7225.3 |#############
   7260.4 |
  (5 below, 3 above range)

warm-container-kernel (n=40, range 1064.3-1193.3 ns)
   1064.3 |########################################
   1070.8 |######
   1077.2 |####
   1083.7 |##
   1090.1 |
   1096.6 |
   1103.0 |####
   1109.5 |
   1115.9 |####
   1122.4 |
   1128.8 |
   1135.3 |
   1141.7 |
   1148.2 |
   1154.6 |
   1161.1 |
   1167.5 |
   1174.0 |
   1180.4 |
   1186.9 |
  (4 below, 6 above range)

warm-container-lanes-deferred (n=40, range 1059.9-1093.3 ns)
   1059.9 |########################################
   1061.6 |################
   1063.2 |################################
   1064.9 |
   1066.6 |####
   1068.2 |########
   1069.9 |
   1071.6 |
   1073.2 |####
   1074.9 |####
   1076.6 |########
   1078.3 |################
   1079.9 |
   1081.6 |
   1083.3 |####
   1084.9 |
   1086.6 |
   1088.3 |
   1089.9 |
   1091.6 |
  (3 below, 3 above range)

warm-container-minimum (n=40, range 6605.2-7027.3 ns)
   6605.2 |########################################
   6626.4 |#########################
   6647.5 |###############
   6668.6 |##########
   6689.7 |#####
   6710.8 |###############
   6731.9 |##########
   6753.0 |####################
   6774.1 |#####
   6795.2 |##########
   6816.3 |#####
   6837.4 |
   6858.5 |##########
   6879.6 |
   6900.7 |
   6921.8 |
   6942.9 |
   6964.0 |
   6985.1 |
   7006.2 |
  (3 below, 3 above range)

warm-container-native (n=40, range 414.9-427.3 ns)
    414.9 |########################################
    415.5 |############################
    416.1 |####################
    416.7 |
    417.3 |####
    418.0 |
    418.6 |
    419.2 |
    419.8 |
    420.4 |####
    421.1 |####
    421.7 |################
    422.3 |####
    422.9 |####
    423.5 |####
    424.2 |
    424.8 |
    425.4 |
    426.0 |
    426.6 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 6686.0-7482.1 ns)
   6686.0 |##########################
   6725.8 |#############
   6765.6 |########################################
   6805.4 |#################################
   6845.2 |##########################
   6885.0 |####################
   6924.8 |######
   6964.6 |
   7004.4 |######
   7044.2 |######
   7084.1 |
   7123.9 |######
   7163.7 |
   7203.5 |
   7243.3 |
   7283.1 |######
   7322.9 |
   7362.7 |
   7402.5 |
   7442.3 |
  (5 below, 6 above range)

```

## Diagnostics

- **warm-container-minimum**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.69 (measurement drift or warm-up artifact)
