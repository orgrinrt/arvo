# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 72% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (4.37 us) leads bitpack-aligned-rand (7.49 us) by 72%, a clear separation rather than a photo finish. CV 62.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-native-rand beats baseline by 42% (significant)

bitpack-native-rand is -3.12 us (42%) faster than baseline bitpack-aligned-rand, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-zeropad-rand is an outlier: 4.2x slower than the field

bitpack-zeropad-rand (18.35 us) is 4.2x the fastest (4.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-native-rand is fastest but the noisiest (CV 62.1%)

bitpack-native-rand wins on median (4.37 us) yet has the highest variance (CV 62.1%), while bitpack-zeropad-rand is the steadiest (CV 0.1%, 18.35 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.2x the fastest

Fastest bitpack-native-rand (4.37 us) to slowest bitpack-zeropad-rand (18.35 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-native-rand is inconsistent: worst-20% is 2.4x its best-20%

bitpack-native-rand's best 20% of batches run at 4.24 us but its worst 20% at 10.32 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 4365.8 ns median (-41.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.20x (fastest 4365.8 ns, slowest 18346.3 ns)

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
| Counter frequency | 2112835320 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 7588ns | 7554ns | 7544ns | 7554ns | 7734ns | base |
| bitpack-native-rand | 5630ns | 4431ns | 4302ns | 4442ns | 10523ns | -25.80% |
| bitpack-zeropad-rand | 18419ns | 18421ns | 18398ns | 18418ns | 18446ns | +142.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 7516ns | 7477ns | 7643ns | base | 2.180 |
| bitpack-native-rand | 5536ns | 4236ns | 10322ns | -26.34% | 2.959 |
| bitpack-zeropad-rand | 18350ns | 18334ns | 18368ns | +144.14% | 0.893 |

## Performance model

- Peak throughput: **3.868 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 2.188 | 56.6% |
| bitpack-native-rand | 3.753 | 97.0% |
| bitpack-zeropad-rand | 0.893 | 23.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 7588ns | 7588ns | base |
| bitpack-native-rand | 5630ns | 5630ns | -25.80% |
| bitpack-zeropad-rand | 18419ns | 18419ns | +142.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 7487ns | base | --- | [7486, 7489] | --- | --- | --- | --- |
| bitpack-native-rand | 4366ns | -3121.4ns (-41.7%) | [-3236, -3111]ns | [4250, 4375] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 18346ns | +10859.6ns (+145.0%) | [+10854, +10872]ns | [18341, 18357] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 7486ns | -42.8% | +145.2% |
| 1 | 2 | 7487ns | -41.6% | +145.0% |
| 1 | 3 | 7488ns | -42.0% | +144.9% |
| 1 | 4 | 7490ns | -43.3% | +145.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | 0.195 | ok |
| bitpack-native-rand | -0.193 | ok |
| bitpack-zeropad-rand | -0.266 | moderate- |

**Consistency summary:**

- **bitpack-native-rand**: won 4/4, lost 0/4
- **bitpack-zeropad-rand**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 19.2ns | 7516.2ns | 0.3% |  |
| bitpack-native-rand | 25.0ns | 5536.5ns | 0.5% |  |
| bitpack-zeropad-rand | 19.5ns | 18349.9ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 7477.4-7642.7 ns)
   7477.4 |#########
   7485.7 |########################################
   7494.0 |#
   7502.2 |
   7510.5 |
   7518.7 |
   7527.0 |
   7535.3 |
   7543.5 |
   7551.8 |
   7560.1 |
   7568.3 |
   7576.6 |
   7584.8 |
   7593.1 |
   7601.4 |
   7609.6 |
   7617.9 |
   7626.2 |
   7634.4 |
  (5 below, 2 above range)

bitpack-native-rand (n=40, range 4236.2-10322.3 ns)
   4236.2 |########################################
   4540.5 |
   4844.8 |
   5149.1 |
   5453.4 |###
   5757.7 |
   6062.0 |
   6366.3 |
   6670.6 |
   6974.9 |#
   7279.2 |#
   7583.6 |
   7887.9 |
   8192.2 |
   8496.5 |
   8800.8 |
   9105.1 |
   9409.4 |
   9713.7 |
  10018.0 |
  (5 below, 5 above range)

bitpack-zeropad-rand (n=40, range 18333.6-18368.0 ns)
  18333.6 |################
  18335.3 |########
  18337.0 |################################
  18338.8 |################
  18340.5 |########################################
  18342.2 |########
  18343.9 |################
  18345.7 |########
  18347.4 |
  18349.1 |
  18350.8 |########
  18352.5 |########
  18354.3 |########
  18356.0 |########################
  18357.7 |
  18359.4 |########
  18361.1 |########################
  18362.9 |########################################
  18364.6 |################
  18366.3 |
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-native-rand**: CV=49.0% (high variance, measurements may be unstable)
