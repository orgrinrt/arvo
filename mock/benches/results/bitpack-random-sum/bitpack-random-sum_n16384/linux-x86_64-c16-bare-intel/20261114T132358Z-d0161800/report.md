# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 82% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (4.11 us) leads bitpack-aligned-rand (7.49 us) by 82%, a clear separation rather than a photo finish. CV 62.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 45% (significant)

bitpack-native-rand is -3.38 us (45%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 4.5x slower than the field

bitpack-zeropad-rand (18.34 us) is 4.5x the fastest (4.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-rand is fastest but the noisiest (CV 62.5%)

bitpack-native-rand wins on median (4.11 us) yet has the highest variance (CV 62.5%), while bitpack-zeropad-rand is the steadiest (CV 0.2%, 18.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.5x the fastest

Fastest bitpack-native-rand (4.11 us) to slowest bitpack-zeropad-rand (18.34 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-rand is inconsistent: worst-20% is 2.4x its best-20%

bitpack-native-rand's best 20% of batches run at 4.06 us but its worst 20% at 9.54 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 4107.4 ns median (-45.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.47x (fastest 4107.4 ns, slowest 18340.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 7612ns | 7560ns | 7551ns | 7560ns | 7828ns | base |
| bitpack-native-rand | 5286ns | 4186ns | 4137ns | 4182ns | 9744ns | -30.56% |
| bitpack-zeropad-rand | 18410ns | 18408ns | 18366ns | 18411ns | 18450ns | +141.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 7536ns | 7479ns | 7733ns | base | 2.174 |
| bitpack-native-rand | 5183ns | 4058ns | 9544ns | -31.22% | 3.161 |
| bitpack-zeropad-rand | 18339ns | 18299ns | 18369ns | +143.36% | 0.893 |

## Performance model

- Peak throughput: **4.038 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 2.188 | 54.2% |
| bitpack-native-rand | 3.989 | 98.8% |
| bitpack-zeropad-rand | 0.893 | 22.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 7612ns | 7612ns | base |
| bitpack-native-rand | 5286ns | 5286ns | -30.56% |
| bitpack-zeropad-rand | 18410ns | 18410ns | +141.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 7489ns | base | --- | [7487, 7490] | --- | --- | --- | --- |
| bitpack-native-rand | 4107ns | -3381.6ns (-45.2%) | [-3397, -3374]ns | [4092, 4116] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 18340ns | +10850.2ns (+144.9%) | [+10847, +10857]ns | [18337, 18344] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 7489ns | -45.1% | +145.1% |
| 1 | 2 | 7486ns | -45.0% | +145.0% |
| 1 | 3 | 7490ns | -45.5% | +144.8% |
| 1 | 4 | 7490ns | -45.7% | +144.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.132 | ok |
| bitpack-native-rand | 0.256 | moderate+ |
| bitpack-zeropad-rand | 0.143 | ok |

**Consistency summary:**

- **bitpack-native-rand**: won 4/4, lost 0/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 19.9ns | 7535.6ns | 0.3% |  |
| bitpack-native-rand | 24.9ns | 5183.3ns | 0.5% |  |
| bitpack-zeropad-rand | 19.6ns | 18338.7ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 7478.7-7732.6 ns)
   7478.7 |########################################
   7491.4 |##########
   7504.1 |
   7516.8 |
   7529.5 |
   7542.2 |
   7554.9 |
   7567.6 |
   7580.3 |
   7593.0 |
   7605.7 |
   7618.4 |
   7631.1 |
   7643.8 |
   7656.5 |#
   7669.2 |
   7681.8 |
   7694.5 |
   7707.2 |
   7719.9 |
  (3 below, 1 above range)

bitpack-native-rand (n=40, range 4057.8-9543.9 ns)
   4057.8 |########################################
   4332.1 |
   4606.4 |
   4880.7 |
   5155.0 |
   5429.3 |
   5703.6 |
   5977.9 |
   6252.2 |
   6526.5 |
   6800.8 |#
   7075.1 |
   7349.4 |#
   7623.7 |
   7898.1 |
   8172.4 |
   8446.7 |#
   8721.0 |
   8995.3 |
   9269.6 |
  (5 below, 4 above range)

bitpack-zeropad-rand (n=40, range 18299.3-18369.0 ns)
  18299.3 |
  18302.8 |
  18306.3 |
  18309.8 |
  18313.3 |
  18316.7 |
  18320.2 |
  18323.7 |#################
  18327.2 |#####
  18330.7 |###########
  18334.1 |########################################
  18337.6 |##################################
  18341.1 |##################################
  18344.6 |
  18348.1 |#####
  18351.5 |
  18355.0 |############################
  18358.5 |#################
  18362.0 |
  18365.5 |#####
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-native-rand**: CV=49.5% (high variance, measurements may be unstable)
