# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (679.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 676.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.3% (fastest 676.92 us to slowest 679.09 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.17 us) is smaller than the fastest variant's own run-to-run std-dev (37.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 676920.9 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 676920.9 ns, slowest 679086.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Dense footprint: sequential sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112804717 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 705357ns | 679500ns | 642828ns | 680049ns | 843807ns | base |
| bitpack-footprint-dense-alt | 688896ns | 677213ns | 638312ns | 688621ns | 740302ns | -2.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 705005ns | 642637ns | 843414ns | base | 5.949 |
| bitpack-footprint-dense-alt | 688577ns | 638069ns | 739783ns | -2.33% | 6.091 |

## Performance model

- Peak throughput: **6.573 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.176 | 94.0% |
| bitpack-footprint-dense-alt | 6.196 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 705357ns | 705357ns | base |
| bitpack-footprint-dense-alt | 688896ns | 688896ns | -2.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 679087ns | base | --- | [669805, 683310] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 676921ns | no significant difference | [-30602, +25405]ns | [675516, 711132] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 640457ns | +5.5% |
| 1 | 2 | 803201ns | -9.0% |
| 1 | 3 | 683168ns | +3.9% |
| 1 | 4 | 670114ns | -4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.476 | moderate- |
| bitpack-footprint-dense-alt | -0.145 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 52.5ns | 705005.1ns | 0.0% |  |
| bitpack-footprint-dense-alt | 33.6ns | 688577.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 642637.4-843414.4 ns)
  642637.4 |####
  652676.2 |#############
  662715.1 |########################################
  672753.9 |#############
  682792.8 |###################################
  692831.6 |
  702870.5 |
  712909.3 |
  722948.2 |#############
  732987.1 |####
  743025.9 |####
  753064.8 |
  763103.6 |
  773142.5 |
  783181.3 |
  793220.2 |
  803259.0 |
  813297.9 |
  823336.7 |
  833375.6 |
  (6 below, 5 above range)

bitpack-footprint-dense-alt (n=40, range 638069.3-739782.5 ns)
  638069.3 |####################
  643155.0 |
  648240.7 |
  653326.3 |
  658412.0 |
  663497.6 |
  668583.3 |
  673669.0 |########################################
  678754.6 |
  683840.3 |
  688925.9 |
  694011.6 |###
  699097.2 |
  704182.9 |###
  709268.6 |################
  714354.2 |###
  719439.9 |###
  724525.5 |##########
  729611.2 |#############
  734696.9 |
  (4 below, 2 above range)

```
