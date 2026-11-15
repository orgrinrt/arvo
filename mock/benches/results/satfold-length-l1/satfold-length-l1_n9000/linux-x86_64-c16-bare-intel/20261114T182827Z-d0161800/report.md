# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (23.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes16-constl at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes16-constl beats baseline by 91% (significant)

satfold-lanes16-constl is -21.23 us (91%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 9.2x slower than the field

satfold-iterfold (23.21 us) is 9.2x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.59)

satfold-neon's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.2x the fastest

Fastest satfold-lanes16-constl (2.51 us) to slowest satfold-iterfold (23.21 us): 9.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-constl is inconsistent: worst-20% is 4.4x its best-20%

satfold-lanes16-constl's best 20% of batches run at 1.48 us but its worst 20% at 6.44 us (4.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16-constl** at 2509.2 ns median (-89.2% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 9.25x (fastest 2509.2 ns, slowest 23208.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 22588ns | 23282ns | 19054ns | 22871ns | 25271ns | base |
| satfold-lanes16 | 4029ns | 4413ns | 1570ns | 4109ns | 6246ns | -82.16% |
| satfold-lanes16-constl | 3565ns | 2605ns | 1535ns | 3181ns | 6746ns | -84.22% |
| satfold-lanes4-idx | 3908ns | 2654ns | 2286ns | 2989ns | 8291ns | -82.70% |
| satfold-lanes64 | 3366ns | 3100ns | 2733ns | 3103ns | 4787ns | -85.10% |
| satfold-neon | 3334ns | 3306ns | 2846ns | 3291ns | 3950ns | -85.24% |
| satfold-neon8 | 3467ns | 3124ns | 2151ns | 3237ns | 5469ns | -84.65% |
| satfold-nolaw | 9124ns | 8886ns | 7280ns | 8962ns | 11454ns | -59.61% |
| satfold-seq | 23301ns | 23138ns | 22514ns | 23071ns | 24780ns | +3.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 22506ns | 18990ns | 25147ns | base | 1.456 |
| satfold-lanes16 | 3870ns | 1513ns | 5970ns | -82.80% | 8.466 |
| satfold-lanes16-constl | 3423ns | 1479ns | 6443ns | -84.79% | 9.572 |
| satfold-lanes4-idx | 3787ns | 2224ns | 8016ns | -83.17% | 8.654 |
| satfold-lanes64 | 3165ns | 2574ns | 4480ns | -85.94% | 10.353 |
| satfold-neon | 3123ns | 2680ns | 3655ns | -86.12% | 10.491 |
| satfold-neon8 | 3262ns | 2034ns | 5153ns | -85.50% | 10.044 |
| satfold-nolaw | 9031ns | 7221ns | 11306ns | -59.87% | 3.628 |
| satfold-seq | 23221ns | 22435ns | 24686ns | +3.18% | 1.411 |

## Performance model

