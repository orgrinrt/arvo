# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1847% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (317.97 us) leads bitpack-write-guarded (6.19 ms) by 1847%, a clear separation rather than a photo finish. CV 28.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.66)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.66, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (317.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 19.5x the fastest

Fastest bitpack-write-dense (317.97 us) to slowest bitpack-write-guarded (6.19 ms): 19.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-guarded is inconsistent: worst-20% is 2.0x its best-20%

bitpack-write-guarded's best 20% of batches run at 5.06 ms but its worst 20% at 9.97 ms (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 317970.0 ns median
- 1 variant significantly slower than baseline
- Spread: 19.47x (fastest 317970.0 ns, slowest 6191502.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 346853ns | 318545ns | 269787ns | 327640ns | 481554ns | base |
| bitpack-write-guarded | 6818546ns | 6194188ns | 5063875ns | 6350573ns | 9977135ns | +1865.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 345940ns | 269211ns | 479717ns | base | 6.062 |
| bitpack-write-guarded | 6815761ns | 5062136ns | 9972862ns | +1870.22% | 0.308 |

## Performance model

- Peak throughput: **7.790 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 6.595 | 84.7% |
| bitpack-write-guarded | 0.339 | 4.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 346853ns | 346853ns | base |
| bitpack-write-guarded | 6818546ns | 6818546ns | +1865.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 317970ns | base | --- | [307008, 348720] | --- | --- | --- | --- |
| bitpack-write-guarded | 6191502ns | +5847838.5ns (+1839.1%) | [+5263860, +6327018]ns | [5703720, 6786113] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|
| 1 | 268228ns | +1837.4% |
| 2 | 264573ns | +1739.3% |
| 3 | 262430ns | +1884.5% |
| 4 | 284000ns | +1716.7% |
| 5 | 281507ns | +1698.6% |
| 6 | 287043ns | +1750.0% |
| 7 | 279109ns | +1686.3% |
| 8 | 267237ns | +1788.8% |
| 9 | 268953ns | +1748.1% |
| 10 | 261654ns | +2378.8% |
| 11 | 323915ns | +2941.8% |
| 12 | 314932ns | +2584.9% |
| 13 | 372895ns | +3189.7% |
| 14 | 319102ns | +3294.2% |
| 15 | 314518ns | +3175.2% |
| 16 | 370748ns | +1710.5% |
| 17 | 383120ns | +1631.5% |
| 18 | 344972ns | +1983.5% |
| 19 | 353607ns | +2822.2% |
| 20 | 411968ns | +2132.2% |
| 21 | 747965ns | +671.8% |
| 22 | 547442ns | +1153.1% |
| 23 | 499652ns | +1053.4% |
| 24 | 420042ns | +1170.0% |
| 25 | 386546ns | +1362.0% |
| 26 | 333985ns | +1550.1% |
| 27 | 352468ns | +2145.1% |
| 28 | 436429ns | +1296.1% |
| 29 | 360442ns | +1645.0% |
| 30 | 387688ns | +1848.7% |
| 31 | 305792ns | +1757.8% |
| 32 | 306175ns | +1770.3% |
| 33 | 335139ns | +1478.6% |
| 34 | 309371ns | +1764.2% |
| 35 | 316838ns | +1820.0% |
| 36 | 307842ns | +1954.0% |
| 37 | 301918ns | +2030.5% |
| 38 | 315233ns | +2611.1% |
| 39 | 327349ns | +2467.6% |
| 40 | 304759ns | +2383.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.663 | HIGH+ (drift/warm-up) |
| bitpack-write-guarded | 0.657 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 61.0ns | 345939.6ns | 0.0% |  |
| bitpack-write-guarded | 215.1ns | 6815761.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 269211.3-479716.5 ns)
  269211.3 |######
  279736.6 |####################
  290261.8 |
  300787.1 |########################################
  311312.4 |#################################
  321837.6 |#############
  332362.9 |#############
  342888.1 |#############
  353413.4 |#############
  363938.7 |#############
  374463.9 |######
  384989.2 |#############
  395514.4 |
  406039.7 |######
  416565.0 |######
  427090.2 |######
  437615.5 |
  448140.7 |
  458666.0 |
  469191.3 |
  (6 below, 3 above range)

bitpack-write-guarded (n=40, range 5062136.0-9972862.2 ns)
  5062136.0 |#################################
  5307672.3 |####################
  5553208.6 |########################################
  5798744.9 |
  6044281.2 |#############
  6289817.5 |##########################
  6535353.9 |#############
  6780890.2 |######
  7026426.5 |######
  7271962.8 |
  7517499.1 |#############
  7763035.4 |######
  8008571.7 |
  8254108.0 |#############
  8499644.3 |######
  8745180.6 |
  8990717.0 |######
  9236253.3 |
  9481789.6 |
  9727325.9 |######
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=25.9% (high variance, measurements may be unstable)
- **bitpack-write-dense**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **bitpack-write-guarded**: CV=27.3% (high variance, measurements may be unstable)
- **bitpack-write-guarded**: autocorrelation=0.66 (measurement drift or warm-up artifact)
