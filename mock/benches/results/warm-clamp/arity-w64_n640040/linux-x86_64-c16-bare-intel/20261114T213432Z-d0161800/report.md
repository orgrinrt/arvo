# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (4.68 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit-dyn at 1.79 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit-dyn dominates: 71% faster than the next best (warm-clamp-head)

warm-clamp-accfit-dyn (1.79 us) leads warm-clamp-head (3.07 us) by 71%, a clear separation rather than a photo finish. CV 111.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-acc64 is an outlier: 2.6x slower than the field

warm-clamp-acc64 (4.68 us) is 2.6x the fastest (1.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 111.0%)

warm-clamp-accfit-dyn wins on median (1.79 us) yet has the highest variance (CV 111.0%), while warm-clamp-acc64 is the steadiest (CV 38.9%, 4.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.78)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit-dyn} vs {warm-clamp-head, warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64} (71% apart)

The field splits into a fast tier {warm-clamp-accfit-dyn} and a slow tier {warm-clamp-head, warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64} with a 71% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit is inconsistent: worst-20% is 4.8x its best-20%

warm-clamp-accfit's best 20% of batches run at 1.28 us but its worst 20% at 6.16 us (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 1791.2 ns median (-61.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.61x (fastest 1791.2 ns, slowest 4675.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3892ns | 4934ns | 1640ns | 3911ns | 6085ns | base |
| warm-clamp-accfit | 3932ns | 4372ns | 1357ns | 3923ns | 6533ns | +1.03% |
| warm-clamp-accfit-dyn | 3513ns | 1880ns | 1447ns | 3140ns | 6697ns | -9.73% |
| warm-clamp-head | 4847ns | 3155ns | 3134ns | 3746ns | 9862ns | +24.54% |
| warm-clamp-min-lanes | 3604ns | 4014ns | 1490ns | 3426ns | 6253ns | -7.38% |
| warm-clamp-minimum | 3692ns | 4270ns | 1511ns | 3684ns | 5897ns | -5.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3674ns | 1552ns | 5717ns | base | 2.230 |
| warm-clamp-accfit | 3719ns | 1280ns | 6158ns | +1.22% | 2.203 |
| warm-clamp-accfit-dyn | 3326ns | 1377ns | 6335ns | -9.48% | 2.463 |
| warm-clamp-head | 4703ns | 3047ns | 9539ns | +28.01% | 1.742 |
| warm-clamp-min-lanes | 3389ns | 1405ns | 5856ns | -7.76% | 2.417 |
| warm-clamp-minimum | 3479ns | 1425ns | 5514ns | -5.31% | 2.355 |

## Performance model

