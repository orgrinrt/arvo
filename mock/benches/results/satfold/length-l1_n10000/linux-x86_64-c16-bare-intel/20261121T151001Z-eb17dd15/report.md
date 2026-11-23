# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 91% (significant)

satfold-neon is -19.02 us (91%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 13.1x slower than the field

satfold-seq (24.48 us) is 13.1x the fastest (1.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 shows alternating (throttle bounce) (autocorr -0.66)

satfold-neon8's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 13.1x the fastest

Fastest satfold-neon (1.86 us) to slowest satfold-seq (24.48 us): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.6x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.42 us but its worst 20% at 8.83 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 1862.2 ns median (-91.1% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 13.15x (fastest 1862.2 ns, slowest 24478.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112723939 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 22149ns | 20990ns | 20793ns | 21427ns | 25668ns | base |
| satfold-lanes16 | 3252ns | 3339ns | 1794ns | 3347ns | 4424ns | -85.32% |
| satfold-lanes16-constl | 3143ns | 3118ns | 1615ns | 3157ns | 4628ns | -85.81% |
| satfold-lanes4-idx | 4223ns | 3048ns | 2482ns | 3176ns | 9102ns | -80.94% |
| satfold-lanes64 | 2271ns | 2158ns | 1964ns | 2210ns | 2764ns | -89.74% |
| satfold-neon | 2075ns | 2048ns | 1906ns | 2048ns | 2326ns | -90.63% |
| satfold-neon8 | 2260ns | 2176ns | 2018ns | 2235ns | 2574ns | -89.80% |
| satfold-nolaw | 9229ns | 8760ns | 8722ns | 8920ns | 10665ns | -58.33% |
| satfold-seq | 23342ns | 24574ns | 20909ns | 23372ns | 25681ns | +5.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 22081ns | 20734ns | 25589ns | base | 1.484 |
| satfold-lanes16 | 3048ns | 1689ns | 4122ns | -86.20% | 10.751 |
| satfold-lanes16-constl | 2944ns | 1520ns | 4317ns | -86.67% | 11.132 |
| satfold-lanes4-idx | 4112ns | 2424ns | 8830ns | -81.38% | 7.969 |
| satfold-lanes64 | 2051ns | 1783ns | 2482ns | -90.71% | 15.979 |
| satfold-neon | 1874ns | 1728ns | 2061ns | -91.51% | 17.481 |
| satfold-neon8 | 2043ns | 1833ns | 2306ns | -90.75% | 16.039 |
| satfold-nolaw | 9160ns | 8662ns | 10583ns | -58.52% | 3.577 |
| satfold-seq | 23271ns | 20844ns | 25602ns | +5.39% | 1.408 |

## Performance model

- Peak throughput: **21.563 Gops/s** (satfold-lanes16-constl; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.566 | 7.3% |
| satfold-lanes16 | 10.436 | 48.4% |
| satfold-lanes16-constl | 11.168 | 51.8% |
| satfold-lanes4-idx | 11.018 | 51.1% |
| satfold-lanes64 | 16.680 | 77.4% |
| satfold-neon | 17.596 | 81.6% |
| satfold-neon8 | 16.541 | 76.7% |
| satfold-nolaw | 3.770 | 17.5% |
| satfold-seq | 1.339 | 6.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 22149ns | 22149ns | base |
| satfold-lanes16 | 3252ns | 3252ns | -85.32% |
| satfold-lanes16-constl | 3143ns | 3143ns | -85.81% |
| satfold-lanes4-idx | 4223ns | 4223ns | -80.94% |
| satfold-lanes64 | 2271ns | 2271ns | -89.74% |
| satfold-neon | 2075ns | 2075ns | -90.63% |
| satfold-neon8 | 2260ns | 2260ns | -89.80% |
| satfold-nolaw | 9229ns | 9229ns | -58.33% |
| satfold-seq | 23342ns | 23342ns | +5.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 20931ns | base | --- | [20928, 21046] | --- | --- | --- | --- |
| satfold-lanes16 | 3140ns | -17944.4ns (-85.7%) | [-19446, -17554]ns | [2798, 3331] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2934ns | -18183.1ns (-86.9%) | [-19379, -18026]ns | [2701, 3248] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2974ns | -18182.2ns (-86.9%) | [-18328, -17797]ns | [2629, 3118] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1964ns | -19007.0ns (-90.8%) | [-19089, -18948]ns | [1912, 2093] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 1862ns | -19020.2ns (-90.9%) | [-19170, -18997]ns | [1796, 1925] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1981ns | -18930.4ns (-90.4%) | [-19017, -18813]ns | [1938, 2104] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8692ns | -12246.5ns (-58.5%) | [-12368, -12217]ns | [8678, 8714] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 24479ns | no significant difference | [-3, +4596]ns | [20940, 25394] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 20927ns | -82.1% | -84.5% | -86.9% | -90.3% | -90.8% | -89.7% | -58.6% | +22.3% |
| 1 | 2 | 20930ns | -85.3% | -86.0% | -85.3% | -90.9% | -90.8% | -90.8% | -58.4% | +21.6% |
| 1 | 3 | 20934ns | -83.4% | -87.2% | -85.1% | -90.9% | -91.4% | -90.0% | -58.6% | +0.0% |
| 1 | 4 | 25589ns | -88.9% | -86.6% | -90.0% | -90.8% | -93.0% | -92.4% | -58.7% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.083 | ok |
| satfold-lanes16 | -0.447 | moderate- |
| satfold-lanes16-constl | -0.331 | moderate- |
| satfold-lanes4-idx | -0.253 | moderate- |
| satfold-lanes64 | -0.150 | ok |
| satfold-neon | 0.215 | moderate+ |
| satfold-neon8 | -0.664 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.096 | ok |
| satfold-seq | 0.258 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.3ns | 22081.2ns | 0.1% |  |
| satfold-lanes16 | 69.3ns | 3047.8ns | 2.3% |  |
| satfold-lanes16-constl | 67.9ns | 2943.7ns | 2.3% |  |
| satfold-lanes4-idx | 33.7ns | 4111.9ns | 0.8% |  |
| satfold-lanes64 | 77.1ns | 2050.7ns | 3.8% |  |
| satfold-neon | 69.1ns | 1874.5ns | 3.7% |  |
| satfold-neon8 | 75.3ns | 2043.1ns | 3.7% |  |
| satfold-nolaw | 20.2ns | 9159.8ns | 0.2% |  |
| satfold-seq | 21.4ns | 23270.9ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 20733.7-25588.8 ns)
  20733.7 |########################################
  20976.5 |#
  21219.2 |###
  21462.0 |#
  21704.7 |
  21947.5 |
  22190.2 |
  22433.0 |
  22675.8 |
  22918.5 |
  23161.3 |
  23404.0 |
  23646.8 |
  23889.5 |
  24132.3 |
  24375.1 |
  24617.8 |
  24860.6 |
  25103.3 |#
  25346.1 |#######
  (5 below, 5 above range)

satfold-lanes16 (n=40, range 1689.0-4122.0 ns)
   1689.0 |
   1810.7 |
   1932.3 |
   2054.0 |
   2175.6 |
   2297.2 |
   2418.9 |
   2540.5 |####################
   2662.2 |#############
   2783.8 |########################################
   2905.5 |
   3027.1 |##########################
   3148.8 |#############
   3270.4 |########################################
   3392.1 |
   3513.7 |#############
   3635.4 |####################
   3757.0 |######
   3878.7 |#############
   4000.3 |######
  (5 below, 3 above range)

satfold-lanes16-constl (n=40, range 1519.6-4317.3 ns)
   1519.6 |
   1659.5 |
   1799.4 |
   1939.3 |
   2079.2 |
   2219.1 |######
   2358.9 |#############
   2498.8 |########################################
   2638.7 |####################
   2778.6 |####################
   2918.5 |#################################
   3058.4 |
   3198.2 |##########################
   3338.1 |######
   3478.0 |####################
   3617.9 |#############
   3757.8 |
   3897.7 |
   4037.5 |######
   4177.4 |
  (5 below, 4 above range)

satfold-lanes4-idx (n=40, range 2424.3-8829.9 ns)
   2424.3 |########################################
   2744.5 |##########
   3064.8 |#################
   3385.1 |##
   3705.4 |
   4025.7 |
   4345.9 |
   4666.2 |
   4986.5 |##
   5306.8 |
   5627.1 |
   5947.4 |##
   6267.6 |
   6587.9 |
   6908.2 |##
   7228.5 |##
   7548.8 |##
   7869.0 |##
   8189.3 |
   8509.6 |
  (2 below, 4 above range)

satfold-lanes64 (n=40, range 1782.9-2482.5 ns)
   1782.9 |##########
   1817.9 |#####
   1852.8 |#####
   1887.8 |########################################
   1922.8 |##########
   1957.8 |###############
   1992.8 |##########
   2027.7 |
   2062.7 |####################
   2097.7 |##########
   2132.7 |#####
   2167.7 |
   2202.6 |#####
   2237.6 |#####
   2272.6 |
   2307.6 |##########
   2342.6 |
   2377.5 |#####
   2412.5 |
   2447.5 |
  (5 below, 4 above range)

satfold-neon (n=40, range 1728.3-2061.0 ns)
   1728.3 |#####
   1744.9 |
   1761.6 |########################################
   1778.2 |#################
   1794.8 |#####
   1811.5 |######################
   1828.1 |#####
   1844.7 |
   1861.4 |
   1878.0 |#####
   1894.6 |#####
   1911.3 |########################################
   1927.9 |#################
   1944.6 |
   1961.2 |#####
   1977.8 |#####
   1994.5 |#####
   2011.1 |
   2027.7 |
   2044.4 |#####
  (3 below, 4 above range)

satfold-neon8 (n=40, range 1833.0-2306.0 ns)
   1833.0 |###########
   1856.6 |
   1880.3 |
   1903.9 |######################
   1927.6 |############################
   1951.2 |######################
   1974.9 |###########
   1998.5 |
   2022.2 |
   2045.8 |
   2069.5 |#####
   2093.1 |########################################
   2116.8 |
   2140.4 |######################
   2164.1 |
   2187.7 |
   2211.4 |#####
   2235.0 |
   2258.7 |
   2282.3 |
  (4 below, 6 above range)

satfold-nolaw (n=40, range 8661.6-10583.0 ns)
   8661.6 |########################################
   8757.7 |#
   8853.8 |
   8949.8 |
   9045.9 |
   9142.0 |
   9238.0 |
   9334.1 |
   9430.2 |
   9526.2 |
   9622.3 |
   9718.4 |
   9814.5 |
   9910.5 |
  10006.6 |
  10102.7 |
  10198.7 |
  10294.8 |
  10390.9 |
  10486.9 |#########
  (3 below, 4 above range)

satfold-seq (n=40, range 20844.0-25602.3 ns)
  20844.0 |########################################
  21081.9 |
  21319.8 |
  21557.7 |
  21795.6 |
  22033.6 |
  22271.5 |
  22509.4 |
  22747.3 |
  22985.2 |
  23223.2 |
  23461.1 |##
  23699.0 |
  23936.9 |
  24174.8 |
  24412.7 |
  24650.7 |
  24888.6 |
  25126.5 |#############
  25364.4 |########################
  (4 below, 6 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=29.2% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=34.1% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=61.7% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.6x (possible bimodal distribution)
