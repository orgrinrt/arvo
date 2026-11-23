# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes64 shows alternating (throttle bounce) (autocorr -0.79)

satfold-lanes64's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (862 ns) is smaller than the fastest variant's own run-to-run std-dev (1.02 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### satfold-neon8 is inconsistent: worst-20% is 2.2x its best-20%

satfold-neon8's best 20% of batches run at 4.90 us but its worst 20% at 10.64 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16** at 5526.9 ns median (-13.0% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 5526.9 ns, slowest 6389.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112723939 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 6163ns | 6426ns | 5213ns | 6318ns | 6646ns | base |
| satfold-lanes16 | 6099ns | 5584ns | 5527ns | 5756ns | 7702ns | -1.02% |
| satfold-lanes16-constl | 6217ns | 5914ns | 5655ns | 5920ns | 7672ns | +0.88% |
| satfold-lanes4-idx | 6093ns | 6053ns | 5012ns | 6037ns | 7340ns | -1.13% |
| satfold-lanes64 | 6046ns | 6014ns | 5374ns | 6002ns | 6847ns | -1.90% |
| satfold-neon | 5847ns | 6106ns | 4983ns | 5679ns | 7218ns | -5.11% |
| satfold-neon8 | 6854ns | 6066ns | 4960ns | 6168ns | 10802ns | +11.21% |
| satfold-nolaw | 5973ns | 6142ns | 5061ns | 6092ns | 6527ns | -3.07% |
| satfold-seq | 6453ns | 6459ns | 5245ns | 6412ns | 7786ns | +4.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 6087ns | 5150ns | 6551ns | base | 5.383 |
| satfold-lanes16 | 6028ns | 5468ns | 7592ns | -0.97% | 5.436 |
| satfold-lanes16-constl | 6133ns | 5582ns | 7545ns | +0.76% | 5.343 |
| satfold-lanes4-idx | 6014ns | 4953ns | 7221ns | -1.19% | 5.448 |
| satfold-lanes64 | 5975ns | 5316ns | 6768ns | -1.84% | 5.484 |
| satfold-neon | 5770ns | 4926ns | 7098ns | -5.20% | 5.679 |
| satfold-neon8 | 6763ns | 4903ns | 10642ns | +11.10% | 4.846 |
| satfold-nolaw | 5898ns | 5003ns | 6435ns | -3.10% | 5.556 |
| satfold-seq | 6374ns | 5185ns | 7665ns | +4.72% | 5.141 |

## Performance model

- Peak throughput: **6.684 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 5.157 | 77.2% |
| satfold-lanes16 | 5.929 | 88.7% |
| satfold-lanes16-constl | 5.608 | 83.9% |
| satfold-lanes4-idx | 5.480 | 82.0% |
| satfold-lanes64 | 5.532 | 82.8% |
| satfold-neon | 5.430 | 81.2% |
| satfold-neon8 | 5.466 | 81.8% |
| satfold-nolaw | 5.400 | 80.8% |
| satfold-seq | 5.129 | 76.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 6163ns | 6163ns | base |
| satfold-lanes16 | 6099ns | 6099ns | -1.02% |
| satfold-lanes16-constl | 6217ns | 6217ns | +0.88% |
| satfold-lanes4-idx | 6093ns | 6093ns | -1.13% |
| satfold-lanes64 | 6046ns | 6046ns | -1.90% |
| satfold-neon | 5847ns | 5847ns | -5.11% |
| satfold-neon8 | 6854ns | 6854ns | +11.21% |
| satfold-nolaw | 5973ns | 5973ns | -3.07% |
| satfold-seq | 6453ns | 6453ns | +4.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 6354ns | base | --- | [6281, 6396] | --- | --- | --- | --- |
| satfold-lanes16 | 5527ns | no significant difference | [-803, +417]ns | [5501, 5577] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes16-constl | 5843ns | -430.8ns (-6.8%) | [-493, -126]ns | [5827, 5854] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| satfold-lanes4-idx | 5980ns | -314.5ns (-4.9%) | [-393, -196]ns | [5961, 6024] | YES | 0.0129 | 0.0064 | 0 |
| satfold-lanes64 | 5923ns | +179.7ns (+2.8%) | [+13, +217]ns | [5367, 6425] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| satfold-neon | 6035ns | -248.1ns (-3.9%) | [-367, -218]ns | [4974, 6049] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 5995ns | -259.1ns (-4.1%) | [-403, -236]ns | [4958, 6009] | YES | 0.0129 | 0.0064 | 0 |
| satfold-nolaw | 6068ns | -261.8ns (-4.1%) | [-331, -229]ns | [6017, 6085] | YES | 0.0129 | 0.0064 | 0 |
| satfold-seq | 6389ns | no significant difference | [-2, +53]ns | [6375, 6400] | no | 0.1236 | 0.1081 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5156ns | +8.0% | +13.3% | +15.7% | +3.9% | -4.4% | -4.9% | +16.7% | +0.7% |
| 1 | 2 | 6307ns | -12.7% | -7.2% | -21.4% | +7.3% | -2.8% | -21.3% | -20.6% | +1.7% |
| 1 | 3 | 6406ns | -14.6% | -12.4% | -4.0% | -16.8% | -22.4% | +64.7% | -3.9% | -0.0% |
| 1 | 4 | 6405ns | +7.2% | -8.3% | -6.3% | +0.2% | -5.6% | -6.2% | -5.0% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.021 | ok |
| satfold-lanes16 | -0.113 | ok |
| satfold-lanes16-constl | -0.490 | moderate- |
| satfold-lanes4-idx | -0.423 | moderate- |
| satfold-lanes64 | -0.792 | HIGH- (thermal bounce) |
| satfold-neon | -0.758 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.281 | moderate- |
| satfold-nolaw | -0.394 | moderate- |
| satfold-seq | -0.093 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 3/4, lost 1/4
- **satfold-lanes4-idx**: won 3/4, lost 1/4
- **satfold-lanes64**: won 1/4, lost 3/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 3/4, lost 1/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 22.9ns | 6086.8ns | 0.4% |  |
| satfold-lanes16 | 21.5ns | 6027.6ns | 0.4% |  |
| satfold-lanes16-constl | 25.2ns | 6133.0ns | 0.4% |  |
| satfold-lanes4-idx | 23.3ns | 6014.4ns | 0.4% |  |
| satfold-lanes64 | 21.9ns | 5975.0ns | 0.4% |  |
| satfold-neon | 23.1ns | 5770.3ns | 0.4% |  |
| satfold-neon8 | 27.1ns | 6762.5ns | 0.4% |  |
| satfold-nolaw | 23.0ns | 5897.8ns | 0.4% |  |
| satfold-seq | 23.9ns | 6374.1ns | 0.4% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 5150.3-6550.9 ns)
   5150.3 |####################
   5220.4 |
   5290.4 |
   5360.4 |
   5430.4 |
   5500.5 |
   5570.5 |
   5640.5 |
   5710.6 |
   5780.6 |
   5850.6 |
   5920.6 |
   5990.7 |
   6060.7 |
   6130.7 |##
   6200.8 |##
   6270.8 |#################
   6340.8 |########################################
   6410.9 |####################
   6480.9 |
  (3 below, 1 above range)

