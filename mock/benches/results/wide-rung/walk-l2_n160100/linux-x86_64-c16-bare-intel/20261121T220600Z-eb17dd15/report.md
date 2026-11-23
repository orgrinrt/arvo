# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (536.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 293.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged beats baseline by 39% (significant)

wide-rung-ragged is -209.53 us (39%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.54)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-ragged, wide-rung-ragged-overread, wide-rung-wordround, wide-rung-wordround-alias} vs {wide-rung-align16} (47% apart)

The field splits into a fast tier {wide-rung-ragged, wide-rung-ragged-overread, wide-rung-wordround, wide-rung-wordround-alias} and a slow tier {wide-rung-align16} with a 47% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-ragged-overread is inconsistent: worst-20% is 2.5x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 189.35 us but its worst 20% at 464.08 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 293285.3 ns median (-45.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.83x (fastest 293285.3 ns, slowest 536259.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112737653 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 534898ns | 536839ns | 491323ns | 535201ns | 577562ns | base |
| wide-rung-ragged | 296414ns | 293607ns | 232661ns | 294985ns | 364453ns | -44.58% |
| wide-rung-ragged-overread | 314840ns | 311259ns | 189525ns | 306721ns | 464511ns | -41.14% |
| wide-rung-wordround | 368418ns | 364345ns | 317081ns | 357855ns | 451441ns | -31.12% |
| wide-rung-wordround-alias | 367201ns | 364696ns | 314869ns | 360429ns | 439850ns | -31.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 534366ns | 490747ns | 577085ns | base | 0.858 |
| wide-rung-ragged | 296105ns | 232466ns | 364052ns | -44.59% | 1.549 |
| wide-rung-ragged-overread | 314560ns | 189351ns | 464076ns | -41.13% | 1.458 |
| wide-rung-wordround | 367983ns | 316658ns | 450928ns | -31.14% | 1.247 |
| wide-rung-wordround-alias | 366829ns | 314608ns | 439368ns | -31.35% | 1.251 |

## Performance model

- Peak throughput: **2.423 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.855 | 35.3% |
| wide-rung-ragged | 1.564 | 64.6% |
| wide-rung-ragged-overread | 1.475 | 60.9% |
| wide-rung-wordround | 1.261 | 52.0% |
| wide-rung-wordround-alias | 1.259 | 52.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 534898ns | 534898ns | base |
| wide-rung-ragged | 296414ns | 296414ns | -44.58% |
| wide-rung-ragged-overread | 314840ns | 314840ns | -41.14% |
| wide-rung-wordround | 368418ns | 368418ns | -31.12% |
| wide-rung-wordround-alias | 367201ns | 367201ns | -31.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 536260ns | base | --- | [532911, 539822] | --- | --- | --- | --- |
| wide-rung-ragged | 293285ns | -209528.9ns (-39.1%) | [-243096, -205031]ns | [286864, 300586] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 311004ns | -206258.2ns (-38.5%) | [-300225, -164733]ns | [243158, 358347] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 363875ns | -178569.8ns (-33.3%) | [-198498, -163645]ns | [337071, 369266] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 364334ns | -180625.9ns (-33.7%) | [-200237, -137361]ns | [357105, 364942] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 536194ns | -39.6% | -23.7% | -31.1% | -41.8% |
| 1 | 2 | 565892ns | -58.9% | -60.3% | -35.6% | -35.5% |
| 1 | 3 | 498928ns | -41.1% | -35.1% | -34.9% | -26.7% |
| 1 | 4 | 536587ns | -45.3% | -65.5% | -36.6% | -33.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.504 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.539 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.449 | moderate- |
| wide-rung-wordround | 0.123 | ok |
| wide-rung-wordround-alias | -0.109 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 130.1ns | 534366.3ns | 0.0% |  |
| wide-rung-ragged | 116.9ns | 296105.3ns | 0.0% |  |
| wide-rung-ragged-overread | 66.6ns | 314560.4ns | 0.0% |  |
| wide-rung-wordround | 143.3ns | 367983.4ns | 0.0% |  |
| wide-rung-wordround-alias | 51.9ns | 366828.9ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 490746.9-577085.3 ns)
  490746.9 |
  495063.8 |###############################
  499380.7 |#############
  503697.6 |
  508014.5 |
  512331.5 |
  516648.4 |
  520965.3 |
  525282.2 |
  529599.2 |#################
  533916.1 |########################################
  538233.0 |########
  542549.9 |####
  546866.9 |####
  551183.8 |
  555500.7 |
  559817.6 |
  564134.5 |###############################
  568451.5 |####
  572768.4 |####
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 232465.9-364052.0 ns)
  232465.9 |######################
  239045.2 |
  245624.5 |
  252203.8 |
  258783.1 |########
  265362.4 |
  271941.7 |####
  278521.0 |####
  285100.3 |########
  291679.6 |########################################
  298258.9 |########
  304838.2 |
  311417.5 |
  317996.8 |########
  324576.1 |
  331155.4 |
  337734.8 |####
  344314.1 |########
  350893.4 |########
  357472.7 |####
  (5 below, 5 above range)

wide-rung-ragged-overread (n=40, range 189351.5-464076.1 ns)
  189351.5 |
  203087.7 |
  216823.9 |########################################
  230560.2 |
  244296.4 |
  258032.6 |#####
  271768.9 |#################
  285505.1 |#####
  299241.3 |#####
  312977.6 |############################
  326713.8 |#####
  340450.0 |
  354186.2 |
  367922.5 |###########
  381658.7 |#####
  395394.9 |#################
  409131.2 |######################
  422867.4 |#####
  436603.6 |
  450339.8 |
  (7 below, 3 above range)

wide-rung-wordround (n=40, range 316658.5-450928.0 ns)
  316658.5 |########
  323371.9 |############
  330085.4 |########################
  336798.9 |########
  343512.4 |
  350225.8 |####
  356939.3 |####
  363652.8 |########################################
  370366.3 |
  377079.7 |########
  383793.2 |############
  390506.7 |
  397220.2 |
  403933.6 |
  410647.1 |
  417360.6 |
  424074.1 |
  430787.5 |########
  437501.0 |
  444214.5 |####
  (3 below, 4 above range)

wide-rung-wordround-alias (n=40, range 314608.5-439368.4 ns)
  314608.5 |
  320846.5 |
  327084.5 |
  333322.5 |#############
  339560.5 |
  345798.4 |
  352036.4 |#######################
  358274.4 |################
  364512.4 |########################################
  370750.4 |
  376988.4 |
  383226.4 |
  389464.4 |
  395702.4 |
  401940.4 |###
  408178.4 |###
  414416.4 |
  420654.4 |
  426892.4 |
  433130.4 |
  (6 below, 4 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: CV=33.1% (high variance, measurements may be unstable)
