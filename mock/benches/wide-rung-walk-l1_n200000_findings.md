# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (2.19 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround at 2.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.87)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-wordround vs stability leader wide-rung-ragged-overread (+4% speed for 1.0x steadier)

wide-rung-wordround is fastest (2.07 us, CV 3.3%); wide-rung-ragged-overread gives up 4.2% median for 1.0x lower variance (CV 3.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged's edge over baseline is significant but tiny (-14 ns, 0.62%)

wide-rung-ragged differs from baseline wide-rung-align16 by -14 ns (0.62%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 2068.4 ns median (-5.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.06x (fastest 2068.4 ns, slowest 2185.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2275ns | 2251ns | 2130ns | 2258ns | 2473ns | base |
| wide-rung-ragged | 2279ns | 2217ns | 2093ns | 2240ns | 2579ns | +0.14% |
| wide-rung-ragged-overread | 2209ns | 2222ns | 2102ns | 2215ns | 2298ns | -2.91% |
| wide-rung-wordround | 2162ns | 2133ns | 2082ns | 2156ns | 2257ns | -5.00% |
| wide-rung-wordround-alias | 2238ns | 2217ns | 2140ns | 2219ns | 2392ns | -1.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2208ns | 2069ns | 2397ns | base | 0.927 |
| wide-rung-ragged | 2209ns | 2024ns | 2503ns | +0.03% | 0.927 |
| wide-rung-ragged-overread | 2143ns | 2041ns | 2230ns | -2.95% | 0.956 |
| wide-rung-wordround | 2098ns | 2021ns | 2192ns | -4.99% | 0.976 |
| wide-rung-wordround-alias | 2171ns | 2075ns | 2320ns | -1.68% | 0.943 |

## Performance model

- Peak throughput: **1.013 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.937 | 92.5% |
| wide-rung-ragged | 0.952 | 93.9% |
| wide-rung-ragged-overread | 0.950 | 93.8% |
| wide-rung-wordround | 0.990 | 97.7% |
| wide-rung-wordround-alias | 0.952 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2275ns | 2275ns | base |
| wide-rung-ragged | 2279ns | 2279ns | +0.14% |
| wide-rung-ragged-overread | 2209ns | 2209ns | -2.91% |
| wide-rung-wordround | 2162ns | 2162ns | -5.00% |
| wide-rung-wordround-alias | 2238ns | 2238ns | -1.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2185ns | base | --- | [2114, 2257] | --- | --- | --- | --- |
| wide-rung-ragged | 2152ns | no significant difference | [-52, +16]ns | [2096, 2210] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-ragged-overread | 2155ns | no significant difference | [-85, +4]ns | [2108, 2185] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround | 2068ns | -70.8ns (-3.2%) | [-122, -50]ns | [2046, 2152] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 2152ns | -29.5ns (-1.4%) | [-47, -6]ns | [2081, 2190] | YES | 0.0332 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2088ns | -3.2% | +4.5% | -2.0% | -0.6% |
| 2 | 2116ns | -4.2% | +3.3% | -3.3% | -2.0% |
| 3 | 2082ns | -2.6% | +4.9% | -1.7% | +5.1% |
| 4 | 2095ns | -3.2% | +4.5% | -2.3% | -0.9% |
| 5 | 2047ns | -1.1% | +7.1% | -0.0% | +1.5% |
| 6 | 2068ns | -2.2% | +5.8% | -1.0% | +0.3% |
| 7 | 2079ns | -2.6% | +6.0% | +1.8% | -0.0% |
| 8 | 2080ns | -2.7% | +2.5% | +0.4% | -0.1% |
| 9 | 2070ns | -2.6% | +2.5% | +0.9% | +0.3% |
| 10 | 2052ns | +0.1% | +0.8% | -0.4% | +1.1% |
| 11 | 2368ns | +5.7% | -7.9% | -9.1% | -2.2% |
| 12 | 2395ns | +4.4% | -8.9% | -10.0% | -3.3% |
| 13 | 2353ns | +6.4% | -7.4% | -8.5% | -1.4% |
| 14 | 2358ns | +6.2% | -9.6% | -8.5% | -1.6% |
| 15 | 2451ns | +2.1% | -14.2% | -12.3% | -5.5% |
| 16 | 2387ns | +4.8% | -11.5% | -9.8% | -2.7% |
| 17 | 2410ns | +3.9% | -12.7% | -10.8% | -3.8% |
| 18 | 2413ns | +3.7% | -11.9% | -16.1% | -3.7% |
| 19 | 2392ns | +4.5% | -12.1% | -15.9% | -3.1% |
| 20 | 2280ns | +9.7% | -7.8% | -11.9% | +1.8% |
| 21 | 2261ns | +0.2% | -3.5% | -4.8% | -8.1% |
| 22 | 2283ns | -3.9% | -3.2% | -2.6% | -9.1% |
| 23 | 2182ns | +2.9% | +1.1% | -1.4% | -1.5% |
| 24 | 2188ns | +3.4% | +1.8% | +1.9% | +0.6% |
| 25 | 2209ns | -1.8% | +0.8% | -2.1% | -0.4% |
| 26 | 2253ns | -1.4% | -1.1% | -2.2% | -2.2% |
| 27 | 2314ns | -5.0% | -1.4% | -5.9% | -6.9% |
| 28 | 2247ns | -3.4% | -0.9% | -0.9% | -2.4% |
| 29 | 2217ns | -2.4% | -0.4% | -2.9% | -2.0% |
| 30 | 2292ns | -4.7% | -2.7% | -6.1% | -6.2% |
| 31 | 2093ns | -0.9% | -2.9% | -4.1% | -0.9% |
| 32 | 2104ns | +1.7% | -3.3% | -4.6% | -1.2% |
| 33 | 2072ns | +1.3% | -2.0% | -2.4% | +0.2% |
| 34 | 2147ns | -2.4% | -5.0% | -4.7% | +0.4% |
| 35 | 2236ns | -6.2% | -8.9% | -8.5% | -4.3% |
| 36 | 2112ns | -0.8% | -3.5% | -3.1% | -1.4% |
| 37 | 2141ns | -1.9% | -4.2% | -4.5% | +0.6% |
| 38 | 2182ns | -3.9% | -4.8% | -6.3% | -1.4% |
| 39 | 2115ns | -0.8% | -2.0% | -3.4% | +1.9% |
| 40 | 2099ns | +0.0% | +0.8% | -2.5% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.780 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.870 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.780 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.692 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.742 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 23/40, lost 16/40
- **wide-rung-ragged-overread**: won 26/40, lost 14/40
- **wide-rung-wordround**: won 35/40, lost 4/40
- **wide-rung-wordround-alias**: won 27/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.2ns | 2208.3ns | 0.1% |  |
| wide-rung-ragged | 2.3ns | 2209.1ns | 0.1% |  |
| wide-rung-ragged-overread | 2.5ns | 2143.1ns | 0.1% |  |
| wide-rung-wordround | 2.1ns | 2098.1ns | 0.1% |  |
| wide-rung-wordround-alias | 2.5ns | 2171.3ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2068.9-2396.8 ns)
   2068.9 |########################################
   2085.3 |################################
   2101.7 |################################
   2118.1 |
   2134.5 |################
   2150.9 |
   2167.3 |################
   2183.7 |########
   2200.1 |########
   2216.4 |########
   2232.8 |################
   2249.2 |################
   2265.6 |########
   2282.0 |################
   2298.4 |########
   2314.8 |
   2331.2 |
   2347.6 |################
   2364.0 |########
   2380.4 |########################
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 2023.8-2502.6 ns)
   2023.8 |#########################
   2047.8 |#####
   2071.7 |#####
   2095.6 |########################################
   2119.6 |#####
   2143.5 |#####
   2167.4 |###############
   2191.4 |##########
   2215.3 |#####
   2239.2 |##########
   2263.2 |#####
   2287.1 |
   2311.1 |
   2335.0 |
   2358.9 |
   2382.9 |
   2406.8 |
   2430.8 |
   2454.7 |
   2478.6 |##############################
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 2041.4-2229.8 ns)
   2041.4 |########
   2050.8 |
   2060.2 |########
   2069.6 |################
   2079.1 |
   2088.5 |
   2097.9 |################################
   2107.3 |################
   2116.7 |########
   2126.1 |########################
   2135.6 |
   2145.0 |
   2154.4 |
   2163.8 |
   2173.2 |########################################
   2182.7 |########################################
   2192.1 |
   2201.5 |################################
   2210.9 |
   2220.3 |################################
  (6 below, 2 above range)

wide-rung-wordround (n=40, range 2021.1-2192.1 ns)
   2021.1 |#######
   2029.6 |
   2038.2 |########################################
   2046.7 |##########
   2055.3 |
   2063.8 |
   2072.4 |
   2080.9 |###
   2089.5 |###
   2098.0 |
   2106.6 |
   2115.1 |###
   2123.7 |
   2132.2 |
   2140.8 |
   2149.3 |########################################
   2157.9 |###
   2166.4 |
   2175.0 |###
   2183.5 |
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 2075.2-2319.6 ns)
   2075.2 |########################################
   2087.4 |
   2099.6 |
   2111.9 |
   2124.1 |
   2136.3 |###
   2148.5 |##########################
   2160.8 |###
   2173.0 |
   2185.2 |######
   2197.4 |##########
   2209.6 |
   2221.9 |
   2234.1 |
   2246.3 |
   2258.5 |
   2270.8 |
   2283.0 |
   2295.2 |
   2307.4 |####################
  (3 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.74 (measurement drift or warm-up artifact)
