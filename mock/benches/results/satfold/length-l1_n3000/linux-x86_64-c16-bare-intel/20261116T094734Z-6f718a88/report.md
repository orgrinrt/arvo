# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (satfold-iterfold, satfold-seq) are a dead heat (<1%)

satfold-iterfold (5.05 us) and satfold-seq (5.06 us) differ by 0.30%, inside the noise, even though the wider field spreads 98.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes4-idx shows alternating (throttle bounce) (autocorr -0.69)

satfold-lanes4-idx's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (satfold-iterfold)

The baseline satfold-iterfold is the fastest (5.05 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {satfold-iterfold, satfold-seq, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-lanes16, satfold-neon8, satfold-neon} vs {satfold-lanes4-idx} (86% apart)

The field splits into a fast tier {satfold-iterfold, satfold-seq, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-lanes16, satfold-neon8, satfold-neon} and a slow tier {satfold-lanes4-idx} with a 86% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader satfold-iterfold vs stability leader satfold-neon (+7% speed for 1.9x steadier)

satfold-iterfold is fastest (5.05 us, CV 27.5%); satfold-neon gives up 6.8% median for 1.9x lower variance (CV 14.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### satfold-lanes64 is inconsistent: worst-20% is 1.8x its best-20%

satfold-lanes64's best 20% of batches run at 4.35 us but its worst 20% at 7.80 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (satfold-iterfold) is the fastest** at 5049.4 ns median
- 5 variants significantly slower than baseline
- Spread: 1.99x (fastest 5049.4 ns, slowest 10036.4 ns)

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
| satfold-iterfold | 5192ns | 5124ns | 4210ns | 4877ns | 7118ns | base |
| satfold-lanes16 | 5847ns | 5350ns | 5220ns | 5352ns | 7962ns | +12.63% |
| satfold-lanes16-constl | 5513ns | 5301ns | 4353ns | 5307ns | 7291ns | +6.20% |
| satfold-lanes4-idx | 10052ns | 10109ns | 8617ns | 9810ns | 12210ns | +93.61% |
| satfold-lanes64 | 5658ns | 5305ns | 4408ns | 5310ns | 7952ns | +8.98% |
| satfold-neon | 5681ns | 5464ns | 5297ns | 5473ns | 6691ns | +9.43% |
| satfold-neon8 | 5603ns | 5403ns | 4334ns | 5371ns | 7568ns | +7.92% |
| satfold-nolaw | 5306ns | 5310ns | 4282ns | 4985ns | 7295ns | +2.21% |
| satfold-seq | 5525ns | 5135ns | 4445ns | 5126ns | 7806ns | +6.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 5111ns | 4151ns | 6973ns | base | 6.412 |
| satfold-lanes16 | 5757ns | 5147ns | 7808ns | +12.65% | 5.692 |
| satfold-lanes16-constl | 5428ns | 4294ns | 7140ns | +6.21% | 6.037 |
| satfold-lanes4-idx | 9976ns | 8557ns | 12108ns | +95.20% | 3.285 |
| satfold-lanes64 | 5571ns | 4349ns | 7796ns | +9.02% | 5.881 |
| satfold-neon | 5597ns | 5223ns | 6568ns | +9.52% | 5.854 |
| satfold-neon8 | 5517ns | 4274ns | 7416ns | +7.94% | 5.940 |
| satfold-nolaw | 5225ns | 4223ns | 7154ns | +2.23% | 6.272 |
| satfold-seq | 5440ns | 4386ns | 7653ns | +6.44% | 6.024 |

## Performance model

- Peak throughput: **7.894 Gops/s** (satfold-iterfold; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 6.490 | 82.2% |
| satfold-lanes16 | 6.209 | 78.6% |
| satfold-lanes16-constl | 6.266 | 79.4% |
| satfold-lanes4-idx | 3.265 | 41.4% |
| satfold-lanes64 | 6.263 | 79.3% |
| satfold-neon | 6.077 | 77.0% |
| satfold-neon8 | 6.149 | 77.9% |
| satfold-nolaw | 6.258 | 79.3% |
| satfold-seq | 6.470 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 5192ns | 5192ns | base |
| satfold-lanes16 | 5847ns | 5847ns | +12.63% |
| satfold-lanes16-constl | 5513ns | 5513ns | +6.20% |
| satfold-lanes4-idx | 10052ns | 10052ns | +93.61% |
| satfold-lanes64 | 5658ns | 5658ns | +8.98% |
| satfold-neon | 5681ns | 5681ns | +9.43% |
| satfold-neon8 | 5603ns | 5603ns | +7.92% |
| satfold-nolaw | 5306ns | 5306ns | +2.21% |
| satfold-seq | 5525ns | 5525ns | +6.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 5049ns | base | --- | [4410, 5180] | --- | --- | --- | --- |
| satfold-lanes16 | 5278ns | +459.4ns (+9.1%) | [+154, +947]ns | [5206, 5301] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 5229ns | no significant difference | [-23, +1015]ns | [5221, 5247] | no | 0.6358 | 0.6358 | 0 |
| satfold-lanes4-idx | 10036ns | +5309.4ns (+105.2%) | [+4248, +5524]ns | [8995, 10140] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5232ns | +148.2ns (+2.9%) | [+111, +323]ns | [5214, 5264] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 5392ns | +634.0ns (+12.6%) | [+264, +1028]ns | [5323, 5496] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 5329ns | +199.1ns (+3.9%) | [+157, +336]ns | [5264, 5424] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 5236ns | no significant difference | [-94, +154]ns | [4340, 5273] | no | 0.3065 | 0.2682 | 0 |
| satfold-seq | 5065ns | no significant difference | [-28, +855]ns | [5034, 5102] | no | 0.2051 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5233ns | +1.0% | -0.2% | +70.4% | +1.0% | +5.1% | +4.0% | +1.4% | -16.2% |
| 1 | 2 | 4199ns | +25.9% | +24.6% | +139.6% | +2.6% | +30.1% | +2.0% | +24.8% | +20.7% |
| 1 | 3 | 4410ns | +19.2% | +19.6% | +93.2% | +19.4% | +21.3% | +21.0% | -1.7% | +15.7% |
| 1 | 4 | 5097ns | +2.8% | -15.8% | +107.7% | +2.3% | +3.4% | +3.3% | -17.1% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | -0.273 | moderate- |
| satfold-lanes16 | 0.277 | moderate+ |
| satfold-lanes16-constl | -0.118 | ok |
| satfold-lanes4-idx | -0.692 | HIGH- (thermal bounce) |
| satfold-lanes64 | -0.471 | moderate- |
| satfold-neon | 0.292 | moderate+ |
| satfold-neon8 | -0.513 | HIGH- (thermal bounce) |
| satfold-nolaw | 0.287 | moderate+ |
| satfold-seq | -0.057 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 0/4, lost 4/4
- **satfold-lanes16-constl**: won 2/4, lost 2/4
- **satfold-lanes4-idx**: won 0/4, lost 4/4
- **satfold-lanes64**: won 0/4, lost 4/4
- **satfold-neon**: won 0/4, lost 4/4
- **satfold-neon8**: won 0/4, lost 4/4
- **satfold-nolaw**: won 2/4, lost 2/4
- **satfold-seq**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 23.6ns | 5110.7ns | 0.5% |  |
| satfold-lanes16 | 26.2ns | 5757.1ns | 0.5% |  |
| satfold-lanes16-constl | 24.7ns | 5428.1ns | 0.5% |  |
| satfold-lanes4-idx | 23.8ns | 9975.9ns | 0.2% |  |
| satfold-lanes64 | 25.8ns | 5571.4ns | 0.5% |  |
| satfold-neon | 25.2ns | 5597.2ns | 0.4% |  |
| satfold-neon8 | 25.0ns | 5516.6ns | 0.5% |  |
| satfold-nolaw | 24.1ns | 5224.8ns | 0.5% |  |
| satfold-seq | 25.2ns | 5440.0ns | 0.5% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 4151.0-6973.4 ns)
   4151.0 |########################################
   4292.1 |#######
   4433.3 |##########
   4574.4 |
   4715.5 |
   4856.6 |
   4997.7 |#####################
   5138.9 |########################################
   5280.0 |
   5421.1 |#######
   5562.2 |
   5703.3 |
   5844.4 |
   5985.6 |
   6126.7 |
   6267.8 |
   6408.9 |
   6550.0 |
   6691.2 |
   6832.3 |
  (2 below, 3 above range)

satfold-lanes16 (n=40, range 5146.6-7808.0 ns)
   5146.6 |########################################
   5279.7 |#########################
   5412.8 |##########
   5545.8 |#####
   5678.9 |
   5812.0 |
   5945.1 |##
   6078.1 |
   6211.2 |
   6344.3 |
   6477.3 |
   6610.4 |
   6743.5 |
   6876.5 |
   7009.6 |
   7142.7 |
   7275.7 |
   7408.8 |
   7541.9 |
   7675.0 |
  (4 below, 3 above range)

satfold-lanes16-constl (n=40, range 4294.4-7139.7 ns)
   4294.4 |##
   4436.7 |
   4578.9 |
   4721.2 |
   4863.4 |
   5005.7 |
   5148.0 |########################################
   5290.2 |
   5432.5 |
   5574.8 |
   5717.0 |
   5859.3 |
   6001.6 |
   6143.8 |
   6286.1 |
   6428.4 |#
   6570.6 |#
   6712.9 |#
   6855.2 |
   6997.4 |
  (6 below, 2 above range)

satfold-lanes4-idx (n=40, range 8556.9-12107.8 ns)
   8556.9 |
   8734.5 |#########################
   8912.0 |##############################
   9089.6 |
   9267.1 |
   9444.7 |
   9622.2 |
   9799.8 |##########
   9977.3 |########################################
  10154.8 |
  10332.4 |#####
  10509.9 |###################################
  10687.5 |##########
  10865.0 |#####
  11042.6 |
  11220.1 |
  11397.7 |
  11575.2 |
  11752.7 |
  11930.3 |
  (6 below, 2 above range)

satfold-lanes64 (n=40, range 4349.4-7795.9 ns)
   4349.4 |#
   4521.8 |#
   4694.1 |
   4866.4 |
   5038.7 |#####
   5211.0 |########################################
   5383.4 |#
   5555.7 |
   5728.0 |
   5900.3 |
   6072.7 |
   6245.0 |
   6417.3 |
   6589.6 |
   6761.9 |
   6934.3 |
   7106.6 |#
   7278.9 |###
   7451.2 |
   7623.6 |
  (6 below, 2 above range)

satfold-neon (n=40, range 5223.1-6568.4 ns)
   5223.1 |########################################
   5290.3 |#################
   5357.6 |##########################
   5424.9 |########
   5492.1 |##########################
   5559.4 |####
   5626.7 |#############
   5693.9 |####
   5761.2 |
   5828.5 |####
   5895.7 |
   5963.0 |
   6030.3 |
   6097.5 |
   6164.8 |
   6232.1 |####
   6299.4 |
   6366.6 |
   6433.9 |
   6501.2 |
  (4 below, 2 above range)

satfold-neon8 (n=40, range 4274.3-7415.5 ns)
   4274.3 |############
   4431.3 |##
   4588.4 |
   4745.5 |
   4902.5 |
   5059.6 |
   5216.6 |########################################
   5373.7 |###########################
   5530.8 |
   5687.8 |
   5844.9 |
   6002.0 |
   6159.0 |
   6316.1 |
   6473.2 |
   6630.2 |
   6787.3 |
   6944.3 |
   7101.4 |
   7258.5 |
  (3 below, 4 above range)

satfold-nolaw (n=40, range 4223.4-7154.1 ns)
   4223.4 |########################################
   4370.0 |
   4516.5 |
   4663.0 |
   4809.6 |
   4956.1 |
   5102.6 |########################
   5249.2 |####################################
   5395.7 |
   5542.2 |
   5688.8 |
   5835.3 |
   5981.8 |
   6128.4 |
   6274.9 |
   6421.4 |
   6567.9 |
   6714.5 |
   6861.0 |
   7007.5 |
  (4 below, 3 above range)

satfold-seq (n=40, range 4386.1-7652.7 ns)
   4386.1 |###########
   4549.5 |
   4712.8 |
   4876.1 |####################
   5039.4 |########################################
   5202.8 |####################
   5366.1 |
   5529.4 |
   5692.8 |
   5856.1 |
   6019.4 |
   6182.7 |
   6346.1 |
   6509.4 |
   6672.7 |
   6836.0 |
   6999.4 |
   7162.7 |
   7326.0 |
   7489.3 |
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: CV=27.2% (high variance, measurements may be unstable)
- **satfold-lanes16**: CV=30.2% (high variance, measurements may be unstable)
- **satfold-lanes16-constl**: CV=26.7% (high variance, measurements may be unstable)
- **satfold-lanes64**: CV=29.7% (high variance, measurements may be unstable)
- **satfold-neon8**: CV=25.9% (high variance, measurements may be unstable)
- **satfold-nolaw**: CV=30.9% (high variance, measurements may be unstable)
- **satfold-seq**: CV=33.6% (high variance, measurements may be unstable)
