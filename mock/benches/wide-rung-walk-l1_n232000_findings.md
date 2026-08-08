# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-wordround (2.03 us) and wide-rung-wordround-alias (2.04 us) differ by 0.62%, inside the noise, even though the wider field spreads 6.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged-overread shows warm-up / thermal drift (autocorr +0.85)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged's edge over baseline is significant but tiny (39 ns, 1.79%)

wide-rung-ragged differs from baseline wide-rung-align16 by 39 ns (1.79%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 2032.1 ns median (-5.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 2032.1 ns, slowest 2166.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2227ns | 2217ns | 2065ns | 2219ns | 2411ns | base |
| wide-rung-ragged | 2301ns | 2235ns | 2082ns | 2283ns | 2570ns | +3.31% |
| wide-rung-ragged-overread | 2257ns | 2187ns | 2076ns | 2214ns | 2568ns | +1.36% |
| wide-rung-wordround | 2116ns | 2095ns | 2066ns | 2098ns | 2217ns | -4.99% |
| wide-rung-wordround-alias | 2181ns | 2108ns | 2090ns | 2142ns | 2391ns | -2.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2160ns | 2005ns | 2335ns | base | 0.948 |
| wide-rung-ragged | 2231ns | 2020ns | 2494ns | +3.28% | 0.918 |
| wide-rung-ragged-overread | 2190ns | 2015ns | 2492ns | +1.40% | 0.935 |
| wide-rung-wordround | 2052ns | 2005ns | 2151ns | -5.00% | 0.998 |
| wide-rung-wordround-alias | 2116ns | 2026ns | 2320ns | -2.02% | 0.968 |

## Performance model

- Peak throughput: **1.021 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.952 | 93.2% |
| wide-rung-ragged | 0.945 | 92.6% |
| wide-rung-ragged-overread | 0.966 | 94.6% |
| wide-rung-wordround | 1.008 | 98.7% |
| wide-rung-wordround-alias | 1.002 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2227ns | 2227ns | base |
| wide-rung-ragged | 2301ns | 2301ns | +3.31% |
| wide-rung-ragged-overread | 2257ns | 2257ns | +1.36% |
| wide-rung-wordround | 2116ns | 2116ns | -4.99% |
| wide-rung-wordround-alias | 2181ns | 2181ns | -2.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2151ns | base | --- | [2074, 2170] | --- | --- | --- | --- |
| wide-rung-ragged | 2166ns | +61.4ns (+2.9%) | [+15, +166]ns | [2150, 2324] | YES | 0.0257 | 0.0064 | 0 |
| wide-rung-ragged-overread | 2120ns | no significant difference | [-31, +53]ns | [2049, 2235] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround | 2032ns | -55.4ns (-2.6%) | [-149, -2]ns | [2009, 2050] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| wide-rung-wordround-alias | 2045ns | no significant difference | [-107, +3]ns | [2044, 2057] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2150ns | -5.9% | -6.2% | -6.8% | -4.9% |
| 2 | 2153ns | -6.4% | -6.3% | -7.0% | -5.0% |
| 3 | 2153ns | -5.6% | -3.4% | -6.9% | -5.0% |
| 4 | 2151ns | -2.3% | -6.3% | -0.1% | -5.0% |
| 5 | 2150ns | +8.3% | -6.1% | -0.2% | -5.0% |
| 6 | 2151ns | +8.1% | -6.3% | -0.3% | -5.0% |
| 7 | 2154ns | +7.5% | +0.1% | -0.2% | -5.3% |
| 8 | 2149ns | +8.1% | +8.5% | +0.1% | -5.0% |
| 9 | 2056ns | +13.3% | +13.4% | +4.6% | -0.7% |
| 10 | 2046ns | +13.8% | +13.9% | +5.0% | -0.1% |
| 11 | 2427ns | +3.1% | -12.6% | -17.2% | -4.5% |
| 12 | 2319ns | +7.4% | -8.5% | -13.1% | +0.0% |
| 13 | 2322ns | +8.2% | -9.4% | -13.4% | -0.0% |
| 14 | 2320ns | +7.6% | -7.7% | -12.5% | -0.1% |
| 15 | 2322ns | +7.2% | -10.1% | -11.7% | -0.1% |
| 16 | 2320ns | +7.1% | -7.5% | -11.9% | +0.1% |
| 17 | 2184ns | +13.1% | -3.0% | -6.1% | +6.2% |
| 18 | 2151ns | +15.5% | +6.2% | -4.9% | +7.7% |
| 19 | 2150ns | +15.9% | +1.7% | -4.7% | +8.0% |
| 20 | 2155ns | +14.7% | -3.0% | -5.0% | +7.6% |
| 21 | 2006ns | +0.8% | +0.8% | -0.0% | -0.1% |
| 22 | 2003ns | +0.7% | +0.8% | +0.2% | +0.2% |
| 23 | 2008ns | +1.3% | +0.4% | +0.0% | -0.0% |
| 24 | 2005ns | +0.8% | +0.4% | +1.4% | +0.9% |
| 25 | 2010ns | +0.4% | +0.3% | -0.2% | +1.9% |
| 26 | 2007ns | +0.3% | +0.3% | +0.1% | +1.8% |
| 27 | 2008ns | +1.5% | +0.5% | +0.0% | +1.8% |
| 28 | 2006ns | +2.1% | +0.6% | +1.7% | +1.8% |
| 29 | 2004ns | +2.4% | +0.7% | +0.1% | +2.7% |
| 30 | 2005ns | +12.0% | +3.5% | +0.1% | +6.8% |
| 31 | 2006ns | +8.0% | +24.1% | +3.6% | +1.9% |
| 32 | 2006ns | +7.8% | +24.3% | +8.4% | +1.8% |
| 33 | 2091ns | +4.3% | +19.1% | -0.5% | -2.2% |
| 34 | 2318ns | -6.5% | +7.5% | -12.7% | -6.4% |
| 35 | 2319ns | -6.7% | +7.4% | -9.9% | -11.3% |
| 36 | 2323ns | -7.0% | +7.2% | -13.6% | -11.7% |
| 37 | 2320ns | -7.5% | +7.5% | -13.5% | -11.9% |
| 38 | 2322ns | -3.8% | +7.3% | -13.5% | -12.0% |
| 39 | 2322ns | -6.9% | +3.0% | -11.0% | -11.9% |
| 40 | 2320ns | -7.2% | +0.6% | -13.5% | -11.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.790 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.845 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.852 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.606 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.803 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 11/40, lost 29/40
- **wide-rung-ragged-overread**: won 14/40, lost 26/40
- **wide-rung-wordround**: won 26/40, lost 8/40
- **wide-rung-wordround-alias**: won 20/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.4ns | 2159.8ns | 0.1% |  |
| wide-rung-ragged | 2.7ns | 2230.7ns | 0.1% |  |
| wide-rung-ragged-overread | 2.7ns | 2190.0ns | 0.1% |  |
| wide-rung-wordround | 2.2ns | 2051.9ns | 0.1% |  |
| wide-rung-wordround-alias | 2.5ns | 2116.1ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2005.2-2334.8 ns)
   2005.2 |#############################
   2021.7 |
   2038.2 |###
   2054.7 |###
   2071.2 |
   2087.6 |###
   2104.1 |
   2120.6 |
   2137.1 |################################
   2153.5 |#######
   2170.0 |###
   2186.5 |
   2203.0 |
   2219.5 |
   2235.9 |
   2252.4 |
   2268.9 |
   2285.4 |
   2301.8 |###
   2318.3 |########################################
  (4 below, 1 above range)

wide-rung-ragged (n=40, range 2020.1-2494.3 ns)
   2020.1 |########################################
   2043.8 |#############
   2067.5 |
   2091.2 |######
   2114.9 |
   2138.7 |####################
   2162.4 |########################################
   2186.1 |
   2209.8 |
   2233.5 |#############
   2257.2 |
   2280.9 |
   2304.6 |##########################
   2328.4 |#############
   2352.1 |
   2375.8 |
   2399.5 |
   2423.2 |
   2446.9 |######
   2470.6 |########################################
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 2015.3-2491.8 ns)
   2015.3 |########################################
   2039.1 |
   2062.9 |######
   2086.8 |##########
   2110.6 |##########
   2134.4 |##########
   2158.2 |
   2182.1 |###
   2205.9 |
   2229.7 |
   2253.6 |
   2277.4 |###
   2301.2 |
   2325.0 |#############
   2348.9 |
   2372.7 |###
   2396.5 |
   2420.3 |
   2444.2 |
   2468.0 |####################
  (2 below, 2 above range)

wide-rung-wordround (n=40, range 2005.2-2151.5 ns)
   2005.2 |########################################
   2012.6 |##
   2019.9 |##
   2027.2 |#####
   2034.5 |##
   2041.8 |########
   2049.1 |########
   2056.4 |
   2063.7 |##
   2071.0 |##
   2078.4 |##
   2085.7 |##
   2093.0 |
   2100.3 |
   2107.6 |
   2114.9 |
   2122.2 |
   2129.5 |
   2136.8 |
   2144.2 |####################
  (3 below, 1 above range)

wide-rung-wordround-alias (n=40, range 2025.8-2320.3 ns)
   2025.8 |####
   2040.5 |########################################
   2055.2 |######
   2070.0 |
   2084.7 |
   2099.4 |
   2114.1 |
   2128.9 |##
   2143.6 |
   2158.3 |##
   2173.1 |
   2187.8 |
   2202.5 |
   2217.2 |
   2232.0 |
   2246.7 |
   2261.4 |
   2276.1 |
   2290.9 |
   2305.6 |############
  (4 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.80 (measurement drift or warm-up artifact)
