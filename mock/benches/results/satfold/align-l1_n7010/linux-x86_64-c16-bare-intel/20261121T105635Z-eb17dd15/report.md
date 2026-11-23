# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 77% (significant)

satfold-neon is -12.08 us (77%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 7.8x slower than the field

satfold-seq (15.65 us) is 7.8x the fastest (2.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.8x the fastest

Fastest satfold-neon (2.01 us) to slowest satfold-seq (15.65 us): 7.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.0x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.07 us but its worst 20% at 8.28 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-neon** at 2007.3 ns median (-87.2% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 7.80x (fastest 2007.3 ns, slowest 15650.5 ns)

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
| Counter frequency | 2112835759 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 15737ns | 15719ns | 14119ns | 15707ns | 17443ns | base |
| satfold-lanes16 | 4281ns | 2688ns | 2666ns | 3137ns | 9327ns | -72.80% |
| satfold-lanes16-constl | 4836ns | 3191ns | 2818ns | 3842ns | 9835ns | -69.27% |
| satfold-lanes4-idx | 4080ns | 2583ns | 2131ns | 3230ns | 8582ns | -74.07% |
| satfold-lanes64 | 4056ns | 2175ns | 1821ns | 3802ns | 7053ns | -74.23% |
| satfold-neon | 3518ns | 2079ns | 1690ns | 3279ns | 6061ns | -77.65% |
| satfold-nolaw | 7200ns | 7477ns | 6132ns | 7426ns | 7590ns | -54.25% |
| satfold-seq | 15717ns | 15730ns | 14121ns | 15719ns | 17306ns | -0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 15662ns | 14058ns | 17352ns | base | 2.092 |
| satfold-lanes16 | 4185ns | 2614ns | 9084ns | -73.28% | 7.830 |
| satfold-lanes16-constl | 4728ns | 2762ns | 9581ns | -69.81% | 6.930 |
| satfold-lanes4-idx | 3959ns | 2071ns | 8277ns | -74.72% | 8.278 |
| satfold-lanes64 | 3914ns | 1763ns | 6773ns | -75.01% | 8.371 |
| satfold-neon | 3398ns | 1636ns | 5832ns | -78.30% | 9.643 |
| satfold-nolaw | 7124ns | 6073ns | 7495ns | -54.51% | 4.599 |
| satfold-seq | 15645ns | 14061ns | 17223ns | -0.11% | 2.094 |

## Performance model

- Peak throughput: **20.028 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.097 | 10.5% |
| satfold-lanes16 | 12.438 | 62.1% |
| satfold-lanes16-constl | 10.478 | 52.3% |
| satfold-lanes4-idx | 12.984 | 64.8% |
| satfold-lanes64 | 15.554 | 77.7% |
| satfold-neon | 16.324 | 81.5% |
| satfold-nolaw | 4.425 | 22.1% |
| satfold-seq | 2.094 | 10.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 15737ns | 15737ns | base |
| satfold-lanes16 | 4281ns | 4281ns | -72.80% |
| satfold-lanes16-constl | 4836ns | 4836ns | -69.27% |
| satfold-lanes4-idx | 4080ns | 4080ns | -74.07% |
| satfold-lanes64 | 4056ns | 4056ns | -74.23% |
| satfold-neon | 3518ns | 3518ns | -77.65% |
| satfold-nolaw | 7200ns | 7200ns | -54.25% |
| satfold-seq | 15717ns | 15717ns | -0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 15626ns | base | --- | [14078, 17193] | --- | --- | --- | --- |
| satfold-lanes16 | 2635ns | -11465.4ns (-73.4%) | [-13959, -11440]ns | [2624, 3213] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3127ns | -10947.3ns (-70.1%) | [-11089, -10935]ns | [3115, 3471] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2524ns | -11554.8ns (-73.9%) | [-13185, -11535]ns | [2518, 3078] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2107ns | -12074.9ns (-77.3%) | [-12445, -10987]ns | [1994, 5836] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 2007ns | -12075.0ns (-77.3%) | [-12422, -12061]ns | [1996, 4935] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7405ns | -8042.7ns (-51.5%) | [-9779, -6676]ns | [7397, 7412] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 15650ns | no significant difference | [-21, +38]ns | [14094, 17202] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 17215ns | -84.8% | -80.8% | -82.2% | -88.4% | -71.6% | -56.8% | -18.2% |
| 1 | 2 | 14066ns | -81.3% | -77.8% | -82.1% | -86.3% | -85.8% | -47.4% | +0.1% |
| 1 | 3 | 14078ns | -81.4% | -77.7% | -81.6% | -76.4% | -76.2% | -47.4% | +22.2% |
| 1 | 4 | 17185ns | -81.3% | -81.8% | -82.5% | -71.4% | -72.0% | -64.6% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.250 | moderate- |
| satfold-lanes16 | -0.087 | ok |
| satfold-lanes16-constl | -0.118 | ok |
| satfold-lanes4-idx | -0.259 | moderate- |
| satfold-lanes64 | 0.233 | moderate+ |
| satfold-neon | -0.299 | moderate- |
| satfold-nolaw | -0.081 | ok |
| satfold-seq | 0.250 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 21.2ns | 15662.0ns | 0.1% |  |
| satfold-lanes16 | 30.2ns | 4184.7ns | 0.7% |  |
| satfold-lanes16-constl | 35.0ns | 4728.1ns | 0.7% |  |
| satfold-lanes4-idx | 37.0ns | 3958.6ns | 0.9% |  |
| satfold-lanes64 | 45.3ns | 3914.4ns | 1.2% |  |
| satfold-neon | 39.3ns | 3398.0ns | 1.2% |  |
| satfold-nolaw | 22.5ns | 7124.3ns | 0.3% |  |
| satfold-seq | 21.3ns | 15645.3ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14057.5-17352.1 ns)
  14057.5 |########################################
  14222.2 |
  14387.0 |
  14551.7 |
  14716.4 |
  14881.2 |
  15045.9 |
  15210.6 |
  15375.3 |
  15540.1 |
  15704.8 |
  15869.5 |
  16034.3 |
  16199.0 |
  16363.7 |
  16528.5 |
  16693.2 |
  16857.9 |
  17022.6 |##############
  17187.4 |##############################
  (3 below, 1 above range)

satfold-lanes16 (n=40, range 2614.0-9084.2 ns)
   2614.0 |########################################
   2937.5 |##################
   3261.0 |
   3584.5 |##
   3908.0 |
   4231.5 |
   4555.1 |
   4878.6 |####
   5202.1 |
   5525.6 |
   5849.1 |##
   6172.6 |
   6496.1 |
   6819.6 |
   7143.2 |
   7466.7 |
   7790.2 |##
   8113.7 |
   8437.2 |##
   8760.7 |##
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 2762.5-9581.1 ns)
   2762.5 |
   3103.4 |########################################
   3444.4 |
   3785.3 |###
   4126.2 |
   4467.2 |
   4808.1 |
   5149.0 |
   5489.9 |
   5830.9 |#
   6171.8 |#
   6512.7 |
   6853.7 |###
   7194.6 |#
   7535.5 |#
   7876.5 |
   8217.4 |
   8558.3 |#
   8899.3 |
   9240.2 |
  (5 below, 5 above range)

satfold-lanes4-idx (n=40, range 2070.6-8276.6 ns)
   2070.6 |#######################
   2380.9 |########################################
   2691.2 |
   3001.5 |################
   3311.8 |###
   3622.1 |###
   3932.4 |
   4242.7 |
   4553.0 |
   4863.3 |
   5173.6 |
   5483.9 |###
   5794.2 |
   6104.5 |###
   6414.8 |######
   6725.1 |###
   7035.4 |######
   7345.7 |
   7656.0 |
   7966.3 |###
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 1762.8-6773.1 ns)
   1762.8 |########################################
   2013.3 |##
   2263.8 |
   2514.3 |
   2764.8 |
   3015.3 |
   3265.8 |
   3516.4 |
   3766.9 |
   4017.4 |
   4267.9 |##
   4518.4 |##
   4768.9 |
   5019.5 |
   5270.0 |
   5520.5 |########
   5771.0 |########
   6021.5 |########
   6272.0 |##
   6522.5 |##########
  (5 below, 3 above range)

