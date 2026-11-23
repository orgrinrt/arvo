# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 67% (significant)

warm-container-native is -1.25 us (67%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.9x slower than the field

warm-container-plusone (4.06 us) is 3.9x the fastest (1.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.79)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.9x the fastest

Fastest warm-container-minimum (1.03 us) to slowest warm-container-plusone (4.06 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 6.2x its best-20%

warm-container-native's best 20% of batches run at 214 ns but its worst 20% at 1.32 us (6.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 1032.2 ns median (-44.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.94x (fastest 1032.2 ns, slowest 4062.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112855779 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 3492ns | 1930ns | 1511ns | 3143ns | 6523ns | base |
| warm-container-kernel | 3684ns | 2493ns | 2040ns | 2755ns | 8117ns | +5.50% |
| warm-container-lanes-deferred | 4063ns | 2494ns | 2040ns | 3428ns | 7993ns | +16.35% |
| warm-container-minimum | 1338ns | 1285ns | 1154ns | 1314ns | 1594ns | -61.69% |
| warm-container-native | 1161ns | 1355ns | 266ns | 1283ns | 1692ns | -66.75% |
| warm-container-plusone | 3771ns | 4205ns | 1516ns | 3699ns | 6244ns | +7.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 3361ns | 1460ns | 6256ns | base | 9.748 |
| warm-container-kernel | 3580ns | 1988ns | 7858ns | +6.49% | 9.154 |
| warm-container-lanes-deferred | 3952ns | 1988ns | 7756ns | +17.56% | 8.292 |
| warm-container-minimum | 1065ns | 928ns | 1237ns | -68.33% | 30.779 |
| warm-container-native | 923ns | 214ns | 1316ns | -72.55% | 35.507 |
| warm-container-plusone | 3633ns | 1464ns | 6005ns | +8.08% | 9.020 |

## Performance model

- Peak throughput: **153.462 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.579 | 11.5% |
| warm-container-kernel | 13.484 | 8.8% |
| warm-container-lanes-deferred | 13.483 | 8.8% |
| warm-container-minimum | 31.747 | 20.7% |
| warm-container-native | 29.962 | 19.5% |
| warm-container-plusone | 8.066 | 5.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 3492ns | 3492ns | base |
| warm-container-kernel | 3684ns | 3684ns | +5.50% |
| warm-container-lanes-deferred | 4063ns | 4063ns | +16.35% |
| warm-container-minimum | 1338ns | 1338ns | -61.69% |
| warm-container-native | 1161ns | 1161ns | -66.75% |
| warm-container-plusone | 3771ns | 3771ns | +7.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 1864ns | base | --- | [1785, 4567] | --- | --- | --- | --- |
| warm-container-kernel | 2430ns | +527.1ns (+28.3%) | [+203, +650]ns | [2000, 2436] | YES | 0.0028 | 0.0022 | 0 |
| warm-container-lanes-deferred | 2430ns | +528.5ns (+28.4%) | [+332, +659]ns | [2010, 5262] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-minimum | 1032ns | -767.9ns (-41.2%) | [-3638, -702]ns | [1027, 1130] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 1094ns | -1254.3ns (-67.3%) | [-3239, -761]ns | [1032, 1161] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 4062ns | no significant difference | [-103, +17]ns | [2441, 4477] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4284ns | -43.1% | -22.0% | -78.3% | -72.0% | -5.2% |
| 1 | 2 | 1466ns | +35.7% | +65.5% | -29.6% | -85.3% | +60.1% |
| 1 | 3 | 1786ns | +11.4% | +13.5% | -35.1% | -41.6% | +141.5% |
| 1 | 4 | 3874ns | -37.1% | -37.3% | -70.0% | -70.5% | -35.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.259 | moderate- |
| warm-container-kernel | -0.250 | moderate- |
| warm-container-lanes-deferred | 0.035 | ok |
| warm-container-minimum | 0.269 | moderate+ |
| warm-container-native | -0.422 | moderate- |
| warm-container-plusone | -0.792 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-kernel**: won 2/4, lost 2/4
- **warm-container-lanes-deferred**: won 2/4, lost 2/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 44.7ns | 3361.4ns | 1.3% |  |
| warm-container-kernel | 34.4ns | 3579.5ns | 1.0% |  |
| warm-container-lanes-deferred | 38.9ns | 3951.6ns | 1.0% |  |
| warm-container-minimum | 96.9ns | 1064.6ns | 9.1% | HIGH |
| warm-container-native | 82.5ns | 922.9ns | 8.9% | HIGH |
| warm-container-plusone | 48.0ns | 3632.9ns | 1.3% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 1459.7-6256.3 ns)
   1459.7 |#######################
   1699.5 |########################################
   1939.3 |
   2179.2 |
   2419.0 |###
   2658.8 |
   2898.7 |
   3138.5 |
   3378.3 |
   3618.2 |
   3858.0 |###
   4097.8 |######
   4337.6 |######
   4577.5 |
   4817.3 |######
   5057.1 |######
   5297.0 |###
   5536.8 |###
   5776.6 |######
   6016.5 |###
  (2 below, 4 above range)

warm-container-kernel (n=40, range 1988.1-7858.4 ns)
   1988.1 |#################################
   2281.6 |########################################
   2575.2 |###
   2868.7 |
   3162.2 |###
   3455.7 |
   3749.2 |
   4042.7 |
   4336.2 |
   4629.7 |
   4923.3 |
   5216.8 |
   5510.3 |
   5803.8 |
   6097.3 |###
   6390.8 |
   6684.3 |###
   6977.8 |###
   7271.4 |######
   7564.9 |###
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 1988.0-7755.6 ns)
   1988.0 |########################################
   2276.4 |########################
   2564.8 |###
   2853.2 |
   3141.6 |
   3429.9 |
   3718.3 |
   4006.7 |
   4295.1 |
   4583.4 |###
   4871.8 |
   5160.2 |
   5448.6 |
   5736.9 |###
   6025.3 |############
   6313.7 |
   6602.1 |######
   6890.4 |###
   7178.8 |######
   7467.2 |
  (3 below, 4 above range)

