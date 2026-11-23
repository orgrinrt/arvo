# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.63)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-ragged-overread vs stability leader wide-rung-wordround-alias (+9% speed for 3.7x steadier)

wide-rung-ragged-overread is fastest (639.29 us, CV 9.9%); wide-rung-wordround-alias gives up 8.9% median for 3.7x lower variance (CV 2.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 639288.9 ns median (-8.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.10x (fastest 639288.9 ns, slowest 703266.7 ns)

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
| wide-rung-align16 | 721041ns | 700640ns | 637541ns | 718511ns | 812132ns | base |
| wide-rung-ragged | 679693ns | 673709ns | 646310ns | 669790ns | 742786ns | -5.73% |
| wide-rung-ragged-overread | 672465ns | 639763ns | 625155ns | 649351ns | 789118ns | -6.74% |
| wide-rung-wordround | 704226ns | 703924ns | 677768ns | 704060ns | 731179ns | -2.33% |
| wide-rung-wordround-alias | 689468ns | 696726ns | 661684ns | 691757ns | 710384ns | -4.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 720374ns | 636942ns | 811495ns | base | 0.637 |
| wide-rung-ragged | 679183ns | 645814ns | 742259ns | -5.72% | 0.675 |
| wide-rung-ragged-overread | 671945ns | 624656ns | 788591ns | -6.72% | 0.683 |
| wide-rung-wordround | 703579ns | 677155ns | 730544ns | -2.33% | 0.652 |
| wide-rung-wordround-alias | 688865ns | 661117ns | 709805ns | -4.37% | 0.666 |

## Performance model

- Peak throughput: **0.734 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.655 | 89.3% |
| wide-rung-ragged | 0.681 | 92.8% |
| wide-rung-ragged-overread | 0.718 | 97.7% |
| wide-rung-wordround | 0.652 | 88.8% |
| wide-rung-wordround-alias | 0.659 | 89.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 721041ns | 721041ns | base |
| wide-rung-ragged | 679693ns | 679693ns | -5.73% |
| wide-rung-ragged-overread | 672465ns | 672465ns | -6.74% |
| wide-rung-wordround | 704226ns | 704226ns | -2.33% |
| wide-rung-wordround-alias | 689468ns | 689468ns | -4.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 699854ns | base | --- | [666451, 784238] | --- | --- | --- | --- |
| wide-rung-ragged | 673177ns | -36738.2ns (-5.2%) | [-123453, -12994]ns | [652509, 678362] | YES (adj: no) | 0.0770 | 0.0385 | 0 |
| wide-rung-ragged-overread | 639289ns | -32707.3ns (-4.7%) | [-52564, -18763]ns | [638090, 643763] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 703267ns | no significant difference | [-83652, +29752]ns | [701979, 704315] | no | 1.0000 | 1.0000 | 0 |
| wide-rung-wordround-alias | 696097ns | no significant difference | [-84385, +32095]ns | [684555, 696710] | no | 1.0000 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 666451ns | +0.8% | -4.7% | +5.5% | +4.5% |
| 1 | 2 | 653354ns | +6.7% | -2.2% | +6.9% | +6.7% |
| 1 | 3 | 781050ns | -16.5% | -18.2% | -10.1% | -11.6% |
| 1 | 4 | 806618ns | -15.9% | -0.5% | -11.9% | -17.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.260 | moderate+ |
| wide-rung-ragged | -0.629 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.079 | ok |
| wide-rung-wordround | -0.025 | ok |
| wide-rung-wordround-alias | 0.094 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 151.4ns | 720374.5ns | 0.0% |  |
| wide-rung-ragged | 162.1ns | 679183.2ns | 0.0% |  |
| wide-rung-ragged-overread | 174.7ns | 671944.5ns | 0.0% |  |
| wide-rung-wordround | 174.8ns | 703578.8ns | 0.0% |  |
| wide-rung-wordround-alias | 174.1ns | 688864.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 636941.6-811494.7 ns)
  636941.6 |#############
  645669.3 |########
  654396.9 |#################
  663124.6 |######################
  671852.2 |####
  680579.9 |
  689307.5 |########
  698035.2 |########
  706762.8 |####
  715490.5 |
  724218.1 |
  732945.8 |####
  741673.4 |
  750401.1 |
  759128.7 |
  767856.4 |
  776584.1 |#############
  785311.7 |########
  794039.4 |####
  802767.0 |########################################
  (3 below, 1 above range)

wide-rung-ragged (n=40, range 645814.1-742259.1 ns)
  645814.1 |######################
  650636.3 |###################################
  655458.6 |########
  660280.8 |
  665103.1 |####
  669925.3 |
  674747.6 |########################################
  679569.8 |####
  684392.1 |####
  689214.3 |
  694036.6 |
  698858.8 |
  703681.1 |
  708503.3 |
  713325.6 |
  718147.8 |
  722970.1 |
  727792.4 |
  732614.6 |##########################
  737436.9 |####
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 624655.5-788590.7 ns)
  624655.5 |##########
  632852.3 |########################################
  641049.0 |##########
  649245.8 |##
  657442.5 |
  665639.3 |
  673836.1 |
  682032.8 |
  690229.6 |#######
  698426.4 |##
  706623.1 |##
  714819.9 |
  723016.6 |
  731213.4 |
  739410.2 |
  747606.9 |
  755803.7 |##
  764000.5 |
  772197.2 |
  780394.0 |
  (3 below, 6 above range)

wide-rung-wordround (n=40, range 677154.9-730544.1 ns)
  677154.9 |##
  679824.3 |
  682493.8 |##
  685163.2 |##
  687832.7 |
  690502.2 |#####
  693171.6 |#####
  695841.1 |##
  698510.5 |
  701180.0 |########################################
  703849.5 |##
  706518.9 |
  709188.4 |#####################
  711857.8 |#####
  714527.3 |
  717196.8 |
  719866.2 |
  722535.7 |
  725205.1 |
  727874.6 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 661117.2-709805.4 ns)
  661117.2 |##
  663551.6 |##
  665986.0 |
  668420.4 |
  670854.9 |#####
  673289.3 |########
  675723.7 |
  678158.1 |##
  680592.5 |
  683026.9 |########
  685461.3 |
  687895.7 |##
  690330.1 |
  692764.5 |##
  695198.9 |########################################
  697633.3 |##
  700067.7 |#####
  702502.2 |#####
  704936.6 |
  707371.0 |
  (4 below, 3 above range)

```
