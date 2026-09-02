# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (10.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 5.74 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 87% faster than the next best (quantiser-radix10)

quantiser-radix2 (5.74 us) leads quantiser-radix10 (10.72 us) by 87%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 46% (significant)

quantiser-radix2 is -4.96 us (46%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix10 shows warm-up / thermal drift (autocorr +0.59)

quantiser-radix10's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### quantiser-radix10 is inconsistent: worst-20% is 1.7x its best-20%

quantiser-radix10's best 20% of batches run at 9.68 us but its worst 20% at 16.38 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 5737.7 ns median (-46.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.87x (fastest 5737.7 ns, slowest 10722.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 11838ns | 10810ns | 9746ns | 10976ns | 16514ns | base |
| quantiser-radix2 | 5882ns | 5809ns | 5614ns | 5791ns | 6426ns | -50.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 11746ns | 9679ns | 16382ns | base | 0.022 |
| quantiser-radix2 | 5795ns | 5536ns | 6286ns | -50.67% | 0.044 |

## Performance model

- Peak throughput: **0.046 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.024 | 51.6% |
| quantiser-radix2 | 0.045 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 11838ns | 11838ns | base |
| quantiser-radix2 | 5882ns | 5882ns | -50.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 10722ns | base | --- | [10641, 11137] | --- | --- | --- | --- |
| quantiser-radix2 | 5738ns | -4954.4ns (-46.2%) | [-5343, -4887]ns | [5667, 5753] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 18853ns | -70.5% |
| 2 | 17628ns | -66.2% |
| 3 | 11236ns | -41.0% |
| 4 | 10550ns | -28.6% |
| 5 | 10918ns | -48.7% |
| 6 | 11990ns | -53.3% |
| 7 | 24311ns | -77.0% |
| 8 | 19779ns | -72.6% |
| 9 | 14803ns | -63.7% |
| 10 | 11839ns | -53.5% |
| 11 | 11565ns | -51.0% |
| 12 | 11852ns | -50.3% |
| 13 | 11464ns | -50.6% |
| 14 | 11506ns | -50.7% |
| 15 | 11491ns | -50.7% |
| 16 | 10795ns | -47.5% |
| 17 | 11037ns | -48.7% |
| 18 | 10498ns | -46.0% |
| 19 | 10491ns | -46.0% |
| 20 | 10629ns | -46.7% |
| 21 | 10740ns | -42.1% |
| 22 | 10640ns | -45.8% |
| 23 | 10851ns | -47.1% |
| 24 | 10641ns | -46.0% |
| 25 | 10689ns | -46.2% |
| 26 | 10645ns | -46.1% |
| 27 | 10643ns | -46.1% |
| 28 | 10640ns | -46.1% |
| 29 | 11594ns | -50.5% |
| 30 | 10754ns | -47.7% |
| 31 | 9665ns | -40.4% |
| 32 | 9669ns | -40.4% |
| 33 | 9667ns | -40.4% |
| 34 | 9670ns | -40.4% |
| 35 | 9694ns | -38.2% |
| 36 | 9686ns | -40.5% |
| 37 | 9668ns | -40.1% |
| 38 | 9712ns | -35.5% |
| 39 | 10705ns | -46.5% |
| 40 | 10642ns | -46.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.587 | HIGH+ (drift/warm-up) |
| quantiser-radix2 | 0.339 | moderate+ |

**Consistency summary:**

- **quantiser-radix2**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.8ns | 11746.3ns | 0.0% |  |
| quantiser-radix2 | 2.5ns | 5794.5ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 9679.1-16381.9 ns)
   9679.1 |############
  10014.2 |
  10349.3 |########################################
  10684.5 |############################
  11019.6 |########
  11354.8 |####################
  11689.9 |############
  12025.0 |
  12360.2 |
  12695.3 |
  13030.5 |
  13365.6 |
  13700.8 |
  14035.9 |
  14371.0 |
  14706.2 |####
  15041.3 |
  15376.5 |
  15711.6 |
  16046.7 |
  (5 below, 4 above range)

quantiser-radix2 (n=40, range 5536.1-6286.3 ns)
   5536.1 |####
   5573.6 |#############
   5611.1 |####
   5648.6 |########################################
   5686.1 |####
   5723.7 |########################################
   5761.2 |##########################
   5798.7 |
   5836.2 |
   5873.7 |####
   5911.2 |
   5948.7 |####
   5986.2 |####
   6023.8 |
   6061.3 |
   6098.8 |
   6136.3 |
   6173.8 |
   6211.3 |####
   6248.8 |####
  (3 below, 2 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=26.0% (high variance, measurements may be unstable)
- **quantiser-radix10**: autocorrelation=0.59 (measurement drift or warm-up artifact)
