# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 153084% faster than the next best (warm-container-native)

warm-container-kernel (4 ns) leads warm-container-native (5.36 us) by 153084%, a clear separation rather than a photo finish. CV 36.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 97% (significant)

warm-container-kernel is -7.97 us (97%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2384.6x slower than the field

warm-container-plusone (8.35 us) is 2384.6x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 36.7%)

warm-container-kernel wins on median (4 ns) yet has the highest variance (CV 36.7%), while warm-container-headroom is the steadiest (CV 4.6%, 8.18 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows warm-up / thermal drift (autocorr +0.83)

warm-container-native's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} (153084% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 153084% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 2384.6x the fastest

Fastest warm-container-kernel (4 ns) to slowest warm-container-plusone (8.35 us): 2384.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.8x its best-20%

warm-container-kernel's best 20% of batches run at 2 ns but its worst 20% at 5 ns (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 3.5 ns median (-100.0% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2384.59x (fastest 3.5 ns, slowest 8346.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8369ns | 8244ns | 8014ns | 8276ns | 9003ns | base |
| warm-container-kernel | 68ns | 66ns | 61ns | 66ns | 82ns | -99.19% |
| warm-container-minimum | 5705ns | 5508ns | 5285ns | 5591ns | 6466ns | -31.83% |
| warm-container-native | 5519ns | 5420ns | 5290ns | 5440ns | 5987ns | -34.05% |
| warm-container-plusone | 8565ns | 8412ns | 7939ns | 8539ns | 9271ns | +2.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8307ns | 7954ns | 8934ns | base | 3.945 |
| warm-container-kernel | 4ns | 2ns | 5ns | -99.96% | 8904.348 |
| warm-container-minimum | 5642ns | 5226ns | 6398ns | -32.08% | 5.808 |
| warm-container-native | 5459ns | 5232ns | 5925ns | -34.29% | 6.003 |
| warm-container-plusone | 8501ns | 7879ns | 9203ns | +2.34% | 3.854 |

## Performance model

- Peak throughput: **17022.338 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.007 | 0.0% |
| warm-container-kernel | 9362.286 | 55.0% |
| warm-container-minimum | 6.014 | 0.0% |
| warm-container-native | 6.112 | 0.0% |
| warm-container-plusone | 3.926 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8369ns | 8369ns | base |
| warm-container-kernel | 68ns | 68ns | -99.19% |
| warm-container-minimum | 5705ns | 5705ns | -31.83% |
| warm-container-native | 5519ns | 5519ns | -34.05% |
| warm-container-plusone | 8565ns | 8565ns | +2.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8178ns | base | --- | [8053, 8303] | --- | --- | --- | --- |
| warm-container-kernel | 4ns | -8175.6ns (-100.0%) | [-8299, -8051]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5449ns | -2643.3ns (-32.3%) | [-2703, -2570]ns | [5346, 5502] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 5361ns | -2814.6ns (-34.4%) | [-2981, -2665]ns | [5292, 5482] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8346ns | +89.2ns (+1.1%) | [+15, +356]ns | [8180, 8941] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 8942ns | -100.0% | -40.4% | -41.6% | -11.8% |
| 2 | 8772ns | -100.0% | -39.6% | -38.0% | -10.3% |
| 3 | 7979ns | -99.9% | -33.5% | -31.6% | +2.3% |
| 4 | 7971ns | -99.9% | -31.7% | -34.1% | +4.1% |
| 5 | 8005ns | -99.9% | -31.3% | -34.0% | +0.1% |
| 6 | 8061ns | -100.0% | -31.8% | -35.2% | -2.2% |
| 7 | 8172ns | -100.0% | -31.5% | -35.5% | -3.6% |
| 8 | 8184ns | -100.0% | -32.8% | -35.8% | +0.9% |
| 9 | 8073ns | -99.9% | -33.2% | -34.6% | +2.7% |
| 10 | 7913ns | -100.0% | -32.1% | -33.0% | +5.1% |
| 11 | 8068ns | -99.9% | -35.3% | -31.9% | -2.3% |
| 12 | 8070ns | -99.9% | -34.4% | -32.0% | +1.6% |
| 13 | 8015ns | -100.0% | -26.5% | -33.6% | -0.5% |
| 14 | 8010ns | -99.9% | -33.1% | -31.3% | -1.6% |
| 15 | 8018ns | -100.0% | -33.9% | -31.7% | -1.7% |
| 16 | 8045ns | -100.0% | -33.8% | -34.1% | -2.1% |
| 17 | 8005ns | -100.0% | -31.3% | -33.1% | +1.3% |
| 18 | 8061ns | -100.0% | -31.8% | -31.9% | +0.2% |
| 19 | 8006ns | -99.9% | -33.0% | -31.2% | +4.4% |
| 20 | 8007ns | -100.0% | -26.4% | -31.4% | +4.5% |
| 21 | 7895ns | -100.0% | -33.8% | -33.5% | +9.9% |
| 22 | 7930ns | -99.9% | -34.1% | -33.8% | +20.7% |
| 23 | 8305ns | -100.0% | -37.1% | -37.0% | +11.8% |
| 24 | 8290ns | -100.0% | -36.9% | -36.9% | +7.9% |
| 25 | 8303ns | -99.9% | -37.0% | -37.0% | +7.7% |
| 26 | 8303ns | -100.0% | -34.3% | -37.0% | +7.7% |
| 27 | 8298ns | -99.9% | -33.7% | -37.0% | +7.9% |
| 28 | 8318ns | -99.9% | -35.4% | -35.3% | +0.2% |
| 29 | 8288ns | -99.9% | -37.0% | -35.3% | -5.0% |
| 30 | 7935ns | -99.9% | -34.1% | -32.8% | +7.7% |
| 31 | 8928ns | -99.9% | -27.9% | -40.3% | +6.9% |
| 32 | 8930ns | -99.9% | -29.1% | -39.0% | +0.3% |
| 33 | 8918ns | -100.0% | -28.9% | -33.5% | +0.3% |
| 34 | 8927ns | -99.9% | -29.0% | -33.6% | +0.1% |
| 35 | 8933ns | -100.0% | -29.1% | -33.7% | +0.1% |
| 36 | 8927ns | -99.9% | -26.7% | -33.6% | +0.2% |
| 37 | 8963ns | -100.0% | -27.8% | -33.9% | +0.2% |
| 38 | 8903ns | -100.0% | -28.8% | -33.4% | +4.9% |
| 39 | 8305ns | -100.0% | -23.7% | -28.7% | +7.6% |
| 40 | 8288ns | -100.0% | -22.9% | -28.5% | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.749 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.111 | ok |
| warm-container-minimum | 0.772 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.830 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.759 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 10/40, lost 29/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.8ns | 8306.6ns | 0.0% |  |
| warm-container-kernel | 2.8ns | 3.7ns | 76.6% | HIGH |
| warm-container-minimum | 2.8ns | 5641.5ns | 0.1% |  |
| warm-container-native | 3.1ns | 5458.6ns | 0.1% |  |
| warm-container-plusone | 2.8ns | 8501.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7954.1-8933.5 ns)
   7954.1 |##########
   8003.0 |########################################
   8052.0 |#########################
   8101.0 |
   8150.0 |##########
   8198.9 |
   8247.9 |###############
   8296.9 |##############################
   8345.9 |
   8394.8 |
   8443.8 |
   8492.8 |
   8541.7 |
   8590.7 |
   8639.7 |
   8688.7 |
   8737.6 |#####
   8786.6 |
   8835.6 |
   8884.6 |###################################
  (4 below, 2 above range)

