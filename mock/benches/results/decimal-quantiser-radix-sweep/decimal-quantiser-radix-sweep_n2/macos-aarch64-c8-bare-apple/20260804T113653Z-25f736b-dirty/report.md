# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (9.13 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 5.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 77% faster than the next best (quantiser-radix10)

quantiser-radix2 (5.16 us) leads quantiser-radix10 (9.13 us) by 77%, a clear separation rather than a photo finish. CV 48.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 46% (significant)

quantiser-radix2 is -4.16 us (46%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 48.4%)

quantiser-radix2 wins on median (5.16 us) yet has the highest variance (CV 48.4%), while quantiser-radix10 is the steadiest (CV 6.8%, 9.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix2 is inconsistent: worst-20% is 1.6x its best-20%

quantiser-radix2's best 20% of batches run at 4.69 us but its worst 20% at 7.68 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: quantiser-radix2** at 5164.6 ns median (-43.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.77x (fastest 5164.6 ns, slowest 9128.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 9379ns | 9210ns | 8830ns | 9249ns | 10317ns | base |
| quantiser-radix2 | 5627ns | 5248ns | 4761ns | 5187ns | 7810ns | -40.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 9294ns | 8755ns | 10215ns | base | 0.028 |
| quantiser-radix2 | 5536ns | 4686ns | 7685ns | -40.43% | 0.046 |

## Performance model

- Peak throughput: **0.055 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.028 | 51.3% |
| quantiser-radix2 | 0.050 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 9379ns | 9379ns | base |
| quantiser-radix2 | 5627ns | 5627ns | -40.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 9129ns | base | --- | [8981, 9379] | --- | --- | --- | --- |
| quantiser-radix2 | 5165ns | -4159.0ns (-45.6%) | [-4354, -3750]ns | [5090, 5207] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 8843ns | -41.0% |
| 2 | 9410ns | -44.7% |
| 3 | 8804ns | -40.0% |
| 4 | 9360ns | -38.6% |
| 5 | 9458ns | -40.6% |
| 6 | 11913ns | -56.4% |
| 7 | 8749ns | -40.9% |
| 8 | 8708ns | -40.7% |
| 9 | 8703ns | -40.9% |
| 10 | 8702ns | -41.1% |
| 11 | 8988ns | -47.2% |
| 12 | 10801ns | -56.1% |
| 13 | 9031ns | -47.7% |
| 14 | 9038ns | -38.6% |
| 15 | 8991ns | -17.1% |
| 16 | 8975ns | -41.2% |
| 17 | 9034ns | -47.8% |
| 18 | 9280ns | +125.0% |
| 19 | 9948ns | -51.5% |
| 20 | 10087ns | -52.5% |
| 21 | 9188ns | -49.3% |
| 22 | 8805ns | -47.2% |
| 23 | 8797ns | -47.0% |
| 24 | 9070ns | -48.6% |
| 25 | 8790ns | -46.7% |
| 26 | 8788ns | -42.0% |
| 27 | 8869ns | -42.6% |
| 28 | 9253ns | -45.0% |
| 29 | 8832ns | -38.6% |
| 30 | 8817ns | -42.2% |
| 31 | 9238ns | -48.7% |
| 32 | 9399ns | -46.3% |
| 33 | 9331ns | -44.3% |
| 34 | 9702ns | -46.2% |
| 35 | 10057ns | -45.1% |
| 36 | 9601ns | -45.3% |
| 37 | 9588ns | -45.7% |
| 38 | 9591ns | -45.6% |
| 39 | 9599ns | -45.9% |
| 40 | 9609ns | -46.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.140 | ok |
| quantiser-radix2 | -0.059 | ok |

**Consistency summary:**

- **quantiser-radix2**: won 39/40, lost 1/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.6ns | 9293.7ns | 0.0% |  |
| quantiser-radix2 | 2.6ns | 5536.2ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 8755.2-10214.9 ns)
   8755.2 |########################################
   8828.2 |####################
   8901.2 |
   8974.2 |########################################
   9047.1 |######
   9120.1 |######
   9193.1 |#############
   9266.1 |#############
   9339.1 |####################
   9412.1 |######
   9485.0 |
   9558.0 |#################################
   9631.0 |######
   9704.0 |
   9777.0 |
   9850.0 |
   9923.0 |######
   9995.9 |######
  10068.9 |######
  10141.9 |
  (4 below, 2 above range)

quantiser-radix2 (n=40, range 4686.1-7684.5 ns)
   4686.1 |##################
   4836.0 |
   4986.0 |################
   5135.9 |########################################
   5285.8 |##
   5435.7 |#####
   5585.6 |##
   5735.6 |##
   5885.5 |
   6035.4 |
   6185.3 |
   6335.2 |
   6485.2 |
   6635.1 |
   6785.0 |
   6934.9 |
   7084.8 |
   7234.8 |
   7384.7 |##
   7534.6 |
  (5 below, 1 above range)

```

## Diagnostics

- **quantiser-radix2**: CV=45.2% (high variance, measurements may be unstable)
