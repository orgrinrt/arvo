# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (5.89 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 5.88 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 5881051.9 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 5881051.9 ns, slowest 5892474.6 ns)

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
| Counter frequency | 2112547631 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 6170363ns | 5893612ns | 5882927ns | 6027552ns | 6886230ns | base |
| bitpack-footprint-dense-alt | 5883239ns | 5882178ns | 5875917ns | 5882968ns | 5891376ns | -4.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 6169288ns | 5881768ns | 6885183ns | base | 5.439 |
| bitpack-footprint-dense-alt | 5882162ns | 5874835ns | 5890288ns | -4.65% | 5.704 |

## Performance model

- Peak throughput: **5.712 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.694 | 99.7% |
| bitpack-footprint-dense-alt | 5.706 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 6170363ns | 6170363ns | base |
| bitpack-footprint-dense-alt | 5883239ns | 5883239ns | -4.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 5892475ns | base | --- | [5887619, 5898335] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 5881052ns | -10745.1ns (-0.2%) | [-20231, -5302]ns | [5879375, 5884547] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 5887886ns | -0.1% |
| 1 | 2 | 6879681ns | -14.5% |
| 1 | 3 | 5884571ns | -0.1% |
| 1 | 4 | 5892382ns | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.421 | moderate- |
| bitpack-footprint-dense-alt | 0.036 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 118.6ns | 6169287.9ns | 0.0% |  |
| bitpack-footprint-dense-alt | 117.7ns | 5882161.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 5881767.5-6885183.4 ns)
  5881767.5 |########################################
  5931938.3 |
  5982109.1 |
  6032279.9 |
  6082450.7 |
  6132621.5 |#
  6182792.3 |
  6232963.1 |
  6283133.9 |
  6333304.7 |
  6383475.5 |
  6433646.3 |
  6483817.0 |
  6533987.8 |
  6584158.6 |
  6634329.4 |
  6684500.2 |
  6734671.0 |
  6784841.8 |
  6835012.6 |###############
  (4 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 5874835.0-5890288.5 ns)
  5874835.0 |####################
  5875607.7 |##########
  5876380.3 |####################
  5877153.0 |####################
  5877925.7 |####################
  5878698.4 |##############################
  5879471.0 |########################################
  5880243.7 |##########
  5881016.4 |####################
  5881789.1 |##########
  5882561.7 |##########
  5883334.4 |####################
  5884107.1 |
  5884879.8 |####################
  5885652.4 |##########
  5886425.1 |##########
  5887197.8 |##############################
  5887970.5 |####################
  5888743.1 |
  5889515.8 |##########
  (3 below, 4 above range)

```
