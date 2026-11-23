# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 beats baseline by 82% (significant)

satfold-lanes16 is -11.53 us (82%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 6.4x slower than the field

satfold-seq (16.12 us) is 6.4x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16 is fastest but the noisiest (CV 112.4%)

satfold-lanes16 wins on median (2.50 us) yet has the highest variance (CV 112.4%), while satfold-gate-false is the steadiest (CV 0.2%, 14.06 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (satfold-lanes16, satfold-gate-true) are a dead heat (<1%)

satfold-lanes16 (2.50 us) and satfold-gate-true (2.51 us) differ by 0.42%, inside the noise, even though the wider field spreads 544.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-seq shows alternating (throttle bounce) (autocorr -0.75)

satfold-seq's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16, satfold-gate-true, satfold-lanes16-3} vs {satfold-gate-false, satfold-seq} (362% apart)

The field splits into a fast tier {satfold-lanes16, satfold-gate-true, satfold-lanes16-3} and a slow tier {satfold-gate-false, satfold-seq} with a 362% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.4x the fastest

Fastest satfold-lanes16 (2.50 us) to slowest satfold-seq (16.12 us): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16 is inconsistent: worst-20% is 3.8x its best-20%

satfold-lanes16's best 20% of batches run at 2.49 us but its worst 20% at 9.47 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16** at 2499.4 ns median (-82.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.45x (fastest 2499.4 ns, slowest 16116.4 ns)

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
| satfold-gate-false | 14122ns | 14120ns | 14084ns | 14120ns | 14167ns | base |
| satfold-gate-true | 4321ns | 2563ns | 2538ns | 3259ns | 9290ns | -69.40% |
| satfold-lanes16 | 4554ns | 2552ns | 2538ns | 3513ns | 9692ns | -67.76% |
| satfold-lanes16-3 | 4183ns | 3105ns | 2541ns | 2951ns | 9519ns | -70.38% |
| satfold-seq | 15745ns | 16185ns | 14128ns | 15766ns | 17299ns | +11.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 14059ns | 14024ns | 14095ns | base | 2.331 |
| satfold-gate-true | 4220ns | 2486ns | 9043ns | -69.98% | 7.764 |
| satfold-lanes16 | 4455ns | 2486ns | 9470ns | -68.31% | 7.355 |
| satfold-lanes16-3 | 4084ns | 2489ns | 9263ns | -70.95% | 8.024 |
| satfold-seq | 15674ns | 14068ns | 17217ns | +11.48% | 2.091 |

## Performance model

- Peak throughput: **13.180 Gops/s** (satfold-gate-true; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 2.331 | 17.7% |
| satfold-gate-true | 13.055 | 99.1% |
| satfold-lanes16 | 13.110 | 99.5% |
| satfold-lanes16-3 | 10.776 | 81.8% |
| satfold-seq | 2.033 | 15.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 14122ns | 14122ns | base |
| satfold-gate-true | 4321ns | 4321ns | -69.40% |
| satfold-lanes16 | 4554ns | 4554ns | -67.76% |
| satfold-lanes16-3 | 4183ns | 4183ns | -70.38% |
| satfold-seq | 15745ns | 15745ns | +11.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 14060ns | base | --- | [14044, 14071] | --- | --- | --- | --- |
| satfold-gate-true | 2510ns | -11513.0ns (-81.9%) | [-11542, -11017]ns | [2499, 3046] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 2499ns | -11527.9ns (-82.0%) | [-11553, -9933]ns | [2488, 4131] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 3041ns | -11020.6ns (-78.4%) | [-11517, -11002]ns | [2503, 3053] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 16116ns | +2063.8ns (+14.7%) | [+42, +3140]ns | [14078, 17198] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 14042ns | -82.2% | -82.3% | -82.2% | +22.5% |
| 1 | 2 | 14086ns | -82.3% | -82.3% | -82.3% | -0.1% |
| 1 | 3 | 14062ns | -78.3% | -82.3% | -78.4% | +22.3% |
| 1 | 4 | 14054ns | -82.2% | -70.0% | -78.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | -0.447 | moderate- |
| satfold-gate-true | -0.432 | moderate- |
| satfold-lanes16 | -0.082 | ok |
| satfold-lanes16-3 | 0.251 | moderate+ |
| satfold-seq | -0.749 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 19.1ns | 14059.4ns | 0.1% |  |
| satfold-gate-true | 31.1ns | 4220.4ns | 0.7% |  |
| satfold-lanes16 | 31.3ns | 4455.4ns | 0.7% |  |
| satfold-lanes16-3 | 30.2ns | 4084.0ns | 0.7% |  |
| satfold-seq | 21.6ns | 15673.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 14024.1-14094.9 ns)
  14024.1 |##########
  14027.6 |####################
  14031.2 |##########
  14034.7 |####################
  14038.2 |####################
  14041.8 |##############################
  14045.3 |
  14048.9 |##########
  14052.4 |####################
  14056.0 |##########
  14059.5 |########################################
  14063.0 |##########
  14066.6 |##########
  14070.1 |####################
  14073.7 |####################
  14077.2 |
  14080.8 |####################
  14084.3 |
  14087.9 |####################
  14091.4 |####################
  (4 below, 5 above range)

satfold-gate-true (n=40, range 2486.2-9042.9 ns)
   2486.2 |########################################
   2814.0 |###############
   3141.9 |
   3469.7 |
   3797.5 |##
   4125.4 |
   4453.2 |
   4781.0 |##
   5108.9 |
   5436.7 |##
   5764.5 |
   6092.4 |
   6420.2 |
   6748.0 |
   7075.9 |
   7403.7 |
   7731.5 |
   8059.4 |##
   8387.2 |######
   8715.0 |##
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 2486.2-9470.1 ns)
   2486.2 |########################################
   2835.4 |##
   3184.6 |##
   3533.8 |
   3883.0 |######
   4232.2 |####
   4581.4 |
   4930.6 |##
   5279.8 |
   5629.0 |
   5978.2 |
   6327.4 |##
   6676.5 |
   7025.7 |
   7374.9 |
   7724.1 |
   8073.3 |
   8422.5 |
   8771.7 |######
   9120.9 |
  (4 below, 6 above range)

satfold-lanes16-3 (n=40, range 2488.8-9262.8 ns)
   2488.8 |#####################
   2827.5 |########################################
   3166.2 |
   3504.9 |
   3843.6 |
   4182.3 |
   4521.0 |
   4859.7 |
   5198.4 |
   5537.1 |
   5875.8 |
   6214.5 |##
   6553.2 |
   6891.9 |
   7230.6 |
   7569.3 |
   7908.0 |
   8246.7 |##
   8585.4 |
   8924.1 |####
  (6 below, 4 above range)

satfold-seq (n=40, range 14067.7-17217.2 ns)
  14067.7 |#####################################
  14225.1 |
  14382.6 |##
  14540.1 |
  14697.6 |
  14855.0 |
  15012.5 |##
  15170.0 |
  15327.5 |
  15485.0 |
  15642.4 |
  15799.9 |
  15957.4 |
  16114.9 |
  16272.4 |
  16429.8 |
  16587.3 |
  16744.8 |
  16902.3 |
  17059.8 |########################################
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-gate-true**: CV=62.6% (high variance, measurements may be unstable)
- **satfold-gate-true**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes16**: CV=63.0% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-lanes16-3**: CV=65.2% (high variance, measurements may be unstable)
- **satfold-lanes16-3**: worst_20/best_20 = 3.7x (possible bimodal distribution)
