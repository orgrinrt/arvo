# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7.72 us) is smaller than the fastest variant's own run-to-run std-dev (12.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (470.37 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 470366.7 ns median
- Spread: 1.02x (fastest 470366.7 ns, slowest 478085.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 472803ns | 471112ns | 456349ns | 471854ns | 492107ns | base |
| bitpack-footprint-dense-alt | 479006ns | 478945ns | 460952ns | 479362ns | 495992ns | +1.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 471885ns | 455568ns | 490720ns | base | 8.888 |
| bitpack-footprint-dense-alt | 478147ns | 460134ns | 495074ns | +1.33% | 8.772 |

## Performance model

- Peak throughput: **9.207 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 8.917 | 96.9% |
| bitpack-footprint-dense-alt | 8.773 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 472803ns | 472803ns | base |
| bitpack-footprint-dense-alt | 479006ns | 479006ns | +1.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 470367ns | base | --- | [466149, 476765] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 478086ns | no significant difference | [-3880, +14624]ns | [475825, 481477] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|
| 1 | 496999ns | -3.9% |
| 2 | 486483ns | -1.3% |
| 3 | 483607ns | +1.6% |
| 4 | 470868ns | +1.3% |
| 5 | 471810ns | +3.0% |
| 6 | 466385ns | +6.1% |
| 7 | 474125ns | +4.0% |
| 8 | 470514ns | +3.7% |
| 9 | 469540ns | +4.7% |
| 10 | 462844ns | +3.2% |
| 11 | 475905ns | -2.9% |
| 12 | 477626ns | -3.1% |
| 13 | 487681ns | -3.3% |
| 14 | 469644ns | -2.6% |
| 15 | 485905ns | -7.0% |
| 16 | 478542ns | -2.5% |
| 17 | 460243ns | +3.0% |
| 18 | 459683ns | -0.8% |
| 19 | 480846ns | -0.9% |
| 20 | 495728ns | -5.7% |
| 21 | 450119ns | +8.3% |
| 22 | 462071ns | +4.7% |
| 23 | 464345ns | +2.8% |
| 24 | 455069ns | +3.2% |
| 25 | 460500ns | +4.4% |
| 26 | 465912ns | +3.1% |
| 27 | 478798ns | -0.2% |
| 28 | 473812ns | +1.1% |
| 29 | 466575ns | +3.2% |
| 30 | 479321ns | -0.9% |
| 31 | 459952ns | +0.7% |
| 32 | 500797ns | -4.5% |
| 33 | 486846ns | -2.7% |
| 34 | 485319ns | -0.8% |
| 35 | 470219ns | +5.6% |
| 36 | 462061ns | +4.3% |
| 37 | 452738ns | +9.6% |
| 38 | 446717ns | +11.2% |
| 39 | 460024ns | +8.7% |
| 40 | 469210ns | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.324 | moderate+ |
| bitpack-footprint-dense-alt | 0.477 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 17/40, lost 23/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 19.9ns | 471884.6ns | 0.0% |  |
| bitpack-footprint-dense-alt | 19.8ns | 478147.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 455568.2-490719.8 ns)
  455568.2 |
  457325.8 |
  459083.3 |########################################
  460840.9 |################
  462598.5 |################
  464356.1 |########
  466113.7 |################
  467871.2 |################
  469628.8 |################################
  471386.4 |########
  473144.0 |################
  474901.6 |########
  476659.1 |########
  478416.7 |########################
  480174.3 |########
  481931.9 |########
  483689.5 |########
  485447.0 |########################
  487204.6 |########
  488962.2 |
  (4 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 460134.2-495074.0 ns)
  460134.2 |########
  461881.2 |########################
  463628.2 |
  465375.1 |########
  467122.1 |########
  468869.1 |########
  470616.1 |########
  472363.1 |################
  474110.1 |########
  475857.1 |########################
  477604.1 |########################################
  479351.1 |########################
  481098.1 |########################
  482845.1 |########
  484592.0 |########
  486339.0 |########
  488086.0 |########
  489833.0 |################
  491580.0 |########
  493327.0 |########
  (3 below, 4 above range)

```
