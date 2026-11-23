# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (191.36 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-narrow at 125.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-windowed beats baseline by 33% (significant)

bitpack-mac-windowed is -63.53 us (33%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-mac-native shows alternating (throttle bounce) (autocorr -0.75)

bitpack-mac-native's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-native} vs {bitpack-mac-naive} (37% apart)

The field splits into a fast tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-narrow** at 125482.6 ns median (-34.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.53x (fastest 125482.6 ns, slowest 191361.7 ns)

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
| Counter frequency | 2112613062 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 190591ns | 191486ns | 187611ns | 190624ns | 193471ns | base |
| bitpack-mac-narrow | 126882ns | 125564ns | 125530ns | 126624ns | 129009ns | -33.43% |
| bitpack-mac-native | 138651ns | 139376ns | 137396ns | 138746ns | 139623ns | -27.25% |
| bitpack-mac-simd | 128266ns | 128768ns | 126147ns | 128764ns | 128893ns | -32.70% |
| bitpack-mac-windowed | 127249ns | 127400ns | 125544ns | 127219ns | 129044ns | -33.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 190505ns | 187529ns | 193386ns | base | 1.376 |
| bitpack-mac-narrow | 126799ns | 125451ns | 128925ns | -33.44% | 2.067 |
| bitpack-mac-native | 138568ns | 137317ns | 139540ns | -27.26% | 1.892 |
| bitpack-mac-simd | 128183ns | 126068ns | 128807ns | -32.71% | 2.045 |
| bitpack-mac-windowed | 127166ns | 125460ns | 128961ns | -33.25% | 2.061 |

## Performance model

- Peak throughput: **2.090 Gops/s** (bitpack-mac-narrow; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.370 | 65.6% |
| bitpack-mac-narrow | 2.089 | 100.0% |
| bitpack-mac-native | 1.882 | 90.1% |
| bitpack-mac-simd | 2.037 | 97.5% |
| bitpack-mac-windowed | 2.059 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 190591ns | 190591ns | base |
| bitpack-mac-narrow | 126882ns | 126882ns | -33.43% |
| bitpack-mac-native | 138651ns | 138651ns | -27.25% |
| bitpack-mac-simd | 128266ns | 128266ns | -32.70% |
| bitpack-mac-windowed | 127249ns | 127249ns | -33.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 191362ns | base | --- | [188082, 192905] | --- | --- | --- | --- |
| bitpack-mac-narrow | 125483ns | -63469.2ns (-33.2%) | [-66316, -62064]ns | [125471, 128630] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 139293ns | -52902.6ns (-27.6%) | [-53680, -50109]ns | [137538, 139310] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 128683ns | -62669.7ns (-32.7%) | [-64302, -59325]ns | [128659, 128697] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 127319ns | -63525.1ns (-33.2%) | [-64429, -61850]ns | [125504, 128668] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 192981ns | -35.0% | -28.8% | -33.3% | -33.3% |
| 1 | 2 | 187871ns | -31.5% | -25.8% | -31.5% | -31.5% |
| 1 | 3 | 187683ns | -33.2% | -26.8% | -31.4% | -33.1% |
| 1 | 4 | 192963ns | -33.3% | -27.8% | -34.9% | -35.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | -0.250 | moderate- |
| bitpack-mac-narrow | -0.752 | HIGH- (thermal bounce) |
| bitpack-mac-native | -0.754 | HIGH- (thermal bounce) |
| bitpack-mac-simd | -0.092 | ok |
| bitpack-mac-windowed | 0.254 | moderate+ |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 22.8ns | 190505.1ns | 0.0% |  |
| bitpack-mac-narrow | 22.3ns | 126798.7ns | 0.0% |  |
| bitpack-mac-native | 21.7ns | 138568.3ns | 0.0% |  |
| bitpack-mac-simd | 22.1ns | 128182.8ns | 0.0% |  |
| bitpack-mac-windowed | 22.0ns | 127165.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 187529.5-193385.8 ns)
  187529.5 |##################
  187822.3 |##################
  188115.1 |##############
  188407.9 |
  188700.8 |
  188993.6 |
  189286.4 |
  189579.2 |
  189872.0 |###
  190164.8 |
  190457.7 |
  190750.5 |
  191043.3 |
  191336.1 |
  191628.9 |
  191921.8 |
  192214.6 |
  192507.4 |##########
  192800.2 |########################################
  193093.0 |##############
  (5 below, 2 above range)

bitpack-mac-narrow (n=40, range 125450.6-128924.9 ns)
  125450.6 |########################################
  125624.3 |
  125798.0 |
  125971.7 |
  126145.4 |
  126319.2 |
  126492.9 |
  126666.6 |
  126840.3 |
  127014.0 |
  127187.7 |
  127361.5 |
  127535.2 |
  127708.9 |
  127882.6 |
  128056.3 |
  128230.0 |
  128403.7 |
  128577.5 |############################
  128751.2 |##
  (4 below, 1 above range)

bitpack-mac-native (n=40, range 137316.7-139539.7 ns)
  137316.7 |####################################
  137427.8 |####
  137539.0 |####
  137650.1 |####
  137761.3 |
  137872.4 |
  137983.6 |
  138094.8 |
  138205.9 |
  138317.1 |
  138428.2 |####
  138539.4 |
  138650.5 |
  138761.7 |
  138872.8 |
  138984.0 |
  139095.1 |
  139206.3 |########################################
  139317.4 |################################
  139428.6 |############
  (4 below, 2 above range)

bitpack-mac-simd (n=40, range 126067.7-128806.5 ns)
  126067.7 |
  126204.7 |
  126341.6 |
  126478.6 |
  126615.5 |#
  126752.4 |
  126889.4 |
  127026.3 |
  127163.2 |
  127300.2 |
  127437.1 |
  127574.1 |
  127711.0 |
  127847.9 |
  127984.9 |
  128121.8 |
  128258.8 |
  128395.7 |
  128532.6 |#############
  128669.6 |########################################
  (6 below, 2 above range)

bitpack-mac-windowed (n=40, range 125460.0-128961.2 ns)
  125460.0 |########################################
  125635.0 |######
  125810.1 |###
  125985.1 |###
  126160.2 |
  126335.3 |
  126510.3 |
  126685.4 |
  126860.4 |
  127035.5 |
  127210.6 |
  127385.6 |
  127560.7 |
  127735.7 |
  127910.8 |
  128085.9 |
  128260.9 |
  128436.0 |##########
  128611.0 |########################################
  128786.1 |##########
  (4 below, 2 above range)

```
