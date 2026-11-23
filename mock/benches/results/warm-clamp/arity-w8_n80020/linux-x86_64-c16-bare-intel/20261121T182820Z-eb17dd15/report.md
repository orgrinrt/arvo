# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head is an outlier: 2.3x slower than the field

warm-clamp-head (5.50 us) is 2.3x the fastest (2.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-minimum, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-minimum (2.40 us) and warm-clamp-min-lanes (2.40 us) differ by 0.20%, inside the noise, even though the wider field spreads 129.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.65)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} (56% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-head} with a 56% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 2.13 us but its worst 20% at 8.22 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-minimum's edge over baseline is significant but tiny (25 ns, 0.55%)

warm-clamp-minimum differs from baseline warm-clamp-acc64 by 25 ns (0.55%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 2398.7 ns median (-47.4% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.29x (fastest 2398.7 ns, slowest 5496.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112755372 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4014ns | 4736ns | 1689ns | 3936ns | 6571ns | base |
| warm-clamp-accfit | 3580ns | 3923ns | 1479ns | 3614ns | 5577ns | -10.81% |
| warm-clamp-accfit-dyn | 3527ns | 4038ns | 1516ns | 3475ns | 5694ns | -12.13% |
| warm-clamp-head | 4565ns | 5644ns | 1963ns | 4442ns | 7535ns | +13.74% |
| warm-clamp-min-lanes | 4116ns | 2467ns | 2182ns | 3304ns | 8488ns | +2.55% |
| warm-clamp-minimum | 4027ns | 2462ns | 2014ns | 3546ns | 7483ns | +0.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3861ns | 1629ns | 6292ns | base | 2.122 |
| warm-clamp-accfit | 3421ns | 1418ns | 5305ns | -11.38% | 2.394 |
| warm-clamp-accfit-dyn | 3369ns | 1454ns | 5409ns | -12.74% | 2.432 |
| warm-clamp-head | 4433ns | 1911ns | 7293ns | +14.83% | 1.848 |
| warm-clamp-min-lanes | 3999ns | 2126ns | 8225ns | +3.58% | 2.049 |
| warm-clamp-minimum | 3913ns | 1962ns | 7240ns | +1.36% | 2.093 |

## Performance model

- Peak throughput: **5.779 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.795 | 31.1% |
| warm-clamp-accfit | 2.178 | 37.7% |
| warm-clamp-accfit-dyn | 2.116 | 36.6% |
| warm-clamp-head | 1.490 | 25.8% |
| warm-clamp-min-lanes | 3.408 | 59.0% |
| warm-clamp-minimum | 3.415 | 59.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4014ns | 4014ns | base |
| warm-clamp-accfit | 3580ns | 3580ns | -10.81% |
| warm-clamp-accfit-dyn | 3527ns | 3527ns | -12.13% |
| warm-clamp-head | 4565ns | 4565ns | +13.74% |
| warm-clamp-min-lanes | 4116ns | 4116ns | +2.55% |
| warm-clamp-minimum | 4027ns | 4027ns | +0.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 4564ns | base | --- | [1952, 5235] | --- | --- | --- | --- |
| warm-clamp-accfit | 3761ns | -500.8ns (-11.0%) | [-941, -179]ns | [2563, 4507] | YES | 0.0003 | 0.0002 | 0 |
| warm-clamp-accfit-dyn | 3872ns | -471.9ns (-10.3%) | [-502, -149]ns | [1708, 4394] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 5497ns | +384.4ns (+8.4%) | [+340, +573]ns | [2340, 5924] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2404ns | no significant difference | [-8, +627]ns | [2397, 3621] | no | 0.0807 | 0.0807 | 0 |
| warm-clamp-minimum | 2399ns | +25.3ns (+0.6%) | [+8, +393]ns | [1973, 5209] | YES | 0.0207 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3638ns | -13.7% | +23.3% | -7.5% | -33.9% | -39.9% |
| 1 | 2 | 4795ns | -27.3% | -11.5% | -60.1% | -49.8% | -49.8% |
| 1 | 3 | 4583ns | -6.0% | -62.8% | +31.3% | -27.3% | +13.7% |
| 1 | 4 | 2093ns | +113.4% | +79.9% | +12.3% | +14.6% | -6.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.150 | ok |
| warm-clamp-accfit | 0.305 | moderate+ |
| warm-clamp-accfit-dyn | -0.215 | moderate- |
| warm-clamp-head | -0.650 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.418 | moderate- |
| warm-clamp-minimum | -0.435 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 49.0ns | 3860.7ns | 1.3% |  |
| warm-clamp-accfit | 56.0ns | 3421.3ns | 1.6% |  |
| warm-clamp-accfit-dyn | 55.9ns | 3368.9ns | 1.7% |  |
| warm-clamp-head | 44.1ns | 4433.3ns | 1.0% |  |
| warm-clamp-min-lanes | 39.1ns | 3998.9ns | 1.0% |  |
| warm-clamp-minimum | 38.6ns | 3913.4ns | 1.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1628.8-6291.9 ns)
   1628.8 |
   1862.0 |########################################
   2095.1 |####
   2328.3 |####
   2561.4 |
   2794.6 |
   3027.7 |
   3260.9 |####
   3494.0 |
   3727.2 |
   3960.4 |
   4193.5 |
   4426.7 |####
   4659.8 |########
   4893.0 |#############
   5126.1 |########
   5359.3 |########
   5592.4 |######################
   5825.6 |####
   6058.8 |####
  (7 below, 4 above range)

warm-clamp-accfit (n=40, range 1417.6-5304.7 ns)
   1417.6 |########################################
   1612.0 |
   1806.3 |
   2000.7 |
   2195.0 |###
   2389.4 |
   2583.7 |
   2778.1 |###
   2972.4 |
   3166.8 |###
   3361.1 |##########
   3555.5 |
   3749.8 |##########
   3944.2 |###
   4138.6 |######
   4332.9 |###
   4527.3 |###
   4721.6 |#############
   4916.0 |###
   5110.3 |####################
  (1 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1453.9-5408.7 ns)
   1453.9 |########################################
   1651.6 |
   1849.3 |###
   2047.1 |###
   2244.8 |
   2442.6 |
   2640.3 |###
   2838.1 |
   3035.8 |
   3233.6 |
   3431.3 |
   3629.0 |##########
   3826.8 |###
   4024.5 |##########
   4222.3 |##########
   4420.0 |###
   4617.8 |##########
   4815.5 |###
   5013.2 |###
   5211.0 |##################
  (3 below, 2 above range)

warm-clamp-head (n=40, range 1911.4-7292.7 ns)
   1911.4 |########################################
   2180.5 |########################################
   2449.6 |######
   2718.6 |
   2987.7 |
   3256.8 |
   3525.8 |
   3794.9 |
   4063.9 |
   4333.0 |
   4602.1 |######
   4871.1 |
   5140.2 |######
   5409.3 |##########################
   5678.3 |#############
   5947.4 |##########################
   6216.5 |#############
   6485.5 |#############
   6754.6 |######
   7023.6 |#############
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2126.5-8224.7 ns)
   2126.5 |########################################
   2431.4 |
   2736.3 |
   3041.2 |
   3346.1 |
   3651.0 |
   3955.9 |
   4260.8 |
   4565.7 |###
   4870.7 |
   5175.6 |
   5480.5 |#
   5785.4 |#
   6090.3 |#
   6395.2 |#
   6700.1 |#
   7005.0 |
   7309.9 |
   7614.8 |###
   7919.8 |
  (5 below, 5 above range)

warm-clamp-minimum (n=40, range 1962.1-7240.4 ns)
   1962.1 |########################################
   2226.0 |#############################
   2490.0 |
   2753.9 |
   3017.8 |
   3281.7 |
   3545.6 |
   3809.5 |
   4073.4 |
   4337.3 |###
   4601.3 |#######
   4865.2 |
   5129.1 |
   5393.0 |###
   5656.9 |#######
   5920.8 |###
   6184.7 |#######
   6448.7 |##########
   6712.6 |#######
   6976.5 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=49.8% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=45.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=47.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-head**: CV=50.0% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=61.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=56.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 3.7x (possible bimodal distribution)
