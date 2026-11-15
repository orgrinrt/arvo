# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon shows warm-up / thermal drift (autocorr +0.81)

satfold-neon's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### satfold-neon's edge over baseline is significant but tiny (12 ns, 0.15%)

satfold-neon differs from baseline satfold-iterfold by 12 ns (0.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes4-idx** at 7377.9 ns median (-6.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.09x (fastest 7377.9 ns, slowest 8075.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 8051ns | 7982ns | 7945ns | 8008ns | 8288ns | base |
| satfold-lanes16 | 8105ns | 7960ns | 7871ns | 7994ns | 8670ns | +0.66% |
| satfold-lanes16-constl | 8075ns | 8002ns | 7883ns | 8018ns | 8434ns | +0.29% |
| satfold-lanes4-idx | 7465ns | 7448ns | 7282ns | 7448ns | 7701ns | -7.28% |
| satfold-lanes64 | 8093ns | 8058ns | 7895ns | 8056ns | 8400ns | +0.51% |
| satfold-neon | 8274ns | 8051ns | 7945ns | 8156ns | 8958ns | +2.77% |
| satfold-neon8 | 8080ns | 8009ns | 7921ns | 8036ns | 8368ns | +0.35% |
| satfold-nolaw | 8165ns | 8134ns | 7948ns | 8143ns | 8448ns | +1.41% |
| satfold-seq | 8115ns | 8061ns | 7941ns | 8039ns | 8517ns | +0.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 7988ns | 7886ns | 8225ns | base | 4.102 |
| satfold-lanes16 | 8044ns | 7814ns | 8603ns | +0.70% | 4.073 |
| satfold-lanes16-constl | 8010ns | 7823ns | 8363ns | +0.27% | 4.091 |
| satfold-lanes4-idx | 7396ns | 7218ns | 7621ns | -7.42% | 4.431 |
| satfold-lanes64 | 8030ns | 7839ns | 8331ns | +0.52% | 4.081 |
| satfold-neon | 8211ns | 7888ns | 8886ns | +2.78% | 3.991 |
| satfold-neon8 | 8014ns | 7864ns | 8300ns | +0.32% | 4.089 |
| satfold-nolaw | 8104ns | 7891ns | 8383ns | +1.45% | 4.043 |
| satfold-seq | 8052ns | 7884ns | 8435ns | +0.79% | 4.070 |

## Performance model

- Peak throughput: **4.540 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 4.141 | 91.2% |
| satfold-lanes16 | 4.147 | 91.4% |
| satfold-lanes16-constl | 4.126 | 90.9% |
| satfold-lanes4-idx | 4.441 | 97.8% |
| satfold-lanes64 | 4.097 | 90.3% |
| satfold-neon | 4.101 | 90.3% |
| satfold-neon8 | 4.124 | 90.8% |
| satfold-nolaw | 4.058 | 89.4% |
| satfold-seq | 4.094 | 90.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 8051ns | 8051ns | base |
| satfold-lanes16 | 8105ns | 8105ns | +0.66% |
| satfold-lanes16-constl | 8075ns | 8075ns | +0.29% |
| satfold-lanes4-idx | 7465ns | 7465ns | -7.28% |
| satfold-lanes64 | 8093ns | 8093ns | +0.51% |
| satfold-neon | 8274ns | 8274ns | +2.77% |
| satfold-neon8 | 8080ns | 8080ns | +0.35% |
| satfold-nolaw | 8165ns | 8165ns | +1.41% |
| satfold-seq | 8115ns | 8115ns | +0.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 7913ns | base | --- | [7895, 7990] | --- | --- | --- | --- |
| satfold-lanes16 | 7901ns | -70.6ns (-0.9%) | [-89, -14]ns | [7837, 8018] | YES (adj: no) | 0.1539 | 0.0385 | 0 |
| satfold-lanes16-constl | 7942ns | no significant difference | [-72, +30]ns | [7909, 8001] | no | 0.9996 | 0.8746 | 0 |
| satfold-lanes4-idx | 7378ns | -646.4ns (-8.2%) | [-683, -525]ns | [7338, 7444] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 7998ns | no significant difference | [-55, +110]ns | [7912, 8055] | no | 1.0000 | 1.0000 | 0 |
| satfold-neon | 7991ns | no significant difference | [-2, +169]ns | [7908, 8298] | no | 0.5728 | 0.4296 | 0 |
| satfold-neon8 | 7946ns | no significant difference | [-33, +29]ns | [7886, 8045] | no | 0.5364 | 0.2682 | 0 |
| satfold-nolaw | 8075ns | no significant difference | [-1, +183]ns | [7970, 8174] | no | 0.5364 | 0.2682 | 0 |
| satfold-seq | 8003ns | no significant difference | [-67, +60]ns | [7894, 8039] | no | 0.5728 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 8006ns | -0.2% | +1.8% | -6.3% | +0.7% | -1.6% | -0.6% | -1.3% | -1.5% |
| 2 | 8033ns | -1.0% | -1.1% | -9.2% | +1.7% | -1.8% | -1.2% | -1.6% | -1.9% |
| 3 | 7999ns | -2.2% | -1.0% | -9.1% | +5.9% | -1.5% | +1.3% | -0.4% | -1.5% |
| 4 | 7978ns | -2.1% | -1.8% | -9.7% | +2.1% | -1.1% | -0.5% | -0.1% | -1.1% |
| 5 | 7907ns | +0.2% | -0.9% | -6.1% | +2.3% | +0.3% | -0.4% | +1.4% | -0.2% |
| 6 | 7915ns | -1.1% | -0.9% | -6.3% | -0.1% | +0.0% | -0.5% | +3.9% | -0.3% |
| 7 | 7893ns | -0.7% | -1.0% | -3.4% | +0.3% | -0.0% | +4.1% | +2.3% | +0.0% |
| 8 | 7989ns | +0.2% | -0.1% | -7.2% | -1.4% | -1.0% | +1.9% | +2.0% | -1.3% |
| 9 | 8050ns | -2.8% | -0.6% | -8.5% | -2.2% | -2.0% | +0.8% | -0.4% | -1.8% |
| 10 | 8056ns | -2.9% | +12.6% | -7.5% | -1.2% | -2.1% | +0.5% | +3.1% | -0.5% |
| 11 | 7882ns | +6.1% | -0.6% | -3.7% | +7.2% | +9.6% | -0.1% | +2.0% | +5.2% |
| 12 | 7889ns | +2.0% | +0.5% | -1.2% | +4.6% | +5.2% | -0.3% | +2.4% | +5.1% |
| 13 | 7892ns | +4.5% | +0.1% | -1.9% | +4.7% | +5.1% | -0.2% | +3.5% | +11.8% |
| 14 | 7891ns | +0.7% | -0.4% | -3.9% | +6.1% | +5.2% | -0.1% | +2.3% | +3.9% |
| 15 | 7974ns | -1.9% | -2.0% | -6.6% | +3.5% | +4.1% | +0.6% | +2.4% | +16.5% |
| 16 | 7990ns | +1.5% | -0.6% | -8.2% | +4.1% | +3.9% | +4.1% | +1.1% | +2.4% |
| 17 | 8340ns | -3.7% | -6.4% | -10.6% | -0.9% | +2.5% | -5.0% | -1.9% | -2.1% |
| 18 | 8179ns | -2.3% | -3.1% | -9.0% | -1.4% | +0.4% | -1.8% | -2.8% | -1.7% |
| 19 | 7912ns | -1.0% | -1.1% | -5.5% | +1.2% | +1.6% | -0.7% | +4.9% | +1.6% |
| 20 | 8049ns | -2.8% | -3.0% | -8.5% | +0.0% | +0.5% | -2.2% | -0.0% | -1.3% |
| 21 | 8116ns | -3.7% | -3.5% | -10.9% | -3.4% | -2.6% | -1.7% | -2.3% | -2.1% |
| 22 | 7954ns | -1.7% | +0.0% | -9.2% | -1.4% | -0.3% | -0.7% | -0.9% | +2.8% |
| 23 | 7920ns | -1.4% | +1.2% | -8.9% | -0.7% | -0.3% | -0.3% | -0.5% | -0.5% |
| 24 | 7889ns | -1.0% | +0.2% | -8.6% | -0.2% | +0.1% | -0.2% | -0.0% | -0.0% |
| 25 | 7898ns | -0.8% | -0.2% | -8.1% | -0.7% | -0.0% | -0.4% | -0.2% | -0.1% |
| 26 | 7893ns | -0.2% | +0.3% | -7.7% | -0.7% | +0.2% | -0.4% | -0.0% | +0.0% |
| 27 | 7896ns | -0.7% | +0.4% | -8.6% | -0.1% | -0.0% | -0.4% | +0.3% | -0.0% |
| 28 | 7890ns | -0.5% | +1.1% | -8.3% | -0.9% | +0.6% | -0.3% | -0.0% | +0.2% |
| 29 | 7908ns | -0.8% | +2.0% | -8.4% | -0.6% | +0.3% | -0.3% | -0.1% | -0.3% |
| 30 | 7894ns | -1.0% | +1.9% | -8.7% | -0.7% | -0.0% | -0.4% | +0.9% | -0.1% |
| 31 | 7884ns | +12.5% | +4.7% | -4.2% | +0.8% | +11.7% | +1.7% | +3.9% | +1.5% |
| 32 | 7908ns | +12.2% | +4.5% | -5.2% | +1.2% | +14.7% | +2.1% | +4.4% | +1.3% |
| 33 | 7880ns | +12.4% | +5.0% | -3.9% | +1.9% | +13.4% | +1.8% | +5.4% | +1.6% |
| 34 | 7891ns | +12.4% | +4.9% | -6.6% | +1.6% | +13.4% | +2.1% | +5.7% | +1.6% |
| 35 | 7887ns | +6.1% | +5.0% | -6.3% | +3.9% | +13.4% | +2.1% | +5.7% | +1.7% |
| 36 | 7893ns | +6.2% | +4.8% | -7.0% | +1.8% | +13.4% | +7.4% | +5.8% | +4.2% |
| 37 | 8173ns | -1.1% | +0.3% | -10.1% | -4.2% | +7.6% | +1.7% | +1.9% | -1.1% |
| 38 | 8328ns | -1.1% | -3.6% | -11.0% | -4.5% | -0.4% | -0.1% | +0.7% | -2.8% |
| 39 | 8302ns | -3.3% | -3.7% | -11.3% | -3.7% | -0.0% | +0.2% | +4.3% | -3.2% |
| 40 | 8304ns | -5.4% | -3.9% | -10.4% | -4.7% | -0.1% | +0.0% | +0.6% | -3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.634 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.712 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.279 | moderate+ |
| satfold-lanes4-idx | 0.669 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.688 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.810 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.603 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.692 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.385 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 27/40, lost 13/40
- **satfold-lanes16-constl**: won 21/40, lost 17/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 18/40, lost 19/40
- **satfold-neon**: won 11/40, lost 21/40
- **satfold-neon8**: won 23/40, lost 15/40
- **satfold-nolaw**: won 10/40, lost 24/40
- **satfold-seq**: won 20/40, lost 15/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.1ns | 7988.3ns | 0.0% |  |
| satfold-lanes16 | 2.8ns | 8044.2ns | 0.0% |  |
| satfold-lanes16-constl | 2.8ns | 8010.2ns | 0.0% |  |
| satfold-lanes4-idx | 3.0ns | 7395.6ns | 0.0% |  |
| satfold-lanes64 | 2.6ns | 8029.9ns | 0.0% |  |
| satfold-neon | 2.3ns | 8210.7ns | 0.0% |  |
| satfold-neon8 | 2.1ns | 8014.2ns | 0.0% |  |
| satfold-nolaw | 2.0ns | 8104.1ns | 0.0% |  |
| satfold-seq | 2.3ns | 8051.7ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 7886.5-8224.7 ns)
   7886.5 |########################################
   7903.4 |##################
   7920.3 |
   7937.2 |
   7954.1 |###
   7971.0 |######
   7987.9 |#########
   8004.8 |###
   8021.8 |###
   8038.7 |######
   8055.6 |###
   8072.5 |
   8089.4 |
   8106.3 |###
   8123.2 |
   8140.1 |
   8157.0 |###
   8174.0 |###
   8190.9 |
   8207.8 |
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 7814.4-8603.4 ns)
   7814.4 |########################################
   7853.9 |#####
   7893.3 |##
   7932.8 |#####
   7972.2 |########
   8011.7 |########
   8051.1 |##
   8090.6 |##
   8130.0 |
   8169.5 |
   8208.9 |#####
   8248.4 |
   8287.8 |
   8327.3 |#####
   8366.7 |##
   8406.2 |
   8445.6 |
   8485.1 |
   8524.5 |
   8564.0 |
  (3 below, 4 above range)

satfold-lanes16-constl (n=40, range 7822.6-8362.7 ns)
   7822.6 |########################################
   7849.6 |######
   7876.6 |#############
   7903.6 |########################################
   7930.6 |####################
   7957.6 |####################
   7984.6 |#############
   8011.6 |#############
   8038.6 |#############
   8065.6 |
   8092.6 |
   8119.6 |
   8146.6 |######
   8173.6 |
   8200.6 |######
   8227.6 |######
   8254.6 |##########################
   8281.6 |######
   8308.7 |
   8335.7 |
  (4 below, 1 above range)

satfold-lanes4-idx (n=40, range 7218.3-7621.0 ns)
   7218.3 |################################
   7238.4 |########
   7258.6 |################
   7278.7 |################
   7298.9 |
   7319.0 |########
   7339.1 |################
   7359.3 |################################
   7379.4 |########
   7399.5 |########################
   7419.7 |########
   7439.8 |########################################
   7459.9 |########
   7480.1 |################
   7500.2 |
   7520.3 |
   7540.5 |########
   7560.6 |########
   7580.7 |################
   7600.9 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 7839.1-8331.3 ns)
   7839.1 |########################################
   7863.7 |####################
   7888.3 |#############
   7912.9 |#############
   7937.5 |####################
   7962.2 |
   7986.8 |####################
   8011.4 |#############
   8036.0 |####################
   8060.6 |######
   8085.2 |######
   8109.8 |
   8134.4 |######
   8159.0 |######
   8183.6 |######
   8208.2 |
   8232.9 |#############
   8257.5 |#############
   8282.1 |
   8306.7 |######
  (3 below, 3 above range)

