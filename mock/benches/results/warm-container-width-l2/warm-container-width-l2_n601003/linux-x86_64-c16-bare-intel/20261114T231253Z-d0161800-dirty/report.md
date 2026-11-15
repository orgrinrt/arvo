# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (716.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-plusone at 224.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-plusone dominates: 29% faster than the next best (warm-container-minimum)

warm-container-plusone (224.36 us) leads warm-container-minimum (289.89 us) by 29%, a clear separation rather than a photo finish. CV 28.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 60% (significant)

warm-container-minimum is -430.91 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.2x slower than the field

warm-container-headroom (716.08 us) is 3.2x the fastest (224.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-plusone is fastest but the noisiest (CV 28.8%)

warm-container-plusone wins on median (224.36 us) yet has the highest variance (CV 28.8%), while warm-container-headroom is the steadiest (CV 9.1%, 716.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.59)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-plusone, warm-container-minimum, warm-container-native} vs {warm-container-headroom} (106% apart)

The field splits into a fast tier {warm-container-plusone, warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom} with a 106% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest warm-container-plusone (224.36 us) to slowest warm-container-headroom (716.08 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 1.9x its best-20%

warm-container-minimum's best 20% of batches run at 195.89 us but its worst 20% at 372.20 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-plusone** at 224363.2 ns median (-68.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.19x (fastest 224363.2 ns, slowest 716080.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 691971ns | 716933ns | 618283ns | 688321ns | 776609ns | base |
| warm-container-minimum | 287328ns | 290232ns | 196041ns | 289278ns | 372768ns | -58.48% |
| warm-container-native | 363593ns | 348192ns | 291453ns | 355069ns | 461305ns | -47.46% |
| warm-container-plusone | 249213ns | 224561ns | 194278ns | 231364ns | 357695ns | -63.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 691187ns | 617547ns | 776082ns | base | 6.068 |
| warm-container-minimum | 286993ns | 195888ns | 372196ns | -58.48% | 14.615 |
| warm-container-native | 363192ns | 291165ns | 460773ns | -47.45% | 11.548 |
| warm-container-plusone | 248967ns | 194120ns | 357267ns | -63.98% | 16.847 |

## Performance model

- Peak throughput: **21.607 Gops/s** (warm-container-plusone; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.857 | 27.1% |
| warm-container-minimum | 14.469 | 67.0% |
| warm-container-native | 12.062 | 55.8% |
| warm-container-plusone | 18.694 | 86.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 691971ns | 691971ns | base |
| warm-container-minimum | 287328ns | 287328ns | -58.48% |
| warm-container-native | 363593ns | 363593ns | -47.46% |
| warm-container-plusone | 249213ns | 249213ns | -63.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 716080ns | base | --- | [622250, 719424] | --- | --- | --- | --- |
| warm-container-minimum | 289886ns | -430910.1ns (-60.2%) | [-480639, -362672]ns | [260400, 313127] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 347718ns | -316886.2ns (-44.3%) | [-414854, -272286]ns | [346061, 381583] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 224363ns | -428886.3ns (-59.9%) | [-453070, -425848]ns | [198743, 251543] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 621816ns | -55.9% | -43.9% | -68.8% |
| 1 | 2 | 769221ns | -62.1% | -55.0% | -55.4% |
| 1 | 3 | 717695ns | -72.8% | -58.1% | -65.0% |
| 1 | 4 | 617777ns | -44.2% | -38.1% | -67.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.264 | moderate- |
| warm-container-minimum | -0.593 | HIGH- (thermal bounce) |
| warm-container-native | -0.513 | HIGH- (thermal bounce) |
| warm-container-plusone | -0.338 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 85.1ns | 691187.4ns | 0.0% |  |
| warm-container-minimum | 24.7ns | 286992.6ns | 0.0% |  |
| warm-container-native | 83.3ns | 363191.7ns | 0.0% |  |
| warm-container-plusone | 32.3ns | 248967.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 617547.1-776082.1 ns)
  617547.1 |########################################
  625473.8 |
  633400.6 |
  641327.3 |
  649254.1 |
  657180.8 |
  665107.6 |######
  673034.3 |
  680961.1 |###
  688887.8 |
  696814.6 |
  704741.3 |
  712668.1 |##########################
  720594.9 |###
  728521.6 |
  736448.4 |
  744375.1 |
  752301.9 |###
  760228.6 |################
  768155.4 |################
  (4 below, 1 above range)

warm-container-minimum (n=40, range 195887.6-372195.8 ns)
  195887.6 |########
  204703.0 |
  213518.4 |########
  222333.8 |
  231149.2 |
  239964.6 |
  248780.0 |####
  257595.5 |################
  266410.9 |
  275226.3 |####
  284041.7 |########################################
  292857.1 |
  301672.5 |
  310487.9 |
  319303.3 |
  328118.7 |################
  336934.2 |############################
  345749.6 |
  354565.0 |####
  363380.4 |
  (6 below, 2 above range)

warm-container-native (n=40, range 291165.1-460773.0 ns)
  291165.1 |#####
  299645.5 |##############
  308125.9 |
  316606.3 |
  325086.7 |
  333567.1 |
  342047.5 |########################################
  350527.9 |
  359008.3 |
  367488.7 |##
  375969.1 |######################
  384449.5 |
  392929.9 |
  401410.2 |#####
  409890.6 |
  418371.0 |
  426851.4 |##
  435331.8 |##
  443812.2 |
  452292.6 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 194120.2-357267.1 ns)
  194120.2 |########################################
  202277.6 |
  210434.9 |
  218592.3 |
  226749.6 |
  234906.9 |
  243064.3 |#########
  251221.6 |##############
  259379.0 |
  267536.3 |
  275693.7 |
  283851.0 |
  292008.3 |
  300165.7 |
  308323.0 |
  316480.4 |
  324637.7 |#########
  332795.1 |
  340952.4 |#######
  349109.8 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=22.7% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=25.9% (high variance, measurements may be unstable)
