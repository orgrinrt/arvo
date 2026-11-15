# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.86)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-ragged (+1% speed for 1.6x steadier)

wide-rung-wordround-alias is fastest (2.05 us, CV 4.0%); wide-rung-ragged gives up 1.3% median for 1.6x lower variance (CV 2.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged's edge over baseline is significant but tiny (-19 ns, 0.92%)

wide-rung-ragged differs from baseline wide-rung-align16 by -19 ns (0.92%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 2052.5 ns median (-3.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 2052.5 ns, slowest 2162.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2201ns | 2186ns | 2105ns | 2170ns | 2393ns | base |
| wide-rung-ragged | 2144ns | 2145ns | 2079ns | 2137ns | 2228ns | -2.63% |
| wide-rung-ragged-overread | 2288ns | 2230ns | 2106ns | 2260ns | 2554ns | +3.93% |
| wide-rung-wordround | 2223ns | 2216ns | 2109ns | 2206ns | 2388ns | +0.98% |
| wide-rung-wordround-alias | 2159ns | 2118ns | 2104ns | 2127ns | 2307ns | -1.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2135ns | 2043ns | 2321ns | base | 0.959 |
| wide-rung-ragged | 2079ns | 2018ns | 2158ns | -2.65% | 0.985 |
| wide-rung-ragged-overread | 2221ns | 2045ns | 2479ns | +4.01% | 0.922 |
| wide-rung-wordround | 2156ns | 2045ns | 2319ns | +0.97% | 0.950 |
| wide-rung-wordround-alias | 2094ns | 2042ns | 2240ns | -1.92% | 0.978 |

## Performance model

- Peak throughput: **1.015 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.967 | 95.3% |
| wide-rung-ragged | 0.985 | 97.1% |
| wide-rung-ragged-overread | 0.947 | 93.3% |
| wide-rung-wordround | 0.953 | 93.9% |
| wide-rung-wordround-alias | 0.998 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2201ns | 2201ns | base |
| wide-rung-ragged | 2144ns | 2144ns | -2.63% |
| wide-rung-ragged-overread | 2288ns | 2288ns | +3.93% |
| wide-rung-wordround | 2223ns | 2223ns | +0.98% |
| wide-rung-wordround-alias | 2159ns | 2159ns | -1.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2117ns | base | --- | [2046, 2151] | --- | --- | --- | --- |
| wide-rung-ragged | 2078ns | no significant difference | [-42, +14]ns | [2052, 2091] | no | 0.6358 | 0.6358 | 0 |
| wide-rung-ragged-overread | 2163ns | +120.0ns (+5.7%) | [+46, +176]ns | [2089, 2330] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 2149ns | no significant difference | [-2, +110]ns | [2076, 2154] | no | 0.3077 | 0.1539 | 0 |
| wide-rung-wordround-alias | 2052ns | no significant difference | [-107, +9]ns | [2045, 2081] | no | 0.5728 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2125ns | +0.3% | +9.8% | +9.2% | -3.8% |
| 2 | 2152ns | -2.3% | +8.4% | +7.7% | -5.0% |
| 3 | 2150ns | -1.4% | +8.4% | +7.9% | -4.9% |
| 4 | 2152ns | -3.0% | +8.3% | +7.8% | -5.0% |
| 5 | 2152ns | -3.3% | +8.3% | +7.9% | -5.0% |
| 6 | 2152ns | -2.4% | +8.4% | +7.7% | -2.1% |
| 7 | 2151ns | +2.0% | +8.4% | +7.6% | -5.1% |
| 8 | 2151ns | +1.0% | +8.5% | +7.8% | -5.0% |
| 9 | 2152ns | -3.0% | +8.3% | +7.6% | -5.0% |
| 10 | 2150ns | -0.6% | +10.0% | +7.7% | -4.9% |
| 11 | 2042ns | +1.6% | +3.7% | +5.5% | -0.1% |
| 12 | 2042ns | -0.5% | +2.4% | +5.4% | +0.3% |
| 13 | 2044ns | -0.4% | +2.1% | +9.2% | +7.0% |
| 14 | 2044ns | -0.9% | +2.7% | +5.4% | +0.2% |
| 15 | 2044ns | -1.2% | +5.8% | +5.3% | +0.7% |
| 16 | 2042ns | -1.0% | +5.9% | +5.4% | +0.1% |
| 17 | 2045ns | -1.6% | +6.0% | +5.1% | +12.2% |
| 18 | 2046ns | +0.3% | +6.0% | +5.1% | +13.3% |
| 19 | 2043ns | +0.7% | +5.7% | +5.3% | +13.5% |
| 20 | 2042ns | +0.7% | +5.8% | +5.4% | +13.7% |
| 21 | 2046ns | +2.3% | +0.6% | -0.1% | +2.0% |
| 22 | 2043ns | +2.3% | +0.5% | +0.1% | +1.7% |
| 23 | 2110ns | +1.0% | -2.6% | -3.1% | -1.6% |
| 24 | 2047ns | +3.8% | +0.3% | -0.1% | +2.2% |
| 25 | 2046ns | +1.8% | +0.3% | -0.1% | +1.5% |
| 26 | 2045ns | +3.6% | +0.4% | +2.8% | +1.7% |
| 27 | 2046ns | +2.4% | +0.6% | +1.7% | +1.8% |
| 28 | 2047ns | +1.4% | +0.6% | -0.3% | +1.7% |
| 29 | 2045ns | +2.1% | +0.5% | +2.4% | +1.7% |
| 30 | 2135ns | +5.6% | -3.7% | -2.7% | -2.4% |
| 31 | 2322ns | -12.7% | -13.2% | -12.0% | -5.3% |
| 32 | 2325ns | -12.9% | -13.1% | -11.7% | -12.1% |
| 33 | 2318ns | -12.4% | -11.3% | -6.3% | -11.9% |
| 34 | 2322ns | -10.8% | +7.4% | -10.7% | -12.0% |
| 35 | 2322ns | -13.2% | +7.4% | -10.6% | -6.5% |
| 36 | 2316ns | -13.0% | +7.8% | -10.4% | -9.1% |
| 37 | 2321ns | -13.2% | +7.6% | -8.3% | -12.0% |
| 38 | 2322ns | -11.6% | +7.6% | -10.7% | -11.9% |
| 39 | 2267ns | -7.6% | +9.9% | -8.5% | -9.9% |
| 40 | 2048ns | +0.2% | +21.7% | +1.3% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.865 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.418 | moderate+ |
| wide-rung-ragged-overread | 0.840 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.856 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.573 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 22/40, lost 18/40
- **wide-rung-ragged-overread**: won 5/40, lost 35/40
- **wide-rung-wordround**: won 14/40, lost 25/40
- **wide-rung-wordround-alias**: won 22/40, lost 17/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.5ns | 2135.3ns | 0.1% |  |
| wide-rung-ragged | 2.5ns | 2078.7ns | 0.1% |  |
| wide-rung-ragged-overread | 2.7ns | 2220.9ns | 0.1% |  |
| wide-rung-wordround | 2.3ns | 2156.0ns | 0.1% |  |
| wide-rung-wordround-alias | 2.5ns | 2094.4ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2042.8-2320.9 ns)
   2042.8 |########################################
   2056.7 |
   2070.6 |
   2084.5 |
   2098.4 |##
   2112.3 |##
   2126.2 |##
   2140.1 |########################
   2154.0 |
   2167.9 |
   2181.8 |
   2195.7 |
   2209.7 |
   2223.6 |
   2237.5 |
   2251.4 |
   2265.3 |##
   2279.2 |
   2293.1 |
   2307.0 |#####
  (4 below, 6 above range)

wide-rung-ragged (n=40, range 2018.2-2158.3 ns)
   2018.2 |########################################
   2025.2 |##############################
   2032.2 |##########
   2039.2 |
   2046.2 |##############################
   2053.2 |####################
   2060.2 |
   2067.2 |##########
   2074.2 |##############################
   2081.3 |##############################
   2088.3 |########################################
   2095.3 |##############################
   2102.3 |
   2109.3 |
   2116.3 |####################
   2123.3 |##########
   2130.3 |##############################
   2137.3 |
   2144.3 |
   2151.3 |
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 2044.6-2478.9 ns)
   2044.6 |########################################
   2066.4 |###
   2088.1 |#######
   2109.8 |###
   2131.5 |
   2153.2 |#####################
   2174.9 |
   2196.6 |
   2218.3 |
   2240.1 |
   2261.8 |
   2283.5 |
   2305.2 |
   2326.9 |################################
   2348.6 |###
   2370.3 |
   2392.1 |
   2413.8 |
   2435.5 |
   2457.2 |
  (2 below, 7 above range)

wide-rung-wordround (n=40, range 2044.9-2318.7 ns)
   2044.9 |####################
   2058.6 |
   2072.3 |########################################
   2085.9 |#####
   2099.6 |#####
   2113.3 |
   2127.0 |#####
   2140.7 |###################################
   2154.4 |##########
   2168.1 |#####
   2181.8 |
   2195.5 |
   2209.2 |
   2222.9 |#####
   2236.5 |
   2250.2 |
   2263.9 |
   2277.6 |
   2291.3 |
   2305.0 |##############################
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 2042.3-2239.6 ns)
   2042.3 |########################################
   2052.2 |##
   2062.1 |
   2071.9 |############
   2081.8 |##########
   2091.6 |##
   2101.5 |#####
   2111.4 |
   2121.2 |
   2131.1 |
   2141.0 |
   2150.8 |
   2160.7 |
   2170.5 |##
   2180.4 |##
   2190.3 |##
   2200.1 |
   2210.0 |
   2219.9 |
   2229.7 |
  (4 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.57 (measurement drift or warm-up artifact)
