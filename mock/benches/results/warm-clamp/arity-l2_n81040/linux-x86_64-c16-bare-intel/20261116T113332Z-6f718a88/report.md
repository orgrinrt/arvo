# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-acc64 dominates: 105% faster than the next best (warm-clamp-head)

warm-clamp-acc64 (118.58 us) leads warm-clamp-head (243.24 us) by 105%, a clear separation rather than a photo finish. CV 0.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn is an outlier: 2.3x slower than the field

warm-clamp-accfit-dyn (275.91 us) is 2.3x the fastest (118.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (118.58 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64} vs {warm-clamp-head, warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-accfit-dyn} (105% apart)

The field splits into a fast tier {warm-clamp-acc64} and a slow tier {warm-clamp-head, warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-accfit-dyn} with a 105% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 118581.9 ns median
- 5 variants significantly slower than baseline
- Spread: 2.33x (fastest 118581.9 ns, slowest 275908.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112826587 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 118676ns | 118653ns | 118603ns | 118653ns | 118816ns | base |
| warm-clamp-accfit | 276388ns | 275674ns | 273128ns | 276090ns | 280542ns | +132.89% |
| warm-clamp-accfit-dyn | 280458ns | 275989ns | 273153ns | 276784ns | 298784ns | +136.32% |
| warm-clamp-head | 243478ns | 243428ns | 243346ns | 243432ns | 243744ns | +105.16% |
| warm-clamp-min-lanes | 250326ns | 250163ns | 250126ns | 250169ns | 250996ns | +110.93% |
| warm-clamp-minimum | 249939ns | 249875ns | 249847ns | 249879ns | 250214ns | +110.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 118599ns | 118529ns | 118728ns | base | 8.841 |
| warm-clamp-accfit | 276305ns | 273050ns | 280459ns | +132.98% | 3.795 |
| warm-clamp-accfit-dyn | 280373ns | 273067ns | 298695ns | +136.40% | 3.740 |
| warm-clamp-head | 243296ns | 243179ns | 243549ns | +105.14% | 4.310 |
| warm-clamp-min-lanes | 250232ns | 250042ns | 250895ns | +110.99% | 4.190 |
| warm-clamp-minimum | 249850ns | 249762ns | 250115ns | +110.67% | 4.197 |

## Performance model

- Peak throughput: **8.847 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.843 | 100.0% |
| warm-clamp-accfit | 3.805 | 43.0% |
| warm-clamp-accfit-dyn | 3.800 | 43.0% |
| warm-clamp-head | 4.311 | 48.7% |
| warm-clamp-min-lanes | 4.193 | 47.4% |
| warm-clamp-minimum | 4.198 | 47.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 118676ns | 118676ns | base |
| warm-clamp-accfit | 276388ns | 276388ns | +132.89% |
| warm-clamp-accfit-dyn | 280458ns | 280458ns | +136.32% |
| warm-clamp-head | 243478ns | 243478ns | +105.16% |
| warm-clamp-min-lanes | 250326ns | 250326ns | +110.93% |
| warm-clamp-minimum | 249939ns | 249939ns | +110.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 118582ns | base | --- | [118571, 118591] | --- | --- | --- | --- |
| warm-clamp-accfit | 275596ns | +156990.0ns (+132.4%) | [+156344, +158608]ns | [275047, 277225] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 275908ns | +157317.4ns (+132.7%) | [+156232, +159968]ns | [274883, 278514] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 243242ns | +124669.2ns (+105.1%) | [+124644, +124702]ns | [243221, 243284] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 250073ns | +131497.5ns (+110.9%) | [+131476, +131530]ns | [250061, 250081] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 249788ns | +131216.2ns (+110.7%) | [+131196, +131236]ns | [249782, 249797] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 118579ns | +136.3% | +138.1% | +105.1% | +110.9% | +110.6% |
| 1 | 2 | 118573ns | +131.6% | +133.2% | +105.2% | +110.9% | +110.7% |
| 1 | 3 | 118586ns | +133.1% | +131.0% | +105.1% | +110.9% | +110.7% |
| 1 | 4 | 118585ns | +130.8% | +131.9% | +105.1% | +110.9% | +110.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.070 | ok |
| warm-clamp-accfit | -0.298 | moderate- |
| warm-clamp-accfit-dyn | 0.109 | ok |
| warm-clamp-head | 0.085 | ok |
| warm-clamp-min-lanes | 0.298 | moderate+ |
| warm-clamp-minimum | -0.192 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/4, lost 4/4
- **warm-clamp-accfit-dyn**: won 0/4, lost 4/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 20.1ns | 118598.6ns | 0.0% |  |
| warm-clamp-accfit | 20.6ns | 276305.3ns | 0.0% |  |
| warm-clamp-accfit-dyn | 20.3ns | 280372.9ns | 0.0% |  |
| warm-clamp-head | 31.6ns | 243296.0ns | 0.0% |  |
| warm-clamp-min-lanes | 20.3ns | 250231.5ns | 0.0% |  |
| warm-clamp-minimum | 20.4ns | 249849.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 118528.7-118727.8 ns)
  118528.7 |######################
  118538.6 |#################
  118548.6 |###########
  118558.5 |#####
  118568.5 |######################
  118578.5 |##################################
  118588.4 |########################################
  118598.4 |#################
  118608.3 |#####
  118618.3 |#####
  118628.2 |#####
  118638.2 |#####
  118648.2 |
  118658.1 |#####
  118668.1 |
  118678.0 |
  118688.0 |#####
  118698.0 |
  118707.9 |
  118717.9 |
  (3 below, 1 above range)

warm-clamp-accfit (n=40, range 273050.1-280458.9 ns)
  273050.1 |
  273420.5 |########################
  273790.9 |########
  274161.4 |################################
  274531.8 |########
  274902.3 |########################
  275272.7 |########################################
  275643.1 |
  276013.6 |################
  276384.0 |
  276754.5 |################
  277124.9 |################################
  277495.3 |########
  277865.8 |################
  278236.2 |########
  278606.7 |
  278977.1 |########
  279347.5 |
  279718.0 |
  280088.4 |################
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 273066.9-298695.0 ns)
  273066.9 |###################################
  274348.3 |########################################
  275629.7 |#################
  276911.2 |#############
  278192.6 |########
  279474.0 |########
  280755.4 |#############
  282036.8 |#################
  283318.2 |
  284599.6 |########
  285881.0 |
  287162.4 |
  288443.8 |
  289725.2 |
  291006.6 |
  292288.0 |
  293569.4 |
  294850.8 |
  296132.2 |
  297413.6 |
  (2 below, 1 above range)

