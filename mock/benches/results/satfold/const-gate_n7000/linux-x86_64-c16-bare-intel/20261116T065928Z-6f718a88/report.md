# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-gate-true beats baseline by 74% (significant)

satfold-gate-true is -11.55 us (74%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 6.2x slower than the field

satfold-seq (15.65 us) is 6.2x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-gate-true, satfold-lanes16) are a dead heat (<1%)

satfold-gate-true (2.52 us) and satfold-lanes16 (2.54 us) differ by 0.61%, inside the noise, even though the wider field spreads 520.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-seq shows alternating (throttle bounce) (autocorr -0.75)

satfold-seq's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-gate-true, satfold-lanes16, satfold-lanes16-3} vs {satfold-gate-false, satfold-seq} (409% apart)

The field splits into a fast tier {satfold-gate-true, satfold-lanes16, satfold-lanes16-3} and a slow tier {satfold-gate-false, satfold-seq} with a 409% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest satfold-gate-true (2.52 us) to slowest satfold-seq (15.65 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-3 is inconsistent: worst-20% is 3.6x its best-20%

satfold-lanes16-3's best 20% of batches run at 2.51 us but its worst 20% at 9.08 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-gate-true** at 2523.6 ns median (-83.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 6.20x (fastest 2523.6 ns, slowest 15646.0 ns)

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
| satfold-gate-false | 15705ns | 15714ns | 14120ns | 15703ns | 17296ns | base |
| satfold-gate-true | 3900ns | 2575ns | 2565ns | 2850ns | 8384ns | -75.17% |
| satfold-lanes16 | 4191ns | 2591ns | 2571ns | 3105ns | 9069ns | -73.32% |
| satfold-lanes16-3 | 4507ns | 3135ns | 2564ns | 3549ns | 9327ns | -71.30% |
| satfold-seq | 15707ns | 15738ns | 14118ns | 15709ns | 17291ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 15634ns | 14060ns | 17212ns | base | 2.096 |
| satfold-gate-true | 3810ns | 2513ns | 8173ns | -75.63% | 8.602 |
| satfold-lanes16 | 4094ns | 2520ns | 8826ns | -73.81% | 8.003 |
| satfold-lanes16-3 | 4405ns | 2512ns | 9083ns | -71.82% | 7.438 |
| satfold-seq | 15635ns | 14058ns | 17210ns | +0.01% | 2.096 |

## Performance model

- Peak throughput: **13.043 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 2.096 | 16.1% |
| satfold-gate-true | 12.984 | 99.6% |
| satfold-lanes16 | 12.905 | 98.9% |
| satfold-lanes16-3 | 10.669 | 81.8% |
| satfold-seq | 2.094 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 15705ns | 15705ns | base |
| satfold-gate-true | 3900ns | 3900ns | -75.17% |
| satfold-lanes16 | 4191ns | 4191ns | -73.32% |
| satfold-lanes16-3 | 4507ns | 4507ns | -71.30% |
| satfold-seq | 15707ns | 15707ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 15636ns | base | --- | [14071, 17191] | --- | --- | --- | --- |
| satfold-gate-true | 2524ns | -11552.5ns (-73.9%) | [-14672, -11145]ns | [2518, 3071] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 2539ns | -11541.1ns (-73.8%) | [-14659, -10982]ns | [2522, 3088] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 3071ns | -11537.8ns (-73.8%) | [-13166, -10990]ns | [2808, 3084] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 15646ns | no significant difference | [-8, +13]ns | [14076, 17194] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 14066ns | -82.1% | -78.0% | -82.1% | +0.1% |
| 1 | 2 | 17192ns | -85.4% | -85.3% | -85.4% | +0.0% |
| 1 | 3 | 17201ns | -85.4% | -85.3% | -82.1% | -18.2% |
| 1 | 4 | 14065ns | -78.1% | -82.0% | -78.1% | +22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | -0.250 | moderate- |
| satfold-gate-true | -0.084 | ok |
| satfold-lanes16 | -0.092 | ok |
| satfold-lanes16-3 | 0.247 | moderate+ |
| satfold-seq | -0.750 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 21.1ns | 15633.5ns | 0.1% |  |
| satfold-gate-true | 28.6ns | 3809.6ns | 0.8% |  |
| satfold-lanes16 | 31.6ns | 4094.5ns | 0.8% |  |
| satfold-lanes16-3 | 33.2ns | 4405.4ns | 0.8% |  |
| satfold-seq | 21.7ns | 15635.2ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 14060.0-17211.9 ns)
  14060.0 |#####################################
  14217.6 |
  14375.2 |
  14532.8 |
  14690.4 |
  14848.0 |
  15005.6 |
  15163.2 |
  15320.8 |
  15478.3 |
  15635.9 |
  15793.5 |
  15951.1 |
  16108.7 |
  16266.3 |
  16423.9 |
  16581.5 |
  16739.1 |
  16896.7 |
  17054.3 |########################################
  (3 below, 2 above range)

satfold-gate-true (n=40, range 2512.8-8173.1 ns)
   2512.8 |########################################
   2795.8 |##########
   3078.9 |####
   3361.9 |
   3644.9 |
   3927.9 |
   4210.9 |
   4493.9 |
   4776.9 |
   5059.9 |##
   5343.0 |
   5626.0 |##
   5909.0 |
   6192.0 |
   6475.0 |##
   6758.0 |
   7041.0 |
   7324.0 |
   7607.1 |
   7890.1 |##
  (4 below, 5 above range)

satfold-lanes16 (n=40, range 2519.9-8825.6 ns)
   2519.9 |########################################
   2835.2 |################
   3150.4 |
   3465.7 |
   3781.0 |
   4096.3 |##
   4411.6 |
   4726.9 |##
   5042.2 |
   5357.4 |
   5672.7 |
   5988.0 |##
   6303.3 |
   6618.6 |
   6933.9 |##
   7249.2 |##
   7564.5 |##
   7879.8 |
   8195.0 |
   8510.3 |##
  (2 below, 4 above range)

satfold-lanes16-3 (n=40, range 2512.3-9082.8 ns)
   2512.3 |###############################
   2840.9 |########################################
   3169.4 |
   3497.9 |
   3826.4 |
   4155.0 |
   4483.5 |
   4812.0 |##
   5140.5 |
   5469.1 |##
   5797.6 |
   6126.1 |
   6454.6 |
   6783.1 |
   7111.7 |##
   7440.2 |##
   7768.7 |#####
   8097.2 |##
   8425.8 |
   8754.3 |##
  (3 below, 4 above range)

satfold-seq (n=40, range 14057.9-17210.2 ns)
  14057.9 |#####################################
  14215.5 |
  14373.1 |
  14530.7 |
  14688.3 |
  14846.0 |
  15003.6 |
  15161.2 |
  15318.8 |
  15476.4 |
  15634.0 |
  15791.6 |
  15949.3 |
  16106.9 |
  16264.5 |
  16422.1 |
  16579.7 |
  16737.3 |
  16894.9 |
  17052.6 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-gate-true**: CV=60.1% (high variance, measurements may be unstable)
- **satfold-gate-true**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **satfold-lanes16**: CV=62.3% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16-3**: CV=59.6% (high variance, measurements may be unstable)
- **satfold-lanes16-3**: worst_20/best_20 = 3.6x (possible bimodal distribution)
