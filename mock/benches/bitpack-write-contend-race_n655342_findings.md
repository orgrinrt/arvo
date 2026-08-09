# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 2023% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (7.64 us) leads bitpack-write-guarded (162.29 us) by 2023%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.73)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.73, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (7.64 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 21.2x the fastest

Fastest bitpack-write-dense (7.64 us) to slowest bitpack-write-guarded (162.29 us): 21.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 7643.8 ns median
- 1 variant significantly slower than baseline
- Spread: 21.23x (fastest 7643.8 ns, slowest 162291.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 7894ns | 7747ns | 7503ns | 7871ns | 8353ns | base |
| bitpack-write-guarded | 163529ns | 162444ns | 153361ns | 162056ns | 178115ns | +1971.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 7791ns | 7405ns | 8245ns | base | 8.412 |
| bitpack-write-guarded | 163236ns | 153099ns | 177764ns | +1995.24% | 0.401 |

## Performance model

- Peak throughput: **8.850 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 8.574 | 96.9% |
| bitpack-write-guarded | 0.404 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 7894ns | 7894ns | base |
| bitpack-write-guarded | 163529ns | 163529ns | +1971.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 7644ns | base | --- | [7628, 7838] | --- | --- | --- | --- |
| bitpack-write-guarded | 162291ns | +154556.8ns (+2022.0%) | [+151492, +156394]ns | [159092, 164010] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|
| 1 | 7645ns | +2048.8% |
| 2 | 7628ns | +2255.7% |
| 3 | 7620ns | +2047.6% |
| 4 | 7633ns | +1905.4% |
| 5 | 7941ns | +2041.8% |
| 6 | 7502ns | +2553.6% |
| 7 | 7663ns | +2373.0% |
| 8 | 7388ns | +2230.2% |
| 9 | 7440ns | +2100.8% |
| 10 | 7650ns | +2101.6% |
| 11 | 7637ns | +1947.2% |
| 12 | 7641ns | +1980.3% |
| 13 | 8049ns | +1923.3% |
| 14 | 8227ns | +1821.4% |
| 15 | 8251ns | +1760.3% |
| 16 | 8230ns | +1758.3% |
| 17 | 8232ns | +1777.2% |
| 18 | 8247ns | +1791.6% |
| 19 | 8225ns | +1910.0% |
| 20 | 8218ns | +1860.1% |
| 21 | 7685ns | +1888.4% |
| 22 | 7642ns | +1900.7% |
| 23 | 7629ns | +1969.6% |
| 24 | 7628ns | +1882.9% |
| 25 | 7624ns | +2006.3% |
| 26 | 8286ns | +1795.5% |
| 27 | 8232ns | +1886.1% |
| 28 | 8247ns | +1970.6% |
| 29 | 8237ns | +1976.9% |
| 30 | 8219ns | +1903.9% |
| 31 | 7372ns | +2082.7% |
| 32 | 7419ns | +2079.8% |
| 33 | 7357ns | +1990.6% |
| 34 | 7404ns | +2107.0% |
| 35 | 7354ns | +2208.7% |
| 36 | 7558ns | +2006.8% |
| 37 | 7734ns | +2006.6% |
| 38 | 7626ns | +2004.5% |
| 39 | 7684ns | +2062.8% |
| 40 | 7626ns | +2066.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.729 | HIGH+ (drift/warm-up) |
| bitpack-write-guarded | 0.550 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 3.7ns | 7790.8ns | 0.0% |  |
| bitpack-write-guarded | 8.0ns | 163235.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 7404.6-8245.2 ns)
   7404.6 |######
   7446.6 |
   7488.7 |###
   7530.7 |###
   7572.7 |
   7614.8 |########################################
   7656.8 |#########
   7698.8 |###
   7740.8 |
   7782.9 |
   7824.9 |
   7866.9 |
   7909.0 |###
   7951.0 |
   7993.0 |
   8035.1 |###
   8077.1 |
   8119.1 |
   8161.2 |
   8203.2 |########################
  (5 below, 4 above range)

bitpack-write-guarded (n=40, range 153098.8-177764.4 ns)
  153098.8 |####################
  154332.0 |##########
  155565.3 |####################
  156798.6 |####################
  158031.9 |##############################
  159265.2 |##########
  160498.4 |########################################
  161731.7 |####################
  162965.0 |########################################
  164198.3 |########################################
  165431.6 |##########
  166664.9 |
  167898.1 |##########
  169131.4 |####################
  170364.7 |####################
  171598.0 |##########
  172831.3 |
  174064.5 |
  175297.8 |
  176531.1 |
  (5 below, 3 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.73 (measurement drift or warm-up artifact)
- **bitpack-write-guarded**: autocorrelation=0.55 (measurement drift or warm-up artifact)
