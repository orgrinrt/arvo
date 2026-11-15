# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 54% (significant)

warm-container-native is -127.17 us (54%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.3x slower than the field

warm-container-plusone (256.94 us) is 2.3x the fastest (109.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (109.37 us) and warm-container-native (110.33 us) differ by 0.88%, inside the noise, even though the wider field spreads 134.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows alternating (throttle bounce) (autocorr -0.73)

warm-container-native's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (112% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 112% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+1% speed for 2.3x steadier)

warm-container-minimum is fastest (109.37 us, CV 3.3%); warm-container-native gives up 0.9% median for 2.3x lower variance (CV 1.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 1.8x its best-20%

warm-container-plusone's best 20% of batches run at 205.23 us but its worst 20% at 373.93 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 109372.6 ns median (-53.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.35x (fastest 109372.6 ns, slowest 256941.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 232967ns | 234224ns | 180944ns | 233949ns | 282045ns | base |
| warm-container-minimum | 108874ns | 109684ns | 102503ns | 109803ns | 112461ns | -53.27% |
| warm-container-native | 109748ns | 110476ns | 107731ns | 110078ns | 110777ns | -52.89% |
| warm-container-plusone | 273693ns | 257238ns | 205402ns | 262851ns | 374509ns | +17.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 232718ns | 180788ns | 281753ns | base | 18.023 |
| warm-container-minimum | 108689ns | 102343ns | 112251ns | -53.30% | 38.590 |
| warm-container-native | 109593ns | 107563ns | 110611ns | -52.91% | 38.272 |
| warm-container-plusone | 273351ns | 205227ns | 373932ns | +17.46% | 15.344 |

## Performance model

- Peak throughput: **40.983 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 17.929 | 43.7% |
| warm-container-minimum | 38.349 | 93.6% |
| warm-container-native | 38.016 | 92.8% |
| warm-container-plusone | 16.324 | 39.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 232967ns | 232967ns | base |
| warm-container-minimum | 108874ns | 108874ns | -53.27% |
| warm-container-native | 109748ns | 109748ns | -52.89% |
| warm-container-plusone | 273693ns | 273693ns | +17.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 233935ns | base | --- | [205315, 263980] | --- | --- | --- | --- |
| warm-container-minimum | 109373ns | -122863.8ns (-52.5%) | [-154655, -96725]ns | [108696, 110570] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 110331ns | -127169.1ns (-54.4%) | [-154812, -96145]ns | [109244, 110528] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 256942ns | no significant difference | [-186, +77836]ns | [250179, 261832] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 180783ns | -43.7% | -38.8% | +44.4% |
| 1 | 2 | 205348ns | -47.1% | -46.8% | -0.1% |
| 1 | 3 | 276794ns | -60.0% | -60.1% | -9.4% |
| 1 | 4 | 263298ns | -57.9% | -58.6% | +29.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.250 | moderate+ |
| warm-container-minimum | 0.104 | ok |
| warm-container-native | -0.730 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.004 | ok |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 32.4ns | 232717.9ns | 0.0% |  |
| warm-container-minimum | 20.8ns | 108688.8ns | 0.0% |  |
| warm-container-native | 20.8ns | 109593.3ns | 0.0% |  |
| warm-container-plusone | 44.9ns | 273350.9ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 180788.1-281753.4 ns)
  180788.1 |######################
  185836.4 |
  190884.6 |
  195932.9 |
  200981.2 |########################################
  206029.4 |####
  211077.7 |
  216126.0 |
  221174.2 |
  226222.5 |
  231270.8 |
  236319.0 |
  241367.3 |
  246415.5 |
  251463.8 |
  256512.1 |####
  261560.3 |###############################
  266608.6 |########
  271656.9 |######################
  276705.1 |#################
  (5 below, 1 above range)

warm-container-minimum (n=40, range 102343.2-112251.2 ns)
  102343.2 |
  102838.6 |
  103334.0 |
  103829.4 |
  104324.8 |
  104820.2 |
  105315.6 |
  105811.0 |
  106306.4 |
  106801.8 |
  107297.2 |
  107792.6 |#######
  108288.0 |#############################
  108783.4 |#######
  109278.8 |#######
  109774.2 |###
  110269.6 |########################################
  110765.0 |##########
  111260.4 |
  111755.8 |
  (7 below, 4 above range)

warm-container-native (n=40, range 107563.3-110611.1 ns)
  107563.3 |
  107715.7 |
  107868.1 |
  108020.5 |
  108172.9 |
  108325.2 |
  108477.6 |
  108630.0 |
  108782.4 |######
  108934.8 |##########
  109087.2 |#######################
  109239.6 |######
  109392.0 |##########
  109544.4 |
  109696.8 |
  109849.1 |
  110001.5 |
  110153.9 |###
  110306.3 |#############
  110458.7 |########################################
  (2 below, 4 above range)

warm-container-plusone (n=40, range 205227.0-373932.3 ns)
  205227.0 |######################
  213662.3 |
  222097.6 |
  230532.8 |
  238968.1 |####
  247403.4 |########################################
  255838.6 |###############################
  264273.9 |
  272709.2 |####
  281144.4 |
  289579.7 |
  298014.9 |########
  306450.2 |
  314885.5 |
  323320.7 |
  331756.0 |########
  340191.3 |######################
  348626.5 |
  357061.8 |
  365497.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-container-plusone**: CV=23.1% (high variance, measurements may be unstable)
