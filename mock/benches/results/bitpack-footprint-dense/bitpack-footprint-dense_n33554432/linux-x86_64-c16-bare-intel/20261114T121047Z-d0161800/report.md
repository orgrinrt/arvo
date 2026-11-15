# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (5.90 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 5.89 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (11.89 us) is smaller than the fastest variant's own run-to-run std-dev (333.78 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 5885534.4 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 5885534.4 ns, slowest 5897426.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 6158069ns | 5898261ns | 5880716ns | 5905470ns | 7193218ns | base |
| bitpack-footprint-dense-alt | 5979741ns | 5886411ns | 5880476ns | 5888658ns | 6352254ns | -2.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 6157145ns | 5879773ns | 7192368ns | base | 5.450 |
| bitpack-footprint-dense-alt | 5978869ns | 5879624ns | 6351375ns | -2.90% | 5.612 |

## Performance model

- Peak throughput: **5.707 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.690 | 99.7% |
| bitpack-footprint-dense-alt | 5.701 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 6158069ns | 6158069ns | base |
| bitpack-footprint-dense-alt | 5979741ns | 5979741ns | -2.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 5897427ns | base | --- | [5886916, 5902009] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 5885534ns | -4254.2ns (-0.1%) | [-10716, -2134]ns | [5884303, 5892669] | YES | 0.0007 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 6802066ns | -13.4% |
| 1 | 2 | 5902009ns | -0.1% |
| 1 | 3 | 5889406ns | -0.1% |
| 1 | 4 | 5880662ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.070 | ok |
| bitpack-footprint-dense-alt | -0.022 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 101.3ns | 6157144.5ns | 0.0% |  |
| bitpack-footprint-dense-alt | 112.8ns | 5978869.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 5879773.1-7192367.6 ns)
  5879773.1 |########################################
  5945402.9 |
  6011032.6 |
  6076662.3 |#
  6142292.0 |
  6207921.8 |
  6273551.5 |
  6339181.2 |
  6404810.9 |
  6470440.6 |
  6536070.4 |
  6601700.1 |
  6667329.8 |
  6732959.5 |####
  6798589.3 |##
  6864219.0 |
  6929848.7 |
  6995478.4 |
  7061108.1 |
  7126737.9 |
  (3 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 5879624.0-6351374.5 ns)
  5879624.0 |########################################
  5903211.5 |##
  5926799.0 |
  5950386.5 |
  5973974.1 |
  5997561.6 |
  6021149.1 |
  6044736.7 |
  6068324.2 |
  6091911.7 |
  6115499.2 |
  6139086.8 |
  6162674.3 |
  6186261.8 |
  6209849.4 |
  6233436.9 |
  6257024.4 |
  6280611.9 |
  6304199.5 |
  6327787.0 |
  (4 below, 3 above range)

```
