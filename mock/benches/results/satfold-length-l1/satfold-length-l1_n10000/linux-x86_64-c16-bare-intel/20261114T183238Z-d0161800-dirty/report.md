# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (20.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 2.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes64 beats baseline by 91% (significant)

satfold-lanes64 is -18.83 us (91%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 10.0x slower than the field

satfold-iterfold (20.70 us) is 10.0x the fastest (2.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon, satfold-lanes64) are a dead heat (<1%)

satfold-neon (2.08 us) and satfold-lanes64 (2.09 us) differ by 0.40%, inside the noise, even though the wider field spreads 896.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-neon8, satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl} vs {satfold-nolaw, satfold-seq, satfold-iterfold} (236% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-neon8, satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl} and a slow tier {satfold-nolaw, satfold-seq, satfold-iterfold} with a 236% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.0x the fastest

Fastest satfold-neon (2.08 us) to slowest satfold-iterfold (20.70 us): 10.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.1x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.54 us but its worst 20% at 7.94 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 2078.0 ns median (-90.0% vs baseline)
- 8 variants significantly faster than baseline
- Spread: 9.96x (fastest 2078.0 ns, slowest 20703.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 21937ns | 20766ns | 20758ns | 21183ns | 25379ns | base |
| satfold-lanes16 | 3620ns | 3114ns | 2818ns | 3090ns | 6012ns | -83.50% |
| satfold-lanes16-constl | 3640ns | 3325ns | 2609ns | 3384ns | 5442ns | -83.41% |
| satfold-lanes4-idx | 4296ns | 2732ns | 2612ns | 3552ns | 8209ns | -80.42% |
| satfold-lanes64 | 2554ns | 2305ns | 1727ns | 2327ns | 4062ns | -88.36% |
| satfold-neon | 2343ns | 2306ns | 1959ns | 2296ns | 2869ns | -89.32% |
| satfold-neon8 | 2298ns | 2322ns | 1808ns | 2284ns | 2828ns | -89.53% |
| satfold-nolaw | 10115ns | 10585ns | 8640ns | 10426ns | 10657ns | -53.89% |
| satfold-seq | 21661ns | 20467ns | 20380ns | 20930ns | 25134ns | -1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 21867ns | 20698ns | 25298ns | base | 1.498 |
| satfold-lanes16 | 3413ns | 2676ns | 5633ns | -84.39% | 9.600 |
| satfold-lanes16-constl | 3422ns | 2480ns | 5106ns | -84.35% | 9.577 |
| satfold-lanes4-idx | 4171ns | 2541ns | 7943ns | -80.92% | 7.855 |
| satfold-lanes64 | 2307ns | 1571ns | 3632ns | -89.45% | 14.205 |
| satfold-neon | 2112ns | 1778ns | 2565ns | -90.34% | 15.517 |
| satfold-neon8 | 2072ns | 1641ns | 2534ns | -90.52% | 15.813 |
| satfold-nolaw | 10038ns | 8575ns | 10555ns | -54.10% | 3.264 |
| satfold-seq | 21589ns | 20321ns | 25059ns | -1.27% | 1.518 |

## Performance model

- Peak throughput: **20.864 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.583 | 7.6% |
| satfold-lanes16 | 11.168 | 53.5% |
| satfold-lanes16-constl | 10.474 | 50.2% |
| satfold-lanes4-idx | 12.329 | 59.1% |
| satfold-lanes64 | 15.706 | 75.3% |
| satfold-neon | 15.769 | 75.6% |
| satfold-neon8 | 15.605 | 74.8% |
| satfold-nolaw | 3.117 | 14.9% |
| satfold-seq | 1.607 | 7.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 21937ns | 21937ns | base |
| satfold-lanes16 | 3620ns | 3620ns | -83.50% |
| satfold-lanes16-constl | 3640ns | 3640ns | -83.41% |
| satfold-lanes4-idx | 4296ns | 4296ns | -80.42% |
| satfold-lanes64 | 2554ns | 2554ns | -88.36% |
| satfold-neon | 2343ns | 2343ns | -89.32% |
| satfold-neon8 | 2298ns | 2298ns | -89.53% |
| satfold-nolaw | 10115ns | 10115ns | -53.89% |
| satfold-seq | 21661ns | 21661ns | -1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 20703ns | base | --- | [20702, 20729] | --- | --- | --- | --- |
| satfold-lanes16 | 2934ns | -18020.4ns (-87.0%) | [-18022, -17942]ns | [2695, 3056] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3129ns | -17934.4ns (-86.6%) | [-18014, -17413]ns | [2943, 3449] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2658ns | -18050.2ns (-87.2%) | [-18161, -15467]ns | [2556, 5251] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2086ns | -18831.7ns (-91.0%) | [-19266, -18596]ns | [1964, 2242] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2078ns | -18625.3ns (-90.0%) | [-18889, -18533]ns | [1923, 2168] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 2100ns | -18756.7ns (-90.6%) | [-19124, -18574]ns | [1923, 2160] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 10512ns | -11302.8ns (-54.6%) | [-12127, -10212]ns | [10491, 10523] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 20388ns | -371.1ns (-1.8%) | [-383, -248]ns | [20341, 20540] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20702ns | -87.1% | -81.6% | -74.6% | -88.5% | -88.7% | -88.8% | -49.1% | -1.7% |
| 1 | 2 | 20702ns | -87.0% | -87.0% | -87.2% | -88.7% | -88.9% | -89.1% | -58.6% | -1.8% |
| 1 | 3 | 25296ns | -87.9% | -88.0% | -89.9% | -91.8% | -92.6% | -92.3% | -58.4% | -19.4% |
| 1 | 4 | 20706ns | -85.7% | -85.4% | -87.7% | -90.8% | -90.7% | -92.0% | -49.3% | +20.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.416 | moderate- |
| satfold-lanes16 | 0.138 | ok |
| satfold-lanes16-constl | -0.360 | moderate- |
| satfold-lanes4-idx | -0.054 | ok |
| satfold-lanes64 | 0.285 | moderate+ |
| satfold-neon | 0.237 | moderate+ |
| satfold-neon8 | 0.267 | moderate+ |
| satfold-nolaw | -0.429 | moderate- |
| satfold-seq | -0.074 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.6ns | 21867.3ns | 0.1% |  |
| satfold-lanes16 | 64.8ns | 3413.2ns | 1.9% |  |
| satfold-lanes16-constl | 73.6ns | 3421.7ns | 2.2% |  |
| satfold-lanes4-idx | 35.6ns | 4171.5ns | 0.9% |  |
| satfold-lanes64 | 79.7ns | 2306.8ns | 3.5% |  |
| satfold-neon | 79.4ns | 2111.7ns | 3.8% |  |
| satfold-neon8 | 79.2ns | 2072.2ns | 3.8% |  |
| satfold-nolaw | 23.2ns | 10037.8ns | 0.2% |  |
| satfold-seq | 20.8ns | 21588.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 20697.8-25297.8 ns)
  20697.8 |########################################
  20927.8 |###
  21157.8 |
  21387.8 |
  21617.8 |
  21847.8 |
  22077.8 |
  22307.8 |
  22537.8 |
  22767.8 |
  22997.8 |
  23227.8 |
  23457.8 |
  23687.8 |
  23917.8 |
  24147.8 |
  24377.8 |
  24607.8 |
  24837.8 |
  25067.8 |##########
  (2 below, 3 above range)