satfold-lanes16 (n=40, range 5468.2-7592.3 ns)
   5468.2 |########################################
   5574.4 |#####
   5680.6 |
   5786.8 |
   5893.0 |
   5999.2 |
   6105.4 |
   6211.6 |
   6317.8 |
   6424.0 |
   6530.2 |
   6636.4 |
   6742.6 |###
   6848.8 |############
   6955.0 |
   7061.2 |
   7167.4 |
   7273.6 |
   7379.9 |
   7486.1 |
  (4 below, 2 above range)

satfold-lanes16-constl (n=40, range 5582.5-7544.8 ns)
   5582.5 |####
   5680.6 |######
   5778.7 |########################################
   5876.8 |##
   5975.0 |####
   6073.1 |##
   6171.2 |####
   6269.3 |
   6367.4 |##
   6465.5 |
   6563.7 |
   6661.8 |
   6759.9 |##
   6858.0 |
   6956.1 |
   7054.2 |
   7152.4 |
   7250.5 |
   7348.6 |
   7446.7 |
  (5 below, 2 above range)

satfold-lanes4-idx (n=40, range 4953.4-7221.3 ns)
   4953.4 |################
   5066.8 |
   5180.2 |
   5293.6 |
   5407.0 |
   5520.4 |
   5633.8 |
   5747.2 |
   5860.6 |####################################
   5974.0 |########################################
   6087.4 |################################
   6200.8 |####
   6314.2 |####
   6427.6 |
   6541.0 |
   6654.3 |
   6767.7 |
   6881.1 |
   6994.5 |
   7107.9 |
  (5 below, 2 above range)

