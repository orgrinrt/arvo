# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (13.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 399 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 97% (significant)

warm-container-lanes-deferred is -13.06 us (97%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 33.8x slower than the field

warm-container-headroom (13.51 us) is 33.8x the fastest (399 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-lanes-deferred, warm-container-native) are a dead heat (<1%)

warm-container-lanes-deferred (399 ns) and warm-container-native (401 ns) differ by 0.35%, inside the noise, even though the wider field spreads 3284.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.84)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-minimum, warm-container-headroom} (3173% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-minimum, warm-container-headroom} with a 3173% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 33.8x the fastest

Fastest warm-container-lanes-deferred (399 ns) to slowest warm-container-headroom (13.51 us): 33.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum's edge over baseline is significant but tiny (23 ns, 0.17%)

warm-container-minimum differs from baseline warm-container-headroom by 23 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 399.2 ns median (-97.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 33.84x (fastest 399.2 ns, slowest 13508.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 13922ns | 13593ns | 13085ns | 13713ns | 15388ns | base |
| warm-container-kernel | 477ns | 468ns | 453ns | 472ns | 517ns | -96.57% |
| warm-container-lanes-deferred | 465ns | 462ns | 460ns | 463ns | 477ns | -96.66% |
| warm-container-minimum | 13504ns | 13340ns | 13108ns | 13379ns | 14274ns | -3.00% |
| warm-container-native | 491ns | 462ns | 452ns | 483ns | 556ns | -96.47% |
| warm-container-plusone | 13364ns | 13319ns | 13124ns | 13331ns | 13701ns | -4.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 13832ns | 13004ns | 15283ns | base | 10.068 |
| warm-container-kernel | 411ns | 390ns | 446ns | -97.03% | 338.714 |
| warm-container-lanes-deferred | 401ns | 395ns | 411ns | -97.10% | 347.430 |
| warm-container-minimum | 13415ns | 13020ns | 14180ns | -3.02% | 10.382 |
| warm-container-native | 424ns | 390ns | 480ns | -96.93% | 328.422 |
| warm-container-plusone | 13276ns | 13036ns | 13607ns | -4.02% | 10.490 |

## Performance model

- Peak throughput: **357.259 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 10.309 | 2.9% |
| warm-container-kernel | 344.542 | 96.4% |
| warm-container-lanes-deferred | 348.858 | 97.6% |
| warm-container-minimum | 10.512 | 2.9% |
| warm-container-native | 347.639 | 97.3% |
| warm-container-plusone | 10.527 | 2.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 13922ns | 13922ns | base |
| warm-container-kernel | 477ns | 477ns | -96.57% |
| warm-container-lanes-deferred | 465ns | 465ns | -96.66% |
| warm-container-minimum | 13504ns | 13504ns | -3.00% |
| warm-container-native | 491ns | 491ns | -96.47% |
| warm-container-plusone | 13364ns | 13364ns | -4.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 13509ns | base | --- | [13218, 13684] | --- | --- | --- | --- |
| warm-container-kernel | 404ns | -13115.4ns (-97.1%) | [-13289, -12815]ns | [396, 412] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 399ns | -13110.2ns (-97.0%) | [-13281, -12816]ns | [398, 400] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 13248ns | -193.4ns (-1.4%) | [-632, -29]ns | [13213, 13300] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-native | 401ns | -13048.1ns (-96.6%) | [-13273, -12797]ns | [397, 444] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 13229ns | no significant difference | [-526, +32]ns | [13211, 13274] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 15763ns | -97.5% | -97.4% | -5.6% | -97.0% | -16.9% |
| 2 | 15775ns | -97.5% | -97.4% | -5.8% | -97.0% | -17.7% |
| 3 | 15636ns | -97.5% | -97.4% | -8.1% | -96.9% | -16.9% |
| 4 | 13513ns | -97.1% | -97.1% | +0.8% | -96.5% | -3.9% |
| 5 | 13346ns | -97.1% | -97.0% | +8.1% | -96.4% | -1.9% |
| 6 | 14848ns | -97.4% | -97.2% | -9.0% | -96.8% | -10.7% |
| 7 | 14780ns | -97.4% | -97.2% | -11.2% | -96.8% | -9.3% |
| 8 | 14793ns | -97.4% | -97.2% | -10.7% | -96.8% | -11.7% |
| 9 | 13578ns | -97.1% | -96.9% | -4.0% | -96.5% | -3.9% |
| 10 | 13695ns | -97.1% | -97.0% | -3.1% | -96.5% | -3.5% |
| 11 | 13052ns | -96.6% | -97.0% | -0.5% | -97.0% | +5.2% |
| 12 | 13050ns | -96.6% | -97.0% | -0.4% | -97.0% | +8.3% |
| 13 | 13160ns | -96.6% | -97.0% | -0.3% | -97.0% | -1.2% |
| 14 | 14440ns | -96.9% | -97.2% | -9.9% | -97.3% | -5.7% |
| 15 | 14843ns | -97.0% | -97.3% | -12.1% | -97.4% | -10.8% |
| 16 | 14360ns | -96.9% | -97.2% | -7.7% | -97.3% | -7.2% |
| 17 | 14853ns | -97.0% | -97.3% | -11.7% | -97.3% | -10.4% |
| 18 | 13626ns | -96.7% | -97.1% | -4.7% | -97.1% | -2.2% |
| 19 | 13192ns | -96.6% | -97.0% | -1.1% | -97.1% | +0.3% |
| 20 | 13674ns | -96.8% | -97.1% | -4.6% | -97.1% | -3.0% |
| 21 | 13657ns | -97.1% | -97.1% | -3.2% | -97.2% | -3.1% |
| 22 | 15756ns | -97.5% | -97.4% | -15.7% | -97.5% | -15.4% |
| 23 | 14736ns | -97.3% | -97.3% | -10.2% | -97.4% | -10.3% |
| 24 | 14555ns | -97.3% | -97.3% | -9.1% | -97.2% | -8.9% |
| 25 | 13299ns | -97.0% | -97.0% | -0.4% | -96.2% | +0.6% |
| 26 | 13309ns | -97.0% | -97.0% | -0.1% | -96.6% | +0.2% |
| 27 | 13673ns | -97.1% | -97.1% | -2.2% | -96.8% | -3.2% |
| 28 | 13505ns | -97.1% | -97.0% | -1.8% | -96.7% | -2.2% |
| 29 | 13310ns | -97.0% | -97.0% | -0.3% | -96.7% | -0.6% |
| 30 | 13230ns | -97.0% | -96.9% | +0.6% | -96.7% | +0.6% |
| 31 | 13205ns | -96.9% | -97.0% | +3.6% | -96.9% | +0.1% |
| 32 | 13045ns | -96.8% | -96.9% | +5.9% | -96.9% | +1.2% |
| 33 | 13035ns | -96.8% | -97.0% | +4.4% | -96.9% | +1.4% |
| 34 | 13008ns | -96.8% | -97.0% | +5.4% | -97.0% | +4.6% |
| 35 | 12986ns | -96.9% | -96.9% | +5.4% | -96.9% | +5.0% |
| 36 | 13045ns | -96.8% | -96.9% | +3.9% | -97.0% | +1.9% |
| 37 | 12985ns | -96.8% | -96.9% | +2.0% | -96.9% | +1.9% |
| 38 | 12999ns | -96.9% | -96.9% | +1.6% | -97.0% | +1.5% |
| 39 | 12985ns | -96.8% | -96.9% | +1.7% | -97.0% | +1.7% |
| 40 | 12990ns | -96.8% | -96.9% | +1.9% | -97.0% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.612 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.837 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.649 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.694 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.745 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.242 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 27/40, lost 13/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 24/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.5ns | 13832.2ns | 0.0% |  |
| warm-container-kernel | 2.2ns | 411.2ns | 0.5% |  |
| warm-container-lanes-deferred | 2.4ns | 400.8ns | 0.6% |  |
| warm-container-minimum | 2.6ns | 13414.6ns | 0.0% |  |
| warm-container-native | 2.5ns | 424.0ns | 0.6% |  |
| warm-container-plusone | 2.6ns | 13276.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 13004.0-15283.4 ns)
  13004.0 |########################################
  13118.0 |##########################
  13232.0 |####################
  13345.9 |######
  13459.9 |#############
  13573.9 |#################################
  13687.8 |######
  13801.8 |
  13915.8 |
  14029.7 |
  14143.7 |
  14257.7 |######
  14371.7 |######
  14485.6 |######
  14599.6 |
  14713.6 |####################
  14827.5 |####################
  14941.5 |
  15055.5 |
  15169.4 |
  (5 below, 4 above range)

warm-container-kernel (n=40, range 390.5-445.6 ns)
    390.5 |#################################
    393.2 |##########################
    396.0 |########################################
    398.7 |######
    401.5 |
    404.2 |
    407.0 |#############
    409.8 |#################################
    412.5 |####################
    415.3 |
    418.0 |
    420.8 |
    423.5 |
    426.3 |
    429.0 |
    431.8 |
    434.6 |
    437.3 |
    440.1 |######
    442.8 |#################################
  (4 below, 4 above range)

warm-container-lanes-deferred (n=40, range 395.1-411.0 ns)
    395.1 |###########
    395.9 |###########
    396.7 |######################
    397.5 |######################
    398.3 |#################
    399.1 |###########
    399.9 |########################################
    400.7 |
    401.5 |#####
    402.3 |#################
    403.1 |#####
    403.8 |
    404.6 |
    405.4 |
    406.2 |
    407.0 |
    407.8 |#####
    408.6 |
    409.4 |
    410.2 |###########
  (4 below, 4 above range)

warm-container-minimum (n=40, range 13020.1-14179.9 ns)
  13020.1 |#################
  13078.1 |#############
  13136.0 |
  13194.0 |########################################
  13252.0 |###############################
  13310.0 |
  13368.0 |####
  13426.0 |
  13484.0 |####
  13542.0 |####
  13600.0 |########
  13658.0 |#############
  13716.0 |
  13774.0 |####
  13831.9 |
  13889.9 |
  13947.9 |
  14005.9 |
  14063.9 |
  14121.9 |
  (4 below, 4 above range)

warm-container-native (n=40, range 389.8-479.9 ns)
    389.8 |############################
    394.3 |########################################
    398.8 |##################################
    403.3 |#####
    407.8 |
    412.3 |
    416.8 |
    421.3 |
    425.8 |
    430.4 |
    434.9 |#####
    439.4 |#####
    443.9 |#################
    448.4 |
    452.9 |
    457.4 |
    461.9 |
    466.4 |
    470.9 |##################################
    475.4 |######################
  (5 below, 1 above range)

warm-container-plusone (n=40, range 13036.0-13607.5 ns)
  13036.0 |####
  13064.6 |####
  13093.1 |########
  13121.7 |
  13150.3 |
  13178.9 |######################
  13207.4 |########################################
  13236.0 |#################
  13264.6 |####
  13293.2 |########
  13321.7 |#################
  13350.3 |####
  13378.9 |####
  13407.5 |
  13436.0 |
  13464.6 |
  13493.2 |
  13521.8 |
  13550.3 |
  13578.9 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.74 (measurement drift or warm-up artifact)
