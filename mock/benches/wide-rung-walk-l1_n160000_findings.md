# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 shows warm-up / thermal drift (autocorr +0.87)

wide-rung-align16's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-wordround's comparison is tie-heavy (10% tied pairs)

10% of paired samples for wide-rung-wordround are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### wide-rung-ragged's edge over baseline is significant but tiny (-7 ns, 0.35%)

wide-rung-ragged differs from baseline wide-rung-align16 by -7 ns (0.35%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1823.1 ns median (-3.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 1823.1 ns, slowest 1954.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 2005ns | 1955ns | 1822ns | 1980ns | 2264ns | base |
| wide-rung-ragged | 1998ns | 2022ns | 1832ns | 2007ns | 2136ns | -0.36% |
| wide-rung-ragged-overread | 1984ns | 1967ns | 1846ns | 1985ns | 2121ns | -1.03% |
| wide-rung-wordround | 2044ns | 1958ns | 1823ns | 2011ns | 2362ns | +1.94% |
| wide-rung-wordround-alias | 1893ns | 1884ns | 1824ns | 1889ns | 1974ns | -5.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1937ns | 1761ns | 2186ns | base | 1.057 |
| wide-rung-ragged | 1930ns | 1773ns | 2062ns | -0.35% | 1.061 |
| wide-rung-ragged-overread | 1916ns | 1784ns | 2051ns | -1.05% | 1.069 |
| wide-rung-wordround | 1975ns | 1762ns | 2281ns | +1.97% | 1.037 |
| wide-rung-wordround-alias | 1829ns | 1762ns | 1906ns | -5.55% | 1.120 |

## Performance model

- Peak throughput: **1.163 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.084 | 93.2% |
| wide-rung-ragged | 1.048 | 90.1% |
| wide-rung-ragged-overread | 1.078 | 92.7% |
| wide-rung-wordround | 1.082 | 93.0% |
| wide-rung-wordround-alias | 1.123 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 2005ns | 2005ns | base |
| wide-rung-ragged | 1998ns | 1998ns | -0.36% |
| wide-rung-ragged-overread | 1984ns | 1984ns | -1.03% |
| wide-rung-wordround | 2044ns | 2044ns | +1.94% |
| wide-rung-wordround-alias | 1893ns | 1893ns | -5.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1889ns | base | --- | [1884, 1892] | --- | --- | --- | --- |
| wide-rung-ragged | 1955ns | no significant difference | [-61, +13]ns | [1834, 2049] | no | 1.0000 | 0.6358 | 0 |
| wide-rung-ragged-overread | 1899ns | no significant difference | [-108, +13]ns | [1898, 1901] | no | 1.0000 | 0.8746 | 0 |
| wide-rung-wordround | 1893ns | no significant difference | [-1, +13]ns | [1887, 2039] | no | 1.0000 | 1.0000 | 1 |
| wide-rung-wordround-alias | 1823ns | -26.2ns (-1.4%) | [-65, -1]ns | [1802, 1854] | YES (adj: no) | 0.3228 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1900ns | -5.6% | +7.8% | +7.4% | -6.8% |
| 2 | 1763ns | +0.5% | +16.2% | +15.7% | -0.2% |
| 3 | 1752ns | +1.2% | +17.1% | +16.5% | +0.6% |
| 4 | 1750ns | +1.2% | +16.8% | +16.7% | +0.8% |
| 5 | 1883ns | -5.9% | +8.7% | +8.2% | -4.8% |
| 6 | 1757ns | +1.0% | +16.6% | +15.9% | +0.0% |
| 7 | 1765ns | +0.5% | +15.9% | +15.2% | -0.4% |
| 8 | 1770ns | +0.0% | +15.5% | +10.0% | -0.2% |
| 9 | 1772ns | +0.2% | +15.6% | -0.2% | -0.5% |
| 10 | 1758ns | +1.1% | +16.7% | +0.4% | +0.2% |
| 11 | 1884ns | +13.7% | +1.1% | +0.6% | +0.7% |
| 12 | 1883ns | +8.9% | +0.8% | +0.3% | +0.7% |
| 13 | 1882ns | +8.9% | +0.9% | +0.7% | +0.8% |
| 14 | 1889ns | +8.5% | +0.6% | -0.0% | +4.7% |
| 15 | 1888ns | +8.3% | +0.5% | -0.0% | +0.6% |
| 16 | 1883ns | +8.7% | +0.8% | +0.2% | +0.6% |
| 17 | 1886ns | +8.5% | +0.6% | +0.1% | +0.4% |
| 18 | 1890ns | +8.5% | +0.4% | -0.1% | +0.2% |
| 19 | 1890ns | +8.4% | +0.6% | -0.1% | +0.2% |
| 20 | 1887ns | +8.6% | +0.6% | +0.2% | +0.4% |
| 21 | 2185ns | -6.2% | -13.1% | +8.8% | -18.6% |
| 22 | 2187ns | -6.5% | -13.1% | +8.5% | -17.7% |
| 23 | 2189ns | -6.4% | -13.2% | +8.3% | -17.8% |
| 24 | 2187ns | -6.3% | -13.3% | +8.6% | -17.5% |
| 25 | 2183ns | -6.1% | -5.5% | +1.0% | -17.4% |
| 26 | 2187ns | -6.2% | -13.1% | -0.1% | -17.5% |
| 27 | 2179ns | -6.0% | -12.8% | -0.0% | -17.2% |
| 28 | 2184ns | -6.1% | -12.8% | -0.2% | -17.5% |
| 29 | 2184ns | -6.0% | -13.0% | -0.1% | -17.5% |
| 30 | 2178ns | -5.9% | -13.0% | +0.0% | -14.2% |
| 31 | 1889ns | -3.2% | -1.3% | -6.8% | -3.4% |
| 32 | 1885ns | -2.9% | -5.9% | -6.4% | -1.1% |
| 33 | 1874ns | -2.1% | -5.5% | -3.3% | -0.9% |
| 34 | 1891ns | -3.2% | -3.7% | -7.0% | -3.6% |
| 35 | 1883ns | -2.6% | -3.6% | -6.3% | -3.0% |
| 36 | 1890ns | -2.8% | +0.5% | -6.5% | -3.5% |
| 37 | 1892ns | -3.1% | -5.5% | -7.0% | -2.7% |
| 38 | 1904ns | -2.1% | -7.2% | -7.4% | -1.6% |
| 39 | 1898ns | -3.3% | -6.9% | -7.2% | -2.5% |
| 40 | 1890ns | -2.6% | -6.3% | -2.8% | -3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.865 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.828 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.790 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.822 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.734 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 22/40, lost 17/40
- **wide-rung-ragged-overread**: won 19/40, lost 21/40
- **wide-rung-wordround**: won 13/40, lost 20/40
- **wide-rung-wordround-alias**: won 26/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.7ns | 1936.7ns | 0.1% |  |
| wide-rung-ragged | 2.6ns | 1929.9ns | 0.1% |  |
| wide-rung-ragged-overread | 2.4ns | 1916.4ns | 0.1% |  |
| wide-rung-wordround | 2.9ns | 1974.9ns | 0.1% |  |
| wide-rung-wordround-alias | 2.5ns | 1829.3ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1760.8-2185.7 ns)
   1760.8 |##############
   1782.1 |
   1803.3 |
   1824.6 |
   1845.8 |
   1867.0 |########################################
   1888.3 |########################################
   1909.5 |
   1930.8 |
   1952.0 |
   1973.3 |
   1994.5 |
   2015.7 |
   2037.0 |
   2058.2 |
   2079.5 |
   2100.7 |
   2122.0 |
   2143.2 |
   2164.4 |#####################
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 1772.9-2062.3 ns)
   1772.9 |#############
   1787.3 |##
   1801.8 |
   1816.3 |##
   1830.7 |#####################
   1845.2 |
   1859.7 |##
   1874.2 |
   1888.6 |
   1903.1 |
   1917.6 |
   1932.0 |
   1946.5 |
   1961.0 |
   1975.5 |
   1989.9 |
   2004.4 |
   2018.9 |
   2033.3 |##########
   2047.8 |########################################
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 1784.0-2050.7 ns)
   1784.0 |##
   1797.3 |
   1810.6 |####
   1824.0 |
   1837.3 |
   1850.6 |##
   1864.0 |
   1877.3 |
   1890.6 |########################################
   1904.0 |####
   1917.3 |
   1930.6 |
   1944.0 |
   1957.3 |
   1970.7 |
   1984.0 |
   1997.3 |
   2010.7 |
   2024.0 |
   2037.3 |####################
  (5 below, 2 above range)

wide-rung-wordround (n=40, range 1761.8-2280.7 ns)
   1761.8 |##############################
   1787.7 |#####
   1813.7 |#####
   1839.6 |
   1865.5 |########################################
   1891.5 |##########
   1917.4 |
   1943.4 |#####
   1969.3 |
   1995.3 |
   2021.2 |###################################
   2047.2 |
   2073.1 |
   2099.1 |
   2125.0 |
   2151.0 |
   2176.9 |#########################
   2202.8 |#####
   2228.8 |
   2254.7 |
  (4 below, 4 above range)

wide-rung-wordround-alias (n=40, range 1761.6-1906.3 ns)
   1761.6 |#################
   1768.9 |####
   1776.1 |####
   1783.3 |
   1790.6 |####
   1797.8 |###################################
   1805.0 |
   1812.3 |
   1819.5 |#############
   1826.7 |########
   1834.0 |####
   1841.2 |
   1848.4 |####
   1855.7 |####
   1862.9 |########
   1870.1 |####
   1877.4 |
   1884.6 |
   1891.8 |########################################
   1899.1 |
  (4 below, 1 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.73 (measurement drift or warm-up artifact)
