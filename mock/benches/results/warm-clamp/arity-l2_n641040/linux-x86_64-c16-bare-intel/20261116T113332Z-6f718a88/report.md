# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (474.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit at 214.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit dominates: 21% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-accfit (214.85 us) leads warm-clamp-accfit-dyn (259.45 us) by 21%, a clear separation rather than a photo finish. CV 31.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 57% (significant)

warm-clamp-accfit is -270.37 us (57%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.2x slower than the field

warm-clamp-acc64 (474.22 us) is 2.2x the fastest (214.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit is fastest but the noisiest (CV 31.0%)

warm-clamp-accfit wins on median (214.85 us) yet has the highest variance (CV 31.0%), while warm-clamp-minimum is the steadiest (CV 6.0%, 438.99 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.53)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-accfit-dyn} vs {warm-clamp-min-lanes, warm-clamp-head, warm-clamp-minimum, warm-clamp-acc64} (53% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-min-lanes, warm-clamp-head, warm-clamp-minimum, warm-clamp-acc64} with a 53% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 2.1x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 154.05 us but its worst 20% at 330.76 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 214845.5 ns median (-54.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.21x (fastest 214845.5 ns, slowest 474217.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112826587 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 461880ns | 474593ns | 400302ns | 468141ns | 504674ns | base |
| warm-clamp-accfit | 236357ns | 215092ns | 196068ns | 218110ns | 331387ns | -48.83% |
| warm-clamp-accfit-dyn | 254534ns | 259735ns | 154205ns | 262448ns | 331120ns | -44.89% |
| warm-clamp-head | 506457ns | 438801ns | 412842ns | 479758ns | 680170ns | +9.65% |
| warm-clamp-min-lanes | 405289ns | 396090ns | 359230ns | 396967ns | 476313ns | -12.25% |
| warm-clamp-minimum | 439938ns | 439258ns | 405547ns | 440991ns | 471169ns | -4.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 461549ns | 400119ns | 504214ns | base | 2.272 |
| warm-clamp-accfit | 236086ns | 195876ns | 330948ns | -48.85% | 4.442 |
| warm-clamp-accfit-dyn | 254242ns | 154049ns | 330755ns | -44.92% | 4.124 |
| warm-clamp-head | 505624ns | 412098ns | 679481ns | +9.55% | 2.074 |
| warm-clamp-min-lanes | 405089ns | 359046ns | 476101ns | -12.23% | 2.589 |
| warm-clamp-minimum | 439673ns | 405370ns | 470813ns | -4.74% | 2.385 |

## Performance model

- Peak throughput: **6.807 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.211 | 32.5% |
| warm-clamp-accfit | 4.881 | 71.7% |
| warm-clamp-accfit-dyn | 4.042 | 59.4% |
| warm-clamp-head | 2.397 | 35.2% |
| warm-clamp-min-lanes | 2.648 | 38.9% |
| warm-clamp-minimum | 2.389 | 35.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 461880ns | 461880ns | base |
| warm-clamp-accfit | 236357ns | 236357ns | -48.83% |
| warm-clamp-accfit-dyn | 254534ns | 254534ns | -44.89% |
| warm-clamp-head | 506457ns | 506457ns | +9.65% |
| warm-clamp-min-lanes | 405289ns | 405289ns | -12.25% |
| warm-clamp-minimum | 439938ns | 439938ns | -4.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 474217ns | base | --- | [470372, 475463] | --- | --- | --- | --- |
| warm-clamp-accfit | 214846ns | -270374.7ns (-57.0%) | [-278422, -244435]ns | [198982, 230443] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 259451ns | -209723.2ns (-44.2%) | [-217481, -204098]ns | [256971, 269276] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 437450ns | no significant difference | [-29269, +71670]ns | [425531, 473985] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-min-lanes | 395914ns | -67769.7ns (-14.3%) | [-80877, -7586]ns | [394623, 397624] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-minimum | 438990ns | -45778.3ns (-9.7%) | [-52765, -19213]ns | [421817, 458214] | YES | 0.0028 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 471057ns | -57.8% | -41.0% | -7.0% | -23.7% | -10.8% |
| 1 | 2 | 476728ns | -58.9% | -44.1% | -13.9% | +1.8% | -4.0% |
| 1 | 3 | 400901ns | -32.6% | -61.8% | +5.8% | -1.0% | +16.6% |
| 1 | 4 | 474760ns | -51.5% | -43.3% | +40.9% | -16.8% | -11.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.462 | moderate- |
| warm-clamp-accfit | -0.086 | ok |
| warm-clamp-accfit-dyn | -0.355 | moderate- |
| warm-clamp-head | -0.068 | ok |
| warm-clamp-min-lanes | -0.528 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.257 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 103.3ns | 461548.6ns | 0.0% |  |
| warm-clamp-accfit | 35.8ns | 236085.5ns | 0.0% |  |
| warm-clamp-accfit-dyn | 72.0ns | 254242.0ns | 0.0% |  |
| warm-clamp-head | 121.0ns | 505624.2ns | 0.0% |  |
| warm-clamp-min-lanes | 21.6ns | 405088.5ns | 0.0% |  |
| warm-clamp-minimum | 92.2ns | 439672.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 400118.8-504213.9 ns)
  400118.8 |#################
  405323.5 |
  410528.3 |
  415733.0 |
  420937.8 |
  426142.5 |
  431347.3 |
  436552.1 |
  441756.8 |
  446961.6 |
  452166.3 |
  457371.1 |
  462575.9 |##
  467780.6 |#################
  472985.4 |########################################
  478190.1 |########
  483394.9 |##
  488599.6 |
  493804.4 |##
  499009.2 |##
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 195875.8-330948.2 ns)
  195875.8 |########################################
  202629.4 |
  209383.0 |
  216136.7 |
  222890.3 |#####
  229643.9 |#####################
  236397.5 |
  243151.1 |
  249904.8 |
  256658.4 |
  263412.0 |################
  270165.6 |##
  276919.2 |
  283672.8 |
  290426.5 |
  297180.1 |
  303933.7 |##
  310687.3 |
  317440.9 |
  324194.5 |
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 154049.3-330755.1 ns)
  154049.3 |####
  162884.6 |
  171719.9 |
  180555.2 |
  189390.5 |
  198225.8 |########
  207061.1 |
  215896.3 |
  224731.6 |
  233566.9 |
  242402.2 |
  251237.5 |########################################
  260072.8 |########
  268908.1 |############################
  277743.4 |
  286578.6 |
  295413.9 |########
  304249.2 |############################
  313084.5 |####
  321919.8 |
  (7 below, 1 above range)

