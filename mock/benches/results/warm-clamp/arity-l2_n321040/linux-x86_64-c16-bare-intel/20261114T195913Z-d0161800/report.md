# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 4.0x slower than the field

warm-clamp-minimum (412.27 us) is 4.0x the fastest (101.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (101.93 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum} (159% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 159% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest warm-clamp-acc64 (101.93 us) to slowest warm-clamp-minimum (412.27 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-clamp-acc64 vs stability leader warm-clamp-accfit-dyn (+5% speed for 9.5x steadier)

warm-clamp-acc64 is fastest (101.93 us, CV 4.9%); warm-clamp-accfit-dyn gives up 4.7% median for 9.5x lower variance (CV 0.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-head is inconsistent: worst-20% is 1.5x its best-20%

warm-clamp-head's best 20% of batches run at 128.68 us but its worst 20% at 194.93 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 101926.2 ns median
- 4 variants significantly slower than baseline
- Spread: 4.04x (fastest 101926.2 ns, slowest 412266.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 102107ns | 102130ns | 96838ns | 102030ns | 107607ns | base |
| warm-clamp-accfit | 102897ns | 106366ns | 96789ns | 103361ns | 107614ns | +0.77% |
| warm-clamp-accfit-dyn | 106852ns | 106885ns | 106201ns | 106855ns | 107491ns | +4.65% |
| warm-clamp-head | 160118ns | 148303ns | 128836ns | 158850ns | 195204ns | +56.81% |
| warm-clamp-min-lanes | 376509ns | 382975ns | 353663ns | 380827ns | 386403ns | +268.74% |
| warm-clamp-minimum | 406446ns | 412430ns | 395519ns | 406772ns | 416392ns | +298.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 101918ns | 96682ns | 107441ns | base | 10.288 |
| warm-clamp-accfit | 102745ns | 96642ns | 107456ns | +0.81% | 10.206 |
| warm-clamp-accfit-dyn | 106695ns | 106052ns | 107325ns | +4.69% | 9.828 |
| warm-clamp-head | 159853ns | 128679ns | 194928ns | +56.85% | 6.560 |
| warm-clamp-min-lanes | 376340ns | 353495ns | 386220ns | +269.26% | 2.786 |
| warm-clamp-minimum | 406267ns | 395350ns | 416215ns | +298.62% | 2.581 |

## Performance model

- Peak throughput: **10.850 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 10.288 | 94.8% |
| warm-clamp-accfit | 9.871 | 91.0% |
| warm-clamp-accfit-dyn | 9.824 | 90.5% |
| warm-clamp-head | 7.084 | 65.3% |
| warm-clamp-min-lanes | 2.739 | 25.2% |
| warm-clamp-minimum | 2.543 | 23.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 102107ns | 102107ns | base |
| warm-clamp-accfit | 102897ns | 102897ns | +0.77% |
| warm-clamp-accfit-dyn | 106852ns | 106852ns | +4.65% |
| warm-clamp-head | 160118ns | 160118ns | +56.81% |
| warm-clamp-min-lanes | 376509ns | 376509ns | +268.74% |
| warm-clamp-minimum | 406446ns | 406446ns | +298.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 101926ns | base | --- | [97203, 106364] | --- | --- | --- | --- |
| warm-clamp-accfit | 106227ns | no significant difference | [-428, +624]ns | [97011, 106635] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-accfit-dyn | 106736ns | +4873.1ns (+4.8%) | [+972, +9149]ns | [106191, 107187] | YES | 0.0028 | 0.0022 | 0 |
| warm-clamp-head | 148014ns | +46324.1ns (+45.4%) | [+33964, +83148]ns | [130693, 190571] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 382814ns | +281749.7ns (+276.4%) | [+277457, +285289]ns | [381511, 384054] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 412267ns | +305641.8ns (+299.9%) | [+301339, +308249]ns | [398013, 414334] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 107434ns | -1.0% | -0.2% | +77.5% | +229.1% | +268.2% |
| 1 | 2 | 106267ns | +1.1% | +0.9% | +82.8% | +261.1% | +289.9% |
| 1 | 3 | 97268ns | -0.5% | +9.2% | +32.3% | +296.8% | +327.3% |
| 1 | 4 | 96689ns | +0.4% | +9.7% | +35.2% | +294.9% | +311.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.286 | moderate+ |
| warm-clamp-accfit | 0.210 | moderate+ |
| warm-clamp-accfit-dyn | 0.255 | moderate+ |
| warm-clamp-head | 0.226 | moderate+ |
| warm-clamp-min-lanes | -0.063 | ok |
| warm-clamp-minimum | -0.244 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.5ns | 101917.8ns | 0.0% |  |
| warm-clamp-accfit | 33.0ns | 102744.7ns | 0.0% |  |
| warm-clamp-accfit-dyn | 30.3ns | 106694.7ns | 0.0% |  |
| warm-clamp-head | 55.8ns | 159853.1ns | 0.0% |  |
| warm-clamp-min-lanes | 20.4ns | 376339.6ns | 0.0% |  |
| warm-clamp-minimum | 35.8ns | 406267.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 96682.1-107440.5 ns)
  96682.1 |########################################
  97220.0 |######################
  97757.9 |####
  98295.8 |
  98833.7 |
  99371.7 |
  99909.6 |
  100447.5 |
  100985.4 |
  101523.4 |
  102061.3 |
  102599.2 |
  103137.1 |
  103675.1 |
  104213.0 |
  104750.9 |
  105288.8 |
  105826.8 |##########################
  106364.7 |#################
  106902.6 |######################
  (5 below, 5 above range)

warm-clamp-accfit (n=40, range 96641.7-107455.8 ns)
  96641.7 |########################################
  97182.4 |###
  97723.1 |
  98263.8 |
  98804.5 |
  99345.2 |
  99885.9 |
  100426.6 |
  100967.3 |
  101508.0 |###
  102048.7 |
  102589.4 |
  103130.1 |
  103670.8 |
  104211.5 |
  104752.2 |
  105292.9 |
  105833.6 |#########################
  106374.3 |##########
  106915.0 |#############################
  (4 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 106052.3-107324.5 ns)
  106052.3 |#################################
  106115.9 |#################################
  106179.5 |#################################
  106243.1 |######
  106306.7 |
  106370.4 |######
  106434.0 |
  106497.6 |######
  106561.2 |
  106624.8 |
  106688.4 |
  106752.0 |
  106815.6 |
  106879.3 |
  106942.9 |######
  107006.5 |
  107070.1 |#############
  107133.7 |########################################
  107197.3 |####################
  107260.9 |#################################
  (2 below, 3 above range)

warm-clamp-head (n=40, range 128678.8-194927.7 ns)
  128678.8 |########################################
  131991.2 |##
  135303.7 |
  138616.1 |
  141928.6 |
  145241.0 |
  148553.4 |
  151865.9 |
  155178.3 |
  158490.8 |
  161803.2 |#####
  165115.7 |
  168428.1 |
  171740.6 |
  175053.0 |
  178365.5 |
  181677.9 |
  184990.3 |
  188302.8 |##################
  191615.2 |#####################
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 353495.1-386219.8 ns)
  353495.1 |##############################
  355131.3 |
  356767.6 |
  358403.8 |
  360040.1 |
  361676.3 |
  363312.5 |
  364948.8 |
  366585.0 |
  368221.2 |
  369857.5 |
  371493.7 |
  373129.9 |
  374766.2 |
  376402.4 |
  378038.6 |
  379674.9 |####################
  381311.1 |##############################
  382947.3 |########################################
  384583.6 |########################################
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 395350.1-416215.0 ns)
  395350.1 |###############
  396393.4 |#########################
  397436.6 |#########################
  398479.9 |
  399523.1 |
  400566.3 |#####
  401609.6 |
  402652.8 |
  403696.1 |
  404739.3 |
  405782.6 |
  406825.8 |
  407869.0 |
  408912.3 |
  409955.5 |
  410998.8 |#####
  412042.0 |#####
  413085.3 |###############
  414128.5 |########################################
  415171.7 |#########################
  (5 below, 3 above range)

```
