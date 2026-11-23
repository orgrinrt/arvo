# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-pipe4 dominates: 25% faster than the next best (bitpack-wide-d16-padal)

bitpack-wide-pipe4 (1.82 ms) leads bitpack-wide-d16-padal (2.27 ms) by 25%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 beats baseline by 20% (significant)

bitpack-wide-pipe4 is -462.12 us (20%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {bitpack-wide-pipe4} vs {bitpack-wide-d16-padal, bitpack-wide-d16, bitpack-wide-d16-control} (25% apart)

The field splits into a fast tier {bitpack-wide-pipe4} and a slow tier {bitpack-wide-d16-padal, bitpack-wide-d16, bitpack-wide-d16-control} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 1815398.9 ns median (-20.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.26x (fastest 1815398.9 ns, slowest 2283108.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112815569 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 2292649ns | 2273971ns | 2175638ns | 2274937ns | 2462796ns | base |
| bitpack-wide-d16-control | 2478435ns | 2284200ns | 2263555ns | 2442265ns | 2801828ns | +8.10% |
| bitpack-wide-d16-padal | 2311149ns | 2270801ns | 2051778ns | 2273754ns | 2682706ns | +0.81% |
| bitpack-wide-pipe4 | 1776271ns | 1816427ns | 1598613ns | 1811900ns | 1847044ns | -22.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 2291512ns | 2174530ns | 2461591ns | base | 14.643 |
| bitpack-wide-d16-control | 2477340ns | 2262455ns | 2800712ns | +8.11% | 13.545 |
| bitpack-wide-d16-padal | 2310009ns | 2050665ns | 2681580ns | +0.81% | 14.526 |
| bitpack-wide-pipe4 | 1775186ns | 1597368ns | 1846021ns | -22.53% | 18.902 |

## Performance model

- Peak throughput: **21.006 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 14.763 | 70.3% |
| bitpack-wide-d16-control | 14.697 | 70.0% |
| bitpack-wide-d16-padal | 14.783 | 70.4% |
| bitpack-wide-pipe4 | 18.483 | 88.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 2292649ns | 2292649ns | base |
| bitpack-wide-d16-control | 2478435ns | 2478435ns | +8.10% |
| bitpack-wide-d16-padal | 2311149ns | 2311149ns | +0.81% |
| bitpack-wide-pipe4 | 1776271ns | 1776271ns | -22.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 2272938ns | base | --- | [2269146, 2279737] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 2283109ns | +59576.7ns (+2.6%) | [+7861, +286295]ns | [2270958, 2747014] | YES | 0.0003 | 0.0002 | 0 |
| bitpack-wide-d16-padal | 2269747ns | no significant difference | [-70604, +17162]ns | [2220326, 2294200] | no | 0.6358 | 0.6358 | 0 |
| bitpack-wide-pipe4 | 1815399ns | -462122.7ns (-20.3%) | [-483244, -452904]ns | [1812538, 1817427] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 2269146ns | -0.0% | +16.5% | -20.1% |
| 1 | 2 | 2275124ns | +22.9% | +0.7% | -29.8% |
| 1 | 3 | 2272938ns | +0.3% | -9.6% | -20.0% |
| 1 | 4 | 2271993ns | +1.5% | -2.3% | -20.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.396 | moderate- |
| bitpack-wide-d16-control | -0.470 | moderate- |
| bitpack-wide-d16-padal | 0.105 | ok |
| bitpack-wide-pipe4 | -0.410 | moderate- |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 0/4, lost 3/4
- **bitpack-wide-d16-padal**: won 2/4, lost 2/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 64.7ns | 2291511.6ns | 0.0% |  |
| bitpack-wide-d16-control | 79.4ns | 2477340.1ns | 0.0% |  |
| bitpack-wide-d16-padal | 82.1ns | 2310008.8ns | 0.0% |  |
| bitpack-wide-pipe4 | 42.2ns | 1775185.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 2174529.9-2461590.6 ns)
  2174529.9 |
  2188883.0 |
  2203236.0 |
  2217589.0 |
  2231942.1 |
  2246295.1 |################
  2260648.1 |########################################
  2275001.2 |########################
  2289354.2 |##
  2303707.2 |##
  2318060.3 |
  2332413.3 |
  2346766.3 |
  2361119.4 |
  2375472.4 |
  2389825.4 |
  2404178.5 |
  2418531.5 |
  2432884.5 |
  2447237.6 |
  (3 below, 5 above range)

bitpack-wide-d16-control (n=40, range 2262455.3-2800711.9 ns)
  2262455.3 |########################################
  2289368.2 |####
  2316281.0 |
  2343193.8 |##
  2370106.6 |
  2397019.5 |
  2423932.3 |
  2450845.1 |
  2477758.0 |
  2504670.8 |
  2531583.6 |
  2558496.4 |
  2585409.3 |
  2612322.1 |
  2639234.9 |##
  2666147.8 |
  2693060.6 |##
  2719973.4 |
  2746886.3 |
  2773799.1 |######################
  (3 below, 4 above range)

bitpack-wide-d16-padal (n=40, range 2050664.6-2681579.8 ns)
  2050664.6 |############
  2082210.4 |####
  2113756.2 |####
  2145301.9 |
  2176847.7 |
  2208393.4 |########################################
  2239939.2 |####
  2271485.0 |################################
  2303030.7 |####
  2334576.5 |
  2366122.2 |####
  2397668.0 |####
  2429213.8 |
  2460759.5 |
  2492305.3 |
  2523851.0 |####
  2555396.8 |
  2586942.6 |
  2618488.3 |################
  2650034.1 |####
  (4 below, 3 above range)

bitpack-wide-pipe4 (n=40, range 1597368.1-1846020.9 ns)
  1597368.1 |########
  1609800.8 |
  1622233.4 |
  1634666.0 |
  1647098.7 |
  1659531.3 |
  1671964.0 |
  1684396.6 |
  1696829.2 |
  1709261.9 |#
  1721694.5 |
  1734127.1 |
  1746559.8 |
  1758992.4 |
  1771425.1 |
  1783857.7 |
  1796290.3 |#
  1808723.0 |########################################
  1821155.6 |####
  1833588.2 |#
  (3 below, 1 above range)

```
