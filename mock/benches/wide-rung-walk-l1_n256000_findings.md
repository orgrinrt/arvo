# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged, wide-rung-ragged-overread) are a dead heat (<1%)

wide-rung-ragged (2.07 us) and wide-rung-ragged-overread (2.08 us) differ by 0.70%, inside the noise, even though the wider field spreads 9.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.86)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-ragged vs stability leader wide-rung-wordround-alias (+1% speed for 1.6x steadier)

wide-rung-ragged is fastest (2.07 us, CV 3.1%); wide-rung-wordround-alias gives up 0.9% median for 1.6x lower variance (CV 2.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged's edge over baseline is significant but tiny (-4 ns, 0.21%)

wide-rung-ragged differs from baseline wide-rung-align16 by -4 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 2065.6 ns median (-3.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 2065.6 ns, slowest 2253.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2209ns | 2195ns | 2117ns | 2186ns | 2372ns | base |
| wide-rung-ragged | 2165ns | 2129ns | 2110ns | 2149ns | 2270ns | -1.99% |
| wide-rung-ragged-overread | 2161ns | 2148ns | 2101ns | 2150ns | 2251ns | -2.20% |
| wide-rung-wordround | 2295ns | 2323ns | 2135ns | 2309ns | 2413ns | +3.88% |
| wide-rung-wordround-alias | 2161ns | 2149ns | 2119ns | 2150ns | 2238ns | -2.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2145ns | 2056ns | 2303ns | base | 0.955 |
| wide-rung-ragged | 2099ns | 2047ns | 2196ns | -2.13% | 0.976 |
| wide-rung-ragged-overread | 2095ns | 2037ns | 2182ns | -2.36% | 0.978 |
| wide-rung-wordround | 2226ns | 2072ns | 2342ns | +3.78% | 0.920 |
| wide-rung-wordround-alias | 2098ns | 2058ns | 2171ns | -2.20% | 0.976 |

## Performance model

- Peak throughput: **1.005 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.961 | 95.6% |
| wide-rung-ragged | 0.991 | 98.6% |
| wide-rung-ragged-overread | 0.985 | 97.9% |
| wide-rung-wordround | 0.909 | 90.4% |
| wide-rung-wordround-alias | 0.982 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2209ns | 2209ns | base |
| wide-rung-ragged | 2165ns | 2165ns | -1.99% |
| wide-rung-ragged-overread | 2161ns | 2161ns | -2.20% |
| wide-rung-wordround | 2295ns | 2295ns | +3.88% |
| wide-rung-wordround-alias | 2161ns | 2161ns | -2.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2132ns | base | --- | [2066, 2171] | --- | --- | --- | --- |
| wide-rung-ragged | 2066ns | -15.0ns (-0.7%) | [-95, -5]ns | [2056, 2118] | YES | 0.0068 | 0.0034 | 1 |
| wide-rung-ragged-overread | 2080ns | no significant difference | [-118, +0]ns | [2054, 2111] | no | 0.0807 | 0.0807 | 0 |
| wide-rung-wordround | 2254ns | +31.9ns (+1.5%) | [+22, +102]ns | [2149, 2333] | YES | 0.0068 | 0.0022 | 0 |
| wide-rung-wordround-alias | 2085ns | -38.5ns (-1.8%) | [-86, -1]ns | [2076, 2090] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2171ns | -5.7% | -5.8% | -5.7% | -3.3% |
| 2 | 2177ns | -5.9% | -5.5% | -5.9% | -4.1% |
| 3 | 2174ns | -5.8% | -5.9% | -6.1% | -4.1% |
| 4 | 2179ns | -5.6% | -2.2% | -1.3% | -4.1% |
| 5 | 2174ns | -5.6% | -5.6% | -5.6% | +1.1% |
| 6 | 2170ns | -6.1% | -6.3% | -1.4% | -3.7% |
| 7 | 2170ns | -5.8% | -3.8% | -0.8% | -4.6% |
| 8 | 2171ns | -3.2% | -5.7% | -0.1% | -6.0% |
| 9 | 2175ns | +7.0% | -5.9% | -0.1% | -5.0% |
| 10 | 2175ns | +1.8% | -6.9% | -0.5% | -5.2% |
| 11 | 2071ns | -0.7% | +6.3% | +1.6% | -0.6% |
| 12 | 2070ns | -0.7% | +0.5% | +3.1% | -0.3% |
| 13 | 2066ns | -0.5% | +2.5% | +4.0% | +0.0% |
| 14 | 2065ns | -0.6% | +0.8% | +5.9% | -0.1% |
| 15 | 2110ns | -2.2% | -0.5% | +1.3% | -1.5% |
| 16 | 2105ns | -2.5% | +1.1% | +0.3% | -2.2% |
| 17 | 2061ns | -0.2% | +1.9% | +1.9% | -0.0% |
| 18 | 2064ns | -0.6% | +2.5% | +1.3% | -0.2% |
| 19 | 2060ns | -0.1% | +0.6% | +1.8% | +0.5% |
| 20 | 2067ns | -0.7% | +0.8% | +1.5% | -0.2% |
| 21 | 2318ns | -7.1% | -6.7% | +0.9% | -10.2% |
| 22 | 2320ns | -6.8% | -7.1% | +1.0% | -10.0% |
| 23 | 2329ns | -8.1% | -7.2% | +0.6% | -9.9% |
| 24 | 2315ns | -7.0% | -6.8% | +0.9% | -10.1% |
| 25 | 2332ns | -7.5% | -7.4% | +0.4% | -10.7% |
| 26 | 2309ns | -6.9% | -3.8% | +1.5% | -9.6% |
| 27 | 2319ns | -7.2% | -10.6% | +1.0% | -10.3% |
| 28 | 2159ns | -0.0% | -5.4% | +7.9% | -3.9% |
| 29 | 2157ns | +0.1% | -5.4% | +8.5% | -2.9% |
| 30 | 2153ns | -0.8% | -5.2% | +8.5% | -3.2% |
| 31 | 2058ns | +0.0% | -0.7% | +12.8% | +1.5% |
| 32 | 2111ns | -2.6% | -2.6% | +10.0% | +0.5% |
| 33 | 2056ns | +0.6% | -0.9% | +13.6% | +5.7% |
| 34 | 2058ns | -0.2% | +0.8% | +13.4% | +5.6% |
| 35 | 2058ns | +0.8% | +4.8% | +13.4% | +5.1% |
| 36 | 2054ns | +9.1% | +3.7% | +13.6% | +4.9% |
| 37 | 2052ns | +1.4% | +9.1% | +13.6% | +5.3% |
| 38 | 2057ns | +3.7% | +2.4% | +13.5% | +5.5% |
| 39 | 2060ns | +1.3% | -0.7% | +13.7% | +5.0% |
| 40 | 2057ns | +1.0% | -1.2% | +12.9% | +5.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.816 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.466 | moderate+ |
| wide-rung-ragged-overread | 0.379 | moderate+ |
| wide-rung-wordround | 0.864 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.734 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 28/40, lost 9/40
- **wide-rung-ragged-overread**: won 26/40, lost 14/40
- **wide-rung-wordround**: won 9/40, lost 30/40
- **wide-rung-wordround-alias**: won 25/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.4ns | 2145.1ns | 0.1% |  |
| wide-rung-ragged | 2.6ns | 2099.4ns | 0.1% |  |
| wide-rung-ragged-overread | 2.4ns | 2094.6ns | 0.1% |  |
| wide-rung-wordround | 2.5ns | 2226.1ns | 0.1% |  |
| wide-rung-wordround-alias | 2.3ns | 2098.0ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2056.2-2302.6 ns)
   2056.2 |########################################
   2068.5 |######
   2080.8 |
   2093.1 |###
   2105.4 |######
   2117.8 |
   2130.1 |
   2142.4 |###
   2154.7 |######
   2167.0 |##############################
   2179.4 |
   2191.7 |
   2204.0 |
   2216.3 |
   2228.6 |
   2241.0 |
   2253.3 |
   2265.6 |
   2277.9 |
   2290.2 |
  (2 below, 7 above range)

wide-rung-ragged (n=40, range 2047.3-2196.4 ns)
   2047.3 |########################################
   2054.7 |########################################
   2062.2 |##########
   2069.6 |#####
   2077.1 |##########
   2084.6 |#####
   2092.0 |
   2099.5 |#####
   2106.9 |
   2114.4 |
   2121.8 |
   2129.3 |##########
   2136.7 |#####
   2144.2 |#####
   2151.6 |##############################
   2159.1 |#####
   2166.5 |
   2174.0 |
   2181.4 |
   2188.9 |
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 2037.0-2182.4 ns)
   2037.0 |########################################
   2044.3 |#################################
   2051.6 |#############
   2058.8 |
   2066.1 |####################
   2073.4 |#############
   2080.6 |#############
   2087.9 |
   2095.2 |#############
   2102.4 |######
   2109.7 |######
   2117.0 |######
   2124.2 |####################
   2131.5 |
   2138.8 |
   2146.0 |
   2153.3 |####################
   2160.6 |####################
   2167.8 |
   2175.1 |
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 2072.3-2341.9 ns)
   2072.3 |
   2085.8 |##########
   2099.2 |##########
   2112.7 |
   2126.2 |#######
   2139.7 |##############
   2153.2 |###
   2166.6 |#######
   2180.1 |###
   2193.6 |
   2207.1 |
   2220.6 |
   2234.0 |
   2247.5 |
   2261.0 |
   2274.5 |
   2288.0 |
   2301.4 |
   2314.9 |##############
   2328.4 |########################################
  (4 below, 5 above range)

wide-rung-wordround-alias (n=40, range 2058.3-2171.1 ns)
   2058.3 |########################################
   2064.0 |#################
   2069.6 |#################
   2075.3 |#################
   2080.9 |######################
   2086.5 |##################################
   2092.2 |#####
   2097.8 |###########
   2103.4 |
   2109.1 |
   2114.7 |
   2120.4 |#####
   2126.0 |
   2131.6 |
   2137.3 |
   2142.9 |
   2148.5 |
   2154.2 |#####
   2159.8 |#################
   2165.5 |###########
  (1 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.73 (measurement drift or warm-up artifact)
