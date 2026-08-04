# Quantiser cost by radix: decimal32 against binary32, exponent spread swept

2 variants, 40 samples per variant.
Baseline: **quantiser-radix10**

## Highlights

Baseline for all deltas below: **quantiser-radix10**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (quantiser-radix10) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline quantiser-radix10 has the worst median (10.68 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest quantiser-radix2 at 6.24 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### quantiser-radix2 dominates: 71% faster than the next best (quantiser-radix10)

quantiser-radix2 (6.24 us) leads quantiser-radix10 (10.68 us) by 71%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-radix2 beats baseline by 42% (significant)

quantiser-radix2 is -4.54 us (42%) faster than baseline quantiser-radix10, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### quantiser-radix2 is fastest but the noisiest (CV 5.7%)

quantiser-radix2 wins on median (6.24 us) yet has the highest variance (CV 5.7%), while quantiser-radix10 is the steadiest (CV 3.9%, 10.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### quantiser-radix10 shows warm-up / thermal drift (autocorr +0.73)

quantiser-radix10's per-pass series has lag-1 autocorrelation +0.73, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: quantiser-radix2** at 6242.3 ns median (-41.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.71x (fastest 6242.3 ns, slowest 10679.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-radix10 | 10844ns | 10751ns | 10253ns | 10820ns | 11504ns | base |
| quantiser-radix2 | 6322ns | 6312ns | 5841ns | 6308ns | 6842ns | -41.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-radix10 | 10765ns | 10188ns | 11421ns | base | 0.024 |
| quantiser-radix2 | 6242ns | 5769ns | 6731ns | -42.02% | 0.041 |

## Performance model

- Peak throughput: **0.044 Gops/s** (quantiser-radix2; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-radix10 | 0.024 | 54.0% |
| quantiser-radix2 | 0.041 | 92.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-radix10 | 10844ns | 10844ns | base |
| quantiser-radix2 | 6322ns | 6322ns | -41.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-radix10 | 10680ns | base | --- | [10638, 10836] | --- | --- | --- | --- |
| quantiser-radix2 | 6242ns | -4435.6ns (-41.5%) | [-4598, -4392]ns | [6227, 6245] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | quantiser-radix10 | quantiser-radix2 |
|---|---|---|
| 1 | 10092ns | -38.1% |
| 2 | 10094ns | -38.1% |
| 3 | 10616ns | -41.2% |
| 4 | 10682ns | -40.9% |
| 5 | 10120ns | -38.3% |
| 6 | 10175ns | -38.6% |
| 7 | 10081ns | -38.1% |
| 8 | 10084ns | -38.1% |
| 9 | 10576ns | -41.0% |
| 10 | 10783ns | -42.1% |
| 11 | 11078ns | -46.1% |
| 12 | 10815ns | -44.7% |
| 13 | 11510ns | -48.0% |
| 14 | 11321ns | -45.9% |
| 15 | 10966ns | -47.6% |
| 16 | 10903ns | -48.1% |
| 17 | 10683ns | -47.3% |
| 18 | 11103ns | -43.1% |
| 19 | 10772ns | -48.2% |
| 20 | 10359ns | -46.1% |
| 21 | 10677ns | -41.3% |
| 22 | 10640ns | -41.5% |
| 23 | 10638ns | -41.5% |
| 24 | 10652ns | -41.5% |
| 25 | 10672ns | -41.7% |
| 26 | 10858ns | -42.6% |
| 27 | 10725ns | -41.9% |
| 28 | 10638ns | -41.3% |
| 29 | 10636ns | -41.3% |
| 30 | 10645ns | -41.4% |
| 31 | 10500ns | -40.0% |
| 32 | 10549ns | -40.2% |
| 33 | 10500ns | -40.4% |
| 34 | 10915ns | -43.0% |
| 35 | 11401ns | -45.4% |
| 36 | 11451ns | -34.6% |
| 37 | 11400ns | -39.5% |
| 38 | 11394ns | -40.1% |
| 39 | 11416ns | -39.7% |
| 40 | 11475ns | -40.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-radix10 | 0.728 | HIGH+ (drift/warm-up) |
| quantiser-radix2 | 0.610 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **quantiser-radix2**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-radix10 | 2.2ns | 10764.9ns | 0.0% |  |
| quantiser-radix2 | 2.3ns | 6241.9ns | 0.0% |  |

## Distribution (algo ns)

```
quantiser-radix10 (n=40, range 10188.0-11420.9 ns)
  10188.0 |
  10249.6 |
  10311.3 |#####
  10372.9 |
  10434.6 |
  10496.2 |###############
  10557.8 |##########
  10619.5 |########################################
  10681.1 |###############
  10742.8 |##########
  10804.4 |##########
  10866.1 |##########
  10927.7 |#####
  10989.4 |
  11051.0 |##########
  11112.6 |
  11174.3 |
  11235.9 |
  11297.6 |#####
  11359.2 |####################
  (6 below, 3 above range)

quantiser-radix2 (n=40, range 5769.1-6731.0 ns)
   5769.1 |
   5817.2 |
   5865.3 |
   5913.4 |
   5961.4 |######
   6009.5 |
   6057.6 |
   6105.7 |##
   6153.8 |
   6201.9 |########################################
   6250.0 |####
   6298.1 |########
   6346.2 |
   6394.3 |
   6442.4 |
   6490.5 |
   6538.6 |
   6586.7 |
   6634.8 |
   6682.9 |
  (5 below, 5 above range)

```

## Diagnostics

- **quantiser-radix10**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **quantiser-radix2**: autocorrelation=0.61 (measurement drift or warm-up artifact)
