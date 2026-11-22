# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.0x slower than the field

satfold-lanes4-idx (8.86 us) is 2.0x the fastest (4.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-iterfold shows alternating (throttle bounce) (autocorr -0.78)

satfold-iterfold's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-nolaw, satfold-lanes16, satfold-lanes16-constl, satfold-seq, satfold-iterfold, satfold-lanes64, satfold-neon} vs {satfold-lanes4-idx} (67% apart)

The field splits into a fast tier {satfold-nolaw, satfold-lanes16, satfold-lanes16-constl, satfold-seq, satfold-iterfold, satfold-lanes64, satfold-neon} and a slow tier {satfold-lanes4-idx} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-iterfold is inconsistent: worst-20% is 2.0x its best-20%

satfold-iterfold's best 20% of batches run at 4.24 us but its worst 20% at 8.51 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-nolaw** at 4333.0 ns median (-16.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.05x (fastest 4333.0 ns, slowest 8863.8 ns)

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
| satfold-iterfold | 5589ns | 5258ns | 4303ns | 4989ns | 8677ns | base |
| satfold-lanes16 | 5349ns | 4445ns | 4246ns | 4653ns | 8539ns | -4.30% |
| satfold-lanes16-constl | 5297ns | 5173ns | 4205ns | 4907ns | 7560ns | -5.23% |
| satfold-lanes4-idx | 9127ns | 8931ns | 8056ns | 9038ns | 10467ns | +63.30% |
| satfold-lanes64 | 5385ns | 5302ns | 4324ns | 5271ns | 6789ns | -3.65% |
| satfold-neon | 5884ns | 5377ns | 5115ns | 5394ns | 8124ns | +5.27% |
| satfold-nolaw | 5154ns | 4392ns | 4269ns | 4586ns | 7744ns | -7.78% |
| satfold-seq | 5597ns | 5256ns | 4320ns | 5029ns | 8577ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5504ns | 4243ns | 8511ns | base | 5.954 |
| satfold-lanes16 | 5267ns | 4187ns | 8376ns | -4.30% | 6.221 |
| satfold-lanes16-constl | 5217ns | 4146ns | 7423ns | -5.21% | 6.281 |
| satfold-lanes4-idx | 9056ns | 7992ns | 10388ns | +64.53% | 3.619 |
| satfold-lanes64 | 5303ns | 4265ns | 6650ns | -3.66% | 6.180 |
| satfold-neon | 5794ns | 5042ns | 7965ns | +5.28% | 5.655 |
| satfold-nolaw | 5075ns | 4210ns | 7606ns | -7.80% | 6.457 |
| satfold-seq | 5512ns | 4261ns | 8414ns | +0.14% | 5.945 |

## Performance model

- Peak throughput: **7.903 Gops/s** (satfold-lanes16-constl; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.319 | 80.0% |
| satfold-lanes16 | 7.471 | 94.5% |
| satfold-lanes16-constl | 6.422 | 81.3% |
| satfold-lanes4-idx | 3.697 | 46.8% |
| satfold-lanes64 | 6.266 | 79.3% |
| satfold-neon | 6.177 | 78.2% |
| satfold-nolaw | 7.563 | 95.7% |
| satfold-seq | 6.321 | 80.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5589ns | 5589ns | base |
| satfold-lanes16 | 5349ns | 5349ns | -4.30% |
| satfold-lanes16-constl | 5297ns | 5297ns | -5.23% |
| satfold-lanes4-idx | 9127ns | 9127ns | +63.30% |
| satfold-lanes64 | 5385ns | 5385ns | -3.65% |
| satfold-neon | 5884ns | 5884ns | +5.27% |
| satfold-nolaw | 5154ns | 5154ns | -7.78% |
| satfold-seq | 5597ns | 5597ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5185ns | base | --- | [4285, 5345] | --- | --- | --- | --- |
| satfold-lanes16 | 4386ns | no significant difference | [-192, +34]ns | [4307, 4850] | no | 0.6014 | 0.4296 | 0 |
| satfold-lanes16-constl | 5102ns | no significant difference | [-300, +139]ns | [4456, 5208] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes4-idx | 8864ns | +3811.3ns (+73.5%) | [+3390, +4132]ns | [8218, 9656] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5230ns | no significant difference | [-100, +108]ns | [5145, 5300] | no | 0.8746 | 0.8746 | 0 |
| satfold-neon | 5305ns | no significant difference | [-88, +967]ns | [5249, 5416] | no | 0.6014 | 0.4296 | 0 |
| satfold-nolaw | 4333ns | no significant difference | [-338, +6]ns | [4301, 4622] | no | 0.2824 | 0.0807 | 0 |
| satfold-seq | 5184ns | no significant difference | [-34, +145]ns | [4466, 5280] | no | 0.6014 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4249ns | +1.8% | +22.6% | +144.2% | +25.4% | +25.9% | -0.3% | +3.9% |
| 1 | 2 | 5505ns | -1.9% | -3.1% | +74.6% | -6.0% | -3.8% | -5.5% | -4.6% |
| 1 | 3 | 4282ns | +1.3% | +2.9% | +86.8% | -0.2% | +25.4% | +1.2% | +0.3% |
| 1 | 4 | 5195ns | -17.3% | -20.2% | +58.2% | +1.9% | +0.7% | -17.7% | +4.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.784 | HIGH- (thermal bounce) |
| satfold-lanes16 | -0.392 | moderate- |
| satfold-lanes16-constl | 0.260 | moderate+ |
| satfold-lanes4-idx | 0.266 | moderate+ |
| satfold-lanes64 | -0.366 | moderate- |
| satfold-neon | -0.558 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.422 | moderate- |
| satfold-seq | -0.716 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-lanes16**: won 2/4, lost 2/4
- **satfold-lanes16-constl**: won 2/4, lost 2/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 2/4, lost 2/4
- **satfold-neon**: won 1/4, lost 3/4
- **satfold-nolaw**: won 3/4, lost 1/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 25.2ns | 5503.9ns | 0.5% |  |
| satfold-lanes16 | 24.1ns | 5267.0ns | 0.5% |  |
| satfold-lanes16-constl | 23.7ns | 5216.9ns | 0.5% |  |
| satfold-lanes4-idx | 22.5ns | 9055.7ns | 0.2% |  |
| satfold-lanes64 | 24.6ns | 5302.7ns | 0.5% |  |
| satfold-neon | 26.9ns | 5794.4ns | 0.5% |  |
| satfold-nolaw | 23.4ns | 5074.6ns | 0.5% |  |
| satfold-seq | 25.3ns | 5511.7ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4243.3-8511.3 ns)
   4243.3 |########################################
   4456.7 |
   4670.1 |
   4883.5 |
   5096.9 |###########################
   5310.3 |########################
   5523.7 |###
   5737.1 |
   5950.5 |
   6163.9 |
   6377.3 |
   6590.7 |
   6804.1 |
   7017.5 |
   7230.9 |
   7444.3 |###
   7657.7 |
   7871.1 |
   8084.5 |
   8297.9 |
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 4187.4-8376.2 ns)
   4187.4 |########################################
   4396.9 |###############
   4606.3 |
   4815.8 |
   5025.2 |##
   5234.6 |##########
   5444.1 |############
   5653.5 |
   5863.0 |
   6072.4 |
   6281.8 |
   6491.3 |
   6700.7 |
   6910.2 |
   7119.6 |
   7329.0 |##
   7538.5 |
   7747.9 |
   7957.4 |
   8166.8 |
  (4 below, 3 above range)

