# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (3.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-pipe2 at 2.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (bitpack-contend-pipe2, bitpack-contend-pipe4) are a dead heat (<1%)

bitpack-contend-pipe2 (2.46 us) and bitpack-contend-pipe4 (2.47 us) differ by 0.15%, inside the noise, even though the wider field spreads 24.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (602 ns) is smaller than the fastest variant's own run-to-run std-dev (2.24 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-contend-pipe4 is inconsistent: worst-20% is 3.9x its best-20%

bitpack-contend-pipe4's best 20% of batches run at 2.02 us but its worst 20% at 7.88 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 2464.6 ns median (-19.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.24x (fastest 2464.6 ns, slowest 3066.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112791945 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 4091ns | 3130ns | 3116ns | 3136ns | 7930ns | base |
| bitpack-contend-d16-control | 4037ns | 2585ns | 2549ns | 3096ns | 8345ns | -1.32% |
| bitpack-contend-packed-simd | 3567ns | 2531ns | 2187ns | 2674ns | 7622ns | -12.81% |
| bitpack-contend-pipe2 | 3963ns | 2529ns | 2241ns | 3211ns | 7942ns | -3.12% |
| bitpack-contend-pipe4 | 3934ns | 2531ns | 2070ns | 3156ns | 8134ns | -3.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 3998ns | 3053ns | 7720ns | base | 4.098 |
| bitpack-contend-d16-control | 3946ns | 2498ns | 8132ns | -1.29% | 4.152 |
| bitpack-contend-packed-simd | 3468ns | 2133ns | 7383ns | -13.25% | 4.724 |
| bitpack-contend-pipe2 | 3852ns | 2184ns | 7692ns | -3.64% | 4.253 |
| bitpack-contend-pipe4 | 3827ns | 2018ns | 7881ns | -4.28% | 4.282 |

## Performance model

- Peak throughput: **8.117 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.343 | 65.8% |
| bitpack-contend-d16-control | 6.467 | 79.7% |
| bitpack-contend-packed-simd | 6.637 | 81.8% |
| bitpack-contend-pipe2 | 6.648 | 81.9% |
| bitpack-contend-pipe4 | 6.638 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 4091ns | 4091ns | base |
| bitpack-contend-d16-control | 4037ns | 4037ns | -1.32% |
| bitpack-contend-packed-simd | 3567ns | 3567ns | -12.81% |
| bitpack-contend-pipe2 | 3963ns | 3963ns | -3.12% |
| bitpack-contend-pipe4 | 3934ns | 3934ns | -3.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3066ns | base | --- | [3055, 3088] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 2533ns | -541.8ns (-17.7%) | [-556, -60]ns | [2508, 3054] | YES | 0.0002 | 0.0002 | 0 |
| bitpack-contend-packed-simd | 2469ns | -587.0ns (-19.1%) | [-606, -585]ns | [2468, 2480] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 2465ns | -604.7ns (-19.7%) | [-623, -593]ns | [2460, 2539] | YES | 0.0002 | 0.0002 | 0 |
| bitpack-contend-pipe4 | 2468ns | -601.2ns (-19.6%) | [-1034, -583]ns | [2455, 3387] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 3096ns | -19.0% | -34.6% | -20.2% | +2.1% |
| 1 | 2 | 3066ns | -0.4% | -19.5% | -7.5% | -19.3% |
| 1 | 3 | 3066ns | -18.5% | -19.5% | -19.6% | -34.1% |
| 1 | 4 | 3055ns | -17.3% | -19.2% | -19.4% | -19.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.019 | ok |
| bitpack-contend-d16-control | -0.445 | moderate- |
| bitpack-contend-packed-simd | -0.084 | ok |
| bitpack-contend-pipe2 | -0.400 | moderate- |
| bitpack-contend-pipe4 | 0.038 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 31.2ns | 3997.8ns | 0.8% |  |
| bitpack-contend-d16-control | 30.8ns | 3946.3ns | 0.8% |  |
| bitpack-contend-packed-simd | 33.3ns | 3468.1ns | 1.0% |  |
| bitpack-contend-pipe2 | 37.6ns | 3852.3ns | 1.0% |  |
| bitpack-contend-pipe4 | 36.3ns | 3826.6ns | 0.9% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 3052.7-7719.9 ns)
   3052.7 |########################################
   3286.1 |
   3519.4 |
   3752.8 |
   3986.1 |
   4219.5 |
   4452.9 |
   4686.2 |
   4919.6 |
   5152.9 |
   5386.3 |
   5619.6 |
   5853.0 |
   6086.4 |##
   6319.7 |
   6553.1 |
   6786.4 |
   7019.8 |
   7253.2 |
   7486.5 |
  (3 below, 5 above range)

bitpack-contend-d16-control (n=40, range 2498.3-8132.1 ns)
   2498.3 |########################################
   2780.0 |##############
   3061.7 |####
   3343.4 |
   3625.1 |
   3906.7 |
   4188.4 |##
   4470.1 |##
   4751.8 |
   5033.5 |
   5315.2 |
   5596.9 |
   5878.6 |
   6160.3 |
   6441.9 |
   6723.6 |##
   7005.3 |####
   7287.0 |##
   7568.7 |##
   7850.4 |####
  (4 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 2133.2-7382.6 ns)
   2133.2 |
   2395.6 |########################################
   2658.1 |
   2920.6 |
   3183.1 |
   3445.5 |#
   3708.0 |
   3970.5 |
   4232.9 |
   4495.4 |#
   4757.9 |
   5020.3 |
   5282.8 |#
   5545.3 |
   5807.8 |
   6070.2 |#
   6332.7 |#
   6595.2 |#
   6857.6 |
   7120.1 |
  (6 below, 4 above range)

bitpack-contend-pipe2 (n=40, range 2184.3-7692.1 ns)
   2184.3 |######################
   2459.7 |########################################
   2735.1 |
   3010.5 |
   3285.9 |
   3561.3 |##
   3836.7 |
   4112.0 |
   4387.4 |
   4662.8 |
   4938.2 |
   5213.6 |
   5489.0 |
   5764.4 |#####
   6039.8 |
   6315.2 |#####
   6590.6 |##
   6866.0 |
   7141.4 |##
   7416.7 |########
  (5 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 2018.5-7881.1 ns)
   2018.5 |#################################
   2311.6 |########################################
   2604.7 |
   2897.9 |
   3191.0 |
   3484.1 |
   3777.3 |
   4070.4 |######
   4363.5 |###
   4656.7 |
   4949.8 |
   5242.9 |
   5536.0 |
   5829.2 |###
   6122.3 |###
   6415.4 |###
   6708.6 |######
   7001.7 |###
   7294.8 |
   7588.0 |###
  (2 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=53.0% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: CV=57.5% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=60.4% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bitpack-contend-pipe2**: CV=58.2% (high variance, measurements may be unstable)
- **bitpack-contend-pipe2**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bitpack-contend-pipe4**: CV=61.6% (high variance, measurements may be unstable)
- **bitpack-contend-pipe4**: worst_20/best_20 = 3.9x (possible bimodal distribution)
