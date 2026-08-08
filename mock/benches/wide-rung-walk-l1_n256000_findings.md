# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread is fastest but the noisiest (CV 19.2%)

wide-rung-ragged-overread wins on median (2.06 us) yet has the highest variance (CV 19.2%), while wide-rung-wordround-alias is the steadiest (CV 2.0%, 2.10 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.80)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (99 ns) is smaller than the fastest variant's own run-to-run std-dev (396 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-ragged-overread vs stability leader wide-rung-wordround-alias (+2% speed for 9.8x steadier)

wide-rung-ragged-overread is fastest (2.06 us, CV 19.2%); wide-rung-wordround-alias gives up 2.0% median for 9.8x lower variance (CV 2.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.8% of the fastest

All 5 variants sit between 2.06 us and 2.16 us - a 4.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged's edge over baseline is significant but tiny (16 ns, 0.77%)

wide-rung-ragged differs from baseline wide-rung-align16 by 16 ns (0.77%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 2056.4 ns median (-2.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.05x (fastest 2056.4 ns, slowest 2155.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2195ns | 2170ns | 2118ns | 2178ns | 2325ns | base |
| wide-rung-ragged | 2277ns | 2224ns | 2106ns | 2237ns | 2567ns | +3.71% |
| wide-rung-ragged-overread | 2218ns | 2120ns | 2070ns | 2133ns | 2621ns | +1.03% |
| wide-rung-wordround | 2174ns | 2149ns | 2113ns | 2169ns | 2252ns | -0.95% |
| wide-rung-wordround-alias | 2171ns | 2162ns | 2117ns | 2167ns | 2233ns | -1.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2129ns | 2055ns | 2254ns | base | 0.962 |
| wide-rung-ragged | 2205ns | 2043ns | 2481ns | +3.60% | 0.929 |
| wide-rung-ragged-overread | 2147ns | 2009ns | 2524ns | +0.88% | 0.954 |
| wide-rung-wordround | 2108ns | 2051ns | 2179ns | -0.96% | 0.971 |
| wide-rung-wordround-alias | 2105ns | 2055ns | 2167ns | -1.13% | 0.973 |

## Performance model

- Peak throughput: **1.020 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.973 | 95.4% |
| wide-rung-ragged | 0.950 | 93.2% |
| wide-rung-ragged-overread | 0.996 | 97.7% |
| wide-rung-wordround | 0.983 | 96.4% |
| wide-rung-wordround-alias | 0.977 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2195ns | 2195ns | base |
| wide-rung-ragged | 2277ns | 2277ns | +3.71% |
| wide-rung-ragged-overread | 2218ns | 2218ns | +1.03% |
| wide-rung-wordround | 2174ns | 2174ns | -0.95% |
| wide-rung-wordround-alias | 2171ns | 2171ns | -1.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2105ns | base | --- | [2086, 2142] | --- | --- | --- | --- |
| wide-rung-ragged | 2155ns | no significant difference | [-10, +90]ns | [2079, 2162] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-ragged-overread | 2056ns | -38.9ns (-1.8%) | [-50, -27]ns | [2047, 2087] | YES | 0.0089 | 0.0022 | 0 |
| wide-rung-wordround | 2084ns | no significant difference | [-33, +8]ns | [2060, 2155] | no | 0.8592 | 0.4296 | 0 |
| wide-rung-wordround-alias | 2097ns | no significant difference | [-39, +8]ns | [2086, 2122] | no | 1.0000 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2137ns | -2.0% | -6.2% | +0.8% | -3.5% |
| 2 | 2052ns | +0.2% | -1.8% | +5.1% | +0.1% |
| 3 | 2139ns | -2.4% | -6.2% | +1.3% | -3.6% |
| 4 | 2054ns | +0.0% | -2.3% | +5.2% | +0.2% |
| 5 | 2136ns | -3.9% | -5.9% | +1.1% | -2.5% |
| 6 | 2055ns | -0.8% | -2.8% | +5.2% | +2.1% |
| 7 | 2135ns | -4.2% | -5.7% | +0.1% | -2.1% |
| 8 | 2054ns | -0.6% | -1.9% | +4.2% | -0.2% |
| 9 | 2060ns | +4.3% | -1.8% | +5.3% | -0.0% |
| 10 | 2057ns | -0.6% | +1.8% | +5.9% | +2.9% |
| 11 | 2091ns | +18.3% | -2.5% | -1.4% | +1.4% |
| 12 | 2098ns | +18.4% | -2.5% | -1.8% | +0.8% |
| 13 | 2178ns | +13.8% | -5.9% | -5.7% | -5.8% |
| 14 | 2114ns | +17.3% | -1.2% | -2.8% | -2.7% |
| 15 | 2161ns | +15.0% | -3.5% | -5.0% | -4.7% |
| 16 | 2096ns | +18.1% | -2.0% | -1.7% | -2.0% |
| 17 | 2102ns | +17.6% | -2.3% | -2.5% | -1.8% |
| 18 | 2098ns | +18.6% | -1.5% | -1.7% | -2.0% |
| 19 | 2175ns | +14.5% | -1.1% | -4.9% | -5.1% |
| 20 | 2188ns | +12.1% | -1.4% | -6.0% | -2.9% |
| 21 | 2059ns | +4.9% | -2.0% | -0.2% | +3.1% |
| 22 | 2058ns | +4.5% | -1.3% | -0.4% | +2.0% |
| 23 | 2050ns | +5.1% | +1.6% | +0.4% | +2.6% |
| 24 | 2059ns | +0.7% | +1.4% | -0.1% | +3.5% |
| 25 | 2058ns | -0.3% | +0.4% | +0.6% | +5.8% |
| 26 | 2058ns | -0.2% | -0.3% | +0.2% | +1.8% |
| 27 | 2062ns | -0.9% | -0.2% | -0.4% | +1.5% |
| 28 | 2109ns | -2.5% | -3.0% | -3.2% | -0.7% |
| 29 | 2088ns | -2.1% | -2.0% | -1.5% | +0.4% |
| 30 | 2083ns | -2.2% | -1.5% | -1.2% | +0.3% |
| 31 | 2162ns | -0.2% | -3.2% | +0.8% | -0.1% |
| 32 | 2170ns | -0.6% | -3.0% | -2.8% | -0.3% |
| 33 | 2163ns | -0.2% | +0.2% | +0.8% | +0.3% |
| 34 | 2166ns | +14.2% | +1.0% | +0.3% | -0.7% |
| 35 | 2660ns | -17.2% | -18.0% | -17.7% | -19.1% |
| 36 | 2320ns | -7.1% | -6.3% | -6.3% | -7.1% |
| 37 | 2169ns | -0.4% | +0.4% | +0.7% | -1.8% |
| 38 | 2170ns | -0.3% | +0.4% | -0.2% | +0.0% |
| 39 | 2158ns | +0.5% | +19.6% | -0.6% | +0.8% |
| 40 | 2146ns | +0.4% | +111.7% | -2.2% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.420 | moderate+ |
| wide-rung-ragged | 0.789 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.217 | moderate+ |
| wide-rung-wordround | 0.795 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.723 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 20/40, lost 19/40
- **wide-rung-ragged-overread**: won 30/40, lost 10/40
- **wide-rung-wordround**: won 23/40, lost 16/40
- **wide-rung-wordround-alias**: won 19/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.3ns | 2128.7ns | 0.1% |  |
| wide-rung-ragged | 2.2ns | 2205.3ns | 0.1% |  |
| wide-rung-ragged-overread | 2.5ns | 2147.5ns | 0.1% |  |
| wide-rung-wordround | 2.6ns | 2108.2ns | 0.1% |  |
| wide-rung-wordround-alias | 2.4ns | 2104.7ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2054.7-2253.6 ns)
   2054.7 |########################################
   2064.6 |
   2074.6 |#####
   2084.5 |##########
   2094.5 |####################
   2104.4 |##########
   2114.4 |
   2124.3 |
   2134.3 |####################
   2144.2 |#####
   2154.2 |####################
   2164.1 |####################
   2174.1 |##########
   2184.0 |#####
   2194.0 |
   2203.9 |
   2213.9 |
   2223.8 |
   2233.8 |
   2243.7 |
  (5 below, 2 above range)

wide-rung-ragged (n=40, range 2043.4-2481.2 ns)
   2043.4 |########################################
   2065.3 |########
   2087.2 |####
   2109.1 |
   2131.0 |########
   2152.9 |########################################
   2174.8 |
   2196.6 |####
   2218.5 |
   2240.4 |
   2262.3 |
   2284.2 |
   2306.1 |
   2328.0 |
   2349.9 |
   2371.8 |
   2393.7 |
   2415.5 |
   2437.4 |####
   2459.3 |########################
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 2008.5-2523.7 ns)
   2008.5 |############################
   2034.3 |########################################
   2060.1 |################
   2085.8 |####################
   2111.6 |
   2137.3 |########
   2163.1 |########################
   2188.8 |
   2214.6 |
   2240.3 |
   2266.1 |
   2291.9 |
   2317.6 |
   2343.4 |
   2369.1 |
   2394.9 |
   2420.6 |
   2446.4 |
   2472.2 |
   2497.9 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 2051.1-2179.0 ns)
   2051.1 |########################################
   2057.5 |###################################
   2063.9 |##########
   2070.3 |
   2076.7 |
   2083.1 |
   2089.5 |
   2095.8 |#####
   2102.2 |#####
   2108.6 |
   2115.0 |
   2121.4 |
   2127.8 |
   2134.2 |##########
   2140.6 |#####
   2147.0 |#####
   2153.4 |##########
   2159.8 |###############
   2166.2 |##########
   2172.6 |###############
  (3 below, 4 above range)

wide-rung-wordround-alias (n=40, range 2055.0-2166.7 ns)
   2055.0 |########################################
   2060.5 |##########################
   2066.1 |
   2071.7 |
   2077.3 |
   2082.9 |######
   2088.5 |####################
   2094.0 |#################################
   2099.6 |######
   2105.2 |
   2110.8 |######
   2116.4 |#############
   2122.0 |#############
   2127.6 |#############
   2133.1 |
   2138.7 |
   2144.3 |
   2149.9 |####################
   2155.5 |######
   2161.1 |#############
  (3 below, 4 above range)

```

## Diagnostics

- **wide-rung-ragged**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.72 (measurement drift or warm-up artifact)