satfold-lanes16 (n=40, range 2676.1-5633.1 ns)
   2676.1 |########################################
   2824.0 |#####################
   2971.8 |##########
   3119.7 |#####
   3267.5 |#####
   3415.4 |#####
   3563.2 |##
   3711.1 |
   3858.9 |
   4006.8 |
   4154.6 |
   4302.5 |##
   4450.3 |
   4598.2 |
   4746.0 |
   4893.9 |
   5041.7 |
   5189.6 |
   5337.4 |
   5485.3 |
  (1 below, 4 above range)

satfold-lanes16-constl (n=40, range 2480.2-5105.6 ns)
   2480.2 |
   2611.4 |########################################
   2742.7 |############################
   2874.0 |############################
   3005.2 |###########
   3136.5 |###########
   3267.8 |#################
   3399.1 |###########
   3530.3 |######################
   3661.6 |#####
   3792.9 |
   3924.1 |###########
   4055.4 |###########
   4186.7 |#####
   4317.9 |
   4449.2 |
   4580.5 |
   4711.8 |
   4843.0 |
   4974.3 |
  (1 below, 3 above range)

satfold-lanes4-idx (n=40, range 2541.2-7943.1 ns)
   2541.2 |########################################
   2811.3 |##
   3081.4 |
   3351.5 |####
   3621.6 |
   3891.7 |
   4161.8 |
   4431.9 |
   4702.0 |
   4972.1 |
   5242.2 |##################
   5512.3 |
   5782.4 |
   6052.5 |
   6322.6 |##
   6592.7 |
   6862.7 |##
   7132.8 |
   7402.9 |
   7673.0 |
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 1570.5-3632.5 ns)
   1570.5 |
   1673.6 |#############
   1776.7 |######
   1879.8 |########################################
   1982.9 |########################################
   2086.0 |#################################
   2189.1 |####################
   2292.2 |#############
   2395.3 |######
   2498.4 |######
   2601.5 |#############
   2704.6 |######
   2807.7 |
   2910.8 |
   3013.9 |
   3117.0 |
   3220.1 |######
   3323.2 |
   3426.3 |
   3529.4 |
  (5 below, 4 above range)

