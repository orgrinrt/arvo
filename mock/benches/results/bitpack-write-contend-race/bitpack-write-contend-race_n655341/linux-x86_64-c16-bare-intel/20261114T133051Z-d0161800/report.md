# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 2237% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (10.15 us) leads bitpack-write-unsound (237.29 us) by 2237%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 24.0x slower than the field

bitpack-write-guarded (243.67 us) is 24.0x the fastest (10.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (10.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 24.0x the fastest

Fastest bitpack-write-dense (10.15 us) to slowest bitpack-write-guarded (243.67 us): 24.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 10151.4 ns median
- 2 variants significantly slower than baseline
- Spread: 24.00x (fastest 10151.4 ns, slowest 243674.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 10250ns | 10234ns | 10161ns | 10233ns | 10390ns | base |
| bitpack-write-guarded | 243598ns | 243762ns | 219172ns | 243593ns | 268039ns | +2276.52% |
| bitpack-write-unsound | 237055ns | 237385ns | 235620ns | 237001ns | 238652ns | +2212.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 10167ns | 10084ns | 10308ns | base | 6.446 |
| bitpack-write-guarded | 243505ns | 219090ns | 267932ns | +2295.03% | 0.269 |
| bitpack-write-unsound | 236953ns | 235524ns | 238535ns | +2230.59% | 0.277 |

## Performance model

- Peak throughput: **6.499 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 6.456 | 99.3% |
| bitpack-write-guarded | 0.269 | 4.1% |
| bitpack-write-unsound | 0.276 | 4.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 10250ns | 10250ns | base |
| bitpack-write-guarded | 243598ns | 243598ns | +2276.52% |
| bitpack-write-unsound | 237055ns | 237055ns | +2212.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 10151ns | base | --- | [10141, 10156] | --- | --- | --- | --- |
| bitpack-write-guarded | 243675ns | +233462.4ns (+2299.8%) | [+209025, +257674]ns | [219148, 267827] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 237286ns | +227130.8ns (+2237.4%) | [+226502, +227153]ns | [236645, 237301] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|---|
| 1 | 1 | 10155ns | +2537.6% | +2220.3% |
| 1 | 2 | 10156ns | +2057.7% | +2234.6% |
| 1 | 3 | 10135ns | +2061.8% | +2241.4% |
| 1 | 4 | 10153ns | +2537.8% | +2237.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.398 | moderate- |
| bitpack-write-guarded | -0.250 | moderate- |
| bitpack-write-unsound | 0.004 | ok |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4
- **bitpack-write-unsound**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 20.6ns | 10167.1ns | 0.2% |  |
| bitpack-write-guarded | 23.3ns | 243505.2ns | 0.0% |  |
| bitpack-write-unsound | 25.5ns | 236953.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 10083.6-10308.2 ns)
  10083.6 |######
  10094.8 |
  10106.1 |###
  10117.3 |##########
  10128.5 |##########
  10139.7 |####################
  10151.0 |########################################
  10162.2 |###
  10173.4 |###
  10184.7 |
  10195.9 |
  10207.1 |
  10218.4 |
  10229.6 |
  10240.8 |
  10252.1 |
  10263.3 |
  10274.5 |
  10285.8 |
  10297.0 |###
  (4 below, 6 above range)

bitpack-write-guarded (n=40, range 219090.3-267931.6 ns)
  219090.3 |#####################################
  221532.4 |
  223974.5 |
  226416.5 |
  228858.6 |
  231300.7 |
  233742.7 |
  236184.8 |
  238626.8 |
  241068.9 |
  243511.0 |
  245953.0 |
  248395.1 |
  250837.2 |
  253279.2 |
  255721.3 |
  258163.3 |
  260605.4 |
  263047.5 |
  265489.5 |########################################
  (3 below, 2 above range)

bitpack-write-unsound (n=40, range 235524.5-238535.4 ns)
  235524.5 |##############
  235675.0 |##
  235825.6 |
  235976.1 |
  236126.7 |##
  236277.2 |
  236427.8 |
  236578.3 |####
  236728.9 |
  236879.4 |##
  237029.9 |
  237180.5 |########################################
  237331.0 |####
  237481.6 |
  237632.1 |
  237782.7 |
  237933.2 |
  238083.8 |
  238234.3 |
  238384.9 |
  (4 below, 2 above range)

```
