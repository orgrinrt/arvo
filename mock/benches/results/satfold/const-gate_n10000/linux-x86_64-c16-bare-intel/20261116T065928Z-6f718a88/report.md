# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16-3 dominates: 13% faster than the next best (satfold-lanes16)

satfold-lanes16-3 (2.85 us) leads satfold-lanes16 (3.23 us) by 13%, a clear separation rather than a photo finish. CV 16.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes16 beats baseline by 90% (significant)

satfold-lanes16 is -20.66 us (90%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 8.7x slower than the field

satfold-seq (24.86 us) is 8.7x the fastest (2.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-gate-true shows alternating (throttle bounce) (autocorr -0.72)

satfold-gate-true's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} vs {satfold-gate-false, satfold-seq} (571% apart)

The field splits into a fast tier {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} and a slow tier {satfold-gate-false, satfold-seq} with a 571% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.7x the fastest

Fastest satfold-lanes16-3 (2.85 us) to slowest satfold-seq (24.86 us): 8.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16 is inconsistent: worst-20% is 2.4x its best-20%

satfold-lanes16's best 20% of batches run at 1.76 us but its worst 20% at 4.29 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16-3** at 2850.1 ns median (-87.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 8.72x (fastest 2850.1 ns, slowest 24862.2 ns)

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
| Counter frequency | 2112736210 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 23086ns | 23102ns | 20766ns | 23085ns | 25411ns | base |
| satfold-gate-true | 3593ns | 3636ns | 2886ns | 3674ns | 4057ns | -84.44% |
| satfold-lanes16 | 3379ns | 3430ns | 1871ns | 3477ns | 4593ns | -85.36% |
| satfold-lanes16-3 | 3131ns | 3049ns | 2518ns | 3093ns | 3856ns | -86.44% |
| satfold-seq | 23827ns | 24937ns | 20385ns | 24564ns | 25056ns | +3.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 23014ns | 20706ns | 25325ns | base | 1.424 |
| satfold-gate-true | 3374ns | 2719ns | 3780ns | -85.34% | 9.712 |
| satfold-lanes16 | 3170ns | 1762ns | 4295ns | -86.23% | 10.336 |
| satfold-lanes16-3 | 2938ns | 2373ns | 3603ns | -87.23% | 11.152 |
| satfold-seq | 23752ns | 20325ns | 24972ns | +3.21% | 1.380 |

## Performance model

- Peak throughput: **18.598 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.424 | 7.7% |
| satfold-gate-true | 9.560 | 51.4% |
| satfold-lanes16 | 10.149 | 54.6% |
| satfold-lanes16-3 | 11.497 | 61.8% |
| satfold-seq | 1.318 | 7.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 23086ns | 23086ns | base |
| satfold-gate-true | 3593ns | 3593ns | -84.44% |
| satfold-lanes16 | 3379ns | 3379ns | -85.36% |
| satfold-lanes16-3 | 3131ns | 3131ns | -86.44% |
| satfold-seq | 23827ns | 23827ns | +3.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 23015ns | base | --- | [20714, 25308] | --- | --- | --- | --- |
| satfold-gate-true | 3428ns | -19843.2ns (-86.2%) | [-21619, -17451]ns | [3325, 3677] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 3229ns | -20662.2ns (-89.8%) | [-22018, -17656]ns | [2977, 3508] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 2850ns | -19926.8ns (-86.6%) | [-21937, -18080]ns | [2739, 3063] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 24862ns | -397.8ns (-1.7%) | [-411, -356]ns | [24846, 24875] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 20713ns | -82.7% | -81.8% | -87.3% | -1.9% |
| 1 | 2 | 20707ns | -83.7% | -83.4% | -85.3% | +20.0% |
| 1 | 3 | 25319ns | -85.9% | -88.4% | -86.0% | -1.7% |
| 1 | 4 | 25319ns | -86.7% | -87.3% | -89.3% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | 0.250 | moderate+ |
| satfold-gate-true | -0.722 | HIGH- (thermal bounce) |
| satfold-lanes16 | 0.133 | ok |
| satfold-lanes16-3 | -0.266 | moderate- |
| satfold-seq | -0.078 | ok |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 21.6ns | 23014.2ns | 0.1% |  |
| satfold-gate-true | 76.2ns | 3374.1ns | 2.3% |  |
| satfold-lanes16 | 72.8ns | 3170.2ns | 2.3% |  |
| satfold-lanes16-3 | 66.9ns | 2938.4ns | 2.3% |  |
| satfold-seq | 22.7ns | 23752.1ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 20706.1-25324.9 ns)
  20706.1 |#####################################
  20937.0 |
  21168.0 |
  21398.9 |
  21629.8 |
  21860.8 |
  22091.7 |
  22322.7 |
  22553.6 |
  22784.5 |
  23015.5 |
  23246.4 |
  23477.4 |
  23708.3 |
  23939.2 |
  24170.2 |
  24401.1 |
  24632.1 |
  24863.0 |
  25093.9 |########################################
  (4 below, 3 above range)

satfold-gate-true (n=40, range 2719.0-3780.4 ns)
   2719.0 |
   2772.0 |
   2825.1 |###
   2878.2 |
   2931.3 |###
   2984.3 |
   3037.4 |#####################
   3090.5 |###
   3143.5 |
   3196.6 |
   3249.7 |#######
   3302.8 |##########
   3355.8 |##############
   3408.9 |###
   3462.0 |#######
   3515.1 |
   3568.1 |
   3621.2 |
   3674.3 |########################################
   3727.3 |##############
  (2 below, 2 above range)

satfold-lanes16 (n=40, range 1762.0-4294.7 ns)
   1762.0 |
   1888.6 |##########
   2015.2 |
   2141.9 |
   2268.5 |
   2395.1 |
   2521.8 |##############################
   2648.4 |
   2775.1 |##############################
   2901.7 |########################################
   3028.3 |########################################
   3155.0 |##########
   3281.6 |##############################
   3408.2 |########################################
   3534.9 |##########
   3661.5 |####################
   3788.2 |########################################
   3914.8 |##########
   4041.4 |##########
   4168.1 |
  (4 below, 4 above range)

satfold-lanes16-3 (n=40, range 2372.6-3602.8 ns)
   2372.6 |######
   2434.1 |####################
   2495.6 |######
   2557.1 |######
   2618.6 |########################################
   2680.1 |######
   2741.7 |#############
   2803.2 |##########################
   2864.7 |####################
   2926.2 |######
   2987.7 |######
   3049.2 |####################
   3110.7 |
   3172.2 |######
   3233.7 |
   3295.3 |##########################
   3356.8 |######
   3418.3 |######
   3479.8 |
   3541.3 |
  (1 below, 5 above range)

satfold-seq (n=40, range 20325.4-24971.7 ns)
  20325.4 |#######
  20557.7 |
  20790.0 |
  21022.3 |
  21254.7 |
  21487.0 |
  21719.3 |
  21951.6 |
  22183.9 |
  22416.2 |
  22648.6 |
  22880.9 |
  23113.2 |
  23345.5 |
  23577.8 |
  23810.1 |
  24042.4 |
  24274.8 |
  24507.1 |
  24739.4 |########################################
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=29.2% (high variance, measurements may be unstable)
