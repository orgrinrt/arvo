# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (17.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes4-idx at 2.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes4-idx dominates: 44% faster than the next best (satfold-lanes16-constl)

satfold-lanes4-idx (2.18 us) leads satfold-lanes16-constl (3.13 us) by 44%, a clear separation rather than a photo finish. CV 118.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes16-constl beats baseline by 82% (significant)

satfold-lanes16-constl is -14.07 us (82%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 7.9x slower than the field

satfold-iterfold (17.20 us) is 7.9x the fastest (2.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 118.2%)

satfold-lanes4-idx wins on median (2.18 us) yet has the highest variance (CV 118.2%), while satfold-iterfold is the steadiest (CV 7.9%, 17.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {satfold-lanes4-idx, satfold-lanes16-constl, satfold-lanes16, satfold-lanes64, satfold-neon, satfold-nolaw} vs {satfold-seq, satfold-iterfold} (132% apart)

The field splits into a fast tier {satfold-lanes4-idx, satfold-lanes16-constl, satfold-lanes16, satfold-lanes64, satfold-neon, satfold-nolaw} and a slow tier {satfold-seq, satfold-iterfold} with a 132% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.9x the fastest

Fastest satfold-lanes4-idx (2.18 us) to slowest satfold-iterfold (17.20 us): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 3.7x its best-20%

satfold-lanes4-idx's best 20% of batches run at 2.17 us but its worst 20% at 8.09 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-seq's edge over baseline is significant but tiny (-17 ns, 0.10%)

satfold-seq differs from baseline satfold-iterfold by -17 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2176.1 ns median (-87.3% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 7.90x (fastest 2176.1 ns, slowest 17201.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 16496ns | 17276ns | 14128ns | 17015ns | 17305ns | base |
| satfold-lanes16 | 4729ns | 3214ns | 3205ns | 3332ns | 10445ns | -71.33% |
| satfold-lanes16-constl | 4914ns | 3198ns | 3173ns | 3658ns | 10420ns | -70.21% |
| satfold-lanes4-idx | 4225ns | 2242ns | 2232ns | 3497ns | 8405ns | -74.39% |
| satfold-lanes64 | 4303ns | 5120ns | 2057ns | 4052ns | 7303ns | -73.91% |
| satfold-neon | 4308ns | 5123ns | 2056ns | 4002ns | 7479ns | -73.88% |
| satfold-nolaw | 7169ns | 7487ns | 6115ns | 7381ns | 7589ns | -56.54% |
| satfold-seq | 16480ns | 17254ns | 14103ns | 16998ns | 17304ns | -0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 16420ns | 14067ns | 17215ns | base | 1.996 |
| satfold-lanes16 | 4623ns | 3143ns | 10176ns | -71.84% | 7.087 |
| satfold-lanes16-constl | 4805ns | 3110ns | 10156ns | -70.74% | 6.820 |
| satfold-lanes4-idx | 4091ns | 2168ns | 8091ns | -75.09% | 8.010 |
| satfold-lanes64 | 4158ns | 1991ns | 7010ns | -74.67% | 7.880 |
| satfold-neon | 4162ns | 1991ns | 7174ns | -74.65% | 7.873 |
| satfold-nolaw | 7095ns | 6055ns | 7507ns | -56.79% | 4.619 |
| satfold-seq | 16402ns | 14043ns | 17205ns | -0.11% | 1.998 |

## Performance model

- Peak throughput: **16.462 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.905 | 11.6% |
| satfold-lanes16 | 10.407 | 63.2% |
| satfold-lanes16-constl | 10.454 | 63.5% |
| satfold-lanes4-idx | 15.058 | 91.5% |
| satfold-lanes64 | 6.591 | 40.0% |
| satfold-neon | 6.587 | 40.0% |
| satfold-nolaw | 4.420 | 26.9% |
| satfold-seq | 1.907 | 11.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 16496ns | 16496ns | base |
| satfold-lanes16 | 4729ns | 4729ns | -71.33% |
| satfold-lanes16-constl | 4914ns | 4914ns | -70.21% |
| satfold-lanes4-idx | 4225ns | 4225ns | -74.39% |
| satfold-lanes64 | 4303ns | 4303ns | -73.91% |
| satfold-neon | 4308ns | 4308ns | -73.88% |
| satfold-nolaw | 7169ns | 7169ns | -56.54% |
| satfold-seq | 16480ns | 16480ns | -0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 17201ns | base | --- | [17190, 17205] | --- | --- | --- | --- |
| satfold-lanes16 | 3149ns | -14034.8ns (-81.6%) | [-14049, -10926]ns | [3147, 3161] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 3135ns | -14067.5ns (-81.8%) | [-14080, -10935]ns | [3122, 3139] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2176ns | -12459.7ns (-72.4%) | [-15026, -11889]ns | [2171, 4089] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 4972ns | -11936.2ns (-69.4%) | [-15178, -10138]ns | [2016, 5182] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 4975ns | -11954.8ns (-69.5%) | [-15186, -9985]ns | [2016, 5182] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 7414ns | -9788.2ns (-56.9%) | [-9809, -9772]ns | [7402, 7426] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 17181ns | -17.3ns (-0.1%) | [-28, -12]ns | [17172, 17189] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14071ns | -77.6% | -77.6% | -84.5% | -63.2% | -63.2% | -47.1% | -0.2% |
| 1 | 2 | 17205ns | -81.7% | -81.8% | -87.4% | -84.4% | -84.8% | -57.0% | -0.1% |
| 1 | 3 | 17205ns | -81.7% | -81.9% | -87.3% | -88.2% | -85.4% | -64.8% | -0.1% |
| 1 | 4 | 17202ns | -81.7% | -81.8% | -87.3% | -83.1% | -88.3% | -56.9% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.084 | ok |
| satfold-lanes16 | -0.251 | moderate- |
| satfold-lanes16-constl | 0.064 | ok |
| satfold-lanes4-idx | 0.007 | ok |
| satfold-lanes64 | -0.013 | ok |
| satfold-neon | -0.019 | ok |
| satfold-nolaw | -0.399 | moderate- |
| satfold-seq | -0.082 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.4ns | 16419.8ns | 0.1% |  |
| satfold-lanes16 | 34.8ns | 4623.4ns | 0.8% |  |
| satfold-lanes16-constl | 36.3ns | 4804.8ns | 0.8% |  |
| satfold-lanes4-idx | 44.8ns | 4090.9ns | 1.1% |  |
| satfold-lanes64 | 44.5ns | 4158.4ns | 1.1% |  |
| satfold-neon | 44.1ns | 4161.9ns | 1.1% |  |
| satfold-nolaw | 22.8ns | 7094.8ns | 0.3% |  |
| satfold-seq | 22.7ns | 16402.3ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14066.8-17215.3 ns)
  14066.8 |#########
  14224.2 |
  14381.6 |
  14539.1 |
  14696.5 |
  14853.9 |
  15011.3 |
  15168.8 |
  15326.2 |
  15483.6 |
  15641.0 |
  15798.5 |
  15955.9 |
  16113.3 |
  16270.8 |
  16428.2 |
  16585.6 |
  16743.0 |
  16900.5 |
  17057.9 |########################################
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 3142.7-10175.7 ns)
   3142.7 |########################################
   3494.4 |#
   3846.0 |#
   4197.7 |#
   4549.3 |
   4901.0 |
   5252.6 |
   5604.3 |
   5955.9 |
   6307.6 |
   6659.2 |
   7010.8 |
   7362.5 |
   7714.1 |
   8065.8 |
   8417.4 |
   8769.1 |#
   9120.7 |#
   9472.4 |#
   9824.0 |#
  (5 below, 4 above range)

satfold-lanes16-constl (n=40, range 3109.9-10155.6 ns)
   3109.9 |########################################
   3462.2 |
   3814.5 |
   4166.8 |
   4519.1 |
   4871.4 |
   5223.6 |#
   5575.9 |
   5928.2 |#
   6280.5 |
   6632.8 |
   6985.1 |
   7337.4 |
   7689.6 |
   8041.9 |
   8394.2 |#
   8746.5 |
   9098.8 |###
   9451.1 |###
   9803.3 |
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 2168.5-8091.4 ns)
   2168.5 |########################################
   2464.6 |
   2760.8 |
   3056.9 |
   3353.1 |##
   3649.2 |##
   3945.3 |
   4241.5 |##
   4537.6 |
   4833.8 |
   5129.9 |##
   5426.1 |
   5722.2 |
   6018.3 |
   6314.5 |
   6610.6 |
   6906.8 |##
   7202.9 |##
   7499.1 |##
   7795.2 |############
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 1991.5-7010.1 ns)
   1991.5 |########################################
   2242.4 |
   2493.3 |
   2744.2 |
   2995.2 |
   3246.1 |###
   3497.0 |
   3748.0 |###
   3998.9 |
   4249.8 |
   4500.8 |
   4751.7 |###
   5002.6 |###########################
   5253.5 |
   5504.5 |
   5755.4 |#########
   6006.3 |###
   6257.3 |
   6508.2 |######
   6759.1 |###
  (4 below, 4 above range)

