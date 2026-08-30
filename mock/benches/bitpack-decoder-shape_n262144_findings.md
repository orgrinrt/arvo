# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (133.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-native at 29.21 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-native dominates: 50% faster than the next best (bitpack-plan-windowed)

bitpack-plan-native (29.21 us) leads bitpack-plan-windowed (43.78 us) by 50%, a clear separation rather than a photo finish. CV 11.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-plan-native beats baseline by 76% (significant)

bitpack-plan-native is -102.31 us (76%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.6x slower than the field

bitpack-plan-naive (133.84 us) is 4.6x the fastest (29.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-plan-native is fastest but the noisiest (CV 11.1%)

bitpack-plan-native wins on median (29.21 us) yet has the highest variance (CV 11.1%), while bitpack-plan-naive is the steadiest (CV 2.9%, 133.84 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-plan-native shows warm-up / thermal drift (autocorr +0.61)

bitpack-plan-native's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} vs {bitpack-plan-naive} (143% apart)

The field splits into a fast tier {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} and a slow tier {bitpack-plan-naive} with a 143% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-plan-native (29.21 us) to slowest bitpack-plan-naive (133.84 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-native** at 29212.1 ns median (-78.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.58x (fastest 29212.1 ns, slowest 133839.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 133651ns | 134138ns | 129457ns | 133324ns | 138825ns | base |
| bitpack-plan-native | 29715ns | 29350ns | 27283ns | 29180ns | 33753ns | -77.77% |
| bitpack-plan-simd | 56024ns | 55401ns | 54483ns | 55617ns | 58786ns | -58.08% |
| bitpack-plan-windowed | 43552ns | 43957ns | 41356ns | 43809ns | 44975ns | -67.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 133317ns | 129160ns | 138407ns | base | 1.966 |
| bitpack-plan-native | 29549ns | 27158ns | 33463ns | -77.84% | 8.871 |
| bitpack-plan-simd | 55824ns | 54311ns | 58580ns | -58.13% | 4.696 |
| bitpack-plan-windowed | 43388ns | 41205ns | 44798ns | -67.45% | 6.042 |

## Performance model

- Peak throughput: **9.652 Gops/s** (bitpack-plan-native; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.959 | 20.3% |
| bitpack-plan-native | 8.974 | 93.0% |
| bitpack-plan-simd | 4.750 | 49.2% |
| bitpack-plan-windowed | 5.988 | 62.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 133651ns | 133651ns | base |
| bitpack-plan-native | 29715ns | 29715ns | -77.77% |
| bitpack-plan-simd | 56024ns | 56024ns | -58.08% |
| bitpack-plan-windowed | 43552ns | 43552ns | -67.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 133839ns | base | --- | [131373, 134005] | --- | --- | --- | --- |
| bitpack-plan-native | 29212ns | -104520.7ns (-78.1%) | [-104875, -102145]ns | [28990, 29266] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 55191ns | -77946.9ns (-58.2%) | [-78824, -76142]ns | [55006, 55802] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 43779ns | -89909.6ns (-67.2%) | [-90381, -89366]ns | [43744, 43815] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|
| 1 | 135654ns | -78.4% | -59.5% | -67.7% |
| 2 | 151393ns | -80.9% | -63.8% | -71.1% |
| 3 | 135801ns | -80.2% | -56.4% | -67.8% |
| 4 | 135904ns | -79.6% | -58.4% | -67.8% |
| 5 | 133916ns | -78.2% | -54.9% | -67.3% |
| 6 | 128041ns | -76.9% | -54.3% | -65.8% |
| 7 | 133585ns | -78.1% | -57.1% | -67.2% |
| 8 | 135446ns | -74.9% | -57.8% | -67.7% |
| 9 | 139126ns | -69.1% | -57.6% | -66.5% |
| 10 | 134998ns | -68.6% | -58.9% | -66.5% |
| 11 | 134099ns | -78.9% | -59.1% | -68.6% |
| 12 | 134107ns | -78.2% | -59.1% | -69.0% |
| 13 | 134422ns | -78.3% | -58.0% | -67.4% |
| 14 | 134205ns | -78.1% | -59.1% | -67.2% |
| 15 | 127362ns | -76.4% | -58.4% | -65.6% |
| 16 | 132584ns | -78.9% | -58.8% | -66.9% |
| 17 | 133921ns | -79.9% | -58.7% | -66.8% |
| 18 | 133824ns | -79.8% | -59.0% | -67.2% |
| 19 | 133856ns | -79.6% | -58.3% | -67.2% |
| 20 | 133814ns | -78.3% | -59.0% | -67.2% |
| 21 | 128245ns | -76.7% | -58.4% | -65.9% |
| 22 | 131529ns | -77.8% | -59.0% | -65.8% |
| 23 | 130343ns | -77.5% | -57.1% | -65.8% |
| 24 | 130061ns | -77.5% | -57.5% | -66.3% |
| 25 | 129794ns | -77.3% | -57.2% | -66.3% |
| 26 | 134390ns | -78.1% | -59.1% | -66.8% |
| 27 | 133855ns | -78.0% | -58.9% | -67.3% |
| 28 | 134072ns | -78.2% | -57.7% | -68.4% |
| 29 | 131002ns | -77.6% | -57.8% | -68.6% |
| 30 | 131217ns | -77.7% | -58.0% | -66.5% |
| 31 | 138935ns | -79.1% | -59.3% | -70.9% |
| 32 | 129724ns | -79.2% | -57.0% | -68.8% |
| 33 | 132990ns | -79.3% | -58.6% | -68.2% |
| 34 | 130621ns | -77.6% | -57.9% | -67.6% |
| 35 | 133880ns | -79.2% | -58.9% | -69.0% |
| 36 | 130002ns | -79.3% | -57.7% | -66.5% |
| 37 | 131141ns | -78.2% | -58.0% | -69.2% |
| 38 | 130832ns | -77.7% | -53.8% | -67.0% |
| 39 | 133938ns | -78.2% | -58.8% | -67.3% |
| 40 | 130054ns | -77.5% | -56.8% | -66.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | 0.205 | moderate+ |
| bitpack-plan-native | 0.606 | HIGH+ (drift/warm-up) |
| bitpack-plan-simd | 0.302 | moderate+ |
| bitpack-plan-windowed | 0.407 | moderate+ |

**Consistency summary:**

- **bitpack-plan-native**: won 40/40, lost 0/40
- **bitpack-plan-simd**: won 40/40, lost 0/40
- **bitpack-plan-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 9.7ns | 133317.1ns | 0.0% |  |
| bitpack-plan-native | 3.9ns | 29549.1ns | 0.0% |  |
| bitpack-plan-simd | 6.4ns | 55824.1ns | 0.0% |  |
| bitpack-plan-windowed | 2.7ns | 43388.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 129160.3-138407.2 ns)
  129160.3 |
  129622.6 |################
  130085.0 |###
  130547.3 |##########
  131009.6 |######
  131472.0 |###
  131934.3 |
  132396.7 |###
  132859.0 |###
  133321.4 |###
  133783.7 |########################################
  134246.1 |######
  134708.4 |###
  135170.8 |###
  135633.1 |##########
  136095.4 |
  136557.8 |
  137020.1 |
  137482.5 |
  137944.8 |
  (3 below, 3 above range)

bitpack-plan-native (n=40, range 27158.4-33462.9 ns)
  27158.4 |##
  27473.7 |#####
  27788.9 |#####
  28104.1 |##
  28419.3 |##
  28734.5 |#######
  29049.8 |########################################
  29365.0 |##########
  29680.2 |##
  29995.4 |##
  30310.6 |
  30625.9 |
  30941.1 |
  31256.3 |
  31571.5 |
  31886.8 |
  32202.0 |
  32517.2 |
  32832.4 |
  33147.6 |
  (5 below, 3 above range)

bitpack-plan-simd (n=40, range 54311.3-58580.1 ns)
  54311.3 |
  54524.7 |#####
  54738.2 |########################################
  54951.6 |########################################
  55165.0 |###############
  55378.5 |##########
  55591.9 |#####
  55805.4 |##########
  56018.8 |#####
  56232.3 |#####
  56445.7 |##########
  56659.1 |#####
  56872.6 |
  57086.0 |##########
  57299.5 |
  57512.9 |
  57726.3 |
  57939.8 |
  58153.2 |
  58366.7 |#####
  (3 below, 4 above range)

bitpack-plan-windowed (n=40, range 41205.0-44797.8 ns)
  41205.0 |
  41384.6 |##
  41564.3 |##
  41743.9 |
  41923.6 |##
  42103.2 |##
  42282.8 |####
  42462.5 |
  42642.1 |
  42821.8 |
  43001.4 |##
  43181.0 |
  43360.7 |
  43540.3 |##
  43720.0 |########################################
  43899.6 |###########
  44079.2 |
  44258.9 |##
  44438.5 |####
  44618.2 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-plan-native**: autocorrelation=0.61 (measurement drift or warm-up artifact)
