# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (47.79 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-narrow at 31.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-windowed beats baseline by 32% (significant)

bitpack-mac-windowed is -15.53 us (32%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-native} vs {bitpack-mac-naive} (37% apart)

The field splits into a fast tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-narrow** at 31731.1 ns median (-33.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.51x (fastest 31731.1 ns, slowest 47791.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked decode against per-element consumer work |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112768841 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 47284ns | 47864ns | 46272ns | 47233ns | 48450ns | base |
| bitpack-mac-narrow | 31829ns | 31815ns | 31290ns | 31823ns | 32386ns | -32.69% |
| bitpack-mac-native | 35335ns | 34860ns | 34399ns | 34741ns | 38054ns | -25.27% |
| bitpack-mac-simd | 32346ns | 32354ns | 32296ns | 32348ns | 32388ns | -31.59% |
| bitpack-mac-windowed | 32347ns | 32356ns | 32299ns | 32351ns | 32386ns | -31.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 47212ns | 46199ns | 48372ns | base | 1.388 |
| bitpack-mac-narrow | 31754ns | 31216ns | 32306ns | -32.74% | 2.064 |
| bitpack-mac-native | 35264ns | 34332ns | 37973ns | -25.31% | 1.858 |
| bitpack-mac-simd | 32270ns | 32222ns | 32309ns | -31.65% | 2.031 |
| bitpack-mac-windowed | 32272ns | 32228ns | 32309ns | -31.65% | 2.031 |

## Performance model

- Peak throughput: **2.099 Gops/s** (bitpack-mac-narrow; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.371 | 65.3% |
| bitpack-mac-narrow | 2.065 | 98.4% |
| bitpack-mac-native | 1.884 | 89.7% |
| bitpack-mac-simd | 2.030 | 96.7% |
| bitpack-mac-windowed | 2.031 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 47284ns | 47284ns | base |
| bitpack-mac-narrow | 31829ns | 31829ns | -32.69% |
| bitpack-mac-native | 35335ns | 35335ns | -25.27% |
| bitpack-mac-simd | 32346ns | 32346ns | -31.59% |
| bitpack-mac-windowed | 32347ns | 32347ns | -31.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 47792ns | base | --- | [46255, 47806] | --- | --- | --- | --- |
| bitpack-mac-narrow | 31731ns | -15498.9ns (-32.4%) | [-15515, -15028]ns | [31237, 32266] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 34791ns | -12993.5ns (-27.2%) | [-13004, -11914]ns | [34463, 34797] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 32276ns | -15505.0ns (-32.4%) | [-15547, -14005]ns | [32257, 32296] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 32274ns | -15527.3ns (-32.5%) | [-15556, -13971]ns | [32259, 32295] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 46211ns | -32.4% | -25.6% | -30.1% | -30.1% |
| 1 | 2 | 47809ns | -32.5% | -27.2% | -32.4% | -32.5% |
| 1 | 3 | 47809ns | -32.5% | -27.2% | -32.6% | -32.5% |
| 1 | 4 | 46248ns | -32.5% | -25.8% | -30.2% | -30.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | -0.250 | moderate- |
| bitpack-mac-narrow | -0.250 | moderate- |
| bitpack-mac-native | -0.249 | moderate- |
| bitpack-mac-simd | -0.205 | moderate- |
| bitpack-mac-windowed | -0.196 | ok |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 19.8ns | 47212.5ns | 0.0% |  |
| bitpack-mac-narrow | 19.5ns | 31754.4ns | 0.1% |  |
| bitpack-mac-native | 19.2ns | 35263.6ns | 0.1% |  |
| bitpack-mac-simd | 19.3ns | 32270.2ns | 0.1% |  |
| bitpack-mac-windowed | 19.3ns | 32271.8ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 46198.9-48372.1 ns)
  46198.9 |################################
  46307.5 |
  46416.2 |##
  46524.9 |
  46633.5 |
  46742.2 |
  46850.9 |
  46959.5 |
  47068.2 |
  47176.8 |
  47285.5 |
  47394.2 |
  47502.8 |
  47611.5 |
  47720.2 |########################################
  47828.8 |#######
  47937.5 |
  48046.1 |
  48154.8 |##
  48263.5 |
  (3 below, 1 above range)

bitpack-mac-narrow (n=40, range 31215.8-32306.0 ns)
  31215.8 |########################################
  31270.3 |
  31324.8 |
  31379.3 |
  31433.8 |
  31488.4 |
  31542.9 |
  31597.4 |
  31651.9 |
  31706.4 |
  31760.9 |
  31815.4 |
  31869.9 |
  31924.4 |
  31978.9 |
  32033.4 |
  32087.9 |
  32142.5 |
  32197.0 |############
  32251.5 |###########################
  (4 below, 4 above range)

bitpack-mac-native (n=40, range 34332.3-37972.5 ns)
  34332.3 |##################
  34514.3 |###
  34696.3 |########################################
  34878.3 |
  35060.3 |
  35242.3 |
  35424.3 |
  35606.4 |
  35788.4 |
  35970.4 |
  36152.4 |
  36334.4 |
  36516.4 |
  36698.4 |
  36880.4 |
  37062.5 |
  37244.5 |
  37426.5 |
  37608.5 |
  37790.5 |
  (4 below, 2 above range)

bitpack-mac-simd (n=40, range 32222.3-32308.5 ns)
  32222.3 |######################
  32226.6 |####
  32230.9 |####
  32235.3 |########
  32239.6 |
  32243.9 |
  32248.2 |####
  32252.5 |####
  32256.8 |####
  32261.1 |########
  32265.4 |########
  32269.7 |
  32274.1 |########
  32278.4 |
  32282.7 |####
  32287.0 |########
  32291.3 |########
  32295.6 |########
  32299.9 |########################################
  32304.2 |####
  (3 below, 2 above range)

bitpack-mac-windowed (n=40, range 32228.4-32308.5 ns)
  32228.4 |##########
  32232.4 |########################################
  32236.4 |##########
  32240.5 |####################
  32244.5 |##########
  32248.5 |
  32252.5 |
  32256.5 |##############################
  32260.5 |##############################
  32264.5 |##########
  32268.5 |
  32272.5 |####################
  32276.5 |
  32280.5 |####################
  32284.5 |
  32288.5 |####################
  32292.5 |########################################
  32296.5 |####################
  32300.5 |########################################
  32304.5 |##########
  (3 below, 4 above range)

```