satfold-neon (n=40, range 1990.6-7174.0 ns)
   1990.6 |########################################
   2249.7 |
   2508.9 |
   2768.1 |
   3027.3 |######
   3286.4 |
   3545.6 |
   3804.8 |
   4063.9 |
   4323.1 |
   4582.3 |###
   4841.5 |
   5100.6 |###########################
   5359.8 |
   5619.0 |######
   5878.1 |###
   6137.3 |
   6396.5 |###
   6655.6 |###
   6914.8 |###
  (4 below, 5 above range)

satfold-nolaw (n=40, range 6054.7-7507.2 ns)
   6054.7 |#############
   6127.4 |
   6200.0 |
   6272.6 |
   6345.2 |
   6417.9 |
   6490.5 |
   6563.1 |
   6635.7 |
   6708.3 |
   6781.0 |
   6853.6 |
   6926.2 |
   6998.8 |
   7071.5 |
   7144.1 |
   7216.7 |
   7289.3 |
   7362.0 |########################################
   7434.6 |######################
  (4 below, 2 above range)

satfold-seq (n=40, range 14043.1-17204.5 ns)
  14043.1 |#######
  14201.2 |
  14359.3 |
  14517.3 |
  14675.4 |
  14833.5 |
  14991.5 |
  15149.6 |
  15307.7 |
  15465.7 |
  15623.8 |
  15781.9 |
  15940.0 |
  16098.0 |
  16256.1 |
  16414.2 |
  16572.2 |
  16730.3 |
  16888.4 |
  17046.5 |########################################
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=60.9% (high variance, measurements may be unstable)
- **satfold-lanes16**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **satfold-lanes16-constl**: CV=60.0% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **satfold-lanes4-idx**: CV=62.9% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **satfold-lanes64**: CV=48.9% (high variance, measurements may be unstable)
- **satfold-lanes64**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **satfold-neon**: CV=50.0% (high variance, measurements may be unstable)
- **satfold-neon**: worst_20/best_20 = 3.6x (possible bimodal distribution)
