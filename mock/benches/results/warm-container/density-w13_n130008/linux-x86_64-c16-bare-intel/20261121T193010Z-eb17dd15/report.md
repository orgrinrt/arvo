# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred dominates: 24% faster than the next best (warm-container-native)

warm-container-lanes-deferred (2.16 us) leads warm-container-native (2.68 us) by 24%, a clear separation rather than a photo finish. CV 105.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-lanes-deferred beats baseline by 41% (significant)

warm-container-lanes-deferred is -1.78 us (41%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is an outlier: 2.3x slower than the field

warm-container-kernel (5.02 us) is 2.3x the fastest (2.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred is fastest but the noisiest (CV 105.5%)

warm-container-lanes-deferred wins on median (2.16 us) yet has the highest variance (CV 105.5%), while warm-container-native is the steadiest (CV 9.8%, 2.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.74)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-native} vs {warm-container-plusone, warm-container-minimum, warm-container-headroom, warm-container-kernel} (61% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-plusone, warm-container-minimum, warm-container-headroom, warm-container-kernel} with a 61% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-kernel is inconsistent: worst-20% is 5.2x its best-20%

warm-container-kernel's best 20% of batches run at 1.76 us but its worst 20% at 9.11 us (5.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-17 ns, 0.40%)

warm-container-plusone differs from baseline warm-container-headroom by -17 ns (0.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 2155.7 ns median (-50.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.33x (fastest 2155.7 ns, slowest 5022.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 13 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112823892 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5018ns | 4391ns | 3596ns | 4151ns | 9039ns | base |
| warm-container-kernel | 4822ns | 5167ns | 1813ns | 4296ns | 9407ns | -3.91% |
| warm-container-lanes-deferred | 3587ns | 2218ns | 1812ns | 2811ns | 7691ns | -28.51% |
| warm-container-minimum | 5315ns | 4383ns | 4371ns | 4380ns | 9064ns | +5.92% |
| warm-container-native | 2914ns | 2862ns | 2588ns | 2868ns | 3379ns | -41.92% |
| warm-container-plusone | 4875ns | 4369ns | 3578ns | 4355ns | 7732ns | -2.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4935ns | 3545ns | 8860ns | base | 14.939 |
| warm-container-kernel | 4677ns | 1762ns | 9110ns | -5.23% | 15.764 |
| warm-container-lanes-deferred | 3477ns | 1761ns | 7427ns | -29.55% | 21.207 |
| warm-container-minimum | 5228ns | 4308ns | 8884ns | +5.94% | 14.101 |
| warm-container-native | 2713ns | 2417ns | 3119ns | -45.03% | 27.177 |
| warm-container-plusone | 4796ns | 3526ns | 7578ns | -2.81% | 15.372 |

## Performance model

- Peak throughput: **41.863 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.047 | 40.7% |
| warm-container-kernel | 14.679 | 35.1% |
| warm-container-lanes-deferred | 34.201 | 81.7% |
| warm-container-minimum | 17.064 | 40.8% |
| warm-container-native | 27.554 | 65.8% |
| warm-container-plusone | 17.122 | 40.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5018ns | 5018ns | base |
| warm-container-kernel | 4822ns | 4822ns | -3.91% |
| warm-container-lanes-deferred | 3587ns | 3587ns | -28.51% |
| warm-container-minimum | 5315ns | 5315ns | +5.92% |
| warm-container-native | 2914ns | 2914ns | -41.92% |
| warm-container-plusone | 4875ns | 4875ns | -2.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4325ns | base | --- | [3560, 4333] | --- | --- | --- | --- |
| warm-container-kernel | 5023ns | no significant difference | [-1793, +687]ns | [1816, 5716] | no | 0.7948 | 0.6358 | 0 |
| warm-container-lanes-deferred | 2156ns | -1781.7ns (-41.2%) | [-2166, -1391]ns | [2151, 2794] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4321ns | no significant difference | [-12, +764]ns | [4310, 4322] | no | 0.8746 | 0.8746 | 0 |
| warm-container-native | 2676ns | -1672.3ns (-38.7%) | [-1858, -1086]ns | [2517, 2802] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4306ns | -17.4ns (-0.4%) | [-26, -11]ns | [4303, 4315] | YES | 0.0276 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4344ns | -25.2% | -59.4% | -0.7% | -42.6% | -0.8% |
| 1 | 2 | 3555ns | +91.4% | -50.2% | +21.5% | -14.0% | +21.3% |
| 1 | 3 | 4330ns | +16.0% | -50.1% | -0.2% | -41.9% | -0.3% |
| 1 | 4 | 3551ns | +4.2% | -39.2% | +21.6% | -23.6% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.744 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.351 | moderate- |
| warm-container-lanes-deferred | 0.255 | moderate+ |
| warm-container-minimum | -0.225 | moderate- |
| warm-container-native | -0.693 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.089 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 1/4, lost 3/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 2/4, lost 2/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 27.6ns | 4935.1ns | 0.6% |  |
| warm-container-kernel | 51.4ns | 4677.0ns | 1.1% |  |
| warm-container-lanes-deferred | 38.9ns | 3476.6ns | 1.1% |  |
| warm-container-minimum | 29.4ns | 5228.5ns | 0.6% |  |
| warm-container-native | 73.5ns | 2712.9ns | 2.7% |  |
| warm-container-plusone | 27.0ns | 4796.4ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3544.8-8860.3 ns)
   3544.8 |####################################
   3810.5 |###
   4076.3 |########################################
   4342.1 |############
   4607.9 |###
   4873.6 |
   5139.4 |
   5405.2 |
   5671.0 |
   5936.8 |
   6202.5 |
   6468.3 |
   6734.1 |
   6999.9 |
   7265.6 |
   7531.4 |
   7797.2 |###
   8063.0 |
   8328.8 |
   8594.5 |###
  (3 below, 4 above range)

warm-container-kernel (n=40, range 1761.7-9109.5 ns)
   1761.7 |########################################
   2129.1 |###
   2496.4 |
   2863.8 |
   3231.2 |
   3598.6 |
   3966.0 |
   4333.4 |
   4700.8 |##########
   5068.2 |##########
   5435.6 |#############
   5803.0 |#############
   6170.4 |######
   6537.8 |######
   6905.1 |
   7272.5 |
   7639.9 |###
   8007.3 |###
   8374.7 |###
   8742.1 |###
  (4 below, 1 above range)

warm-container-lanes-deferred (n=40, range 1761.2-7427.5 ns)
   1761.2 |###########################
   2044.5 |########################################
   2327.8 |
   2611.1 |
   2894.4 |
   3177.7 |###
   3461.1 |###
   3744.4 |###
   4027.7 |
   4311.0 |###
   4594.3 |
   4877.6 |
   5161.0 |
   5444.3 |
   5727.6 |
   6010.9 |
   6294.2 |###
   6577.5 |######
   6860.9 |######
   7144.2 |######
  (4 below, 3 above range)

warm-container-minimum (n=40, range 4308.2-8883.5 ns)
   4308.2 |########################################
   4537.0 |
   4765.7 |
   4994.5 |
   5223.3 |
   5452.0 |
   5680.8 |
   5909.6 |
   6138.3 |
   6367.1 |
   6595.9 |
   6824.6 |
   7053.4 |
   7282.2 |
   7510.9 |
   7739.7 |
   7968.5 |
   8197.2 |#
   8426.0 |
   8654.7 |
  (1 below, 4 above range)

warm-container-native (n=40, range 2416.9-3119.1 ns)
   2416.9 |################################
   2452.0 |########################################
   2487.1 |########################
   2522.2 |########
   2557.4 |
   2592.5 |########
   2627.6 |################################
   2662.7 |################
   2697.8 |########################
   2732.9 |########
   2768.0 |########
   2803.1 |
   2838.2 |
   2873.3 |################################
   2908.4 |################
   2943.5 |################
   2978.6 |
   3013.7 |
   3048.8 |
   3083.9 |
  (2 below, 5 above range)

warm-container-plusone (n=40, range 3526.0-7577.7 ns)
   3526.0 |####
   3728.6 |
   3931.1 |#
   4133.7 |########################################
   4336.3 |#
   4538.9 |
   4741.5 |
   4944.1 |
   5146.7 |
   5349.2 |
   5551.8 |
   5754.4 |
   5957.0 |
   6159.6 |
   6362.2 |
   6564.8 |
   6767.3 |
   6969.9 |
   7172.5 |#
   7375.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=48.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=71.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=65.4% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-minimum**: CV=47.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=45.2% (high variance, measurements may be unstable)
