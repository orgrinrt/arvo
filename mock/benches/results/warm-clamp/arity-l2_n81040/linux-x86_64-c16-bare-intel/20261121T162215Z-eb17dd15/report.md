# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-acc64 dominates: 111% faster than the next best (warm-clamp-minimum)

warm-clamp-acc64 (118.62 us) leads warm-clamp-minimum (249.88 us) by 111%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head is an outlier: 2.4x slower than the field

warm-clamp-head (285.59 us) is 2.4x the fastest (118.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 is fastest but the noisiest (CV 10.1%)

warm-clamp-acc64 wins on median (118.62 us) yet has the highest variance (CV 10.1%), while warm-clamp-min-lanes is the steadiest (CV 0.2%, 250.12 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.67)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (118.62 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64} vs {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-head} (111% apart)

The field splits into a fast tier {warm-clamp-acc64} and a slow tier {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-head} with a 111% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 118620.3 ns median
- 5 variants significantly slower than baseline
- Spread: 2.41x (fastest 118620.3 ns, slowest 285585.1 ns)

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
| Counter frequency | 2112705153 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 111837ns | 118690ns | 90735ns | 116511ns | 118913ns | base |
| warm-clamp-accfit | 278118ns | 277556ns | 274675ns | 277770ns | 282605ns | +148.68% |
| warm-clamp-accfit-dyn | 263169ns | 263241ns | 260877ns | 263223ns | 265302ns | +135.32% |
| warm-clamp-head | 286828ns | 285791ns | 281646ns | 286730ns | 292303ns | +156.47% |
| warm-clamp-min-lanes | 250378ns | 250203ns | 250185ns | 250218ns | 251049ns | +123.88% |
| warm-clamp-minimum | 250284ns | 249977ns | 249934ns | 250003ns | 251477ns | +123.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 111762ns | 90662ns | 118825ns | base | 9.382 |
| warm-clamp-accfit | 278030ns | 274592ns | 282520ns | +148.77% | 3.771 |
| warm-clamp-accfit-dyn | 263084ns | 260799ns | 265206ns | +135.40% | 3.986 |
| warm-clamp-head | 286650ns | 281482ns | 292132ns | +156.48% | 3.658 |
| warm-clamp-min-lanes | 250287ns | 250099ns | 250958ns | +123.95% | 4.189 |
| warm-clamp-minimum | 250192ns | 249848ns | 251390ns | +123.86% | 4.191 |

## Performance model

- Peak throughput: **11.566 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.840 | 76.4% |
| warm-clamp-accfit | 3.779 | 32.7% |
| warm-clamp-accfit-dyn | 3.985 | 34.5% |
| warm-clamp-head | 3.672 | 31.7% |
| warm-clamp-min-lanes | 4.192 | 36.2% |
| warm-clamp-minimum | 4.196 | 36.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 111837ns | 111837ns | base |
| warm-clamp-accfit | 278118ns | 278118ns | +148.68% |
| warm-clamp-accfit-dyn | 263169ns | 263169ns | +135.32% |
| warm-clamp-head | 286828ns | 286828ns | +156.47% |
| warm-clamp-min-lanes | 250378ns | 250378ns | +123.88% |
| warm-clamp-minimum | 250284ns | 250284ns | +123.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 118620ns | base | --- | [118598, 118631] | --- | --- | --- | --- |
| warm-clamp-accfit | 277477ns | +161104.0ns (+135.8%) | [+159529, +163791]ns | [276167, 279158] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 263161ns | +145477.6ns (+122.6%) | [+144802, +146208]ns | [262497, 263789] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 285585ns | +170831.8ns (+144.0%) | [+166937, +173264]ns | [285209, 287682] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 250115ns | +131538.5ns (+110.9%) | [+131504, +131868]ns | [250110, 250122] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 249877ns | +131285.2ns (+110.7%) | [+131237, +131545]ns | [249866, 249938] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 118620ns | +137.8% | +122.9% | +145.9% | +110.9% | +110.7% |
| 1 | 2 | 118625ns | +134.2% | +120.3% | +137.4% | +110.9% | +110.6% |
| 1 | 3 | 118636ns | +132.9% | +122.2% | +140.8% | +110.8% | +110.6% |
| 1 | 4 | 90599ns | +203.8% | +189.9% | +214.9% | +176.1% | +175.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.084 | ok |
| warm-clamp-accfit | 0.144 | ok |
| warm-clamp-accfit-dyn | -0.671 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.441 | moderate- |
| warm-clamp-min-lanes | -0.244 | moderate- |
| warm-clamp-minimum | -0.413 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 0/4, lost 4/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 20.0ns | 111761.7ns | 0.0% |  |
| warm-clamp-accfit | 20.2ns | 278030.1ns | 0.0% |  |
| warm-clamp-accfit-dyn | 19.6ns | 263084.2ns | 0.0% |  |
| warm-clamp-head | 31.7ns | 286650.4ns | 0.0% |  |
| warm-clamp-min-lanes | 19.9ns | 250287.2ns | 0.0% |  |
| warm-clamp-minimum | 19.7ns | 250191.9ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 90662.4-118825.3 ns)
  90662.4 |####
  92070.5 |#
  93478.7 |
  94886.8 |
  96295.0 |
  97703.1 |
  99111.3 |
  100519.4 |
  101927.6 |
  103335.7 |
  104743.9 |
  106152.0 |
  107560.1 |
  108968.3 |
  110376.4 |
  111784.6 |
  113192.7 |
  114600.9 |
  116009.0 |
  117417.2 |########################################
  (6 below, 4 above range)

