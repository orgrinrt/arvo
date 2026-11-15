# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (682.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 653.79 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-footprint-dense-alt is fastest but the noisiest (CV 10.7%)

bitpack-footprint-dense-alt wins on median (653.79 us) yet has the highest variance (CV 10.7%), while bitpack-footprint-dense is the steadiest (CV 5.2%, 682.70 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (28.91 us) is smaller than the fastest variant's own run-to-run std-dev (70.21 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bitpack-footprint-dense-alt vs stability leader bitpack-footprint-dense (+4% speed for 2.1x steadier)

bitpack-footprint-dense-alt is fastest (653.79 us, CV 10.7%); bitpack-footprint-dense gives up 4.4% median for 2.1x lower variance (CV 5.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 653789.9 ns median (-4.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.04x (fastest 653789.9 ns, slowest 682701.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 695381ns | 682956ns | 677189ns | 686098ns | 741422ns | base |
| bitpack-footprint-dense-alt | 687285ns | 654040ns | 638060ns | 659416ns | 820117ns | -1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 695126ns | 676970ns | 741140ns | base | 6.034 |
| bitpack-footprint-dense-alt | 687054ns | 637875ns | 819849ns | -1.16% | 6.105 |

## Performance model

- Peak throughput: **6.575 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 6.144 | 93.4% |
| bitpack-footprint-dense-alt | 6.415 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 695381ns | 695381ns | base |
| bitpack-footprint-dense-alt | 687285ns | 687285ns | -1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 682702ns | base | --- | [679098, 690743] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 653790ns | -43873.9ns (-6.4%) | [-44787, -8273]ns | [638187, 669424] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|---|
| 1 | 1 | 677611ns | -1.2% |
| 1 | 2 | 677455ns | +20.3% |
| 1 | 3 | 682744ns | -6.5% |
| 1 | 4 | 701804ns | -9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.085 | ok |
| bitpack-footprint-dense-alt | -0.297 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 50.6ns | 695125.9ns | 0.0% |  |
| bitpack-footprint-dense-alt | 45.2ns | 687053.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 676970.3-741140.3 ns)
  676970.3 |########################################
  680178.8 |################################
  683387.3 |###
  686595.8 |
  689804.3 |
  693012.8 |
  696221.3 |###
  699429.8 |#############################
  702638.3 |###
  705846.8 |###
  709055.3 |
  712263.8 |
  715472.3 |
  718680.8 |
  721889.3 |
  725097.8 |
  728306.3 |###
  731514.8 |
  734723.3 |
  737931.8 |
  (5 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 637875.1-819849.5 ns)
  637875.1 |########################################
  646973.9 |
  656072.6 |
  665171.3 |########################
  674270.0 |
  683368.7 |
  692467.4 |##
  701566.2 |
  710664.9 |##
  719763.6 |##
  728862.3 |
  737961.0 |
  747059.7 |
  756158.5 |
  765257.2 |
  774355.9 |
  783454.6 |##
  792553.3 |
  801652.0 |
  810750.8 |#############
  (5 below, 2 above range)

```
