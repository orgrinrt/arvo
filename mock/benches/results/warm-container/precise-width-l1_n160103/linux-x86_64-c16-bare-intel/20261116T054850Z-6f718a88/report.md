# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (4.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 51 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 4491% faster than the next best (warm-container-minimum)

warm-container-kernel (51 ns) leads warm-container-minimum (2.36 us) by 4491%, a clear separation rather than a photo finish. CV 22.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 98% (significant)

warm-container-kernel is -3.95 us (98%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 78.1x slower than the field

warm-container-headroom (4.01 us) is 78.1x the fastest (51 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.79)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (4491% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 4491% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 78.1x the fastest

Fastest warm-container-kernel (51 ns) to slowest warm-container-headroom (4.01 us): 78.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.6x its best-20%

warm-container-native's best 20% of batches run at 2.55 us but its worst 20% at 9.29 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 51.4 ns median (-98.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 78.09x (fastest 51.4 ns, slowest 4013.8 ns)

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
| Counter frequency | 2112536030 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 4799ns | 4078ns | 3336ns | 3826ns | 9182ns | base |
| warm-container-kernel | 309ns | 276ns | 232ns | 292ns | 440ns | -93.55% |
| warm-container-minimum | 4266ns | 2412ns | 2401ns | 3542ns | 8302ns | -11.11% |
| warm-container-native | 4661ns | 2940ns | 2602ns | 3717ns | 9553ns | -2.87% |
| warm-container-plusone | 4862ns | 3346ns | 3334ns | 3593ns | 10197ns | +1.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4713ns | 3284ns | 8978ns | base | 6.953 |
| warm-container-kernel | 54ns | 43ns | 73ns | -98.85% | 606.899 |
| warm-container-minimum | 4163ns | 2349ns | 8075ns | -11.67% | 7.872 |
| warm-container-native | 4550ns | 2546ns | 9293ns | -3.46% | 7.202 |
| warm-container-plusone | 4776ns | 3283ns | 9984ns | +1.34% | 6.861 |

## Performance model

- Peak throughput: **763.378 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.164 | 1.1% |
| warm-container-kernel | 637.510 | 83.5% |
| warm-container-minimum | 13.887 | 1.8% |
| warm-container-native | 11.392 | 1.5% |
| warm-container-plusone | 9.947 | 1.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4799ns | 4799ns | base |
| warm-container-kernel | 309ns | 309ns | -93.55% |
| warm-container-minimum | 4266ns | 4266ns | -11.11% |
| warm-container-native | 4661ns | 4661ns | -2.87% |
| warm-container-plusone | 4862ns | 4862ns | +1.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4014ns | base | --- | [3335, 4036] | --- | --- | --- | --- |
| warm-container-kernel | 51ns | -3953.0ns (-98.5%) | [-3981, -3285]ns | [47, 52] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2360ns | -995.5ns (-24.8%) | [-1672, -933]ns | [2350, 4826] | YES | 0.0030 | 0.0022 | 0 |
| warm-container-native | 2876ns | -958.3ns (-23.9%) | [-1143, -415]ns | [2872, 3474] | YES | 0.0014 | 0.0007 | 0 |
| warm-container-plusone | 3294ns | no significant difference | [-723, +1]ns | [3285, 3296] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3286ns | -98.7% | -28.2% | -12.4% | +0.1% |
| 1 | 2 | 3294ns | -98.4% | +1.8% | -0.5% | +0.0% |
| 1 | 3 | 4069ns | -98.8% | -42.3% | -29.3% | -19.2% |
| 1 | 4 | 4027ns | -98.2% | -41.5% | -28.6% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.238 | moderate+ |
| warm-container-kernel | -0.177 | ok |
| warm-container-minimum | -0.415 | moderate- |
| warm-container-native | -0.413 | moderate- |
| warm-container-plusone | -0.790 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 29.1ns | 4712.6ns | 0.6% |  |
| warm-container-kernel | 88.9ns | 54.0ns | 164.6% | HIGH |
| warm-container-minimum | 35.6ns | 4162.8ns | 0.9% |  |
| warm-container-native | 39.0ns | 4549.6ns | 0.9% |  |
| warm-container-plusone | 29.7ns | 4775.8ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3284.3-8978.3 ns)
   3284.3 |#########################
   3569.0 |
   3853.7 |########################################
   4138.4 |
   4423.1 |
   4707.8 |##
   4992.5 |
   5277.2 |
   5561.9 |
   5846.6 |
   6131.3 |
   6416.0 |
   6700.7 |##
   6985.4 |
   7270.1 |
   7554.8 |
   7839.5 |
   8124.2 |
   8408.9 |
   8693.6 |
  (7 below, 5 above range)

warm-container-kernel (n=40, range 42.9-72.9 ns)
     42.9 |##############################
     44.4 |
     45.9 |###################################
     47.4 |#####
     48.9 |
     50.4 |###################################
     51.9 |####################
     53.4 |#####
     54.9 |
     56.4 |
     57.9 |
     59.4 |
     60.9 |
     62.4 |
     63.9 |
     65.4 |
     66.9 |
     68.4 |
     69.9 |
     71.4 |########################################
  (4 below, 2 above range)

warm-container-minimum (n=40, range 2349.4-8074.8 ns)
   2349.4 |########################################
   2635.6 |#
   2921.9 |
   3208.2 |
   3494.5 |
   3780.7 |
   4067.0 |#
   4353.3 |
   4639.6 |
   4925.8 |
   5212.1 |#
   5498.4 |#
   5784.7 |
   6070.9 |
   6357.2 |###
   6643.5 |#
   6929.8 |
   7216.0 |###
   7502.3 |#
   7788.6 |#####
  (2 below, 3 above range)

warm-container-native (n=40, range 2546.3-9292.6 ns)
   2546.3 |########################################
   2883.6 |#########
   3221.0 |
   3558.3 |
   3895.6 |####
   4232.9 |
   4570.2 |
   4907.5 |
   5244.8 |
   5582.1 |##
   5919.5 |
   6256.8 |
   6594.1 |##
   6931.4 |##
   7268.7 |##
   7606.0 |##
   7943.3 |##
   8280.6 |
   8618.0 |
   8955.3 |####
  (5 below, 4 above range)

warm-container-plusone (n=40, range 3282.8-9984.4 ns)
   3282.8 |########################################
   3617.9 |
   3953.0 |
   4288.0 |#
   4623.1 |
   4958.2 |#
   5293.3 |
   5628.4 |
   5963.5 |
   6298.5 |#
   6633.6 |
   6968.7 |
   7303.8 |
   7638.9 |
   7974.0 |#
   8309.0 |
   8644.1 |
   8979.2 |
   9314.3 |#
   9649.4 |###
  (5 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=55.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: CV=21.1% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=169.0% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=58.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-container-native**: CV=59.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-plusone**: CV=56.4% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.0x (possible bimodal distribution)
