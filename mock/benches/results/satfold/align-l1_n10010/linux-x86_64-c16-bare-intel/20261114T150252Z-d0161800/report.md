# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (25.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes64 at 2.10 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes64 beats baseline by 92% (significant)

satfold-lanes64 is -23.48 us (92%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 12.2x slower than the field

satfold-iterfold (25.58 us) is 12.2x the fastest (2.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-constl shows alternating (throttle bounce) (autocorr -0.68)

satfold-lanes16-constl's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.2x the fastest

Fastest satfold-lanes64 (2.10 us) to slowest satfold-iterfold (25.58 us): 12.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 3.40 us but its worst 20% at 10.32 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 2101.9 ns median (-91.8% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 12.17x (fastest 2101.9 ns, slowest 25580.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 25567ns | 25654ns | 20915ns | 25605ns | 30104ns | base |
| satfold-lanes16 | 3330ns | 3301ns | 2222ns | 3380ns | 4290ns | -86.97% |
| satfold-lanes16-constl | 3540ns | 3476ns | 2916ns | 3508ns | 4260ns | -86.15% |
| satfold-lanes4-idx | 5342ns | 4226ns | 3457ns | 4230ns | 10562ns | -79.11% |
| satfold-lanes64 | 2564ns | 2298ns | 1980ns | 2307ns | 3918ns | -89.97% |
| satfold-neon | 2572ns | 2508ns | 2244ns | 2533ns | 3016ns | -89.94% |
| satfold-nolaw | 10697ns | 10676ns | 10644ns | 10674ns | 10822ns | -58.16% |
| satfold-seq | 24249ns | 25395ns | 20772ns | 25013ns | 25436ns | -5.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 25473ns | 20853ns | 29953ns | base | 1.286 |
| satfold-lanes16 | 3135ns | 2093ns | 4023ns | -87.69% | 10.453 |
| satfold-lanes16-constl | 3334ns | 2757ns | 3991ns | -86.91% | 9.828 |
| satfold-lanes4-idx | 5238ns | 3398ns | 10315ns | -79.44% | 6.256 |
| satfold-lanes64 | 2326ns | 1807ns | 3558ns | -90.87% | 14.085 |
| satfold-neon | 2338ns | 2045ns | 2710ns | -90.82% | 14.018 |
| satfold-nolaw | 10621ns | 10568ns | 10735ns | -58.31% | 3.085 |
| satfold-seq | 24173ns | 20712ns | 25339ns | -5.10% | 1.356 |

## Performance model

- Peak throughput: **18.129 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.281 | 7.1% |
| satfold-lanes16 | 10.491 | 57.9% |
| satfold-lanes16-constl | 10.002 | 55.2% |
| satfold-lanes4-idx | 7.889 | 43.5% |
| satfold-lanes64 | 15.590 | 86.0% |
| satfold-neon | 14.288 | 78.8% |
| satfold-nolaw | 3.091 | 17.1% |
| satfold-seq | 1.294 | 7.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 25567ns | 25567ns | base |
| satfold-lanes16 | 3330ns | 3330ns | -86.97% |
| satfold-lanes16-constl | 3540ns | 3540ns | -86.15% |
| satfold-lanes4-idx | 5342ns | 5342ns | -79.11% |
| satfold-lanes64 | 2564ns | 2564ns | -89.97% |
| satfold-neon | 2572ns | 2572ns | -89.94% |
| satfold-nolaw | 10697ns | 10697ns | -58.16% |
| satfold-seq | 24249ns | 24249ns | -5.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 25580ns | base | --- | [25474, 25591] | --- | --- | --- | --- |
| satfold-lanes16 | 3124ns | -22415.3ns (-87.6%) | [-22845, -21635]ns | [2923, 3377] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3276ns | -21809.2ns (-85.3%) | [-22231, -21587]ns | [3052, 3563] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 4154ns | -21357.8ns (-83.5%) | [-21434, -18466]ns | [4151, 4164] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2102ns | -23483.8ns (-91.8%) | [-23631, -23341]ns | [1992, 2149] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2293ns | -23307.8ns (-91.1%) | [-23484, -23171]ns | [2208, 2416] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 10600ns | -14990.0ns (-58.6%) | [-15017, -14871]ns | [10596, 10605] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 25321ns | -255.1ns (-1.0%) | [-269, -149]ns | [25315, 25325] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 29952ns | -90.5% | -89.9% | -88.7% | -93.1% | -91.6% | -64.6% | -30.8% |
| 1 | 2 | 25583ns | -88.1% | -86.0% | -83.8% | -92.0% | -91.3% | -58.6% | -1.0% |
| 1 | 3 | 20895ns | -82.9% | -86.3% | -80.1% | -83.0% | -88.7% | -49.3% | +21.2% |
| 1 | 4 | 25580ns | -84.8% | -84.9% | -83.7% | -91.8% | -91.4% | -58.7% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.009 | ok |
| satfold-lanes16 | 0.298 | moderate+ |
| satfold-lanes16-constl | -0.676 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.080 | ok |
| satfold-lanes64 | -0.402 | moderate- |
| satfold-neon | -0.447 | moderate- |
| satfold-nolaw | -0.142 | ok |
| satfold-seq | -0.084 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 27.8ns | 25473.2ns | 0.1% |  |
| satfold-lanes16 | 67.3ns | 3134.6ns | 2.1% |  |
| satfold-lanes16-constl | 73.2ns | 3334.0ns | 2.2% |  |
| satfold-lanes4-idx | 30.6ns | 5237.6ns | 0.6% |  |
| satfold-lanes64 | 76.6ns | 2326.4ns | 3.3% |  |
| satfold-neon | 81.3ns | 2337.6ns | 3.5% |  |
| satfold-nolaw | 24.0ns | 10620.7ns | 0.2% |  |
| satfold-seq | 23.1ns | 24173.3ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 20853.4-29953.4 ns)
  20853.4 |################
  21308.4 |
  21763.4 |
  22218.4 |
  22673.4 |
  23128.4 |
  23583.4 |
  24038.4 |
  24493.4 |
  24948.4 |#######
  25403.4 |########################################
  25858.4 |
  26313.4 |
  26768.4 |
  27223.4 |
  27678.4 |
  28133.4 |
  28588.4 |
  29043.4 |
  29498.4 |##############
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 2093.3-4023.2 ns)
   2093.3 |
   2189.8 |
   2286.3 |
   2382.8 |
   2479.3 |
   2575.8 |#################
   2672.3 |#####
   2768.8 |########################################
   2865.2 |
   2961.7 |######################
   3058.2 |#################
   3154.7 |#####
   3251.2 |######################
   3347.7 |#####
   3444.2 |#####
   3540.7 |###########
   3637.2 |#####
   3733.7 |#####
   3830.2 |
   3926.7 |##################################
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 2756.6-3991.2 ns)
   2756.6 |####################
   2818.3 |#############
   2880.1 |######
   2941.8 |
   3003.5 |########################################
   3065.3 |######
   3127.0 |
   3188.7 |######
   3250.5 |#################################
   3312.2 |#############
   3373.9 |######
   3435.6 |
   3497.4 |######
   3559.1 |####################
   3620.8 |######
   3682.6 |
   3744.3 |######
   3806.0 |
   3867.8 |#############
   3929.5 |########################################
  (3 below, 1 above range)

satfold-lanes4-idx (n=40, range 3398.0-10315.5 ns)
   3398.0 |###
   3743.8 |
   4089.7 |########################################
   4435.6 |
   4781.5 |#
   5127.4 |
   5473.2 |
   5819.1 |
   6165.0 |
   6510.9 |
   6856.7 |#
   7202.6 |
   7548.5 |
   7894.4 |
   8240.2 |
   8586.1 |#
   8932.0 |
   9277.9 |
   9623.7 |
   9969.6 |
  (6 below, 5 above range)

satfold-lanes64 (n=40, range 1807.5-3558.1 ns)
   1807.5 |#############
   1895.0 |########################################
   1982.5 |#################
   2070.0 |########################################
   2157.6 |########
   2245.1 |#############
   2332.6 |####
   2420.2 |
   2507.7 |
   2595.2 |
   2682.8 |
   2770.3 |
   2857.8 |
   2945.3 |
   3032.9 |
   3120.4 |
   3207.9 |
   3295.5 |####
   3383.0 |
   3470.5 |####
  (2 below, 5 above range)

satfold-neon (n=40, range 2044.7-2710.3 ns)
   2044.7 |######
   2078.0 |#################################
   2111.3 |
   2144.5 |##########################
   2177.8 |######
   2211.1 |######
   2244.4 |######
   2277.7 |########################################
   2310.9 |
   2344.2 |####################
   2377.5 |######
   2410.8 |######
   2444.1 |####################
   2477.3 |######
   2510.6 |#############
   2543.9 |
   2577.2 |####################
   2610.5 |
   2643.8 |
   2677.0 |
  (3 below, 4 above range)

satfold-nolaw (n=40, range 10568.2-10735.3 ns)
  10568.2 |#######
  10576.6 |#######
  10584.9 |##############
  10593.3 |################################
  10601.7 |########################################
  10610.0 |#######
  10618.4 |##################
  10626.7 |
  10635.1 |
  10643.4 |
  10651.8 |
  10660.1 |
  10668.5 |
  10676.8 |
  10685.2 |
  10693.6 |
  10701.9 |
  10710.3 |
  10718.6 |
  10727.0 |
  (4 below, 1 above range)

satfold-seq (n=40, range 20712.4-25338.7 ns)
  20712.4 |########
  20943.8 |
  21175.1 |
  21406.4 |
  21637.7 |
  21869.0 |
  22100.3 |
  22331.7 |
  22563.0 |
  22794.3 |
  23025.6 |
  23256.9 |
  23488.2 |
  23719.5 |
  23950.9 |
  24182.2 |
  24413.5 |
  24644.8 |
  24876.1 |
  25107.4 |########################################
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=23.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=54.4% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **satfold-lanes64**: CV=28.9% (high variance, measurements may be unstable)
