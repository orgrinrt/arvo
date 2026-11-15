# Layout::Bitpacked reading, random-access column sum: byte-aligned slot vs zero-inter-value-padding

3 variants, 40 samples per variant.
Baseline: **bitpack-aligned-rand**

## Highlights

Baseline for all deltas below: **bitpack-aligned-rand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-aligned-rand) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-aligned-rand has the worst median (4.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-native-rand at 3.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.44 us) is smaller than the fastest variant's own run-to-run std-dev (1.54 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-native-rand is inconsistent: worst-20% is 4.1x its best-20%

bitpack-native-rand's best 20% of batches run at 1.44 us but its worst 20% at 5.88 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-native-rand** at 3546.6 ns median (-28.8% vs baseline)
- Spread: 1.41x (fastest 3546.6 ns, slowest 4983.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 4606ns | 5122ns | 1927ns | 4452ns | 7747ns | base |
| bitpack-native-rand | 3747ns | 3742ns | 1514ns | 3666ns | 6225ns | -18.64% |
| bitpack-zeropad-rand | 6020ns | 4655ns | 4643ns | 4985ns | 10503ns | +30.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-aligned-rand | 4471ns | 1874ns | 7494ns | base | 0.916 |
| bitpack-native-rand | 3543ns | 1437ns | 5880ns | -20.76% | 1.156 |
| bitpack-zeropad-rand | 5942ns | 4591ns | 10334ns | +32.92% | 0.689 |

## Performance model

- Peak throughput: **2.850 Gops/s** (bitpack-native-rand; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-aligned-rand | 0.822 | 28.8% |
| bitpack-native-rand | 1.155 | 40.5% |
| bitpack-zeropad-rand | 0.890 | 31.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-aligned-rand | 4606ns | 4606ns | base |
| bitpack-native-rand | 3747ns | 3747ns | -18.64% |
| bitpack-zeropad-rand | 6020ns | 6020ns | +30.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-aligned-rand | 4983ns | base | --- | [2291, 6033] | --- | --- | --- | --- |
| bitpack-native-rand | 3547ns | no significant difference | [-1363, +196]ns | [3046, 3807] | no | 0.1539 | 0.1539 | 0 |
| bitpack-zeropad-rand | 4602ns | no significant difference | [-300, +2716]ns | [4600, 5613] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-aligned-rand | bitpack-native-rand | bitpack-zeropad-rand |
|---|---|---|---|---|
| 1 | 1 | 3792ns | +2.7% | +21.2% |
| 1 | 2 | 5086ns | -43.3% | -9.6% |
| 1 | 3 | 5714ns | -73.8% | -19.5% |
| 1 | 4 | 3284ns | -1.4% | +71.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-aligned-rand | -0.297 | moderate- |
| bitpack-native-rand | -0.162 | ok |
| bitpack-zeropad-rand | -0.082 | ok |

**Consistency summary:**

- **bitpack-native-rand**: won 3/4, lost 1/4
- **bitpack-zeropad-rand**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-aligned-rand | 46.2ns | 4470.7ns | 1.0% |  |
| bitpack-native-rand | 66.2ns | 3542.7ns | 1.9% |  |
| bitpack-zeropad-rand | 25.6ns | 5942.5ns | 0.4% |  |

## Distribution (algo ns)

```
bitpack-aligned-rand (n=40, range 1874.5-7494.5 ns)
   1874.5 |########################################
   2155.5 |########################################
   2436.5 |
   2717.5 |
   2998.5 |
   3279.5 |
   3560.5 |
   3841.5 |########
   4122.5 |
   4403.5 |########
   4684.5 |########
   4965.5 |
   5246.5 |################
   5527.5 |################
   5808.5 |################
   6089.5 |########################
   6370.5 |########################
   6651.5 |################
   6932.5 |########
   7213.5 |########
  (7 below, 4 above range)

bitpack-native-rand (n=40, range 1437.0-5880.1 ns)
   1437.0 |
   1659.1 |
   1881.3 |######
   2103.5 |#################################
   2325.6 |
   2547.8 |
   2769.9 |######
   2992.1 |####################
   3214.2 |####################
   3436.4 |########################################
   3658.6 |####################
   3880.7 |######
   4102.9 |######
   4325.0 |#############
   4547.2 |
   4769.4 |######
   4991.5 |######
   5213.7 |######
   5435.8 |#############
   5658.0 |
  (5 below, 4 above range)

bitpack-zeropad-rand (n=40, range 4590.9-10333.7 ns)
   4590.9 |########################################
   4878.0 |
   5165.2 |##
   5452.3 |##################
   5739.4 |
   6026.6 |
   6313.7 |
   6600.9 |
   6888.0 |
   7175.1 |
   7462.3 |
   7749.4 |
   8036.6 |
   8323.7 |
   8610.8 |##
   8898.0 |
   9185.1 |
   9472.3 |
   9759.4 |
  10046.5 |
  (4 below, 5 above range)

```

## Diagnostics

- **bitpack-aligned-rand**: CV=51.2% (high variance, measurements may be unstable)
- **bitpack-aligned-rand**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **bitpack-native-rand**: CV=43.4% (high variance, measurements may be unstable)
- **bitpack-native-rand**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **bitpack-zeropad-rand**: CV=44.7% (high variance, measurements may be unstable)
