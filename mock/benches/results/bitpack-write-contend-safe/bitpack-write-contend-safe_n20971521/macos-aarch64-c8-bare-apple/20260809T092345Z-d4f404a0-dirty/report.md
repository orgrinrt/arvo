# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (6.30 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 291.69 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1159% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (291.69 us) leads bitpack-write-windowed (3.67 ms) by 1159%, a clear separation rather than a photo finish. CV 9.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 95% (significant)

bitpack-write-dense is -5.99 ms (95%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 21.6x slower than the field

bitpack-write-aligned (6.30 ms) is 21.6x the fastest (291.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 9.2%)

bitpack-write-dense wins on median (291.69 us) yet has the highest variance (CV 9.2%), while bitpack-write-windowed is the steadiest (CV 6.6%, 3.67 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.78)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.6x the fastest

Fastest bitpack-write-dense (291.69 us) to slowest bitpack-write-aligned (6.30 ms): 21.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 291690.2 ns median (-95.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.59x (fastest 291690.2 ns, slowest 6297861.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6623491ns | 6299640ns | 6226650ns | 6466988ns | 7489844ns | base |
| bitpack-write-dense | 301497ns | 292076ns | 275580ns | 294477ns | 348473ns | -95.45% |
| bitpack-write-windowed | 3785956ns | 3673058ns | 3653228ns | 3692560ns | 4198873ns | -42.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 6621623ns | 6225217ns | 7487365ns | base | 0.317 |
| bitpack-write-dense | 301051ns | 275234ns | 347805ns | -95.45% | 6.966 |
| bitpack-write-windowed | 3784404ns | 3651957ns | 4196544ns | -42.85% | 0.554 |

## Performance model

- Peak throughput: **7.620 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.333 | 4.4% |
| bitpack-write-dense | 7.190 | 94.4% |
| bitpack-write-windowed | 0.571 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 6623491ns | 6623491ns | base |
| bitpack-write-dense | 301497ns | 301497ns | -95.45% |
| bitpack-write-windowed | 3785956ns | 3785956ns | -42.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6297861ns | base | --- | [6268696, 6420481] | --- | --- | --- | --- |
| bitpack-write-dense | 291690ns | -6016040.2ns (-95.5%) | [-6137654, -5984854]ns | [284959, 299705] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 3671753ns | -2628612.9ns (-41.7%) | [-2687735, -2584859]ns | [3663397, 3699172] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|
| 1 | 7327315ns | -95.6% | -42.9% |
| 2 | 7294402ns | -95.2% | -44.0% |
| 3 | 7305235ns | -95.3% | -43.8% |
| 4 | 7417149ns | -95.0% | -46.6% |
| 5 | 7140405ns | -95.1% | -48.8% |
| 6 | 7363888ns | -95.5% | -50.3% |
| 7 | 7455014ns | -95.7% | -50.1% |
| 8 | 7339263ns | -95.5% | -50.1% |
| 9 | 7444610ns | -95.4% | -50.8% |
| 10 | 8236886ns | -95.6% | -55.3% |
| 11 | 6253337ns | -95.6% | -41.5% |
| 12 | 6218850ns | -95.4% | -40.4% |
| 13 | 6297708ns | -95.6% | -41.7% |
| 14 | 6292507ns | -95.6% | -42.1% |
| 15 | 6227407ns | -95.5% | -41.2% |
| 16 | 6375951ns | -95.6% | -42.8% |
| 17 | 6430740ns | -95.8% | -42.9% |
| 18 | 7021411ns | -96.1% | -48.1% |
| 19 | 6290657ns | -95.7% | -42.0% |
| 20 | 6245151ns | -95.4% | -41.3% |
| 21 | 6276004ns | -95.4% | -37.5% |
| 22 | 6410222ns | -95.4% | -37.1% |
| 23 | 6310948ns | -95.3% | -27.1% |
| 24 | 7314790ns | -95.9% | -36.0% |
| 25 | 6713908ns | -95.6% | -43.5% |
| 26 | 6310082ns | -95.3% | -41.6% |
| 27 | 6231184ns | -95.4% | -41.4% |
| 28 | 6309310ns | -95.4% | -41.9% |
| 29 | 6220118ns | -95.5% | -41.0% |
| 30 | 6298015ns | -95.5% | -41.9% |
| 31 | 6247540ns | -95.4% | -41.3% |
| 32 | 6235099ns | -95.3% | -41.1% |
| 33 | 6294160ns | -95.4% | -41.8% |
| 34 | 6250104ns | -95.1% | -41.4% |
| 35 | 6215750ns | -95.1% | -39.9% |
| 36 | 6249293ns | -95.0% | -41.4% |
| 37 | 6285807ns | -95.3% | -41.3% |
| 38 | 6215664ns | -95.5% | -39.0% |
| 39 | 6237667ns | -95.6% | -39.3% |
| 40 | 6261388ns | -95.6% | -41.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.651 | HIGH+ (drift/warm-up) |
| bitpack-write-dense | 0.782 | HIGH+ (drift/warm-up) |
| bitpack-write-windowed | 0.668 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-write-dense**: won 40/40, lost 0/40
- **bitpack-write-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 109.6ns | 6621623.5ns | 0.0% |  |
| bitpack-write-dense | 17.3ns | 301050.7ns | 0.0% |  |
| bitpack-write-windowed | 60.4ns | 3784403.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 6225217.4-7487364.5 ns)
  6225217.4 |########################################
  6288324.8 |##########################
  6351432.1 |######
  6414539.5 |###
  6477646.8 |
  6540754.2 |
  6603861.5 |
  6666968.9 |###
  6730076.3 |
  6793183.6 |
  6856291.0 |
  6919398.3 |
  6982505.7 |###
  7045613.0 |
  7108720.4 |###
  7171827.7 |
  7234935.1 |###
  7298042.5 |#############
  7361149.8 |######
  7424257.2 |######
  (4 below, 1 above range)

bitpack-write-dense (n=40, range 275234.0-347804.8 ns)
  275234.0 |########################################
  278862.6 |######################
  282491.1 |###########
  286119.7 |#################
  289748.2 |#################
  293376.7 |#################
  297005.3 |###########
  300633.8 |###########
  304262.4 |#####
  307890.9 |
  311519.4 |#####
  315148.0 |
  318776.5 |
  322405.0 |###########
  326033.6 |#####
  329662.1 |
  333290.7 |#####
  336919.2 |#####
  340547.7 |#####
  344176.3 |
  (2 below, 4 above range)

bitpack-write-windowed (n=40, range 3651957.2-4196544.0 ns)
  3651957.2 |########################################
  3679186.5 |######
  3706415.9 |####
  3733645.2 |##
  3760874.5 |##
  3788103.9 |####
  3815333.2 |
  3842562.6 |
  3869791.9 |
  3897021.2 |##
  3924250.6 |
  3951479.9 |##
  3978709.2 |
  4005938.6 |##
  4033167.9 |
  4060397.3 |##
  4087626.6 |##
  4114855.9 |
  4142085.3 |
  4169314.6 |##
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-write-aligned**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **bitpack-write-dense**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **bitpack-write-windowed**: autocorrelation=0.67 (measurement drift or warm-up artifact)
