# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native dominates: 25% faster than the next best (warm-container-minimum)

warm-container-native (208.29 us) leads warm-container-minimum (261.34 us) by 25%, a clear separation rather than a photo finish. CV 17.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 71% (significant)

warm-container-native is -494.72 us (71%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3.6x slower than the field

warm-container-plusone (757.62 us) is 3.6x the fastest (208.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 17.7%)

warm-container-native wins on median (208.29 us) yet has the highest variance (CV 17.7%), while warm-container-headroom is the steadiest (CV 9.8%, 696.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows alternating (throttle bounce) (autocorr -0.75)

warm-container-plusone's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (167% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 167% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest warm-container-native (208.29 us) to slowest warm-container-plusone (757.62 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 1.6x its best-20%

warm-container-minimum's best 20% of batches run at 194.44 us but its worst 20% at 303.25 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 208291.7 ns median (-70.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.64x (fastest 208291.7 ns, slowest 757620.3 ns)

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
| warm-container-headroom | 729938ns | 697794ns | 663056ns | 719445ns | 828300ns | base |
| warm-container-minimum | 253392ns | 261592ns | 194715ns | 256204ns | 303632ns | -65.29% |
| warm-container-native | 227463ns | 208515ns | 193919ns | 221174ns | 279875ns | -68.84% |
| warm-container-plusone | 757714ns | 758336ns | 656443ns | 746206ns | 893510ns | +3.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 729087ns | 662269ns | 827419ns | base | 5.753 |
| warm-container-minimum | 253057ns | 194437ns | 303247ns | -65.29% | 16.575 |
| warm-container-native | 227174ns | 193746ns | 279413ns | -68.84% | 18.463 |
| warm-container-plusone | 756937ns | 655481ns | 892866ns | +3.82% | 5.541 |

## Performance model

- Peak throughput: **21.648 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.019 | 27.8% |
| warm-container-minimum | 16.049 | 74.1% |
| warm-container-native | 20.137 | 93.0% |
| warm-container-plusone | 5.536 | 25.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 729938ns | 729938ns | base |
| warm-container-minimum | 253392ns | 253392ns | -65.29% |
| warm-container-native | 227463ns | 227463ns | -68.84% |
| warm-container-plusone | 757714ns | 757714ns | +3.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 696902ns | base | --- | [675038, 785629] | --- | --- | --- | --- |
| warm-container-minimum | 261342ns | -472339.0ns (-67.8%) | [-502931, -421551]ns | [238245, 277171] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 208292ns | -494718.2ns (-71.0%) | [-513627, -474339]ns | [193958, 244907] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 757620ns | no significant difference | [-12921, +90815]ns | [674312, 783255] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 670317ns | -60.2% | -71.1% | +14.9% |
| 1 | 2 | 786251ns | -69.6% | -65.6% | -16.8% |
| 1 | 3 | 684226ns | -59.2% | -71.7% | +29.6% |
| 1 | 4 | 673161ns | -71.1% | -65.6% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.406 | moderate- |
| warm-container-minimum | -0.486 | moderate- |
| warm-container-native | -0.746 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.747 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 55.3ns | 729086.6ns | 0.0% |  |
| warm-container-minimum | 38.8ns | 253056.9ns | 0.0% |  |
| warm-container-native | 41.5ns | 227174.1ns | 0.0% |  |
| warm-container-plusone | 73.0ns | 756937.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 662268.6-827419.1 ns)
  662268.6 |########################################
  670526.1 |########
  678783.6 |############
  687041.1 |####
  695298.7 |############
  703556.2 |
  711813.7 |####
  720071.2 |####
  728328.8 |
  736586.3 |
  744843.8 |
  753101.3 |
  761358.9 |
  769616.4 |
  777873.9 |################
  786131.4 |########################
  794389.0 |
  802646.5 |####
  810904.0 |
  819161.5 |####
  (3 below, 4 above range)

warm-container-minimum (n=40, range 194437.5-303247.2 ns)
  194437.5 |#########################
  199877.9 |
  205318.4 |
  210758.9 |
  216199.4 |
  221639.9 |
  227080.4 |
  232520.9 |####################
  237961.4 |###############
  243401.9 |
  248842.3 |
  254282.8 |#####
  259723.3 |####################
  265163.8 |
  270604.3 |##########
  276044.8 |########################################
  281485.3 |###############
  286925.8 |#####
  292366.3 |
  297806.7 |
  (5 below, 4 above range)

warm-container-native (n=40, range 193746.4-279413.5 ns)
  193746.4 |########################################
  198029.8 |
  202313.1 |
  206596.5 |
  210879.8 |
  215163.2 |
  219446.5 |##
  223729.9 |#####
  228013.2 |#######
  232296.6 |
  236580.0 |
  240863.3 |
  245146.7 |
  249430.0 |
  253713.4 |
  257996.7 |##
  262280.1 |
  266563.4 |###############
  270846.8 |#####
  275130.1 |#####
  (4 below, 3 above range)

warm-container-plusone (n=40, range 655480.5-892865.7 ns)
  655480.5 |####
  667349.8 |########################################
  679219.0 |
  691088.3 |
  702957.6 |
  714826.8 |####
  726696.1 |
  738565.3 |########
  750434.6 |#############
  762303.8 |####
  774173.1 |##########################
  786042.4 |########
  797911.6 |
  809780.9 |
  821650.1 |
  833519.4 |########
  845388.6 |
  857257.9 |
  869127.1 |
  880996.4 |###############################
  (5 below, 1 above range)

```
