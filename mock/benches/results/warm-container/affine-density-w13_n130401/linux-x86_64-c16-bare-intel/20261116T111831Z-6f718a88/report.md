# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred dominates: 20% faster than the next best (warm-container-native)

warm-container-lanes-deferred (909 ns) leads warm-container-native (1.09 us) by 20%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-lanes-deferred beats baseline by 79% (significant)

warm-container-lanes-deferred is -3.49 us (79%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 4.9x slower than the field

warm-container-plusone (4.50 us) is 4.9x the fastest (909 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-lanes-deferred, warm-container-native, warm-container-kernel} vs {warm-container-minimum, warm-container-headroom, warm-container-plusone} (303% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-native, warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 303% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.9x the fastest

Fastest warm-container-lanes-deferred (909 ns) to slowest warm-container-plusone (4.50 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 3.8x its best-20%

warm-container-minimum's best 20% of batches run at 1.49 us but its worst 20% at 5.71 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 908.5 ns median (-79.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.95x (fastest 908.5 ns, slowest 4495.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112561731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3665ns | 4588ns | 1556ns | 3667ns | 5768ns | base |
| warm-container-kernel | 1290ns | 1316ns | 1102ns | 1282ns | 1502ns | -64.80% |
| warm-container-lanes-deferred | 1138ns | 1098ns | 1014ns | 1122ns | 1310ns | -68.95% |
| warm-container-minimum | 3950ns | 4546ns | 1541ns | 4086ns | 5949ns | +7.77% |
| warm-container-native | 1274ns | 1309ns | 1062ns | 1259ns | 1531ns | -65.24% |
| warm-container-plusone | 3991ns | 4649ns | 1629ns | 4030ns | 6234ns | +8.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3532ns | 1504ns | 5527ns | base | 4.638 |
| warm-container-kernel | 1058ns | 912ns | 1208ns | -70.05% | 15.485 |
| warm-container-lanes-deferred | 933ns | 840ns | 1053ns | -73.57% | 17.552 |
| warm-container-minimum | 3808ns | 1490ns | 5712ns | +7.80% | 4.303 |
| warm-container-native | 1047ns | 880ns | 1260ns | -70.35% | 15.643 |
| warm-container-plusone | 3846ns | 1575ns | 5986ns | +8.89% | 4.260 |

## Performance model

- Peak throughput: **19.511 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.693 | 18.9% |
| warm-container-kernel | 15.045 | 77.1% |
| warm-container-lanes-deferred | 18.033 | 92.4% |
| warm-container-minimum | 3.729 | 19.1% |
| warm-container-native | 15.051 | 77.1% |
| warm-container-plusone | 3.644 | 18.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3665ns | 3665ns | base |
| warm-container-kernel | 1290ns | 1290ns | -64.80% |
| warm-container-lanes-deferred | 1138ns | 1138ns | -68.95% |
| warm-container-minimum | 3950ns | 3950ns | +7.77% |
| warm-container-native | 1274ns | 1274ns | -65.24% |
| warm-container-plusone | 3991ns | 3991ns | +8.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4436ns | base | --- | [1630, 4761] | --- | --- | --- | --- |
| warm-container-kernel | 1089ns | -3355.5ns (-75.6%) | [-3710, -630]ns | [991, 1090] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 909ns | -3494.1ns (-78.8%) | [-3842, -728]ns | [908, 941] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 4394ns | no significant difference | [-32, +204]ns | [3592, 4789] | no | 0.3352 | 0.2682 | 0 |
| warm-container-native | 1089ns | -3437.7ns (-77.5%) | [-3754, -664]ns | [945, 1096] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4496ns | no significant difference | [-14, +434]ns | [2536, 4947] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4558ns | -75.6% | -77.2% | -23.6% | -78.7% | -3.4% |
| 1 | 2 | 2136ns | -53.4% | -57.4% | +115.5% | -43.0% | +109.7% |
| 1 | 3 | 4583ns | -76.2% | -80.2% | -2.3% | -76.1% | -1.2% |
| 1 | 4 | 4349ns | -79.1% | -80.7% | -0.6% | -79.3% | +10.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.487 | moderate- |
| warm-container-kernel | -0.476 | moderate- |
| warm-container-lanes-deferred | -0.006 | ok |
| warm-container-minimum | -0.208 | moderate- |
| warm-container-native | -0.198 | ok |
| warm-container-plusone | 0.071 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-lanes-deferred**: won 4/4, lost 0/4
- **warm-container-minimum**: won 3/4, lost 1/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 46.6ns | 3532.3ns | 1.3% |  |
| warm-container-kernel | 86.0ns | 1058.1ns | 8.1% | HIGH |
| warm-container-lanes-deferred | 81.2ns | 933.5ns | 8.7% | HIGH |
| warm-container-minimum | 50.2ns | 3807.8ns | 1.3% |  |
| warm-container-native | 83.3ns | 1047.4ns | 8.0% | HIGH |
| warm-container-plusone | 51.1ns | 3846.3ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1504.1-5527.1 ns)
   1504.1 |########################################
   1705.2 |###
   1906.4 |
   2107.5 |
   2308.7 |###
   2509.8 |
   2711.0 |###
   2912.1 |###
   3113.3 |
   3314.4 |
   3515.6 |
   3716.7 |
   3917.9 |
   4119.0 |###
   4320.2 |##########
   4521.4 |##########
   4722.5 |##############
   4923.7 |##########
   5124.8 |##########
   5326.0 |###
  (3 below, 4 above range)

warm-container-kernel (n=40, range 912.5-1208.3 ns)
    912.5 |
    927.2 |#####
    942.0 |#####
    956.8 |
    971.6 |
    986.4 |###################################
   1001.2 |
   1016.0 |
   1030.8 |
   1045.6 |
   1060.4 |#####
   1075.2 |########################################
   1090.0 |##############################
   1104.7 |
   1119.5 |
   1134.3 |##########
   1149.1 |
   1163.9 |#####
   1178.7 |
   1193.5 |#####
  (7 below, 5 above range)

warm-container-lanes-deferred (n=40, range 839.8-1053.4 ns)
    839.8 |#####
    850.4 |
    861.1 |
    871.8 |
    882.5 |#####
    893.2 |##
    903.9 |########################################
    914.5 |##
    925.2 |
    935.9 |
    946.6 |
    957.3 |#####
    968.0 |
    978.6 |
    989.3 |#################
   1000.0 |
   1010.7 |
   1021.4 |
   1032.1 |#####
   1042.8 |
  (6 below, 4 above range)

warm-container-minimum (n=40, range 1489.7-5712.4 ns)
   1489.7 |########################################
   1700.9 |
   1912.0 |#####
   2123.1 |#####
   2334.3 |
   2545.4 |
   2756.5 |
   2967.7 |#####
   3178.8 |
   3389.9 |
   3601.1 |
   3812.2 |
   4023.3 |#################
   4234.5 |######################
   4445.6 |###########
   4656.8 |######################
   4867.9 |#################
   5079.0 |###########
   5290.2 |######################
   5501.3 |#####
  (4 below, 3 above range)

warm-container-native (n=40, range 880.1-1259.9 ns)
    880.1 |#######
    899.1 |#############################
    918.1 |
    937.1 |#######
    956.1 |
    975.0 |###
    994.0 |##########
   1013.0 |
   1032.0 |
   1051.0 |
   1070.0 |###
   1089.0 |########################################
   1108.0 |
   1127.0 |
   1146.0 |
   1165.0 |###
   1184.0 |
   1202.9 |##################
   1221.9 |
   1240.9 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 1574.6-5985.9 ns)
   1574.6 |
   1795.2 |########################################
   2015.7 |
   2236.3 |
   2456.8 |
   2677.4 |
   2898.0 |
   3118.5 |##########
   3339.1 |
   3559.7 |#####
   3780.2 |
   4000.8 |
   4221.3 |###############
   4441.9 |#####
   4662.5 |####################
   4883.0 |#########################
   5103.6 |#####
   5324.2 |####################
   5544.7 |
   5765.3 |#####
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=48.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-container-minimum**: CV=43.6% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **warm-container-plusone**: CV=45.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 3.8x (possible bimodal distribution)
