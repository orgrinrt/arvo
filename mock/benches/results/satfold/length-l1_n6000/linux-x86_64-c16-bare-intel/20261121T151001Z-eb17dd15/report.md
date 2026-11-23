# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx dominates: 123% faster than the next best (satfold-lanes16)

satfold-lanes4-idx (2.41 us) leads satfold-lanes16 (5.39 us) by 123%, a clear separation rather than a photo finish. CV 109.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-lanes4-idx beats baseline by 72% (significant)

satfold-lanes4-idx is -9.62 us (72%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 6.2x slower than the field

satfold-seq (14.90 us) is 6.2x the fastest (2.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes4-idx is fastest but the noisiest (CV 109.9%)

satfold-lanes4-idx wins on median (2.41 us) yet has the highest variance (CV 109.9%), while satfold-seq is the steadiest (CV 1.8%, 14.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-neon shows alternating (throttle bounce) (autocorr -0.76)

satfold-neon's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.2x the fastest

Fastest satfold-lanes4-idx (2.41 us) to slowest satfold-seq (14.90 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes4-idx is inconsistent: worst-20% is 4.3x its best-20%

satfold-lanes4-idx's best 20% of batches run at 1.97 us but its worst 20% at 8.39 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes4-idx** at 2413.8 ns median (-81.9% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.17x (fastest 2413.8 ns, slowest 14895.5 ns)

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
| satfold-iterfold | 13503ns | 13374ns | 11851ns | 13480ns | 15225ns | base |
| satfold-lanes16 | 5325ns | 5462ns | 4480ns | 5067ns | 6945ns | -60.57% |
| satfold-lanes16-constl | 6792ns | 5512ns | 4505ns | 5692ns | 12381ns | -49.70% |
| satfold-lanes4-idx | 4072ns | 2477ns | 2022ns | 3230ns | 8648ns | -69.85% |
| satfold-lanes64 | 9587ns | 9932ns | 7880ns | 9799ns | 10659ns | -29.00% |
| satfold-neon | 11083ns | 10686ns | 8715ns | 10651ns | 14747ns | -17.92% |
| satfold-neon8 | 10001ns | 9947ns | 8565ns | 10010ns | 11409ns | -25.94% |
| satfold-nolaw | 5643ns | 5589ns | 4590ns | 5160ns | 8143ns | -58.21% |
| satfold-seq | 14907ns | 14990ns | 14486ns | 14938ns | 15238ns | +10.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 13433ns | 11793ns | 15146ns | base | 2.439 |
| satfold-lanes16 | 5247ns | 4422ns | 6809ns | -60.94% | 6.245 |
| satfold-lanes16-constl | 6696ns | 4447ns | 12185ns | -50.15% | 4.893 |
| satfold-lanes4-idx | 3959ns | 1969ns | 8386ns | -70.53% | 8.277 |
| satfold-lanes64 | 9513ns | 7816ns | 10581ns | -29.18% | 3.445 |
| satfold-neon | 11009ns | 8635ns | 14668ns | -18.04% | 2.976 |
| satfold-neon8 | 9927ns | 8499ns | 11338ns | -26.10% | 3.301 |
| satfold-nolaw | 5560ns | 4531ns | 7992ns | -58.60% | 5.893 |
| satfold-seq | 14830ns | 14414ns | 15158ns | +10.40% | 2.210 |

## Performance model

- Peak throughput: **16.640 Gops/s** (satfold-lanes4-idx; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 2.462 | 14.8% |
| satfold-lanes16 | 6.079 | 36.5% |
| satfold-lanes16-constl | 6.023 | 36.2% |
| satfold-lanes4-idx | 13.575 | 81.6% |
| satfold-lanes64 | 3.323 | 20.0% |
| satfold-neon | 3.086 | 18.5% |
| satfold-neon8 | 3.318 | 19.9% |
| satfold-nolaw | 5.939 | 35.7% |
| satfold-seq | 2.200 | 13.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 13503ns | 13503ns | base |
| satfold-lanes16 | 5325ns | 5325ns | -60.57% |
| satfold-lanes16-constl | 6792ns | 6792ns | -49.70% |
| satfold-lanes4-idx | 4072ns | 4072ns | -69.85% |
| satfold-lanes64 | 9587ns | 9587ns | -29.00% |
| satfold-neon | 11083ns | 11083ns | -17.92% |
| satfold-neon8 | 10001ns | 10001ns | -25.94% |
| satfold-nolaw | 5643ns | 5643ns | -58.21% |
| satfold-seq | 14907ns | 14907ns | +10.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 13307ns | base | --- | [12002, 14896] | --- | --- | --- | --- |
| satfold-lanes16 | 5390ns | -7599.2ns (-57.1%) | [-9689, -7359]ns | [4445, 5402] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 5440ns | -6484.6ns (-48.7%) | [-6644, -6376]ns | [5430, 5444] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2414ns | -9618.0ns (-72.3%) | [-11119, -9230]ns | [2406, 2667] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 9860ns | -4317.9ns (-32.4%) | [-4560, -4072]ns | [9633, 10050] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 10620ns | -3236.5ns (-24.3%) | [-3340, -3131]ns | [8746, 11850] | YES | 0.0022 | 0.0022 | 0 |
| satfold-neon8 | 9875ns | -3644.8ns (-27.4%) | [-4071, -3184]ns | [9712, 10254] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 5518ns | -7545.3ns (-56.7%) | [-9507, -7306]ns | [4556, 5553] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 14896ns | +1417.3ns (+10.7%) | [+156, +2554]ns | [14726, 14995] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14811ns | -70.0% | -31.9% | -83.7% | -32.5% | -41.4% | -34.4% | -69.2% | +1.1% |
| 1 | 2 | 11868ns | -54.5% | -54.1% | -79.6% | -18.5% | +23.5% | -14.7% | -53.2% | +23.2% |
| 1 | 3 | 11911ns | -62.7% | -54.3% | -80.4% | -34.2% | -26.6% | -27.3% | -61.9% | +22.7% |
| 1 | 4 | 15107ns | -64.2% | -70.5% | -86.9% | -30.7% | -21.6% | -25.9% | -63.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.248 | moderate- |
| satfold-lanes16 | -0.749 | HIGH- (thermal bounce) |
| satfold-lanes16-constl | -0.043 | ok |
| satfold-lanes4-idx | 0.065 | ok |
| satfold-lanes64 | -0.459 | moderate- |
| satfold-neon | -0.758 | HIGH- (thermal bounce) |
| satfold-neon8 | -0.579 | HIGH- (thermal bounce) |
| satfold-nolaw | -0.751 | HIGH- (thermal bounce) |
| satfold-seq | -0.239 | moderate- |

**Consistency summary:**

- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-constl**: won 4/4, lost 0/4
- **satfold-lanes4-idx**: won 4/4, lost 0/4
- **satfold-lanes64**: won 4/4, lost 0/4
- **satfold-neon**: won 3/4, lost 1/4
- **satfold-neon8**: won 4/4, lost 0/4
- **satfold-nolaw**: won 4/4, lost 0/4
- **satfold-seq**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 22.0ns | 13432.6ns | 0.2% |  |
| satfold-lanes16 | 23.0ns | 5247.0ns | 0.4% |  |
| satfold-lanes16-constl | 29.1ns | 6696.5ns | 0.4% |  |
| satfold-lanes4-idx | 37.6ns | 3959.1ns | 0.9% |  |
| satfold-lanes64 | 23.0ns | 9513.0ns | 0.2% |  |
| satfold-neon | 23.1ns | 11009.3ns | 0.2% |  |
| satfold-neon8 | 22.8ns | 9926.8ns | 0.2% |  |
| satfold-nolaw | 23.3ns | 5560.4ns | 0.4% |  |
| satfold-seq | 24.8ns | 14829.7ns | 0.2% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 11793.0-15146.0 ns)
  11793.0 |########################################
  11960.6 |###############################
  12128.3 |####
  12295.9 |
  12463.6 |
  12631.2 |
  12798.9 |
  12966.5 |
  13134.2 |
  13301.8 |
  13469.5 |
  13637.1 |
  13804.8 |
  13972.4 |
  14140.1 |
  14307.7 |####
  14475.4 |#############
  14643.0 |####
  14810.7 |#################
  14978.3 |###############################
  (3 below, 4 above range)

satfold-lanes16 (n=40, range 4422.4-6808.8 ns)
   4422.4 |#################################
   4541.7 |
   4661.0 |
   4780.4 |
   4899.7 |
   5019.0 |
   5138.3 |
   5257.6 |
   5377.0 |########################################
   5496.3 |
   5615.6 |
   5734.9 |
   5854.3 |
   5973.6 |
   6092.9 |
   6212.2 |
   6331.5 |
   6450.9 |
   6570.2 |
   6689.5 |##
  (3 below, 3 above range)

satfold-lanes16-constl (n=40, range 4446.9-12185.2 ns)
   4446.9 |##########
   4833.8 |
   5220.7 |########################################
   5607.6 |##
   5994.5 |##
   6381.5 |
   6768.4 |##
   7155.3 |
   7542.2 |
   7929.1 |##
   8316.0 |
   8703.0 |
   9089.9 |##
   9476.8 |
   9863.7 |
  10250.6 |##
  10637.6 |####
  11024.5 |
  11411.4 |
  11798.3 |
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 1969.2-8385.7 ns)
   1969.2 |######################
   2290.1 |########################################
   2610.9 |#####
   2931.7 |
   3252.5 |
   3573.3 |
   3894.2 |##
   4215.0 |
   4535.8 |
   4856.6 |
   5177.5 |
   5498.3 |##
   5819.1 |
   6139.9 |
   6460.8 |#####
   6781.6 |##
   7102.4 |#####
   7423.2 |########
   7744.0 |
   8064.9 |##
  (3 below, 2 above range)

satfold-lanes64 (n=40, range 7816.5-10581.2 ns)
   7816.5 |########################################
   7954.7 |######
   8093.0 |
   8231.2 |
   8369.4 |
   8507.7 |
   8645.9 |
   8784.1 |
   8922.4 |
   9060.6 |
   9198.9 |
   9337.1 |#############
   9475.3 |######
   9613.6 |########################################
   9751.8 |#############
   9890.1 |##########################
  10028.3 |####################
  10166.5 |####################
  10304.8 |######
  10443.0 |#################################
  (3 below, 3 above range)

satfold-neon (n=40, range 8635.3-14668.0 ns)
   8635.3 |########################################
   8936.9 |
   9238.6 |######
   9540.2 |
   9841.9 |
  10143.5 |
  10445.1 |
  10746.8 |
  11048.4 |
  11350.0 |
  11651.7 |##############################
  11953.3 |
  12255.0 |
  12556.6 |
  12858.2 |
  13159.9 |
  13461.5 |
  13763.1 |
  14064.8 |
  14366.4 |#####################
  (5 below, 3 above range)

satfold-neon8 (n=40, range 8499.3-11337.6 ns)
   8499.3 |########
   8641.2 |################
   8783.1 |########
   8925.0 |########
   9066.9 |
   9208.8 |########
   9350.8 |
   9492.7 |################
   9634.6 |########################################
   9776.5 |################################
   9918.4 |################
  10060.3 |########################
  10202.3 |################
  10344.2 |########
  10486.1 |################
  10628.0 |########
  10769.9 |
  10911.8 |################
  11053.8 |########
  11195.7 |################
  (4 below, 3 above range)

satfold-nolaw (n=40, range 4530.7-7992.3 ns)
   4530.7 |########################################
   4703.8 |
   4876.8 |
   5049.9 |
   5223.0 |
   5396.1 |##################################
   5569.2 |#################
   5742.2 |
   5915.3 |
   6088.4 |
   6261.5 |
   6434.6 |
   6607.6 |
   6780.7 |
   6953.8 |
   7126.9 |
   7300.0 |
   7473.0 |
   7646.1 |
   7819.2 |
  (5 below, 3 above range)

satfold-seq (n=40, range 14413.7-15158.4 ns)
  14413.7 |
  14450.9 |####################
  14488.1 |##########
  14525.4 |
  14562.6 |##############################
  14599.9 |
  14637.1 |####################
  14674.3 |##########
  14711.6 |####################
  14748.8 |####################
  14786.0 |##########
  14823.3 |##########
  14860.5 |##########
  14897.7 |####################
  14935.0 |####################
  14972.2 |####################
  15009.4 |####################
  15046.7 |########################################
  15083.9 |####################
  15121.2 |####################
  (4 below, 4 above range)

```

## Diagnostics

- **satfold-lanes16**: CV=21.8% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=44.2% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: CV=67.0% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **satfold-neon**: CV=22.3% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=34.7% (high variance, measurements may be unstable)
