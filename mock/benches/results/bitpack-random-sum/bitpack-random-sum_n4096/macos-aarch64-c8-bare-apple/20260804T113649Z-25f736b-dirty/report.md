# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-native-rand dominates: 29% faster than the next best (bitpack-aligned-rand)

bitpack-native-rand (2.71 us) leads bitpack-aligned-rand (3.50 us) by 29%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-zeropad-rand is an outlier: 2.2x slower than the field

bitpack-zeropad-rand (5.88 us) is 2.2x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-aligned-rand shows warm-up / thermal drift (autocorr +0.70)

bitpack-aligned-rand's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-native-rand** at 2709.4 ns median (-22.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.17x (fastest 2709.4 ns, slowest 5875.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 3536ns | 3575ns | 3288ns | 3563ns | 3702ns | base |
| bitpack-native-rand | 2868ns | 2783ns | 2776ns | 2805ns | 3147ns | -18.90% |
| bitpack-zeropad-rand | 6389ns | 5960ns | 5853ns | 6079ns | 7855ns | +80.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 3460ns | 3216ns | 3621ns | base | 1.184 |
| bitpack-native-rand | 2786ns | 2704ns | 3035ns | -19.47% | 1.470 |
| bitpack-zeropad-rand | 6291ns | 5776ns | 7690ns | +81.81% | 0.651 |

## Performance model

- Peak throughput: **1.515 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 1.171 | 77.3% |
| bitpack-native-rand | 1.512 | 99.8% |
| bitpack-zeropad-rand | 0.697 | 46.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 3536ns | 3536ns | base |
| bitpack-native-rand | 2868ns | 2868ns | -18.90% |
| bitpack-zeropad-rand | 6389ns | 6389ns | +80.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 3498ns | base | --- | [3492, 3506] | --- | --- | --- | --- |
| bitpack-native-rand | 2709ns | -778.1ns (-22.2%) | [-789, -547]ns | [2706, 2715] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-zeropad-rand | 5876ns | +2588.9ns (+74.0%) | [+2374, +2725]ns | [5848, 6262] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|
| 1 | 3445ns | -21.5% | +68.1% |
| 2 | 3213ns | -15.8% | +81.8% |
| 3 | 3218ns | -16.0% | +80.4% |
| 4 | 3218ns | -15.5% | +80.7% |
| 5 | 3216ns | -15.5% | +80.2% |
| 6 | 3215ns | -15.7% | +86.8% |
| 7 | 3222ns | -15.8% | +80.4% |
| 8 | 3242ns | -16.4% | +79.6% |
| 9 | 3211ns | -15.8% | +75.7% |
| 10 | 3215ns | -15.9% | +83.2% |
| 11 | 3497ns | -22.3% | +65.6% |
| 12 | 3506ns | -22.1% | +65.5% |
| 13 | 3491ns | -22.5% | +66.0% |
| 14 | 3761ns | -28.1% | +53.7% |
| 15 | 3489ns | -22.4% | +82.6% |
| 16 | 3492ns | -22.4% | +68.0% |
| 17 | 3505ns | -22.6% | +67.2% |
| 18 | 3493ns | -22.6% | +84.2% |
| 19 | 3495ns | -22.7% | +355.0% |
| 20 | 3489ns | -22.4% | +66.4% |
| 21 | 3498ns | -7.4% | +68.3% |
| 22 | 3498ns | -15.4% | +67.3% |
| 23 | 3503ns | -15.5% | +67.6% |
| 24 | 3495ns | -15.3% | +97.7% |
| 25 | 3517ns | -15.7% | +65.2% |
| 26 | 3791ns | -21.8% | +55.0% |
| 27 | 3549ns | -16.1% | +65.6% |
| 28 | 3534ns | -8.1% | +75.4% |
| 29 | 3543ns | -16.4% | +65.2% |
| 30 | 3503ns | -15.8% | +67.8% |
| 31 | 3498ns | -22.7% | +80.9% |
| 32 | 3720ns | -27.2% | +70.7% |
| 33 | 3518ns | -23.2% | +80.1% |
| 34 | 3550ns | -23.8% | +78.2% |
| 35 | 3510ns | -22.9% | +81.7% |
| 36 | 3518ns | -22.9% | +89.2% |
| 37 | 3520ns | -23.1% | +82.6% |
| 38 | 3509ns | -22.9% | +83.0% |
| 39 | 3500ns | -22.7% | +82.7% |
| 40 | 3495ns | -22.5% | +82.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | 0.702 | HIGH+ (drift/warm-up) |
| bitpack-native-rand | 0.653 | HIGH+ (drift/warm-up) |
| bitpack-zeropad-rand | 0.004 | ok |

**Consistency summary:**

- **bitpack-native-rand**: won 40/40, lost 0/40
- **bitpack-zeropad-rand**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 2.2ns | 3460.0ns | 0.1% |  |
| bitpack-native-rand | 2.6ns | 2786.2ns | 0.1% |  |
| bitpack-zeropad-rand | 3.0ns | 6290.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 3215.8-3621.0 ns)
   3215.8 |##########
   3236.1 |###
   3256.3 |
   3276.6 |
   3296.9 |
   3317.1 |
   3337.4 |
   3357.6 |
   3377.9 |
   3398.2 |
   3418.4 |
   3438.7 |###
   3459.0 |
   3479.2 |########################################
   3499.5 |####################################
   3519.7 |###
   3540.0 |##########
   3560.3 |
   3580.5 |
   3600.8 |
  (5 below, 3 above range)

bitpack-native-rand (n=40, range 2703.6-3035.1 ns)
   2703.6 |########################################
   2720.1 |#
   2736.7 |
   2753.3 |
   2769.9 |
   2786.4 |
   2803.0 |
   2819.6 |
   2836.2 |
   2852.7 |
   2869.3 |
   2885.9 |
   2902.5 |
   2919.0 |
   2935.6 |#
   2952.2 |#########
   2968.8 |#
   2985.3 |
   3001.9 |
   3018.5 |
  (3 below, 2 above range)

bitpack-zeropad-rand (n=40, range 5775.7-7690.0 ns)
   5775.7 |########################################
   5871.4 |###########
   5967.1 |##
   6062.8 |
   6158.5 |##
   6254.2 |######
   6350.0 |#################
   6445.7 |
   6541.4 |
   6637.1 |##
   6732.8 |
   6828.5 |##
   6924.3 |
   7020.0 |
   7115.7 |
   7211.4 |
   7307.1 |
   7402.8 |
   7498.6 |
   7594.3 |
  (1 below, 1 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **bitpack-native-rand**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **bitpack-zeropad-rand**: CV=24.9% (high variance, measurements may be unstable)
