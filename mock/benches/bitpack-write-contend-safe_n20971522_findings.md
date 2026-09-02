# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (4.54 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 237.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1227% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (237.87 us) leads bitpack-write-windowed (3.16 ms) by 1227%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 95% (significant)

bitpack-write-dense is -4.29 ms (95%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 19.1x slower than the field

bitpack-write-aligned (4.54 ms) is 19.1x the fastest (237.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.78)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 19.1x the fastest

Fastest bitpack-write-dense (237.87 us) to slowest bitpack-write-aligned (4.54 ms): 19.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 237868.2 ns median (-94.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 19.08x (fastest 237868.2 ns, slowest 4537385.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 4609035ns | 4538794ns | 4463249ns | 4534968ns | 4977023ns | base |
| bitpack-write-dense | 244618ns | 238259ns | 232448ns | 241034ns | 267539ns | -94.69% |
| bitpack-write-windowed | 3239372ns | 3158137ns | 3137933ns | 3164887ns | 3564268ns | -29.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 4607518ns | 4461998ns | 4975157ns | base | 0.455 |
| bitpack-write-dense | 244156ns | 231966ns | 267131ns | -94.70% | 8.589 |
| bitpack-write-windowed | 3238019ns | 3136724ns | 3562471ns | -29.72% | 0.648 |

## Performance model

- Peak throughput: **9.041 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.462 | 5.1% |
| bitpack-write-dense | 8.816 | 97.5% |
| bitpack-write-windowed | 0.664 | 7.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 4609035ns | 4609035ns | base |
| bitpack-write-dense | 244618ns | 244618ns | -94.69% |
| bitpack-write-windowed | 3239372ns | 3239372ns | -29.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 4537385ns | base | --- | [4502424, 4566371] | --- | --- | --- | --- |
| bitpack-write-dense | 237868ns | -4292211.8ns (-94.6%) | [-4325072, -4265203]ns | [236397, 243407] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 3157149ns | -1350735.3ns (-29.8%) | [-1409357, -1333080]ns | [3148836, 3175244] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|
| 1 | 4578365ns | -94.8% | -31.4% |
| 2 | 4532488ns | -94.9% | -28.2% |
| 3 | 4462327ns | -94.7% | -28.8% |
| 4 | 4458766ns | -94.7% | -29.4% |
| 5 | 4461682ns | -94.8% | -29.3% |
| 6 | 4468579ns | -94.8% | -29.5% |
| 7 | 4462384ns | -94.7% | -29.7% |
| 8 | 4468692ns | -94.7% | -29.8% |
| 9 | 4516208ns | -94.9% | -29.9% |
| 10 | 4518160ns | -94.7% | -4.7% |
| 11 | 4476758ns | -94.1% | -28.2% |
| 12 | 4779122ns | -94.3% | -33.8% |
| 13 | 4640130ns | -94.0% | -30.4% |
| 14 | 4465862ns | -94.1% | -27.7% |
| 15 | 5090554ns | -95.0% | -35.3% |
| 16 | 4671524ns | -94.3% | -32.4% |
| 17 | 4594900ns | -94.2% | -31.1% |
| 18 | 4542283ns | -94.1% | -30.5% |
| 19 | 4570022ns | -94.3% | -30.0% |
| 20 | 4459961ns | -94.3% | -26.8% |
| 21 | 4558942ns | -94.8% | -30.9% |
| 22 | 4474543ns | -94.7% | -29.8% |
| 23 | 4607870ns | -94.9% | -31.8% |
| 24 | 4639255ns | -94.9% | -31.9% |
| 25 | 4456423ns | -94.8% | -29.7% |
| 26 | 4499955ns | -94.8% | -29.2% |
| 27 | 4574980ns | -94.9% | -31.6% |
| 28 | 4482489ns | -94.7% | -30.0% |
| 29 | 4544901ns | -94.6% | -30.9% |
| 30 | 4545284ns | -94.8% | -30.8% |
| 31 | 4809372ns | -94.7% | -33.6% |
| 32 | 4506588ns | -94.7% | -30.0% |
| 33 | 4487932ns | -94.5% | -30.1% |
| 34 | 4504892ns | -94.8% | +4.4% |
| 35 | 4586926ns | -94.8% | -31.2% |
| 36 | 4517017ns | -94.6% | -29.8% |
| 37 | 4550578ns | -94.6% | -29.6% |
| 38 | 6507541ns | -96.4% | -51.0% |
| 39 | 4663760ns | -94.9% | -32.5% |
| 40 | 4562719ns | -94.9% | -31.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.033 | ok |
| bitpack-write-dense | 0.778 | HIGH+ (drift/warm-up) |
| bitpack-write-windowed | -0.058 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 40/40, lost 0/40
- **bitpack-write-windowed**: won 39/40, lost 1/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 71.7ns | 4607518.3ns | 0.0% |  |
| bitpack-write-dense | 9.1ns | 244155.7ns | 0.0% |  |
| bitpack-write-windowed | 62.3ns | 3238019.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 4461998.0-4975157.2 ns)
  4461998.0 |########################################
  4487656.0 |####################
  4513313.9 |####################
  4538971.9 |##############################
  4564629.9 |####################
  4590287.8 |##########
  4615945.8 |##########
  4641603.7 |#####
  4667261.7 |#####
  4692919.7 |
  4718577.6 |
  4744235.6 |
  4769893.5 |#####
  4795551.5 |#####
  4821209.4 |
  4846867.4 |
  4872525.4 |
  4898183.3 |
  4923841.3 |
  4949499.2 |
  (4 below, 2 above range)

bitpack-write-dense (n=40, range 231966.2-267131.5 ns)
  231966.2 |#############
  233724.5 |########
  235482.7 |########################################
  237241.0 |#################
  238999.2 |########
  240757.5 |
  242515.8 |########
  244274.0 |####
  246032.3 |####
  247790.6 |
  249548.8 |
  251307.1 |
  253065.4 |####
  254823.6 |####
  256581.9 |####
  258340.1 |####
  260098.4 |
  261856.7 |####
  263614.9 |########
  265373.2 |
  (5 below, 4 above range)

bitpack-write-windowed (n=40, range 3136724.1-3562471.4 ns)
  3136724.1 |########################################
  3158011.4 |#############
  3179298.8 |########
  3200586.2 |####
  3221873.5 |####
  3243160.9 |####
  3264448.3 |
  3285735.6 |##
  3307023.0 |
  3328310.4 |
  3349597.7 |
  3370885.1 |
  3392172.5 |
  3413459.8 |
  3434747.2 |
  3456034.5 |
  3477321.9 |
  3498609.3 |
  3519896.6 |
  3541184.0 |
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.78 (measurement drift or warm-up artifact)
