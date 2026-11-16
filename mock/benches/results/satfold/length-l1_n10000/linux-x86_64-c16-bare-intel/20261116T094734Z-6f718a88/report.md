# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (23.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes64 at 1.97 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes64 beats baseline by 91% (significant)

satfold-lanes64 is -20.96 us (91%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 11.7x slower than the field

satfold-iterfold (23.04 us) is 11.7x the fastest (1.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.80)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 11.7x the fastest

Fastest satfold-lanes64 (1.97 us) to slowest satfold-iterfold (23.04 us): 11.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.7x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.26 us but its worst 20% at 8.39 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes64** at 1967.5 ns median (-91.5% vs baseline)
- 8 variants significantly faster than baseline
- Spread: 11.71x (fastest 1967.5 ns, slowest 23039.7 ns)

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
| Counter frequency | 2112716245 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 23082ns | 23106ns | 20757ns | 23079ns | 25414ns | base |
| satfold-lanes16 | 3478ns | 3554ns | 2439ns | 3549ns | 4303ns | -84.93% |
| satfold-lanes16-constl | 3334ns | 3457ns | 2354ns | 3425ns | 4040ns | -85.56% |
| satfold-lanes4-idx | 4142ns | 2707ns | 2322ns | 3240ns | 8666ns | -82.06% |
| satfold-lanes64 | 2221ns | 2168ns | 1952ns | 2195ns | 2571ns | -90.38% |
| satfold-neon | 2339ns | 2290ns | 1886ns | 2303ns | 2898ns | -89.87% |
| satfold-neon8 | 2402ns | 2327ns | 2077ns | 2342ns | 2906ns | -89.59% |
| satfold-nolaw | 9156ns | 8674ns | 8643ns | 8838ns | 10622ns | -60.33% |
| satfold-seq | 22694ns | 22690ns | 20386ns | 22688ns | 25022ns | -1.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 23009ns | 20698ns | 25326ns | base | 1.424 |
| satfold-lanes16 | 3262ns | 2292ns | 4002ns | -85.82% | 10.046 |
| satfold-lanes16-constl | 3126ns | 2215ns | 3757ns | -86.42% | 10.484 |
| satfold-lanes4-idx | 4023ns | 2257ns | 8394ns | -82.52% | 8.146 |
| satfold-lanes64 | 2002ns | 1769ns | 2287ns | -91.30% | 16.371 |
| satfold-neon | 2111ns | 1711ns | 2598ns | -90.83% | 15.522 |
| satfold-neon8 | 2165ns | 1883ns | 2576ns | -90.59% | 15.132 |
| satfold-nolaw | 9086ns | 8582ns | 10536ns | -60.51% | 3.606 |
| satfold-seq | 22623ns | 20325ns | 24943ns | -1.68% | 1.448 |

## Performance model

- Peak throughput: **19.151 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.422 | 7.4% |
| satfold-lanes16 | 9.789 | 51.1% |
| satfold-lanes16-constl | 10.056 | 52.5% |
| satfold-lanes4-idx | 12.436 | 64.9% |
| satfold-lanes64 | 16.654 | 87.0% |
| satfold-neon | 15.785 | 82.4% |
| satfold-neon8 | 15.516 | 81.0% |
| satfold-nolaw | 3.805 | 19.9% |
| satfold-seq | 1.448 | 7.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 23082ns | 23082ns | base |
| satfold-lanes16 | 3478ns | 3478ns | -84.93% |
| satfold-lanes16-constl | 3334ns | 3334ns | -85.56% |
| satfold-lanes4-idx | 4142ns | 4142ns | -82.06% |
| satfold-lanes64 | 2221ns | 2221ns | -90.38% |
| satfold-neon | 2339ns | 2339ns | -89.87% |
| satfold-neon8 | 2402ns | 2402ns | -89.59% |
| satfold-nolaw | 9156ns | 9156ns | -60.33% |
| satfold-seq | 22694ns | 22694ns | -1.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 23040ns | base | --- | [20704, 25304] | --- | --- | --- | --- |
| satfold-lanes16 | 3348ns | -20620.2ns (-89.5%) | [-21968, -17301]ns | [3115, 3516] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3258ns | -19715.7ns (-85.6%) | [-21704, -17908]ns | [3021, 3358] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2635ns | -18413.8ns (-79.9%) | [-22630, -17444]ns | [2571, 3277] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 1968ns | -20964.7ns (-91.0%) | [-23231, -18816]ns | [1912, 2076] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2076ns | -20442.6ns (-88.7%) | [-22926, -18762]ns | [1924, 2248] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 2112ns | -20948.0ns (-90.9%) | [-23177, -18662]ns | [1985, 2234] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 8612ns | -13458.6ns (-58.4%) | [-14783, -12113]ns | [8603, 8624] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 22625ns | -379.0ns (-1.6%) | [-397, -366]ns | [20369, 24859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25308ns | -88.0% | -86.0% | -89.5% | -92.5% | -93.0% | -91.9% | -66.0% | -1.8% |
| 1 | 2 | 20703ns | -83.3% | -86.6% | -85.2% | -89.8% | -90.1% | -88.1% | -58.4% | -1.7% |
| 1 | 3 | 25308ns | -87.6% | -86.9% | -89.7% | -92.0% | -90.3% | -91.7% | -58.4% | -1.6% |
| 1 | 4 | 20702ns | -81.6% | -84.5% | -85.8% | -90.9% | -89.8% | -90.5% | -58.5% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.750 | HIGH- (thermal bounce) |
| satfold-lanes16 | -0.397 | moderate- |
| satfold-lanes16-constl | -0.621 | HIGH- (thermal bounce) |
| satfold-lanes4-idx | -0.803 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.257 | moderate- |
| satfold-neon | 0.021 | ok |
| satfold-neon8 | -0.272 | moderate- |
| satfold-nolaw | -0.420 | moderate- |
| satfold-seq | -0.751 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.7ns | 23009.0ns | 0.1% |  |
| satfold-lanes16 | 74.2ns | 3261.6ns | 2.3% |  |
| satfold-lanes16-constl | 75.0ns | 3125.6ns | 2.4% |  |
| satfold-lanes4-idx | 36.7ns | 4022.7ns | 0.9% |  |
| satfold-lanes64 | 75.5ns | 2001.6ns | 3.8% |  |
| satfold-neon | 79.3ns | 2111.1ns | 3.8% |  |
| satfold-neon8 | 81.8ns | 2165.4ns | 3.8% |  |
| satfold-nolaw | 20.7ns | 9086.4ns | 0.2% |  |
| satfold-seq | 21.8ns | 22623.2ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 20697.8-25325.8 ns)
  20697.8 |###################################
  20929.2 |
  21160.6 |
  21392.0 |
  21623.4 |
  21854.8 |
  22086.2 |
  22317.6 |
  22549.0 |
  22780.4 |
  23011.8 |
  23243.2 |
  23474.6 |
  23706.0 |
  23937.4 |
  24168.8 |
  24400.2 |
  24631.6 |
  24863.0 |
  25094.4 |########################################
  (3 below, 1 above range)

satfold-lanes16 (n=40, range 2292.3-4001.8 ns)
   2292.3 |########
   2377.8 |
   2463.2 |
   2548.7 |
   2634.2 |########
   2719.7 |
   2805.1 |########################
   2890.6 |################
   2976.1 |################
   3061.6 |########################################
   3147.1 |########
   3232.5 |########
   3318.0 |################################
   3403.5 |########################
   3489.0 |########################
   3574.4 |########
   3659.9 |################
   3745.4 |########
   3830.9 |################################
   3916.3 |########
  (2 below, 3 above range)

satfold-lanes16-constl (n=40, range 2214.9-3757.4 ns)
   2214.9 |
   2292.0 |
   2369.1 |########
   2446.3 |
   2523.4 |################
   2600.5 |########
   2677.6 |
   2754.8 |########################################
   2831.9 |
   2909.0 |########
   2986.2 |########################
   3063.3 |################
   3140.4 |################
   3217.5 |########################################
   3294.7 |################
   3371.8 |########
   3448.9 |########
   3526.0 |########################
   3603.2 |########################################
   3680.3 |
  (2 below, 4 above range)

satfold-lanes4-idx (n=40, range 2257.4-8393.5 ns)
   2257.4 |##########################
   2564.2 |########################################
   2871.0 |
   3177.8 |######
   3484.6 |###
   3791.5 |###
   4098.3 |
   4405.1 |
   4711.9 |
   5018.7 |###
   5325.5 |
   5632.3 |
   5939.1 |######
   6245.9 |###
   6552.7 |###
   6859.5 |
   7166.3 |
   7473.1 |
   7779.9 |###
   8086.7 |###
  (5 below, 4 above range)

satfold-lanes64 (n=40, range 1768.9-2286.8 ns)
   1768.9 |####
   1794.8 |####
   1820.7 |
   1846.6 |####
   1872.5 |
   1898.4 |########################################
   1924.3 |
   1950.2 |########
   1976.1 |########
   2002.0 |
   2027.8 |####
   2053.7 |############
   2079.6 |################
   2105.5 |########
   2131.4 |
   2157.3 |
   2183.2 |
   2209.1 |
   2235.0 |####
   2260.9 |####
  (6 below, 5 above range)

satfold-neon (n=40, range 1711.0-2598.1 ns)
   1711.0 |
   1755.4 |##########################
   1799.7 |
   1844.1 |######
   1888.4 |########################################
   1932.8 |#############
   1977.1 |
   2021.5 |######
   2065.8 |##########################
   2110.2 |####################
   2154.5 |
   2198.9 |######
   2243.3 |#############
   2287.6 |#################################
   2332.0 |######
   2376.3 |######
   2420.7 |
   2465.0 |
   2509.4 |#############
   2553.7 |
  (4 below, 3 above range)

satfold-neon8 (n=40, range 1882.9-2576.4 ns)
   1882.9 |
   1917.6 |########################################
   1952.2 |###############
   1986.9 |#####
   2021.6 |
   2056.3 |
   2091.0 |########################################
   2125.6 |#####
   2160.3 |#####
   2195.0 |#####
   2229.7 |#####
   2264.3 |##########
   2299.0 |####################
   2333.7 |#####
   2368.4 |#####
   2403.1 |
   2437.7 |
   2472.4 |
   2507.1 |
   2541.8 |#####
  (3 below, 4 above range)

satfold-nolaw (n=40, range 8582.3-10536.0 ns)
   8582.3 |########################################
   8679.9 |
   8777.6 |
   8875.3 |
   8973.0 |
   9070.7 |
   9168.4 |
   9266.1 |
   9363.7 |
   9461.4 |
   9559.1 |
   9656.8 |
   9754.5 |
   9852.2 |
   9949.8 |
  10047.5 |
  10145.2 |
  10242.9 |
  10340.6 |
  10438.3 |##########
  (4 below, 3 above range)

satfold-seq (n=40, range 20324.6-24943.2 ns)
  20324.6 |#####################################
  20555.5 |
  20786.4 |
  21017.4 |
  21248.3 |
  21479.2 |
  21710.2 |
  21941.1 |
  22172.0 |
  22403.0 |
  22633.9 |
  22864.8 |
  23095.8 |
  23326.7 |
  23557.6 |
  23788.6 |
  24019.5 |
  24250.4 |
  24481.4 |
  24712.3 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=21.2% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=20.1% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=61.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.7x (possible bimodal distribution)
