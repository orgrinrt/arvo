# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum dominates: 64% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-minimum (2.29 us) leads warm-clamp-accfit-dyn (3.76 us) by 64%, a clear separation rather than a photo finish. CV 105.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is an outlier: 2.2x slower than the field

warm-clamp-min-lanes (4.99 us) is 2.2x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum is fastest but the noisiest (CV 105.0%)

warm-clamp-minimum wins on median (2.29 us) yet has the highest variance (CV 105.0%), while warm-clamp-accfit-dyn is the steadiest (CV 28.7%, 3.76 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.68)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum} vs {warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-min-lanes} (64% apart)

The field splits into a fast tier {warm-clamp-minimum} and a slow tier {warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-min-lanes} with a 64% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-head is inconsistent: worst-20% is 4.3x its best-20%

warm-clamp-head's best 20% of batches run at 1.12 us but its worst 20% at 4.84 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 2287.2 ns median (-48.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.18x (fastest 2287.2 ns, slowest 4986.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3829ns | 4666ns | 1412ns | 4122ns | 5369ns | base |
| warm-clamp-accfit | 3735ns | 4221ns | 1421ns | 3981ns | 5310ns | -2.46% |
| warm-clamp-accfit-dyn | 3783ns | 3947ns | 1794ns | 4034ns | 5020ns | -1.21% |
| warm-clamp-head | 3663ns | 4192ns | 1203ns | 3949ns | 5265ns | -4.34% |
| warm-clamp-min-lanes | 4524ns | 5178ns | 1861ns | 4702ns | 6649ns | +18.13% |
| warm-clamp-minimum | 3760ns | 2359ns | 2254ns | 2693ns | 8468ns | -1.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3643ns | 1347ns | 5077ns | base | 2.249 |
| warm-clamp-accfit | 3552ns | 1356ns | 5020ns | -2.51% | 2.307 |
| warm-clamp-accfit-dyn | 3599ns | 1712ns | 4756ns | -1.22% | 2.276 |
| warm-clamp-head | 3405ns | 1123ns | 4837ns | -6.55% | 2.406 |
| warm-clamp-min-lanes | 4346ns | 1792ns | 6355ns | +19.29% | 1.885 |
| warm-clamp-minimum | 3633ns | 2183ns | 8147ns | -0.27% | 2.255 |

## Performance model

