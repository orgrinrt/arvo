# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 18% faster than the next best (warm-clamp-accfit)

warm-clamp-min-lanes (1.78 us) leads warm-clamp-accfit (2.11 us) by 18%, a clear separation rather than a photo finish. CV 50.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 22% (significant)

warm-clamp-min-lanes is -484 ns (22%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is fastest but the noisiest (CV 50.4%)

warm-clamp-min-lanes wins on median (1.78 us) yet has the highest variance (CV 50.4%), while warm-clamp-minimum is the steadiest (CV 3.2%, 2.24 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.83)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64, warm-clamp-minimum} vs {warm-clamp-accfit-dyn} (32% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64, warm-clamp-minimum} and a slow tier {warm-clamp-accfit-dyn} with a 32% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes is inconsistent: worst-20% is 1.8x its best-20%

warm-clamp-min-lanes's best 20% of batches run at 1.75 us but its worst 20% at 3.08 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-clamp-minimum's edge over baseline is significant but tiny (2 ns, 0.07%)

warm-clamp-minimum differs from baseline warm-clamp-acc64 by 2 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 1779.2 ns median (-20.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.66x (fastest 1779.2 ns, slowest 2956.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2349ns | 2306ns | 2189ns | 2307ns | 2635ns | base |
| warm-clamp-accfit | 2397ns | 2170ns | 2136ns | 2179ns | 3310ns | +2.03% |
| warm-clamp-accfit-dyn | 3029ns | 3018ns | 2860ns | 3007ns | 3265ns | +28.96% |
| warm-clamp-head | 2293ns | 2238ns | 2126ns | 2258ns | 2565ns | -2.38% |
| warm-clamp-min-lanes | 2123ns | 1841ns | 1815ns | 1857ns | 3230ns | -9.60% |
| warm-clamp-minimum | 2291ns | 2305ns | 2196ns | 2291ns | 2385ns | -2.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2280ns | 2126ns | 2559ns | base | 3.593 |
| warm-clamp-accfit | 2318ns | 2075ns | 3173ns | +1.69% | 3.533 |
| warm-clamp-accfit-dyn | 2967ns | 2804ns | 3194ns | +30.13% | 2.761 |
| warm-clamp-head | 2192ns | 2038ns | 2457ns | -3.86% | 3.737 |
| warm-clamp-min-lanes | 2041ns | 1752ns | 3078ns | -10.50% | 4.015 |
| warm-clamp-minimum | 2222ns | 2129ns | 2311ns | -2.54% | 3.687 |

## Performance model

- Peak throughput: **4.676 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.666 | 78.4% |
| warm-clamp-accfit | 3.889 | 83.2% |
| warm-clamp-accfit-dyn | 2.771 | 59.3% |
| warm-clamp-head | 3.842 | 82.2% |
| warm-clamp-min-lanes | 4.604 | 98.5% |
| warm-clamp-minimum | 3.662 | 78.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 2349ns | 2349ns | base |
| warm-clamp-accfit | 2397ns | 2397ns | +2.03% |
| warm-clamp-accfit-dyn | 3029ns | 3029ns | +28.96% |
| warm-clamp-head | 2293ns | 2293ns | -2.38% |
| warm-clamp-min-lanes | 2123ns | 2123ns | -9.60% |
| warm-clamp-minimum | 2291ns | 2291ns | -2.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2235ns | base | --- | [2172, 2243] | --- | --- | --- | --- |
| warm-clamp-accfit | 2106ns | -90.8ns (-4.1%) | [-131, -54]ns | [2104, 2109] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2957ns | +746.5ns (+33.4%) | [+655, +814]ns | [2949, 2963] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2132ns | -124.4ns (-5.6%) | [-165, -91]ns | [2075, 2196] | YES | 0.0008 | 0.0007 | 0 |
| warm-clamp-min-lanes | 1779ns | -464.0ns (-20.8%) | [-489, -381]ns | [1762, 1817] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2237ns | no significant difference | [-76, +3]ns | [2233, 2241] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 2131ns | +1.1% | +40.9% | -3.0% | +62.1% | -0.4% |
| 2 | 2180ns | -5.1% | +35.4% | +13.2% | -15.1% | -1.9% |
| 3 | 2122ns | -2.3% | +39.3% | +16.3% | +210.6% | +0.2% |
| 4 | 2125ns | -2.6% | +39.1% | +16.1% | +120.7% | -0.0% |
| 5 | 2138ns | -3.1% | +38.3% | +15.6% | -14.8% | +2.2% |
| 6 | 2122ns | -2.5% | +41.5% | +16.2% | -14.2% | +0.7% |
| 7 | 2124ns | -1.1% | +46.2% | +16.3% | +14.8% | +1.4% |
| 8 | 2159ns | -3.7% | +38.7% | +14.2% | -13.8% | -1.4% |
| 9 | 2125ns | -2.6% | +39.0% | +12.2% | -12.7% | +0.0% |
| 10 | 2123ns | -0.6% | +39.4% | +8.9% | -12.6% | +0.1% |
| 11 | 2578ns | -15.6% | +43.1% | -15.8% | -31.8% | -6.0% |
| 12 | 2578ns | -15.4% | +8.8% | -15.8% | -28.0% | -6.0% |
| 13 | 2577ns | +57.0% | +8.8% | -14.9% | -29.5% | -7.8% |
| 14 | 2576ns | +39.6% | +9.0% | -14.7% | -30.1% | -13.0% |
| 15 | 2578ns | +27.1% | +12.2% | -16.4% | -29.5% | -13.4% |
| 16 | 2576ns | -15.3% | +8.8% | -16.2% | -30.1% | -13.2% |
| 17 | 2419ns | -9.2% | +15.9% | -5.0% | -27.5% | -7.5% |
| 18 | 2410ns | +67.3% | +16.5% | -4.0% | -24.6% | -6.9% |
| 19 | 2409ns | +60.6% | +16.3% | -3.7% | -25.8% | -7.3% |
| 20 | 2411ns | -9.6% | +16.3% | -3.9% | -27.1% | -6.7% |
| 21 | 2237ns | -6.0% | +28.9% | -7.4% | -21.6% | -3.4% |
| 22 | 2240ns | -5.9% | +29.7% | -7.3% | -21.6% | -3.6% |
| 23 | 2240ns | -6.0% | +33.3% | -7.4% | -21.7% | -3.5% |
| 24 | 2589ns | -18.7% | +14.5% | -19.9% | -32.4% | -13.3% |
| 25 | 2375ns | -11.3% | +24.5% | -12.3% | -26.3% | -5.6% |
| 26 | 2209ns | -4.9% | +34.1% | -6.2% | -20.6% | +1.5% |
| 27 | 2181ns | -3.3% | +35.6% | -3.9% | -19.4% | +2.7% |
| 28 | 2239ns | -5.9% | +32.4% | -7.4% | -21.7% | -0.1% |
| 29 | 2244ns | -6.3% | +31.7% | -5.9% | -21.6% | -0.3% |
| 30 | 2242ns | -5.8% | +32.2% | -4.0% | -21.9% | -0.3% |
| 31 | 2210ns | -4.6% | +33.8% | -7.9% | -20.8% | +1.3% |
| 32 | 2164ns | -2.6% | +36.3% | -5.9% | -18.0% | +3.5% |
| 33 | 2154ns | -2.3% | +51.9% | -5.5% | -17.6% | +4.2% |
| 34 | 2158ns | -2.4% | +37.5% | -2.3% | -17.8% | +4.0% |
| 35 | 2279ns | -7.6% | +27.3% | -10.4% | -21.8% | -1.9% |
| 36 | 2146ns | -2.0% | +38.5% | -4.8% | -14.5% | +4.4% |
| 37 | 2145ns | -0.7% | +40.1% | -5.0% | -17.3% | +4.5% |
| 38 | 2210ns | -4.8% | +33.4% | -7.8% | -20.0% | +2.3% |
| 39 | 2234ns | -5.8% | +55.8% | -8.8% | -21.0% | +0.1% |
| 40 | 2236ns | -5.9% | +28.0% | -8.8% | -16.0% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.742 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.544 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.014 | ok |
| warm-clamp-head | 0.827 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.384 | moderate+ |
| warm-clamp-minimum | 0.643 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 34/40, lost 6/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 31/40, lost 9/40
- **warm-clamp-min-lanes**: won 36/40, lost 4/40
- **warm-clamp-minimum**: won 22/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.1ns | 2279.9ns | 0.1% |  |
| warm-clamp-accfit | 3.8ns | 2318.5ns | 0.2% |  |
| warm-clamp-accfit-dyn | 3.2ns | 2966.8ns | 0.1% |  |
| warm-clamp-head | 3.1ns | 2191.9ns | 0.1% |  |
| warm-clamp-min-lanes | 3.7ns | 2040.5ns | 0.2% |  |
| warm-clamp-minimum | 2.9ns | 2222.0ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2126.3-2558.9 ns)
   2126.3 |######################
   2148.0 |######################
   2169.6 |###########
   2191.2 |#################
   2212.9 |#####
   2234.5 |########################################
   2256.1 |
   2277.7 |#####
   2299.4 |
   2321.0 |
   2342.6 |
   2364.3 |#####
   2385.9 |
   2407.5 |######################
   2429.1 |
   2450.8 |
   2472.4 |
   2494.0 |
   2515.6 |
   2537.3 |
  (6 below, 7 above range)

warm-clamp-accfit (n=40, range 2075.3-3172.7 ns)
   2075.3 |########################################
   2130.1 |########
   2185.0 |#
   2239.9 |
   2294.7 |
   2349.6 |
   2404.5 |
   2459.3 |
   2514.2 |
   2569.1 |
   2624.0 |
   2678.8 |
   2733.7 |
   2788.6 |
   2843.4 |
   2898.3 |
   2953.2 |
   3008.0 |
   3062.9 |
   3117.8 |
  (6 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 2804.3-3194.3 ns)
   2804.3 |#############
   2823.8 |
   2843.3 |###
   2862.8 |###
   2882.3 |###
   2901.8 |######
   2921.3 |
   2940.8 |########################################
   2960.3 |####################
   2979.8 |######
   2999.3 |##########
   3018.8 |
   3038.3 |
   3057.8 |
   3077.3 |
   3096.8 |###
   3116.3 |
   3135.8 |
   3155.3 |
   3174.8 |
  (4 below, 3 above range)

warm-clamp-head (n=40, range 2037.8-2457.5 ns)
   2037.8 |############################
   2058.7 |########################################
   2079.7 |###########
   2100.7 |###########
   2121.7 |
   2142.7 |#################
   2163.7 |###########
   2184.7 |###########
   2205.6 |
   2226.6 |
   2247.6 |
   2268.6 |
   2289.6 |#####
   2310.6 |######################
   2331.6 |
   2352.6 |
   2373.5 |#####
   2394.5 |
   2415.5 |
   2436.5 |
  (4 below, 7 above range)

warm-clamp-min-lanes (n=40, range 1751.9-3078.4 ns)
   1751.9 |########################################
   1818.2 |###############
   1884.6 |
   1950.9 |
   2017.2 |
   2083.5 |
   2149.9 |
   2216.2 |
   2282.5 |
   2348.8 |
   2415.2 |#
   2481.5 |
   2547.8 |
   2614.1 |
   2680.5 |
   2746.8 |
   2813.1 |
   2879.5 |
   2945.8 |
   3012.1 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 2129.2-2311.4 ns)
   2129.2 |###
   2138.3 |#######
   2147.4 |###
   2156.5 |##########
   2165.6 |
   2174.7 |
   2183.8 |###
   2193.0 |
   2202.1 |
   2211.2 |
   2220.3 |
   2229.4 |####################################
   2238.5 |########################################
   2247.6 |###
   2256.8 |###
   2265.9 |###
   2275.0 |
   2284.1 |
   2293.2 |
   2302.3 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: CV=24.0% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: CV=44.0% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: autocorrelation=0.64 (measurement drift or warm-up artifact)
