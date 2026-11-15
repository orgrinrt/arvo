# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.74 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 1.74 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 64% (significant)

warm-container-kernel is -5.64 us (64%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 5.0x slower than the field

warm-container-headroom (8.74 us) is 5.0x the fastest (1.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.61)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.0x the fastest

Fastest warm-container-native (1.74 us) to slowest warm-container-headroom (8.74 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 4.6x its best-20%

warm-container-kernel's best 20% of batches run at 1.61 us but its worst 20% at 7.45 us (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-minimum's edge over baseline is significant but tiny (-3 ns, 0.03%)

warm-container-minimum differs from baseline warm-container-headroom by -3 ns (0.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 1736.2 ns median (-80.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.03x (fastest 1736.2 ns, slowest 8740.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8408ns | 8804ns | 7199ns | 8671ns | 8828ns | base |
| warm-container-kernel | 3890ns | 1923ns | 1668ns | 3345ns | 7748ns | -53.73% |
| warm-container-lanes-deferred | 4490ns | 3300ns | 2084ns | 3852ns | 8813ns | -46.59% |
| warm-container-minimum | 8805ns | 8801ns | 8792ns | 8801ns | 8828ns | +4.72% |
| warm-container-native | 3601ns | 1800ns | 1787ns | 3131ns | 6826ns | -57.17% |
| warm-container-plusone | 8616ns | 7261ns | 7202ns | 7749ns | 12633ns | +2.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8342ns | 7148ns | 8743ns | base | 16.693 |
| warm-container-kernel | 3749ns | 1612ns | 7446ns | -55.06% | 37.146 |
| warm-container-lanes-deferred | 4354ns | 2030ns | 8512ns | -47.81% | 31.986 |
| warm-container-minimum | 8736ns | 8728ns | 8739ns | +4.72% | 15.941 |
| warm-container-native | 3462ns | 1723ns | 6524ns | -58.50% | 40.229 |
| warm-container-plusone | 8546ns | 7147ns | 12522ns | +2.44% | 16.296 |

## Performance model

- Peak throughput: **86.379 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 15.933 | 18.4% |
| warm-container-kernel | 74.855 | 86.7% |
| warm-container-lanes-deferred | 43.641 | 50.5% |
| warm-container-minimum | 15.938 | 18.5% |
| warm-container-native | 80.210 | 92.9% |
| warm-container-plusone | 19.441 | 22.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8408ns | 8408ns | base |
| warm-container-kernel | 3890ns | 3890ns | -53.73% |
| warm-container-lanes-deferred | 4490ns | 4490ns | -46.59% |
| warm-container-minimum | 8805ns | 8805ns | +4.72% |
| warm-container-native | 3601ns | 3601ns | -57.17% |
| warm-container-plusone | 8616ns | 8616ns | +2.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8741ns | base | --- | [8739, 8741] | --- | --- | --- | --- |
| warm-container-kernel | 1860ns | -5636.9ns (-64.5%) | [-6878, -2849]ns | [1850, 5480] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 3191ns | -3961.3ns (-45.3%) | [-6326, -3721]ns | [2416, 4213] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8738ns | -2.8ns (-0.0%) | [-3, -1]ns | [8737, 8738] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-native | 1736ns | -5421.0ns (-62.0%) | [-6668, -3503]ns | [1726, 5238] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 7163ns | no significant difference | [-736, +5]ns | [7156, 8742] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 8742ns | -69.9% | -77.4% | -0.0% | -80.2% | -18.1% |
| 1 | 2 | 7152ns | -78.0% | -55.3% | +22.2% | -75.9% | +0.0% |
| 1 | 3 | 8742ns | -38.8% | -71.2% | -0.0% | -40.3% | +0.1% |
| 1 | 4 | 8740ns | -78.8% | -72.3% | -0.0% | -75.3% | +5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.417 | moderate- |
| warm-container-kernel | -0.606 | HIGH- (thermal bounce) |
| warm-container-lanes-deferred | -0.493 | moderate- |
| warm-container-minimum | 0.243 | moderate+ |
| warm-container-native | -0.339 | moderate- |
| warm-container-plusone | 0.288 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 0/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 23.6ns | 8342.5ns | 0.3% |  |
| warm-container-kernel | 50.0ns | 3749.1ns | 1.3% |  |
| warm-container-lanes-deferred | 46.1ns | 4353.9ns | 1.1% |  |
| warm-container-minimum | 24.9ns | 8736.1ns | 0.3% |  |
| warm-container-native | 50.3ns | 3461.8ns | 1.5% |  |
| warm-container-plusone | 24.7ns | 8545.8ns | 0.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7147.9-8743.2 ns)
   7147.9 |##########
   7227.7 |
   7307.4 |
   7387.2 |
   7466.9 |
   7546.7 |
   7626.5 |
   7706.2 |
   7786.0 |
   7865.8 |
   7945.5 |
   8025.3 |
   8105.0 |
   8184.8 |
   8264.6 |
   8344.3 |
   8424.1 |
   8503.9 |
   8583.6 |
   8663.4 |########################################
  (3 below, 3 above range)

warm-container-kernel (n=40, range 1612.3-7446.0 ns)
   1612.3 |########################################
   1903.9 |
   2195.6 |
   2487.3 |
   2779.0 |
   3070.7 |
   3362.4 |##
   3654.1 |
   3945.8 |
   4237.4 |
   4529.1 |
   4820.8 |##
   5112.5 |##
   5404.2 |##
   5695.9 |####
   5987.6 |##
   6279.3 |####
   6571.0 |########
   6862.6 |
   7154.3 |##
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 2030.3-8511.7 ns)
   2030.3 |
   2354.4 |########################################
   2678.4 |
   3002.5 |#######################
   3326.6 |
   3650.7 |
   3974.7 |
   4298.8 |
   4622.9 |
   4946.9 |###
   5271.0 |
   5595.1 |
   5919.1 |
   6243.2 |
   6567.3 |###
   6891.4 |######
   7215.4 |######
   7539.5 |##########
   7863.6 |
   8187.6 |###
  (7 below, 4 above range)

warm-container-minimum (n=40, range 8727.9-8739.3 ns)
   8727.9 |#####
   8728.4 |
   8729.0 |
   8729.6 |
   8730.2 |
   8730.7 |
   8731.3 |
   8731.9 |
   8732.5 |
   8733.0 |
   8733.6 |
   8734.2 |
   8734.8 |
   8735.3 |#####
   8735.9 |#################
   8736.5 |#################
   8737.0 |##################################
   8737.6 |#################
   8738.2 |########################################
   8738.8 |##################################
  (6 below, 4 above range)

warm-container-native (n=40, range 1723.5-6523.8 ns)
   1723.5 |########################################
   1963.5 |
   2203.5 |##
   2443.5 |##
   2683.5 |
   2923.5 |
   3163.6 |
   3403.6 |
   3643.6 |
   3883.6 |
   4123.6 |
   4363.6 |
   4603.6 |
   4843.7 |
   5083.7 |########
   5323.7 |##
   5563.7 |######
   5803.7 |####
   6043.7 |########
   6283.8 |##
  (3 below, 1 above range)

warm-container-plusone (n=40, range 7147.5-12521.6 ns)
   7147.5 |########################################
   7416.2 |
   7684.9 |
   7953.6 |
   8222.3 |##
   8491.0 |#######################
   8759.7 |
   9028.4 |
   9297.1 |
   9565.8 |
   9834.5 |##
  10103.2 |
  10372.0 |
  10640.7 |
  10909.4 |
  11178.1 |
  11446.8 |
  11715.5 |
  11984.2 |
  12252.9 |##
  (7 below, 3 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=65.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.2% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **warm-container-native**: CV=61.4% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=31.6% (high variance, measurements may be unstable)
