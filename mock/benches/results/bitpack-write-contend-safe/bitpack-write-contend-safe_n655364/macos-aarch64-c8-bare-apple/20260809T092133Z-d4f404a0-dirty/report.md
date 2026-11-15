# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (121.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 7.94 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1130% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (7.94 us) leads bitpack-write-windowed (97.65 us) by 1130%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 95% (significant)

bitpack-write-dense is -116.09 us (95%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 15.3x slower than the field

bitpack-write-aligned (121.78 us) is 15.3x the fastest (7.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.60)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 15.3x the fastest

Fastest bitpack-write-dense (7.94 us) to slowest bitpack-write-aligned (121.78 us): 15.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 7936.5 ns median (-93.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 15.34x (fastest 7936.5 ns, slowest 121775.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 126388ns | 122005ns | 114900ns | 122392ns | 149865ns | base |
| bitpack-write-dense | 8147ns | 8046ns | 7470ns | 8116ns | 8916ns | -93.55% |
| bitpack-write-windowed | 102666ns | 97759ns | 93093ns | 98596ns | 124447ns | -18.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 126144ns | 114705ns | 149644ns | base | 0.520 |
| bitpack-write-dense | 8041ns | 7377ns | 8799ns | -93.63% | 8.150 |
| bitpack-write-windowed | 102443ns | 92915ns | 124187ns | -18.79% | 0.640 |

## Performance model

- Peak throughput: **8.884 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.538 | 6.1% |
| bitpack-write-dense | 8.258 | 92.9% |
| bitpack-write-windowed | 0.671 | 7.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 126388ns | 126388ns | base |
| bitpack-write-dense | 8147ns | 8147ns | -93.55% |
| bitpack-write-windowed | 102666ns | 102666ns | -18.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 121776ns | base | --- | [119820, 124559] | --- | --- | --- | --- |
| bitpack-write-dense | 7936ns | -114245.9ns (-93.8%) | [-116476, -111751]ns | [7716, 8332] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 97654ns | -22003.3ns (-18.1%) | [-27907, -20625]ns | [94270, 102367] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|
| 1 | 147721ns | -94.8% | -35.6% |
| 2 | 115377ns | -93.5% | -18.3% |
| 3 | 116545ns | -93.6% | +8.3% |
| 4 | 114751ns | -93.5% | -19.1% |
| 5 | 121809ns | -94.0% | -24.6% |
| 6 | 131113ns | -94.0% | -20.0% |
| 7 | 120625ns | -93.9% | -16.8% |
| 8 | 156926ns | -95.3% | -33.0% |
| 9 | 120402ns | -93.9% | -16.9% |
| 10 | 121472ns | -93.9% | -21.4% |
| 11 | 146175ns | -93.6% | -35.9% |
| 12 | 124758ns | -93.2% | -24.2% |
| 13 | 115229ns | -92.7% | -9.3% |
| 14 | 114754ns | -93.1% | -10.6% |
| 15 | 114944ns | -93.1% | -18.9% |
| 16 | 114539ns | -92.8% | -18.6% |
| 17 | 115003ns | -93.1% | -19.2% |
| 18 | 114755ns | -93.0% | -16.3% |
| 19 | 121632ns | -93.6% | -22.5% |
| 20 | 133610ns | -93.5% | -20.3% |
| 21 | 139162ns | -93.8% | +10.8% |
| 22 | 140094ns | -93.8% | -3.1% |
| 23 | 142621ns | -93.9% | -28.4% |
| 24 | 150290ns | -94.3% | -32.8% |
| 25 | 174165ns | -95.0% | -41.6% |
| 26 | 119907ns | -92.7% | -12.1% |
| 27 | 115152ns | -92.4% | -10.7% |
| 28 | 114026ns | -92.4% | +1.8% |
| 29 | 122495ns | -92.9% | +8.8% |
| 30 | 114867ns | -92.3% | -13.6% |
| 31 | 115411ns | -93.6% | -19.0% |
| 32 | 121743ns | -94.0% | -23.4% |
| 33 | 119734ns | -93.8% | -20.8% |
| 34 | 123914ns | -93.8% | -24.9% |
| 35 | 124126ns | -93.6% | -6.5% |
| 36 | 123862ns | -93.6% | -24.3% |
| 37 | 124594ns | -93.6% | -16.8% |
| 38 | 126726ns | -93.6% | -26.2% |
| 39 | 124523ns | -93.9% | -24.7% |
| 40 | 126226ns | -93.8% | -26.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.341 | moderate+ |
| bitpack-write-dense | 0.599 | HIGH+ (drift/warm-up) |
| bitpack-write-windowed | 0.341 | moderate+ |

**Consistency summary:**

- **bitpack-write-dense**: won 40/40, lost 0/40
- **bitpack-write-windowed**: won 36/40, lost 4/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 5.0ns | 126144.5ns | 0.0% |  |
| bitpack-write-dense | 4.0ns | 8041.1ns | 0.0% |  |
| bitpack-write-windowed | 7.0ns | 102442.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 114704.8-149644.3 ns)
  114704.8 |########################################
  116451.8 |####
  118198.7 |########
  119945.7 |################
  121692.7 |############
  123439.7 |########################
  125186.6 |########
  126933.6 |
  128680.6 |
  130427.6 |####
  132174.5 |####
  133921.5 |
  135668.5 |
  137415.5 |####
  139162.5 |####
  140909.4 |####
  142656.4 |
  144403.4 |
  146150.4 |########
  147897.3 |
  (2 below, 3 above range)

bitpack-write-dense (n=40, range 7376.7-8798.5 ns)
   7376.7 |################
   7447.8 |########################
   7518.9 |########
   7590.0 |########
   7661.1 |########
   7732.2 |########
   7803.3 |################
   7874.3 |########################################
   7945.4 |################
   8016.5 |
   8087.6 |########
   8158.7 |
   8229.8 |########
   8300.9 |
   8372.0 |########
   8443.1 |########
   8514.2 |
   8585.3 |########################################
   8656.4 |################################
   8727.4 |########
  (6 below, 2 above range)

bitpack-write-windowed (n=40, range 92915.1-124186.5 ns)
  92915.1 |########################################
  94478.7 |#############
  96042.3 |###
  97605.8 |
  99169.4 |##########
  100733.0 |##########
  102296.5 |##########
  103860.1 |#############
  105423.7 |###
  106987.3 |
  108550.8 |
  110114.4 |
  111678.0 |
  113241.5 |
  114805.1 |######
  116368.7 |
  117932.3 |
  119495.8 |
  121059.4 |
  122623.0 |
  (3 below, 4 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.60 (measurement drift or warm-up artifact)
