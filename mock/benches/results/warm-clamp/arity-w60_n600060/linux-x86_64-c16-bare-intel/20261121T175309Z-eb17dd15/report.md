# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (5.49 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-min-lanes at 2.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-min-lanes dominates: 14% faster than the next best (warm-clamp-head)

warm-clamp-min-lanes (2.68 us) leads warm-clamp-head (3.04 us) by 14%, a clear separation rather than a photo finish. CV 89.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn beats baseline by 53% (significant)

warm-clamp-accfit-dyn is -2.89 us (53%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.0x slower than the field

warm-clamp-acc64 (5.49 us) is 2.0x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-head's best 20% of batches run at 2.49 us but its worst 20% at 9.70 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2676.4 ns median (-51.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.05x (fastest 2676.4 ns, slowest 5485.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891686 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 6613ns | 5580ns | 4496ns | 5579ns | 11830ns | base |
| warm-clamp-accfit | 3792ns | 4026ns | 1736ns | 3512ns | 6686ns | -42.66% |
| warm-clamp-accfit-dyn | 3667ns | 4267ns | 1605ns | 3537ns | 6120ns | -44.55% |
| warm-clamp-head | 4439ns | 3120ns | 2557ns | 3209ns | 10011ns | -32.88% |
| warm-clamp-min-lanes | 4023ns | 2761ns | 2216ns | 3146ns | 8461ns | -39.16% |
| warm-clamp-minimum | 5424ns | 5389ns | 4385ns | 5173ns | 7214ns | -17.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 6489ns | 4420ns | 11589ns | base | 1.262 |
| warm-clamp-accfit | 3619ns | 1661ns | 6368ns | -44.23% | 2.264 |
| warm-clamp-accfit-dyn | 3504ns | 1538ns | 5816ns | -46.00% | 2.338 |
| warm-clamp-head | 4313ns | 2491ns | 9697ns | -33.54% | 1.900 |
| warm-clamp-min-lanes | 3888ns | 2147ns | 8142ns | -40.09% | 2.107 |
| warm-clamp-minimum | 5318ns | 4308ns | 7045ns | -18.05% | 1.540 |

## Performance model

- Peak throughput: **5.325 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.493 | 28.0% |
| warm-clamp-accfit | 2.130 | 40.0% |
| warm-clamp-accfit-dyn | 2.003 | 37.6% |
| warm-clamp-head | 2.696 | 50.6% |
| warm-clamp-min-lanes | 3.061 | 57.5% |
| warm-clamp-minimum | 1.547 | 29.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 6613ns | 6613ns | base |
| warm-clamp-accfit | 3792ns | 3792ns | -42.66% |
| warm-clamp-accfit-dyn | 3667ns | 3667ns | -44.55% |
| warm-clamp-head | 4439ns | 4439ns | -32.88% |
| warm-clamp-min-lanes | 4023ns | 4023ns | -39.16% |
| warm-clamp-minimum | 5424ns | 5424ns | -17.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 5486ns | base | --- | [5195, 5505] | --- | --- | --- | --- |
| warm-clamp-accfit | 3846ns | -2537.4ns (-46.3%) | [-3463, -1764]ns | [1892, 4417] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4090ns | -2888.2ns (-52.6%) | [-3765, -2221]ns | [1717, 4854] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 3038ns | -2162.9ns (-39.4%) | [-2459, -2038]ns | [2766, 3048] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2676ns | -2518.5ns (-45.9%) | [-3297, -1750]ns | [2675, 2689] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 5296ns | no significant difference | [-130, +92]ns | [4823, 5319] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 5199ns | -65.1% | -9.3% | -41.5% | -48.5% | +2.2% |
| 1 | 2 | 5500ns | -24.2% | -21.1% | -44.6% | -60.0% | -2.1% |
| 1 | 3 | 9464ns | -53.5% | -53.1% | -73.6% | -77.4% | -54.4% |
| 1 | 4 | 4422ns | -57.4% | -65.2% | -43.5% | -39.5% | +5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.474 | moderate- |
| warm-clamp-accfit | -0.250 | moderate- |
| warm-clamp-accfit-dyn | -0.085 | ok |
| warm-clamp-head | 0.250 | moderate+ |
| warm-clamp-min-lanes | -0.252 | moderate- |
| warm-clamp-minimum | 0.072 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.2ns | 6489.4ns | 0.5% |  |
| warm-clamp-accfit | 48.2ns | 3619.0ns | 1.3% |  |
| warm-clamp-accfit-dyn | 45.8ns | 3504.3ns | 1.3% |  |
| warm-clamp-head | 34.5ns | 4312.6ns | 0.8% |  |
| warm-clamp-min-lanes | 36.2ns | 3887.5ns | 0.9% |  |
| warm-clamp-minimum | 25.4ns | 5318.3ns | 0.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 4420.1-11589.1 ns)
   4420.1 |############
   4778.6 |###
   5137.0 |########################################
   5495.5 |##################
   5853.9 |###
   6212.4 |###
   6570.8 |
   6929.3 |
   7287.7 |###
   7646.2 |
   8004.6 |###
   8363.1 |###
   8721.5 |
   9080.0 |
   9438.4 |
   9796.9 |
  10155.3 |######
  10513.8 |
  10872.2 |
  11230.7 |
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 1660.9-6367.6 ns)
   1660.9 |########################################
   1896.2 |##########
   2131.6 |
   2366.9 |
   2602.2 |
   2837.6 |
   3072.9 |###
   3308.2 |
   3543.6 |#######
   3778.9 |#######
   4014.3 |#######
   4249.6 |#######
   4484.9 |##############
   4720.3 |#######
   4955.6 |###
   5191.0 |#######
   5426.3 |
   5661.6 |
   5897.0 |
   6132.3 |###
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1538.3-5815.7 ns)
   1538.3 |########################################
   1752.2 |#################
   1966.0 |
   2179.9 |
   2393.8 |
   2607.6 |####
   2821.5 |
   3035.4 |
   3249.2 |
   3463.1 |
   3677.0 |
   3890.9 |####
   4104.7 |####
   4318.6 |########
   4532.5 |####
   4746.3 |#############
   4960.2 |#############
   5174.1 |#############
   5387.9 |####
   5601.8 |#################
  (5 below, 2 above range)

