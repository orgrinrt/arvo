# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 187 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 104% (significant)

warm-container-native is -8.77 us (104%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 45.1x slower than the field

warm-container-headroom (8.41 us) is 45.1x the fastest (187 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.87)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-plusone, warm-container-headroom} (1862% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-plusone, warm-container-headroom} with a 1862% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 45.1x the fastest

Fastest warm-container-native (187 ns) to slowest warm-container-headroom (8.41 us): 45.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 186.7 ns median (-97.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 45.07x (fastest 186.7 ns, slowest 8414.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8925ns | 8477ns | 8351ns | 8640ns | 10355ns | base |
| warm-container-kernel | 487ns | 483ns | 474ns | 486ns | 506ns | -94.54% |
| warm-container-lanes-deferred | 489ns | 486ns | 474ns | 485ns | 516ns | -94.52% |
| warm-container-minimum | 270ns | 252ns | 247ns | 259ns | 327ns | -96.97% |
| warm-container-native | 252ns | 250ns | 247ns | 250ns | 261ns | -97.18% |
| warm-container-plusone | 8471ns | 8396ns | 8345ns | 8424ns | 8736ns | -5.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8857ns | 8287ns | 10275ns | base | 3.700 |
| warm-container-kernel | 429ns | 418ns | 446ns | -95.16% | 76.432 |
| warm-container-lanes-deferred | 429ns | 418ns | 454ns | -95.16% | 76.380 |
| warm-container-minimum | 203ns | 184ns | 245ns | -97.71% | 161.654 |
| warm-container-native | 188ns | 184ns | 198ns | -97.87% | 173.905 |
| warm-container-plusone | 8404ns | 8284ns | 8666ns | -5.11% | 3.899 |

## Performance model

- Peak throughput: **178.451 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.894 | 2.2% |
| warm-container-kernel | 77.247 | 43.3% |
| warm-container-lanes-deferred | 77.210 | 43.3% |
| warm-container-minimum | 172.282 | 96.5% |
| warm-container-native | 175.512 | 98.4% |
| warm-container-plusone | 3.935 | 2.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8925ns | 8925ns | base |
| warm-container-kernel | 487ns | 487ns | -94.54% |
| warm-container-lanes-deferred | 489ns | 489ns | -94.52% |
| warm-container-minimum | 270ns | 270ns | -96.97% |
| warm-container-native | 252ns | 252ns | -97.18% |
| warm-container-plusone | 8471ns | 8471ns | -5.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8415ns | base | --- | [8333, 8506] | --- | --- | --- | --- |
| warm-container-kernel | 424ns | -7984.2ns (-94.9%) | [-8079, -7901]ns | [419, 436] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 424ns | -7966.7ns (-94.7%) | [-8080, -7902]ns | [421, 426] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 190ns | -8224.6ns (-97.7%) | [-8314, -8135]ns | [188, 193] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 187ns | -8228.0ns (-97.8%) | [-8320, -8148]ns | [186, 188] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8328ns | -95.0ns (-1.1%) | [-189, -0]ns | [8301, 8420] | YES (adj: no) | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8880ns | -95.3% | -95.3% | -97.2% | -97.9% | -6.7% |
| 2 | 10052ns | -95.8% | -95.8% | -97.6% | -98.1% | -17.6% |
| 3 | 8355ns | -95.0% | -95.0% | -97.0% | -97.7% | -0.9% |
| 4 | 8345ns | -95.0% | -95.0% | -97.1% | -97.7% | +3.8% |
| 5 | 8289ns | -94.9% | -94.9% | -97.0% | -97.7% | +1.6% |
| 6 | 9942ns | -95.6% | -95.8% | -97.5% | -98.1% | -15.5% |
| 7 | 10081ns | -95.7% | -95.9% | -97.6% | -98.1% | -10.3% |
| 8 | 9542ns | -95.6% | -95.6% | -97.5% | -98.0% | -10.2% |
| 9 | 9402ns | -95.6% | -95.6% | -97.4% | -98.0% | -10.0% |
| 10 | 9517ns | -95.6% | -95.6% | -97.4% | -98.0% | -11.6% |
| 11 | 8491ns | -94.9% | -95.0% | -97.8% | -97.8% | -2.3% |
| 12 | 8439ns | -94.7% | -94.9% | -97.7% | -97.7% | -1.8% |
| 13 | 8427ns | -94.8% | -94.9% | -97.7% | -97.8% | -1.4% |
| 14 | 8285ns | -94.9% | -94.7% | -97.7% | -97.8% | +0.0% |
| 15 | 8286ns | -94.9% | -94.6% | -97.7% | -97.7% | +0.5% |
| 16 | 8287ns | -94.9% | -94.7% | -97.8% | -97.8% | -0.0% |
| 17 | 8290ns | -94.9% | -94.7% | -97.7% | -97.0% | +0.0% |
| 18 | 8317ns | -94.9% | -94.7% | -97.7% | -97.7% | +5.8% |
| 19 | 8322ns | -94.9% | -94.7% | -97.7% | -97.7% | +1.2% |
| 20 | 8402ns | -95.0% | -93.6% | -97.7% | -97.8% | +2.5% |
| 21 | 8458ns | -95.0% | -95.0% | -97.8% | -97.8% | -2.0% |
| 22 | 8458ns | -95.0% | -95.0% | -97.7% | -97.8% | -2.0% |
| 23 | 8521ns | -95.1% | -95.0% | -97.7% | -97.8% | -2.0% |
| 24 | 8472ns | -95.1% | -95.0% | -97.8% | -97.8% | -2.2% |
| 25 | 8327ns | -95.0% | -94.9% | -97.8% | -97.8% | -0.5% |
| 26 | 8290ns | -94.9% | -94.9% | -97.6% | -97.8% | +2.3% |
| 27 | 8342ns | -95.0% | -94.9% | -97.7% | -97.8% | -0.6% |
| 28 | 8287ns | -95.0% | -94.8% | -97.7% | -97.8% | +0.3% |
| 29 | 8288ns | -95.0% | -94.6% | -97.7% | -97.8% | +0.1% |
| 30 | 8309ns | -95.0% | -94.8% | -97.6% | -97.7% | +3.3% |
| 31 | 10912ns | -95.9% | -96.2% | -98.3% | -98.3% | -21.9% |
| 32 | 10911ns | -95.9% | -96.1% | -98.3% | -98.3% | -22.8% |
| 33 | 10552ns | -95.8% | -96.0% | -98.2% | -98.2% | -20.1% |
| 34 | 8288ns | -94.6% | -94.9% | -97.8% | -97.7% | +0.2% |
| 35 | 8352ns | -94.7% | -94.9% | -97.8% | -97.8% | -0.4% |
| 36 | 8326ns | -94.7% | -94.9% | -97.7% | -97.8% | +0.1% |
| 37 | 8339ns | -94.7% | -95.0% | -97.8% | -97.8% | -0.3% |
| 38 | 9132ns | -95.2% | -95.4% | -97.9% | -98.0% | -8.6% |
| 39 | 10082ns | -95.6% | -95.8% | -98.2% | -98.2% | -16.3% |
| 40 | 9670ns | -95.4% | -95.7% | -98.1% | -98.1% | -11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.557 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.756 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.200 | ok |
| warm-container-minimum | 0.868 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.012 | ok |
| warm-container-plusone | 0.191 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 25/40, lost 11/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.3ns | 8856.6ns | 0.0% |  |
| warm-container-kernel | 1.9ns | 428.7ns | 0.4% |  |
| warm-container-lanes-deferred | 2.2ns | 429.0ns | 0.5% |  |
| warm-container-minimum | 2.8ns | 202.7ns | 1.4% |  |
| warm-container-native | 2.4ns | 188.4ns | 1.3% |  |
| warm-container-plusone | 2.5ns | 8404.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8287.2-10275.4 ns)
   8287.2 |########################################
   8386.7 |################
   8486.1 |#####
   8585.5 |
   8684.9 |
   8784.3 |##
   8883.7 |
   8983.1 |
   9082.5 |##
   9181.9 |
   9281.3 |
   9380.7 |##
   9480.1 |#####
   9579.6 |##
   9679.0 |
   9778.4 |
   9877.8 |##
   9977.2 |##
  10076.6 |#####
  10176.0 |
  (4 below, 3 above range)

warm-container-kernel (n=40, range 417.5-446.3 ns)
    417.5 |########################################
    418.9 |####################
    420.4 |#####
    421.8 |
    423.3 |####################
    424.7 |#####
    426.1 |##########
    427.6 |
    429.0 |
    430.5 |
    431.9 |
    433.3 |#####
    434.8 |
    436.2 |###############
    437.7 |
    439.1 |
    440.5 |#####
    442.0 |##########
    443.4 |
    444.8 |#########################
  (5 below, 3 above range)

warm-container-lanes-deferred (n=40, range 418.0-454.4 ns)
    418.0 |##############################
    419.8 |####################
    421.6 |##########
    423.4 |####################
    425.2 |########################################
    427.1 |#####
    428.9 |
    430.7 |#####
    432.5 |#####
    434.3 |
    436.2 |
    438.0 |#####
    439.8 |##########
    441.6 |###############
    443.4 |
    445.3 |#####
    447.1 |
    448.9 |
    450.7 |
    452.5 |
  (5 below, 1 above range)

warm-container-minimum (n=40, range 183.9-245.4 ns)
    183.9 |###############################
    187.0 |########################################
    190.0 |##########################
    193.1 |########
    196.2 |########
    199.3 |
    202.3 |
    205.4 |
    208.5 |
    211.6 |
    214.7 |
    217.7 |
    220.8 |
    223.9 |
    227.0 |
    230.0 |
    233.1 |
    236.2 |
    239.3 |
    242.3 |###############################
  (4 below, 3 above range)

warm-container-native (n=40, range 183.6-198.2 ns)
    183.6 |########################################
    184.4 |#################
    185.1 |#################
    185.8 |#################
    186.5 |##################################
    187.3 |###########
    188.0 |###########
    188.7 |######################
    189.4 |######################
    190.2 |###########
    190.9 |
    191.6 |
    192.4 |#####
    193.1 |
    193.8 |
    194.5 |
    195.3 |
    196.0 |
    196.7 |
    197.4 |
  (2 below, 1 above range)

warm-container-plusone (n=40, range 8284.4-8665.7 ns)
   8284.4 |########################################
   8303.5 |##############
   8322.5 |#######
   8341.6 |#######
   8360.6 |
   8379.7 |###
   8398.8 |###
   8417.8 |##############
   8436.9 |###
   8456.0 |###
   8475.0 |###
   8494.1 |
   8513.2 |###
   8532.2 |###
   8551.3 |
   8570.4 |#######
   8589.4 |
   8608.5 |###
   8627.5 |
   8646.6 |###
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.87 (measurement drift or warm-up artifact)
