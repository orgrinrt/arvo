# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1569% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (306.43 us) leads bitpack-write-guarded (5.12 ms) by 1569%, a clear separation rather than a photo finish. CV 15.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 15.5%)

bitpack-write-dense wins on median (306.43 us) yet has the highest variance (CV 15.5%), while bitpack-write-guarded is the steadiest (CV 14.3%, 5.12 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-guarded shows warm-up / thermal drift (autocorr +0.69)

bitpack-write-guarded's per-pass series has lag-1 autocorrelation +0.69, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (306.43 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.7x the fastest

Fastest bitpack-write-dense (306.43 us) to slowest bitpack-write-guarded (5.12 ms): 16.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 306425.8 ns median
- 1 variant significantly slower than baseline
- Spread: 16.69x (fastest 306425.8 ns, slowest 5115273.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 318872ns | 307315ns | 259057ns | 316556ns | 385637ns | base |
| bitpack-write-guarded | 5505927ns | 5117224ns | 4858294ns | 5325954ns | 6693478ns | +1626.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 318149ns | 258518ns | 384651ns | base | 6.592 |
| bitpack-write-guarded | 5503966ns | 4856743ns | 6691172ns | +1630.00% | 0.381 |

## Performance model

- Peak throughput: **8.112 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 6.844 | 84.4% |
| bitpack-write-guarded | 0.410 | 5.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 318872ns | 318872ns | base |
| bitpack-write-guarded | 5505927ns | 5505927ns | +1626.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 306426ns | base | --- | [288943, 351313] | --- | --- | --- | --- |
| bitpack-write-guarded | 5115274ns | +4839611.3ns (+1579.4%) | [+4673340, +5057032]ns | [5017822, 5397119] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|
| 1 | 368373ns | +1628.0% |
| 2 | 287392ns | +2138.0% |
| 3 | 344845ns | +1799.2% |
| 4 | 350600ns | +1685.0% |
| 5 | 315478ns | +1956.3% |
| 6 | 314583ns | +2024.2% |
| 7 | 385786ns | +1606.4% |
| 8 | 352026ns | +1943.3% |
| 9 | 356331ns | +1781.2% |
| 10 | 371624ns | +1610.7% |
| 11 | 280402ns | +1822.4% |
| 12 | 298266ns | +1618.3% |
| 13 | 280538ns | +1689.7% |
| 14 | 294363ns | +1582.6% |
| 15 | 265756ns | +1825.9% |
| 16 | 298269ns | +2197.3% |
| 17 | 279611ns | +1760.2% |
| 18 | 263618ns | +2357.8% |
| 19 | 296721ns | +1610.1% |
| 20 | 294843ns | +1734.2% |
| 21 | 355181ns | +1311.9% |
| 22 | 375948ns | +1233.2% |
| 23 | 357801ns | +1296.2% |
| 24 | 348867ns | +1314.1% |
| 25 | 367595ns | +1248.6% |
| 26 | 383274ns | +1179.5% |
| 27 | 369709ns | +1260.4% |
| 28 | 411891ns | +1105.2% |
| 29 | 352798ns | +1345.0% |
| 30 | 410605ns | +1216.0% |
| 31 | 246418ns | +2030.7% |
| 32 | 264647ns | +1846.3% |
| 33 | 252254ns | +1926.6% |
| 34 | 290493ns | +1577.8% |
| 35 | 318773ns | +1494.4% |
| 36 | 256800ns | +1825.1% |
| 37 | 259420ns | +1743.4% |
| 38 | 272791ns | +1695.3% |
| 39 | 259235ns | +1752.5% |
| 40 | 272028ns | +1634.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.543 | HIGH+ (drift/warm-up) |
| bitpack-write-guarded | 0.689 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 42.7ns | 318148.9ns | 0.0% |  |
| bitpack-write-guarded | 139.3ns | 5503965.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 258518.4-384651.4 ns)
  258518.4 |########################################
  264825.0 |##########
  271131.7 |####################
  277438.3 |##############################
  283745.0 |##########
  290051.6 |##############################
  296358.3 |##############################
  302664.9 |
  308971.6 |##########
  315278.2 |####################
  321584.9 |
  327891.5 |
  334198.2 |
  340504.8 |##########
  346811.5 |########################################
  353118.1 |##############################
  359424.8 |
  365731.4 |########################################
  372038.1 |##########
  378344.7 |##########
  (3 below, 3 above range)

bitpack-write-guarded (n=40, range 4856743.2-6691171.9 ns)
  4856743.2 |#########################
  4948464.6 |########################################
  5040186.1 |##############################
  5131907.5 |##########
  5223628.9 |#####
  5315350.4 |##########
  5407071.8 |#####
  5498793.2 |
  5590514.7 |
  5682236.1 |
  5773957.6 |
  5865679.0 |
  5957400.4 |
  6049121.9 |
  6140843.3 |
  6232564.8 |#####
  6324286.2 |##########
  6416007.6 |###############
  6507729.1 |##########
  6599450.5 |#####
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **bitpack-write-guarded**: autocorrelation=0.69 (measurement drift or warm-up artifact)