warm-clamp-head (n=40, range 2490.5-9697.4 ns)
   2490.5 |###########################
   2850.9 |########################################
   3211.2 |##
   3571.5 |
   3931.9 |
   4292.2 |
   4652.6 |
   5012.9 |
   5373.3 |
   5733.6 |
   6094.0 |
   6454.3 |
   6814.7 |
   7175.0 |
   7535.3 |
   7895.7 |##
   8256.0 |
   8616.4 |
   8976.7 |#####
   9337.1 |#####
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 2147.0-8142.2 ns)
   2147.0 |###############
   2446.8 |########################################
   2746.5 |
   3046.3 |
   3346.0 |##
   3645.8 |
   3945.6 |
   4245.3 |
   4545.1 |
   4844.8 |##
   5144.6 |
   5444.4 |
   5744.1 |
   6043.9 |
   6343.6 |##
   6643.4 |#####
   6943.2 |##
   7242.9 |#######
   7542.7 |
   7842.4 |
  (6 below, 3 above range)

warm-clamp-minimum (n=40, range 4308.4-7045.0 ns)
   4308.4 |##############
   4445.3 |##
   4582.1 |####
   4718.9 |
   4855.8 |##
   4992.6 |####
   5129.4 |##
   5266.3 |########################################
   5403.1 |
   5539.9 |
   5676.8 |
   5813.6 |
   5950.4 |
   6087.2 |
   6224.1 |
   6360.9 |
   6497.7 |
   6634.6 |
   6771.4 |
   6908.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=43.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=51.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=51.7% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-head**: CV=65.6% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=61.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=27.0% (high variance, measurements may be unstable)
