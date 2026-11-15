# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (162.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 23.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 329% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (23.22 us) leads bitpack-write-windowed (99.65 us) by 329%, a clear separation rather than a photo finish. CV 35.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 89% (significant)

bitpack-write-dense is -143.81 us (89%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 7.0x slower than the field

bitpack-write-aligned (162.15 us) is 7.0x the fastest (23.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 35.2%)

bitpack-write-dense wins on median (23.22 us) yet has the highest variance (CV 35.2%), while bitpack-write-windowed is the steadiest (CV 14.6%, 99.65 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 7.0x the fastest

Fastest bitpack-write-dense (23.22 us) to slowest bitpack-write-aligned (162.15 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 2.6x its best-20%

bitpack-write-dense's best 20% of batches run at 12.29 us but its worst 20% at 31.50 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 23218.0 ns median (-85.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.98x (fastest 23218.0 ns, slowest 162147.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 161152ns | 162255ns | 116017ns | 159183ns | 212195ns | base |
| bitpack-write-dense | 22723ns | 23321ns | 12380ns | 23208ns | 31612ns | -85.90% |
| bitpack-write-windowed | 105012ns | 99757ns | 91104ns | 101676ns | 128928ns | -34.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 161036ns | 115907ns | 212084ns | base | 0.407 |
| bitpack-write-dense | 22620ns | 12294ns | 31504ns | -85.95% | 2.897 |
| bitpack-write-windowed | 104891ns | 90980ns | 128814ns | -34.87% | 0.625 |

## Performance model

- Peak throughput: **5.331 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.404 | 7.6% |
| bitpack-write-dense | 2.823 | 53.0% |
| bitpack-write-windowed | 0.658 | 12.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 161152ns | 161152ns | base |
| bitpack-write-dense | 22723ns | 22723ns | -85.90% |
| bitpack-write-windowed | 105012ns | 105012ns | -34.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 162148ns | base | --- | [161236, 174597] | --- | --- | --- | --- |
| bitpack-write-dense | 23218ns | -143807.4ns (-88.7%) | [-147477, -131976]ns | [16983, 30128] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 99654ns | -69496.1ns (-42.9%) | [-74842, -52960]ns | [96297, 108414] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 174656ns | -82.7% | -42.9% |
| 1 | 2 | 212455ns | -85.8% | -56.3% |
| 1 | 3 | 161592ns | -92.4% | -36.9% |
| 1 | 4 | 116046ns | -85.4% | +9.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.086 | ok |
| bitpack-write-dense | 0.148 | ok |
| bitpack-write-windowed | 0.062 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 25.6ns | 161036.3ns | 0.0% |  |
| bitpack-write-dense | 26.9ns | 22620.4ns | 0.1% |  |
| bitpack-write-windowed | 32.0ns | 104890.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 115907.3-212083.5 ns)
  115907.3 |############################
  120716.1 |########
  125524.9 |
  130333.7 |
  135142.5 |
  139951.4 |
  144760.2 |
  149569.0 |
  154377.8 |####
  159186.6 |########################################
  163995.4 |####
  168804.2 |
  173613.0 |####################################
  178421.8 |
  183230.6 |
  188039.5 |
  192848.3 |
  197657.1 |
  202465.9 |
  207274.7 |
  (3 below, 7 above range)

bitpack-write-dense (n=40, range 12294.5-31503.9 ns)
  12294.5 |#################
  13255.0 |
  14215.4 |
  15175.9 |
  16136.4 |############################
  17096.8 |
  18057.3 |
  19017.8 |
  19978.2 |
  20938.7 |
  21899.2 |
  22859.6 |
  23820.1 |
  24780.6 |
  25741.1 |
  26701.5 |
  27662.0 |
  28622.5 |#####
  29582.9 |########################################
  30543.4 |##
  (4 below, 3 above range)

bitpack-write-windowed (n=40, range 90979.6-128814.3 ns)
  90979.6 |########################################
  92871.4 |#####
  94763.1 |#####
  96654.8 |#####
  98546.6 |###################################
  100438.3 |
  102330.0 |
  104221.8 |#####
  106113.5 |###############
  108005.2 |###############
  109897.0 |#########################
  111788.7 |
  113680.4 |
  115572.2 |
  117463.9 |
  119355.6 |
  121247.4 |
  123139.1 |
  125030.8 |
  126922.6 |##########
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-write-aligned**: CV=21.4% (high variance, measurements may be unstable)
- **bitpack-write-dense**: CV=36.2% (high variance, measurements may be unstable)
