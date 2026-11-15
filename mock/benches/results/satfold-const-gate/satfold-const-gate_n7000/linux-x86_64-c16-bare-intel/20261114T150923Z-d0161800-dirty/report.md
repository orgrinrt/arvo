# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 dominates: 74% faster than the next best (satfold-lanes16-3)

satfold-lanes16 (3.09 us) leads satfold-lanes16-3 (5.37 us) by 74%, a clear separation rather than a photo finish. CV 88.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes16 beats baseline by 74% (significant)

satfold-lanes16 is -11.54 us (74%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 5.6x slower than the field

satfold-seq (17.19 us) is 5.6x the fastest (3.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16 is fastest but the noisiest (CV 88.7%)

satfold-lanes16 wins on median (3.09 us) yet has the highest variance (CV 88.7%), while satfold-seq is the steadiest (CV 7.7%, 17.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.6x the fastest

Fastest satfold-lanes16 (3.09 us) to slowest satfold-seq (17.19 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-3 is inconsistent: worst-20% is 3.8x its best-20%

satfold-lanes16-3's best 20% of batches run at 2.51 us but its worst 20% at 9.48 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-seq's edge over baseline is significant but tiny (30 ns, 0.19%)

satfold-seq differs from baseline satfold-gate-false by 30 ns (0.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes16** at 3094.6 ns median (-80.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.56x (fastest 3094.6 ns, slowest 17194.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 15724ns | 15737ns | 14118ns | 15702ns | 17395ns | base |
| satfold-gate-true | 6264ns | 6640ns | 2784ns | 6251ns | 9781ns | -60.16% |
| satfold-lanes16 | 4820ns | 3157ns | 2716ns | 3862ns | 9798ns | -69.35% |
| satfold-lanes16-3 | 5828ns | 5482ns | 2564ns | 5651ns | 9624ns | -62.93% |
| satfold-seq | 16519ns | 17269ns | 14183ns | 17031ns | 17321ns | +5.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 15651ns | 14057ns | 17303ns | base | 2.094 |
| satfold-gate-true | 6136ns | 2728ns | 9571ns | -60.79% | 5.340 |
| satfold-lanes16 | 4710ns | 2662ns | 9533ns | -69.91% | 6.958 |
| satfold-lanes16-3 | 5717ns | 2513ns | 9481ns | -63.47% | 5.732 |
| satfold-seq | 16443ns | 14118ns | 17225ns | +5.06% | 1.993 |

## Performance model

- Peak throughput: **13.040 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 2.097 | 16.1% |
| satfold-gate-true | 5.036 | 38.6% |
| satfold-lanes16 | 10.589 | 81.2% |
| satfold-lanes16-3 | 6.099 | 46.8% |
| satfold-seq | 1.906 | 14.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 15724ns | 15724ns | base |
| satfold-gate-true | 6264ns | 6264ns | -60.16% |
| satfold-lanes16 | 4820ns | 4820ns | -69.35% |
| satfold-lanes16-3 | 5828ns | 5828ns | -62.93% |
| satfold-seq | 16519ns | 16519ns | +5.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 15627ns | base | --- | [14070, 17196] | --- | --- | --- | --- |
| satfold-gate-true | 6507ns | -8710.4ns (-55.7%) | [-11549, -7012]ns | [3082, 8747] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 3095ns | -11540.1ns (-73.8%) | [-13464, -10967]ns | [3082, 3116] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 5372ns | -11202.5ns (-71.7%) | [-11616, -8308]ns | [3071, 8067] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 17194ns | +29.5ns (+0.2%) | [+14, +86]ns | [17188, 17210] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 14065ns | -71.3% | -78.0% | -82.1% | +0.3% |
| 1 | 2 | 14067ns | -32.6% | -82.0% | -32.6% | +22.4% |
| 1 | 3 | 17199ns | -82.1% | -81.9% | -85.4% | +0.1% |
| 1 | 4 | 17200ns | -82.1% | -82.1% | -82.1% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | 0.250 | moderate+ |
| satfold-gate-true | -0.319 | moderate- |
| satfold-lanes16 | -0.437 | moderate- |
| satfold-lanes16-3 | -0.481 | moderate- |
| satfold-seq | -0.085 | ok |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 21.5ns | 15651.1ns | 0.1% |  |
| satfold-gate-true | 38.0ns | 6136.1ns | 0.6% |  |
| satfold-lanes16 | 36.2ns | 4709.6ns | 0.8% |  |
| satfold-lanes16-3 | 34.8ns | 5716.8ns | 0.6% |  |
| satfold-seq | 23.1ns | 16442.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 14057.3-17303.1 ns)
  14057.3 |###################################
  14219.6 |
  14381.9 |
  14544.2 |
  14706.4 |
  14868.7 |
  15031.0 |
  15193.3 |
  15355.6 |
  15517.9 |
  15680.2 |
  15842.5 |
  16004.8 |
  16167.0 |
  16329.3 |
  16491.6 |
  16653.9 |
  16816.2 |
  16978.5 |
  17140.8 |########################################
  (3 below, 1 above range)

satfold-gate-true (n=40, range 2728.1-9571.2 ns)
   2728.1 |
   3070.2 |########################################
   3412.4 |###
   3754.6 |
   4096.7 |
   4438.9 |
   4781.0 |
   5123.2 |
   5465.3 |###
   5807.5 |###
   6149.7 |
   6491.8 |
   6834.0 |###
   7176.1 |###
   7518.3 |
   7860.5 |###
   8202.6 |######
   8544.8 |######
   8886.9 |
   9229.1 |########################################
  (5 below, 1 above range)

satfold-lanes16 (n=40, range 2661.7-9533.0 ns)
   2661.7 |
   3005.3 |########################################
   3348.8 |
   3692.4 |
   4036.0 |#
   4379.5 |
   4723.1 |
   5066.7 |
   5410.2 |#
   5753.8 |#
   6097.4 |
   6440.9 |
   6784.5 |
   7128.1 |
   7471.6 |
   7815.2 |#
   8158.8 |###
   8502.3 |
   8845.9 |#####
   9189.5 |#
  (6 below, 3 above range)

satfold-lanes16-3 (n=40, range 2512.8-9481.1 ns)
   2512.8 |########################################
   2861.2 |###############################
   3209.6 |
   3558.1 |
   3906.5 |
   4254.9 |
   4603.3 |
   4951.7 |####
   5300.1 |####
   5648.6 |
   5997.0 |####
   6345.4 |
   6693.8 |
   7042.2 |####
   7390.6 |####
   7739.0 |########
   8087.5 |####
   8435.9 |########
   8784.3 |
   9132.7 |###################################
  (3 below, 3 above range)

satfold-seq (n=40, range 14117.8-17225.1 ns)
  14117.8 |###
  14273.2 |###
  14428.5 |
  14583.9 |
  14739.3 |
  14894.6 |
  15050.0 |
  15205.4 |
  15360.7 |
  15516.1 |
  15671.5 |
  15826.8 |
  15982.2 |
  16137.6 |
  16292.9 |
  16448.3 |
  16603.7 |
  16759.0 |
  16914.4 |
  17069.8 |########################################
  (6 below, 4 above range)

```

## Diagnostics

- **satfold-gate-true**: CV=49.2% (high variance, measurements may be unstable)
- **satfold-gate-true**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16**: CV=58.3% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes16-3**: CV=52.7% (high variance, measurements may be unstable)
- **satfold-lanes16-3**: worst_20/best_20 = 3.8x (possible bimodal distribution)