warm-clamp-head (n=40, range 412097.6-679480.9 ns)
  412097.6 |###############################
  425466.8 |###################################
  438836.0 |####
  452205.1 |########
  465574.3 |########
  478943.5 |
  492312.6 |
  505681.8 |
  519050.9 |
  532420.1 |####
  545789.3 |
  559158.4 |
  572527.6 |
  585896.8 |
  599265.9 |
  612635.1 |####
  626004.2 |
  639373.4 |
  652742.6 |####
  666111.7 |########################################
  (7 below, 1 above range)

warm-clamp-min-lanes (n=40, range 359046.2-476101.0 ns)
  359046.2 |#################
  364898.9 |
  370751.7 |
  376604.4 |
  382457.1 |
  388309.9 |#######
  394162.6 |########################################
  400015.4 |##
  405868.1 |
  411720.8 |
  417573.6 |
  423426.3 |
  429279.1 |
  435131.8 |#######
  440984.5 |
  446837.3 |##
  452690.0 |
  458542.7 |
  464395.5 |
  470248.2 |
  (3 below, 6 above range)

warm-clamp-minimum (n=40, range 405369.8-470813.0 ns)
  405369.8 |
  408641.9 |
  411914.1 |
  415186.2 |#################
  418458.4 |########################################
  421730.6 |##################################
  425002.7 |
  428274.9 |
  431547.0 |
  434819.2 |
  438091.4 |
  441363.5 |
  444635.7 |
  447907.8 |
  451180.0 |#####
  454452.2 |######################
  457724.3 |#################
  460996.5 |#####
  464268.6 |######################
  467540.8 |#################
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-accfit**: CV=28.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=25.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=22.1% (high variance, measurements may be unstable)
