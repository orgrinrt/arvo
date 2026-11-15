# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 19% faster than the next best (warm-clamp-accfit)

warm-clamp-min-lanes (1.83 us) leads warm-clamp-accfit (2.17 us) by 19%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 84% (significant)

warm-clamp-min-lanes is -8.68 us (84%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 5.7x slower than the field

warm-clamp-minimum (10.37 us) is 5.7x the fastest (1.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.73)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.73, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn} vs {warm-clamp-acc64, warm-clamp-minimum} (298% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-acc64, warm-clamp-minimum} with a 298% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.7x the fastest

Fastest warm-clamp-min-lanes (1.83 us) to slowest warm-clamp-minimum (10.37 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 1825.4 ns median (-82.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.68x (fastest 1825.4 ns, slowest 10368.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 10453ns | 10410ns | 10229ns | 10402ns | 10827ns | base |
| warm-clamp-accfit | 2250ns | 2236ns | 2193ns | 2246ns | 2318ns | -78.48% |
| warm-clamp-accfit-dyn | 2672ns | 2654ns | 2599ns | 2666ns | 2764ns | -74.44% |
| warm-clamp-head | 2303ns | 2298ns | 2288ns | 2298ns | 2332ns | -77.97% |
| warm-clamp-min-lanes | 1906ns | 1889ns | 1856ns | 1895ns | 1989ns | -81.76% |
| warm-clamp-minimum | 10525ns | 10438ns | 10382ns | 10467ns | 10843ns | +0.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 10381ns | 10165ns | 10738ns | base | 0.789 |
| warm-clamp-accfit | 2183ns | 2130ns | 2249ns | -78.97% | 3.752 |
| warm-clamp-accfit-dyn | 2612ns | 2542ns | 2702ns | -74.84% | 3.137 |
| warm-clamp-head | 2210ns | 2198ns | 2234ns | -78.71% | 3.707 |
| warm-clamp-min-lanes | 1841ns | 1792ns | 1918ns | -82.27% | 4.451 |
| warm-clamp-minimum | 10455ns | 10315ns | 10773ns | +0.71% | 0.784 |

## Performance model

- Peak throughput: **4.570 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 0.793 | 17.4% |
| warm-clamp-accfit | 3.777 | 82.6% |
| warm-clamp-accfit-dyn | 3.160 | 69.1% |
| warm-clamp-head | 3.714 | 81.3% |
| warm-clamp-min-lanes | 4.488 | 98.2% |
| warm-clamp-minimum | 0.790 | 17.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 10453ns | 10453ns | base |
| warm-clamp-accfit | 2250ns | 2250ns | -78.48% |
| warm-clamp-accfit-dyn | 2672ns | 2672ns | -74.44% |
| warm-clamp-head | 2303ns | 2303ns | -77.97% |
| warm-clamp-min-lanes | 1906ns | 1906ns | -81.76% |
| warm-clamp-minimum | 10525ns | 10525ns | +0.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 10330ns | base | --- | [10316, 10362] | --- | --- | --- | --- |
| warm-clamp-accfit | 2169ns | -8137.3ns (-78.8%) | [-8176, -8081]ns | [2162, 2185] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2592ns | -7720.1ns (-74.7%) | [-7753, -7656]ns | [2579, 2618] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2206ns | -8122.7ns (-78.6%) | [-8154, -8106]ns | [2202, 2210] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1825ns | -8495.0ns (-82.2%) | [-8518, -8466]ns | [1821, 1831] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 10369ns | no significant difference | [-23, +162]ns | [10336, 10396] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 10188ns | -79.0% | -74.3% | -78.4% | -82.4% | +1.9% |
| 2 | 10147ns | -79.0% | -75.0% | -78.3% | -82.4% | +1.6% |
| 3 | 10145ns | -79.1% | -75.1% | -78.3% | -82.4% | +4.2% |
| 4 | 10229ns | -78.8% | -75.3% | -78.4% | -82.5% | +4.5% |
| 5 | 10159ns | -79.1% | -75.1% | -78.3% | -82.4% | +5.2% |
| 6 | 10171ns | -79.1% | -75.0% | -78.3% | -82.1% | +7.4% |
| 7 | 10268ns | -79.3% | -75.2% | -78.5% | -82.4% | +3.6% |
| 8 | 10215ns | -79.1% | -74.4% | -78.4% | -82.3% | +1.4% |
| 9 | 10150ns | -79.0% | -74.7% | -78.3% | -82.3% | +1.6% |
| 10 | 10148ns | -78.9% | -74.6% | -78.2% | -82.3% | +1.6% |
| 11 | 10701ns | -79.8% | -75.8% | -79.4% | -83.0% | -3.4% |
| 12 | 10693ns | -79.8% | -75.6% | -78.9% | -83.0% | -2.5% |
| 13 | 10685ns | -79.6% | -76.0% | -79.4% | -82.4% | -3.4% |
| 14 | 10361ns | -78.6% | -75.2% | -78.7% | -82.5% | -0.3% |
| 15 | 10373ns | -79.1% | -75.2% | -78.8% | -82.4% | -0.6% |
| 16 | 10313ns | -79.0% | -74.9% | -78.6% | -82.3% | +0.1% |
| 17 | 10350ns | -79.0% | -74.7% | -77.4% | -82.1% | -0.3% |
| 18 | 10350ns | -78.8% | -75.0% | -78.7% | -82.3% | +0.4% |
| 19 | 10406ns | -79.2% | -75.3% | -78.8% | -82.4% | -0.3% |
| 20 | 10690ns | -79.8% | -75.9% | -79.4% | -83.0% | -3.3% |
| 21 | 10393ns | -79.2% | -75.1% | -78.7% | -81.8% | -0.7% |
| 22 | 10365ns | -79.2% | -74.9% | -78.7% | -81.8% | -0.5% |
| 23 | 10317ns | -78.3% | -75.1% | -78.6% | -81.7% | +0.0% |
| 24 | 10895ns | -80.0% | -75.7% | -79.7% | -82.6% | -4.9% |
| 25 | 10386ns | -79.0% | -74.2% | -78.7% | -81.8% | +2.0% |
| 26 | 10332ns | -79.1% | -74.1% | -78.6% | -81.7% | +6.6% |
| 27 | 10311ns | -79.0% | -73.9% | -78.6% | -79.5% | +0.7% |
| 28 | 10466ns | -79.3% | -74.2% | -78.9% | -82.0% | -0.9% |
| 29 | 10325ns | -78.8% | -74.1% | -78.6% | -81.7% | +0.1% |
| 30 | 10396ns | -79.0% | -74.3% | -78.7% | -82.2% | -0.7% |
| 31 | 10318ns | -78.1% | -75.1% | -78.7% | -82.3% | +0.7% |
| 32 | 10318ns | -78.2% | -74.9% | -78.7% | -82.2% | +3.6% |
| 33 | 10315ns | -78.2% | -73.5% | -78.7% | -82.3% | +3.7% |
| 34 | 10347ns | -78.4% | -74.2% | -78.7% | -82.4% | +3.4% |
| 35 | 10362ns | -78.4% | -75.0% | -78.8% | -82.4% | +4.0% |
| 36 | 10322ns | -78.3% | -73.9% | -78.7% | -82.3% | +2.2% |
| 37 | 10303ns | -78.2% | -73.2% | -78.7% | -82.2% | +0.3% |
| 38 | 10328ns | -78.3% | -74.3% | -78.7% | -82.4% | +0.7% |
| 39 | 10340ns | -78.3% | -74.1% | -78.8% | -82.4% | -0.2% |
| 40 | 11368ns | -79.9% | -77.2% | -80.7% | -84.0% | -8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.202 | moderate+ |
| warm-clamp-accfit | 0.733 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.622 | HIGH+ (drift/warm-up) |
| warm-clamp-head | -0.034 | ok |
| warm-clamp-min-lanes | 0.501 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.560 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 15/40, lost 23/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.2ns | 10381.2ns | 0.0% |  |
| warm-clamp-accfit | 2.5ns | 2183.4ns | 0.1% |  |
| warm-clamp-accfit-dyn | 3.0ns | 2611.7ns | 0.1% |  |
| warm-clamp-head | 3.0ns | 2209.8ns | 0.1% |  |
| warm-clamp-min-lanes | 2.7ns | 1840.5ns | 0.1% |  |
| warm-clamp-minimum | 3.2ns | 10455.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 10165.5-10737.9 ns)
  10165.5 |########
  10194.1 |####
  10222.7 |####
  10251.3 |####
  10279.9 |####
  10308.6 |########################################
  10337.2 |############################
  10365.8 |############
  10394.4 |########
  10423.1 |
  10451.7 |####
  10480.3 |
  10508.9 |
  10537.5 |
  10566.2 |
  10594.8 |
  10623.4 |
  10652.0 |
  10680.7 |################
  10709.3 |
  (5 below, 2 above range)

warm-clamp-accfit (n=40, range 2129.7-2248.9 ns)
   2129.7 |###########
   2135.7 |###########
   2141.6 |
   2147.6 |
   2153.6 |#####
   2159.5 |########################################
   2165.5 |############################
   2171.4 |#####
   2177.4 |#################
   2183.4 |
   2189.3 |###########
   2195.3 |
   2201.3 |
   2207.2 |
   2213.2 |#####
   2219.1 |
   2225.1 |
   2231.1 |#####
   2237.0 |##################################
   2243.0 |###########
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 2542.3-2701.8 ns)
   2542.3 |######
   2550.3 |
   2558.2 |######
   2566.2 |########################################
   2574.2 |#############
   2582.2 |##########################
   2590.1 |####################
   2598.1 |######
   2606.1 |#############
   2614.1 |#############
   2622.1 |
   2630.0 |
   2638.0 |
   2646.0 |#############
   2654.0 |
   2661.9 |######
   2669.9 |####################
   2677.9 |#############
   2685.9 |#############
   2693.8 |######
  (5 below, 2 above range)

warm-clamp-head (n=40, range 2197.8-2234.3 ns)
   2197.8 |########################################
   2199.6 |###############
   2201.4 |#########################
   2203.2 |#####
   2205.1 |##########
   2206.9 |#####
   2208.7 |##############################
   2210.6 |##############################
   2212.4 |###############
   2214.2 |
   2216.0 |
   2217.8 |#####
   2219.7 |
   2221.5 |
   2223.3 |
   2225.2 |
   2227.0 |
   2228.8 |
   2230.6 |
   2232.5 |
  (2 below, 2 above range)

warm-clamp-min-lanes (n=40, range 1792.4-1917.6 ns)
   1792.4 |#####
   1798.7 |
   1804.9 |###########
   1811.2 |###########
   1817.4 |########################################
   1823.7 |########################################
   1830.0 |###########
   1836.2 |#####
   1842.5 |
   1848.7 |#####
   1855.0 |#####
   1861.3 |
   1867.5 |
   1873.8 |
   1880.0 |###########
   1886.3 |############################
   1892.5 |###########
   1898.8 |
   1905.1 |
   1911.3 |
  (6 below, 1 above range)

warm-clamp-minimum (n=40, range 10315.2-10772.6 ns)
  10315.2 |########################################
  10338.0 |##########
  10360.9 |################
  10383.8 |##########
  10406.7 |###
  10429.5 |
  10452.4 |
  10475.3 |
  10498.2 |
  10521.0 |
  10543.9 |###
  10566.8 |###
  10589.6 |###
  10612.5 |###
  10635.4 |
  10658.3 |
  10681.1 |################
  10704.0 |
  10726.9 |
  10749.8 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.56 (measurement drift or warm-up artifact)
