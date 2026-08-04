# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (7.42 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 4.14 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 79% faster than the next best (quantiser-radix10)

quantiser-radix2 (4.14 us) leads quantiser-radix10 (7.42 us) by 79%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 44% (significant)

quantiser-radix2 is -3.25 us (44%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix10 shows warm-up / thermal drift (autocorr +0.64)

quantiser-radix10's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: quantiser-radix2** at 4138.4 ns median (-44.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.79x (fastest 4138.4 ns, slowest 7417.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 7879ns | 7488ns | 7135ns | 7487ns | 9798ns | base |
| quantiser-radix2 | 4222ns | 4207ns | 4141ns | 4196ns | 4383ns | -46.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 7790ns | 7062ns | 9646ns | base | 0.033 |
| quantiser-radix2 | 4145ns | 4050ns | 4301ns | -46.79% | 0.062 |

## Performance model

- Peak throughput: **0.063 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.035 | 54.6% |
| quantiser-radix2 | 0.062 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 7879ns | 7879ns | base |
| quantiser-radix2 | 4222ns | 4222ns | -46.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 7417ns | base | --- | [7294, 7532] | --- | --- | --- | --- |
| quantiser-radix2 | 4138ns | -3293.3ns (-44.4%) | [-3390, -3218]ns | [4106, 4140] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 7528ns | -45.0% |
| 2 | 7534ns | -45.1% |
| 3 | 7535ns | -45.1% |
| 4 | 7531ns | -45.0% |
| 5 | 7525ns | -45.0% |
| 6 | 7536ns | -45.1% |
| 7 | 7560ns | -45.2% |
| 8 | 7556ns | -43.6% |
| 9 | 7560ns | -44.7% |
| 10 | 7530ns | -44.5% |
| 11 | 6878ns | -39.8% |
| 12 | 6868ns | -39.8% |
| 13 | 6882ns | -39.8% |
| 14 | 6902ns | -40.0% |
| 15 | 11560ns | -64.2% |
| 16 | 13085ns | -68.0% |
| 17 | 12228ns | -66.2% |
| 18 | 7434ns | -44.3% |
| 19 | 7903ns | -47.6% |
| 20 | 8656ns | -52.2% |
| 21 | 7326ns | -43.9% |
| 22 | 7947ns | -42.9% |
| 23 | 7811ns | -48.1% |
| 24 | 7981ns | -49.2% |
| 25 | 7442ns | -45.5% |
| 26 | 7356ns | -44.8% |
| 27 | 7401ns | -45.4% |
| 28 | 7330ns | -44.9% |
| 29 | 7278ns | -44.4% |
| 30 | 7280ns | -44.5% |
| 31 | 7238ns | -42.9% |
| 32 | 7238ns | -43.3% |
| 33 | 7233ns | -43.2% |
| 34 | 7277ns | -43.9% |
| 35 | 7292ns | -44.0% |
| 36 | 7298ns | -44.1% |
| 37 | 7262ns | -43.8% |
| 38 | 7256ns | -42.1% |
| 39 | 7295ns | -39.3% |
| 40 | 7263ns | -39.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.635 | HIGH+ (drift/warm-up) |
| quantiser-radix2 | 0.300 | moderate+ |

**Consistency summary:**

- **quantiser-radix2**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.6ns | 7789.9ns | 0.0% |  |
| quantiser-radix2 | 2.1ns | 4145.2ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 7062.1-9646.3 ns)
   7062.1 |
   7191.3 |########################################
   7320.5 |####################
   7449.7 |#################################
   7578.9 |
   7708.1 |###
   7837.3 |######
   7966.5 |###
   8095.8 |
   8225.0 |
   8354.2 |
   8483.4 |
   8612.6 |###
   8741.8 |
   8871.0 |
   9000.2 |
   9129.4 |
   9258.7 |
   9387.9 |
   9517.1 |
  (4 below, 3 above range)

quantiser-radix2 (n=40, range 4050.2-4300.6 ns)
   4050.2 |#########
   4062.7 |###
   4075.2 |############
   4087.7 |
   4100.3 |#########
   4112.8 |
   4125.3 |############
   4137.8 |########################################
   4150.3 |
   4162.9 |
   4175.4 |######
   4187.9 |######
   4200.4 |
   4212.9 |
   4225.4 |
   4238.0 |
   4250.5 |###
   4263.0 |
   4275.5 |
   4288.0 |
  (4 below, 3 above range)

```

## Diagnostics

- **quantiser-radix10**: autocorrelation=0.64 (measurement drift or warm-up artifact)
