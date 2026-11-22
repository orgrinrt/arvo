# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx beats baseline by 87% (significant)

satfold-lanes4-idx is -12.28 us (87%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 8.5x slower than the field

satfold-seq (15.61 us) is 8.5x the fastest (1.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 123.3%)

satfold-lanes4-idx wins on median (1.84 us) yet has the highest variance (CV 123.3%), while satfold-nolaw is the steadiest (CV 8.0%, 7.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes16 shows alternating (throttle bounce) (autocorr -0.74)

satfold-lanes16's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.5x the fastest

Fastest satfold-lanes4-idx (1.84 us) to slowest satfold-seq (15.61 us): 8.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes64 is inconsistent: worst-20% is 4.1x its best-20%

satfold-lanes64's best 20% of batches run at 1.62 us but its worst 20% at 6.67 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes4-idx** at 1838.0 ns median (-86.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 8.49x (fastest 1838.0 ns, slowest 15609.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112779582 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 14920ns | 14133ns | 14120ns | 14398ns | 17283ns | base |
| satfold-lanes16 | 4258ns | 3198ns | 2618ns | 3141ns | 9248ns | -71.46% |
| satfold-lanes16-constl | 4487ns | 3181ns | 2598ns | 3462ns | 9449ns | -69.93% |
| satfold-lanes4-idx | 3871ns | 1891ns | 1827ns | 3435ns | 7224ns | -74.05% |
| satfold-lanes64 | 3745ns | 2067ns | 1680ns | 3358ns | 6968ns | -74.90% |
| satfold-neon | 3765ns | 2407ns | 1686ns | 3462ns | 6753ns | -74.76% |
| satfold-nolaw | 7150ns | 7480ns | 6113ns | 7372ns | 7519ns | -52.08% |
| satfold-seq | 16111ns | 15693ns | 14100ns | 15696ns | 19364ns | +7.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 14852ns | 14059ns | 17204ns | base | 2.206 |
| satfold-lanes16 | 4163ns | 2566ns | 9012ns | -71.97% | 7.871 |
| satfold-lanes16-constl | 4387ns | 2546ns | 9210ns | -70.46% | 7.469 |
| satfold-lanes4-idx | 3752ns | 1776ns | 6980ns | -74.74% | 8.734 |
| satfold-lanes64 | 3608ns | 1625ns | 6666ns | -75.71% | 9.083 |
| satfold-neon | 3633ns | 1632ns | 6493ns | -75.54% | 9.019 |
| satfold-nolaw | 7075ns | 6053ns | 7430ns | -52.36% | 4.631 |
| satfold-seq | 16037ns | 14040ns | 19274ns | +7.98% | 2.043 |

## Performance model

- Peak throughput: **20.165 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.329 | 11.5% |
| satfold-lanes16 | 10.453 | 51.8% |
| satfold-lanes16-constl | 10.511 | 52.1% |
| satfold-lanes4-idx | 17.828 | 88.4% |
| satfold-lanes64 | 16.378 | 81.2% |
| satfold-neon | 14.068 | 69.8% |
| satfold-nolaw | 4.424 | 21.9% |
| satfold-seq | 2.099 | 10.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 14920ns | 14920ns | base |
| satfold-lanes16 | 4258ns | 4258ns | -71.46% |
| satfold-lanes16-constl | 4487ns | 4487ns | -69.93% |
| satfold-lanes4-idx | 3871ns | 3871ns | -74.05% |
| satfold-lanes64 | 3745ns | 3745ns | -74.90% |
| satfold-neon | 3765ns | 3765ns | -74.76% |
| satfold-nolaw | 7150ns | 7150ns | -52.08% |
| satfold-seq | 16111ns | 16111ns | +7.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 14070ns | base | --- | [14068, 14077] | --- | --- | --- | --- |
| satfold-lanes16 | 3135ns | -11492.9ns (-81.7%) | [-11503, -10932]ns | [2857, 3148] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3118ns | -11507.6ns (-81.8%) | [-11519, -10957]ns | [2835, 3126] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1838ns | -12280.4ns (-87.3%) | [-12291, -9921]ns | [1776, 5560] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2001ns | -12061.6ns (-85.7%) | [-12263, -10792]ns | [1980, 5270] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2329ns | -12072.2ns (-85.8%) | [-12380, -9848]ns | [1706, 5168] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7407ns | -7345.2ns (-52.2%) | [-8016, -6667]ns | [7400, 7419] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 15610ns | no significant difference | [-15, +3116]ns | [14060, 17194] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17202ns | -81.7% | -81.8% | -89.3% | -78.6% | -90.4% | -56.8% | -18.3% |
| 1 | 2 | 14068ns | -81.7% | -81.8% | -78.2% | -69.2% | -69.8% | -47.4% | -0.1% |
| 1 | 3 | 14067ns | -77.7% | -81.9% | -80.8% | -85.8% | -83.4% | -47.3% | +22.3% |
| 1 | 4 | 14071ns | -81.8% | -77.8% | -86.3% | -82.9% | -76.6% | -57.0% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.083 | ok |
| satfold-lanes16 | -0.742 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.250 | moderate- |
| satfold-lanes4-idx | -0.279 | moderate- |
| satfold-lanes64 | 0.034 | ok |
| satfold-neon | -0.690 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.089 | ok |
| satfold-seq | 0.250 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 20.8ns | 14851.9ns | 0.1% |  |
| satfold-lanes16 | 31.6ns | 4163.3ns | 0.8% |  |
| satfold-lanes16-constl | 33.5ns | 4387.4ns | 0.8% |  |
| satfold-lanes4-idx | 42.2ns | 3751.9ns | 1.1% |  |
| satfold-lanes64 | 42.8ns | 3607.7ns | 1.2% |  |
| satfold-neon | 43.2ns | 3633.3ns | 1.2% |  |
| satfold-nolaw | 22.7ns | 7075.3ns | 0.3% |  |
| satfold-seq | 21.7ns | 16036.8ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14058.9-17204.2 ns)
  14058.9 |########################################
  14216.2 |
  14373.4 |
  14530.7 |
  14688.0 |
  14845.2 |
  15002.5 |
  15159.8 |
  15317.0 |
  15474.3 |
  15631.6 |
  15788.8 |
  15946.1 |
  16103.4 |
  16260.6 |
  16417.9 |
  16575.2 |
  16732.4 |
  16889.7 |
  17047.0 |########
  (5 below, 5 above range)

satfold-lanes16 (n=40, range 2566.2-9012.0 ns)
   2566.2 |####################
   2888.5 |########################################
   3210.8 |
   3533.0 |##
   3855.3 |
   4177.6 |##
   4499.9 |
   4822.2 |
   5144.5 |
   5466.8 |##
   5789.1 |
   6111.4 |
   6433.7 |
   6755.9 |
   7078.2 |
   7400.5 |##
   7722.8 |##
   8045.1 |
   8367.4 |#####
   8689.7 |
  (6 below, 3 above range)

satfold-lanes16-constl (n=40, range 2546.4-9210.5 ns)
   2546.4 |#########################
   2879.6 |########################################
   3212.8 |
   3546.0 |
   3879.2 |
   4212.4 |##
   4545.6 |
   4878.8 |
   5212.0 |
   5545.2 |#####
   5878.4 |
   6211.6 |
   6544.8 |
   6878.1 |##
   7211.3 |
   7544.5 |
   7877.7 |##
   8210.9 |
   8544.1 |#####
   8877.3 |
  (5 below, 5 above range)

satfold-lanes4-idx (n=40, range 1775.8-6980.4 ns)
   1775.8 |########################################
   2036.1 |##
   2296.3 |
   2556.5 |
   2816.8 |
   3077.0 |
   3337.2 |
   3597.4 |##
   3857.7 |
   4117.9 |##
   4378.1 |
   4638.4 |
   4898.6 |
   5158.8 |##
   5419.0 |####
   5679.3 |##
   5939.5 |######
   6199.7 |##
   6460.0 |######
   6720.2 |##
  (3 below, 4 above range)

satfold-lanes64 (n=40, range 1625.0-6665.6 ns)
   1625.0 |########################################
   1877.0 |########################################
   2129.1 |
   2381.1 |
   2633.1 |
   2885.1 |
   3137.2 |#####
   3389.2 |
   3641.2 |#####
   3893.3 |#####
   4145.3 |
   4397.3 |
   4649.4 |
   4901.4 |##########
   5153.4 |#####
   5405.5 |#########################
   5657.5 |
   5909.5 |#########################
   6161.6 |
   6413.6 |#####
  (5 below, 2 above range)

satfold-neon (n=40, range 1631.8-6493.0 ns)
   1631.8 |########################################
   1874.9 |##################
   2117.9 |
   2361.0 |
   2604.0 |###
   2847.1 |
   3090.2 |
   3333.2 |
   3576.3 |###
   3819.3 |
   4062.4 |
   4305.5 |
   4548.5 |
   4791.6 |##########
   5034.6 |#######
   5277.7 |#######
   5520.8 |#######
   5763.8 |#######
   6006.9 |#######
   6249.9 |###
  (4 below, 4 above range)

satfold-nolaw (n=40, range 6052.5-7430.3 ns)
   6052.5 |########
   6121.4 |
   6190.3 |
   6259.2 |
   6328.1 |
   6397.0 |
   6465.8 |
   6534.7 |
   6603.6 |
   6672.5 |
   6741.4 |
   6810.3 |
   6879.2 |
   6948.0 |
   7016.9 |
   7085.8 |
   7154.7 |
   7223.6 |
   7292.5 |
   7361.4 |########################################
  (4 below, 3 above range)

satfold-seq (n=40, range 14039.7-19273.8 ns)
  14039.7 |########################################
  14301.4 |
  14563.1 |
  14824.8 |
  15086.5 |
  15348.2 |
  15609.9 |
  15871.6 |
  16133.3 |
  16395.0 |
  16656.7 |
  16918.4 |#########
  17180.1 |################################
  17441.8 |
  17703.5 |
  17965.2 |
  18227.0 |
  18488.7 |
  18750.4 |
  19012.1 |
  (3 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=62.7% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=59.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=60.4% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **satfold-lanes64**: CV=58.8% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **satfold-neon**: CV=56.4% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 4.0x (possible bimodal distribution)