warm-container-minimum (n=40, range 928.1-1236.9 ns)
    928.1 |#######################
    943.5 |
    959.0 |
    974.4 |
    989.8 |###
   1005.3 |
   1020.7 |########################################
   1036.2 |
   1051.6 |###
   1067.0 |
   1082.5 |
   1097.9 |
   1113.3 |###
   1128.8 |###
   1144.2 |######
   1159.7 |####################
   1175.1 |###
   1190.5 |###
   1206.0 |
   1221.4 |
  (4 below, 3 above range)

warm-container-native (n=40, range 213.5-1316.1 ns)
    213.5 |#####################
    268.7 |
    323.8 |
    378.9 |
    434.0 |
    489.2 |
    544.3 |
    599.4 |
    654.6 |
    709.7 |
    764.8 |
    820.0 |
    875.1 |
    930.2 |
    985.4 |#############################
   1040.5 |#######
   1095.6 |#######
   1150.8 |########################################
   1205.9 |#######
   1261.0 |
  (4 below, 5 above range)

warm-container-plusone (n=40, range 1464.5-6005.2 ns)
   1464.5 |########################################
   1691.5 |########################################
   1918.5 |
   2145.6 |
   2372.6 |
   2599.7 |
   2826.7 |
   3053.7 |########################
   3280.8 |
   3507.8 |########
   3734.8 |########
   3961.9 |########################
   4188.9 |################################
   4415.9 |################
   4643.0 |################
   4870.0 |################
   5097.1 |
   5324.1 |########
   5551.1 |################
   5778.2 |################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=57.8% (high variance, measurements may be unstable)
- **warm-container-headroom**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-container-kernel**: CV=65.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-container-lanes-deferred**: CV=60.9% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-native**: CV=45.6% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 6.2x (possible bimodal distribution)
- **warm-container-plusone**: CV=47.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: worst_20/best_20 = 4.1x (possible bimodal distribution)
