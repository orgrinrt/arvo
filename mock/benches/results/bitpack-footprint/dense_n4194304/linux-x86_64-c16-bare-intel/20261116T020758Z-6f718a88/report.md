# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (676.47 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 650.44 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 650444.9 ns median (-3.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.04x (fastest 650444.9 ns, slowest 676473.7 ns)

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
| bitpack-footprint-dense | 677344ns | 676821ns | 641992ns | 678909ns | 708002ns | base |
| bitpack-footprint-dense-alt | 652968ns | 650677ns | 638130ns | 650391ns | 675535ns | -3.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 677023ns | 641797ns | 707608ns | base | 6.195 |
| bitpack-footprint-dense-alt | 652709ns | 637924ns | 675161ns | -3.59% | 6.426 |

## Performance model

- Peak throughput: **6.575 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.200 | 94.3% |
| bitpack-footprint-dense-alt | 6.448 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 677344ns | 677344ns | base |
| bitpack-footprint-dense-alt | 652968ns | 652968ns | -3.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 676474ns | base | --- | [674730, 679267] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 650445ns | -17651.1ns (-2.6%) | [-22062, -1265]ns | [638056, 657652] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 678928ns | -3.2% |
| 1 | 2 | 705201ns | -9.5% |
| 1 | 3 | 638333ns | -0.1% |
| 1 | 4 | 675098ns | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.439 | moderate- |
| bitpack-footprint-dense-alt | -0.211 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 30.6ns | 677023.3ns | 0.0% |  |
| bitpack-footprint-dense-alt | 38.3ns | 652709.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 641796.6-707608.4 ns)
  641796.6 |
  645087.2 |
  648377.8 |####
  651668.4 |
  654959.0 |############
  658249.6 |
  661540.2 |
  664830.8 |
  668121.3 |
  671411.9 |################
  674702.5 |########################################
  677993.1 |################
  681283.7 |
  684574.3 |####
  687864.9 |
  691155.5 |
  694446.1 |
  697736.7 |
  701027.3 |########
  704317.8 |################################
  (6 below, 1 above range)

bitpack-footprint-dense-alt (n=40, range 637923.8-675160.5 ns)
  637923.8 |########################################
  639785.6 |
  641647.5 |
  643509.3 |##
  645371.1 |
  647233.0 |
  649094.8 |
  650956.6 |
  652818.5 |
  654680.3 |##
  656542.2 |##################
  658404.0 |##
  660265.8 |
  662127.7 |
  663989.5 |
  665851.3 |
  667713.2 |
  669575.0 |
  671436.9 |
  673298.7 |#####################
  (4 below, 3 above range)

```
