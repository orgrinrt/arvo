# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (11.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 64 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 3618% faster than the next best (warm-container-minimum)

warm-container-kernel (64 ns) leads warm-container-minimum (2.36 us) by 3618%, a clear separation rather than a photo finish. CV 18.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 99% (significant)

warm-container-kernel is -11.37 us (99%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 179.8x slower than the field

warm-container-headroom (11.43 us) is 179.8x the fastest (64 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.55)

warm-container-native's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} (3618% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-native, warm-container-plusone, warm-container-headroom} with a 3618% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 179.8x the fastest

Fastest warm-container-kernel (64 ns) to slowest warm-container-headroom (11.43 us): 179.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 3.6x its best-20%

warm-container-minimum's best 20% of batches run at 2.35 us but its worst 20% at 8.40 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 63.5 ns median (-99.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 179.84x (fastest 63.5 ns, slowest 11428.7 ns)

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
| warm-container-headroom | 11524ns | 11517ns | 11514ns | 11518ns | 11554ns | base |
| warm-container-kernel | 385ns | 352ns | 304ns | 370ns | 510ns | -96.66% |
| warm-container-minimum | 3936ns | 2430ns | 2417ns | 2857ns | 8689ns | -65.85% |
| warm-container-native | 4007ns | 3025ns | 2517ns | 3031ns | 8425ns | -65.23% |
| warm-container-plusone | 9947ns | 9421ns | 9417ns | 9600ns | 11520ns | -13.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 11431ns | 11427ns | 11443ns | base | 2.866 |
| warm-container-kernel | 67ns | 55ns | 85ns | -99.41% | 489.312 |
| warm-container-minimum | 3816ns | 2351ns | 8396ns | -66.61% | 8.586 |
| warm-container-native | 3885ns | 2447ns | 8135ns | -66.01% | 8.434 |
| warm-container-plusone | 9869ns | 9346ns | 11430ns | -13.66% | 3.320 |

## Performance model

- Peak throughput: **594.431 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.867 | 0.5% |
| warm-container-kernel | 515.625 | 86.7% |
| warm-container-minimum | 13.868 | 2.3% |
| warm-container-native | 11.131 | 1.9% |
| warm-container-plusone | 3.504 | 0.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 11524ns | 11524ns | base |
| warm-container-kernel | 385ns | 385ns | -96.66% |
| warm-container-minimum | 3936ns | 3936ns | -65.85% |
| warm-container-native | 4007ns | 4007ns | -65.23% |
| warm-container-plusone | 9947ns | 9947ns | -13.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 11429ns | base | --- | [11428, 11429] | --- | --- | --- | --- |
| warm-container-kernel | 64ns | -11365.2ns (-99.4%) | [-11371, -11364]ns | [62, 65] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 2363ns | -9066.0ns (-79.3%) | [-9077, -8548]ns | [2351, 2880] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 2944ns | -8484.2ns (-74.2%) | [-8555, -8405]ns | [2874, 3023] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 9351ns | -2077.7ns (-18.2%) | [-2079, -2077]ns | [9350, 9352] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 11429ns | -99.3% | -79.4% | -74.2% | -18.2% |
| 1 | 2 | 11428ns | -99.4% | -79.4% | -78.9% | -0.0% |
| 1 | 3 | 11428ns | -99.5% | -74.7% | -73.4% | -18.2% |
| 1 | 4 | 11429ns | -99.4% | -79.4% | -74.9% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.104 | ok |
| warm-container-kernel | 0.022 | ok |
| warm-container-minimum | -0.415 | moderate- |
| warm-container-native | -0.552 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.416 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.0ns | 11431.4ns | 0.2% |  |
| warm-container-kernel | 125.4ns | 67.0ns | 187.2% | HIGH |
| warm-container-minimum | 33.5ns | 3816.4ns | 0.9% |  |
| warm-container-native | 33.0ns | 3885.2ns | 0.8% |  |
| warm-container-plusone | 21.4ns | 9869.5ns | 0.2% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 11427.2-11443.3 ns)
  11427.2 |##########################
  11428.0 |########################################
  11428.8 |#######################
  11429.6 |
  11430.4 |######
  11431.2 |
  11432.0 |###
  11432.8 |
  11433.6 |
  11434.4 |
  11435.3 |###
  11436.1 |
  11436.9 |
  11437.7 |
  11438.5 |###
  11439.3 |
  11440.1 |######
  11440.9 |###
  11441.7 |
  11442.5 |
  (3 below, 2 above range)

warm-container-kernel (n=40, range 55.1-85.2 ns)
     55.1 |##################
     56.6 |#####################
     58.1 |
     59.6 |###
     61.1 |
     62.6 |########################################
     64.1 |#######
     65.6 |###
     67.1 |
     68.6 |
     70.2 |
     71.7 |###
     73.2 |###
     74.7 |
     76.2 |
     77.7 |
     79.2 |
     80.7 |
     82.2 |
     83.7 |#########################
  (2 below, 3 above range)

warm-container-minimum (n=40, range 2350.8-8395.8 ns)
   2350.8 |########################################
   2653.0 |##############
   2955.3 |##
   3257.5 |
   3559.8 |
   3862.0 |##
   4164.3 |
   4466.5 |
   4768.8 |
   5071.0 |
   5373.3 |
   5675.5 |
   5977.8 |
   6280.0 |##
   6582.3 |
   6884.5 |
   7186.8 |##
   7489.0 |##
   7791.3 |##
   8093.5 |##
  (2 below, 4 above range)

warm-container-native (n=40, range 2447.0-8135.1 ns)
   2447.0 |####
   2731.4 |########################################
   3015.8 |###############
   3300.2 |
   3584.6 |
   3869.0 |
   4153.4 |
   4437.8 |
   4722.2 |
   5006.6 |
   5291.1 |##
   5575.5 |
   5859.9 |
   6144.3 |
   6428.7 |
   6713.1 |
   6997.5 |##
   7281.9 |##
   7566.3 |
   7850.7 |##
  (5 below, 4 above range)

warm-container-plusone (n=40, range 9345.9-11429.8 ns)
   9345.9 |########################################
   9450.0 |
   9554.2 |
   9658.4 |
   9762.6 |
   9866.8 |
   9971.0 |
  10075.2 |
  10179.4 |
  10283.6 |
  10387.8 |
  10492.0 |
  10596.2 |
  10700.4 |
  10804.6 |
  10908.8 |
  11013.0 |
  11117.2 |
  11221.4 |
  11325.6 |#############
  (3 below, 1 above range)

```

## Diagnostics

- **warm-container-kernel**: bridge=171.1% of algo (FFI overhead may distort results)
- **warm-container-minimum**: CV=63.1% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-container-native**: CV=57.2% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.3x (possible bimodal distribution)
