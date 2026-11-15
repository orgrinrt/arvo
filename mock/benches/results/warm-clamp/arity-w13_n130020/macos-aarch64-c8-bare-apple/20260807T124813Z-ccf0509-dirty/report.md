# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 16% faster than the next best (warm-clamp-minimum)

warm-clamp-accfit (273 ns) leads warm-clamp-minimum (318 ns) by 16%, a clear separation rather than a photo finish. CV 11.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 62% (significant)

warm-clamp-accfit is -440 ns (62%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 7.0x slower than the field

warm-clamp-accfit-dyn (1.92 us) is 7.0x the fastest (273 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit is fastest but the noisiest (CV 11.0%)

warm-clamp-accfit wins on median (273 ns) yet has the highest variance (CV 11.0%), while warm-clamp-min-lanes is the steadiest (CV 1.6%, 392 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.86)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-accfit-dyn} (169% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-accfit-dyn} with a 169% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.0x the fastest

Fastest warm-clamp-accfit (273 ns) to slowest warm-clamp-accfit-dyn (1.92 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 273.3 ns median (-61.6% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.02x (fastest 273.3 ns, slowest 1918.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 817ns | 777ns | 760ns | 798ns | 933ns | base |
| warm-clamp-accfit | 357ns | 333ns | 327ns | 348ns | 415ns | -56.29% |
| warm-clamp-accfit-dyn | 2006ns | 1980ns | 1947ns | 1983ns | 2132ns | +145.42% |
| warm-clamp-head | 623ns | 628ns | 597ns | 622ns | 650ns | -23.80% |
| warm-clamp-min-lanes | 453ns | 452ns | 446ns | 452ns | 465ns | -44.52% |
| warm-clamp-minimum | 374ns | 383ns | 326ns | 381ns | 400ns | -54.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 751ns | 699ns | 858ns | base | 10.902 |
| warm-clamp-accfit | 293ns | 269ns | 339ns | -60.95% | 27.917 |
| warm-clamp-accfit-dyn | 1943ns | 1888ns | 2066ns | +158.55% | 4.217 |
| warm-clamp-head | 559ns | 536ns | 584ns | -25.63% | 14.660 |
| warm-clamp-min-lanes | 395ns | 389ns | 405ns | -47.47% | 20.755 |
| warm-clamp-minimum | 308ns | 269ns | 329ns | -59.01% | 26.596 |

## Performance model

- Peak throughput: **30.439 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 11.498 | 37.8% |
| warm-clamp-accfit | 29.974 | 98.5% |
| warm-clamp-accfit-dyn | 4.270 | 14.0% |
| warm-clamp-head | 14.526 | 47.7% |
| warm-clamp-min-lanes | 20.882 | 68.6% |
| warm-clamp-minimum | 25.802 | 84.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 817ns | 817ns | base |
| warm-clamp-accfit | 357ns | 357ns | -56.29% |
| warm-clamp-accfit-dyn | 2006ns | 2006ns | +145.42% |
| warm-clamp-head | 623ns | 623ns | -23.80% |
| warm-clamp-min-lanes | 453ns | 453ns | -44.52% |
| warm-clamp-minimum | 374ns | 374ns | -54.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 712ns | base | --- | [709, 738] | --- | --- | --- | --- |
| warm-clamp-accfit | 273ns | -442.1ns (-62.0%) | [-464, -438]ns | [272, 294] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1918ns | +1193.2ns (+167.5%) | [+1153, +1218]ns | [1912, 1931] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 564ns | -163.5ns (-22.9%) | [-172, -149]ns | [546, 565] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 392ns | -320.5ns (-45.0%) | [-340, -311]ns | [391, 396] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 318ns | -442.1ns (-62.0%) | [-468, -394]ns | [307, 326] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 845ns | -60.9% | +126.9% | -36.3% | -53.4% | -61.5% |
| 2 | 847ns | -60.8% | +126.0% | -35.4% | -53.9% | -61.4% |
| 3 | 851ns | -60.8% | +125.3% | -36.5% | -54.1% | -61.6% |
| 4 | 854ns | -61.4% | +132.9% | -36.1% | -54.3% | -61.8% |
| 5 | 846ns | -60.8% | +127.8% | -33.1% | -54.3% | -61.5% |
| 6 | 850ns | -60.9% | +125.7% | -36.7% | -53.9% | -61.8% |
| 7 | 847ns | -60.9% | +131.1% | -37.1% | -52.5% | -61.6% |
| 8 | 852ns | -61.4% | +121.7% | -36.5% | -53.6% | -61.9% |
| 9 | 917ns | -64.5% | +106.0% | -41.3% | -56.6% | -64.2% |
| 10 | 849ns | -61.5% | +122.2% | -36.8% | -53.1% | -61.7% |
| 11 | 705ns | -50.2% | +172.3% | -24.2% | -44.8% | -56.5% |
| 12 | 701ns | -49.1% | +173.8% | -23.2% | -44.2% | -56.3% |
| 13 | 700ns | -50.5% | +174.2% | -23.5% | -43.9% | -55.7% |
| 14 | 699ns | -60.0% | +179.9% | -23.2% | -44.2% | -56.3% |
| 15 | 702ns | -62.0% | +177.4% | -22.9% | -44.0% | -56.3% |
| 16 | 695ns | -60.9% | +178.3% | -18.8% | -44.2% | -55.8% |
| 17 | 702ns | -61.6% | +185.2% | -19.3% | -44.4% | -56.1% |
| 18 | 700ns | -56.5% | +185.4% | -19.3% | -44.0% | -56.2% |
| 19 | 702ns | -61.2% | +189.7% | -20.8% | -44.4% | -56.1% |
| 20 | 697ns | -61.3% | +185.2% | -22.1% | -43.8% | -55.7% |
| 21 | 703ns | -61.6% | +177.2% | -8.1% | -44.2% | -53.5% |
| 22 | 709ns | -61.8% | +170.5% | -20.5% | -45.1% | -53.7% |
| 23 | 711ns | -62.3% | +174.1% | -21.1% | -44.2% | -53.5% |
| 24 | 710ns | -62.0% | +173.3% | -20.3% | -44.7% | -53.8% |
| 25 | 708ns | -62.0% | +170.8% | -13.9% | -44.8% | -53.8% |
| 26 | 700ns | -61.0% | +174.0% | -19.1% | -44.3% | -52.9% |
| 27 | 702ns | -61.4% | +172.6% | -19.5% | -44.2% | -53.1% |
| 28 | 815ns | -66.7% | +205.0% | -30.7% | -52.0% | -59.5% |
| 29 | 712ns | -61.7% | +191.4% | -20.8% | -45.1% | -53.7% |
| 30 | 712ns | -61.9% | +168.1% | -20.4% | -45.3% | -53.8% |
| 31 | 745ns | -62.1% | +153.5% | -23.4% | -46.4% | -63.5% |
| 32 | 740ns | -62.8% | +155.2% | -23.6% | -46.1% | -63.4% |
| 33 | 736ns | -63.3% | +156.6% | -23.3% | -45.9% | -63.5% |
| 34 | 734ns | -62.9% | +159.4% | -22.9% | -46.1% | -63.5% |
| 35 | 736ns | -62.9% | +156.8% | -23.4% | -46.0% | -63.4% |
| 36 | 735ns | -62.7% | +157.3% | -23.3% | -45.5% | -63.0% |
| 37 | 744ns | -63.4% | +153.4% | -23.1% | -46.5% | -63.8% |
| 38 | 720ns | -62.1% | +162.0% | -20.9% | -42.7% | -62.4% |
| 39 | 712ns | -61.5% | +165.3% | -20.0% | -41.8% | -62.1% |
| 40 | 710ns | -61.5% | +167.3% | -20.2% | -42.1% | -62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.802 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.863 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.262 | moderate+ |
| warm-clamp-head | 0.310 | moderate+ |
| warm-clamp-min-lanes | 0.675 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.864 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.7ns | 751.4ns | 0.4% |  |
| warm-clamp-accfit | 2.5ns | 293.4ns | 0.9% |  |
| warm-clamp-accfit-dyn | 2.4ns | 1942.8ns | 0.1% |  |
| warm-clamp-head | 2.5ns | 558.8ns | 0.4% |  |
| warm-clamp-min-lanes | 2.3ns | 394.7ns | 0.6% |  |
| warm-clamp-minimum | 2.4ns | 308.0ns | 0.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 699.3-858.5 ns)
    699.3 |########################################
    707.2 |################################
    715.2 |####
    723.1 |
    731.1 |################
    739.1 |############
    747.0 |
    755.0 |
    762.9 |
    770.9 |
    778.9 |
    786.8 |
    794.8 |
    802.7 |
    810.7 |####
    818.7 |
    826.6 |
    834.6 |
    842.5 |########################
    850.5 |############
  (3 below, 1 above range)

warm-clamp-accfit (n=40, range 269.1-339.3 ns)
    269.1 |########################################
    272.6 |########################
    276.1 |###
    279.7 |###
    283.2 |
    286.7 |
    290.2 |
    293.7 |
    297.2 |
    300.7 |
    304.2 |###
    307.7 |
    311.2 |
    314.7 |
    318.3 |
    321.8 |
    325.3 |######
    328.8 |#####################
    332.3 |###
    335.8 |
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1887.6-2066.4 ns)
   1887.6 |################################
   1896.6 |########
   1905.5 |########
   1914.5 |########################################
   1923.4 |####
   1932.3 |########
   1941.3 |############
   1950.2 |########
   1959.1 |
   1968.1 |
   1977.0 |
   1986.0 |########
   1994.9 |########
   2003.8 |
   2012.8 |
   2021.7 |
   2030.6 |####
   2039.6 |
   2048.5 |
   2057.5 |
  (3 below, 2 above range)

warm-clamp-head (n=40, range 536.3-584.0 ns)
    536.3 |##############
    538.7 |##############
    541.1 |#######
    543.5 |###
    545.8 |###
    548.2 |
    550.6 |
    553.0 |
    555.4 |###
    557.8 |
    560.1 |###
    562.5 |##################
    564.9 |########################################
    567.3 |##########
    569.7 |###
    572.0 |###
    574.4 |
    576.8 |
    579.2 |
    581.6 |
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 389.3-404.6 ns)
    389.3 |##########################
    390.0 |########################################
    390.8 |####################
    391.6 |##########################
    392.3 |##########################
    393.1 |
    393.9 |######
    394.6 |######
    395.4 |######
    396.2 |######
    396.9 |######
    397.7 |##########################
    398.5 |#############
    399.2 |
    400.0 |######
    400.8 |
    401.5 |######
    402.3 |
    403.1 |
    403.8 |
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 269.4-329.4 ns)
    269.4 |####################
    272.4 |
    275.4 |
    278.4 |
    281.4 |
    284.4 |
    287.4 |
    290.4 |
    293.4 |
    296.4 |
    299.4 |
    302.4 |
    305.4 |################################
    308.4 |########
    311.4 |
    314.4 |
    317.4 |
    320.4 |
    323.4 |########################
    326.4 |########################################
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.86 (measurement drift or warm-up artifact)
