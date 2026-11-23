# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 51 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 5486% faster than the next best (warm-container-minimum)

warm-container-kernel (51 ns) leads warm-container-minimum (2.87 us) by 5486%, a clear separation rather than a photo finish. CV 10.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -4.05 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 79.7x slower than the field

warm-container-headroom (4.10 us) is 79.7x the fastest (51 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (5486% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 5486% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 79.7x the fastest

Fastest warm-container-kernel (51 ns) to slowest warm-container-headroom (4.10 us): 79.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.6x its best-20%

warm-container-native's best 20% of batches run at 2.42 us but its worst 20% at 8.60 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 51.4 ns median (-98.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 79.67x (fastest 51.4 ns, slowest 4095.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112691662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4953ns | 4177ns | 3421ns | 4181ns | 8801ns | base |
| warm-container-kernel | 289ns | 278ns | 235ns | 283ns | 359ns | -94.17% |
| warm-container-minimum | 3736ns | 2935ns | 2400ns | 2850ns | 7730ns | -24.57% |
| warm-container-native | 4063ns | 2941ns | 2470ns | 3001ns | 8844ns | -17.96% |
| warm-container-plusone | 4879ns | 4177ns | 3464ns | 4181ns | 8389ns | -1.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4847ns | 3354ns | 8582ns | base | 6.761 |
| warm-container-kernel | 51ns | 43ns | 59ns | -98.94% | 639.064 |
| warm-container-minimum | 3645ns | 2349ns | 7525ns | -24.80% | 8.990 |
| warm-container-native | 3966ns | 2417ns | 8602ns | -18.17% | 8.263 |
| warm-container-plusone | 4772ns | 3394ns | 8174ns | -1.53% | 6.866 |

## Performance model

- Peak throughput: **755.676 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.002 | 1.1% |
| warm-container-kernel | 637.510 | 84.4% |
| warm-container-minimum | 11.412 | 1.5% |
| warm-container-native | 11.387 | 1.5% |
| warm-container-plusone | 8.004 | 1.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4953ns | 4953ns | base |
| warm-container-kernel | 289ns | 289ns | -94.17% |
| warm-container-minimum | 3736ns | 3736ns | -24.57% |
| warm-container-native | 4063ns | 4063ns | -17.96% |
| warm-container-plusone | 4879ns | 4879ns | -1.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4095ns | base | --- | [4095, 4107] | --- | --- | --- | --- |
| warm-container-kernel | 51ns | -4049.4ns (-98.9%) | [-4055, -4044]ns | [51, 52] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2871ns | -1216.7ns (-29.7%) | [-1236, -1143]ns | [2381, 2886] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2878ns | -1208.2ns (-29.5%) | [-1223, -729]ns | [2871, 2903] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4094ns | no significant difference | [-2, +1]ns | [4093, 4106] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 4107ns | -98.7% | -30.1% | -29.9% | -0.0% |
| 1 | 2 | 4095ns | -98.7% | -42.4% | -42.5% | -0.0% |
| 1 | 3 | 3356ns | -98.3% | -30.0% | -14.1% | -0.2% |
| 1 | 4 | 4095ns | -99.0% | -28.8% | -29.2% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.404 | moderate- |
| warm-container-kernel | -0.438 | moderate- |
| warm-container-minimum | -0.250 | moderate- |
| warm-container-native | -0.386 | moderate- |
| warm-container-plusone | -0.409 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.7ns | 4846.6ns | 0.6% |  |
| warm-container-kernel | 85.0ns | 51.3ns | 165.7% | HIGH |
| warm-container-minimum | 31.4ns | 3644.8ns | 0.9% |  |
| warm-container-native | 33.0ns | 3965.8ns | 0.8% |  |
| warm-container-plusone | 28.2ns | 4772.4ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3354.1-8582.3 ns)
   3354.1 |####
   3615.5 |
   3876.9 |########################################
   4138.3 |#
   4399.8 |
   4661.2 |
   4922.6 |
   5184.0 |
   5445.4 |
   5706.8 |
   5968.2 |
   6229.6 |
   6491.0 |
   6752.4 |
   7013.9 |#
   7275.3 |
   7536.7 |
   7798.1 |
   8059.5 |
   8320.9 |
  (5 below, 5 above range)

warm-container-kernel (n=40, range 43.4-58.6 ns)
     43.4 |
     44.1 |##
     44.9 |
     45.6 |
     46.4 |######
     47.2 |
     47.9 |
     48.7 |
     49.4 |
     50.2 |
     51.0 |########################################
     51.7 |
     52.5 |
     53.3 |
     54.0 |
     54.8 |
     55.5 |
     56.3 |########
     57.1 |########
     57.8 |
  (6 below, 2 above range)

warm-container-minimum (n=40, range 2349.2-7525.1 ns)
   2349.2 |##############################
   2607.9 |
   2866.7 |########################################
   3125.5 |
   3384.3 |
   3643.1 |
   3901.9 |##
   4160.7 |
   4419.5 |
   4678.3 |##
   4937.1 |##
   5195.9 |
   5454.7 |
   5713.5 |
   5972.3 |##
   6231.1 |
   6489.9 |
   6748.7 |
   7007.5 |
   7266.3 |
  (3 below, 5 above range)

warm-container-native (n=40, range 2417.2-8601.7 ns)
   2417.2 |
   2726.4 |########################################
   3035.6 |
   3344.8 |###
   3654.1 |
   3963.3 |
   4272.5 |
   4581.8 |#
   4891.0 |
   5200.2 |
   5509.4 |
   5818.7 |
   6127.9 |
   6437.1 |
   6746.4 |
   7055.6 |
   7364.8 |
   7674.0 |
   7983.3 |#
   8292.5 |#
  (7 below, 5 above range)

warm-container-plusone (n=40, range 3394.2-8174.0 ns)
   3394.2 |
   3633.2 |#
   3872.2 |########################################
   4111.2 |#
   4350.2 |
   4589.2 |
   4828.2 |
   5067.2 |
   5306.1 |
   5545.1 |
   5784.1 |
   6023.1 |
   6262.1 |
   6501.1 |
   6740.1 |
   6979.1 |
   7218.1 |
   7457.0 |
   7696.0 |
   7935.0 |
  (7 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=47.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=165.4% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=58.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-container-native**: CV=61.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=46.9% (high variance, measurements may be unstable)
