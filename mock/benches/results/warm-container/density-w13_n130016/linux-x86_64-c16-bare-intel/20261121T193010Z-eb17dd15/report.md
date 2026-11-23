# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (7.95 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 2.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 47% faster than the next best (warm-container-lanes-deferred)

warm-container-kernel (2.08 us) leads warm-container-lanes-deferred (3.06 us) by 47%, a clear separation rather than a photo finish. CV 97.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 69% (significant)

warm-container-kernel is -5.45 us (69%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.8x slower than the field

warm-container-headroom (7.95 us) is 3.8x the fastest (2.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 97.5%)

warm-container-kernel wins on median (2.08 us) yet has the highest variance (CV 97.5%), while warm-container-minimum is the steadiest (CV 9.6%, 7.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.70)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.8x the fastest

Fastest warm-container-kernel (2.08 us) to slowest warm-container-headroom (7.95 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 4.2x its best-20%

warm-container-kernel's best 20% of batches run at 1.51 us but its worst 20% at 6.30 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (-5 ns, 0.06%)

warm-container-plusone differs from baseline warm-container-headroom by -5 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 2084.2 ns median (-73.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 3.81x (fastest 2084.2 ns, slowest 7946.0 ns)

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
| warm-container-headroom | 8026ns | 8030ns | 7201ns | 8015ns | 8883ns | base |
| warm-container-kernel | 3661ns | 2159ns | 1565ns | 3394ns | 6556ns | -54.39% |
| warm-container-lanes-deferred | 4099ns | 3122ns | 2767ns | 3208ns | 8107ns | -48.92% |
| warm-container-minimum | 7615ns | 7212ns | 7200ns | 7351ns | 8822ns | -5.12% |
| warm-container-native | 3881ns | 4367ns | 1666ns | 3675ns | 6713ns | -51.65% |
| warm-container-plusone | 8010ns | 8024ns | 7199ns | 8008ns | 8824ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 7962ns | 7148ns | 8808ns | base | 17.492 |
| warm-container-kernel | 3532ns | 1514ns | 6297ns | -55.64% | 39.435 |
| warm-container-lanes-deferred | 4009ns | 2711ns | 7924ns | -49.64% | 34.736 |
| warm-container-minimum | 7552ns | 7145ns | 8747ns | -5.14% | 18.440 |
| warm-container-native | 3734ns | 1608ns | 6433ns | -53.10% | 37.296 |
| warm-container-plusone | 7945ns | 7144ns | 8746ns | -0.20% | 17.527 |

## Performance model

- Peak throughput: **92.008 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.526 | 19.0% |
| warm-container-kernel | 66.817 | 72.6% |
| warm-container-lanes-deferred | 45.531 | 49.5% |
| warm-container-minimum | 19.460 | 21.1% |
| warm-container-native | 33.055 | 35.9% |
| warm-container-plusone | 17.531 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8026ns | 8026ns | base |
| warm-container-kernel | 3661ns | 3661ns | -54.39% |
| warm-container-lanes-deferred | 4099ns | 4099ns | -48.92% |
| warm-container-minimum | 7615ns | 7615ns | -5.12% |
| warm-container-native | 3881ns | 3881ns | -51.65% |
| warm-container-plusone | 8010ns | 8010ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7946ns | base | --- | [7156, 8745] | --- | --- | --- | --- |
| warm-container-kernel | 2084ns | -5447.5ns (-68.6%) | [-5641, -3011]ns | [1850, 5161] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 3059ns | -4096.4ns (-51.6%) | [-5393, -4087]ns | [3058, 3087] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 7157ns | no significant difference | [-10, +0]ns | [7155, 7158] | no | 0.2559 | 0.2559 | 2 |
| warm-container-native | 4213ns | -3879.4ns (-48.8%) | [-5425, -3173]ns | [1736, 4934] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 7944ns | -5.0ns (-0.1%) | [-11, -3]ns | [7153, 8739] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 8808ns | -59.5% | -67.8% | -18.8% | -54.7% | -0.7% |
| 1 | 2 | 8745ns | -78.7% | -65.0% | +0.0% | -43.6% | -0.0% |
| 1 | 3 | 7155ns | -49.5% | -57.3% | -0.0% | -75.8% | -0.1% |
| 1 | 4 | 7155ns | -59.1% | -57.1% | +0.0% | -39.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.259 | moderate+ |
| warm-container-kernel | -0.697 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.073 | ok |
| warm-container-minimum | -0.417 | moderate- |
| warm-container-native | -0.555 | HIGH- (thermal bounce) |
| warm-container-plusone | 0.250 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 1/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 22.4ns | 7961.7ns | 0.3% |  |
| warm-container-kernel | 45.6ns | 3531.5ns | 1.3% |  |
| warm-container-lanes-deferred | 31.1ns | 4009.2ns | 0.8% |  |
| warm-container-minimum | 20.9ns | 7552.2ns | 0.3% |  |
| warm-container-native | 51.9ns | 3734.0ns | 1.4% |  |
| warm-container-plusone | 22.1ns | 7945.5ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7148.1-8807.8 ns)
   7148.1 |########################################
   7231.0 |
   7314.0 |
   7397.0 |
   7480.0 |
   7563.0 |
   7646.0 |
   7729.0 |
   7812.0 |
   7894.9 |
   7977.9 |
   8060.9 |
   8143.9 |
   8226.9 |
   8309.9 |
   8392.9 |
   8475.9 |
   8558.9 |
   8641.8 |
   8724.8 |########################################
  (5 below, 5 above range)

warm-container-kernel (n=40, range 1513.6-6296.7 ns)
   1513.6 |########################################
   1752.8 |###############################
   1991.9 |
   2231.1 |####
   2470.2 |
   2709.4 |
   2948.5 |
   3187.7 |
   3426.8 |
   3666.0 |
   3905.2 |####
   4144.3 |####
   4383.5 |
   4622.6 |####
   4861.8 |####
   5100.9 |#############
   5340.1 |########
   5579.2 |#################
   5818.4 |####
   6057.6 |####
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 2711.4-7924.5 ns)
   2711.4 |
   2972.1 |########################################
   3232.7 |#####
   3493.4 |
   3754.0 |#
   4014.7 |
   4275.3 |
   4536.0 |
   4796.6 |
   5057.3 |#
   5318.0 |
   5578.6 |
   5839.3 |
   6099.9 |
   6360.6 |
   6621.2 |
   6881.9 |#
   7142.5 |#
   7403.2 |#
   7663.8 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 7145.2-8747.3 ns)
   7145.2 |########################################
   7225.3 |
   7305.4 |
   7385.5 |
   7465.6 |
   7545.7 |
   7625.8 |
   7705.9 |
   7786.0 |
   7866.1 |
   7946.3 |
   8026.4 |
   8106.5 |
   8186.6 |
   8266.7 |
   8346.8 |
   8426.9 |
   8507.0 |
   8587.1 |
   8667.2 |##########
  (4 below, 3 above range)

warm-container-native (n=40, range 1608.0-6432.8 ns)
   1608.0 |########################################
   1849.2 |
   2090.4 |
   2331.7 |
   2572.9 |
   2814.2 |##
   3055.4 |
   3296.6 |##
   3537.9 |
   3779.1 |
   4020.4 |##
   4261.6 |########
   4502.9 |
   4744.1 |########
   4985.3 |##############
   5226.6 |
   5467.8 |########
   5709.1 |##
   5950.3 |
   6191.5 |#####
  (3 below, 3 above range)

warm-container-plusone (n=40, range 7144.4-8745.7 ns)
   7144.4 |###################################
   7224.5 |
   7304.5 |
   7384.6 |
   7464.7 |
   7544.7 |
   7624.8 |
   7704.8 |
   7784.9 |
   7865.0 |
   7945.0 |
   8025.1 |
   8105.2 |
   8185.2 |
   8265.3 |
   8345.4 |
   8425.4 |
   8505.5 |
   8585.5 |
   8665.6 |########################################
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=57.6% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=52.0% (high variance, measurements may be unstable)
- **warm-container-native**: CV=52.5% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 4.0x (possible bimodal distribution)
