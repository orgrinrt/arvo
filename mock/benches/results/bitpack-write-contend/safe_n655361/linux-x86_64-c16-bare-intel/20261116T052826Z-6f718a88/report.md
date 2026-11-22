# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (214.14 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 9.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1374% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (9.98 us) leads bitpack-write-windowed (147.09 us) by 1374%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 95% (significant)

bitpack-write-dense is -204.16 us (95%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 21.5x slower than the field

bitpack-write-aligned (214.14 us) is 21.5x the fastest (9.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.69)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.5x the fastest

Fastest bitpack-write-dense (9.98 us) to slowest bitpack-write-aligned (214.14 us): 21.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 9979.5 ns median (-95.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.46x (fastest 9979.5 ns, slowest 214144.2 ns)

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
| Counter frequency | 2112697486 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 214827ns | 214235ns | 192747ns | 214484ns | 237939ns | base |
| bitpack-write-dense | 10082ns | 10061ns | 10038ns | 10065ns | 10178ns | -95.31% |
| bitpack-write-windowed | 140401ns | 147179ns | 119802ns | 144950ns | 147350ns | -34.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 214729ns | 192661ns | 237829ns | base | 0.305 |
| bitpack-write-dense | 10000ns | 9960ns | 10090ns | -95.34% | 6.554 |
| bitpack-write-windowed | 140313ns | 119730ns | 147247ns | -34.66% | 0.467 |

## Performance model

- Peak throughput: **6.580 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.306 | 4.7% |
| bitpack-write-dense | 6.567 | 99.8% |
| bitpack-write-windowed | 0.446 | 6.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 214827ns | 214827ns | base |
| bitpack-write-dense | 10082ns | 10082ns | -95.31% |
| bitpack-write-windowed | 140401ns | 140401ns | -34.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 214144ns | base | --- | [192742, 235566] | --- | --- | --- | --- |
| bitpack-write-dense | 9980ns | -204159.5ns (-95.3%) | [-225591, -182765]ns | [9970, 9996] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 147095ns | -66707.6ns (-31.2%) | [-88450, -45660]ns | [147078, 147103] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 237315ns | -95.8% | -49.4% |
| 1 | 2 | 235534ns | -95.8% | -37.5% |
| 1 | 3 | 192741ns | -94.8% | -23.7% |
| 1 | 4 | 192669ns | -94.8% | -23.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.260 | moderate+ |
| bitpack-write-dense | -0.689 | HIGH- (thermal bounce) |
| bitpack-write-windowed | -0.084 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 23.0ns | 214729.3ns | 0.0% |  |
| bitpack-write-dense | 20.9ns | 9999.9ns | 0.2% |  |
| bitpack-write-windowed | 23.7ns | 140313.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 192661.0-237828.8 ns)
  192661.0 |########################################
  194919.4 |
  197177.8 |
  199436.1 |
  201694.5 |
  203952.9 |
  206211.3 |
  208469.7 |
  210728.1 |
  212986.5 |
  215244.9 |
  217503.3 |
  219761.6 |
  222020.0 |
  224278.4 |
  226536.8 |
  228795.2 |
  231053.6 |
  233312.0 |################
  235570.4 |##################################
  (5 below, 1 above range)

bitpack-write-dense (n=40, range 9960.4-10090.2 ns)
   9960.4 |########################################
   9966.9 |############################
   9973.4 |############################
   9979.9 |###########
   9986.4 |###########
   9992.9 |###########
   9999.4 |###########
  10005.9 |#################
  10012.4 |###########
  10018.8 |
  10025.3 |#####
  10031.8 |
  10038.3 |
  10044.8 |#####
  10051.3 |
  10057.8 |
  10064.3 |
  10070.8 |
  10077.3 |
  10083.8 |
  (4 below, 4 above range)

bitpack-write-windowed (n=40, range 119730.3-147246.8 ns)
  119730.3 |########
  121106.1 |
  122481.9 |
  123857.7 |
  125233.6 |
  126609.4 |
  127985.2 |
  129361.0 |
  130736.9 |
  132112.7 |
  133488.5 |
  134864.3 |
  136240.2 |
  137616.0 |
  138991.8 |
  140367.6 |
  141743.5 |
  143119.3 |
  144495.1 |
  145870.9 |########################################
  (4 below, 2 above range)

```
