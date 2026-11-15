# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 801 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native beats baseline by 87% (significant)

warm-container-native is -5.01 us (87%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 7.2x slower than the field

warm-container-headroom (5.75 us) is 7.2x the fastest (801 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.87)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-lanes-deferred, warm-container-minimum} vs {warm-container-plusone, warm-container-headroom} (523% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-lanes-deferred, warm-container-minimum} and a slow tier {warm-container-plusone, warm-container-headroom} with a 523% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.2x the fastest

Fastest warm-container-kernel (801 ns) to slowest warm-container-headroom (5.75 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-kernel** at 801.0 ns median (-86.1% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 7.17x (fastest 801.0 ns, slowest 5747.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6054ns | 5810ns | 5700ns | 5880ns | 6929ns | base |
| warm-container-kernel | 903ns | 862ns | 859ns | 871ns | 1045ns | -85.08% |
| warm-container-lanes-deferred | 959ns | 978ns | 861ns | 962ns | 1047ns | -84.16% |
| warm-container-minimum | 957ns | 978ns | 862ns | 959ns | 1048ns | -84.19% |
| warm-container-native | 919ns | 876ns | 860ns | 894ns | 1052ns | -84.82% |
| warm-container-plusone | 5824ns | 5725ns | 5700ns | 5740ns | 6203ns | -3.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5992ns | 5644ns | 6859ns | base | 5.469 |
| warm-container-kernel | 839ns | 798ns | 970ns | -86.00% | 39.054 |
| warm-container-lanes-deferred | 889ns | 800ns | 972ns | -85.15% | 36.840 |
| warm-container-minimum | 888ns | 801ns | 973ns | -85.18% | 36.893 |
| warm-container-native | 853ns | 799ns | 976ns | -85.76% | 38.401 |
| warm-container-plusone | 5763ns | 5644ns | 6141ns | -3.81% | 5.686 |

## Performance model

- Peak throughput: **41.056 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.702 | 13.9% |
| warm-container-kernel | 40.909 | 99.6% |
| warm-container-lanes-deferred | 36.150 | 88.1% |
| warm-container-minimum | 36.142 | 88.0% |
| warm-container-native | 40.196 | 97.9% |
| warm-container-plusone | 5.798 | 14.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6054ns | 6054ns | base |
| warm-container-kernel | 903ns | 903ns | -85.08% |
| warm-container-lanes-deferred | 959ns | 959ns | -84.16% |
| warm-container-minimum | 957ns | 957ns | -84.19% |
| warm-container-native | 919ns | 919ns | -84.82% |
| warm-container-plusone | 5824ns | 5824ns | -3.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5747ns | base | --- | [5662, 5940] | --- | --- | --- | --- |
| warm-container-kernel | 801ns | -4923.9ns (-85.7%) | [-5005, -4848]ns | [800, 803] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 906ns | -4876.3ns (-84.8%) | [-4970, -4824]ns | [811, 969] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 907ns | -4921.2ns (-85.6%) | [-4992, -4848]ns | [806, 967] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 815ns | -4945.2ns (-86.0%) | [-5138, -4847]ns | [806, 828] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 5652ns | -74.8ns (-1.3%) | [-253, -0]ns | [5647, 5711] | YES (adj: no) | 0.0533 | 0.0533 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5958ns | -86.6% | -86.6% | -83.7% | -86.6% | -5.3% |
| 2 | 5644ns | -85.8% | -85.9% | -82.8% | -85.8% | +0.0% |
| 3 | 5648ns | -85.8% | -85.8% | -82.9% | -85.9% | +1.2% |
| 4 | 5970ns | -86.6% | -86.6% | -83.7% | -86.6% | -4.8% |
| 5 | 5645ns | -85.9% | -85.8% | -82.8% | -85.8% | +0.0% |
| 6 | 5644ns | -85.9% | -85.8% | -82.8% | -85.8% | +0.0% |
| 7 | 5739ns | -86.0% | -85.6% | -83.1% | -86.0% | -1.6% |
| 8 | 5646ns | -85.8% | -85.0% | -82.9% | -84.5% | +0.0% |
| 9 | 5642ns | -85.8% | -85.3% | -82.8% | -83.9% | +0.1% |
| 10 | 5645ns | -85.8% | -85.6% | -82.8% | -83.9% | -0.0% |
| 11 | 5718ns | -86.0% | -83.0% | -86.0% | -85.7% | -1.3% |
| 12 | 5646ns | -85.8% | -82.9% | -85.8% | -85.5% | -0.0% |
| 13 | 5645ns | -85.9% | -82.9% | -85.8% | -85.3% | +1.1% |
| 14 | 5643ns | -85.8% | -82.8% | -85.8% | -85.5% | -0.0% |
| 15 | 5647ns | -85.8% | -82.8% | -85.8% | -85.6% | -0.1% |
| 16 | 5649ns | -85.8% | -82.8% | -85.8% | -85.3% | +8.8% |
| 17 | 5660ns | -85.8% | -82.8% | -85.8% | -85.5% | +21.6% |
| 18 | 6317ns | -87.3% | -84.6% | -87.3% | -87.2% | +8.4% |
| 19 | 5698ns | -86.0% | -83.0% | -86.0% | -85.8% | +1.3% |
| 20 | 5729ns | -86.0% | -83.1% | -86.0% | -85.8% | -0.2% |
| 21 | 5664ns | -82.9% | -82.9% | -85.8% | -85.2% | +4.9% |
| 22 | 5755ns | -83.1% | -83.1% | -86.0% | -86.1% | +1.3% |
| 23 | 5939ns | -83.7% | -83.7% | -86.5% | -86.5% | -3.0% |
| 24 | 5942ns | -83.7% | -83.6% | -86.5% | -86.5% | -4.9% |
| 25 | 5938ns | -85.2% | -83.7% | -84.4% | -86.3% | -4.9% |
| 26 | 5942ns | -84.8% | -83.7% | -83.7% | -86.6% | -4.4% |
| 27 | 5938ns | -83.6% | -83.6% | -83.6% | -86.5% | -3.7% |
| 28 | 5719ns | -83.0% | -83.0% | -83.0% | -85.9% | -1.3% |
| 29 | 5872ns | -83.5% | -83.5% | -83.5% | -86.4% | -3.9% |
| 30 | 5935ns | -83.7% | -83.7% | -83.6% | -86.4% | -4.9% |
| 31 | 6930ns | -88.5% | -88.4% | -88.1% | -85.5% | -18.2% |
| 32 | 6892ns | -88.4% | -88.4% | -88.3% | -85.9% | -18.0% |
| 33 | 6889ns | -88.4% | -88.4% | -88.1% | -85.9% | -18.0% |
| 34 | 6847ns | -88.3% | -88.2% | -86.8% | -85.8% | -17.4% |
| 35 | 6842ns | -88.2% | -88.2% | -86.7% | -85.7% | -17.4% |
| 36 | 5963ns | -86.6% | -86.6% | -84.8% | -83.7% | -4.1% |
| 37 | 5652ns | -85.9% | -85.6% | -83.8% | -82.7% | +0.7% |
| 38 | 6816ns | -88.3% | -88.2% | -86.7% | -85.8% | -14.8% |
| 39 | 6849ns | -88.3% | -88.0% | -86.8% | -85.8% | -14.9% |
| 40 | 6805ns | -88.2% | -88.1% | -86.7% | -88.0% | -14.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.692 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.803 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.867 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.815 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.780 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.644 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 22/40, lost 9/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.8ns | 5991.6ns | 0.0% |  |
| warm-container-kernel | 2.8ns | 839.0ns | 0.3% |  |
| warm-container-lanes-deferred | 2.7ns | 889.5ns | 0.3% |  |
| warm-container-minimum | 2.7ns | 888.2ns | 0.3% |  |
| warm-container-native | 2.8ns | 853.3ns | 0.3% |  |
| warm-container-plusone | 2.9ns | 5763.4ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5644.4-6858.6 ns)
   5644.4 |########################################
   5705.1 |################
   5765.8 |
   5826.5 |###
   5887.2 |####################
   5947.9 |##########
   6008.6 |
   6069.3 |
   6130.0 |
   6190.8 |
   6251.5 |
   6312.2 |###
   6372.9 |
   6433.6 |
   6494.3 |
   6555.0 |
   6615.7 |
   6676.4 |
   6737.1 |
   6797.9 |################
  (4 below, 3 above range)

warm-container-kernel (n=40, range 798.1-970.2 ns)
    798.1 |########################################
    806.7 |#
    815.3 |
    823.9 |
    832.5 |
    841.1 |
    849.7 |
    858.4 |
    867.0 |
    875.6 |#
    884.2 |
    892.8 |
    901.4 |#
    910.0 |
    918.6 |
    927.2 |
    935.8 |
    944.4 |
    953.0 |
    961.6 |########
  (4 below, 3 above range)

warm-container-lanes-deferred (n=40, range 799.6-972.5 ns)
    799.6 |#########################
    808.2 |#######
    816.9 |##
    825.5 |#####
    834.1 |
    842.8 |##
    851.4 |
    860.1 |
    868.7 |
    877.4 |
    886.0 |
    894.7 |
    903.3 |
    911.9 |
    920.6 |
    929.2 |
    937.9 |
    946.5 |
    955.2 |
    963.8 |########################################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 800.7-972.8 ns)
    800.7 |########################################
    809.3 |####
    817.9 |########
    826.5 |
    835.1 |
    843.7 |
    852.3 |
    860.9 |
    869.5 |
    878.1 |
    886.7 |
    895.3 |
    903.9 |########################
    912.5 |####
    921.1 |####
    929.7 |
    938.3 |
    946.9 |
    955.5 |
    964.1 |########################################
  (4 below, 5 above range)

warm-container-native (n=40, range 799.4-976.4 ns)
    799.4 |########################################
    808.2 |#########################
    817.1 |#######
    825.9 |#######
    834.8 |###
    843.6 |
    852.5 |
    861.3 |
    870.2 |###
    879.0 |
    887.9 |
    896.7 |
    905.6 |#######
    914.4 |
    923.3 |
    932.1 |
    941.0 |
    949.8 |
    958.7 |
    967.5 |#############################
  (5 below, 1 above range)

warm-container-plusone (n=40, range 5643.5-6141.2 ns)
   5643.5 |########################################
   5668.4 |#########
   5693.3 |#######
   5718.2 |####
   5743.0 |##
   5767.9 |##
   5792.8 |##
   5817.7 |####
   5842.6 |##
   5867.5 |
   5892.4 |
   5917.3 |##
   5942.1 |
   5967.0 |
   5991.9 |
   6016.8 |
   6041.7 |
   6066.6 |
   6091.5 |
   6116.4 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.64 (measurement drift or warm-up artifact)