- Peak throughput: **7.295 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.837 | 25.2% |
| warm-clamp-accfit | 2.035 | 27.9% |
| warm-clamp-accfit-dyn | 2.178 | 29.9% |
| warm-clamp-head | 2.093 | 28.7% |
| warm-clamp-min-lanes | 1.643 | 22.5% |
| warm-clamp-minimum | 3.582 | 49.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3829ns | 3829ns | base |
| warm-clamp-accfit | 3735ns | 3735ns | -2.46% |
| warm-clamp-accfit-dyn | 3783ns | 3783ns | -1.21% |
| warm-clamp-head | 3663ns | 3663ns | -4.34% |
| warm-clamp-min-lanes | 4524ns | 4524ns | +18.13% |
| warm-clamp-minimum | 3760ns | 3760ns | -1.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4460ns | base | --- | [3452, 4812] | --- | --- | --- | --- |
| warm-clamp-accfit | 4026ns | no significant difference | [-204, +31]ns | [3581, 4467] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-accfit-dyn | 3761ns | no significant difference | [-637, +11]ns | [3678, 4108] | no | 0.4470 | 0.2682 | 0 |
| warm-clamp-head | 3914ns | -427.7ns (-9.6%) | [-526, -258]ns | [3598, 4169] | YES | 0.0005 | 0.0002 | 0 |
| warm-clamp-min-lanes | 4986ns | +490.9ns (+11.0%) | [+431, +780]ns | [4213, 5439] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2287ns | no significant difference | [-1701, +918]ns | [2281, 2384] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4664ns | -19.1% | -20.9% | -11.7% | +5.2% | -53.4% |
| 1 | 2 | 4514ns | -25.0% | -11.6% | -8.1% | +14.5% | -49.3% |
| 1 | 3 | 3361ns | +33.1% | +11.8% | +7.8% | +46.1% | -32.5% |
| 1 | 4 | 4225ns | -4.5% | -9.0% | -10.3% | +36.1% | -43.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.141 | ok |
| warm-clamp-accfit | -0.243 | moderate- |
| warm-clamp-accfit-dyn | -0.684 | HIGH- (thermal bounce) |
| warm-clamp-head | 0.088 | ok |
| warm-clamp-min-lanes | -0.311 | moderate- |
| warm-clamp-minimum | -0.087 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 65.6ns | 3643.1ns | 1.8% |  |
| warm-clamp-accfit | 63.7ns | 3551.5ns | 1.8% |  |
| warm-clamp-accfit-dyn | 63.9ns | 3598.6ns | 1.8% |  |
| warm-clamp-head | 69.1ns | 3404.5ns | 2.0% |  |
| warm-clamp-min-lanes | 57.8ns | 4345.8ns | 1.3% |  |
| warm-clamp-minimum | 38.7ns | 3633.3ns | 1.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1346.5-5077.4 ns)
   1346.5 |########################################
   1533.1 |#####
   1719.6 |
   1906.2 |#####
   2092.7 |
   2279.2 |
   2465.8 |
   2652.3 |
   2838.9 |##########
   3025.4 |
   3212.0 |
   3398.5 |
   3585.0 |
   3771.6 |#####
   3958.1 |###############
   4144.7 |
   4331.2 |###############
   4517.7 |##########
   4704.3 |##############################
   4890.8 |########################################
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 1356.1-5020.1 ns)
   1356.1 |###################################
   1539.3 |#####
   1722.5 |
   1905.7 |
   2088.9 |#####
   2272.1 |
   2455.3 |
   2638.5 |
   2821.7 |
   3004.9 |#####
   3188.1 |
   3371.3 |##########
   3554.5 |##########
   3737.7 |###############
   3920.9 |#########################
   4104.1 |#####
   4287.3 |#####
   4470.5 |####################
   4653.7 |
   4836.9 |########################################
  (2 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1711.5-4756.3 ns)
   1711.5 |
   1863.8 |
   2016.0 |
   2168.2 |
   2320.5 |#####
   2472.7 |
   2625.0 |
   2777.2 |
   2929.4 |
   3081.7 |##########
   3233.9 |#####
   3386.2 |##########
   3538.4 |###############
   3690.6 |########################################
   3842.9 |##########
   3995.1 |##########
   4147.4 |##########
   4299.6 |###############
   4451.8 |###############
   4604.1 |
  (6 below, 5 above range)

warm-clamp-head (n=40, range 1122.9-4836.9 ns)
   1122.9 |#########################
   1308.6 |
   1494.3 |
   1680.0 |#####
   1865.7 |
   2051.4 |#####
   2237.1 |#####
   2422.8 |
   2608.5 |
   2794.2 |
   2979.9 |
   3165.6 |##########
   3351.3 |
   3537.0 |##############################
   3722.7 |
   3908.4 |###################################
   4094.1 |#####
   4279.8 |########################################
   4465.5 |#####
   4651.2 |##########
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 1791.7-6355.2 ns)
   1791.7 |########################################
   2019.9 |#####
   2248.1 |#####
   2476.2 |
   2704.4 |
   2932.6 |
   3160.7 |
   3388.9 |
   3617.1 |
   3845.3 |#####
   4073.4 |
   4301.6 |#####
   4529.8 |#####
   4757.9 |####################
   4986.1 |##########
   5214.3 |####################
   5442.5 |###############
   5670.6 |#####
   5898.8 |###############
   6127.0 |####################
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 2182.6-8147.1 ns)
   2182.6 |########################################
   2480.8 |
   2779.0 |
   3077.2 |
   3375.5 |
   3673.7 |
   3971.9 |
   4270.2 |
   4568.4 |
   4866.6 |
   5164.8 |
   5463.1 |#
   5761.3 |
   6059.5 |#
   6357.8 |
   6656.0 |
   6954.2 |
   7252.4 |#
   7550.7 |###
   7848.9 |#
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=42.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=39.1% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=30.0% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=39.8% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=41.4% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=66.1% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
