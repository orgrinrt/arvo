# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (194.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 10.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1232% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (10.03 us) leads bitpack-write-windowed (133.62 us) by 1232%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 95% (significant)

bitpack-write-dense is -184.03 us (95%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 19.3x slower than the field

bitpack-write-aligned (194.06 us) is 19.3x the fastest (10.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-windowed shows alternating (throttle bounce) (autocorr -0.76)

bitpack-write-windowed's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 19.3x the fastest

Fastest bitpack-write-dense (10.03 us) to slowest bitpack-write-aligned (194.06 us): 19.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 10035.0 ns median (-94.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 19.34x (fastest 10035.0 ns, slowest 194061.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef1 |
| Counter frequency | 2112601419 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 204466ns | 194139ns | 192867ns | 197430ns | 237173ns | base |
| bitpack-write-dense | 10120ns | 10112ns | 10105ns | 10113ns | 10159ns | -95.05% |
| bitpack-write-windowed | 132845ns | 133706ns | 119156ns | 133065ns | 145875ns | -35.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 204379ns | 192784ns | 237070ns | base | 0.321 |
| bitpack-write-dense | 10038ns | 10030ns | 10058ns | -95.09% | 6.529 |
| bitpack-write-windowed | 132758ns | 119075ns | 145777ns | -35.04% | 0.494 |

## Performance model

- Peak throughput: **6.534 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.338 | 5.2% |
| bitpack-write-dense | 6.531 | 99.9% |
| bitpack-write-windowed | 0.490 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 204466ns | 204466ns | base |
| bitpack-write-dense | 10120ns | 10120ns | -95.05% |
| bitpack-write-windowed | 132845ns | 132845ns | -35.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 194062ns | base | --- | [194025, 194101] | --- | --- | --- | --- |
| bitpack-write-dense | 10035ns | -184028.7ns (-94.8%) | [-184067, -183994]ns | [10032, 10036] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 133622ns | -74332.2ns (-38.3%) | [-74984, -73447]ns | [120334, 145636] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 192815ns | -94.8% | -24.5% |
| 1 | 2 | 194057ns | -94.8% | -38.6% |
| 1 | 3 | 237237ns | -95.8% | -38.6% |
| 1 | 4 | 194071ns | -94.8% | -38.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.394 | moderate- |
| bitpack-write-dense | 0.279 | moderate+ |
| bitpack-write-windowed | -0.761 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 21.6ns | 204378.8ns | 0.0% |  |
| bitpack-write-dense | 20.5ns | 10038.3ns | 0.2% |  |
| bitpack-write-windowed | 22.1ns | 132758.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 192783.6-237069.5 ns)
  192783.6 |########################################
  194997.9 |
  197212.2 |
  199426.5 |
  201640.8 |
  203855.1 |
  206069.4 |
  208283.7 |
  210498.0 |
  212712.3 |
  214926.6 |
  217140.9 |
  219355.2 |
  221569.5 |
  223783.8 |
  225998.1 |
  228212.4 |
  230426.7 |
  232641.0 |
  234855.3 |######
  (4 below, 6 above range)

bitpack-write-dense (n=40, range 10029.9-10058.4 ns)
  10029.9 |########################################
  10031.3 |############################
  10032.8 |######################
  10034.2 |########################################
  10035.6 |############################
  10037.0 |######################
  10038.5 |###########
  10039.9 |#####
  10041.3 |
  10042.7 |
  10044.2 |
  10045.6 |
  10047.0 |
  10048.4 |
  10049.9 |
  10051.3 |
  10052.7 |
  10054.1 |
  10055.6 |
  10057.0 |
  (3 below, 2 above range)

bitpack-write-windowed (n=40, range 119075.1-145777.3 ns)
  119075.1 |#########################
  120410.2 |####
  121745.3 |
  123080.4 |
  124415.5 |
  125750.6 |
  127085.8 |
  128420.9 |
  129756.0 |
  131091.1 |
  132426.2 |
  133761.3 |
  135096.4 |
  136431.5 |
  137766.7 |
  139101.8 |
  140436.9 |
  141772.0 |
  143107.1 |
  144442.2 |########################################
  (6 below, 1 above range)

```