- Peak throughput: **22.150 Gops/s** (satfold-lanes16-constl; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.412 | 6.4% |
| satfold-lanes16 | 7.704 | 34.8% |
| satfold-lanes16-constl | 13.059 | 59.0% |
| satfold-lanes4-idx | 12.690 | 57.3% |
| satfold-lanes64 | 11.209 | 50.6% |
| satfold-neon | 10.529 | 47.5% |
| satfold-neon8 | 11.136 | 50.3% |
| satfold-nolaw | 3.726 | 16.8% |
| satfold-seq | 1.422 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 22588ns | 22588ns | base |
| satfold-lanes16 | 4029ns | 4029ns | -82.16% |
| satfold-lanes16-constl | 3565ns | 3565ns | -84.22% |
| satfold-lanes4-idx | 3908ns | 3908ns | -82.70% |
| satfold-lanes64 | 3366ns | 3366ns | -85.10% |
| satfold-neon | 3334ns | 3334ns | -85.24% |
| satfold-neon8 | 3467ns | 3467ns | -84.65% |
| satfold-nolaw | 9124ns | 9124ns | -59.61% |
| satfold-seq | 23301ns | 23301ns | +3.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 23208ns | base | --- | [23009, 23210] | --- | --- | --- | --- |
| satfold-lanes16 | 4253ns | -18218.8ns (-78.5%) | [-21540, -17202]ns | [3589, 4830] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2509ns | -21231.0ns (-91.5%) | [-21369, -18057]ns | [1667, 4636] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2582ns | -20628.9ns (-88.9%) | [-20633, -19346]ns | [2578, 2596] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2923ns | -20144.3ns (-86.8%) | [-20350, -19806]ns | [2782, 3022] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 3112ns | -19798.4ns (-85.3%) | [-20100, -19668]ns | [2941, 3238] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 2943ns | -20246.2ns (-87.2%) | [-20545, -19253]ns | [2763, 3194] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8794ns | -14438.0ns (-62.2%) | [-14459, -14186]ns | [8767, 8831] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 23051ns | no significant difference | [-432, +45]ns | [22769, 23245] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 23210ns | -78.8% | -92.8% | -88.9% | -87.5% | -87.0% | -87.8% | -68.9% | -1.8% |
| 1 | 2 | 23212ns | -85.6% | -83.2% | -88.9% | -88.5% | -86.6% | -88.5% | -62.2% | +0.2% |
| 1 | 3 | 18991ns | -78.7% | -76.3% | -86.3% | -84.4% | -85.6% | -83.2% | -40.5% | +22.4% |
| 1 | 4 | 23036ns | -83.5% | -92.7% | -90.8% | -87.1% | -85.4% | -84.1% | -61.7% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.393 | moderate- |
| satfold-lanes16 | -0.474 | moderate- |
| satfold-lanes16-constl | -0.257 | moderate- |
| satfold-lanes4-idx | -0.127 | ok |
| satfold-lanes64 | -0.206 | moderate- |
| satfold-neon | -0.592 | HIGH- (thermal bounce) |
| satfold-neon8 | 0.211 | moderate+ |
| satfold-nolaw | -0.068 | ok |
| satfold-seq | -0.213 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 24.5ns | 22505.6ns | 0.1% |  |
| satfold-lanes16 | 56.2ns | 3870.4ns | 1.5% |  |
| satfold-lanes16-constl | 48.5ns | 3423.5ns | 1.4% |  |
| satfold-lanes4-idx | 36.4ns | 3786.6ns | 1.0% |  |
| satfold-lanes64 | 66.9ns | 3165.2ns | 2.1% |  |
| satfold-neon | 69.2ns | 3123.4ns | 2.2% |  |
| satfold-neon8 | 66.3ns | 3262.5ns | 2.0% |  |
| satfold-nolaw | 27.3ns | 9031.3ns | 0.3% |  |
| satfold-seq | 24.5ns | 23220.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 18990.2-25146.9 ns)
  18990.2 |############
  19298.0 |
  19605.9 |
  19913.7 |
  20221.5 |
  20529.4 |
  20837.2 |
  21145.0 |
  21452.9 |
  21760.7 |
  22068.6 |
  22376.4 |
  22684.2 |######
  22992.1 |########################################
  23299.9 |
  23607.7 |
  23915.6 |
  24223.4 |
  24531.2 |
  24839.1 |
  (3 below, 3 above range)

satfold-lanes16 (n=40, range 1513.2-5969.6 ns)
   1513.2 |########################################
   1736.0 |
   1958.8 |
   2181.6 |
   2404.5 |
   2627.3 |
   2850.1 |#####
   3072.9 |
   3295.7 |
   3518.6 |##########
   3741.4 |##########
   3964.2 |###############
   4187.0 |#####
   4409.8 |#####
   4632.6 |#########################
   4855.5 |###############
   5078.3 |#####
   5301.1 |###############
   5523.9 |
   5746.7 |
  (4 below, 6 above range)

