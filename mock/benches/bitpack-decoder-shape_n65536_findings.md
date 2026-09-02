# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (33.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-native at 7.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-native dominates: 40% faster than the next best (bitpack-plan-windowed)

bitpack-plan-native (7.29 us) leads bitpack-plan-windowed (10.21 us) by 40%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-plan-native beats baseline by 77% (significant)

bitpack-plan-native is -25.66 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.6x slower than the field

bitpack-plan-naive (33.46 us) is 4.6x the fastest (7.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-plan-native shows warm-up / thermal drift (autocorr +0.56)

bitpack-plan-native's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} vs {bitpack-plan-naive} (144% apart)

The field splits into a fast tier {bitpack-plan-native, bitpack-plan-windowed, bitpack-plan-simd} and a slow tier {bitpack-plan-naive} with a 144% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-plan-native (7.29 us) to slowest bitpack-plan-naive (33.46 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-plan-naive is inconsistent: worst-20% is 1.6x its best-20%

bitpack-plan-naive's best 20% of batches run at 31.15 us but its worst 20% at 48.97 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-plan-native** at 7289.6 ns median (-78.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.59x (fastest 7289.6 ns, slowest 33460.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 35918ns | 33560ns | 31242ns | 33053ns | 49192ns | base |
| bitpack-plan-native | 7312ns | 7410ns | 6825ns | 7331ns | 7742ns | -79.64% |
| bitpack-plan-simd | 13901ns | 13793ns | 12787ns | 13772ns | 15402ns | -61.30% |
| bitpack-plan-windowed | 10612ns | 10298ns | 10159ns | 10538ns | 11288ns | -70.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 35784ns | 31148ns | 48969ns | base | 1.831 |
| bitpack-plan-native | 7180ns | 6717ns | 7555ns | -79.94% | 9.128 |
| bitpack-plan-simd | 13790ns | 12693ns | 15239ns | -61.46% | 4.752 |
| bitpack-plan-windowed | 10526ns | 10081ns | 11194ns | -70.58% | 6.226 |

## Performance model

- Peak throughput: **9.757 Gops/s** (bitpack-plan-native; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.959 | 20.1% |
| bitpack-plan-native | 8.990 | 92.1% |
| bitpack-plan-simd | 4.784 | 49.0% |
| bitpack-plan-windowed | 6.419 | 65.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 35918ns | 35918ns | base |
| bitpack-plan-native | 7312ns | 7312ns | -79.64% |
| bitpack-plan-simd | 13901ns | 13901ns | -61.30% |
| bitpack-plan-windowed | 10612ns | 10612ns | -70.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 33460ns | base | --- | [32219, 33487] | --- | --- | --- | --- |
| bitpack-plan-native | 7290ns | -26179.6ns (-78.2%) | [-26203, -25091]ns | [7288, 7291] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 13698ns | -19723.3ns (-58.9%) | [-19798, -18563]ns | [13695, 13747] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 10210ns | -22742.9ns (-68.0%) | [-23320, -21347]ns | [10104, 10940] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|
| 1 | 31225ns | -78.4% | -50.9% | -65.0% |
| 2 | 31728ns | -77.9% | -56.2% | -65.5% |
| 3 | 33671ns | -78.2% | -62.4% | -69.3% |
| 4 | 31492ns | -76.8% | -50.4% | -67.8% |
| 5 | 89311ns | -90.7% | -82.3% | -88.7% |
| 6 | 98566ns | -92.6% | -85.4% | -89.4% |
| 7 | 33441ns | -78.2% | -59.1% | -69.8% |
| 8 | 30883ns | -76.3% | -55.6% | -60.4% |
| 9 | 32344ns | -77.5% | -57.7% | -66.0% |
| 10 | 34405ns | -78.8% | -59.6% | -66.7% |
| 11 | 32392ns | -77.6% | -60.8% | -68.9% |
| 12 | 33488ns | -78.2% | -60.7% | -69.9% |
| 13 | 33483ns | -78.2% | -59.0% | -69.9% |
| 14 | 33512ns | -78.2% | -59.1% | -69.9% |
| 15 | 33335ns | -78.1% | -58.9% | -69.6% |
| 16 | 31258ns | -76.4% | -55.9% | -67.8% |
| 17 | 32071ns | -76.2% | -57.2% | -66.0% |
| 18 | 33479ns | -78.2% | -59.1% | -67.3% |
| 19 | 32095ns | -77.3% | -57.6% | -65.9% |
| 20 | 31933ns | -77.2% | -60.5% | -65.6% |
| 21 | 33295ns | -79.8% | -61.9% | -69.7% |
| 22 | 32369ns | -79.2% | -60.4% | -68.8% |
| 23 | 30852ns | -78.2% | -55.4% | -67.2% |
| 24 | 30902ns | -78.3% | -55.5% | -66.9% |
| 25 | 31430ns | -78.6% | -56.2% | -65.0% |
| 26 | 33490ns | -79.9% | -44.5% | -67.3% |
| 27 | 31655ns | -78.8% | -54.7% | -65.4% |
| 28 | 31144ns | -78.4% | -55.7% | -64.9% |
| 29 | 33480ns | -79.8% | -58.7% | -67.3% |
| 30 | 31901ns | -79.0% | -57.0% | -65.7% |
| 31 | 33903ns | -78.2% | -59.6% | -70.3% |
| 32 | 33520ns | -78.3% | -59.1% | -69.9% |
| 33 | 33486ns | -78.2% | -59.0% | -69.9% |
| 34 | 33498ns | -78.2% | -59.1% | -69.6% |
| 35 | 33480ns | -78.2% | -59.1% | -69.8% |
| 36 | 33489ns | -78.2% | -59.1% | -69.9% |
| 37 | 33518ns | -78.2% | -62.3% | -69.9% |
| 38 | 33480ns | -78.2% | -62.3% | -69.6% |
| 39 | 34037ns | -78.6% | -62.3% | -67.9% |
| 40 | 34335ns | -77.2% | -61.7% | -68.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | 0.462 | moderate+ |
| bitpack-plan-native | 0.557 | HIGH+ (drift/warm-up) |
| bitpack-plan-simd | 0.270 | moderate+ |
| bitpack-plan-windowed | 0.384 | moderate+ |

**Consistency summary:**

- **bitpack-plan-native**: won 40/40, lost 0/40
- **bitpack-plan-simd**: won 40/40, lost 0/40
- **bitpack-plan-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 4.8ns | 35784.4ns | 0.0% |  |
| bitpack-plan-native | 3.3ns | 7179.8ns | 0.0% |  |
| bitpack-plan-simd | 3.4ns | 13790.4ns | 0.0% |  |
| bitpack-plan-windowed | 2.5ns | 10526.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 31148.2-48968.6 ns)
  31148.2 |##################
  32039.3 |###########
  32930.3 |########################################
  33821.3 |#########
  34712.3 |
  35603.3 |
  36494.3 |
  37385.4 |
  38276.4 |
  39167.4 |
  40058.4 |
  40949.4 |
  41840.4 |
  42731.5 |
  43622.5 |
  44513.5 |
  45404.5 |
  46295.5 |
  47186.5 |
  48077.6 |
  (4 below, 2 above range)

bitpack-plan-native (n=40, range 6716.7-7554.7 ns)
   6716.7 |############
   6758.6 |##
   6800.5 |
   6842.4 |
   6884.3 |
   6926.2 |
   6968.1 |
   7010.0 |##
   7051.9 |
   7093.8 |
   7135.7 |
   7177.6 |
   7219.5 |
   7261.4 |########################################
   7303.3 |######
   7345.2 |##
   7387.1 |##
   7429.0 |
   7470.9 |
   7512.8 |
  (4 below, 3 above range)

bitpack-plan-simd (n=40, range 12693.3-15238.7 ns)
  12693.3 |######
  12820.5 |###
  12947.8 |
  13075.1 |######
  13202.4 |
  13329.6 |
  13456.9 |
  13584.2 |########################################
  13711.5 |#################################
  13838.7 |######
  13966.0 |
  14093.3 |
  14220.6 |###
  14347.8 |###
  14475.1 |
  14602.4 |
  14729.6 |
  14856.9 |
  14984.2 |
  15111.5 |
  (5 below, 4 above range)

bitpack-plan-windowed (n=40, range 10081.2-11194.0 ns)
  10081.2 |########################################
  10136.8 |#####
  10192.5 |##
  10248.1 |
  10303.7 |##
  10359.4 |
  10415.0 |
  10470.7 |##
  10526.3 |
  10581.9 |
  10637.6 |
  10693.2 |
  10748.8 |
  10804.5 |
  10860.1 |##
  10915.8 |############################
  10971.4 |###########
  11027.0 |
  11082.7 |
  11138.3 |
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-plan-naive**: CV=37.5% (high variance, measurements may be unstable)
- **bitpack-plan-native**: autocorrelation=0.56 (measurement drift or warm-up artifact)
