# Container fork, operation-density sweep at 13 bits (8192 elements, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (5.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 257 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-native dominates: 253% faster than the next best (warm-container-lanes-deferred)

warm-container-native (257 ns) leads warm-container-lanes-deferred (907 ns) by 253%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-native beats baseline by 95% (significant)

warm-container-native is -5.47 us (95%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 22.4x slower than the field

warm-container-headroom (5.75 us) is 22.4x the fastest (257 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows warm-up / thermal drift (autocorr +0.86)

warm-container-headroom's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-lanes-deferred, warm-container-kernel} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (512% apart)

The field splits into a fast tier {warm-container-native, warm-container-lanes-deferred, warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 512% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.4x the fastest

Fastest warm-container-native (257 ns) to slowest warm-container-headroom (5.75 us): 22.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-minimum's edge over baseline is significant but tiny (-3 ns, 0.04%)

warm-container-minimum differs from baseline warm-container-headroom by -3 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-native** at 257.1 ns median (-95.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 22.36x (fastest 257.1 ns, slowest 5749.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6055ns | 5806ns | 5706ns | 5872ns | 6953ns | base |
| warm-container-kernel | 1007ns | 999ns | 950ns | 999ns | 1086ns | -83.37% |
| warm-container-lanes-deferred | 991ns | 968ns | 950ns | 976ns | 1079ns | -83.63% |
| warm-container-minimum | 5803ns | 5795ns | 5709ns | 5785ns | 5951ns | -4.16% |
| warm-container-native | 322ns | 319ns | 311ns | 318ns | 343ns | -94.68% |
| warm-container-plusone | 6072ns | 5799ns | 5704ns | 5931ns | 6862ns | +0.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5986ns | 5650ns | 6851ns | base | 6.842 |
| warm-container-kernel | 945ns | 893ns | 1022ns | -84.21% | 43.342 |
| warm-container-lanes-deferred | 928ns | 890ns | 1014ns | -84.49% | 44.120 |
| warm-container-minimum | 5742ns | 5650ns | 5885ns | -4.08% | 7.133 |
| warm-container-native | 260ns | 251ns | 279ns | -95.65% | 157.374 |
| warm-container-plusone | 6006ns | 5647ns | 6781ns | +0.33% | 6.820 |

## Performance model

- Peak throughput: **162.928 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.124 | 4.4% |
| warm-container-kernel | 43.681 | 26.8% |
| warm-container-lanes-deferred | 45.147 | 27.7% |
| warm-container-minimum | 7.140 | 4.4% |
| warm-container-native | 159.315 | 97.8% |
| warm-container-plusone | 7.135 | 4.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6055ns | 6055ns | base |
| warm-container-kernel | 1007ns | 1007ns | -83.37% |
| warm-container-lanes-deferred | 991ns | 991ns | -83.63% |
| warm-container-minimum | 5803ns | 5803ns | -4.16% |
| warm-container-native | 322ns | 322ns | -94.68% |
| warm-container-plusone | 6072ns | 6072ns | +0.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5750ns | base | --- | [5660, 5788] | --- | --- | --- | --- |
| warm-container-kernel | 938ns | -4813.0ns (-83.7%) | [-4849, -4761]ns | [913, 942] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 907ns | -4852.3ns (-84.4%) | [-4873, -4742]ns | [895, 911] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5736ns | no significant difference | [-134, +86]ns | [5692, 5755] | no | 1.0000 | 1.0000 | 0 |
| warm-container-native | 257ns | -5492.3ns (-95.5%) | [-5532, -5404]ns | [255, 258] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 5741ns | no significant difference | [-103, +64]ns | [5672, 5881] | no | 0.3352 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 6832ns | -86.9% | -86.9% | -17.1% | -96.3% | -15.4% |
| 2 | 6836ns | -84.5% | -86.9% | -16.1% | -96.1% | -15.9% |
| 3 | 6499ns | -85.7% | -86.3% | -11.8% | -95.9% | -9.6% |
| 4 | 6838ns | -86.3% | -87.0% | -15.9% | -96.0% | -16.7% |
| 5 | 6905ns | -86.1% | -87.1% | -16.4% | -96.2% | -16.5% |
| 6 | 6836ns | -86.3% | -87.0% | -12.9% | -96.1% | -16.6% |
| 7 | 6836ns | -86.3% | -87.0% | -16.8% | -96.2% | -16.9% |
| 8 | 6832ns | -86.2% | -86.9% | -15.7% | -96.1% | -16.6% |
| 9 | 6889ns | -86.7% | -87.1% | -16.3% | -96.1% | -17.9% |
| 10 | 6833ns | -86.6% | -87.0% | -17.4% | -96.1% | -17.2% |
| 11 | 5665ns | -84.2% | -82.0% | +1.7% | -95.5% | -0.0% |
| 12 | 5652ns | -84.2% | -82.1% | +1.6% | -95.5% | -0.1% |
| 13 | 5649ns | -84.2% | -82.1% | +2.2% | -95.5% | -0.1% |
| 14 | 5653ns | -84.1% | -82.0% | +1.8% | -95.6% | +12.5% |
| 15 | 5653ns | -84.2% | -82.1% | +1.6% | -95.5% | +22.5% |
| 16 | 5655ns | -84.2% | -82.1% | +0.2% | -95.5% | +21.1% |
| 17 | 5805ns | -84.6% | -82.6% | -2.6% | -95.6% | +18.0% |
| 18 | 5652ns | -84.3% | -82.1% | +0.7% | -95.5% | +21.1% |
| 19 | 5652ns | -84.2% | -82.1% | +1.5% | -95.6% | +32.7% |
| 20 | 5692ns | -84.3% | -82.2% | +1.2% | -95.6% | +13.2% |
| 21 | 5906ns | -84.1% | -84.9% | -4.1% | -95.6% | -0.6% |
| 22 | 5767ns | -84.3% | -84.5% | -2.1% | -95.5% | -1.5% |
| 23 | 5870ns | -84.5% | -84.8% | -2.6% | -95.6% | +0.3% |
| 24 | 5748ns | -84.1% | -84.4% | -1.7% | -95.5% | -1.8% |
| 25 | 5750ns | -84.2% | -84.4% | +1.9% | -95.5% | -1.8% |
| 26 | 5750ns | -83.7% | -84.5% | +3.9% | -95.4% | -1.8% |
| 27 | 5752ns | -83.7% | -84.4% | -1.8% | -95.5% | -1.8% |
| 28 | 5770ns | -83.7% | -84.3% | -2.0% | -95.6% | -0.7% |
| 29 | 5770ns | -83.6% | -84.5% | +4.6% | -95.5% | -2.1% |
| 30 | 5765ns | -83.7% | -84.5% | -1.7% | -95.6% | -2.0% |
| 31 | 5646ns | -82.0% | -83.9% | +1.9% | -93.6% | +0.3% |
| 32 | 5653ns | -82.0% | -83.9% | +2.6% | -95.4% | -0.1% |
| 33 | 5650ns | -82.0% | -83.9% | +1.9% | -95.5% | -0.0% |
| 34 | 5708ns | -82.2% | -84.1% | +2.2% | -95.5% | +12.5% |
| 35 | 5648ns | -82.0% | -83.9% | +1.6% | -95.6% | +13.6% |
| 36 | 5652ns | -82.0% | -83.9% | +0.7% | -95.6% | +13.4% |
| 37 | 5655ns | -82.0% | -83.9% | +3.4% | -95.4% | +13.4% |
| 38 | 5807ns | -82.5% | -84.1% | -2.7% | -95.6% | +10.4% |
| 39 | 5664ns | -82.1% | -83.7% | +0.7% | -95.5% | +13.0% |
| 40 | 5651ns | -82.1% | -83.7% | -0.1% | -95.4% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.860 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.697 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.836 | HIGH+ (drift/warm-up) |
| warm-container-minimum | -0.115 | ok |
| warm-container-native | 0.023 | ok |
| warm-container-plusone | 0.779 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 19/40, lost 20/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 20/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 5986.2ns | 0.0% |  |
| warm-container-kernel | 2.3ns | 945.0ns | 0.2% |  |
| warm-container-lanes-deferred | 2.4ns | 928.4ns | 0.3% |  |
| warm-container-minimum | 2.9ns | 5742.1ns | 0.1% |  |
| warm-container-native | 2.9ns | 260.3ns | 1.1% |  |
| warm-container-plusone | 2.9ns | 6005.9ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5650.1-6850.6 ns)
   5650.1 |########################################
   5710.2 |##################
   5770.2 |########
   5830.2 |##
   5890.2 |##
   5950.3 |
   6010.3 |
   6070.3 |
   6130.3 |
   6190.4 |
   6250.4 |
   6310.4 |
   6370.4 |
   6430.5 |
   6490.5 |##
   6550.5 |
   6610.5 |
   6670.5 |
   6730.6 |
   6790.6 |##################
  (3 below, 2 above range)

warm-container-kernel (n=40, range 892.6-1022.0 ns)
    892.6 |########################################
    899.1 |
    905.6 |###############
    912.0 |###############
    918.5 |
    925.0 |
    931.4 |####################
    937.9 |###################################
    944.4 |
    950.8 |
    957.3 |#####
    963.8 |
    970.2 |
    976.7 |
    983.2 |
    989.6 |
    996.1 |
   1002.6 |
   1009.0 |###############
   1015.5 |###################################
  (3 below, 1 above range)

warm-container-lanes-deferred (n=40, range 890.2-1013.6 ns)
    890.2 |########################################
    896.4 |#####
    902.6 |#####
    908.8 |################
    914.9 |
    921.1 |########
    927.3 |
    933.4 |
    939.6 |
    945.8 |
    951.9 |
    958.1 |
    964.3 |
    970.4 |
    976.6 |
    982.8 |
    988.9 |
    995.1 |
   1001.3 |
   1007.4 |##################
  (2 below, 3 above range)

warm-container-minimum (n=40, range 5650.0-5884.9 ns)
   5650.0 |################################
   5661.8 |########################
   5673.5 |
   5685.3 |########################
   5697.0 |########
   5708.7 |########
   5720.5 |########
   5732.2 |################################
   5744.0 |########################################
   5755.7 |########################
   5767.5 |########################
   5779.2 |
   5790.9 |########
   5802.7 |
   5814.4 |
   5826.2 |########
   5837.9 |########
   5849.7 |
   5861.4 |########
   5873.1 |
  (5 below, 3 above range)

warm-container-native (n=40, range 251.4-278.8 ns)
    251.4 |######################
    252.8 |############################
    254.1 |#####
    255.5 |##################################
    256.9 |########################################
    258.2 |
    259.6 |###########
    261.0 |###########
    262.4 |###########
    263.7 |
    265.1 |###########
    266.5 |#################
    267.8 |
    269.2 |
    270.6 |#####
    271.9 |
    273.3 |
    274.7 |
    276.1 |
    277.4 |
  (4 below, 1 above range)

warm-container-plusone (n=40, range 5647.0-6781.3 ns)
   5647.0 |########################################
   5703.7 |#####
   5760.4 |########
   5817.1 |##
   5873.8 |#####
   5930.5 |
   5987.3 |
   6044.0 |
   6100.7 |
   6157.4 |
   6214.1 |
   6270.8 |
   6327.5 |##
   6384.3 |################
   6441.0 |##
   6497.7 |
   6554.4 |
   6611.1 |
   6667.8 |
   6724.5 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.78 (measurement drift or warm-up artifact)
