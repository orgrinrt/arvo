# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread is an outlier: 2.3x slower than the field

wide-rung-ragged-overread (4.01 us) is 2.3x the fastest (1.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### wide-rung-wordround-alias is fastest but the noisiest (CV 108.5%)

wide-rung-wordround-alias wins on median (1.76 us) yet has the highest variance (CV 108.5%), while wide-rung-ragged-overread is the steadiest (CV 39.3%, 4.01 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (wide-rung-wordround-alias, wide-rung-align16) are a dead heat (<1%)

wide-rung-wordround-alias (1.76 us) and wide-rung-align16 (1.76 us) differ by 0.19%, inside the noise, even though the wider field spreads 127.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.75)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-wordround-alias, wide-rung-align16, wide-rung-ragged} vs {wide-rung-wordround, wide-rung-ragged-overread} (111% apart)

The field splits into a fast tier {wide-rung-wordround-alias, wide-rung-align16, wide-rung-ragged} and a slow tier {wide-rung-wordround, wide-rung-ragged-overread} with a 111% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-wordround-alias is inconsistent: worst-20% is 4.1x its best-20%

wide-rung-wordround-alias's best 20% of batches run at 1.50 us but its worst 20% at 6.21 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (35 ns, 1.99%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by 35 ns (1.99%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1761.2 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.28x (fastest 1761.2 ns, slowest 4010.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112830321 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3315ns | 1846ns | 1646ns | 2952ns | 6074ns | base |
| wide-rung-ragged | 3440ns | 1870ns | 1567ns | 3086ns | 6376ns | +3.77% |
| wide-rung-ragged-overread | 3721ns | 4196ns | 1685ns | 3674ns | 5899ns | +12.23% |
| wide-rung-wordround | 3544ns | 3939ns | 1501ns | 3482ns | 5770ns | +6.88% |
| wide-rung-wordround-alias | 3145ns | 1844ns | 1571ns | 2531ns | 6561ns | -5.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3157ns | 1572ns | 5748ns | base | 0.649 |
| wide-rung-ragged | 3278ns | 1497ns | 6053ns | +3.84% | 0.625 |
| wide-rung-ragged-overread | 3547ns | 1610ns | 5600ns | +12.36% | 0.577 |
| wide-rung-wordround | 3376ns | 1434ns | 5459ns | +6.93% | 0.607 |
| wide-rung-wordround-alias | 2992ns | 1501ns | 6205ns | -5.24% | 0.685 |

## Performance model

