# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes is an outlier: 8.0x slower than the field

warm-clamp-min-lanes (3.98 us) is 8.0x the fastest (498 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 is fastest but the noisiest (CV 9.2%)

warm-clamp-acc64 wins on median (498 ns) yet has the highest variance (CV 9.2%), while warm-clamp-head is the steadiest (CV 2.5%, 2.24 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-acc64, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-acc64 (498 ns) and warm-clamp-minimum (499 ns) differ by 0.26%, inside the noise, even though the wider field spreads 699.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.89)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (498 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-accfit-dyn} vs {warm-clamp-head, warm-clamp-min-lanes} (129% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-head, warm-clamp-min-lanes} with a 129% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.0x the fastest

Fastest warm-clamp-acc64 (498 ns) to slowest warm-clamp-min-lanes (3.98 us): 8.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum's edge over baseline is significant but tiny (3 ns, 0.58%)

warm-clamp-minimum differs from baseline warm-clamp-acc64 by 3 ns (0.58%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 497.5 ns median
- 4 variants significantly slower than baseline
- Spread: 7.99x (fastest 497.5 ns, slowest 3976.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 587ns | 559ns | 555ns | 569ns | 676ns | base |
| warm-clamp-accfit | 616ns | 630ns | 562ns | 613ns | 677ns | +4.85% |
| warm-clamp-accfit-dyn | 1043ns | 1041ns | 1024ns | 1037ns | 1081ns | +77.65% |
| warm-clamp-head | 2357ns | 2334ns | 2329ns | 2338ns | 2443ns | +301.40% |
| warm-clamp-min-lanes | 4257ns | 4042ns | 3934ns | 4188ns | 4785ns | +624.84% |
| warm-clamp-minimum | 588ns | 560ns | 556ns | 569ns | 677ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 524ns | 495ns | 604ns | base | 15.640 |
| warm-clamp-accfit | 548ns | 499ns | 607ns | +4.58% | 14.954 |
| warm-clamp-accfit-dyn | 981ns | 965ns | 1016ns | +87.37% | 8.347 |
| warm-clamp-head | 2266ns | 2240ns | 2348ns | +332.65% | 3.615 |
| warm-clamp-min-lanes | 4187ns | 3871ns | 4709ns | +699.35% | 1.957 |
| warm-clamp-minimum | 524ns | 496ns | 605ns | +0.12% | 15.621 |

## Performance model

- Peak throughput: **16.547 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 16.466 | 99.5% |
| warm-clamp-accfit | 14.596 | 88.2% |
| warm-clamp-accfit-dyn | 8.364 | 50.6% |
| warm-clamp-head | 3.651 | 22.1% |
| warm-clamp-min-lanes | 2.060 | 12.5% |
| warm-clamp-minimum | 16.423 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 587ns | 587ns | base |
| warm-clamp-accfit | 616ns | 616ns | +4.85% |
| warm-clamp-accfit-dyn | 1043ns | 1043ns | +77.65% |
| warm-clamp-head | 2357ns | 2357ns | +301.40% |
| warm-clamp-min-lanes | 4257ns | 4257ns | +624.84% |
| warm-clamp-minimum | 588ns | 588ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 498ns | base | --- | [497, 499] | --- | --- | --- | --- |
| warm-clamp-accfit | 561ns | +8.1ns (+1.6%) | [+2, +20]ns | [510, 565] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 979ns | +468.9ns (+94.3%) | [+468, +478]ns | [966, 981] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2244ns | +1746.9ns (+351.1%) | [+1744, +1748]ns | [2242, 2246] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3976ns | +3480.7ns (+699.6%) | [+3428, +3748]ns | [3937, 4298] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 499ns | no significant difference | [-0, +1]ns | [498, 499] | no | 0.1877 | 0.1877 | 3 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 497ns | +5.2% | +100.2% | +350.7% | +680.1% | +0.0% |
| 2 | 498ns | +4.6% | +94.1% | +350.9% | +678.3% | +0.3% |
| 3 | 497ns | +2.8% | +94.0% | +351.4% | +679.0% | +0.2% |
| 4 | 497ns | +1.3% | +94.1% | +351.5% | +678.9% | +0.6% |
| 5 | 497ns | +1.5% | +94.3% | +351.7% | +697.8% | +0.7% |
| 6 | 499ns | +0.8% | +93.0% | +349.4% | +705.5% | +0.2% |
| 7 | 496ns | +1.4% | +94.9% | +352.5% | +712.5% | +0.2% |
| 8 | 497ns | +2.0% | +94.2% | +351.8% | +712.5% | +0.2% |
| 9 | 497ns | +1.8% | +94.5% | +351.5% | +718.6% | +0.4% |
| 10 | 493ns | +2.4% | +95.7% | +358.1% | +698.4% | +0.3% |
| 11 | 499ns | +2.1% | +93.5% | +350.0% | +691.3% | -0.3% |
| 12 | 494ns | +4.7% | +95.6% | +354.1% | +705.8% | +0.9% |
| 13 | 498ns | +3.6% | +94.1% | +350.8% | +698.8% | -0.3% |
| 14 | 497ns | +0.8% | +94.1% | +351.0% | +691.7% | -0.3% |
| 15 | 498ns | -0.2% | +100.9% | +350.0% | +707.0% | +0.0% |
| 16 | 495ns | +0.8% | +95.0% | +352.7% | +689.7% | +0.5% |
| 17 | 498ns | -0.3% | +94.1% | +350.7% | +684.0% | -0.3% |
| 18 | 498ns | -0.3% | +93.3% | +350.8% | +691.2% | -0.2% |
| 19 | 497ns | +0.1% | +107.1% | +351.1% | +687.3% | -0.1% |
| 20 | 497ns | +23.3% | +96.2% | +351.0% | +677.3% | +0.2% |
| 21 | 495ns | +22.5% | +102.2% | +375.3% | +825.7% | +0.5% |
| 22 | 498ns | +21.4% | +100.3% | +367.4% | +844.8% | +0.3% |
| 23 | 500ns | +21.4% | +99.0% | +363.8% | +840.5% | -0.2% |
| 24 | 497ns | +22.1% | +99.3% | +366.6% | +845.2% | +0.2% |
| 25 | 504ns | +19.5% | +95.0% | +352.5% | +832.1% | -1.0% |
| 26 | 502ns | +19.5% | +95.3% | +354.1% | +836.8% | -0.6% |
| 27 | 499ns | +21.4% | +96.5% | +413.2% | +840.6% | +0.0% |
| 28 | 497ns | +20.9% | +97.5% | +356.8% | +864.2% | +0.2% |
| 29 | 495ns | +22.9% | +97.9% | +359.6% | +681.6% | +0.8% |
| 30 | 495ns | +21.1% | +97.8% | +356.2% | +819.4% | +0.8% |
| 31 | 601ns | -4.8% | +63.1% | +291.2% | +555.4% | +1.0% |
| 32 | 604ns | -6.4% | +62.4% | +273.3% | +541.1% | -0.1% |
| 33 | 602ns | -6.2% | +63.2% | +272.6% | +543.9% | +0.7% |
| 34 | 602ns | -6.6% | +62.5% | +272.1% | +653.8% | -0.2% |
| 35 | 606ns | -7.6% | +62.1% | +270.0% | +675.0% | +0.3% |
| 36 | 605ns | -7.0% | +62.2% | +270.7% | +675.4% | -0.3% |
| 37 | 605ns | -6.7% | +86.0% | +270.5% | +673.8% | -0.5% |
| 38 | 602ns | -6.3% | +62.6% | +271.9% | +651.9% | +0.5% |
| 39 | 602ns | -6.2% | +62.6% | +272.4% | +543.5% | +0.1% |
| 40 | 605ns | -7.0% | +62.2% | +270.8% | +539.9% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.889 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.892 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.062 | ok |
| warm-clamp-head | 0.211 | moderate+ |
| warm-clamp-min-lanes | 0.659 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.892 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 13/40, lost 26/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 13/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.1ns | 523.8ns | 0.6% |  |
| warm-clamp-accfit | 2.6ns | 547.8ns | 0.5% |  |
| warm-clamp-accfit-dyn | 2.7ns | 981.4ns | 0.3% |  |
| warm-clamp-head | 2.6ns | 2266.2ns | 0.1% |  |
| warm-clamp-min-lanes | 2.7ns | 4186.9ns | 0.1% |  |
| warm-clamp-minimum | 2.5ns | 524.4ns | 0.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 495.1-603.9 ns)
    495.1 |########################################
    500.5 |###
    506.0 |
    511.4 |
    516.8 |
    522.3 |
    527.7 |
    533.2 |
    538.6 |
    544.0 |
    549.5 |
    554.9 |
    560.4 |
    565.8 |
    571.2 |
    576.7 |
    582.1 |
    587.5 |
    593.0 |
    598.4 |########
  (3 below, 5 above range)

warm-clamp-accfit (n=40, range 499.2-606.7 ns)
    499.2 |########################################
    504.5 |##########################
    509.9 |######
    515.3 |#############
    520.7 |#############
    526.0 |
    531.4 |
    536.8 |
    542.2 |
    547.5 |
    552.9 |
    558.3 |##########################
    563.7 |#################################
    569.0 |######
    574.4 |
    579.8 |
    585.2 |
    590.5 |
    595.9 |####################
    601.3 |####################
  (4 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 964.5-1016.4 ns)
    964.5 |########################################
    967.1 |
    969.7 |
    972.3 |##
    974.9 |
    977.5 |##############
    980.1 |############################
    982.7 |
    985.3 |
    987.9 |##
    990.5 |
    993.0 |#####
    995.6 |##
    998.2 |##
   1000.8 |##
   1003.4 |
   1006.0 |
   1008.6 |
   1011.2 |
   1013.8 |
  (2 below, 2 above range)

warm-clamp-head (n=40, range 2240.5-2348.2 ns)
   2240.5 |########################################
   2245.9 |#
   2251.3 |#
   2256.7 |###
   2262.0 |
   2267.4 |#
   2272.8 |#
   2278.2 |###
   2283.6 |
   2289.0 |
   2294.3 |
   2299.7 |
   2305.1 |
   2310.5 |
   2315.9 |###
   2321.3 |#
   2326.7 |
   2332.0 |
   2337.4 |
   2342.8 |
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 3870.7-4709.0 ns)
   3870.7 |########################################
   3912.6 |##################
   3954.6 |##########
   3996.5 |##############
   4038.4 |###
   4080.3 |
   4122.2 |
   4164.1 |
   4206.0 |
   4248.0 |
   4289.9 |
   4331.8 |
   4373.7 |
   4415.6 |
   4457.5 |
   4499.4 |###
   4541.4 |#######
   4583.3 |###
   4625.2 |
   4667.1 |################################
  (2 below, 1 above range)

warm-clamp-minimum (n=40, range 496.2-604.6 ns)
    496.2 |########################################
    501.7 |
    507.1 |
    512.5 |
    517.9 |
    523.3 |
    528.8 |
    534.2 |
    539.6 |
    545.0 |
    550.4 |
    555.9 |
    561.3 |
    566.7 |
    572.1 |
    577.5 |
    583.0 |
    588.4 |
    593.8 |#
    599.2 |########
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.89 (measurement drift or warm-up artifact)
