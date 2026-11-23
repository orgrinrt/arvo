# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 86% (significant)

warm-container-native is -160.89 us (86%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 7.3x slower than the field

warm-container-plusone (187.30 us) is 7.3x the fastest (25.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 85.3%)

warm-container-native wins on median (25.77 us) yet has the highest variance (CV 85.3%), while warm-container-plusone is the steadiest (CV 0.4%, 187.30 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (25.77 us) and warm-container-minimum (25.78 us) differ by 0.01%, inside the noise, even though the wider field spreads 626.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (624% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 624% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest warm-container-native (25.77 us) to slowest warm-container-plusone (187.30 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-native is inconsistent: worst-20% is 1.7x its best-20%

warm-container-native's best 20% of batches run at 25.54 us but its worst 20% at 43.48 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 25774.2 ns median (-86.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.27x (fastest 25774.2 ns, slowest 187301.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112751190 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 186409ns | 186751ns | 185286ns | 186512ns | 187223ns | base |
| warm-container-minimum | 25825ns | 25847ns | 25604ns | 25846ns | 25986ns | -86.15% |
| warm-container-native | 29357ns | 25856ns | 25613ns | 25855ns | 43604ns | -84.25% |
| warm-container-plusone | 187251ns | 187454ns | 186029ns | 187369ns | 188116ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 186246ns | 185117ns | 187069ns | base | 22.520 |
| warm-container-minimum | 25752ns | 25531ns | 25917ns | -86.17% | 162.873 |
| warm-container-native | 29274ns | 25545ns | 43482ns | -84.28% | 143.276 |
| warm-container-plusone | 187087ns | 185868ns | 187946ns | +0.45% | 22.419 |

## Performance model

- Peak throughput: **164.282 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 22.479 | 13.7% |
| warm-container-minimum | 162.725 | 99.1% |
| warm-container-native | 162.733 | 99.1% |
| warm-container-plusone | 22.393 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 186409ns | 186409ns | base |
| warm-container-minimum | 25825ns | 25825ns | -86.15% |
| warm-container-native | 29357ns | 29357ns | -84.25% |
| warm-container-plusone | 187251ns | 187251ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 186588ns | base | --- | [185710, 186793] | --- | --- | --- | --- |
| warm-container-minimum | 25775ns | -160763.0ns (-86.2%) | [-160956, -160202]ns | [25760, 25797] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 25774ns | -160892.4ns (-86.2%) | [-161105, -159848]ns | [25758, 25803] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 187301ns | +1021.0ns (+0.5%) | [+581, +1595]ns | [187092, 187345] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 185784ns | -86.1% | -86.1% | +0.8% |
| 1 | 2 | 185375ns | -86.2% | -86.1% | +1.0% |
| 1 | 3 | 187051ns | -86.2% | -86.2% | -0.5% |
| 1 | 4 | 186618ns | -86.2% | -86.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.002 | ok |
| warm-container-minimum | -0.475 | moderate- |
| warm-container-native | -0.090 | ok |
| warm-container-plusone | -0.469 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 19.6ns | 186246.5ns | 0.0% |  |
| warm-container-minimum | 19.4ns | 25752.0ns | 0.1% |  |
| warm-container-native | 22.0ns | 29274.3ns | 0.1% |  |
| warm-container-plusone | 20.2ns | 187087.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 185117.3-187069.0 ns)
  185117.3 |##############################
  185214.9 |##########
  185312.4 |####################
  185410.0 |####################
  185507.6 |####################
  185605.2 |##########
  185702.8 |##########
  185800.4 |
  185898.0 |
  185995.6 |##########
  186093.1 |
  186190.7 |
  186288.3 |
  186385.9 |
  186483.5 |########################################
  186581.1 |##############################
  186678.7 |##############################
  186776.3 |##############################
  186873.8 |########################################
  186971.4 |##############################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 25531.0-25916.8 ns)
  25531.0 |#################
  25550.3 |###########
  25569.6 |
  25588.9 |#####
  25608.2 |
  25627.5 |
  25646.8 |
  25666.0 |
  25685.3 |
  25704.6 |
  25723.9 |###########
  25743.2 |############################
  25762.5 |########################################
  25781.8 |#################
  25801.0 |############################
  25820.3 |#####
  25839.6 |######################
  25858.9 |#####
  25878.2 |
  25897.5 |###########
  (3 below, 1 above range)

warm-container-native (n=40, range 25544.9-43481.9 ns)
  25544.9 |########################################
  26441.8 |
  27338.6 |
  28235.5 |
  29132.3 |
  30029.2 |
  30926.0 |
  31822.9 |
  32719.7 |
  33616.5 |
  34513.4 |
  35410.2 |
  36307.1 |
  37203.9 |
  38100.8 |
  38997.6 |
  39894.5 |
  40791.3 |
  41688.2 |
  42585.0 |
  (6 below, 1 above range)

warm-container-plusone (n=40, range 185868.4-187945.6 ns)
  185868.4 |######
  185972.3 |
  186076.1 |#############
  186180.0 |#############
  186283.9 |
  186387.7 |
  186491.6 |
  186595.5 |######
  186699.3 |
  186803.2 |######
  186907.0 |######
  187010.9 |#################################
  187114.8 |#############
  187218.6 |########################################
  187322.5 |########################################
  187426.3 |#############
  187530.2 |#############
  187634.1 |######
  187737.9 |######
  187841.8 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-container-native**: CV=75.1% (high variance, measurements may be unstable)
