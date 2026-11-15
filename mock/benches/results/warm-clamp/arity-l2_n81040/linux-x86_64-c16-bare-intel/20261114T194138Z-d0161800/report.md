# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-acc64 dominates: 105% faster than the next best (warm-clamp-head)

warm-clamp-acc64 (118.57 us) leads warm-clamp-head (243.20 us) by 105%, a clear separation rather than a photo finish. CV 11.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn is an outlier: 2.3x slower than the field

warm-clamp-accfit-dyn (276.76 us) is 2.3x the fastest (118.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 is fastest but the noisiest (CV 11.6%)

warm-clamp-acc64 wins on median (118.57 us) yet has the highest variance (CV 11.6%), while warm-clamp-head is the steadiest (CV 0.1%, 243.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.61)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (118.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64} vs {warm-clamp-head, warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-accfit-dyn} (105% apart)

The field splits into a fast tier {warm-clamp-acc64} and a slow tier {warm-clamp-head, warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-accfit-dyn} with a 105% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 118568.5 ns median
- 5 variants significantly slower than baseline
- Spread: 2.33x (fastest 118568.5 ns, slowest 276764.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 106564ns | 118640ns | 90556ns | 107828ns | 118780ns | base |
| warm-clamp-accfit | 276127ns | 275895ns | 272560ns | 275733ns | 280872ns | +159.12% |
| warm-clamp-accfit-dyn | 276551ns | 276844ns | 272581ns | 276686ns | 280117ns | +159.52% |
| warm-clamp-head | 243399ns | 243356ns | 243202ns | 243370ns | 243683ns | +128.41% |
| warm-clamp-min-lanes | 250567ns | 250167ns | 250131ns | 250176ns | 252173ns | +135.13% |
| warm-clamp-minimum | 249954ns | 249874ns | 249839ns | 249884ns | 250278ns | +134.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 106487ns | 90484ns | 118694ns | base | 9.847 |
| warm-clamp-accfit | 276042ns | 272481ns | 280791ns | +159.23% | 3.799 |
| warm-clamp-accfit-dyn | 276467ns | 272493ns | 280038ns | +159.63% | 3.793 |
| warm-clamp-head | 243213ns | 243012ns | 243477ns | +128.40% | 4.311 |
| warm-clamp-min-lanes | 250475ns | 250048ns | 252064ns | +135.22% | 4.186 |
| warm-clamp-minimum | 249866ns | 249759ns | 250183ns | +134.65% | 4.197 |

## Performance model

- Peak throughput: **11.589 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.844 | 76.3% |
| warm-clamp-accfit | 3.802 | 32.8% |
| warm-clamp-accfit-dyn | 3.789 | 32.7% |
| warm-clamp-head | 4.312 | 37.2% |
| warm-clamp-min-lanes | 4.193 | 36.2% |
| warm-clamp-minimum | 4.198 | 36.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 106564ns | 106564ns | base |
| warm-clamp-accfit | 276127ns | 276127ns | +159.12% |
| warm-clamp-accfit-dyn | 276551ns | 276551ns | +159.52% |
| warm-clamp-head | 243399ns | 243399ns | +128.41% |
| warm-clamp-min-lanes | 250567ns | 250567ns | +135.13% |
| warm-clamp-minimum | 249954ns | 249954ns | +134.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 118569ns | base | --- | [90552, 118601] | --- | --- | --- | --- |
| warm-clamp-accfit | 275816ns | +162601.7ns (+137.1%) | [+160590, +182556]ns | [273876, 277093] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 276765ns | +160340.6ns (+135.2%) | [+158588, +185079]ns | [275922, 277379] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 243195ns | +124869.1ns (+105.3%) | [+124594, +152465]ns | [243162, 243206] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 250080ns | +132207.1ns (+111.5%) | [+131533, +159534]ns | [250071, 250090] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 249792ns | +131668.5ns (+111.0%) | [+131227, +159210]ns | [249786, 249811] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 118605ns | +136.7% | +134.3% | +105.0% | +110.9% | +110.6% |
| 1 | 2 | 118612ns | +130.3% | +130.0% | +105.0% | +110.8% | +110.6% |
| 1 | 3 | 90525ns | +202.5% | +205.9% | +168.6% | +176.2% | +175.9% |
| 1 | 4 | 90525ns | +205.9% | +205.1% | +168.7% | +176.2% | +175.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.250 | moderate+ |
| warm-clamp-accfit | -0.234 | moderate- |
| warm-clamp-accfit-dyn | -0.609 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.176 | ok |
| warm-clamp-min-lanes | -0.108 | ok |
| warm-clamp-minimum | -0.158 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 0/4, lost 4/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 20.3ns | 106486.5ns | 0.0% |  |
| warm-clamp-accfit | 20.2ns | 276042.3ns | 0.0% |  |
| warm-clamp-accfit-dyn | 21.9ns | 276467.2ns | 0.0% |  |
| warm-clamp-head | 33.7ns | 243212.9ns | 0.0% |  |
| warm-clamp-min-lanes | 21.1ns | 250474.8ns | 0.0% |  |
| warm-clamp-minimum | 21.6ns | 249866.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 90483.9-118694.3 ns)
  90483.9 |############################
  91894.4 |
  93305.0 |
  94715.5 |
  96126.0 |
  97536.5 |
  98947.0 |
  100357.6 |
  101768.1 |
  103178.6 |
  104589.1 |
  105999.6 |
  107410.2 |
  108820.7 |
  110231.2 |##
  111641.7 |
  113052.2 |
  114462.8 |
  115873.3 |
  117283.8 |########################################
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 272480.8-280791.3 ns)
  272480.8 |#############
  272896.3 |########################################
  273311.8 |######
  273727.3 |##########################
  274142.9 |######
  274558.4 |######
  274973.9 |
  275389.4 |######
  275805.0 |######
  276220.5 |####################
  276636.0 |#############
  277051.5 |#############
  277467.1 |
  277882.6 |######
  278298.1 |######
  278713.6 |#############
  279129.2 |######
  279544.7 |
  279960.2 |
  280375.7 |####################
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 272493.5-280038.4 ns)
  272493.5 |################
  272870.7 |
  273248.0 |
  273625.2 |########
  274002.5 |########
  274379.7 |########
  274757.0 |########
  275134.2 |########################
  275511.5 |
  275888.7 |################################
  276266.0 |################
  276643.2 |########################################
  277020.5 |################
  277397.7 |########################
  277775.0 |################
  278152.2 |
  278529.4 |################
  278906.7 |################
  279283.9 |########
  279661.2 |########
  (4 below, 3 above range)

