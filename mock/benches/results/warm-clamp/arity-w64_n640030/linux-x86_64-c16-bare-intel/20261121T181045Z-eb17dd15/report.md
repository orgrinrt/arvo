# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 17% faster than the next best (warm-clamp-acc64)

warm-clamp-head (3.03 us) leads warm-clamp-acc64 (3.53 us) by 17%, a clear separation rather than a photo finish. CV 75.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head is fastest but the noisiest (CV 75.6%)

warm-clamp-head wins on median (3.03 us) yet has the highest variance (CV 75.6%), while warm-clamp-min-lanes is the steadiest (CV 37.4%, 4.45 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.42 us) is smaller than the fastest variant's own run-to-run std-dev (2.29 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.2x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.37 us but its worst 20% at 5.69 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 3031.6 ns median (-14.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.47x (fastest 3031.6 ns, slowest 4454.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112579971 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3623ns | 3719ns | 1531ns | 3445ns | 6251ns | base |
| warm-clamp-accfit | 3640ns | 3895ns | 1596ns | 3468ns | 6199ns | +0.46% |
| warm-clamp-accfit-dyn | 3748ns | 4134ns | 1444ns | 3759ns | 6022ns | +3.46% |
| warm-clamp-head | 3761ns | 3113ns | 2101ns | 2933ns | 7903ns | +3.79% |
| warm-clamp-min-lanes | 4024ns | 4691ns | 1630ns | 4134ns | 6088ns | +11.07% |
| warm-clamp-minimum | 3546ns | 3955ns | 1476ns | 3432ns | 5958ns | -2.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3422ns | 1450ns | 5878ns | base | 2.394 |
| warm-clamp-accfit | 3459ns | 1522ns | 5870ns | +1.07% | 2.368 |
| warm-clamp-accfit-dyn | 3555ns | 1369ns | 5693ns | +3.88% | 2.304 |
| warm-clamp-head | 3647ns | 2024ns | 7643ns | +6.56% | 2.246 |
| warm-clamp-min-lanes | 3809ns | 1547ns | 5741ns | +11.29% | 2.151 |
| warm-clamp-minimum | 3363ns | 1404ns | 5627ns | -1.73% | 2.436 |

## Performance model

