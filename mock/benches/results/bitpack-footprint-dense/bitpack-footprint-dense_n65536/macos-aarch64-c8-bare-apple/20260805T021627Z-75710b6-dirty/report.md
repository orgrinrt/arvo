# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (6.76 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 6.72 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-footprint-dense shows warm-up / thermal drift (autocorr +0.85)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (45 ns) is smaller than the fastest variant's own run-to-run std-dev (303 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bitpack-footprint-dense-alt's edge over baseline is significant but tiny (-2 ns, 0.03%)

bitpack-footprint-dense-alt differs from baseline bitpack-footprint-dense by -2 ns (0.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 6717.5 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 6717.5 ns, slowest 6762.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 7000ns | 6878ns | 6578ns | 6983ns | 7473ns | base |
| bitpack-footprint-dense-alt | 7009ns | 6857ns | 6825ns | 6881ns | 7576ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 6882ns | 6474ns | 7341ns | base | 9.523 |
| bitpack-footprint-dense-alt | 6885ns | 6715ns | 7448ns | +0.05% | 9.519 |

## Performance model

- Peak throughput: **10.123 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 9.691 | 95.7% |
| bitpack-footprint-dense-alt | 9.756 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 7000ns | 7000ns | base |
| bitpack-footprint-dense-alt | 7009ns | 7009ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 6762ns | base | --- | [6718, 6856] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 6718ns | -2.7ns (-0.0%) | [-47, -0]ns | [6716, 6742] | YES (adj: no) | 0.0533 | 0.0533 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|
| 1 | 7319ns | -0.0% |
| 2 | 7304ns | -3.6% |
| 3 | 7300ns | -8.0% |
| 4 | 7305ns | -8.0% |
| 5 | 7294ns | -7.9% |
| 6 | 7302ns | -8.0% |
| 7 | 7303ns | -8.1% |
| 8 | 7312ns | -8.1% |
| 9 | 7301ns | -4.6% |
| 10 | 7296ns | -0.0% |
| 11 | 7326ns | -0.5% |
| 12 | 7290ns | +0.0% |
| 13 | 7560ns | -3.3% |
| 14 | 6740ns | +12.7% |
| 15 | 6713ns | +14.1% |
| 16 | 6718ns | +16.4% |
| 17 | 6713ns | +0.1% |
| 18 | 6780ns | -0.2% |
| 19 | 6716ns | -0.0% |
| 20 | 6717ns | -0.0% |
| 21 | 6762ns | -0.7% |
| 22 | 6769ns | -0.8% |
| 23 | 6765ns | -0.7% |
| 24 | 6929ns | -3.1% |
| 25 | 6718ns | -0.0% |
| 26 | 6771ns | -0.8% |
| 27 | 6718ns | -0.0% |
| 28 | 6746ns | -0.5% |
| 29 | 6712ns | +0.9% |
| 30 | 6718ns | +0.0% |
| 31 | 6280ns | +6.9% |
| 32 | 6281ns | +6.9% |
| 33 | 6282ns | +6.9% |
| 34 | 6280ns | +9.5% |
| 35 | 6528ns | +2.9% |
| 36 | 6718ns | -0.0% |
| 37 | 6716ns | +0.0% |
| 38 | 6784ns | -1.1% |
| 39 | 6714ns | +0.8% |
| 40 | 6763ns | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.855 | HIGH+ (drift/warm-up) |
| bitpack-footprint-dense-alt | 0.732 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 19/40, lost 10/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 2.2ns | 6881.6ns | 0.0% |  |
| bitpack-footprint-dense-alt | 2.5ns | 6884.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 6473.7-7341.4 ns)
   6473.7 |
   6517.1 |###
   6560.5 |
   6603.8 |
   6647.2 |
   6690.6 |########################################
   6734.0 |#######################
   6777.4 |######
   6820.8 |
   6864.2 |
   6907.5 |###
   6950.9 |
   6994.3 |
   7037.7 |
   7081.1 |
   7124.5 |
   7167.9 |
   7211.2 |
   7254.6 |##########
   7298.0 |##############################
  (4 below, 1 above range)

bitpack-footprint-dense-alt (n=40, range 6714.5-7447.9 ns)
   6714.5 |########################################
   6751.2 |#####
   6787.9 |
   6824.5 |
   6861.2 |#
   6897.9 |
   6934.5 |#
   6971.2 |
   7007.9 |#
   7044.5 |
   7081.2 |
   7117.9 |
   7154.5 |
   7191.2 |
   7227.9 |
   7264.5 |#####
   7301.2 |###
   7337.9 |
   7374.5 |
   7411.2 |
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **bitpack-footprint-dense-alt**: autocorrelation=0.73 (measurement drift or warm-up artifact)