satfold-neon (n=40, range 1777.5-2564.6 ns)
   1777.5 |###############
   1816.9 |
   1856.2 |
   1895.6 |########################################
   1935.0 |#####
   1974.3 |#####
   2013.7 |
   2053.0 |##############################
   2092.3 |#####
   2131.7 |#####
   2171.1 |#####
   2210.4 |
   2249.8 |
   2289.1 |#########################
   2328.4 |#####
   2367.8 |#####
   2407.2 |
   2446.5 |
   2485.9 |
   2525.2 |###############
  (5 below, 3 above range)

satfold-neon8 (n=40, range 1641.0-2534.0 ns)
   1641.0 |###############
   1685.6 |#####
   1730.3 |
   1774.9 |###############
   1819.6 |##########
   1864.2 |
   1908.9 |###############
   1953.5 |##########
   1998.2 |
   2042.8 |
   2087.5 |########################################
   2132.1 |##########
   2176.8 |#####
   2221.4 |#####
   2266.1 |###############
   2310.7 |##########
   2355.4 |#####
   2400.0 |
   2444.7 |
   2489.3 |
  (3 below, 5 above range)

satfold-nolaw (n=40, range 8575.5-10554.6 ns)
   8575.5 |#####
   8674.4 |
   8773.4 |
   8872.3 |
   8971.3 |
   9070.2 |
   9169.2 |
   9268.2 |
   9367.1 |
   9466.1 |
   9565.0 |
   9664.0 |
   9762.9 |
   9861.9 |
   9960.8 |
  10059.8 |
  10158.7 |
  10257.7 |
  10356.7 |
  10455.6 |########################################
  (6 below, 1 above range)

satfold-seq (n=40, range 20320.9-25059.0 ns)
  20320.9 |########################################
  20557.8 |#####
  20794.7 |
  21031.6 |
  21268.5 |
  21505.4 |
  21742.3 |
  21979.3 |#
  22216.2 |
  22453.1 |
  22690.0 |
  22926.9 |
  23163.8 |
  23400.7 |
  23637.6 |
  23874.5 |
  24111.4 |
  24348.3 |
  24585.2 |
  24822.1 |##########
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=42.5% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=33.5% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=56.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **satfold-lanes64**: CV=35.0% (high variance, measurements may be unstable)