- Peak throughput: **1.428 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.161 | 81.3% |
| wide-rung-ragged | 1.145 | 80.2% |
| wide-rung-ragged-overread | 0.511 | 35.8% |
| wide-rung-wordround | 0.544 | 38.1% |
| wide-rung-wordround-alias | 1.163 | 81.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3315ns | 3315ns | base |
| wide-rung-ragged | 3440ns | 3440ns | +3.77% |
| wide-rung-ragged-overread | 3721ns | 3721ns | +12.23% |
| wide-rung-wordround | 3544ns | 3544ns | +6.88% |
| wide-rung-wordround-alias | 3145ns | 3145ns | -5.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1765ns | base | --- | [1754, 4317] | --- | --- | --- | --- |
| wide-rung-ragged | 1789ns | no significant difference | [-123, +317]ns | [1780, 4955] | no | 0.1614 | 0.0807 | 0 |
| wide-rung-ragged-overread | 4011ns | +35.1ns (+2.0%) | [+11, +268]ns | [1888, 4444] | YES (adj: no) | 0.1539 | 0.0385 | 0 |
| wide-rung-wordround | 3768ns | no significant difference | [-156, +298]ns | [1755, 4548] | no | 0.3576 | 0.2682 | 0 |
| wide-rung-wordround-alias | 1761ns | no significant difference | [-315, +20]ns | [1760, 2626] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1735ns | +37.6% | +109.4% | +156.7% | -18.1% |
| 1 | 2 | 3668ns | -43.2% | -18.8% | -50.6% | -40.0% |
| 1 | 3 | 1759ns | -19.1% | +144.1% | +143.9% | +0.1% |
| 1 | 4 | 2721ns | -34.3% | -33.9% | -35.3% | -35.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.751 | HIGH- (thermal bounce) |
| wide-rung-ragged | 0.119 | ok |
| wide-rung-ragged-overread | -0.545 | HIGH- (thermal bounce) |
| wide-rung-wordround | -0.727 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.526 | HIGH- (thermal bounce) |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 2/4, lost 2/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 44.9ns | 3156.8ns | 1.4% |  |
| wide-rung-ragged | 48.1ns | 3277.9ns | 1.5% |  |
| wide-rung-ragged-overread | 49.7ns | 3547.0ns | 1.4% |  |
| wide-rung-wordround | 47.7ns | 3375.6ns | 1.4% |  |
| wide-rung-wordround-alias | 42.4ns | 2991.5ns | 1.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1572.3-5748.4 ns)
   1572.3 |########################################
   1781.1 |
   1989.9 |
   2198.7 |
   2407.5 |
   2616.3 |
   2825.1 |
   3033.9 |##
   3242.7 |
   3451.5 |
   3660.3 |##
   3869.1 |
   4077.9 |##
   4286.7 |######
   4495.5 |##
   4704.3 |######
   4913.2 |##
   5122.0 |####
   5330.8 |
   5539.6 |##
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 1497.1-6052.9 ns)
   1497.1 |
   1724.9 |########################################
   1952.7 |
   2180.5 |
   2408.3 |##
   2636.1 |
   2863.8 |##
   3091.6 |
   3319.4 |
   3547.2 |
   3775.0 |##
   4002.8 |
   4230.6 |
   4458.4 |
   4686.1 |
   4913.9 |#####
   5141.7 |##########
   5369.5 |##
   5597.3 |#######
   5825.1 |#####
  (6 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1609.6-5600.3 ns)
   1609.6 |########################################
   1809.1 |########
   2008.7 |
   2208.2 |####
   2407.8 |
   2607.3 |
   2806.8 |
   3006.4 |
   3205.9 |
   3405.4 |
   3605.0 |####
   3804.5 |########
   4004.0 |#############
   4203.6 |#################
   4403.1 |#############
   4602.6 |####
   4802.2 |#############
   5001.7 |####
   5201.3 |####
   5400.8 |####
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 1434.5-5458.9 ns)
   1434.5 |########################################
   1635.7 |##################################
   1836.9 |
   2038.1 |#####
   2239.3 |
   2440.6 |
   2641.8 |#####
   2843.0 |
   3044.2 |
   3245.4 |#####
   3446.7 |#####
   3647.9 |
   3849.1 |
   4050.3 |###########
   4251.5 |###########
   4452.8 |######################
   4654.0 |###########
   4855.2 |######################
   5056.4 |
   5257.6 |#################
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 1500.8-6205.0 ns)
   1500.8 |
   1736.0 |########################################
   1971.2 |
   2206.4 |
   2441.6 |####
   2676.8 |
   2912.0 |
   3147.3 |##
   3382.5 |
   3617.7 |
   3852.9 |
   4088.1 |
   4323.3 |##
   4558.5 |
   4793.7 |####
   5028.9 |##
   5264.2 |##
   5499.4 |####
   5734.6 |##
   5969.8 |
  (6 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=54.9% (high variance, measurements may be unstable)
- **wide-rung-align16**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **wide-rung-ragged**: CV=58.7% (high variance, measurements may be unstable)
- **wide-rung-ragged**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **wide-rung-ragged-overread**: CV=44.4% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **wide-rung-wordround**: CV=48.8% (high variance, measurements may be unstable)
- **wide-rung-wordround**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **wide-rung-wordround-alias**: CV=63.9% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 4.1x (possible bimodal distribution)
