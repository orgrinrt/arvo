# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (715.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 459.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged dominates: 20% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (459.85 us) leads wide-rung-ragged-overread (551.36 us) by 20%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 32% (significant)

wide-rung-ragged is -230.86 us (32%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: wide-rung-ragged** at 459848.4 ns median (-35.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.56x (fastest 459848.4 ns, slowest 715281.0 ns)

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
| wide-rung-align16 | 709839ns | 715774ns | 642374ns | 705968ns | 788915ns | base |
| wide-rung-ragged | 464695ns | 460067ns | 420444ns | 462774ns | 514707ns | -34.54% |
| wide-rung-ragged-overread | 545985ns | 551818ns | 443738ns | 549895ns | 636504ns | -23.08% |
| wide-rung-wordround | 576551ns | 589180ns | 537754ns | 579674ns | 605981ns | -18.78% |
| wide-rung-wordround-alias | 611018ns | 619970ns | 556497ns | 617094ns | 647314ns | -13.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 709197ns | 641296ns | 788404ns | base | 0.647 |
| wide-rung-ragged | 464395ns | 420236ns | 514329ns | -34.52% | 0.988 |
| wide-rung-ragged-overread | 545622ns | 443413ns | 636157ns | -23.06% | 0.841 |
| wide-rung-wordround | 576148ns | 537368ns | 605532ns | -18.76% | 0.796 |
| wide-rung-wordround-alias | 610558ns | 556057ns | 646840ns | -13.91% | 0.751 |

## Performance model

- Peak throughput: **1.092 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.641 | 58.8% |
| wide-rung-ragged | 0.998 | 91.4% |
| wide-rung-ragged-overread | 0.832 | 76.2% |
| wide-rung-wordround | 0.779 | 71.4% |
| wide-rung-wordround-alias | 0.741 | 67.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 709839ns | 709839ns | base |
| wide-rung-ragged | 464695ns | 464695ns | -34.54% |
| wide-rung-ragged-overread | 545985ns | 545985ns | -23.08% |
| wide-rung-wordround | 576551ns | 576551ns | -18.78% |
| wide-rung-wordround-alias | 611018ns | 611018ns | -13.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 715281ns | base | --- | [684710, 719743] | --- | --- | --- | --- |
| wide-rung-ragged | 459848ns | -230863.7ns (-32.3%) | [-262958, -197079]ns | [458479, 467599] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 551364ns | -164576.1ns (-23.0%) | [-177544, -155799]ns | [521666, 571380] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 588883ns | -119965.9ns (-16.8%) | [-131219, -91161]ns | [564206, 592984] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 619482ns | -87711.4ns (-12.3%) | [-100535, -84734]ns | [618872, 622836] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 641696ns | -28.5% | -30.4% | -12.1% | -13.1% |
| 1 | 2 | 715281ns | -32.2% | -26.1% | -16.7% | -11.8% |
| 1 | 3 | 720641ns | -36.1% | -20.8% | -18.1% | -14.0% |
| 1 | 4 | 785694ns | -44.3% | -19.5% | -31.5% | -21.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.036 | ok |
| wide-rung-ragged | -0.033 | ok |
| wide-rung-ragged-overread | 0.190 | ok |
| wide-rung-wordround | -0.178 | ok |
| wide-rung-wordround-alias | -0.216 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 130.4ns | 709196.7ns | 0.0% |  |
| wide-rung-ragged | 82.9ns | 464394.8ns | 0.0% |  |
| wide-rung-ragged-overread | 66.7ns | 545622.0ns | 0.0% |  |
| wide-rung-wordround | 118.2ns | 576148.4ns | 0.0% |  |
| wide-rung-wordround-alias | 116.3ns | 610558.0ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 641296.0-788403.8 ns)
  641296.0 |####################
  648651.4 |
  656006.8 |###
  663362.2 |
  670717.6 |
  678072.9 |##########
  685428.3 |###
  692783.7 |######
  700139.1 |
  707494.5 |###
  714849.9 |########################################
  722205.3 |
  729560.7 |
  736916.0 |
  744271.4 |###
  751626.8 |
  758982.2 |
  766337.6 |
  773693.0 |
  781048.4 |#######################
  (4 below, 2 above range)

wide-rung-ragged (n=40, range 420236.3-514329.3 ns)
  420236.3 |
  424941.0 |
  429645.6 |#######
  434350.3 |#########################
  439054.9 |
  443759.6 |
  448464.2 |###
  453168.9 |###
  457873.5 |########################################
  462578.2 |#######
  467282.8 |###
  471987.5 |
  476692.1 |
  481396.8 |#########################
  486101.4 |###
  490806.1 |###
  495510.7 |###
  500215.4 |
  504920.0 |
  509624.7 |###
  (2 below, 2 above range)

wide-rung-ragged-overread (n=40, range 443412.9-636157.5 ns)
  443412.9 |##########
  453050.2 |##########
  462687.4 |#####
  472324.6 |
  481961.8 |
  491599.1 |
  501236.3 |
  510873.5 |##########
  520510.7 |###############
  530148.0 |#####
  539785.2 |####################
  549422.4 |####################
  559059.6 |
  568696.9 |#########################
  578334.1 |
  587971.3 |#####
  597608.6 |
  607245.8 |
  616883.0 |
  626520.2 |########################################
  (5 below, 2 above range)

wide-rung-wordround (n=40, range 537367.7-605532.3 ns)
  537367.7 |###################################
  540775.9 |
  544184.1 |
  547592.4 |
  551000.6 |
  554408.8 |
  557817.0 |#####
  561225.3 |###################################
  564633.5 |#####
  568041.7 |
  571450.0 |
  574858.2 |
  578266.4 |
  581674.7 |
  585082.9 |
  588491.1 |########################################
  591899.4 |####################
  595307.6 |#########################
  598715.8 |##########
  602124.0 |
  (2 below, 3 above range)

wide-rung-wordround-alias (n=40, range 556056.7-646840.0 ns)
  556056.7 |###############
  560595.8 |###
  565135.0 |###
  569674.1 |
  574213.3 |
  578752.5 |
  583291.6 |
  587830.8 |
  592370.0 |
  596909.1 |
  601448.3 |
  605987.5 |
  610526.6 |
  615065.8 |########################################
  619605.0 |############
  624144.1 |######
  628683.3 |########################
  633222.5 |
  637761.6 |
  642300.8 |
  (3 below, 3 above range)

```
