# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d64 dominates: 68% faster than the next best (bitpack-contend-d32)

bitpack-contend-d64 (1.37 us) leads bitpack-contend-d32 (2.31 us) by 68%, a clear separation rather than a photo finish. CV 122.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d32 beats baseline by 24% (significant)

bitpack-contend-d32 is -745 ns (24%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d16-control is an outlier: 2.2x slower than the field

bitpack-contend-d16-control (3.07 us) is 2.2x the fastest (1.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d64 is fastest but the noisiest (CV 122.7%)

bitpack-contend-d64 wins on median (1.37 us) yet has the highest variance (CV 122.7%), while bitpack-contend-d16 is the steadiest (CV 61.2%, 3.05 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-contend-d64} vs {bitpack-contend-d32, bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16, bitpack-contend-d16-control} (68% apart)

The field splits into a fast tier {bitpack-contend-d64} and a slow tier {bitpack-contend-d32, bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16, bitpack-contend-d16-control} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-contend-d64 is inconsistent: worst-20% is 3.9x its best-20%

bitpack-contend-d64's best 20% of batches run at 1.34 us but its worst 20% at 5.28 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d64** at 1370.8 ns median (-55.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.24x (fastest 1370.8 ns, slowest 3069.2 ns)

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
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3695ns | 3118ns | 2557ns | 3104ns | 6603ns | base |
| bitpack-contend-d16-control | 4016ns | 3132ns | 2553ns | 3049ns | 8381ns | +8.70% |
| bitpack-contend-d32 | 3504ns | 2390ns | 2118ns | 2640ns | 7484ns | -5.15% |
| bitpack-contend-d64 | 3024ns | 1438ns | 1410ns | 2708ns | 5587ns | -18.15% |
| bitpack-contend-packed | 3766ns | 2531ns | 2186ns | 2861ns | 8064ns | +1.94% |
| bitpack-contend-packed-simd | 3718ns | 2531ns | 2530ns | 2685ns | 8003ns | +0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 3609ns | 2506ns | 6419ns | base | 4.540 |
| bitpack-contend-d16-control | 3925ns | 2502ns | 8161ns | +8.75% | 4.174 |
| bitpack-contend-d32 | 3375ns | 2047ns | 7178ns | -6.48% | 4.854 |
| bitpack-contend-d64 | 2870ns | 1344ns | 5279ns | -20.49% | 5.710 |
| bitpack-contend-packed | 3663ns | 2132ns | 7814ns | +1.51% | 4.472 |
| bitpack-contend-packed-simd | 3616ns | 2468ns | 7755ns | +0.20% | 4.531 |

## Performance model

- Peak throughput: **12.190 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 5.365 | 44.0% |
| bitpack-contend-d16-control | 5.338 | 43.8% |
| bitpack-contend-d32 | 7.100 | 58.2% |
| bitpack-contend-d64 | 11.952 | 98.1% |
| bitpack-contend-packed | 6.636 | 54.4% |
| bitpack-contend-packed-simd | 6.636 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 3695ns | 3695ns | base |
| bitpack-contend-d16-control | 4016ns | 4016ns | +8.70% |
| bitpack-contend-d32 | 3504ns | 3504ns | -5.15% |
| bitpack-contend-d64 | 3024ns | 3024ns | -18.15% |
| bitpack-contend-packed | 3766ns | 3766ns | +1.94% |
| bitpack-contend-packed-simd | 3718ns | 3718ns | +0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 3054ns | base | --- | [3053, 3066] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 3069ns | no significant difference | [-3, +22]ns | [2794, 3081] | no | 0.1009 | 0.0807 | 0 |
| bitpack-contend-d32 | 2308ns | -745.3ns (-24.4%) | [-752, -215]ns | [2307, 2320] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-d64 | 1371ns | no significant difference | [-1696, +112]ns | [1359, 4076] | no | 0.1539 | 0.1539 | 0 |
| bitpack-contend-packed | 2469ns | -585.6ns (-19.2%) | [-593, -45]ns | [2468, 2480] | YES | 0.0003 | 0.0002 | 0 |
| bitpack-contend-packed-simd | 2469ns | -583.6ns (-19.1%) | [-585, -48]ns | [2469, 2480] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3066ns | +0.3% | -24.8% | -55.8% | -19.5% | -19.5% |
| 1 | 2 | 3067ns | +0.5% | -24.6% | -55.5% | -34.0% | -19.5% |
| 1 | 3 | 2505ns | +0.6% | -7.9% | -45.9% | -1.2% | -1.4% |
| 1 | 4 | 3053ns | -17.9% | -16.7% | +30.4% | -19.2% | -19.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.403 | moderate- |
| bitpack-contend-d16-control | 0.253 | moderate+ |
| bitpack-contend-d32 | -0.098 | ok |
| bitpack-contend-d64 | -0.086 | ok |
| bitpack-contend-packed | -0.422 | moderate- |
| bitpack-contend-packed-simd | -0.083 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 4/4, lost 0/4
- **bitpack-contend-d64**: won 3/4, lost 1/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 28.2ns | 3609.0ns | 0.8% |  |
| bitpack-contend-d16-control | 30.6ns | 3924.9ns | 0.8% |  |
| bitpack-contend-d32 | 35.9ns | 3375.0ns | 1.1% |  |
| bitpack-contend-d64 | 42.7ns | 2869.5ns | 1.5% |  |
| bitpack-contend-packed | 35.4ns | 3663.4ns | 1.0% |  |
| bitpack-contend-packed-simd | 34.7ns | 3616.2ns | 1.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 2505.9-6419.5 ns)
   2505.9 |#######
   2701.6 |
   2897.3 |########################################
   3093.0 |#####
   3288.6 |
   3484.3 |
   3680.0 |
   3875.7 |
   4071.4 |#
   4267.0 |
   4462.7 |
   4658.4 |#
   4854.1 |
   5049.7 |
   5245.4 |
   5441.1 |
   5636.8 |
   5832.5 |
   6028.1 |
   6223.8 |
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 2501.9-8161.1 ns)
   2501.9 |##############################
   2784.9 |################
   3067.8 |########################################
   3350.8 |
   3633.7 |
   3916.7 |
   4199.7 |######
   4482.6 |
   4765.6 |
   5048.6 |
   5331.5 |
   5614.5 |
   5897.4 |
   6180.4 |
   6463.4 |
   6746.3 |
   7029.3 |
   7312.3 |######
   7595.2 |###
   7878.2 |
  (5 below, 4 above range)

