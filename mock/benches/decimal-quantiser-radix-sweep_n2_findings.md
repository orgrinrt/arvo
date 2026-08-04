# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (8.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 4.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 88% faster than the next best (quantiser-radix10)

quantiser-radix2 (4.34 us) leads quantiser-radix10 (8.15 us) by 88%, a clear separation rather than a photo finish. CV 8.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 46% (significant)

quantiser-radix2 is -3.73 us (46%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 8.4%)

quantiser-radix2 wins on median (4.34 us) yet has the highest variance (CV 8.4%), while quantiser-radix10 is the steadiest (CV 5.1%, 8.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix10 shows warm-up / thermal drift (autocorr +0.80)

quantiser-radix10's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: quantiser-radix2** at 4339.4 ns median (-46.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.88x (fastest 4339.4 ns, slowest 8154.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 8297ns | 8225ns | 7795ns | 8234ns | 8988ns | base |
| quantiser-radix2 | 4466ns | 4410ns | 4169ns | 4403ns | 4955ns | -46.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 8219ns | 7728ns | 8882ns | base | 0.031 |
| quantiser-radix2 | 4393ns | 4104ns | 4858ns | -46.55% | 0.058 |

## Performance model

- Peak throughput: **0.062 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.031 | 50.3% |
| quantiser-radix2 | 0.059 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 8297ns | 8297ns | base |
| quantiser-radix2 | 4466ns | 4466ns | -46.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 8155ns | base | --- | [8122, 8288] | --- | --- | --- | --- |
| quantiser-radix2 | 4339ns | -3788.6ns (-46.5%) | [-3887, -3693]ns | [4291, 4353] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 8691ns | -47.4% |
| 2 | 8693ns | -50.4% |
| 3 | 9344ns | -53.8% |
| 4 | 8886ns | -51.5% |
| 5 | 9002ns | -51.6% |
| 6 | 8176ns | -42.7% |
| 7 | 8216ns | -42.9% |
| 8 | 8264ns | -43.3% |
| 9 | 8198ns | -42.9% |
| 10 | 8716ns | -46.3% |
| 11 | 8948ns | -51.4% |
| 12 | 8772ns | -27.1% |
| 13 | 8206ns | -45.7% |
| 14 | 8295ns | -47.3% |
| 15 | 8369ns | -48.0% |
| 16 | 8280ns | -47.6% |
| 17 | 8387ns | -48.1% |
| 18 | 8535ns | -49.2% |
| 19 | 8457ns | -48.4% |
| 20 | 8523ns | -48.8% |
| 21 | 8118ns | -47.1% |
| 22 | 8131ns | -47.3% |
| 23 | 8134ns | -47.2% |
| 24 | 8130ns | -47.3% |
| 25 | 8129ns | -45.3% |
| 26 | 8127ns | -47.2% |
| 27 | 8134ns | -47.3% |
| 28 | 7903ns | -45.7% |
| 29 | 7602ns | -43.6% |
| 30 | 7597ns | -43.5% |
| 31 | 7798ns | -44.3% |
| 32 | 7767ns | -44.1% |
| 33 | 7782ns | -47.7% |
| 34 | 7773ns | -47.7% |
| 35 | 7765ns | -47.7% |
| 36 | 7776ns | -47.8% |
| 37 | 7765ns | -47.7% |
| 38 | 7812ns | -47.1% |
| 39 | 7776ns | -42.5% |
| 40 | 7786ns | -47.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.798 | HIGH+ (drift/warm-up) |
| quantiser-radix2 | 0.175 | ok |

**Consistency summary:**

- **quantiser-radix2**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.4ns | 8219.1ns | 0.0% |  |
| quantiser-radix2 | 2.2ns | 4392.8ns | 0.1% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 7727.6-8881.5 ns)
   7727.6 |########################################
   7785.3 |#################
   7843.0 |
   7900.7 |#####
   7958.4 |
   8016.1 |
   8073.8 |############################
   8131.5 |#################
   8189.2 |#################
   8246.9 |#################
   8304.5 |
   8362.2 |###########
   8419.9 |#####
   8477.6 |###########
   8535.3 |
   8593.0 |
   8650.7 |###########
   8708.4 |#####
   8766.1 |#####
   8823.8 |
  (2 below, 4 above range)

quantiser-radix2 (n=40, range 4104.2-4858.4 ns)
   4104.2 |###
   4141.9 |
   4179.6 |
   4217.3 |
   4255.0 |#############################
   4292.7 |##############
   4330.4 |########################################
   4368.2 |
   4405.9 |
   4443.6 |##########
   4481.3 |
   4519.0 |
   4556.7 |###
   4594.4 |
   4632.1 |
   4669.9 |##################
   4707.6 |
   4745.3 |
   4783.0 |
   4820.7 |
  (6 below, 1 above range)

```

## Diagnostics

- **quantiser-radix10**: autocorrelation=0.80 (measurement drift or warm-up artifact)
