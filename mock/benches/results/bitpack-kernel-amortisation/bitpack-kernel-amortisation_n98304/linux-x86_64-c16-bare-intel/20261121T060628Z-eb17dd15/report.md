# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (70.45 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-windowed at 48.20 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-simd beats baseline by 33% (significant)

bitpack-mac-simd is -23.23 us (33%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-mac-windowed, bitpack-mac-simd) are a dead heat (<1%)

bitpack-mac-windowed (48.20 us) and bitpack-mac-simd (48.22 us) differ by 0.03%, inside the noise, even though the wider field spreads 46.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-narrow, bitpack-mac-native} vs {bitpack-mac-naive} (35% apart)

The field splits into a fast tier {bitpack-mac-windowed, bitpack-mac-simd, bitpack-mac-narrow, bitpack-mac-native} and a slow tier {bitpack-mac-naive} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-windowed** at 48202.2 ns median (-31.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.46x (fastest 48202.2 ns, slowest 70454.9 ns)

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
| bitpack-mac-naive | 71515ns | 70526ns | 70443ns | 70765ns | 74838ns | base |
| bitpack-mac-narrow | 48307ns | 48305ns | 48255ns | 48300ns | 48381ns | -32.45% |
| bitpack-mac-native | 52300ns | 52293ns | 52282ns | 52294ns | 52335ns | -26.87% |
| bitpack-mac-simd | 48032ns | 48290ns | 47144ns | 48202ns | 48409ns | -32.84% |
| bitpack-mac-windowed | 48156ns | 48288ns | 47533ns | 48280ns | 48407ns | -32.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 71439ns | 70372ns | 74758ns | base | 1.376 |
| bitpack-mac-narrow | 48235ns | 48182ns | 48303ns | -32.48% | 2.038 |
| bitpack-mac-native | 52227ns | 52214ns | 52253ns | -26.89% | 1.882 |
| bitpack-mac-simd | 47958ns | 47071ns | 48333ns | -32.87% | 2.050 |
| bitpack-mac-windowed | 48083ns | 47463ns | 48331ns | -32.69% | 2.044 |

## Performance model

- Peak throughput: **2.088 Gops/s** (bitpack-mac-simd; best 20% batches)
- Ops per call: 98304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 1.395 | 66.8% |
| bitpack-mac-narrow | 2.038 | 97.6% |
| bitpack-mac-native | 1.882 | 90.1% |
| bitpack-mac-simd | 2.039 | 97.6% |
| bitpack-mac-windowed | 2.039 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 71515ns | 71515ns | base |
| bitpack-mac-narrow | 48307ns | 48307ns | -32.45% |
| bitpack-mac-native | 52300ns | 52300ns | -26.87% |
| bitpack-mac-simd | 48032ns | 48032ns | -32.84% |
| bitpack-mac-windowed | 48156ns | 48156ns | -32.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 70455ns | base | --- | [70422, 70495] | --- | --- | --- | --- |
| bitpack-mac-narrow | 48233ns | -22219.2ns (-31.5%) | [-22291, -22199]ns | [48216, 48243] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 52222ns | -18233.4ns (-25.9%) | [-18270, -18196]ns | [52218, 52227] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 48218ns | -23227.3ns (-33.0%) | [-23383, -22239]ns | [48195, 48232] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 48202ns | -22330.7ns (-31.7%) | [-23333, -22220]ns | [48185, 48231] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|---|
| 1 | 1 | 70401ns | -31.5% | -25.8% | -31.5% | -32.3% |
| 1 | 2 | 72367ns | -33.4% | -27.8% | -33.3% | -33.4% |
| 1 | 3 | 70434ns | -31.5% | -25.9% | -33.2% | -31.6% |
| 1 | 4 | 70447ns | -31.5% | -25.9% | -31.5% | -31.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | -0.431 | moderate- |
| bitpack-mac-narrow | 0.101 | ok |
| bitpack-mac-native | -0.445 | moderate- |
| bitpack-mac-simd | -0.408 | moderate- |
| bitpack-mac-windowed | -0.105 | ok |

**Consistency summary:**

- **bitpack-mac-narrow**: won 4/4, lost 0/4
- **bitpack-mac-native**: won 4/4, lost 0/4
- **bitpack-mac-simd**: won 4/4, lost 0/4
- **bitpack-mac-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 20.9ns | 71438.8ns | 0.0% |  |
| bitpack-mac-narrow | 20.0ns | 48234.5ns | 0.0% |  |
| bitpack-mac-native | 20.1ns | 52227.1ns | 0.0% |  |
| bitpack-mac-simd | 20.4ns | 47958.1ns | 0.0% |  |
| bitpack-mac-windowed | 20.0ns | 48082.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 70371.9-74758.1 ns)
  70371.9 |########################################
  70591.2 |
  70810.5 |
  71029.8 |
  71249.1 |
  71468.4 |
  71687.8 |
  71907.1 |
  72126.4 |###
  72345.7 |############
  72565.0 |
  72784.3 |
  73003.6 |
  73222.9 |
  73442.3 |
  73661.6 |
  73880.9 |
  74100.2 |
  74319.5 |
  74538.8 |
  (3 below, 1 above range)

bitpack-mac-narrow (n=40, range 48182.5-48302.6 ns)
  48182.5 |
  48188.5 |################
  48194.5 |########################
  48200.5 |########
  48206.5 |################
  48212.5 |################
  48218.5 |
  48224.5 |########################################
  48230.5 |########
  48236.5 |########################################
  48242.5 |########################################
  48248.5 |################################
  48254.6 |
  48260.6 |########
  48266.6 |########
  48272.6 |
  48278.6 |
  48284.6 |
  48290.6 |
  48296.6 |
  (5 below, 3 above range)

bitpack-mac-native (n=40, range 52213.8-52253.2 ns)
  52213.8 |########################################
  52215.8 |################################
  52217.8 |################
  52219.7 |################
  52221.7 |################################
  52223.7 |################
  52225.6 |########################
  52227.6 |########
  52229.6 |########################################
  52231.5 |########
  52233.5 |
  52235.5 |
  52237.4 |
  52239.4 |
  52241.4 |########
  52243.3 |########
  52245.3 |########
  52247.3 |
  52249.3 |########
  52251.2 |
  (5 below, 2 above range)

bitpack-mac-simd (n=40, range 47071.1-48333.3 ns)
  47071.1 |#######
  47134.2 |#
  47197.3 |#
  47260.4 |
  47323.5 |
  47386.6 |
  47449.7 |
  47512.9 |
  47576.0 |
  47639.1 |
  47702.2 |
  47765.3 |
  47828.4 |
  47891.5 |
  47954.6 |
  48017.8 |
  48080.9 |#
  48144.0 |#######
  48207.1 |########################################
  48270.2 |###
  (4 below, 2 above range)

bitpack-mac-windowed (n=40, range 47463.1-48330.9 ns)
  47463.1 |
  47506.5 |
  47549.9 |
  47593.3 |
  47636.7 |
  47680.1 |
  47723.5 |
  47766.8 |
  47810.2 |
  47853.6 |
  47897.0 |
  47940.4 |
  47983.8 |
  48027.2 |
  48070.6 |
  48113.9 |#############
  48157.3 |####################################
  48200.7 |########################################
  48244.1 |####################
  48287.5 |
  (5 below, 2 above range)

```