bitpack-contend-d32 (n=40, range 2047.0-7178.0 ns)
   2047.0 |
   2303.6 |########################################
   2560.1 |
   2816.7 |
   3073.2 |#
   3329.8 |#
   3586.3 |
   3842.9 |
   4099.4 |
   4356.0 |
   4612.5 |
   4869.1 |
   5125.6 |
   5382.2 |
   5638.7 |
   5895.3 |#
   6151.8 |#
   6408.4 |#####
   6664.9 |
   6921.5 |#
  (5 below, 3 above range)

bitpack-contend-d64 (n=40, range 1344.1-5279.3 ns)
   1344.1 |########################################
   1540.8 |
   1737.6 |
   1934.4 |
   2131.1 |
   2327.9 |
   2524.6 |
   2721.4 |
   2918.2 |##
   3114.9 |##
   3311.7 |##
   3508.4 |
   3705.2 |
   3902.0 |####
   4098.7 |####
   4295.5 |##
   4492.2 |####
   4689.0 |##
   4885.8 |######
   5082.5 |
  (3 below, 5 above range)

bitpack-contend-packed (n=40, range 2132.0-7814.3 ns)
   2132.0 |
   2416.1 |########################################
   2700.2 |
   2984.3 |
   3268.4 |
   3552.6 |
   3836.7 |#
   4120.8 |
   4404.9 |
   4689.0 |#
   4973.1 |
   5257.3 |
   5541.4 |
   5825.5 |
   6109.6 |#
   6393.7 |#
   6677.8 |#
   6962.0 |#
   7246.1 |#
   7530.2 |
  (6 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 2468.2-7755.2 ns)
   2468.2 |########################################
   2732.6 |
   2996.9 |
   3261.3 |
   3525.6 |
   3789.9 |#
   4054.3 |
   4318.6 |#
   4583.0 |#
   4847.3 |
   5111.7 |#
   5376.0 |
   5640.4 |
   5904.7 |
   6169.1 |
   6433.4 |
   6697.8 |
   6962.1 |
   7226.5 |
   7490.8 |
  (4 below, 6 above range)

```

## Diagnostics

- **bitpack-contend-d16**: CV=51.8% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: CV=58.3% (high variance, measurements may be unstable)
- **bitpack-contend-d16-control**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-contend-d32**: CV=60.4% (high variance, measurements may be unstable)
- **bitpack-contend-d32**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **bitpack-contend-d64**: CV=58.6% (high variance, measurements may be unstable)
- **bitpack-contend-d64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **bitpack-contend-packed**: CV=61.3% (high variance, measurements may be unstable)
- **bitpack-contend-packed**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **bitpack-contend-packed-simd**: CV=61.4% (high variance, measurements may be unstable)
- **bitpack-contend-packed-simd**: worst_20/best_20 = 3.1x (possible bimodal distribution)
