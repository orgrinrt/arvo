# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 29% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (10.83 us) leads bitpack-aligned-rand (13.98 us) by 29%, a clear separation rather than a photo finish. CV 12.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 22% (significant)

bitpack-native-rand is -3.11 us (22%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 2.2x slower than the field

bitpack-zeropad-rand (23.69 us) is 2.2x the fastest (10.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-aligned-rand shows warm-up / thermal drift (autocorr +0.81)

bitpack-aligned-rand's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-native-rand** at 10834.6 ns median (-22.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.19x (fastest 10834.6 ns, slowest 23690.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 14405ns | 14073ns | 13979ns | 14080ns | 15808ns | base |
| bitpack-native-rand | 11447ns | 10923ns | 10891ns | 10978ns | 13412ns | -20.53% |
| bitpack-zeropad-rand | 25778ns | 23790ns | 23301ns | 24024ns | 33515ns | +78.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 14305ns | 13890ns | 15681ns | base | 1.145 |
| bitpack-native-rand | 11346ns | 10806ns | 13269ns | -20.68% | 1.444 |
| bitpack-zeropad-rand | 25656ns | 23208ns | 33330ns | +79.35% | 0.639 |

## Performance model

- Peak throughput: **1.516 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 1.172 | 77.3% |
| bitpack-native-rand | 1.512 | 99.7% |
| bitpack-zeropad-rand | 0.692 | 45.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 14405ns | 14405ns | base |
| bitpack-native-rand | 11447ns | 11447ns | -20.53% |
| bitpack-zeropad-rand | 25778ns | 25778ns | +78.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 13980ns | base | --- | [13956, 13994] | --- | --- | --- | --- |
| bitpack-native-rand | 10835ns | -3136.6ns (-22.4%) | [-3168, -3053]ns | [10826, 10911] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 23690ns | +9566.7ns (+68.4%) | [+9442, +9755]ns | [23523, 24152] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|
| 1 | 15055ns | -27.2% | +60.9% |
| 2 | 16133ns | -26.0% | +46.0% |
| 3 | 17400ns | -7.4% | +41.1% |
| 4 | 17849ns | -39.3% | +46.6% |
| 5 | 15178ns | -16.7% | +58.7% |
| 6 | 15261ns | -11.9% | +58.6% |
| 7 | 14278ns | +24.4% | +63.7% |
| 8 | 13819ns | -20.4% | +169.4% |
| 9 | 13807ns | -18.6% | +68.1% |
| 10 | 13840ns | -18.7% | +69.9% |
| 11 | 13918ns | -20.7% | +82.5% |
| 12 | 13953ns | -16.5% | +67.8% |
| 13 | 13953ns | -22.5% | +67.4% |
| 14 | 13939ns | -22.6% | +70.5% |
| 15 | 14067ns | -19.3% | +325.0% |
| 16 | 13927ns | -21.6% | +73.5% |
| 17 | 13948ns | -21.8% | +109.7% |
| 18 | 13945ns | -21.9% | +120.8% |
| 19 | 13958ns | -21.9% | +92.9% |
| 20 | 13947ns | -21.8% | +84.4% |
| 21 | 13989ns | -22.6% | +68.0% |
| 22 | 14294ns | -23.6% | +65.9% |
| 23 | 13936ns | -22.3% | +68.9% |
| 24 | 13930ns | -22.3% | +68.5% |
| 25 | 14080ns | -23.1% | +66.2% |
| 26 | 14000ns | -22.7% | +67.2% |
| 27 | 13983ns | -22.7% | +67.7% |
| 28 | 13991ns | -22.8% | +68.0% |
| 29 | 13996ns | -22.6% | +68.8% |
| 30 | 13942ns | -22.3% | +68.9% |
| 31 | 14001ns | -22.8% | +72.5% |
| 32 | 13985ns | -22.7% | +81.8% |
| 33 | 13974ns | -22.8% | +70.5% |
| 34 | 14042ns | -22.9% | +67.1% |
| 35 | 13982ns | -22.5% | +120.2% |
| 36 | 13977ns | -22.7% | +67.8% |
| 37 | 13974ns | -22.6% | +57.7% |
| 38 | 13983ns | -22.7% | +69.2% |
| 39 | 13978ns | -22.4% | +71.3% |
| 40 | 13970ns | -22.5% | +69.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | 0.806 | HIGH+ (drift/warm-up) |
| bitpack-native-rand | 0.253 | moderate+ |
| bitpack-zeropad-rand | -0.051 | ok |

**Consistency summary:**

- **bitpack-native-rand**: won 39/40, lost 1/40
- **bitpack-zeropad-rand**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 3.0ns | 14304.6ns | 0.0% |  |
| bitpack-native-rand | 3.3ns | 11346.4ns | 0.0% |  |
| bitpack-zeropad-rand | 4.8ns | 25655.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 13889.5-15681.1 ns)
  13889.5 |########################################
  13979.1 |#########################
  14068.7 |##
  14158.3 |
  14247.8 |####
  14337.4 |
  14427.0 |
  14516.6 |
  14606.2 |
  14695.7 |
  14785.3 |
  14874.9 |
  14964.5 |
  15054.0 |##
  15143.6 |##
  15233.2 |##
  15322.8 |
  15412.4 |
  15501.9 |
  15591.5 |
  (3 below, 3 above range)

bitpack-native-rand (n=40, range 10805.6-13268.9 ns)
  10805.6 |########################################
  10928.8 |####
  11052.0 |
  11175.1 |###
  11298.3 |#
  11421.4 |
  11544.6 |#
  11667.8 |
  11790.9 |
  11914.1 |#
  12037.3 |
  12160.4 |
  12283.6 |
  12406.8 |
  12529.9 |#
  12653.1 |
  12776.2 |
  12899.4 |
  13022.6 |
  13145.7 |
  (3 below, 3 above range)

bitpack-zeropad-rand (n=40, range 23207.9-33330.4 ns)
  23207.9 |########################################
  23714.1 |################
  24220.2 |####
  24726.3 |
  25232.4 |######
  25738.6 |##
  26244.7 |
  26750.8 |##
  27256.9 |
  27763.0 |
  28269.2 |
  28775.3 |##
  29281.4 |
  29787.5 |
  30293.6 |####
  30799.8 |
  31305.9 |
  31812.0 |
  32318.1 |
  32824.2 |
  (1 below, 2 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **bitpack-zeropad-rand**: CV=23.8% (high variance, measurements may be unstable)