- Peak throughput: **5.984 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.319 | 38.8% |
| warm-clamp-accfit | 2.206 | 36.9% |
| warm-clamp-accfit-dyn | 2.079 | 34.7% |
| warm-clamp-head | 2.702 | 45.2% |
| warm-clamp-min-lanes | 1.839 | 30.7% |
| warm-clamp-minimum | 2.178 | 36.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3623ns | 3623ns | base |
| warm-clamp-accfit | 3640ns | 3640ns | +0.46% |
| warm-clamp-accfit-dyn | 3748ns | 3748ns | +3.46% |
| warm-clamp-head | 3761ns | 3761ns | +3.79% |
| warm-clamp-min-lanes | 4024ns | 4024ns | +11.07% |
| warm-clamp-minimum | 3546ns | 3546ns | -2.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3532ns | base | --- | [1605, 4642] | --- | --- | --- | --- |
| warm-clamp-accfit | 3714ns | no significant difference | [-184, +220]ns | [1674, 4655] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-accfit-dyn | 3940ns | no significant difference | [-131, +373]ns | [3434, 4303] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-head | 3032ns | no significant difference | [-487, +1396]ns | [2492, 3044] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-min-lanes | 4454ns | +67.2ns (+1.9%) | [+40, +463]ns | [3248, 4869] | YES | 0.0321 | 0.0064 | 0 |
| warm-clamp-minimum | 3762ns | no significant difference | [-258, +12]ns | [1460, 4443] | no | 0.2017 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4321ns | -32.1% | +8.3% | -29.6% | -25.5% | -55.5% |
| 1 | 2 | 1680ns | +172.3% | +138.7% | +81.2% | +169.8% | +52.0% |
| 1 | 3 | 3573ns | +1.4% | +7.5% | -30.3% | +18.1% | +9.9% |
| 1 | 4 | 3985ns | -57.9% | -3.5% | -48.1% | +15.0% | +5.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.430 | moderate- |
| warm-clamp-accfit | -0.097 | ok |
| warm-clamp-accfit-dyn | 0.076 | ok |
| warm-clamp-head | 0.271 | moderate+ |
| warm-clamp-min-lanes | -0.245 | moderate- |
| warm-clamp-minimum | 0.303 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 3/4, lost 1/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 52.5ns | 3422.4ns | 1.5% |  |
| warm-clamp-accfit | 51.0ns | 3459.0ns | 1.5% |  |
| warm-clamp-accfit-dyn | 56.3ns | 3555.4ns | 1.6% |  |
| warm-clamp-head | 30.9ns | 3647.0ns | 0.8% |  |
| warm-clamp-min-lanes | 57.6ns | 3808.7ns | 1.5% |  |
| warm-clamp-minimum | 47.6ns | 3363.2ns | 1.4% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1450.2-5878.2 ns)
   1450.2 |########################################
   1671.6 |######
   1893.0 |###
   2114.4 |###
   2335.8 |
   2557.2 |
   2778.6 |
   3000.0 |
   3221.4 |
   3442.8 |######
   3664.2 |
   3885.6 |######
   4107.0 |######
   4328.4 |
   4549.8 |######
   4771.2 |
   4992.6 |######
   5214.0 |###
   5435.4 |#############
   5656.8 |#############
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 1522.2-5870.4 ns)
   1522.2 |########################################
   1739.6 |
   1957.0 |
   2174.4 |
   2391.8 |
   2609.2 |##
   2826.7 |
   3044.1 |##
   3261.5 |
   3478.9 |
   3696.3 |
   3913.7 |
   4131.1 |##
   4348.5 |#####
   4566.0 |###########
   4783.4 |########
   5000.8 |#####
   5218.2 |##
   5435.6 |##
   5653.0 |##
  (4 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 1368.9-5693.3 ns)
   1368.9 |########################################
   1585.1 |##########################
   1801.4 |
   2017.6 |
   2233.8 |
   2450.0 |
   2666.2 |
   2882.5 |
   3098.7 |######
   3314.9 |
   3531.1 |######
   3747.3 |#################################
   3963.5 |##########################
   4179.8 |#################################
   4396.0 |######
   4612.2 |#############
   4828.4 |#############
   5044.6 |######
   5260.9 |
   5477.1 |######
  (3 below, 4 above range)

warm-clamp-head (n=40, range 2023.8-7642.5 ns)
   2023.8 |#####
   2304.8 |######################
   2585.7 |
   2866.6 |########################################
   3147.6 |
   3428.5 |
   3709.4 |
   3990.4 |
   4271.3 |##
   4552.2 |
   4833.2 |##
   5114.1 |
   5395.0 |
   5676.0 |
   5956.9 |##
   6237.8 |
   6518.8 |##
   6799.7 |
   7080.6 |
   7361.6 |##
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1547.0-5740.7 ns)
   1547.0 |########################################
   1756.7 |
   1966.3 |
   2176.0 |
   2385.7 |
   2595.4 |
   2805.1 |
   3014.8 |###
   3224.5 |###
   3434.1 |
   3643.8 |
   3853.5 |#######
   4063.2 |###
   4272.9 |##########
   4482.6 |##########
   4692.3 |##########
   4901.9 |#######
   5111.6 |###
   5321.3 |##########
   5531.0 |#######
  (2 below, 5 above range)

warm-clamp-minimum (n=40, range 1403.8-5627.1 ns)
   1403.8 |########################################
   1614.9 |
   1826.1 |
   2037.3 |
   2248.4 |###
   2459.6 |
   2670.8 |###
   2881.9 |
   3093.1 |
   3304.3 |###
   3515.4 |###
   3726.6 |######
   3937.8 |######
   4148.9 |###
   4360.1 |######
   4571.3 |
   4782.5 |######
   4993.6 |######
   5204.8 |#############
   5416.0 |######
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=54.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=51.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=45.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-clamp-head**: CV=62.9% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=43.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=52.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
