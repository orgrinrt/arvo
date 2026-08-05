# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-dense shows warm-up / thermal drift (autocorr +0.88)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (34 ns) is smaller than the fastest variant's own run-to-run std-dev (101 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bitpack-footprint-dense)

The baseline bitpack-footprint-dense is the fastest (1.64 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (bitpack-footprint-dense) is the fastest** at 1635.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 1635.2 ns, slowest 1668.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1722ns | 1704ns | 1626ns | 1699ns | 1888ns | base |
| bitpack-footprint-dense-alt | 1797ns | 1742ns | 1738ns | 1763ns | 1960ns | +4.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 1650ns | 1557ns | 1809ns | base | 9.933 |
| bitpack-footprint-dense-alt | 1722ns | 1665ns | 1878ns | +4.40% | 9.514 |

## Performance model

- Peak throughput: **10.524 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 10.020 | 95.2% |
| bitpack-footprint-dense-alt | 9.818 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 1722ns | 1722ns | base |
| bitpack-footprint-dense-alt | 1797ns | 1797ns | +4.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 1635ns | base | --- | [1561, 1666] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 1669ns | +110.6ns (+6.8%) | [+105, +114]ns | [1668, 1672] | YES | 0.0007 | 0.0007 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|
| 1 | 1560ns | +6.8% |
| 2 | 1559ns | +6.9% |
| 3 | 1561ns | +6.7% |
| 4 | 1607ns | +3.8% |
| 5 | 1571ns | +6.1% |
| 6 | 1567ns | +6.5% |
| 7 | 1562ns | +6.7% |
| 8 | 1560ns | +6.8% |
| 9 | 1559ns | +6.9% |
| 10 | 1562ns | +6.6% |
| 11 | 1557ns | +7.4% |
| 12 | 1559ns | +7.3% |
| 13 | 1556ns | +7.4% |
| 14 | 1558ns | +7.3% |
| 15 | 1556ns | +7.4% |
| 16 | 1558ns | +7.1% |
| 17 | 1559ns | +7.1% |
| 18 | 1556ns | +7.2% |
| 19 | 1558ns | +7.3% |
| 20 | 1557ns | +7.2% |
| 21 | 1809ns | -7.9% |
| 22 | 1809ns | -7.9% |
| 23 | 1807ns | -7.8% |
| 24 | 1809ns | -7.8% |
| 25 | 1809ns | -7.7% |
| 26 | 1808ns | -8.0% |
| 27 | 1812ns | -3.8% |
| 28 | 1806ns | -7.8% |
| 29 | 1809ns | -8.1% |
| 30 | 1808ns | +24.8% |
| 31 | 1667ns | +8.2% |
| 32 | 1665ns | +8.8% |
| 33 | 1666ns | +8.7% |
| 34 | 1665ns | +8.5% |
| 35 | 1665ns | +8.6% |
| 36 | 1665ns | +8.5% |
| 37 | 1666ns | +15.0% |
| 38 | 1667ns | +8.5% |
| 39 | 1663ns | +8.7% |
| 40 | 1665ns | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.884 | HIGH+ (drift/warm-up) |
| bitpack-footprint-dense-alt | 0.361 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 9/40, lost 31/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 2.0ns | 1649.5ns | 0.1% |  |
| bitpack-footprint-dense-alt | 1.8ns | 1722.0ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 1556.9-1809.1 ns)
   1556.9 |########################################
   1569.5 |###
   1582.1 |
   1594.7 |###
   1607.3 |
   1619.9 |
   1632.5 |
   1645.1 |
   1657.8 |##############################
   1670.4 |
   1683.0 |
   1695.6 |
   1708.2 |
   1720.8 |
   1733.4 |
   1746.0 |
   1758.7 |
   1771.3 |
   1783.9 |
   1796.5 |#####################
  (5 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 1664.8-1878.3 ns)
   1664.8 |########################################
   1675.5 |
   1686.2 |
   1696.9 |
   1707.5 |
   1718.2 |
   1728.9 |
   1739.5 |#
   1750.2 |
   1760.9 |
   1771.6 |
   1782.2 |
   1792.9 |
   1803.6 |#############
   1814.2 |
   1824.9 |
   1835.6 |
   1846.3 |
   1856.9 |
   1867.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: autocorrelation=0.88 (measurement drift or warm-up artifact)
