# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (8.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 4.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 81% faster than the next best (quantiser-radix10)

quantiser-radix2 (4.47 us) leads quantiser-radix10 (8.07 us) by 81%, a clear separation rather than a photo finish. CV 69.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 43% (significant)

quantiser-radix2 is -3.47 us (43%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 69.7%)

quantiser-radix2 wins on median (4.47 us) yet has the highest variance (CV 69.7%), while quantiser-radix10 is the steadiest (CV 32.8%, 8.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix2 is inconsistent: worst-20% is 2.1x its best-20%

quantiser-radix2's best 20% of batches run at 4.09 us but its worst 20% at 8.53 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 4467.5 ns median (-44.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.81x (fastest 4467.5 ns, slowest 8065.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 8744ns | 8156ns | 7552ns | 8198ns | 11576ns | base |
| quantiser-radix2 | 5293ns | 4552ns | 4161ns | 4525ns | 8725ns | -39.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 8655ns | 7478ns | 11453ns | base | 0.030 |
| quantiser-radix2 | 5187ns | 4091ns | 8530ns | -40.07% | 0.049 |

## Performance model

- Peak throughput: **0.063 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.032 | 50.7% |
| quantiser-radix2 | 0.057 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 8744ns | 8744ns | base |
| quantiser-radix2 | 5293ns | 5293ns | -39.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 8066ns | base | --- | [7982, 8279] | --- | --- | --- | --- |
| quantiser-radix2 | 4468ns | -3474.6ns (-43.1%) | [-4062, -3401]ns | [4274, 4573] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 8304ns | -46.2% |
| 2 | 8320ns | -47.8% |
| 3 | 8263ns | -50.1% |
| 4 | 8286ns | -50.3% |
| 5 | 8281ns | -50.5% |
| 6 | 8272ns | -50.4% |
| 7 | 8277ns | -50.5% |
| 8 | 8332ns | -50.8% |
| 9 | 8320ns | -50.7% |
| 10 | 8325ns | -50.8% |
| 11 | 7925ns | +10.5% |
| 12 | 8076ns | +179.8% |
| 13 | 8139ns | -42.4% |
| 14 | 11304ns | -55.3% |
| 15 | 8292ns | -42.7% |
| 16 | 8008ns | -42.3% |
| 17 | 7960ns | +52.0% |
| 18 | 24371ns | -81.4% |
| 19 | 8053ns | -38.0% |
| 20 | 8079ns | -40.9% |
| 21 | 7321ns | -39.0% |
| 22 | 7359ns | -38.1% |
| 23 | 7345ns | -39.8% |
| 24 | 7329ns | -43.4% |
| 25 | 7347ns | -44.5% |
| 26 | 7342ns | -44.6% |
| 27 | 9380ns | -56.5% |
| 28 | 7955ns | -39.0% |
| 29 | 7894ns | -46.7% |
| 30 | 7888ns | -47.8% |
| 31 | 7989ns | -43.3% |
| 32 | 8056ns | -38.0% |
| 33 | 9549ns | -48.7% |
| 34 | 8005ns | -42.6% |
| 35 | 7968ns | -43.1% |
| 36 | 7975ns | -43.3% |
| 37 | 10848ns | -59.2% |
| 38 | 9514ns | -53.5% |
| 39 | 8044ns | -42.5% |
| 40 | 7907ns | -44.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | -0.034 | ok |
| quantiser-radix2 | 0.161 | ok |

**Consistency summary:**

- **quantiser-radix2**: won 37/40, lost 3/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 3.0ns | 8655.1ns | 0.0% |  |
| quantiser-radix2 | 2.0ns | 5186.8ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 7478.1-11453.0 ns)
   7478.1 |
   7676.9 |
   7875.6 |########################################
   8074.4 |##############
   8273.1 |#########################
   8471.8 |
   8670.6 |
   8869.3 |
   9068.1 |
   9266.8 |##
   9465.5 |#####
   9664.3 |
   9863.0 |
  10061.8 |
  10260.5 |
  10459.3 |
  10658.0 |##
  10856.7 |
  11055.5 |
  11254.2 |##
  (6 below, 1 above range)

quantiser-radix2 (n=40, range 4090.9-8530.2 ns)
   4090.9 |########################################
   4312.8 |########################################
   4534.8 |#####################
   4756.8 |##########
   4978.7 |##########
   5200.7 |
   5422.6 |
   5644.6 |
   5866.6 |
   6088.5 |
   6310.5 |
   6532.5 |
   6754.4 |
   6976.4 |
   7198.4 |
   7420.3 |
   7642.3 |
   7864.3 |
   8086.2 |
   8308.2 |
  (3 below, 3 above range)

```

## Diagnostics

- **quantiser-radix10**: CV=30.6% (high variance, measurements may be unstable)
- **quantiser-radix2**: CV=60.0% (high variance, measurements may be unstable)
