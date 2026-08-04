# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (9.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 5.26 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 82% faster than the next best (quantiser-radix10)

quantiser-radix2 (5.26 us) leads quantiser-radix10 (9.57 us) by 82%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 45% (significant)

quantiser-radix2 is -4.28 us (45%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: quantiser-radix2** at 5256.7 ns median (-45.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.82x (fastest 5256.7 ns, slowest 9573.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 9695ns | 9645ns | 9203ns | 9629ns | 10384ns | base |
| quantiser-radix2 | 5244ns | 5325ns | 4978ns | 5256ns | 5474ns | -45.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 9618ns | 9135ns | 10278ns | base | 0.027 |
| quantiser-radix2 | 5170ns | 4905ns | 5392ns | -46.24% | 0.050 |

## Performance model

- Peak throughput: **0.052 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.027 | 51.2% |
| quantiser-radix2 | 0.049 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 9695ns | 9695ns | base |
| quantiser-radix2 | 5244ns | 5244ns | -45.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 9574ns | base | --- | [9454, 9610] | --- | --- | --- | --- |
| quantiser-radix2 | 5257ns | -4381.2ns (-45.8%) | [-4455, -4340]ns | [5035, 5260] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 9456ns | -46.5% |
| 2 | 9458ns | -47.0% |
| 3 | 9455ns | -41.5% |
| 4 | 10475ns | -52.2% |
| 5 | 9370ns | -46.6% |
| 6 | 10310ns | -51.4% |
| 7 | 9512ns | -47.3% |
| 8 | 9520ns | -47.4% |
| 9 | 9557ns | -47.6% |
| 10 | 10628ns | -51.9% |
| 11 | 9089ns | -46.4% |
| 12 | 9608ns | -48.5% |
| 13 | 9390ns | -48.3% |
| 14 | 10423ns | -53.5% |
| 15 | 9034ns | -41.3% |
| 16 | 9050ns | -44.4% |
| 17 | 9328ns | -47.8% |
| 18 | 9071ns | -42.2% |
| 19 | 9069ns | -46.7% |
| 20 | 9068ns | -44.5% |
| 21 | 10041ns | -44.3% |
| 22 | 10065ns | -47.1% |
| 23 | 10038ns | -47.0% |
| 24 | 10043ns | -47.1% |
| 25 | 10240ns | -48.1% |
| 26 | 9453ns | -43.7% |
| 27 | 9438ns | -43.7% |
| 28 | 9430ns | -43.6% |
| 29 | 9431ns | -42.7% |
| 30 | 9434ns | -43.6% |
| 31 | 9590ns | -45.2% |
| 32 | 9666ns | -45.6% |
| 33 | 9612ns | -45.3% |
| 34 | 9609ns | -45.3% |
| 35 | 9610ns | -45.3% |
| 36 | 9662ns | -45.6% |
| 37 | 9609ns | -45.3% |
| 38 | 9609ns | -44.5% |
| 39 | 9610ns | -45.3% |
| 40 | 9662ns | -45.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.017 | ok |
| quantiser-radix2 | 0.404 | moderate+ |

**Consistency summary:**

- **quantiser-radix2**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.5ns | 9618.1ns | 0.0% |  |
| quantiser-radix2 | 2.2ns | 5170.3ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 9134.8-10278.2 ns)
   9134.8 |
   9192.0 |
   9249.1 |
   9306.3 |#####
   9363.5 |##########
   9420.6 |########################################
   9477.8 |##########
   9535.0 |##########
   9592.1 |###################################
   9649.3 |###############
   9706.5 |
   9763.6 |
   9820.8 |
   9878.0 |
   9935.2 |
   9992.3 |###############
  10049.5 |#####
  10106.7 |
  10163.8 |
  10221.0 |#####
  (6 below, 4 above range)

quantiser-radix2 (n=40, range 4905.2-5392.5 ns)
   4905.2 |
   4929.6 |####
   4953.9 |
   4978.3 |
   5002.7 |###############################
   5027.0 |########
   5051.4 |####
   5075.8 |
   5100.1 |####
   5124.5 |
   5148.9 |
   5173.2 |
   5197.6 |
   5221.9 |####
   5246.3 |########################################
   5270.7 |
   5295.0 |######################
   5319.4 |######################
   5343.8 |
   5368.1 |
  (5 below, 3 above range)

```