warm-clamp-accfit (n=40, range 274591.6-282519.5 ns)
  274591.6 |##########
  274988.0 |########################################
  275384.4 |####################
  275780.8 |##############################
  276177.2 |####################
  276573.6 |
  276970.0 |##############################
  277366.4 |##############################
  277762.8 |##########
  278159.2 |####################
  278555.6 |
  278952.0 |####################
  279348.4 |####################
  279744.7 |
  280141.1 |####################
  280537.5 |##########
  280933.9 |
  281330.3 |
  281726.7 |##############################
  282123.1 |##########
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 260798.6-265206.1 ns)
  260798.6 |
  261019.0 |
  261239.4 |########################
  261459.7 |########
  261680.1 |
  261900.5 |################################
  262120.9 |########
  262341.2 |########################
  262561.6 |########################
  262782.0 |
  263002.3 |########################
  263222.7 |################
  263443.1 |################
  263663.5 |########
  263883.8 |################################
  264104.2 |
  264324.6 |########################################
  264545.0 |########
  264765.3 |
  264985.7 |########################
  (3 below, 1 above range)

warm-clamp-head (n=40, range 281482.3-292131.9 ns)
  281482.3 |####################
  282014.7 |##########
  282547.2 |
  283079.7 |
  283612.2 |
  284144.7 |#####
  284677.2 |###############
  285209.6 |########################################
  285742.1 |#####
  286274.6 |##########
  286807.1 |
  287339.6 |#####
  287872.0 |#####
  288404.5 |
  288937.0 |
  289469.5 |
  290002.0 |#####
  290534.4 |#####
  291066.9 |##############################
  291599.4 |##########
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 250098.6-250958.4 ns)
  250098.6 |########################################
  250141.6 |#
  250184.6 |#
  250227.6 |
  250270.6 |####
  250313.6 |
  250356.5 |#
  250399.5 |#
  250442.5 |
  250485.5 |#
  250528.5 |
  250571.5 |
  250614.5 |
  250657.5 |
  250700.4 |
  250743.4 |
  250786.4 |
  250829.4 |
  250872.4 |
  250915.4 |
  (2 below, 3 above range)

warm-clamp-minimum (n=40, range 249847.8-251389.5 ns)
  249847.8 |########################################
  249924.9 |########
  250002.0 |###
  250079.1 |###
  250156.2 |#
  250233.2 |
  250310.3 |
  250387.4 |
  250464.5 |
  250541.6 |
  250618.7 |
  250695.8 |#
  250772.8 |
  250849.9 |
  250927.0 |
  251004.1 |
  251081.2 |
  251158.3 |
  251235.4 |
  251312.4 |
  (2 below, 4 above range)

```
