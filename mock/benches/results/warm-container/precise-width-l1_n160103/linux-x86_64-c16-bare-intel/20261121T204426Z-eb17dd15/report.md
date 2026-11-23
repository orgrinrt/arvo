# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (3.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 58 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 3987% faster than the next best (warm-container-minimum)

warm-container-kernel (58 ns) leads warm-container-minimum (2.36 us) by 3987%, a clear separation rather than a photo finish. CV 9.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 98% (significant)

warm-container-kernel is -3.24 us (98%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 57.0x slower than the field

warm-container-headroom (3.29 us) is 57.0x the fastest (58 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (3987% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 3987% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 57.0x the fastest

Fastest warm-container-kernel (58 ns) to slowest warm-container-headroom (3.29 us): 57.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 3.8x its best-20%

warm-container-native's best 20% of batches run at 2.42 us but its worst 20% at 9.13 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 57.8 ns median (-98.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 56.97x (fastest 57.8 ns, slowest 3292.7 ns)

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
| warm-container-headroom | 4739ns | 3344ns | 3333ns | 3587ns | 9600ns | base |
| warm-container-kernel | 336ns | 320ns | 277ns | 327ns | 424ns | -92.91% |
| warm-container-minimum | 3954ns | 2414ns | 2401ns | 2853ns | 8809ns | -16.58% |
| warm-container-native | 4187ns | 2938ns | 2470ns | 3026ns | 9385ns | -11.66% |
| warm-container-plusone | 4350ns | 3344ns | 3334ns | 3494ns | 7932ns | -8.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 4656ns | 3283ns | 9401ns | base | 7.038 |
| warm-container-kernel | 59ns | 52ns | 65ns | -98.73% | 554.427 |
| warm-container-minimum | 3858ns | 2349ns | 8566ns | -17.14% | 8.493 |
| warm-container-native | 4086ns | 2417ns | 9131ns | -12.25% | 8.021 |
| warm-container-plusone | 4272ns | 3283ns | 7759ns | -8.24% | 7.670 |

## Performance model

- Peak throughput: **633.504 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 9.952 | 1.6% |
| warm-container-kernel | 566.920 | 89.5% |
| warm-container-minimum | 13.873 | 2.2% |
| warm-container-native | 11.409 | 1.8% |
| warm-container-plusone | 9.952 | 1.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 4739ns | 4739ns | base |
| warm-container-kernel | 336ns | 336ns | -92.91% |
| warm-container-minimum | 3954ns | 3954ns | -16.58% |
| warm-container-native | 4187ns | 4187ns | -11.66% |
| warm-container-plusone | 4350ns | 4350ns | -8.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 3293ns | base | --- | [3283, 4012] | --- | --- | --- | --- |
| warm-container-kernel | 58ns | -3237.1ns (-98.3%) | [-3948, -3230]ns | [57, 64] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2362ns | -932.8ns (-28.3%) | [-943, -526]ns | [2350, 2872] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2872ns | -410.8ns (-12.5%) | [-420, -409]ns | [2872, 2884] | YES | 0.0002 | 0.0002 | 0 |
| warm-container-plusone | 3293ns | no significant difference | [-134, +10]ns | [3283, 3301] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3288ns | -98.0% | -28.4% | -12.5% | +22.4% |
| 1 | 2 | 4024ns | -98.4% | -41.6% | -41.5% | -18.4% |
| 1 | 3 | 3283ns | -98.2% | -12.5% | -12.3% | +0.0% |
| 1 | 4 | 3292ns | -98.4% | -28.6% | -12.6% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.423 | moderate- |
| warm-container-kernel | 0.267 | moderate+ |
| warm-container-minimum | -0.423 | moderate- |
| warm-container-native | -0.416 | moderate- |
| warm-container-plusone | -0.083 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 28.1ns | 4655.9ns | 0.6% |  |
| warm-container-kernel | 97.3ns | 59.1ns | 164.6% | HIGH |
| warm-container-minimum | 32.3ns | 3858.0ns | 0.8% |  |
| warm-container-native | 34.1ns | 4085.5ns | 0.8% |  |
| warm-container-plusone | 25.7ns | 4272.1ns | 0.6% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3282.5-9401.1 ns)
   3282.5 |########################################
   3588.5 |
   3894.4 |#################
   4200.3 |
   4506.2 |
   4812.2 |
   5118.1 |
   5424.0 |
   5730.0 |
   6035.9 |
   6341.8 |##
   6647.8 |##
   6953.7 |
   7259.6 |
   7565.5 |
   7871.5 |##
   8177.4 |
   8483.3 |
   8789.3 |
   9095.2 |##
  (6 below, 4 above range)

warm-container-kernel (n=40, range 51.7-65.1 ns)
     51.7 |##########################
     52.4 |
     53.1 |
     53.7 |
     54.4 |
     55.1 |
     55.7 |
     56.4 |#################
     57.1 |######################
     57.7 |#################
     58.4 |
     59.1 |
     59.7 |
     60.4 |
     61.1 |
     61.7 |
     62.4 |
     63.1 |####
     63.7 |########
     64.4 |########################################
  (4 below, 5 above range)

warm-container-minimum (n=40, range 2349.5-8566.0 ns)
   2349.5 |########################################
   2660.3 |################
   2971.1 |##
   3282.0 |
   3592.8 |##
   3903.6 |
   4214.4 |
   4525.3 |
   4836.1 |
   5146.9 |
   5457.7 |
   5768.6 |
   6079.4 |
   6390.2 |##
   6701.0 |
   7011.9 |##
   7322.7 |####
   7633.5 |
   7944.3 |##
   8255.2 |##
  (2 below, 3 above range)

warm-container-native (n=40, range 2417.1-9130.8 ns)
   2417.1 |
   2752.7 |########################################
   3088.4 |#
   3424.1 |#
   3759.8 |#
   4095.5 |
   4431.2 |
   4766.8 |
   5102.5 |
   5438.2 |
   5773.9 |
   6109.6 |#
   6445.3 |
   6781.0 |
   7116.6 |
   7452.3 |
   7788.0 |#
   8123.7 |
   8459.4 |#
   8795.1 |#
  (7 below, 4 above range)

warm-container-plusone (n=40, range 3282.6-7758.7 ns)
   3282.6 |########################################
   3506.4 |
   3730.2 |
   3954.0 |###########
   4177.9 |#
   4401.7 |
   4625.5 |
   4849.3 |
   5073.1 |
   5296.9 |
   5520.7 |
   5744.5 |
   5968.3 |
   6192.1 |
   6415.9 |
   6639.7 |
   6863.5 |
   7087.3 |
   7311.1 |
   7534.9 |
  (2 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=56.3% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=164.3% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=65.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=63.9% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=51.8% (high variance, measurements may be unstable)