satfold-lanes16-constl (n=40, range 1479.4-6442.9 ns)
   1479.4 |########################################
   1727.5 |##
   1975.7 |
   2223.9 |
   2472.1 |
   2720.2 |
   2968.4 |##
   3216.6 |
   3464.8 |##
   3712.9 |
   3961.1 |##
   4209.3 |#####
   4457.5 |########
   4705.6 |##
   4953.8 |########
   5202.0 |##
   5450.2 |#####
   5698.3 |
   5946.5 |
   6194.7 |##
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 2223.8-8015.5 ns)
   2223.8 |
   2513.3 |########################################
   2802.9 |
   3092.5 |#
   3382.1 |
   3671.7 |#
   3961.3 |
   4250.9 |#
   4540.5 |
   4830.0 |
   5119.6 |
   5409.2 |
   5698.8 |
   5988.4 |
   6278.0 |#
   6567.6 |
   6857.2 |
   7146.7 |#####
   7436.3 |
   7725.9 |#
  (6 below, 4 above range)

satfold-lanes64 (n=40, range 2574.3-4479.8 ns)
   2574.3 |#################
   2669.6 |############################
   2764.9 |########################################
   2860.1 |#################
   2955.4 |############################
   3050.7 |#####
   3145.9 |######################
   3241.2 |###########
   3336.5 |###########
   3431.8 |#####
   3527.0 |
   3622.3 |
   3717.6 |
   3812.8 |
   3908.1 |
   4003.4 |
   4098.7 |
   4193.9 |
   4289.2 |
   4384.5 |
  (4 below, 3 above range)

satfold-neon (n=40, range 2680.1-3655.5 ns)
   2680.1 |#####
   2728.8 |##################################
   2777.6 |#####
   2826.4 |
   2875.1 |#####
   2923.9 |######################
   2972.7 |#################
   3021.4 |###########
   3070.2 |
   3119.0 |
   3167.8 |#################
   3216.5 |########################################
   3265.3 |#####
   3314.1 |
   3362.8 |
   3411.6 |###########
   3460.4 |#####
   3509.1 |#####
   3557.9 |#####
   3606.7 |###########
  (2 below, 2 above range)

satfold-neon8 (n=40, range 2034.0-5152.9 ns)
   2034.0 |
   2189.9 |
   2345.9 |
   2501.8 |########################################
   2657.8 |############################
   2813.7 |##################################
   2969.7 |#####
   3125.6 |############################
   3281.6 |
   3437.5 |#################
   3593.5 |###########
   3749.4 |###########
   3905.4 |
   4061.3 |
   4217.3 |#####
   4373.2 |###########
   4529.1 |
   4685.1 |
   4841.0 |
   4997.0 |
  (3 below, 3 above range)

satfold-nolaw (n=40, range 7220.7-11306.3 ns)
   7220.7 |##############
   7425.0 |
   7629.2 |
   7833.5 |
   8037.8 |
   8242.1 |
   8446.4 |
   8650.6 |########################################
   8854.9 |
   9059.2 |
   9263.5 |
   9467.8 |
   9672.0 |
   9876.3 |
  10080.6 |
  10284.9 |
  10489.1 |
  10693.4 |
  10897.7 |
  11102.0 |##########
  (3 below, 5 above range)

satfold-seq (n=40, range 22435.2-24686.1 ns)
  22435.2 |###############
  22547.7 |
  22660.3 |##########
  22772.8 |############
  22885.4 |##
  22997.9 |
  23110.5 |##
  23223.0 |########################################
  23335.5 |
  23448.1 |
  23560.6 |
  23673.2 |
  23785.7 |
  23898.3 |
  24010.8 |
  24123.4 |
  24235.9 |
  24348.5 |
  24461.0 |
  24573.6 |
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=43.6% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=58.9% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=59.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes64**: CV=31.0% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=38.4% (high variance, measurements may be unstable)
