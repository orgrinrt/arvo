# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-pipe4 dominates: 16% faster than the next best (bitpack-wide-d16-control)

bitpack-wide-pipe4 (458.64 us) leads bitpack-wide-d16-control (530.04 us) by 16%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-d16-padal is inconsistent: worst-20% is 4.0x its best-20%

bitpack-wide-d16-padal's best 20% of batches run at 530.61 us but its worst 20% at 2.11 ms (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 458641.1 ns median (-21.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.29x (fastest 458641.1 ns, slowest 592732.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112726093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 572635ns | 588003ns | 530202ns | 569133ns | 625575ns | base |
| bitpack-wide-d16-control | 554969ns | 530821ns | 525274ns | 544879ns | 614936ns | -3.09% |
| bitpack-wide-d16-padal | 885953ns | 593443ns | 531202ns | 596879ns | 2107924ns | +54.71% |
| bitpack-wide-pipe4 | 460919ns | 459197ns | 432428ns | 461241ns | 488447ns | -19.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 571853ns | 529536ns | 624517ns | base | 14.669 |
| bitpack-wide-d16-control | 554189ns | 524601ns | 614016ns | -3.09% | 15.137 |
| bitpack-wide-d16-padal | 885125ns | 530607ns | 2106314ns | +54.78% | 9.477 |
| bitpack-wide-pipe4 | 460343ns | 431835ns | 487740ns | -19.50% | 18.223 |

## Performance model

- Peak throughput: **19.425 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 14.283 | 73.5% |
| bitpack-wide-d16-control | 15.826 | 81.5% |
| bitpack-wide-d16-padal | 14.152 | 72.9% |
| bitpack-wide-pipe4 | 18.290 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 572635ns | 572635ns | base |
| bitpack-wide-d16-control | 554969ns | 554969ns | -3.09% |
| bitpack-wide-d16-padal | 885953ns | 885953ns | +54.71% |
| bitpack-wide-pipe4 | 460919ns | 460919ns | -19.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 587331ns | base | --- | [533557, 588608] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 530040ns | -5728.0ns (-1.0%) | [-27194, -4932]ns | [528310, 556027] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-wide-d16-padal | 592733ns | +16251.0ns (+2.8%) | [+4104, +26282]ns | [591582, 609521] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 458641ns | -115485.9ns (-19.7%) | [-128142, -75523]ns | [456100, 463160] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 590313ns | -10.4% | +3.3% | -21.5% |
| 1 | 2 | 530094ns | -1.0% | +8.9% | -13.5% |
| 1 | 3 | 546161ns | +8.5% | +73.5% | -16.7% |
| 1 | 4 | 587883ns | -4.6% | +0.7% | -27.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.269 | moderate- |
| bitpack-wide-d16-control | -0.040 | ok |
| bitpack-wide-d16-padal | -0.465 | moderate- |
| bitpack-wide-pipe4 | 0.039 | ok |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 3/4, lost 1/4
- **bitpack-wide-d16-padal**: won 0/4, lost 4/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 45.5ns | 571853.0ns | 0.0% |  |
| bitpack-wide-d16-control | 42.7ns | 554188.7ns | 0.0% |  |
| bitpack-wide-d16-padal | 74.6ns | 885124.6ns | 0.0% |  |
| bitpack-wide-pipe4 | 75.1ns | 460342.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 529536.4-624516.9 ns)
  529536.4 |########################################
  534285.4 |
  539034.5 |
  543783.5 |
  548532.5 |###
  553281.5 |
  558030.6 |######
  562779.6 |
  567528.6 |
  572277.6 |
  577026.7 |
  581775.7 |
  586524.7 |########################################
  591273.8 |
  596022.8 |###
  600771.8 |###
  605520.8 |###
  610269.9 |###
  615018.9 |###
  619767.9 |
  (2 below, 4 above range)

bitpack-wide-d16-control (n=40, range 524600.7-614015.9 ns)
  524600.7 |########################################
  529071.5 |####################
  533542.2 |###
  538013.0 |###
  542483.8 |
  546954.5 |
  551425.3 |
  555896.0 |
  560366.8 |
  564837.5 |
  569308.3 |###
  573779.0 |
  578249.8 |
  582720.6 |###
  587191.3 |###
  591662.1 |#############
  596132.8 |######
  600603.6 |###
  605074.3 |###
  609545.1 |###
  (6 below, 2 above range)

bitpack-wide-d16-padal (n=40, range 530607.3-2106314.5 ns)
  530607.3 |########################################
  609392.6 |###################
  688178.0 |
  766963.4 |
  845748.7 |
  924534.1 |
  1003319.4 |
  1082104.8 |
  1160890.2 |
  1239675.5 |
  1318460.9 |#
  1397246.2 |
  1476031.6 |
  1554817.0 |
  1633602.3 |
  1712387.7 |
  1791173.0 |
  1869958.4 |
  1948743.8 |
  2027529.1 |
  (4 below, 4 above range)

bitpack-wide-pipe4 (n=40, range 431835.0-487739.9 ns)
  431835.0 |
  434630.3 |#####
  437425.5 |
  440220.8 |
  443016.0 |
  445811.2 |
  448606.5 |
  451401.7 |
  454197.0 |########################################
  456992.2 |########################################
  459787.5 |#################
  462582.7 |##################################
  465377.9 |
  468173.2 |
  470968.4 |
  473763.7 |
  476558.9 |#####
  479354.2 |###########
  482149.4 |###########
  484944.6 |#####
  (6 below, 4 above range)

```

## Diagnostics

- **bitpack-wide-d16-padal**: CV=95.9% (high variance, measurements may be unstable)
- **bitpack-wide-d16-padal**: worst_20/best_20 = 4.0x (possible bimodal distribution)
