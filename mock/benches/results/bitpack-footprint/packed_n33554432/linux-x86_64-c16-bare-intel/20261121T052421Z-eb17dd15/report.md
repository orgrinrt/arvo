# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 360% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (4.75 ms) leads bitpack-footprint-packed-naive (21.87 ms) by 360%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 5.7%)

bitpack-footprint-packed wins on median (4.75 ms) yet has the highest variance (CV 5.7%), while bitpack-footprint-packed-naive is the steadiest (CV 0.4%, 21.87 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (4.75 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-footprint-packed (4.75 ms) to slowest bitpack-footprint-packed-naive (21.87 ms): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 4749115.7 ns median
- 1 variant significantly slower than baseline
- Spread: 4.60x (fastest 4749115.7 ns, slowest 21866097.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112746100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 4850919ns | 4749934ns | 4736497ns | 4750251ns | 5267346ns | base |
| bitpack-footprint-packed-naive | 21872114ns | 21867075ns | 21805320ns | 21858877ns | 21978621ns | +350.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 4850069ns | 4735674ns | 5266499ns | base | 6.918 |
| bitpack-footprint-packed-naive | 21871113ns | 21804316ns | 21977606ns | +350.94% | 1.534 |

## Performance model

- Peak throughput: **7.085 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.065 | 99.7% |
| bitpack-footprint-packed-naive | 1.535 | 21.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 4850919ns | 4850919ns | base |
| bitpack-footprint-packed-naive | 21872114ns | 21872114ns | +350.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 4749116ns | base | --- | [4744645, 4753457] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 21866098ns | +17120163.8ns (+360.5%) | [+17094576, +17128305]ns | [21846267, 21873047] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 4738580ns | +361.7% |
| 1 | 2 | 4751003ns | +360.4% |
| 1 | 3 | 4746035ns | +360.6% |
| 1 | 4 | 5133187ns | +325.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.086 | ok |
| bitpack-footprint-packed-naive | 0.111 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 154.4ns | 4850069.1ns | 0.0% |  |
| bitpack-footprint-packed-naive | 256.4ns | 21871113.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 4735674.2-5266499.4 ns)
  4735674.2 |########################################
  4762215.4 |#####
  4788756.7 |
  4815298.0 |
  4841839.2 |
  4868380.5 |
  4894921.7 |
  4921463.0 |
  4948004.2 |
  4974545.5 |
  5001086.8 |
  5027628.0 |
  5054169.3 |
  5080710.5 |
  5107251.8 |
  5133793.1 |
  5160334.3 |
  5186875.6 |
  5213416.8 |
  5239958.1 |
  (3 below, 5 above range)

bitpack-footprint-packed-naive (n=40, range 21804316.1-21977606.2 ns)
  21804316.1 |#########################
  21812980.6 |####################
  21821645.1 |##########
  21830309.6 |
  21838974.1 |#####
  21847638.6 |#####
  21856303.1 |####################
  21864967.6 |###################################
  21873632.2 |########################################
  21882296.7 |#####
  21890961.2 |#####
  21899625.7 |
  21908290.2 |#####
  21916954.7 |
  21925619.2 |
  21934283.7 |
  21942948.2 |
  21951612.7 |
  21960277.2 |#####
  21968941.7 |
  (2 below, 2 above range)

```