- Peak throughput: **6.400 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.752 | 27.4% |
| warm-clamp-accfit | 1.969 | 30.8% |
| warm-clamp-accfit-dyn | 4.574 | 71.5% |
| warm-clamp-head | 2.670 | 41.7% |
| warm-clamp-min-lanes | 2.170 | 33.9% |
| warm-clamp-minimum | 2.023 | 31.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3892ns | 3892ns | base |
| warm-clamp-accfit | 3932ns | 3932ns | +1.03% |
| warm-clamp-accfit-dyn | 3513ns | 3513ns | -9.73% |
| warm-clamp-head | 4847ns | 4847ns | +24.54% |
| warm-clamp-min-lanes | 3604ns | 3604ns | -7.38% |
| warm-clamp-minimum | 3692ns | 3692ns | -5.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4675ns | base | --- | [1572, 5074] | --- | --- | --- | --- |
| warm-clamp-accfit | 4160ns | no significant difference | [-308, +307]ns | [1895, 4841] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 1791ns | no significant difference | [-271, +114]ns | [1674, 4517] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-head | 3069ns | +1498.4ns (+32.0%) | [+865, +1510]ns | [3061, 3077] | YES (adj: no) | 0.1924 | 0.0385 | 0 |
| warm-clamp-min-lanes | 3776ns | no significant difference | [-592, +2]ns | [1556, 4496] | no | 0.5370 | 0.2682 | 0 |
| warm-clamp-minimum | 4050ns | no significant difference | [-304, +10]ns | [1600, 4745] | no | 0.5370 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3809ns | -19.7% | -64.0% | -19.6% | -59.0% | -11.6% |
| 1 | 2 | 3011ns | +51.4% | +46.1% | +1.9% | +50.6% | -47.8% |
| 1 | 3 | 3940ns | +5.6% | -57.5% | -22.0% | -60.3% | +6.6% |
| 1 | 4 | 5046ns | -31.6% | -38.0% | -39.3% | -17.5% | -5.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.063 | ok |
| warm-clamp-accfit | -0.307 | moderate- |
| warm-clamp-accfit-dyn | -0.750 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.252 | moderate- |
| warm-clamp-min-lanes | -0.776 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.035 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 57.4ns | 3674.2ns | 1.6% |  |
| warm-clamp-accfit | 57.8ns | 3719.1ns | 1.6% |  |
| warm-clamp-accfit-dyn | 51.6ns | 3325.9ns | 1.6% |  |
| warm-clamp-head | 37.5ns | 4703.4ns | 0.8% |  |
| warm-clamp-min-lanes | 58.0ns | 3388.9ns | 1.7% |  |
| warm-clamp-minimum | 56.6ns | 3479.1ns | 1.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1551.9-5717.1 ns)
   1551.9 |########################################
   1760.2 |###
   1968.5 |
   2176.7 |
   2385.0 |###
   2593.2 |
   2801.5 |
   3009.8 |
   3218.0 |###
   3426.3 |
   3634.5 |
   3842.8 |
   4051.0 |
   4259.3 |
   4467.6 |######
   4675.8 |######
   4884.1 |################
   5092.3 |######
   5300.6 |###
   5508.8 |#######################
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 1280.0-6158.0 ns)
   1280.0 |#################################
   1523.9 |#################################
   1767.8 |
   2011.7 |######
   2255.6 |
   2499.5 |
   2743.4 |######
   2987.3 |
   3231.2 |
   3475.1 |
   3719.0 |#############
   3962.9 |#############
   4206.8 |#############
   4450.7 |####################
   4694.6 |#############
   4938.5 |######
   5182.4 |#############
   5426.3 |######
   5670.2 |######
   5914.1 |########################################
  (4 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1377.4-6335.0 ns)
   1377.4 |######
   1625.3 |########################################
   1873.1 |###
   2121.0 |
   2368.9 |
   2616.8 |
   2864.7 |
   3112.5 |###
   3360.4 |
   3608.3 |
   3856.2 |###
   4104.1 |######
   4351.9 |######
   4599.8 |##########
   4847.7 |
   5095.6 |######
   5343.5 |###
   5591.3 |###
   5839.2 |######
   6087.1 |
  (6 below, 4 above range)

warm-clamp-head (n=40, range 3047.0-9538.8 ns)
   3047.0 |########################################
   3371.6 |
   3696.2 |
   4020.8 |###
   4345.4 |
   4670.0 |
   4994.6 |#
   5319.1 |
   5643.7 |
   5968.3 |
   6292.9 |
   6617.5 |
   6942.1 |#
   7266.7 |
   7591.2 |
   7915.8 |
   8240.4 |#
   8565.0 |#
   8889.6 |###
   9214.2 |#
  (6 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1404.7-5856.0 ns)
   1404.7 |########################################
   1627.3 |
   1849.8 |##
   2072.4 |
   2295.0 |
   2517.5 |
   2740.1 |
   2962.7 |
   3185.2 |
   3407.8 |##
   3630.4 |#####
   3852.9 |
   4075.5 |########
   4298.1 |#####
   4520.6 |########
   4743.2 |##
   4965.8 |
   5188.3 |########
   5410.9 |#####
   5633.5 |#####
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 1425.4-5513.8 ns)
   1425.4 |########################################
   1629.8 |
   1834.2 |
   2038.7 |
   2243.1 |###
   2447.5 |
   2651.9 |
   2856.3 |
   3060.8 |#######
   3265.2 |
   3469.6 |
   3674.0 |###
   3878.5 |#######
   4082.9 |
   4287.3 |##########
   4491.7 |
   4696.1 |#####################
   4900.6 |#######
   5105.0 |##########
   5309.4 |###
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=49.5% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=51.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=59.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-clamp-head**: CV=56.7% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=53.6% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=48.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
