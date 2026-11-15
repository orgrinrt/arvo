# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (738.96 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 106.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 86% (significant)

warm-container-native is -637.68 us (86%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 6.9x slower than the field

warm-container-headroom (738.96 us) is 6.9x the fastest (106.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (106.43 us) and warm-container-minimum (106.56 us) differ by 0.12%, inside the noise, even though the wider field spreads 594.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.89)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (591% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 591% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.9x the fastest

Fastest warm-container-native (106.43 us) to slowest warm-container-headroom (738.96 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum is inconsistent: worst-20% is 1.6x its best-20%

warm-container-minimum's best 20% of batches run at 104.80 us but its worst 20% at 164.96 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 106429.6 ns median (-85.6% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.94x (fastest 106429.6 ns, slowest 738955.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 815641ns | 739397ns | 729499ns | 742259ns | 1121930ns | base |
| warm-container-minimum | 120224ns | 106840ns | 104973ns | 110062ns | 165958ns | -85.26% |
| warm-container-native | 107239ns | 106602ns | 104980ns | 106619ns | 111357ns | -86.85% |
| warm-container-plusone | 769832ns | 737075ns | 731160ns | 739375ns | 899873ns | -5.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 814628ns | 728811ns | 1119466ns | base | 5.149 |
| warm-container-minimum | 119832ns | 104798ns | 164961ns | -85.29% | 35.001 |
| warm-container-native | 107037ns | 104790ns | 111154ns | -86.86% | 39.185 |
| warm-container-plusone | 769048ns | 730296ns | 898721ns | -5.60% | 5.454 |

## Performance model

- Peak throughput: **40.026 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.676 | 14.2% |
| warm-container-minimum | 39.361 | 98.3% |
| warm-container-native | 39.409 | 98.5% |
| warm-container-plusone | 5.696 | 14.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 815641ns | 815641ns | base |
| warm-container-minimum | 120224ns | 120224ns | -85.26% |
| warm-container-native | 107239ns | 107239ns | -86.85% |
| warm-container-plusone | 769832ns | 769832ns | -5.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 738955ns | base | --- | [731418, 747971] | --- | --- | --- | --- |
| warm-container-minimum | 106560ns | -627091.1ns (-84.9%) | [-635455, -624776]ns | [105424, 107893] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 106430ns | -633402.1ns (-85.7%) | [-643018, -625546]ns | [105319, 107160] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 736378ns | +2079.3ns (+0.3%) | [+303, +5136]ns | [732785, 744973] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|
| 1 | 764753ns | -82.4% | -85.9% | +37.0% |
| 2 | 779510ns | -79.8% | -86.1% | +36.7% |
| 3 | 858107ns | -81.6% | -87.6% | +23.2% |
| 4 | 772257ns | -78.6% | -85.8% | +13.3% |
| 5 | 766256ns | -79.1% | -86.3% | +10.1% |
| 6 | 779693ns | -79.6% | -86.1% | +1.5% |
| 7 | 1907375ns | -91.5% | -94.3% | -60.8% |
| 8 | 1553752ns | -88.9% | -92.7% | -51.7% |
| 9 | 1173858ns | -85.1% | -90.4% | -36.5% |
| 10 | 1131174ns | -85.1% | -89.8% | -35.0% |
| 11 | 729267ns | -85.4% | -85.5% | +0.4% |
| 12 | 735531ns | -85.7% | -85.4% | -0.6% |
| 13 | 756592ns | -85.6% | -86.0% | -2.9% |
| 14 | 741454ns | -85.9% | -85.9% | -1.3% |
| 15 | 744519ns | -85.5% | -85.9% | -0.6% |
| 16 | 744746ns | -85.6% | -85.9% | +0.3% |
| 17 | 733613ns | -85.5% | -85.7% | +1.4% |
| 18 | 731385ns | -85.6% | -85.4% | +0.0% |
| 19 | 748502ns | -85.9% | -85.9% | -2.6% |
| 20 | 757122ns | -86.1% | -86.1% | -3.3% |
| 21 | 731450ns | -85.6% | -85.4% | +1.8% |
| 22 | 728027ns | -85.5% | -85.2% | +1.2% |
| 23 | 727150ns | -85.6% | -84.9% | +0.8% |
| 24 | 729898ns | -85.6% | -85.4% | +0.2% |
| 25 | 752359ns | -85.9% | -85.9% | -2.8% |
| 26 | 738605ns | -85.6% | -85.6% | -1.3% |
| 27 | 729282ns | -85.1% | -85.4% | +0.1% |
| 28 | 728824ns | -85.2% | -85.1% | +0.4% |
| 29 | 728858ns | -85.0% | -85.4% | +0.2% |
| 30 | 730642ns | -85.4% | -84.9% | +0.1% |
| 31 | 747440ns | -85.9% | -86.0% | +0.2% |
| 32 | 739306ns | -85.8% | -85.8% | +0.8% |
| 33 | 731381ns | -85.6% | -85.7% | +0.6% |
| 34 | 731604ns | -85.7% | -85.5% | +0.3% |
| 35 | 729792ns | -85.7% | -85.2% | +0.4% |
| 36 | 729290ns | -85.6% | -85.6% | +1.1% |
| 37 | 730815ns | -85.3% | -85.6% | +2.8% |
| 38 | 733281ns | -85.7% | -85.7% | +0.4% |
| 39 | 747208ns | -85.9% | -85.9% | -0.4% |
| 40 | 730426ns | -85.4% | -85.6% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.600 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.894 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.448 | moderate+ |
| warm-container-plusone | 0.802 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 13/40, lost 24/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 87.4ns | 814627.6ns | 0.0% |  |
| warm-container-minimum | 19.2ns | 119832.4ns | 0.0% |  |
| warm-container-native | 3.4ns | 107037.2ns | 0.0% |  |
| warm-container-plusone | 21.0ns | 769048.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 728811.4-1119465.8 ns)
  728811.4 |########################################
  748344.1 |##########
  767876.8 |#####
  787409.5 |
  806942.3 |
  826475.0 |
  846007.7 |#
  865540.4 |
  885073.1 |
  904605.9 |
  924138.6 |
  943671.3 |
  963204.0 |
  982736.8 |
  1002269.5 |
  1021802.2 |
  1041334.9 |
  1060867.7 |
  1080400.4 |
  1099933.1 |
  (2 below, 4 above range)

warm-container-minimum (n=40, range 104797.7-164961.2 ns)
  104797.7 |########################################
  107805.9 |#########
  110814.1 |
  113822.2 |
  116830.4 |
  119838.6 |
  122846.8 |
  125854.9 |
  128863.1 |
  131871.3 |#
  134879.5 |
  137887.6 |
  140895.8 |
  143904.0 |
  146912.2 |
  149920.3 |
  152928.5 |
  155936.7 |#####
  158944.9 |#
  161953.0 |###
  (4 below, 3 above range)

warm-container-native (n=40, range 104790.1-111154.1 ns)
  104790.1 |########################################
  105108.3 |############################
  105426.5 |#####
  105744.7 |
  106062.9 |#################
  106381.1 |##################################
  106699.3 |#####
  107017.5 |
  107335.7 |#####
  107653.9 |###########
  107972.1 |###########
  108290.3 |#####
  108608.5 |#####
  108926.7 |#####
  109244.9 |
  109563.1 |###########
  109881.3 |
  110199.5 |#####
  110517.7 |
  110835.9 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 730295.8-898721.0 ns)
  730295.8 |########################################
  738717.0 |################
  747138.3 |######
  755559.6 |##
  763980.8 |
  772402.1 |
  780823.3 |
  789244.6 |##
  797665.9 |
  806087.1 |
  814508.4 |
  822929.6 |
  831350.9 |
  839772.2 |##
  848193.4 |
  856614.7 |
  865035.9 |
  873457.2 |##
  881878.5 |
  890299.7 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=28.6% (high variance, measurements may be unstable)
- **warm-container-headroom**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **warm-container-minimum**: CV=20.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.80 (measurement drift or warm-up artifact)
