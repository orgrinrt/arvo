# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (2.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 2.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-wordround-alias, wide-rung-ragged-overread) are a dead heat (<1%)

wide-rung-wordround-alias (2.08 us) and wide-rung-ragged-overread (2.08 us) differ by 0.17%, inside the noise, even though the wider field spreads 10.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.87)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-ragged-overread (+0% speed for 1.8x steadier)

wide-rung-wordround-alias is fastest (2.08 us, CV 6.1%); wide-rung-ragged-overread gives up 0.2% median for 1.8x lower variance (CV 3.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-ragged's edge over baseline is significant but tiny (-45 ns, 1.96%)

wide-rung-ragged differs from baseline wide-rung-align16 by -45 ns (1.96%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 2076.9 ns median (-9.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.10x (fastest 2076.9 ns, slowest 2293.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2367ns | 2362ns | 2247ns | 2368ns | 2487ns | base |
| wide-rung-ragged | 2294ns | 2240ns | 2181ns | 2284ns | 2436ns | -3.11% |
| wide-rung-ragged-overread | 2174ns | 2144ns | 2097ns | 2167ns | 2269ns | -8.18% |
| wide-rung-wordround | 2279ns | 2209ns | 2068ns | 2258ns | 2555ns | -3.71% |
| wide-rung-wordround-alias | 2208ns | 2142ns | 2105ns | 2170ns | 2429ns | -6.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 2297ns | 2182ns | 2414ns | base | 0.892 |
| wide-rung-ragged | 2226ns | 2118ns | 2362ns | -3.09% | 0.920 |
| wide-rung-ragged-overread | 2109ns | 2034ns | 2199ns | -8.20% | 0.971 |
| wide-rung-wordround | 2211ns | 2005ns | 2480ns | -3.76% | 0.926 |
| wide-rung-wordround-alias | 2142ns | 2043ns | 2356ns | -6.75% | 0.956 |

## Performance model

- Peak throughput: **1.021 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.893 | 87.4% |
| wide-rung-ragged | 0.942 | 92.2% |
| wide-rung-ragged-overread | 0.984 | 96.4% |
| wide-rung-wordround | 0.956 | 93.6% |
| wide-rung-wordround-alias | 0.986 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2367ns | 2367ns | base |
| wide-rung-ragged | 2294ns | 2294ns | -3.11% |
| wide-rung-ragged-overread | 2174ns | 2174ns | -8.18% |
| wide-rung-wordround | 2279ns | 2279ns | -3.71% |
| wide-rung-wordround-alias | 2208ns | 2208ns | -6.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2294ns | base | --- | [2229, 2366] | --- | --- | --- | --- |
| wide-rung-ragged | 2174ns | -53.6ns (-2.3%) | [-141, -27]ns | [2172, 2258] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-ragged-overread | 2080ns | -217.2ns (-9.5%) | [-266, -177]ns | [2043, 2171] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 2143ns | no significant difference | [-138, +72]ns | [2075, 2319] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround-alias | 2077ns | -203.2ns (-8.9%) | [-314, -115]ns | [2045, 2126] | YES | 0.0030 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 2202ns | -1.3% | -7.8% | +5.1% | -5.8% |
| 2 | 2223ns | -2.4% | -8.5% | +4.3% | -6.6% |
| 3 | 2189ns | -0.9% | -0.9% | +6.0% | -5.0% |
| 4 | 2194ns | -1.1% | -7.2% | +5.7% | -5.4% |
| 5 | 2222ns | -2.3% | -8.5% | +4.3% | -1.8% |
| 6 | 2229ns | -2.4% | -8.7% | +5.7% | -5.0% |
| 7 | 2255ns | -8.0% | -9.6% | +1.3% | -4.7% |
| 8 | 2255ns | -8.5% | -9.7% | -4.7% | -5.3% |
| 9 | 2242ns | -8.0% | -9.4% | -3.9% | -7.1% |
| 10 | 2219ns | -6.9% | -8.2% | -3.1% | -7.8% |
| 11 | 2165ns | +8.2% | +0.7% | -2.8% | +7.1% |
| 12 | 2182ns | +7.8% | +0.6% | -3.3% | +12.8% |
| 13 | 2159ns | +8.4% | +1.4% | -4.0% | +7.6% |
| 14 | 2172ns | +5.6% | +0.7% | -4.5% | +11.7% |
| 15 | 2197ns | -1.0% | -0.5% | -5.6% | +5.8% |
| 16 | 2208ns | -1.5% | -0.8% | -5.9% | +5.3% |
| 17 | 2241ns | -3.1% | -2.2% | -7.5% | +3.9% |
| 18 | 2229ns | -0.4% | -1.8% | -6.6% | +4.4% |
| 19 | 2234ns | -2.9% | -1.8% | -8.3% | +4.1% |
| 20 | 2197ns | -1.2% | +0.3% | -6.7% | +6.0% |
| 21 | 2339ns | +4.3% | -12.9% | +5.9% | -9.9% |
| 22 | 2366ns | -1.1% | -11.4% | +4.8% | -12.2% |
| 23 | 2333ns | +0.2% | -12.5% | +6.1% | -11.0% |
| 24 | 2470ns | -5.2% | -17.3% | +0.4% | -13.0% |
| 25 | 2365ns | -1.1% | -13.7% | +4.9% | -11.8% |
| 26 | 2380ns | -1.7% | -14.2% | +4.3% | -13.3% |
| 27 | 2413ns | -3.0% | -15.3% | +2.7% | -15.3% |
| 28 | 2397ns | -2.3% | -11.0% | +3.4% | -14.8% |
| 29 | 2398ns | +0.2% | -13.3% | +3.2% | -14.9% |
| 30 | 2385ns | -1.9% | -12.4% | +3.9% | -14.3% |
| 31 | 2360ns | -8.0% | -5.6% | -15.0% | -13.3% |
| 32 | 2389ns | -6.9% | -9.3% | -10.5% | -14.4% |
| 33 | 2342ns | -7.3% | -11.4% | -14.3% | -12.7% |
| 34 | 2358ns | -7.8% | -12.0% | -15.0% | -13.3% |
| 35 | 2371ns | -8.4% | -12.5% | -15.4% | -13.8% |
| 36 | 2403ns | -9.5% | -9.6% | -16.6% | -14.9% |
| 37 | 2420ns | -10.2% | -10.2% | -17.1% | -15.6% |
| 38 | 2421ns | -10.3% | -9.0% | -17.2% | -15.6% |
| 39 | 2372ns | -8.4% | -12.2% | -15.5% | -13.8% |
| 40 | 2387ns | -9.0% | -13.2% | -16.0% | -14.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.866 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.695 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.561 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.810 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.820 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 33/40, lost 7/40
- **wide-rung-ragged-overread**: won 35/40, lost 5/40
- **wide-rung-wordround**: won 23/40, lost 17/40
- **wide-rung-wordround-alias**: won 30/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.4ns | 2297.1ns | 0.1% |  |
| wide-rung-ragged | 2.3ns | 2226.1ns | 0.1% |  |
| wide-rung-ragged-overread | 2.2ns | 2108.8ns | 0.1% |  |
| wide-rung-wordround | 2.5ns | 2210.8ns | 0.1% |  |
| wide-rung-wordround-alias | 2.3ns | 2142.0ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2181.9-2414.0 ns)
   2181.9 |##########
   2193.5 |########################################
   2205.1 |##########
   2216.7 |##############################
   2228.3 |##############################
   2239.9 |####################
   2251.5 |####################
   2263.1 |
   2274.7 |
   2286.3 |
   2297.9 |
   2309.5 |
   2321.1 |
   2332.7 |##############################
   2344.3 |
   2355.9 |########################################
   2367.5 |####################
   2379.1 |########################################
   2390.7 |####################
   2402.3 |####################
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 2118.2-2362.4 ns)
   2118.2 |
   2130.4 |
   2142.7 |
   2154.9 |
   2167.1 |########################################
   2179.3 |
   2191.5 |
   2203.7 |
   2215.9 |####
   2228.1 |
   2240.3 |
   2252.6 |
   2264.8 |
   2277.0 |
   2289.2 |##
   2301.4 |
   2313.6 |
   2325.8 |##
   2338.0 |##################
   2350.2 |##
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 2034.2-2199.1 ns)
   2034.2 |########################################
   2042.4 |#############
   2050.7 |
   2058.9 |
   2067.2 |#################
   2075.4 |########
   2083.7 |####
   2091.9 |####
   2100.2 |
   2108.4 |
   2116.7 |
   2124.9 |
   2133.2 |####
   2141.4 |
   2149.7 |
   2157.9 |
   2166.2 |#################
   2174.4 |####
   2182.6 |##########################
   2190.9 |########
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 2005.5-2479.6 ns)
   2005.5 |##################################
   2029.2 |###########
   2052.9 |######################
   2076.6 |###########
   2100.3 |###########
   2124.0 |#####
   2147.7 |#################
   2171.4 |
   2195.1 |
   2218.8 |
   2242.6 |
   2266.3 |#####
   2290.0 |
   2313.7 |############################
   2337.4 |#####
   2361.1 |
   2384.8 |
   2408.5 |
   2432.2 |
   2455.9 |########################################
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 2043.2-2355.7 ns)
   2043.2 |########################################
   2058.8 |###
   2074.4 |#############################
   2090.1 |
   2105.7 |#######
   2121.3 |###
   2136.9 |#######
   2152.6 |
   2168.2 |###
   2183.8 |
   2199.4 |
   2215.1 |
   2230.7 |
   2246.3 |
   2262.0 |
   2277.6 |
   2293.2 |
   2308.8 |#######
   2324.5 |#####################
   2340.1 |
  (4 below, 2 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.82 (measurement drift or warm-up artifact)
