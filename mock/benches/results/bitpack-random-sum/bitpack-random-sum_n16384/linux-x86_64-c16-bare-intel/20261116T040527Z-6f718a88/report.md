# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 81% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (4.13 us) leads bitpack-aligned-rand (7.49 us) by 81%, a clear separation rather than a photo finish. CV 60.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 45% (significant)

bitpack-native-rand is -3.35 us (45%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 4.4x slower than the field

bitpack-zeropad-rand (18.34 us) is 4.4x the fastest (4.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-rand is fastest but the noisiest (CV 60.8%)

bitpack-native-rand wins on median (4.13 us) yet has the highest variance (CV 60.8%), while bitpack-zeropad-rand is the steadiest (CV 0.1%, 18.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-native-rand (4.13 us) to slowest bitpack-zeropad-rand (18.34 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-rand is inconsistent: worst-20% is 2.1x its best-20%

bitpack-native-rand's best 20% of batches run at 4.10 us but its worst 20% at 8.43 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 4132.9 ns median (-44.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.44x (fastest 4132.9 ns, slowest 18335.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112826911 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 7605ns | 7558ns | 7548ns | 7558ns | 7803ns | base |
| bitpack-native-rand | 5088ns | 4208ns | 4179ns | 4216ns | 8614ns | -33.09% |
| bitpack-zeropad-rand | 18401ns | 18402ns | 18367ns | 18402ns | 18428ns | +141.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 7531ns | 7478ns | 7713ns | base | 2.175 |
| bitpack-native-rand | 4992ns | 4102ns | 8434ns | -33.72% | 3.282 |
| bitpack-zeropad-rand | 18331ns | 18298ns | 18353ns | +143.41% | 0.894 |

## Performance model

- Peak throughput: **3.994 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 2.188 | 54.8% |
| bitpack-native-rand | 3.964 | 99.3% |
| bitpack-zeropad-rand | 0.894 | 22.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 7605ns | 7605ns | base |
| bitpack-native-rand | 5088ns | 5088ns | -33.09% |
| bitpack-zeropad-rand | 18401ns | 18401ns | +141.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 7490ns | base | --- | [7485, 7491] | --- | --- | --- | --- |
| bitpack-native-rand | 4133ns | -3354.2ns (-44.8%) | [-3367, -3340]ns | [4119, 4146] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 18336ns | +10847.5ns (+144.8%) | [+10845, +10850]ns | [18333, 18338] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 7484ns | -44.9% | +145.0% |
| 1 | 2 | 7491ns | -44.7% | +144.9% |
| 1 | 3 | 7491ns | -44.7% | +144.8% |
| 1 | 4 | 7490ns | -44.8% | +144.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.122 | ok |
| bitpack-native-rand | -0.128 | ok |
| bitpack-zeropad-rand | -0.216 | moderate- |

**Consistency summary:**

- **bitpack-native-rand**: won 4/4, lost 0/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 19.8ns | 7531.2ns | 0.3% |  |
| bitpack-native-rand | 24.5ns | 4992.0ns | 0.5% |  |
| bitpack-zeropad-rand | 19.7ns | 18331.2ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 7477.8-7713.0 ns)
   7477.8 |################################
   7489.5 |########################################
   7501.3 |
   7513.0 |
   7524.8 |
   7536.6 |
   7548.3 |
   7560.1 |
   7571.8 |
   7583.6 |
   7595.4 |
   7607.1 |
   7618.9 |
   7630.7 |
   7642.4 |
   7654.2 |
   7665.9 |
   7677.7 |
   7689.5 |
   7701.2 |
  (3 below, 1 above range)

bitpack-native-rand (n=40, range 4101.9-8433.8 ns)
   4101.9 |########################################
   4318.5 |
   4535.1 |#
   4751.7 |
   4968.3 |
   5184.9 |
   5401.5 |
   5618.1 |
   5834.7 |
   6051.3 |
   6267.9 |
   6484.5 |
   6701.1 |
   6917.7 |
   7134.2 |
   7350.8 |
   7567.4 |
   7784.0 |
   8000.6 |
   8217.2 |
  (4 below, 4 above range)

bitpack-zeropad-rand (n=40, range 18297.7-18353.4 ns)
  18297.7 |
  18300.5 |
  18303.3 |#####
  18306.1 |
  18308.9 |
  18311.6 |
  18314.4 |
  18317.2 |
  18320.0 |
  18322.8 |###############
  18325.5 |##########
  18328.3 |####################
  18331.1 |##########
  18333.9 |########################################
  18336.7 |###################################
  18339.5 |####################
  18342.2 |####################
  18345.0 |#####
  18347.8 |
  18350.6 |
  (3 below, 1 above range)

```

## Diagnostics

- **bitpack-native-rand**: CV=50.3% (high variance, measurements may be unstable)