warm-container-kernel (n=40, range 1.9-5.4 ns)
      1.9 |
      2.1 |
      2.3 |
      2.4 |########################################
      2.6 |
      2.8 |##########################
      3.0 |
      3.1 |########################################
      3.3 |
      3.5 |
      3.7 |#############
      3.8 |
      4.0 |
      4.2 |##########################
      4.4 |
      4.5 |#################################
      4.7 |
      4.9 |##########################
      5.1 |
      5.2 |
  (4 below, 5 above range)

warm-container-minimum (n=40, range 5226.4-6398.0 ns)
   5226.4 |#################################
   5285.0 |########################################
   5343.6 |#################################
   5402.1 |#############
   5460.7 |########################################
   5519.3 |
   5577.9 |######
   5636.5 |
   5695.0 |
   5753.6 |
   5812.2 |
   5870.8 |#############
   5929.3 |
   5987.9 |
   6046.5 |
   6105.1 |
   6163.7 |
   6222.2 |
   6280.8 |########################################
   6339.4 |######
  (3 below, 3 above range)

warm-container-native (n=40, range 5231.8-5924.9 ns)
   5231.8 |########################################
   5266.4 |####################
   5301.1 |###############
   5335.7 |###############
   5370.4 |#####
   5405.0 |#####
   5439.7 |##########
   5474.4 |###################################
   5509.0 |
   5543.7 |
   5578.3 |
   5613.0 |
   5647.6 |
   5682.3 |
   5716.9 |
   5751.6 |
   5786.3 |
   5820.9 |
   5855.6 |
   5890.2 |##############################
  (3 below, 2 above range)

warm-container-plusone (n=40, range 7879.4-9202.7 ns)
   7879.4 |#########################
   7945.5 |#######
   8011.7 |###
   8077.9 |###
   8144.0 |#######
   8210.2 |###
   8276.4 |##############
   8342.5 |#######
   8408.7 |
   8474.9 |
   8541.0 |###
   8607.2 |
   8673.4 |###
   8739.5 |
   8805.7 |
   8871.9 |###
   8938.0 |########################################
   9004.2 |
   9070.4 |
   9136.5 |
  (2 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=34.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=77.1% of algo (FFI overhead may distort results)
- **warm-container-minimum**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.76 (measurement drift or warm-up artifact)