satfold-lanes16-constl (n=40, range 4146.4-7422.7 ns)
   4146.4 |###################################
   4310.2 |#########################
   4474.0 |###############
   4637.8 |
   4801.6 |
   4965.4 |###############
   5129.2 |########################################
   5293.1 |#########################
   5456.9 |##########
   5620.7 |
   5784.5 |
   5948.3 |
   6112.1 |
   6276.0 |
   6439.8 |
   6603.6 |
   6767.4 |
   6931.2 |
   7095.0 |
   7258.9 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 7991.6-10387.7 ns)
   7991.6 |#################
   8111.4 |########################################
   8231.2 |#############
   8351.0 |
   8470.8 |
   8590.6 |
   8710.4 |
   8830.2 |
   8950.0 |
   9069.8 |
   9189.6 |
   9309.4 |####
   9429.2 |########
   9549.0 |#############
   9668.9 |#################
   9788.7 |
   9908.5 |
  10028.3 |####
  10148.1 |####
  10267.9 |#################
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 4265.4-6650.4 ns)
   4265.4 |#######
   4384.6 |#######
   4503.9 |
   4623.1 |
   4742.4 |
   4861.6 |
   4980.9 |#######
   5100.1 |#############################
   5219.4 |########################################
   5338.6 |##########
   5457.9 |#######
   5577.1 |###
   5696.4 |
   5815.6 |
   5934.9 |
   6054.1 |
   6173.4 |
   6292.6 |###
   6411.9 |
   6531.1 |
  (5 below, 3 above range)

satfold-neon (n=40, range 5042.1-7964.9 ns)
   5042.1 |#######################
   5188.3 |########################################
   5334.4 |####################################
   5480.5 |##########
   5626.7 |
   5772.8 |
   5919.0 |
   6065.1 |
   6211.3 |
   6357.4 |
   6503.5 |
   6649.7 |
   6795.8 |
   6942.0 |
   7088.1 |
   7234.2 |
   7380.4 |
   7526.5 |
   7672.7 |
   7818.8 |
  (3 below, 4 above range)

satfold-nolaw (n=40, range 4210.2-7605.6 ns)
   4210.2 |########################################
   4380.0 |
   4549.8 |###
   4719.5 |
   4889.3 |
   5059.1 |###########
   5228.8 |#
   5398.6 |###
   5568.4 |#
   5738.1 |
   5907.9 |
   6077.7 |
   6247.4 |
   6417.2 |
   6587.0 |
   6756.7 |
   6926.5 |
   7096.3 |
   7266.0 |
   7435.8 |
  (4 below, 3 above range)

satfold-seq (n=40, range 4260.9-8414.4 ns)
   4260.9 |########################################
   4468.6 |#######
   4676.3 |###
   4883.9 |###
   5091.6 |#############################
   5299.3 |#####################
   5507.0 |##########
   5714.6 |###
   5922.3 |
   6130.0 |###
   6337.7 |
   6545.3 |
   6753.0 |
   6960.7 |
   7168.4 |
   7376.1 |
   7583.7 |
   7791.4 |
   7999.1 |
   8206.8 |
  (3 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=36.5% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=41.1% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=31.6% (high variance, measurements may be unstable)
- **satfold-neon**: CV=27.2% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=36.9% (high variance, measurements may be unstable)
- **satfold-seq**: CV=39.0% (high variance, measurements may be unstable)
