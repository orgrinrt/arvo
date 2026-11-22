# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d32 dominates: 30% faster than the next best (bitpack-contend-packed-simd)

bitpack-contend-d32 (1.90 us) leads bitpack-contend-packed-simd (2.47 us) by 30%, a clear separation rather than a photo finish. CV 107.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d32 beats baseline by 25% (significant)

bitpack-contend-d32 is -620 ns (25%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.1x slower than the field

bitpack-contend-d64 (3.99 us) is 2.1x the fastest (1.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d32 is fastest but the noisiest (CV 107.3%)

bitpack-contend-d32 wins on median (1.90 us) yet has the highest variance (CV 107.3%), while bitpack-contend-d64 is the steadiest (CV 36.3%, 3.99 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-packed-simd shows alternating (throttle bounce) (autocorr -0.75)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-contend-packed is inconsistent: worst-20% is 3.7x its best-20%

bitpack-contend-packed's best 20% of batches run at 2.01 us but its worst 20% at 7.54 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d32** at 1897.5 ns median (-24.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.10x (fastest 1897.5 ns, slowest 3992.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749291 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 4031ns | 2559ns | 2548ns | 3043ns | 8480ns | base |
| bitpack-contend-d16-control | 3889ns | 2562ns | 2549ns | 2793ns | 8518ns | -3.53% |
| bitpack-contend-d32 | 3478ns | 1964ns | 1949ns | 2838ns | 6928ns | -13.73% |
| bitpack-contend-d64 | 3493ns | 4193ns | 1429ns | 3597ns | 5246ns | -13.34% |
| bitpack-contend-packed | 3875ns | 2662ns | 2067ns | 3168ns | 7807ns | -3.87% |
| bitpack-contend-packed-simd | 3721ns | 2532ns | 2071ns | 2990ns | 7566ns | -7.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 3940ns | 2497ns | 8258ns | base | 4.159 |
| bitpack-contend-d16-control | 3799ns | 2497ns | 8293ns | -3.57% | 4.313 |
| bitpack-contend-d32 | 3349ns | 1883ns | 6639ns | -14.99% | 4.892 |
| bitpack-contend-d64 | 3316ns | 1361ns | 4951ns | -15.83% | 4.941 |
| bitpack-contend-packed | 3748ns | 2014ns | 7539ns | -4.86% | 4.371 |
| bitpack-contend-packed-simd | 3617ns | 2019ns | 7322ns | -8.18% | 4.529 |

## Performance model

- Peak throughput: **12.037 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.534 | 54.3% |
| bitpack-contend-d16-control | 6.531 | 54.3% |
| bitpack-contend-d32 | 8.635 | 71.7% |
| bitpack-contend-d64 | 4.104 | 34.1% |
| bitpack-contend-packed | 6.319 | 52.5% |
| bitpack-contend-packed-simd | 6.637 | 55.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 4031ns | 4031ns | base |
| bitpack-contend-d16-control | 3889ns | 3889ns | -3.53% |
| bitpack-contend-d32 | 3478ns | 3478ns | -13.73% |
| bitpack-contend-d64 | 3493ns | 3493ns | -13.34% |
| bitpack-contend-packed | 3875ns | 3875ns | -3.87% |
| bitpack-contend-packed-simd | 3721ns | 3721ns | -7.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 2507ns | base | --- | [2497, 3052] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 2509ns | no significant difference | [-10, +12]ns | [2498, 2572] | no | 0.6358 | 0.6358 | 0 |
| bitpack-contend-d32 | 1898ns | -620.4ns (-24.7%) | [-1107, -610]ns | [1887, 3358] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-d64 | 3992ns | no significant difference | [-1417, +741]ns | [2833, 4312] | no | 0.5370 | 0.4296 | 0 |
| bitpack-contend-packed | 2593ns | -245.2ns (-9.8%) | [-480, -37]ns | [2461, 2826] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 2469ns | -477.3ns (-19.0%) | [-487, -339]ns | [2468, 2482] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2497ns | +0.6% | +13.8% | +60.6% | -19.0% | -18.9% |
| 1 | 2 | 2502ns | +0.3% | -23.9% | +55.7% | -1.1% | -1.3% |
| 1 | 3 | 2502ns | +2.8% | +11.0% | +59.9% | -19.2% | -19.1% |
| 1 | 4 | 3052ns | -18.2% | -38.2% | -22.4% | -7.5% | -18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.081 | ok |
| bitpack-contend-d16-control | -0.540 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.725 | HIGH- (thermal bounce) |
| bitpack-contend-d64 | -0.121 | ok |
| bitpack-contend-packed | -0.527 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.747 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 2/4, lost 2/4
- **bitpack-contend-d64**: won 1/4, lost 3/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 31.1ns | 3939.7ns | 0.8% |  |
| bitpack-contend-d16-control | 30.0ns | 3799.2ns | 0.8% |  |
| bitpack-contend-d32 | 35.7ns | 3349.1ns | 1.1% |  |
| bitpack-contend-d64 | 50.0ns | 3316.0ns | 1.5% |  |
| bitpack-contend-packed | 42.0ns | 3748.3ns | 1.1% |  |
| bitpack-contend-packed-simd | 35.3ns | 3617.5ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 2496.9-8258.0 ns)
   2496.9 |########################################
   2784.9 |################
   3073.0 |
   3361.1 |
   3649.1 |
   3937.2 |
   4225.2 |
   4513.3 |
   4801.3 |
   5089.4 |
   5377.5 |##
   5665.5 |
   5953.6 |
   6241.6 |
   6529.7 |##
   6817.7 |
   7105.8 |##
   7393.9 |##
   7681.9 |####
   7970.0 |##
  (3 below, 3 above range)

bitpack-contend-d16-control (n=40, range 2497.1-8293.4 ns)
   2497.1 |########################################
   2786.9 |#
   3076.7 |
   3366.5 |
   3656.4 |
   3946.2 |
   4236.0 |
   4525.8 |
   4815.6 |
   5105.4 |
   5395.2 |
   5685.1 |
   5974.9 |
   6264.7 |
   6554.5 |
   6844.3 |
   7134.1 |#
   7424.0 |
   7713.8 |##
   8003.6 |##
  (2 below, 4 above range)

bitpack-contend-d32 (n=40, range 1882.8-6639.4 ns)
   1882.8 |########################################
   2120.6 |####
   2358.5 |
   2596.3 |
   2834.1 |##
   3071.9 |
   3309.8 |
   3547.6 |##
   3785.4 |##
   4023.3 |
   4261.1 |
   4498.9 |
   4736.8 |
   4974.6 |
   5212.4 |
   5450.2 |######
   5688.1 |######
   5925.9 |##
   6163.7 |####
   6401.6 |##
  (4 below, 2 above range)

bitpack-contend-d64 (n=40, range 1361.2-4950.7 ns)
   1361.2 |########################################
   1540.7 |
   1720.1 |
   1899.6 |
   2079.1 |
   2258.6 |
   2438.0 |
   2617.5 |#####
   2797.0 |
   2976.5 |#####
   3155.9 |
   3335.4 |##########
   3514.9 |
   3694.4 |##########
   3873.9 |####################
   4053.3 |#####
   4232.8 |#########################
   4412.3 |###############
   4591.8 |###############
   4771.2 |#####
  (5 below, 4 above range)

bitpack-contend-packed (n=40, range 2013.6-7539.0 ns)
   2013.6 |########################################
   2289.9 |###############################
   2566.1 |###############################
   2842.4 |
   3118.7 |
   3394.9 |####
   3671.2 |
   3947.5 |
   4223.8 |####
   4500.0 |
   4776.3 |
   5052.6 |
   5328.8 |####
   5605.1 |
   5881.4 |
   6157.6 |####
   6433.9 |####
   6710.2 |########
   6986.4 |
   7262.7 |########
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 2019.4-7321.8 ns)
   2019.4 |#####################
   2284.6 |########################################
   2549.7 |##
   2814.8 |
   3079.9 |
   3345.0 |
   3610.1 |
   3875.3 |
   4140.4 |
   4405.5 |##
   4670.6 |
   4935.7 |
   5200.8 |
   5466.0 |#####
   5731.1 |
   5996.2 |
   6261.3 |##
   6526.4 |#####
   6791.6 |#####
   7056.7 |##
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=59.0% (high variance, measurements may be unstable)
- **bitpack-contend-d16**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-d16-control**: CV=62.4% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-d32**: CV=60.8% (high variance, measurements may be unstable)
- **bitpack-contend-d32**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bitpack-contend-d64**: CV=43.7% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bitpack-contend-packed**: CV=58.6% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=58.8% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.6x (possible bimodal distribution)