satfold-neon (n=40, range 1636.1-5831.5 ns)
   1636.1 |#####
   1845.8 |########################################
   2055.6 |
   2265.4 |
   2475.2 |
   2684.9 |##
   2894.7 |
   3104.5 |
   3314.2 |
   3524.0 |
   3733.8 |
   3943.6 |
   4153.3 |
   4363.1 |
   4572.9 |##
   4782.6 |#####
   4992.4 |###########
   5202.2 |##
   5412.0 |########
   5621.7 |########
  (6 below, 3 above range)

satfold-nolaw (n=40, range 6073.1-7494.5 ns)
   6073.1 |###########
   6144.2 |
   6215.2 |
   6286.3 |
   6357.4 |
   6428.4 |
   6499.5 |
   6570.6 |
   6641.7 |
   6712.7 |
   6783.8 |
   6854.9 |
   6925.9 |
   6997.0 |
   7068.1 |
   7139.2 |
   7210.2 |
   7281.3 |
   7352.4 |########################################
   7423.4 |#################
  (3 below, 1 above range)

satfold-seq (n=40, range 14060.7-17223.0 ns)
  14060.7 |########################################
  14218.8 |
  14376.9 |
  14535.0 |
  14693.1 |
  14851.2 |
  15009.4 |
  15167.5 |
  15325.6 |
  15483.7 |
  15641.8 |
  15799.9 |
  15958.0 |
  16116.2 |
  16274.3 |
  16432.4 |
  16590.5 |
  16748.6 |
  16906.7 |
  17064.9 |###################################
  (2 below, 4 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=62.7% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=57.9% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=62.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **satfold-lanes64**: CV=55.3% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **satfold-neon**: CV=52.2% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.6x (possible bimodal distribution)
