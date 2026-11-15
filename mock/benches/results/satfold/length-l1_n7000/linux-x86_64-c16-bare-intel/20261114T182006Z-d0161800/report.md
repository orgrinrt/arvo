# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 beats baseline by 82% (significant)

satfold-lanes16 is -14.07 us (82%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 5.6x slower than the field

satfold-seq (17.20 us) is 5.6x the fastest (3.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-constl is fastest but the noisiest (CV 86.6%)

satfold-lanes16-constl wins on median (3.08 us) yet has the highest variance (CV 86.6%), while satfold-iterfold is the steadiest (CV 7.9%, 17.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (satfold-lanes16-constl, satfold-lanes16) are a dead heat (<1%)

satfold-lanes16-constl (3.08 us) and satfold-lanes16 (3.09 us) differ by 0.39%, inside the noise, even though the wider field spreads 459.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-nolaw shows alternating (throttle bounce) (autocorr -0.75)

satfold-nolaw's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx, satfold-neon, satfold-neon8, satfold-lanes64, satfold-nolaw} vs {satfold-iterfold, satfold-seq} (133% apart)

The field splits into a fast tier {satfold-lanes16-constl, satfold-lanes16, satfold-lanes4-idx, satfold-neon, satfold-neon8, satfold-lanes64, satfold-nolaw} and a slow tier {satfold-iterfold, satfold-seq} with a 133% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest satfold-lanes16-constl (3.08 us) to slowest satfold-seq (17.20 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-neon8 is inconsistent: worst-20% is 3.8x its best-20%

satfold-neon8's best 20% of batches run at 1.63 us but its worst 20% at 6.26 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16-constl** at 3076.4 ns median (-82.1% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 5.59x (fastest 3076.4 ns, slowest 17203.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 16509ns | 17281ns | 14140ns | 17023ns | 17337ns | base |
| satfold-lanes16 | 4741ns | 3151ns | 2579ns | 3973ns | 9209ns | -71.28% |
| satfold-lanes16-constl | 4696ns | 3140ns | 2570ns | 3861ns | 9326ns | -71.56% |
| satfold-lanes4-idx | 4380ns | 3404ns | 1977ns | 4133ns | 7524ns | -73.47% |
| satfold-lanes64 | 4372ns | 5089ns | 1900ns | 4358ns | 6886ns | -73.52% |
| satfold-neon | 4065ns | 3683ns | 1854ns | 3809ns | 7047ns | -75.37% |
| satfold-neon8 | 4020ns | 4522ns | 1684ns | 3956ns | 6546ns | -75.65% |
| satfold-nolaw | 6981ns | 7461ns | 6113ns | 6914ns | 8052ns | -57.71% |
| satfold-seq | 16503ns | 17280ns | 14128ns | 17022ns | 17324ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 16429ns | 14075ns | 17238ns | base | 1.995 |
| satfold-lanes16 | 4634ns | 2527ns | 8965ns | -71.79% | 7.071 |
| satfold-lanes16-constl | 4586ns | 2518ns | 9060ns | -72.09% | 7.146 |
| satfold-lanes4-idx | 4238ns | 1920ns | 7232ns | -74.20% | 7.731 |
| satfold-lanes64 | 4216ns | 1836ns | 6617ns | -74.34% | 7.772 |
| satfold-neon | 3918ns | 1793ns | 6776ns | -76.15% | 8.363 |
| satfold-neon8 | 3872ns | 1628ns | 6259ns | -76.43% | 8.462 |
| satfold-nolaw | 6905ns | 6054ns | 7936ns | -57.97% | 4.746 |
| satfold-seq | 16426ns | 14068ns | 17232ns | -0.02% | 1.995 |

## Performance model

- Peak throughput: **20.123 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.905 | 9.5% |
| satfold-lanes16 | 10.610 | 52.7% |
| satfold-lanes16-constl | 10.652 | 52.9% |
| satfold-lanes4-idx | 9.903 | 49.2% |
| satfold-lanes64 | 6.659 | 33.1% |
| satfold-neon | 9.203 | 45.7% |
| satfold-neon8 | 7.491 | 37.2% |
| satfold-nolaw | 4.435 | 22.0% |
| satfold-seq | 1.905 | 9.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 16509ns | 16509ns | base |
| satfold-lanes16 | 4741ns | 4741ns | -71.28% |
| satfold-lanes16-constl | 4696ns | 4696ns | -71.56% |
| satfold-lanes4-idx | 4380ns | 4380ns | -73.47% |
| satfold-lanes64 | 4372ns | 4372ns | -73.52% |
| satfold-neon | 4065ns | 4065ns | -75.37% |
| satfold-neon8 | 4020ns | 4020ns | -75.65% |
| satfold-nolaw | 6981ns | 6981ns | -57.71% |
| satfold-seq | 16503ns | 16503ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 17201ns | base | --- | [17184, 17216] | --- | --- | --- | --- |
| satfold-lanes16 | 3088ns | -14071.6ns (-81.8%) | [-14131, -11396]ns | [3082, 3157] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3076ns | -13035.2ns (-75.8%) | [-14143, -10988]ns | [3076, 3089] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 3309ns | -11897.8ns (-69.2%) | [-14914, -10712]ns | [2167, 6045] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 4921ns | -11644.9ns (-67.7%) | [-13274, -11010]ns | [2190, 5566] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 3560ns | -12160.3ns (-70.7%) | [-14406, -11536]ns | [1948, 5475] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 4374ns | -12448.6ns (-72.4%) | [-13560, -11683]ns | [1963, 5294] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7388ns | -9807.0ns (-57.0%) | [-9822, -9783]ns | [6071, 7406] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 17204ns | no significant difference | [-18, +15]ns | [17194, 17212] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17194ns | -83.4% | -84.3% | -88.0% | -71.2% | -71.9% | -71.6% | -64.8% | -18.2% |
| 1 | 2 | 17210ns | -82.1% | -82.1% | -85.0% | -87.3% | -83.8% | -75.2% | -57.0% | -0.0% |
| 1 | 3 | 14073ns | -65.3% | -78.1% | -65.8% | -65.0% | -77.1% | -65.8% | -56.9% | +22.3% |
| 1 | 4 | 17228ns | -82.1% | -85.3% | -72.0% | -68.8% | -88.6% | -88.2% | -57.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.424 | moderate- |
| satfold-lanes16 | -0.320 | moderate- |
| satfold-lanes16-constl | -0.234 | moderate- |
| satfold-lanes4-idx | 0.286 | moderate+ |
| satfold-lanes64 | -0.303 | moderate- |
| satfold-neon | -0.162 | ok |
| satfold-neon8 | -0.214 | moderate- |
| satfold-nolaw | -0.746 | HIGH- (thermal bounce) |
| satfold-seq | -0.081 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 22.7ns | 16428.8ns | 0.1% |  |
| satfold-lanes16 | 35.8ns | 4634.1ns | 0.8% |  |
| satfold-lanes16-constl | 35.5ns | 4585.7ns | 0.8% |  |
| satfold-lanes4-idx | 47.1ns | 4238.4ns | 1.1% |  |
| satfold-lanes64 | 51.4ns | 4216.0ns | 1.2% |  |
| satfold-neon | 47.7ns | 3918.4ns | 1.2% |  |
| satfold-neon8 | 47.0ns | 3872.4ns | 1.2% |  |
| satfold-nolaw | 22.4ns | 6904.9ns | 0.3% |  |
| satfold-seq | 23.1ns | 16426.2ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14074.9-17238.0 ns)
  14074.9 |######
  14233.0 |
  14391.2 |
  14549.3 |
  14707.5 |
  14865.6 |
  15023.8 |
  15181.9 |
  15340.1 |
  15498.3 |
  15656.4 |
  15814.6 |
  15972.7 |
  16130.9 |
  16289.0 |
  16447.2 |
  16605.3 |
  16763.5 |
  16921.6 |
  17079.8 |########################################
  (6 below, 4 above range)

satfold-lanes16 (n=40, range 2526.8-8965.3 ns)
   2526.8 |################
   2848.7 |########################################
   3170.7 |
   3492.6 |
   3814.5 |
   4136.4 |
   4458.4 |
   4780.3 |
   5102.2 |
   5424.1 |
   5746.1 |
   6068.0 |
   6389.9 |
   6711.8 |#######
   7033.8 |##
   7355.7 |####
   7677.6 |##
   7999.5 |####
   8321.5 |##
   8643.4 |
  (3 below, 3 above range)

satfold-lanes16-constl (n=40, range 2518.2-9060.0 ns)
   2518.2 |##########
   2845.3 |########################################
   3172.4 |
   3499.5 |
   3826.6 |
   4153.7 |
   4480.8 |
   4807.9 |
   5135.0 |##
   5462.0 |
   5789.1 |
   6116.2 |
   6443.3 |##
   6770.4 |
   7097.5 |#####
   7424.6 |##
   7751.7 |#######
   8078.8 |
   8405.8 |##
   8732.9 |##
  (7 below, 3 above range)

satfold-lanes4-idx (n=40, range 1920.3-7232.1 ns)
   1920.3 |########################################
   2185.9 |###
   2451.4 |
   2717.0 |
   2982.6 |###
   3248.2 |
   3513.8 |###
   3779.4 |###
   4045.0 |
   4310.6 |
   4576.2 |
   4841.8 |
   5107.4 |
   5373.0 |
   5638.6 |######
   5904.2 |############
   6169.8 |
   6435.3 |###############
   6700.9 |######
   6966.5 |######
  (5 below, 3 above range)

satfold-lanes64 (n=40, range 1836.3-6616.9 ns)
   1836.3 |########################################
   2075.3 |
   2314.3 |###
   2553.4 |
   2792.4 |###
   3031.4 |
   3270.5 |
   3509.5 |###
   3748.5 |
   3987.6 |###
   4226.6 |
   4465.6 |###
   4704.6 |###
   4943.7 |######
   5182.7 |#############
   5421.7 |
   5660.8 |#############
   5899.8 |##########
   6138.8 |######
   6377.9 |###
  (2 below, 4 above range)

satfold-neon (n=40, range 1792.6-6775.6 ns)
   1792.6 |########################################
   2041.7 |
   2290.9 |
   2540.0 |
   2789.2 |
   3038.3 |
   3287.5 |##
   3536.6 |##
   3785.8 |
   4034.9 |
   4284.1 |##
   4533.2 |##
   4782.4 |##
   5031.5 |##
   5280.7 |##
   5529.8 |#####
   5779.0 |##########
   6028.1 |
   6277.3 |#######
   6526.4 |
  (3 below, 5 above range)

satfold-neon8 (n=40, range 1628.4-6258.9 ns)
   1628.4 |
   1859.9 |########################################
   2091.5 |####
   2323.0 |
   2554.5 |
   2786.0 |
   3017.6 |####
   3249.1 |
   3480.6 |
   3712.1 |
   3943.7 |########
   4175.2 |
   4406.7 |####
   4638.2 |####
   4869.8 |########
   5101.3 |########
   5332.8 |######################
   5564.3 |####
   5795.9 |#################
   6027.4 |
  (7 below, 4 above range)

satfold-nolaw (n=40, range 6054.1-7935.7 ns)
   6054.1 |###############################
   6148.1 |
   6242.2 |
   6336.3 |
   6430.4 |
   6524.5 |
   6618.6 |
   6712.6 |
   6806.7 |
   6900.8 |
   6994.9 |
   7089.0 |
   7183.1 |
   7277.1 |
   7371.2 |########################################
   7465.3 |
   7559.4 |
   7653.5 |##
   7747.6 |
   7841.6 |
  (4 below, 3 above range)

satfold-seq (n=40, range 14067.8-17232.3 ns)
  14067.8 |########
  14226.0 |
  14384.2 |
  14542.5 |
  14700.7 |
  14858.9 |
  15017.1 |
  15175.4 |
  15333.6 |
  15491.8 |
  15650.0 |
  15808.3 |
  15966.5 |
  16124.7 |
  16283.0 |
  16441.2 |
  16599.4 |
  16757.6 |
  16915.9 |
  17074.1 |########################################
  (4 below, 1 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=56.9% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=58.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=53.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-lanes64**: CV=46.3% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-neon**: CV=53.1% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-neon8**: CV=48.9% (high variance, measurements may be unstable)
- **satfold-neon8**: worst_20/best_20 = 3.8x (possible bimodal distribution)