satfold-lanes64 (n=40, range 5315.6-6768.3 ns)
   5315.6 |########################################
   5388.2 |
   5460.8 |######
   5533.5 |
   5606.1 |
   5678.8 |
   5751.4 |
   5824.0 |
   5896.7 |
   5969.3 |
   6041.9 |
   6114.6 |
   6187.2 |
   6259.9 |
   6332.5 |#########
   6405.1 |##################
   6477.8 |###
   6550.4 |
   6623.1 |
   6695.7 |##################
  (5 below, 4 above range)

satfold-neon (n=40, range 4925.7-7098.3 ns)
   4925.7 |########################################
   5034.3 |
   5142.9 |
   5251.6 |
   5360.2 |
   5468.8 |
   5577.4 |
   5686.1 |
   5794.7 |
   5903.3 |
   6012.0 |########################################
   6120.6 |#################
   6229.2 |
   6337.9 |
   6446.5 |
   6555.1 |
   6663.8 |
   6772.4 |
   6881.0 |
   6989.7 |
  (4 below, 2 above range)

satfold-neon8 (n=40, range 4902.8-10641.5 ns)
   4902.8 |########################################
   5189.7 |
   5476.7 |
   5763.6 |############################
   6050.5 |
   6337.5 |
   6624.4 |
   6911.3 |
   7198.3 |
   7485.2 |
   7772.2 |
   8059.1 |
   8346.0 |
   8633.0 |
   8919.9 |
   9206.8 |
   9493.8 |
   9780.7 |
  10067.7 |
  10354.6 |#########################
  (5 below, 2 above range)

satfold-nolaw (n=40, range 5002.9-6435.2 ns)
   5002.9 |########################
   5074.5 |
   5146.1 |
   5217.7 |
   5289.3 |
   5360.9 |
   5432.6 |
   5504.2 |
   5575.8 |
   5647.4 |
   5719.0 |
   5790.6 |
   5862.2 |
   5933.9 |################
   6005.5 |########################################
   6077.1 |########################################
   6148.7 |########################
   6220.3 |
   6291.9 |
   6363.5 |
  (3 below, 1 above range)

satfold-seq (n=40, range 5185.3-7664.9 ns)
   5185.3 |########
   5309.3 |
   5433.3 |
   5557.3 |
   5681.2 |
   5805.2 |
   5929.2 |
   6053.2 |
   6177.2 |
   6301.1 |########################################
   6425.1 |#####
   6549.1 |
   6673.1 |
   6797.0 |
   6921.0 |#
   7045.0 |
   7169.0 |
   7293.0 |#
   7416.9 |
   7540.9 |
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16-constl**: CV=20.1% (high variance, measurements may be unstable)
- **satfold-neon**: CV=22.0% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=35.6% (high variance, measurements may be unstable)
