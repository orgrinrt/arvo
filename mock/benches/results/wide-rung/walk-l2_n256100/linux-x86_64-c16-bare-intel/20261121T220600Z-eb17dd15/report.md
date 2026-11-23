# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (752.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 702.69 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (702.69 us) and wide-rung-wordround (705.72 us) differ by 0.43%, inside the noise, even though the wider field spreads 7.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged-overread shows alternating (throttle bounce) (autocorr -0.80)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-align16 (+7% speed for 1.3x steadier)

wide-rung-wordround-alias is fastest (702.69 us, CV 5.2%); wide-rung-align16 gives up 7.1% median for 1.3x lower variance (CV 4.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 702691.1 ns median (-6.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.07x (fastest 702691.1 ns, slowest 752288.7 ns)

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
| wide-rung-align16 | 736468ns | 752920ns | 697081ns | 737923ns | 771492ns | base |
| wide-rung-ragged | 718121ns | 707555ns | 679090ns | 709914ns | 781771ns | -2.49% |
| wide-rung-ragged-overread | 717517ns | 709516ns | 661040ns | 720129ns | 766155ns | -2.57% |
| wide-rung-wordround | 711022ns | 706318ns | 657865ns | 710551ns | 765589ns | -3.46% |
| wide-rung-wordround-alias | 694512ns | 703181ns | 642105ns | 697427ns | 738172ns | -5.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 735856ns | 696444ns | 770876ns | base | 0.623 |
| wide-rung-ragged | 717547ns | 678607ns | 781084ns | -2.49% | 0.639 |
| wide-rung-ragged-overread | 716869ns | 660448ns | 765478ns | -2.58% | 0.640 |
| wide-rung-wordround | 710431ns | 657187ns | 765117ns | -3.46% | 0.646 |
| wide-rung-wordround-alias | 693957ns | 641540ns | 737647ns | -5.69% | 0.661 |

## Performance model

- Peak throughput: **0.715 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.610 | 85.3% |
| wide-rung-ragged | 0.649 | 90.7% |
| wide-rung-ragged-overread | 0.647 | 90.5% |
| wide-rung-wordround | 0.650 | 90.9% |
| wide-rung-wordround-alias | 0.653 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 736468ns | 736468ns | base |
| wide-rung-ragged | 718121ns | 718121ns | -2.49% |
| wide-rung-ragged-overread | 717517ns | 717517ns | -2.57% |
| wide-rung-wordround | 711022ns | 711022ns | -3.46% |
| wide-rung-wordround-alias | 694512ns | 694512ns | -5.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 752289ns | base | --- | [713524, 754913] | --- | --- | --- | --- |
| wide-rung-ragged | 706965ns | -31306.9ns (-4.2%) | [-45652, -12529]ns | [704871, 712760] | YES | 0.0385 | 0.0385 | 0 |
| wide-rung-ragged-overread | 708823ns | -7110.5ns (-0.9%) | [-40425, -3444]ns | [703754, 752435] | YES | 0.0086 | 0.0064 | 0 |
| wide-rung-wordround | 705725ns | -45232.7ns (-6.0%) | [-52505, -10372]ns | [700607, 707037] | YES | 0.0044 | 0.0022 | 0 |
| wide-rung-wordround-alias | 702691ns | -45224.2ns (-6.0%) | [-53035, -32620]ns | [689278, 704365] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 711579ns | +3.8% | +3.8% | +6.5% | -1.2% |
| 1 | 2 | 754913ns | -6.5% | -8.7% | -6.4% | -9.6% |
| 1 | 3 | 756138ns | -6.1% | -0.2% | -11.2% | -5.8% |
| 1 | 4 | 713524ns | -2.0% | -0.9% | -1.4% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.249 | moderate- |
| wide-rung-ragged | -0.113 | ok |
| wide-rung-ragged-overread | -0.798 | HIGH- (thermal bounce) |
| wide-rung-wordround | 0.056 | ok |
| wide-rung-wordround-alias | -0.787 | HIGH- (thermal bounce) |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 164.0ns | 735856.3ns | 0.0% |  |
| wide-rung-ragged | 135.6ns | 717546.6ns | 0.0% |  |
| wide-rung-ragged-overread | 210.7ns | 716868.8ns | 0.0% |  |
| wide-rung-wordround | 156.2ns | 710431.5ns | 0.0% |  |
| wide-rung-wordround-alias | 187.3ns | 693957.0ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 696444.0-770875.6 ns)
  696444.0 |
  700165.6 |########
  703887.2 |####
  707608.7 |
  711330.3 |########################################
  715051.9 |########
  718773.5 |
  722495.1 |
  726216.6 |
  729938.2 |
  733659.8 |
  737381.4 |
  741103.0 |
  744824.5 |
  748546.1 |########
  752267.7 |########################################
  755989.3 |################
  759710.9 |################
  763432.5 |
  767154.0 |
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 678606.9-781084.0 ns)
  678606.9 |####
  683730.7 |
  688854.6 |####
  693978.5 |#############
  699102.3 |#################
  704226.2 |########################################
  709350.0 |######################
  714473.9 |####
  719597.7 |#############
  724721.6 |####
  729845.4 |
  734969.3 |####
  740093.2 |########
  745217.0 |
  750340.9 |
  755464.7 |
  760588.6 |
  765712.4 |####
  770836.3 |
  775960.1 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 660448.0-765478.4 ns)
  660448.0 |######
  665699.5 |
  670951.0 |
  676202.6 |####################
  681454.1 |
  686705.6 |######
  691957.1 |
  697208.6 |####################
  702460.1 |########################################
  707711.7 |########################################
  712963.2 |######
  718214.7 |
  723466.2 |
  728717.7 |
  733969.3 |
  739220.8 |
  744472.3 |
  749723.8 |########################################
  754975.3 |####################
  760226.8 |##########################
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 657187.3-765116.7 ns)
  657187.3 |####
  662583.8 |
  667980.2 |####
  673376.7 |
  678773.2 |########
  684169.6 |
  689566.1 |####
  694962.6 |######################
  700359.0 |##########################
  705755.5 |########################################
  711152.0 |
  716548.5 |
  721944.9 |
  727341.4 |
  732737.9 |
  738134.3 |
  743530.8 |
  748927.3 |
  754323.7 |###################################
  759720.2 |########
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 641540.3-737647.2 ns)
  641540.3 |####
  646345.6 |####
  651151.0 |
  655956.3 |########
  660761.7 |
  665567.0 |########
  670372.4 |
  675177.7 |####
  679983.0 |########
  684788.4 |####
  689593.7 |####
  694399.1 |####
  699204.4 |########################################
  704009.8 |######################
  708815.1 |#################
  713620.5 |########
  718425.8 |
  723231.2 |####
  728036.5 |
  732841.9 |
  (4 below, 3 above range)

```
