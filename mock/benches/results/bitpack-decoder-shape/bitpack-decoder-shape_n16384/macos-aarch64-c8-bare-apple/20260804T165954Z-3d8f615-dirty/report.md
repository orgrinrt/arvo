# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (7.99 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-native at 1.81 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-native dominates: 41% faster than the next best (bitpack-plan-windowed)

bitpack-plan-native (1.81 us) leads bitpack-plan-windowed (2.55 us) by 41%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-plan-native beats baseline by 77% (significant)

bitpack-plan-native is -6.16 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.4x slower than the field

bitpack-plan-naive (7.99 us) is 4.4x the fastest (1.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-plan-windowed shows warm-up / thermal drift (autocorr +0.79)

bitpack-plan-windowed's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} vs {bitpack-plan-naive} (135% apart)

The field splits into a fast tier {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} and a slow tier {bitpack-plan-naive} with a 135% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-plan-native (1.81 us) to slowest bitpack-plan-naive (7.99 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-native** at 1808.1 ns median (-77.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.42x (fastest 1808.1 ns, slowest 7993.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 8426ns | 8059ns | 8017ns | 8250ns | 9361ns | base |
| bitpack-plan-native | 1873ns | 1887ns | 1745ns | 1887ns | 1960ns | -77.77% |
| bitpack-plan-simd | 3604ns | 3493ns | 3486ns | 3498ns | 4042ns | -57.22% |
| bitpack-plan-windowed | 2699ns | 2620ns | 2594ns | 2690ns | 2831ns | -67.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 8346ns | 7948ns | 9253ns | base | 1.963 |
| bitpack-plan-native | 1795ns | 1671ns | 1879ns | -78.49% | 9.128 |
| bitpack-plan-simd | 3513ns | 3400ns | 3931ns | -57.91% | 4.664 |
| bitpack-plan-windowed | 2622ns | 2521ns | 2749ns | -68.59% | 6.250 |

## Performance model

- Peak throughput: **9.806 Gops/s** (bitpack-plan-native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 2.050 | 20.9% |
| bitpack-plan-native | 9.061 | 92.4% |
| bitpack-plan-simd | 4.811 | 49.1% |
| bitpack-plan-windowed | 6.429 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 8426ns | 8426ns | base |
| bitpack-plan-native | 1873ns | 1873ns | -77.77% |
| bitpack-plan-simd | 3604ns | 3604ns | -57.22% |
| bitpack-plan-windowed | 2699ns | 2699ns | -67.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 7993ns | base | --- | [7951, 8569] | --- | --- | --- | --- |
| bitpack-plan-native | 1808ns | -6282.1ns (-78.6%) | [-6663, -6189]ns | [1806, 1811] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 3405ns | -4563.8ns (-57.1%) | [-5088, -4546]ns | [3403, 3423] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 2548ns | -5428.8ns (-67.9%) | [-5944, -5258]ns | [2526, 2737] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|
| 1 | 7950ns | -77.3% | -57.2% | -68.2% |
| 2 | 7951ns | -77.2% | -57.2% | -68.3% |
| 3 | 7949ns | -77.2% | -57.2% | -68.3% |
| 4 | 8342ns | -78.4% | -59.2% | -69.8% |
| 5 | 7990ns | -77.4% | -57.4% | -66.4% |
| 6 | 7953ns | -77.3% | -57.0% | -65.6% |
| 7 | 8000ns | -77.3% | -57.0% | -65.8% |
| 8 | 7949ns | -76.0% | -57.1% | -65.6% |
| 9 | 7948ns | -78.9% | -57.1% | -65.5% |
| 10 | 7950ns | -78.8% | -57.2% | -65.6% |
| 11 | 8624ns | -79.0% | -58.7% | -68.4% |
| 12 | 8632ns | -79.0% | -60.6% | -70.8% |
| 13 | 8627ns | -79.1% | -60.6% | -70.7% |
| 14 | 8622ns | -79.1% | -60.6% | -70.7% |
| 15 | 8632ns | -77.2% | -60.6% | -70.7% |
| 16 | 8105ns | -77.8% | -57.2% | -68.9% |
| 17 | 8003ns | -77.4% | -57.5% | -68.5% |
| 18 | 7946ns | -77.2% | -57.2% | -68.3% |
| 19 | 7950ns | -77.3% | -57.2% | -68.2% |
| 20 | 7968ns | -77.3% | -57.3% | -68.4% |
| 21 | 7948ns | -79.0% | -21.6% | -65.6% |
| 22 | 7947ns | -79.1% | -51.2% | -65.5% |
| 23 | 7950ns | -79.0% | -56.8% | -65.6% |
| 24 | 7978ns | -79.1% | -57.0% | -64.8% |
| 25 | 7952ns | -78.9% | -56.9% | -65.6% |
| 26 | 7948ns | -77.2% | -56.9% | -65.5% |
| 27 | 7996ns | -77.4% | -56.9% | -65.7% |
| 28 | 7951ns | -77.3% | -56.9% | -65.5% |
| 29 | 7952ns | -77.2% | -56.9% | -65.5% |
| 30 | 7950ns | -77.2% | -51.5% | -65.5% |
| 31 | 9444ns | -82.4% | -63.9% | -72.3% |
| 32 | 9453ns | -80.6% | -62.3% | -73.3% |
| 33 | 10440ns | -81.9% | -67.5% | -75.8% |
| 34 | 9447ns | -80.8% | -63.9% | -72.8% |
| 35 | 9063ns | -80.0% | -62.5% | -71.0% |
| 36 | 8653ns | -79.1% | -60.7% | -70.8% |
| 37 | 8632ns | -78.5% | -60.6% | -70.8% |
| 38 | 8632ns | -78.0% | -60.6% | -70.7% |
| 39 | 8890ns | -79.6% | -61.7% | -71.6% |
| 40 | 8515ns | -78.2% | -60.1% | -70.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | 0.772 | HIGH+ (drift/warm-up) |
| bitpack-plan-native | 0.370 | moderate+ |
| bitpack-plan-simd | 0.109 | ok |
| bitpack-plan-windowed | 0.787 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-plan-native**: won 40/40, lost 0/40
- **bitpack-plan-simd**: won 40/40, lost 0/40
- **bitpack-plan-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 2.1ns | 8345.8ns | 0.0% |  |
| bitpack-plan-native | 2.0ns | 1794.8ns | 0.1% |  |
| bitpack-plan-simd | 2.6ns | 3512.7ns | 0.1% |  |
| bitpack-plan-windowed | 2.0ns | 2621.5ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 7948.1-9252.8 ns)
   7948.1 |########################################
   8013.3 |
   8078.5 |##
   8143.8 |
   8209.0 |
   8274.3 |
   8339.5 |##
   8404.7 |
   8470.0 |##
   8535.2 |
   8600.4 |################
   8665.7 |
   8730.9 |
   8796.2 |
   8861.4 |##
   8926.6 |
   8991.9 |
   9057.1 |##
   9122.3 |
   9187.6 |
  (3 below, 4 above range)

bitpack-plan-native (n=40, range 1670.8-1878.7 ns)
   1670.8 |####
   1681.2 |##
   1691.6 |
   1702.0 |
   1712.4 |
   1722.8 |
   1733.2 |
   1743.6 |
   1754.0 |
   1764.3 |
   1774.7 |
   1785.1 |
   1795.5 |############
   1805.9 |########################################
   1816.3 |
   1826.7 |
   1837.1 |##
   1847.5 |####
   1857.9 |
   1868.3 |
  (5 below, 4 above range)

bitpack-plan-simd (n=40, range 3399.7-3930.9 ns)
   3399.7 |########################################
   3426.3 |########
   3452.9 |#
   3479.4 |
   3506.0 |
   3532.5 |
   3559.1 |###
   3585.7 |
   3612.2 |
   3638.8 |
   3665.3 |
   3691.9 |
   3718.4 |
   3745.0 |
   3771.6 |
   3798.1 |
   3824.7 |
   3851.2 |#
   3877.8 |#
   3904.4 |
  (4 below, 1 above range)

bitpack-plan-windowed (n=40, range 2521.4-2749.0 ns)
   2521.4 |########################################
   2532.8 |
   2544.2 |
   2555.5 |##
   2566.9 |
   2578.3 |
   2589.7 |
   2601.1 |
   2612.4 |##
   2623.8 |##
   2635.2 |
   2646.6 |
   2658.0 |
   2669.3 |
   2680.7 |##
   2692.1 |
   2703.5 |
   2714.9 |
   2726.3 |############
   2737.6 |#########################
  (4 below, 1 above range)

```

## Diagnostics

- **bitpack-plan-naive**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **bitpack-plan-windowed**: autocorrelation=0.79 (measurement drift or warm-up artifact)
