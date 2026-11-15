# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-clamp-min-lanes, warm-clamp-acc64) are a dead heat (<1%)

warm-clamp-min-lanes (1.83 us) and warm-clamp-acc64 (1.83 us) differ by 0.14%, inside the noise, even though the wider field spreads 54.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.81)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-minimum, warm-clamp-head, warm-clamp-accfit} vs {warm-clamp-accfit-dyn} (36% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-minimum, warm-clamp-head, warm-clamp-accfit} and a slow tier {warm-clamp-accfit-dyn} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-min-lanes's edge over baseline is significant but tiny (13 ns, 0.70%)

warm-clamp-min-lanes differs from baseline warm-clamp-acc64 by 13 ns (0.70%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 1831.1 ns median (-0.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.54x (fastest 1831.1 ns, slowest 2822.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1897ns | 1897ns | 1870ns | 1894ns | 1932ns | base |
| warm-clamp-accfit | 2186ns | 2138ns | 2084ns | 2131ns | 2455ns | +15.26% |
| warm-clamp-accfit-dyn | 3073ns | 2880ns | 2824ns | 3016ns | 3495ns | +62.02% |
| warm-clamp-head | 2129ns | 2113ns | 2070ns | 2127ns | 2193ns | +12.23% |
| warm-clamp-min-lanes | 1905ns | 1896ns | 1869ns | 1894ns | 1974ns | +0.43% |
| warm-clamp-minimum | 1896ns | 1901ns | 1873ns | 1898ns | 1914ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 1833ns | 1807ns | 1866ns | base | 4.469 |
| warm-clamp-accfit | 2120ns | 2021ns | 2381ns | +15.65% | 3.864 |
| warm-clamp-accfit-dyn | 3008ns | 2765ns | 3419ns | +64.10% | 2.723 |
| warm-clamp-head | 2035ns | 1982ns | 2093ns | +10.99% | 4.027 |
| warm-clamp-min-lanes | 1841ns | 1807ns | 1907ns | +0.45% | 4.449 |
| warm-clamp-minimum | 1831ns | 1810ns | 1848ns | -0.09% | 4.473 |

## Performance model

- Peak throughput: **4.534 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 4.468 | 98.5% |
| warm-clamp-accfit | 3.951 | 87.1% |
| warm-clamp-accfit-dyn | 2.902 | 64.0% |
| warm-clamp-head | 4.058 | 89.5% |
| warm-clamp-min-lanes | 4.474 | 98.7% |
| warm-clamp-minimum | 4.460 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 1897ns | 1897ns | base |
| warm-clamp-accfit | 2186ns | 2186ns | +15.26% |
| warm-clamp-accfit-dyn | 3073ns | 3073ns | +62.02% |
| warm-clamp-head | 2129ns | 2129ns | +12.23% |
| warm-clamp-min-lanes | 1905ns | 1905ns | +0.43% |
| warm-clamp-minimum | 1896ns | 1896ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1834ns | base | --- | [1827, 1836] | --- | --- | --- | --- |
| warm-clamp-accfit | 2074ns | +236.2ns (+12.9%) | [+210, +246]ns | [2036, 2077] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2822ns | +991.1ns (+54.1%) | [+977, +1284]ns | [2806, 3145] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2019ns | +187.7ns (+10.2%) | [+182, +213]ns | [2014, 2056] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1831ns | +6.9ns (+0.4%) | [+2, +13]ns | [1812, 1848] | YES | 0.0296 | 0.0237 | 1 |
| warm-clamp-minimum | 1837ns | no significant difference | [-2, +6]ns | [1829, 1840] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 1855ns | +11.8% | +51.1% | +8.6% | +1.5% | -0.8% |
| 2 | 1836ns | +13.1% | +53.2% | +11.8% | +0.8% | +0.3% |
| 3 | 1835ns | +13.3% | +52.7% | +13.6% | +0.6% | +0.1% |
| 4 | 1835ns | +13.2% | +52.7% | +13.7% | +1.1% | +0.1% |
| 5 | 1829ns | +13.6% | +53.5% | +14.0% | +1.3% | +0.7% |
| 6 | 1835ns | +13.2% | +53.3% | +13.6% | +4.6% | +0.3% |
| 7 | 1840ns | +12.7% | +53.6% | +13.4% | +0.4% | -0.1% |
| 8 | 1838ns | +13.0% | +54.6% | +14.4% | +0.6% | +0.0% |
| 9 | 1857ns | +11.7% | +59.3% | +11.3% | -0.4% | -0.9% |
| 10 | 1835ns | +13.0% | +53.6% | +10.2% | +0.8% | +0.0% |
| 11 | 1805ns | +34.4% | +83.7% | +11.5% | +0.6% | +0.5% |
| 12 | 1807ns | +34.9% | +84.5% | +12.0% | +0.3% | +0.3% |
| 13 | 1827ns | +33.4% | +71.2% | +13.6% | -0.9% | -0.1% |
| 14 | 1822ns | +33.4% | +53.0% | +10.5% | -0.2% | -0.5% |
| 15 | 1822ns | +33.3% | +52.9% | +9.8% | -0.6% | +0.8% |
| 16 | 1810ns | +34.5% | +54.6% | +9.3% | +0.3% | +2.5% |
| 17 | 1811ns | +21.9% | +54.9% | +9.4% | +0.2% | +1.8% |
| 18 | 1809ns | +10.9% | +81.5% | +9.4% | +0.3% | +1.8% |
| 19 | 1805ns | +11.0% | +84.6% | +17.5% | +0.3% | +2.0% |
| 20 | 1803ns | +12.1% | +57.8% | +9.8% | +0.3% | +2.1% |
| 21 | 1803ns | +14.0% | +99.8% | +10.0% | +0.0% | +0.6% |
| 22 | 1820ns | +11.6% | +88.1% | +13.1% | -0.6% | -0.4% |
| 23 | 1812ns | +13.7% | +82.5% | +9.5% | +0.0% | -0.1% |
| 24 | 1851ns | +10.9% | +79.4% | +7.3% | -2.4% | -2.0% |
| 25 | 1858ns | +9.1% | +83.6% | +6.8% | -2.4% | -2.6% |
| 26 | 1850ns | +9.8% | +80.9% | +9.4% | -2.3% | -2.3% |
| 27 | 1835ns | +10.4% | +83.6% | +13.9% | -1.3% | -1.1% |
| 28 | 1838ns | +16.7% | +82.6% | +13.4% | -1.7% | -0.9% |
| 29 | 1895ns | +18.5% | +66.9% | +10.0% | -4.6% | -4.6% |
| 30 | 1857ns | +11.5% | +48.2% | +12.4% | -2.6% | -2.5% |
| 31 | 1830ns | +15.0% | +49.5% | +9.9% | +4.3% | +0.3% |
| 32 | 1830ns | +11.0% | +49.8% | +10.1% | +4.5% | +0.3% |
| 33 | 1904ns | +6.8% | +43.6% | +6.0% | +2.2% | -3.4% |
| 34 | 1832ns | +10.7% | +52.7% | +9.9% | +4.0% | +1.2% |
| 35 | 1828ns | +11.1% | +52.5% | +10.4% | +4.5% | +2.0% |
| 36 | 1838ns | +10.6% | +52.1% | +9.9% | +2.0% | +0.3% |
| 37 | 1836ns | +10.6% | +53.2% | +9.9% | +0.6% | -0.1% |
| 38 | 1836ns | +14.5% | +90.6% | +9.9% | +0.5% | +0.3% |
| 39 | 1830ns | +11.4% | +53.4% | +10.1% | +0.7% | +0.6% |
| 40 | 1827ns | +11.3% | +56.7% | +10.4% | +1.0% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.414 | moderate+ |
| warm-clamp-accfit | 0.807 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.535 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.432 | moderate+ |
| warm-clamp-min-lanes | 0.753 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.730 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/40, lost 40/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 12/40, lost 26/40
- **warm-clamp-minimum**: won 14/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.8ns | 1833.1ns | 0.2% |  |
| warm-clamp-accfit | 2.5ns | 2120.0ns | 0.1% |  |
| warm-clamp-accfit-dyn | 2.9ns | 3008.2ns | 0.1% |  |
| warm-clamp-head | 2.4ns | 2034.5ns | 0.1% |  |
| warm-clamp-min-lanes | 2.6ns | 1841.4ns | 0.1% |  |
| warm-clamp-minimum | 2.6ns | 1831.4ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1806.7-1865.8 ns)
   1806.7 |##########
   1809.7 |###############
   1812.6 |
   1815.6 |
   1818.5 |#####
   1821.5 |##########
   1824.4 |##########
   1827.4 |#########################
   1830.4 |#####
   1833.3 |########################################
   1836.3 |###############
   1839.2 |#####
   1842.2 |
   1845.1 |
   1848.1 |##########
   1851.0 |
   1854.0 |###############
   1857.0 |#####
   1859.9 |
   1862.9 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 2021.2-2381.0 ns)
   2021.2 |########################################
   2039.2 |######
   2057.2 |####################
   2075.2 |####################
   2093.2 |######
   2111.2 |
   2129.2 |###
   2147.2 |
   2165.2 |
   2183.1 |
   2201.1 |###
   2219.1 |
   2237.1 |###
   2255.1 |
   2273.1 |
   2291.1 |
   2309.1 |
   2327.1 |
   2345.1 |
   2363.0 |
  (3 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 2765.0-3418.6 ns)
   2765.0 |############
   2797.7 |########################################
   2830.4 |#########
   2863.1 |
   2895.8 |
   2928.4 |###
   2961.1 |
   2993.8 |
   3026.5 |
   3059.1 |
   3091.8 |
   3124.5 |###
   3157.2 |###
   3189.9 |
   3222.5 |
   3255.2 |###
   3287.9 |######
   3320.6 |############
   3353.2 |######
   3385.9 |###
  (4 below, 3 above range)

warm-clamp-head (n=40, range 1982.0-2092.8 ns)
   1982.0 |####################
   1987.6 |
   1993.1 |
   1998.7 |#####
   2004.2 |
   2009.7 |###################################
   2015.3 |##############################
   2020.8 |###############
   2026.3 |
   2031.9 |
   2037.4 |
   2043.0 |
   2048.5 |#####
   2054.0 |#####
   2059.6 |
   2065.1 |#####
   2070.7 |#####
   2076.2 |
   2081.7 |########################################
   2087.3 |#####
  (4 below, 2 above range)

warm-clamp-min-lanes (n=40, range 1807.0-1907.5 ns)
   1807.0 |########################################
   1812.0 |########################
   1817.1 |####
   1822.1 |
   1827.1 |
   1832.1 |
   1837.1 |
   1842.2 |################
   1847.2 |############################
   1852.2 |####
   1857.2 |
   1862.3 |
   1867.3 |
   1872.3 |####
   1877.3 |
   1882.3 |####
   1887.4 |
   1892.4 |
   1897.4 |
   1902.4 |####
  (3 below, 5 above range)

warm-clamp-minimum (n=40, range 1810.2-1848.2 ns)
   1810.2 |
   1812.1 |############################
   1814.0 |###########
   1815.9 |
   1817.8 |
   1819.7 |
   1821.6 |#####
   1823.5 |#####
   1825.4 |
   1827.3 |
   1829.2 |
   1831.1 |
   1833.0 |#####
   1834.9 |############################
   1836.8 |######################
   1838.7 |######################
   1840.6 |########################################
   1842.5 |###########
   1844.4 |
   1846.3 |
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.73 (measurement drift or warm-up artifact)
