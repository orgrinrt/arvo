# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head is an outlier: 3.8x slower than the field

warm-clamp-head (2.38 us) is 3.8x the fastest (621 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum is fastest but the noisiest (CV 9.1%)

warm-clamp-minimum wins on median (621 ns) yet has the highest variance (CV 9.1%), while warm-clamp-head is the steadiest (CV 1.6%, 2.38 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-minimum, warm-clamp-acc64) are a dead heat (<1%)

warm-clamp-minimum (621 ns) and warm-clamp-acc64 (622 ns) differ by 0.17%, inside the noise, even though the wider field spreads 283.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.91)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.91, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-acc64, warm-clamp-accfit} vs {warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-head} (124% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-acc64, warm-clamp-accfit} and a slow tier {warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-head} with a 124% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.8x the fastest

Fastest warm-clamp-minimum (621 ns) to slowest warm-clamp-head (2.38 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (10 ns, 1.67%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 10 ns (1.67%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 621.5 ns median (-0.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 3.83x (fastest 621.5 ns, slowest 2380.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 723ns | 687ns | 678ns | 703ns | 828ns | base |
| warm-clamp-accfit | 746ns | 746ns | 682ns | 741ns | 826ns | +3.23% |
| warm-clamp-accfit-dyn | 1622ns | 1581ns | 1538ns | 1595ns | 1787ns | +124.33% |
| warm-clamp-head | 2490ns | 2470ns | 2465ns | 2476ns | 2557ns | +244.32% |
| warm-clamp-min-lanes | 2075ns | 2063ns | 2030ns | 2069ns | 2138ns | +187.00% |
| warm-clamp-minimum | 721ns | 684ns | 679ns | 698ns | 830ns | -0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 658ns | 616ns | 753ns | base | 12.457 |
| warm-clamp-accfit | 679ns | 621ns | 752ns | +3.29% | 12.060 |
| warm-clamp-accfit-dyn | 1556ns | 1473ns | 1716ns | +136.53% | 5.266 |
| warm-clamp-head | 2399ns | 2376ns | 2463ns | +264.84% | 3.414 |
| warm-clamp-min-lanes | 2011ns | 1968ns | 2072ns | +205.77% | 4.074 |
| warm-clamp-minimum | 655ns | 617ns | 753ns | -0.45% | 12.513 |

## Performance model

- Peak throughput: **13.290 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 13.160 | 99.0% |
| warm-clamp-accfit | 12.066 | 90.8% |
| warm-clamp-accfit-dyn | 5.396 | 40.6% |
| warm-clamp-head | 3.441 | 25.9% |
| warm-clamp-min-lanes | 4.098 | 30.8% |
| warm-clamp-minimum | 13.182 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 723ns | 723ns | base |
| warm-clamp-accfit | 746ns | 746ns | +3.23% |
| warm-clamp-accfit-dyn | 1622ns | 1622ns | +124.33% |
| warm-clamp-head | 2490ns | 2490ns | +244.32% |
| warm-clamp-min-lanes | 2075ns | 2075ns | +187.00% |
| warm-clamp-minimum | 721ns | 721ns | -0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 622ns | base | --- | [621, 634] | --- | --- | --- | --- |
| warm-clamp-accfit | 679ns | no significant difference | [-7, +21]ns | [634, 704] | no | 0.3352 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 1518ns | +882.7ns (+141.8%) | [+858, +921]ns | [1476, 1553] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2381ns | +1759.2ns (+282.6%) | [+1754, +1764]ns | [2379, 2390] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1999ns | +1358.5ns (+218.2%) | [+1348, +1376]ns | [1994, 2017] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 621ns | no significant difference | [-15, +2]ns | [620, 628] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 752ns | -6.7% | +122.6% | +215.8% | +174.8% | -17.5% |
| 2 | 752ns | -6.2% | +122.7% | +215.9% | +175.3% | -17.5% |
| 3 | 753ns | -6.5% | +121.9% | +216.0% | +175.0% | -17.7% |
| 4 | 751ns | -6.2% | +122.6% | +216.3% | +168.9% | -17.4% |
| 5 | 754ns | -6.8% | +121.9% | +234.2% | +164.4% | -17.8% |
| 6 | 752ns | -6.5% | +122.7% | +238.6% | +165.2% | -17.3% |
| 7 | 752ns | -6.4% | +131.4% | +216.4% | +165.0% | -15.3% |
| 8 | 754ns | -6.7% | +122.0% | +215.4% | +164.2% | -17.5% |
| 9 | 752ns | -6.1% | +122.7% | +216.2% | +165.9% | -18.4% |
| 10 | 753ns | -6.6% | +121.7% | +215.3% | +168.0% | -17.5% |
| 11 | 621ns | +21.0% | +137.9% | +286.0% | +230.0% | -0.3% |
| 12 | 633ns | +18.7% | +132.8% | +276.1% | +215.3% | -2.0% |
| 13 | 621ns | +21.1% | +138.1% | +283.3% | +222.6% | +0.3% |
| 14 | 620ns | +21.2% | +138.4% | +284.0% | +221.9% | -0.1% |
| 15 | 621ns | +21.2% | +137.6% | +283.5% | +236.2% | +1.9% |
| 16 | 628ns | +20.0% | +134.8% | +278.9% | +229.3% | -1.4% |
| 17 | 622ns | +20.5% | +137.1% | +282.7% | +231.7% | +2.3% |
| 18 | 612ns | +22.9% | +140.7% | +288.6% | +238.0% | +5.1% |
| 19 | 655ns | +14.8% | +125.6% | +263.6% | +216.0% | -5.5% |
| 20 | 629ns | +19.6% | +134.5% | +287.3% | +228.9% | -1.2% |
| 21 | 618ns | +3.8% | +138.6% | +284.8% | +223.9% | +22.1% |
| 22 | 621ns | +1.7% | +136.8% | +282.5% | +221.6% | +20.7% |
| 23 | 622ns | +1.2% | +138.9% | +282.5% | +226.1% | +20.9% |
| 24 | 619ns | +1.6% | +139.2% | +284.3% | +225.7% | +21.5% |
| 25 | 690ns | -8.6% | +113.2% | +246.3% | +184.8% | +9.1% |
| 26 | 619ns | +1.9% | +138.4% | +283.9% | +218.2% | +21.7% |
| 27 | 622ns | +1.1% | +137.0% | +281.9% | +218.6% | +20.8% |
| 28 | 616ns | +3.2% | +139.2% | +286.1% | +219.5% | +22.1% |
| 29 | 622ns | +4.8% | +136.5% | +282.2% | +215.9% | +20.8% |
| 30 | 614ns | +6.8% | +139.7% | +286.9% | +220.1% | +22.4% |
| 31 | 649ns | -1.9% | +138.8% | +269.4% | +203.8% | -4.3% |
| 32 | 624ns | +0.7% | +148.9% | +283.3% | +215.7% | -0.5% |
| 33 | 619ns | +3.4% | +151.3% | +300.0% | +218.9% | +0.1% |
| 34 | 620ns | +0.4% | +150.1% | +286.9% | +223.9% | +0.1% |
| 35 | 617ns | +0.1% | +151.8% | +296.9% | +218.9% | +0.2% |
| 36 | 620ns | -0.3% | +149.9% | +292.9% | +227.0% | +0.3% |
| 37 | 617ns | +1.1% | +151.6% | +292.1% | +220.8% | -0.1% |
| 38 | 622ns | -0.5% | +149.3% | +288.9% | +218.2% | +0.2% |
| 39 | 631ns | -2.2% | +146.0% | +282.9% | +217.1% | -2.6% |
| 40 | 634ns | -1.7% | +206.4% | +281.5% | +215.4% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.833 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.908 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.579 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.375 | moderate+ |
| warm-clamp-min-lanes | 0.669 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.847 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 16/40, lost 23/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 19/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.2ns | 657.6ns | 0.5% |  |
| warm-clamp-accfit | 3.0ns | 679.3ns | 0.4% |  |
| warm-clamp-accfit-dyn | 3.0ns | 1555.5ns | 0.2% |  |
| warm-clamp-head | 3.2ns | 2399.3ns | 0.1% |  |
| warm-clamp-min-lanes | 2.6ns | 2010.8ns | 0.1% |  |
| warm-clamp-minimum | 2.9ns | 654.7ns | 0.4% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 616.4-752.9 ns)
    616.4 |########################################
    623.3 |######
    630.1 |######
    636.9 |
    643.7 |##
    650.6 |##
    657.4 |
    664.2 |
    671.0 |
    677.9 |
    684.7 |##
    691.5 |
    698.3 |
    705.1 |
    712.0 |
    718.8 |
    725.6 |
    732.4 |
    739.3 |
    746.1 |#############
  (3 below, 4 above range)

warm-clamp-accfit (n=40, range 621.1-752.3 ns)
    621.1 |############
    627.7 |############################
    634.2 |############
    640.8 |####
    647.3 |####
    653.9 |####
    660.4 |
    667.0 |
    673.6 |
    680.1 |
    686.7 |
    693.2 |
    699.8 |########################################
    706.4 |
    712.9 |
    719.5 |
    726.0 |
    732.6 |
    739.1 |
    745.7 |############################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1472.7-1716.0 ns)
   1472.7 |########################################
   1484.9 |##
   1497.0 |
   1509.2 |
   1521.4 |
   1533.5 |
   1545.7 |########################
   1557.9 |
   1570.0 |
   1582.2 |
   1594.4 |
   1606.5 |
   1618.7 |
   1630.9 |
   1643.0 |
   1655.2 |
   1667.4 |########################
   1679.5 |
   1691.7 |
   1703.9 |
  (4 below, 2 above range)

warm-clamp-head (n=40, range 2375.9-2463.1 ns)
   2375.9 |########################################
   2380.2 |########################
   2384.6 |
   2388.9 |######
   2393.3 |#########
   2397.7 |
   2402.0 |
   2406.4 |
   2410.7 |
   2415.1 |######
   2419.5 |######
   2423.8 |
   2428.2 |
   2432.5 |###
   2436.9 |###
   2441.3 |
   2445.6 |###
   2450.0 |
   2454.4 |
   2458.7 |
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1968.0-2071.7 ns)
   1968.0 |#################
   1973.2 |#####
   1978.3 |#################
   1983.5 |
   1988.7 |###########
   1993.9 |##################################
   1999.1 |######################
   2004.3 |#####
   2009.5 |
   2014.7 |###########
   2019.8 |#####
   2025.0 |###########
   2030.2 |
   2035.4 |
   2040.6 |
   2045.8 |#####
   2051.0 |
   2056.2 |
   2061.4 |#####
   2066.5 |########################################
  (5 below, 1 above range)

warm-clamp-minimum (n=40, range 617.3-752.6 ns)
    617.3 |########################################
    624.0 |
    630.8 |#####
    637.6 |#
    644.3 |
    651.1 |
    657.9 |
    664.6 |
    671.4 |
    678.1 |
    684.9 |
    691.7 |
    698.4 |
    705.2 |
    712.0 |
    718.7 |
    725.5 |
    732.3 |
    739.0 |
    745.8 |##########
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.91 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
