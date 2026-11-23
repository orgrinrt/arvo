# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (3.37 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 392.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 565% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (392.09 us) leads bitpack-write-windowed (2.61 ms) by 565%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 88% (significant)

bitpack-write-dense is -2.98 ms (88%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 8.6x slower than the field

bitpack-write-aligned (3.37 ms) is 8.6x the fastest (392.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 10.8%)

bitpack-write-dense wins on median (392.09 us) yet has the highest variance (CV 10.8%), while bitpack-write-aligned is the steadiest (CV 0.4%, 3.37 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.6x the fastest

Fastest bitpack-write-dense (392.09 us) to slowest bitpack-write-aligned (3.37 ms): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 392086.7 ns median (-88.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.60x (fastest 392086.7 ns, slowest 3370888.9 ns)

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
| bitpack-write-aligned | 3376151ns | 3371304ns | 3366082ns | 3372258ns | 3397902ns | base |
| bitpack-write-dense | 416057ns | 392436ns | 382332ns | 403415ns | 487709ns | -87.68% |
| bitpack-write-windowed | 2617636ns | 2609630ns | 2604046ns | 2609439ns | 2655818ns | -22.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 3375781ns | 3365761ns | 3397531ns | base | 0.621 |
| bitpack-write-dense | 415707ns | 382034ns | 487228ns | -87.69% | 5.045 |
| bitpack-write-windowed | 2617277ns | 2603716ns | 2655454ns | -22.47% | 0.801 |

## Performance model

- Peak throughput: **5.489 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.622 | 11.3% |
| bitpack-write-dense | 5.349 | 97.4% |
| bitpack-write-windowed | 0.804 | 14.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 3376151ns | 3376151ns | base |
| bitpack-write-dense | 416057ns | 416057ns | -87.68% |
| bitpack-write-windowed | 2617636ns | 2617636ns | -22.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 3370889ns | base | --- | [3369516, 3373041] | --- | --- | --- | --- |
| bitpack-write-dense | 392087ns | -2979615.1ns (-88.4%) | [-2982104, -2969376]ns | [386241, 411128] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 2609197ns | -761076.6ns (-22.6%) | [-762592, -758307]ns | [2607872, 2610072] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 3366815ns | -88.5% | -22.5% |
| 1 | 2 | 3370980ns | -88.4% | -22.6% |
| 1 | 3 | 3368691ns | -85.5% | -22.7% |
| 1 | 4 | 3373608ns | -87.8% | -22.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.353 | moderate- |
| bitpack-write-dense | -0.229 | moderate- |
| bitpack-write-windowed | -0.157 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 51.6ns | 3375781.2ns | 0.0% |  |
| bitpack-write-dense | 77.1ns | 415706.7ns | 0.0% |  |
| bitpack-write-windowed | 52.6ns | 2617276.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 3365760.9-3397530.6 ns)
  3365760.9 |########################################
  3367349.4 |#####
  3368937.9 |###################################
  3370526.4 |##############################
  3372114.8 |#####
  3373703.3 |#####
  3375291.8 |###############
  3376880.3 |
  3378468.8 |
  3380057.3 |
  3381645.8 |#####
  3383234.3 |#####
  3384822.7 |
  3386411.2 |##########
  3387999.7 |
  3389588.2 |#####
  3391176.7 |#####
  3392765.2 |
  3394353.7 |#####
  3395942.2 |
  (3 below, 3 above range)

bitpack-write-dense (n=40, range 382034.4-487228.4 ns)
  382034.4 |########################################
  387294.1 |########################
  392553.8 |###
  397813.5 |
  403073.2 |
  408332.9 |##################
  413592.6 |
  418852.3 |
  424112.0 |
  429371.7 |
  434631.4 |
  439891.1 |
  445150.8 |
  450410.5 |
  455670.2 |
  460929.9 |
  466189.6 |
  471449.3 |
  476709.0 |
  481968.7 |#####################
  (2 below, 3 above range)

bitpack-write-windowed (n=40, range 2603716.3-2655454.3 ns)
  2603716.3 |################################
  2606303.2 |#####################
  2608890.1 |########################################
  2611477.0 |#######
  2614063.9 |###
  2616650.8 |#######
  2619237.7 |
  2621824.6 |
  2624411.5 |
  2626998.4 |
  2629585.3 |
  2632172.2 |
  2634759.1 |###
  2637346.0 |
  2639932.9 |
  2642519.8 |
  2645106.7 |###
  2647693.6 |
  2650280.5 |
  2652867.4 |
  (3 below, 4 above range)

```
