# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (11.69 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 6.96 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 68% faster than the next best (quantiser-radix10)

quantiser-radix2 (6.96 us) leads quantiser-radix10 (11.69 us) by 68%, a clear separation rather than a photo finish. CV 33.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 39% (significant)

quantiser-radix2 is -4.60 us (39%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 33.4%)

quantiser-radix2 wins on median (6.96 us) yet has the highest variance (CV 33.4%), while quantiser-radix10 is the steadiest (CV 4.5%, 11.69 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix2 is inconsistent: worst-20% is 1.7x its best-20%

quantiser-radix2's best 20% of batches run at 6.57 us but its worst 20% at 10.89 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 6960.8 ns median (-40.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.68x (fastest 6960.8 ns, slowest 11686.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 11875ns | 11764ns | 11267ns | 11807ns | 12689ns | base |
| quantiser-radix2 | 7787ns | 7048ns | 6649ns | 7087ns | 11024ns | -34.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 11783ns | 11194ns | 12549ns | base | 0.022 |
| quantiser-radix2 | 7694ns | 6570ns | 10889ns | -34.70% | 0.033 |

## Performance model

- Peak throughput: **0.039 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.022 | 56.2% |
| quantiser-radix2 | 0.037 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 11875ns | 11875ns | base |
| quantiser-radix2 | 7787ns | 7787ns | -34.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 11686ns | base | --- | [11534, 11950] | --- | --- | --- | --- |
| quantiser-radix2 | 6961ns | -4628.9ns (-39.6%) | [-4979, -4307]ns | [6895, 7092] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 12021ns | -44.1% |
| 2 | 12007ns | -44.0% |
| 3 | 12060ns | -44.3% |
| 4 | 12025ns | -42.2% |
| 5 | 12202ns | -39.6% |
| 6 | 11984ns | -37.3% |
| 7 | 12199ns | -42.3% |
| 8 | 12106ns | -42.5% |
| 9 | 11915ns | -41.6% |
| 10 | 11915ns | -41.9% |
| 11 | 13545ns | -14.6% |
| 12 | 12903ns | -40.7% |
| 13 | 11688ns | -43.8% |
| 14 | 12645ns | -27.8% |
| 15 | 11619ns | -44.4% |
| 16 | 11622ns | -45.0% |
| 17 | 12063ns | -47.2% |
| 18 | 11686ns | -11.8% |
| 19 | 11802ns | +22.8% |
| 20 | 11687ns | +61.2% |
| 21 | 11565ns | -39.9% |
| 22 | 11578ns | -42.9% |
| 23 | 11521ns | -38.3% |
| 24 | 11772ns | -41.6% |
| 25 | 12730ns | -46.0% |
| 26 | 11513ns | -40.2% |
| 27 | 11520ns | -40.2% |
| 28 | 12009ns | -42.7% |
| 29 | 11533ns | -39.9% |
| 30 | 11534ns | -40.1% |
| 31 | 10472ns | -34.5% |
| 32 | 10858ns | -29.8% |
| 33 | 11370ns | -37.6% |
| 34 | 11365ns | -37.2% |
| 35 | 11401ns | -37.8% |
| 36 | 11356ns | -37.8% |
| 37 | 11369ns | -37.8% |
| 38 | 11369ns | -37.7% |
| 39 | 11397ns | -37.6% |
| 40 | 11397ns | -37.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.480 | moderate+ |
| quantiser-radix2 | 0.426 | moderate+ |

**Consistency summary:**

- **quantiser-radix2**: won 38/40, lost 2/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.5ns | 11783.1ns | 0.0% |  |
| quantiser-radix2 | 2.9ns | 7694.2ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 11194.5-12549.3 ns)
  11194.5 |
  11262.2 |
  11330.0 |########################################
  11397.7 |#####
  11465.4 |#################
  11533.2 |######################
  11600.9 |###########
  11668.7 |#################
  11736.4 |###########
  11804.1 |
  11871.9 |###########
  11939.6 |###########
  12007.4 |############################
  12075.1 |#####
  12142.8 |###########
  12210.6 |
  12278.3 |
  12346.0 |
  12413.8 |
  12481.5 |
  (2 below, 4 above range)

quantiser-radix2 (n=40, range 6570.2-10889.2 ns)
   6570.2 |############
   6786.1 |########################################
   7002.1 |##############################
   7218.0 |###
   7434.0 |######
   7649.9 |###
   7865.9 |
   8081.8 |
   8297.8 |
   8513.7 |
   8729.7 |
   8945.6 |###
   9161.6 |
   9377.5 |
   9593.5 |
   9809.5 |
  10025.4 |
  10241.4 |###
  10457.3 |
  10673.3 |
  (4 below, 3 above range)

```

## Diagnostics

- **quantiser-radix2**: CV=30.2% (high variance, measurements may be unstable)
