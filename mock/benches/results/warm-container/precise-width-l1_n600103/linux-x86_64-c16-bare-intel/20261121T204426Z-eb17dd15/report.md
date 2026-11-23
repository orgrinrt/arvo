# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 1.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 301% faster than the next best (warm-container-minimum)

warm-container-kernel (1.01 us) leads warm-container-minimum (4.03 us) by 301%, a clear separation rather than a photo finish. CV 14.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 89% (significant)

warm-container-kernel is -7.50 us (89%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 8.4x slower than the field

warm-container-headroom (8.43 us) is 8.4x the fastest (1.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (301% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 301% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.4x the fastest

Fastest warm-container-kernel (1.01 us) to slowest warm-container-headroom (8.43 us): 8.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone is inconsistent: worst-20% is 2.5x its best-20%

warm-container-plusone's best 20% of batches run at 3.31 us but its worst 20% at 8.36 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1006.4 ns median (-88.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 8.38x (fastest 1006.4 ns, slowest 8430.8 ns)

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
| warm-container-headroom | 8974ns | 8498ns | 8491ns | 8661ns | 10396ns | base |
| warm-container-kernel | 1299ns | 1216ns | 1110ns | 1259ns | 1611ns | -85.52% |
| warm-container-minimum | 4404ns | 4114ns | 3367ns | 3871ns | 7039ns | -50.93% |
| warm-container-native | 4646ns | 4118ns | 3368ns | 3880ns | 8222ns | -48.23% |
| warm-container-plusone | 4865ns | 4120ns | 3375ns | 4124ns | 8577ns | -45.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8898ns | 8424ns | 10306ns | base | 3.683 |
| warm-container-kernel | 1071ns | 922ns | 1321ns | -87.97% | 30.608 |
| warm-container-minimum | 4307ns | 3300ns | 6857ns | -51.59% | 7.607 |
| warm-container-native | 4542ns | 3300ns | 8005ns | -48.96% | 7.215 |
| warm-container-plusone | 4758ns | 3307ns | 8357ns | -46.53% | 6.887 |

## Performance model

- Peak throughput: **35.539 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.887 | 10.9% |
| warm-container-kernel | 32.560 | 91.6% |
| warm-container-minimum | 8.126 | 22.9% |
| warm-container-native | 8.123 | 22.9% |
| warm-container-plusone | 8.115 | 22.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8974ns | 8974ns | base |
| warm-container-kernel | 1299ns | 1299ns | -85.52% |
| warm-container-minimum | 4404ns | 4404ns | -50.93% |
| warm-container-native | 4646ns | 4646ns | -48.23% |
| warm-container-plusone | 4865ns | 4865ns | -45.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8431ns | base | --- | [8430, 8432] | --- | --- | --- | --- |
| warm-container-kernel | 1006ns | -7502.2ns (-89.0%) | [-7510, -7424]ns | [1006, 1087] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4033ns | -5125.7ns (-60.8%) | [-5131, -4753]ns | [3310, 4034] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 4034ns | -5119.5ns (-60.7%) | [-5129, -4398]ns | [3343, 4036] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4038ns | -4394.4ns (-52.1%) | [-5117, -4381]ns | [4036, 4049] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 8431ns | -89.1% | -60.9% | -60.8% | -52.0% |
| 1 | 2 | 8429ns | -85.1% | -52.1% | -60.8% | -52.1% |
| 1 | 3 | 10304ns | -89.7% | -60.9% | -60.8% | -60.7% |
| 1 | 4 | 8430ns | -88.1% | -60.8% | -52.1% | -60.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.417 | moderate- |
| warm-container-kernel | -0.449 | moderate- |
| warm-container-minimum | -0.250 | moderate- |
| warm-container-native | 0.254 | moderate+ |
| warm-container-plusone | -0.088 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 21.1ns | 8898.3ns | 0.2% |  |
| warm-container-kernel | 83.8ns | 1070.6ns | 7.8% | HIGH |
| warm-container-minimum | 26.1ns | 4307.4ns | 0.6% |  |
| warm-container-native | 27.6ns | 4541.6ns | 0.6% |  |
| warm-container-plusone | 28.6ns | 4758.2ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8424.4-10306.4 ns)
   8424.4 |########################################
   8518.5 |
   8612.6 |
   8706.7 |
   8800.8 |
   8894.9 |
   8989.0 |
   9083.1 |
   9177.2 |
   9271.3 |
   9365.4 |
   9459.5 |
   9553.6 |
   9647.7 |
   9741.8 |
   9835.9 |
   9930.0 |
  10024.1 |
  10118.2 |
  10212.3 |#############
  (4 below, 1 above range)

warm-container-kernel (n=40, range 922.0-1320.6 ns)
    922.0 |#############################
    942.0 |
    961.9 |
    981.8 |###
   1001.8 |########################################
   1021.7 |
   1041.6 |#######
   1061.5 |###
   1081.5 |
   1101.4 |##############
   1121.3 |
   1141.2 |
   1161.2 |
   1181.1 |
   1201.0 |
   1221.0 |##################
   1240.9 |
   1260.8 |
   1280.7 |###
   1300.7 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 3299.9-6856.6 ns)
   3299.9 |#####################
   3477.8 |
   3655.6 |
   3833.5 |
   4011.3 |########################################
   4189.1 |
   4367.0 |
   4544.8 |
   4722.6 |
   4900.5 |
   5078.3 |
   5256.1 |
   5434.0 |
   5611.8 |
   5789.6 |####
   5967.5 |
   6145.3 |
   6323.1 |
   6501.0 |
   6678.8 |
  (6 below, 3 above range)

warm-container-native (n=40, range 3300.1-8004.6 ns)
   3300.1 |##########################
   3535.4 |
   3770.6 |
   4005.8 |########################################
   4241.0 |
   4476.2 |
   4711.5 |
   4946.7 |
   5181.9 |
   5417.1 |
   5652.3 |
   5887.6 |##
   6122.8 |
   6358.0 |##
   6593.2 |
   6828.4 |
   7063.7 |
   7298.9 |
   7534.1 |
   7769.3 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 3306.8-8356.8 ns)
   3306.8 |#####
   3559.3 |
   3811.8 |########################################
   4064.3 |#
   4316.8 |
   4569.3 |
   4821.8 |
   5074.3 |
   5326.8 |
   5579.3 |
   5831.8 |
   6084.3 |
   6336.8 |#
   6589.3 |#
   6841.8 |#
   7094.3 |
   7346.8 |
   7599.3 |#
   7851.8 |
   8104.3 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=42.4% (high variance, measurements may be unstable)
- **warm-container-native**: CV=48.5% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=46.6% (high variance, measurements may be unstable)
