# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-gate-true beats baseline by 87% (significant)

satfold-gate-true is -17.76 us (87%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 6.3x slower than the field

satfold-seq (20.94 us) is 6.3x the fastest (3.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-gate-true, satfold-lanes16) are a dead heat (<1%)

satfold-gate-true (3.33 us) and satfold-lanes16 (3.35 us) differ by 0.41%, inside the noise, even though the wider field spreads 528.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-gate-true shows alternating (throttle bounce) (autocorr -0.57)

satfold-gate-true's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-gate-true, satfold-lanes16, satfold-lanes16-3} vs {satfold-gate-false, satfold-seq} (470% apart)

The field splits into a fast tier {satfold-gate-true, satfold-lanes16, satfold-lanes16-3} and a slow tier {satfold-gate-false, satfold-seq} with a 470% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.3x the fastest

Fastest satfold-gate-true (3.33 us) to slowest satfold-seq (20.94 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-3 is inconsistent: worst-20% is 4.5x its best-20%

satfold-lanes16-3's best 20% of batches run at 907 ns but its worst 20% at 4.07 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-gate-true** at 3334.1 ns median (-83.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.28x (fastest 3334.1 ns, slowest 20939.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112526248 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 21616ns | 20492ns | 20401ns | 20869ns | 25070ns | base |
| satfold-gate-true | 3271ns | 3542ns | 1460ns | 3422ns | 4629ns | -84.87% |
| satfold-lanes16 | 3484ns | 3559ns | 2424ns | 3618ns | 4144ns | -83.88% |
| satfold-lanes16-3 | 3251ns | 3810ns | 964ns | 3633ns | 4394ns | -84.96% |
| satfold-seq | 22116ns | 21000ns | 20824ns | 21366ns | 25657ns | +2.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 21548ns | 20342ns | 24992ns | base | 1.521 |
| satfold-gate-true | 3066ns | 1375ns | 4320ns | -85.77% | 10.688 |
| satfold-lanes16 | 3269ns | 2282ns | 3859ns | -84.83% | 10.025 |
| satfold-lanes16-3 | 3047ns | 907ns | 4069ns | -85.86% | 10.753 |
| satfold-seq | 22048ns | 20758ns | 25578ns | +2.32% | 1.486 |

## Performance model

- Peak throughput: **36.136 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.605 | 4.4% |
| satfold-gate-true | 9.828 | 27.2% |
| satfold-lanes16 | 9.788 | 27.1% |
| satfold-lanes16-3 | 9.139 | 25.3% |
| satfold-seq | 1.565 | 4.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 21616ns | 21616ns | base |
| satfold-gate-true | 3271ns | 3271ns | -84.87% |
| satfold-lanes16 | 3484ns | 3484ns | -83.88% |
| satfold-lanes16-3 | 3251ns | 3251ns | -84.96% |
| satfold-seq | 22116ns | 22116ns | +2.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 20421ns | base | --- | [20398, 20460] | --- | --- | --- | --- |
| satfold-gate-true | 3334ns | -17762.5ns (-87.0%) | [-19328, -17453]ns | [2751, 3678] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 3348ns | -17192.2ns (-84.2%) | [-17363, -16931]ns | [3276, 3640] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 3586ns | -17079.0ns (-83.6%) | [-19320, -16867]ns | [3314, 3700] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 20940ns | +484.7ns (+2.4%) | [+350, +557]ns | [20937, 20944] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 20397ns | -83.4% | -83.8% | -81.9% | +25.5% |
| 1 | 2 | 24964ns | -85.2% | -86.7% | -85.5% | -16.1% |
| 1 | 3 | 20375ns | -87.1% | -83.0% | -83.5% | +2.4% |
| 1 | 4 | 20417ns | -82.8% | -82.2% | -82.4% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | -0.421 | moderate- |
| satfold-gate-true | -0.566 | HIGH- (thermal bounce) |
| satfold-lanes16 | 0.241 | moderate+ |
| satfold-lanes16-3 | -0.147 | ok |
| satfold-seq | -0.075 | ok |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 19.8ns | 21548.3ns | 0.1% |  |
| satfold-gate-true | 68.1ns | 3065.9ns | 2.2% |  |
| satfold-lanes16 | 74.6ns | 3268.6ns | 2.3% |  |
| satfold-lanes16-3 | 70.8ns | 3047.2ns | 2.3% |  |
| satfold-seq | 20.2ns | 22048.4ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 20342.0-24991.7 ns)
  20342.0 |########################################
  20574.5 |
  20807.0 |
  21039.5 |
  21272.0 |
  21504.5 |
  21736.9 |
  21969.4 |
  22201.9 |
  22434.4 |
  22666.9 |
  22899.3 |
  23131.8 |
  23364.3 |
  23596.8 |
  23829.3 |
  24061.7 |
  24294.2 |
  24526.7 |
  24759.2 |#########
  (4 below, 4 above range)

satfold-gate-true (n=40, range 1374.8-4320.1 ns)
   1374.8 |
   1522.1 |
   1669.4 |
   1816.6 |
   1963.9 |
   2111.2 |
   2258.4 |
   2405.7 |#################
   2553.0 |######################
   2700.2 |######################
   2847.5 |#####
   2994.8 |#####
   3142.0 |
   3289.3 |#################
   3436.5 |
   3583.8 |########################################
   3731.1 |######################
   3878.3 |
   4025.6 |#################
   4172.9 |
  (6 below, 4 above range)

satfold-lanes16 (n=40, range 2282.0-3859.5 ns)
   2282.0 |
   2360.9 |
   2439.7 |
   2518.6 |
   2597.5 |
   2676.4 |
   2755.2 |####
   2834.1 |
   2913.0 |
   2991.9 |#################
   3070.7 |#################
   3149.6 |
   3228.5 |####
   3307.4 |###############################
   3386.2 |####
   3465.1 |
   3544.0 |#############
   3622.8 |########################################
   3701.7 |########
   3780.6 |
  (4 below, 4 above range)

satfold-lanes16-3 (n=40, range 906.8-4068.9 ns)
    906.8 |############
   1064.9 |
   1223.0 |
   1381.1 |
   1539.2 |####
   1697.3 |
   1855.4 |
   2013.5 |
   2171.6 |
   2329.8 |
   2487.9 |
   2646.0 |
   2804.1 |####
   2962.2 |########
   3120.3 |####
   3278.4 |####################
   3436.5 |########
   3594.6 |########################################
   3752.7 |################
   3910.8 |########
  (5 below, 4 above range)

satfold-seq (n=40, range 20758.5-25578.4 ns)
  20758.5 |########################################
  20999.5 |
  21240.5 |
  21481.5 |
  21722.5 |
  21963.5 |
  22204.4 |
  22445.4 |
  22686.4 |
  22927.4 |
  23168.4 |
  23409.4 |
  23650.4 |
  23891.4 |
  24132.4 |
  24373.4 |
  24614.4 |
  24855.4 |
  25096.4 |
  25337.4 |#####
  (6 below, 7 above range)

```

## Diagnostics

- **satfold-gate-true**: CV=34.7% (high variance, measurements may be unstable)
- **satfold-gate-true**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **satfold-lanes16-3**: CV=38.0% (high variance, measurements may be unstable)
- **satfold-lanes16-3**: worst_20/best_20 = 4.5x (possible bimodal distribution)
