# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged-overread shows warm-up / thermal drift (autocorr +0.87)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged's edge over baseline is significant but tiny (14 ns, 0.73%)

wide-rung-ragged differs from baseline wide-rung-align16 by 14 ns (0.73%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 1794.3 ns median (-3.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.09x (fastest 1794.3 ns, slowest 1960.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 1932ns | 1914ns | 1816ns | 1910ns | 2112ns | base |
| wide-rung-ragged | 2004ns | 1903ns | 1831ns | 1975ns | 2266ns | +3.76% |
| wide-rung-ragged-overread | 2024ns | 2005ns | 1828ns | 2008ns | 2269ns | +4.79% |
| wide-rung-wordround | 1978ns | 2025ns | 1827ns | 1984ns | 2112ns | +2.39% |
| wide-rung-wordround-alias | 1861ns | 1855ns | 1821ns | 1856ns | 1917ns | -3.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1866ns | 1756ns | 2039ns | base | 1.098 |
| wide-rung-ragged | 1938ns | 1770ns | 2192ns | +3.85% | 1.057 |
| wide-rung-ragged-overread | 1955ns | 1766ns | 2190ns | +4.77% | 1.048 |
| wide-rung-wordround | 1910ns | 1763ns | 2040ns | +2.35% | 1.072 |
| wide-rung-wordround-alias | 1799ns | 1760ns | 1852ns | -3.60% | 1.139 |

## Performance model

- Peak throughput: **1.167 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.107 | 94.9% |
| wide-rung-ragged | 1.115 | 95.6% |
| wide-rung-ragged-overread | 1.058 | 90.7% |
| wide-rung-wordround | 1.044 | 89.5% |
| wide-rung-wordround-alias | 1.141 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1932ns | 1932ns | base |
| wide-rung-ragged | 2004ns | 2004ns | +3.76% |
| wide-rung-ragged-overread | 2024ns | 2024ns | +4.79% |
| wide-rung-wordround | 1978ns | 1978ns | +2.39% |
| wide-rung-wordround-alias | 1861ns | 1861ns | -3.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1850ns | base | --- | [1764, 1888] | --- | --- | --- | --- |
| wide-rung-ragged | 1837ns | +14.0ns (+0.8%) | [+10, +16]ns | [1774, 2049] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 1935ns | +116.7ns (+6.3%) | [+1, +296]ns | [1802, 2060] | YES (adj: no) | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround | 1961ns | no significant difference | [-2, +152]ns | [1774, 2036] | no | 0.4177 | 0.4177 | 2 |
| wide-rung-wordround-alias | 1794ns | -45.6ns (-2.5%) | [-122, -1]ns | [1765, 1823] | YES (adj: no) | 0.1065 | 0.0533 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 1960ns | +11.8% | +11.5% | +3.9% | -10.0% |
| 2 | 1884ns | +16.4% | +16.2% | +8.1% | -6.4% |
| 3 | 1880ns | +16.5% | +16.5% | +8.4% | -6.2% |
| 4 | 1886ns | +16.0% | +16.1% | +8.0% | -6.5% |
| 5 | 1889ns | +16.0% | +15.7% | +7.8% | -6.9% |
| 6 | 1884ns | +16.3% | +17.6% | +8.3% | -6.4% |
| 7 | 1890ns | +15.9% | +15.4% | +7.6% | -6.3% |
| 8 | 1888ns | +16.1% | +15.7% | +7.9% | -5.0% |
| 9 | 1880ns | +16.6% | +16.1% | +8.4% | -4.3% |
| 10 | 1948ns | +12.7% | +12.2% | +4.6% | -7.8% |
| 11 | 1764ns | +0.5% | -0.0% | -0.0% | +0.1% |
| 12 | 1766ns | +0.5% | +0.2% | -0.2% | -0.5% |
| 13 | 1763ns | +0.5% | +0.4% | +0.0% | -0.1% |
| 14 | 1764ns | +0.1% | +0.3% | -0.0% | +0.0% |
| 15 | 1821ns | -2.7% | -2.9% | -2.2% | +2.8% |
| 16 | 1775ns | -0.1% | -0.3% | -0.4% | +0.7% |
| 17 | 1763ns | +0.6% | +0.1% | +0.0% | -0.6% |
| 18 | 1762ns | +0.8% | +0.3% | +7.1% | -0.2% |
| 19 | 1765ns | +0.3% | -0.3% | -0.5% | -0.1% |
| 20 | 1765ns | +0.4% | +0.2% | +0.1% | -0.1% |
| 21 | 2036ns | +0.6% | -11.3% | -8.3% | -8.4% |
| 22 | 2040ns | +0.4% | -11.6% | -11.0% | -7.1% |
| 23 | 2036ns | +0.7% | -11.2% | -13.1% | -10.1% |
| 24 | 2031ns | +0.8% | -11.2% | -11.6% | -10.2% |
| 25 | 2058ns | -0.4% | -12.5% | -13.9% | -11.3% |
| 26 | 2033ns | +1.1% | -11.2% | -12.6% | -10.2% |
| 27 | 2037ns | +0.6% | -11.6% | -13.3% | -10.5% |
| 28 | 2036ns | -1.0% | -10.8% | -13.0% | -10.4% |
| 29 | 2035ns | -6.8% | -11.6% | -13.0% | -8.0% |
| 30 | 2031ns | -6.4% | -11.4% | -12.8% | -10.0% |
| 31 | 1748ns | +1.2% | +17.9% | +16.7% | +4.5% |
| 32 | 1748ns | +1.2% | +17.5% | +17.1% | +4.5% |
| 33 | 1755ns | +1.0% | +17.5% | +16.0% | +3.8% |
| 34 | 1754ns | +0.9% | +17.5% | +16.0% | +3.9% |
| 35 | 1762ns | +0.9% | +16.8% | +15.4% | +3.6% |
| 36 | 1756ns | +0.9% | +17.3% | +16.1% | +3.7% |
| 37 | 1762ns | +0.7% | +17.0% | +15.5% | +3.2% |
| 38 | 1763ns | +0.4% | +17.0% | +15.3% | +0.1% |
| 39 | 1763ns | +0.5% | +16.7% | +15.7% | +0.4% |
| 40 | 1760ns | +0.8% | +17.0% | +15.8% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.785 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.854 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.873 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.829 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.592 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 5/40, lost 33/40
- **wide-rung-ragged-overread**: won 13/40, lost 26/40
- **wide-rung-wordround**: won 14/40, lost 22/40
- **wide-rung-wordround-alias**: won 23/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.5ns | 1866.0ns | 0.1% |  |
| wide-rung-ragged | 2.4ns | 1937.8ns | 0.1% |  |
| wide-rung-ragged-overread | 2.4ns | 1954.9ns | 0.1% |  |
| wide-rung-wordround | 2.4ns | 1909.9ns | 0.1% |  |
| wide-rung-wordround-alias | 2.4ns | 1798.9ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 1755.5-2038.8 ns)
   1755.5 |########################################
   1769.7 |##
   1783.9 |
   1798.0 |
   1812.2 |##
   1826.4 |
   1840.5 |
   1854.7 |
   1868.9 |#####
   1883.0 |#################
   1897.2 |
   1911.3 |
   1925.5 |
   1939.7 |##
   1953.8 |##
   1968.0 |
   1982.2 |
   1996.3 |
   2010.5 |
   2024.7 |######################
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 1769.6-2192.0 ns)
   1769.6 |########################################
   1790.7 |
   1811.9 |
   1833.0 |
   1854.1 |
   1875.2 |
   1896.3 |####
   1917.5 |
   1938.6 |
   1959.7 |
   1980.8 |
   2001.9 |##
   2023.1 |
   2044.2 |################
   2065.3 |
   2086.4 |
   2107.5 |
   2128.7 |
   2149.8 |
   2170.9 |################
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 1766.3-2190.4 ns)
   1766.3 |############################
   1787.5 |####################################
   1808.7 |####
   1829.9 |
   1851.1 |
   1872.3 |
   1893.5 |
   1914.7 |
   1935.9 |
   1957.1 |
   1978.3 |
   1999.5 |
   2020.8 |
   2042.0 |########################################
   2063.2 |
   2084.4 |
   2105.6 |
   2126.8 |
   2148.0 |
   2169.2 |####################################
  (3 below, 1 above range)

wide-rung-wordround (n=40, range 1763.0-2039.7 ns)
   1763.0 |######################
   1776.8 |##
   1790.7 |##
   1804.5 |##
   1818.3 |
   1832.2 |
   1846.0 |
   1859.8 |##
   1873.7 |
   1887.5 |##
   1901.3 |
   1915.2 |
   1929.0 |
   1942.8 |
   1956.7 |
   1970.5 |
   1984.3 |
   1998.2 |
   2012.0 |
   2025.8 |########################################
  (5 below, 2 above range)

wide-rung-wordround-alias (n=40, range 1759.8-1851.7 ns)
   1759.8 |########################################
   1764.4 |#############
   1769.0 |########
   1773.6 |
   1778.2 |
   1782.8 |
   1787.4 |####
   1792.0 |########
   1796.6 |####
   1801.2 |
   1805.8 |
   1810.4 |
   1815.0 |####
   1819.6 |######################
   1824.2 |###############################
   1828.7 |####
   1833.3 |
   1837.9 |
   1842.5 |
   1847.1 |
  (4 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.59 (measurement drift or warm-up artifact)