satfold-neon (n=40, range 7888.1-8885.8 ns)
   7888.1 |########################################
   7938.0 |##
   7987.9 |
   8037.8 |##
   8087.6 |##
   8137.5 |
   8187.4 |##
   8237.3 |
   8287.2 |##################
   8337.1 |
   8387.0 |
   8436.8 |
   8486.7 |
   8536.6 |##
   8586.5 |
   8636.4 |##
   8686.3 |
   8736.1 |
   8786.0 |####
   8835.9 |
  (2 below, 5 above range)

satfold-neon8 (n=40, range 7863.9-8299.9 ns)
   7863.9 |########################################
   7885.7 |############
   7907.5 |####
   7929.3 |########
   7951.1 |####
   7972.9 |####
   7994.7 |
   8016.5 |################
   8038.3 |########
   8060.1 |####
   8081.9 |####
   8103.7 |########
   8125.5 |####
   8147.3 |
   8169.1 |
   8190.9 |
   8212.7 |####
   8234.5 |
   8256.3 |
   8278.1 |
  (4 below, 6 above range)

satfold-nolaw (n=40, range 7891.0-8383.0 ns)
   7891.0 |########################################
   7915.6 |####################
   7940.2 |####################
   7964.8 |####################
   7989.4 |
   8014.0 |####################
   8038.6 |####################
   8063.2 |########################################
   8087.8 |
   8112.4 |
   8137.0 |##########
   8161.6 |##############################
   8186.2 |##########
   8210.8 |##########
   8235.4 |##########
   8260.0 |
   8284.6 |##############################
   8309.2 |##########
   8333.8 |########################################
   8358.4 |
  (5 below, 2 above range)

satfold-seq (n=40, range 7884.3-8435.0 ns)
   7884.3 |########################################
   7911.8 |
   7939.3 |#####
   7966.9 |
   7994.4 |#################
   8021.9 |###########
   8049.5 |
   8077.0 |#####
   8104.6 |
   8132.1 |
   8159.6 |########
   8187.2 |##
   8214.7 |##
   8242.2 |
   8269.8 |#####
   8297.3 |
   8324.9 |
   8352.4 |
   8379.9 |
   8407.5 |
  (3 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.69 (measurement drift or warm-up artifact)