warm-clamp-head (n=40, range 243178.5-243549.5 ns)
  243178.5 |###########
  243197.1 |######################
  243215.6 |########################################
  243234.2 |######################
  243252.7 |#####
  243271.3 |######################
  243289.8 |############################
  243308.4 |###########
  243326.9 |###########
  243345.5 |
  243364.0 |#####
  243382.6 |
  243401.1 |
  243419.7 |
  243438.2 |
  243456.8 |
  243475.3 |
  243493.8 |
  243512.4 |
  243530.9 |
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 250042.0-250894.6 ns)
  250042.0 |########################################
  250084.6 |####
  250127.3 |#
  250169.9 |
  250212.5 |###
  250255.2 |
  250297.8 |
  250340.4 |#
  250383.1 |
  250425.7 |
  250468.3 |
  250511.0 |#
  250553.6 |
  250596.2 |
  250638.9 |
  250681.5 |
  250724.1 |
  250766.8 |
  250809.4 |
  250852.0 |#
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 249762.2-250115.3 ns)
  249762.2 |#########################
  249779.8 |########################################
  249797.5 |#################
  249815.1 |##
  249832.8 |
  249850.5 |#####
  249868.1 |##
  249885.8 |
  249903.4 |
  249921.1 |
  249938.7 |##
  249956.4 |##
  249974.1 |
  249991.7 |
  250009.4 |
  250027.0 |
  250044.7 |
  250062.4 |
  250080.0 |
  250097.7 |
  (3 below, 2 above range)

```