warm-clamp-head (n=40, range 243011.6-243477.5 ns)
  243011.6 |######
  243034.9 |##########################
  243058.2 |
  243081.5 |######
  243104.8 |#############
  243128.1 |
  243151.4 |#################################
  243174.7 |########################################
  243198.0 |########################################
  243221.3 |######
  243244.5 |######
  243267.8 |######
  243291.1 |
  243314.4 |
  243337.7 |######
  243361.0 |######
  243384.3 |######
  243407.6 |
  243430.9 |######
  243454.2 |
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 250048.2-252063.7 ns)
  250048.2 |########################################
  250149.0 |######
  250249.8 |###
  250350.5 |
  250451.3 |
  250552.1 |
  250652.9 |#
  250753.6 |#
  250854.4 |#
  250955.2 |
  251056.0 |
  251156.7 |
  251257.5 |
  251358.3 |
  251459.1 |
  251559.9 |
  251660.6 |
  251761.4 |
  251862.2 |
  251963.0 |
  (4 below, 1 above range)

warm-clamp-minimum (n=40, range 249758.8-250182.6 ns)
  249758.8 |########################
  249780.0 |########################################
  249801.2 |###############
  249822.4 |#########
  249843.6 |###
  249864.8 |
  249886.0 |###
  249907.1 |###
  249928.3 |
  249949.5 |
  249970.7 |
  249991.9 |
  250013.1 |
  250034.3 |
  250055.4 |
  250076.6 |
  250097.8 |
  250119.0 |###
  250140.2 |
  250161.4 |
  (4 below, 3 above range)

```
