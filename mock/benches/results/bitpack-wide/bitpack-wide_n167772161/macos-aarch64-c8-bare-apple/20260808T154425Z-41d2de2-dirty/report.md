# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (1.60 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-d16-padal at 763.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-d16-padal dominates: 84% faster than the next best (bitpack-wide-pipe4)

bitpack-wide-d16-padal (763.59 us) leads bitpack-wide-pipe4 (1.41 ms) by 84%, a clear separation rather than a photo finish. CV 24.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-d16-padal beats baseline by 52% (significant)

bitpack-wide-d16-padal is -835.31 us (52%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16 is an outlier: 2.1x slower than the field

bitpack-wide-d16 (1.60 ms) is 2.1x the fastest (763.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-wide-d16-padal shows warm-up / thermal drift (autocorr +0.77)

bitpack-wide-d16-padal's per-pass series has lag-1 autocorrelation +0.77, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-wide-d16-padal} vs {bitpack-wide-pipe4, bitpack-wide-d16-control, bitpack-wide-d16} (84% apart)

The field splits into a fast tier {bitpack-wide-d16-padal} and a slow tier {bitpack-wide-pipe4, bitpack-wide-d16-control, bitpack-wide-d16} with a 84% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-wide-d16-padal is inconsistent: worst-20% is 1.8x its best-20%

bitpack-wide-d16-padal's best 20% of batches run at 645.49 us but its worst 20% at 1.15 ms (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-wide-d16-padal** at 763594.2 ns median (-52.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.10x (fastest 763594.2 ns, slowest 1602516.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1645888ns | 1604321ns | 1489808ns | 1614257ns | 1896860ns | base |
| bitpack-wide-d16-control | 1663058ns | 1601166ns | 1505741ns | 1624044ns | 1937419ns | +1.04% |
| bitpack-wide-d16-padal | 828735ns | 764954ns | 647058ns | 780845ns | 1154082ns | -49.65% |
| bitpack-wide-pipe4 | 1481011ns | 1409845ns | 1275585ns | 1413800ns | 1888069ns | -10.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 1644138ns | 1488131ns | 1895108ns | base | 10.204 |
| bitpack-wide-d16-control | 1661319ns | 1503977ns | 1935469ns | +1.04% | 10.099 |
| bitpack-wide-d16-padal | 827078ns | 645490ns | 1151661ns | -49.70% | 20.285 |
| bitpack-wide-pipe4 | 1479282ns | 1273972ns | 1885740ns | -10.03% | 11.341 |

## Performance model

- Peak throughput: **25.991 Gops/s** (bitpack-wide-d16-padal; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 10.469 | 40.3% |
| bitpack-wide-d16-control | 10.488 | 40.4% |
| bitpack-wide-d16-padal | 21.971 | 84.5% |
| bitpack-wide-pipe4 | 11.913 | 45.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 1645888ns | 1645888ns | base |
| bitpack-wide-d16-control | 1663058ns | 1663058ns | +1.04% |
| bitpack-wide-d16-padal | 828735ns | 828735ns | -49.65% |
| bitpack-wide-pipe4 | 1481011ns | 1481011ns | -10.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 1602516ns | base | --- | [1579946, 1657696] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 1599594ns | no significant difference | [-21659, +32914]ns | [1541562, 1699888] | no | 0.4296 | 0.4296 | 0 |
| bitpack-wide-d16-padal | 763594ns | -842325.8ns (-52.6%) | [-862425, -800833]ns | [720933, 823235] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 1408308ns | -202111.5ns (-12.6%) | [-225283, -168245]ns | [1322041, 1504141] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|
| 1 | 1602523ns | +0.8% | -52.1% | -6.2% |
| 2 | 1602509ns | +8.3% | -55.2% | -5.8% |
| 3 | 1571487ns | +12.7% | -54.3% | -4.2% |
| 4 | 1629622ns | +10.7% | -53.0% | -9.6% |
| 5 | 1739748ns | -4.3% | -59.0% | -11.7% |
| 6 | 1663693ns | -8.0% | -49.9% | -9.2% |
| 7 | 1589848ns | -0.3% | -40.9% | -6.9% |
| 8 | 1470271ns | +12.4% | -45.7% | -0.7% |
| 9 | 1640417ns | +8.0% | -54.6% | -9.8% |
| 10 | 1651699ns | +5.8% | -50.2% | -7.8% |
| 11 | 1962087ns | +14.4% | -48.8% | -20.3% |
| 12 | 1936645ns | +5.4% | -37.3% | -14.7% |
| 13 | 1905681ns | +1.4% | -45.2% | +101.0% |
| 14 | 1860128ns | +0.1% | -34.7% | -11.2% |
| 15 | 1955150ns | -11.2% | -32.9% | -16.0% |
| 16 | 1777261ns | -9.2% | -33.7% | -9.4% |
| 17 | 1634374ns | +10.3% | -29.2% | -2.1% |
| 18 | 1684922ns | +14.8% | -44.4% | -8.9% |
| 19 | 1904468ns | -2.2% | -51.4% | -19.5% |
| 20 | 1859441ns | -5.3% | -41.6% | -19.9% |
| 21 | 1501947ns | +0.6% | -50.9% | -14.1% |
| 22 | 1556257ns | -2.6% | -54.5% | -16.6% |
| 23 | 1552231ns | -3.5% | -60.1% | -12.9% |
| 24 | 1492839ns | +0.3% | -51.5% | -12.0% |
| 25 | 1491876ns | +2.6% | -59.5% | -12.8% |
| 26 | 1525071ns | +0.7% | -57.5% | -14.2% |
| 27 | 1459820ns | +7.4% | -50.5% | -11.3% |
| 28 | 1482461ns | +4.8% | -54.9% | -14.6% |
| 29 | 1498387ns | +0.4% | -55.6% | -11.5% |
| 30 | 1512765ns | -1.5% | -56.6% | -12.5% |
| 31 | 1696488ns | -9.9% | -61.7% | -25.9% |
| 32 | 1669453ns | -6.7% | -50.7% | -20.9% |
| 33 | 1670857ns | -0.7% | -53.2% | -19.7% |
| 34 | 1605276ns | +2.4% | -52.6% | -19.6% |
| 35 | 1588405ns | -4.1% | -52.9% | -20.1% |
| 36 | 1598650ns | -4.0% | -49.3% | -20.7% |
| 37 | 1600133ns | -5.8% | -58.2% | -21.2% |
| 38 | 1507451ns | +0.1% | -43.4% | -14.5% |
| 39 | 1589602ns | -1.3% | -57.1% | -14.6% |
| 40 | 1523577ns | +1.6% | -57.1% | -11.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | 0.698 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-control | 0.746 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-padal | 0.773 | HIGH+ (drift/warm-up) |
| bitpack-wide-pipe4 | 0.238 | moderate+ |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 17/40, lost 22/40
- **bitpack-wide-d16-padal**: won 40/40, lost 0/40
- **bitpack-wide-pipe4**: won 39/40, lost 1/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 45.9ns | 1644138.0ns | 0.0% |  |
| bitpack-wide-d16-control | 44.3ns | 1661319.2ns | 0.0% |  |
| bitpack-wide-d16-padal | 37.0ns | 827078.2ns | 0.0% |  |
| bitpack-wide-pipe4 | 47.8ns | 1479282.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 1488131.4-1895107.5 ns)
  1488131.4 |########################################
  1508480.3 |########################
  1528829.1 |
  1549177.9 |################
  1569526.7 |################################
  1589875.5 |########################################
  1610224.3 |########
  1630573.1 |################
  1650921.9 |################################
  1671270.7 |########
  1691619.5 |########
  1711968.3 |
  1732317.1 |########
  1752665.9 |
  1773014.7 |########
  1793363.5 |
  1813712.3 |
  1834061.1 |
  1854409.9 |################
  1874758.7 |
  (3 below, 5 above range)

bitpack-wide-d16-control (n=40, range 1503977.1-1935468.7 ns)
  1503977.1 |########################################
  1525551.7 |########################################
  1547126.3 |################################
  1568700.9 |################
  1590275.5 |
  1611850.0 |################
  1633424.6 |################
  1654999.2 |################
  1676573.8 |
  1698148.4 |
  1719722.9 |################
  1741297.5 |################
  1762872.1 |################
  1784446.7 |################
  1806021.2 |
  1827595.8 |
  1849170.4 |################
  1870745.0 |
  1892319.6 |
  1913894.1 |################
  (4 below, 2 above range)

bitpack-wide-d16-padal (n=40, range 645489.7-1151660.8 ns)
  645489.7 |########################################
  670798.3 |#####
  696106.8 |######################
  721415.4 |######################
  746723.9 |######################
  772032.5 |#####
  797341.1 |###########
  822649.6 |#################
  847958.2 |#####
  873266.7 |
  898575.3 |
  923883.8 |#################
  949192.4 |
  974500.9 |
  999809.5 |#####
  1025118.0 |#####
  1050426.6 |
  1075735.1 |#####
  1101043.7 |
  1126352.2 |
  (2 below, 5 above range)

bitpack-wide-pipe4 (n=40, range 1273972.0-1885739.8 ns)
  1273972.0 |########################################
  1304560.4 |#################################
  1335148.8 |##########################
  1365737.2 |
  1396325.6 |
  1426913.9 |
  1457502.3 |##########################
  1488090.7 |#################################
  1518679.1 |##########################
  1549267.5 |######
  1579855.9 |#############
  1610444.3 |
  1641032.7 |####################
  1671621.1 |
  1702209.5 |
  1732797.9 |
  1763386.3 |
  1793974.7 |
  1824563.1 |
  1855151.4 |
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-wide-d16**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-control**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-padal**: CV=22.4% (high variance, measurements may be unstable)
- **bitpack-wide-d16-padal**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **bitpack-wide-pipe4**: CV=26.8% (high variance, measurements may be unstable)
