# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.89 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 391 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 90% (significant)

warm-container-lanes-deferred is -7.99 us (90%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 22.7x slower than the field

warm-container-headroom (8.89 us) is 22.7x the fastest (391 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-lanes-deferred, warm-container-native) are a dead heat (<1%)

warm-container-lanes-deferred (391 ns) and warm-container-native (392 ns) differ by 0.28%, inside the noise, even though the wider field spreads 2172.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.92)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.92, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-native, warm-container-minimum, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (1782% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-native, warm-container-minimum, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 1782% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.7x the fastest

Fastest warm-container-lanes-deferred (391 ns) to slowest warm-container-headroom (8.89 us): 22.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 391.2 ns median (-95.6% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 22.72x (fastest 391.2 ns, slowest 8889.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 9141ns | 8958ns | 8357ns | 9021ns | 10286ns | base |
| warm-container-kernel | 505ns | 513ns | 451ns | 508ns | 551ns | -94.48% |
| warm-container-lanes-deferred | 453ns | 452ns | 451ns | 453ns | 456ns | -95.05% |
| warm-container-minimum | 507ns | 512ns | 453ns | 496ns | 596ns | -94.45% |
| warm-container-native | 488ns | 454ns | 451ns | 465ns | 596ns | -94.66% |
| warm-container-plusone | 8535ns | 8392ns | 8347ns | 8467ns | 8926ns | -6.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 9073ns | 8298ns | 10210ns | base | 3.611 |
| warm-container-kernel | 437ns | 390ns | 478ns | -95.19% | 75.017 |
| warm-container-lanes-deferred | 391ns | 388ns | 394ns | -95.69% | 83.745 |
| warm-container-minimum | 436ns | 390ns | 516ns | -95.20% | 75.183 |
| warm-container-native | 422ns | 389ns | 515ns | -95.35% | 77.615 |
| warm-container-plusone | 8468ns | 8283ns | 8858ns | -6.68% | 3.870 |

## Performance model

- Peak throughput: **84.470 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.686 | 4.4% |
| warm-container-kernel | 74.052 | 87.7% |
| warm-container-lanes-deferred | 83.763 | 99.2% |
| warm-container-minimum | 78.524 | 93.0% |
| warm-container-native | 83.528 | 98.9% |
| warm-container-plusone | 3.935 | 4.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 9141ns | 9141ns | base |
| warm-container-kernel | 505ns | 505ns | -94.48% |
| warm-container-lanes-deferred | 453ns | 453ns | -95.05% |
| warm-container-minimum | 507ns | 507ns | -94.45% |
| warm-container-native | 488ns | 488ns | -94.66% |
| warm-container-plusone | 8535ns | 8535ns | -6.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8889ns | base | --- | [8412, 9395] | --- | --- | --- | --- |
| warm-container-kernel | 442ns | -8428.1ns (-94.8%) | [-8935, -7983]ns | [418, 471] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 391ns | -8499.8ns (-95.6%) | [-9003, -8022]ns | [390, 392] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 417ns | -8432.9ns (-94.9%) | [-9003, -8005]ns | [393, 445] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 392ns | -8435.8ns (-94.9%) | [-9004, -8019]ns | [391, 394] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8328ns | -371.9ns (-4.2%) | [-1047, -41]ns | [8293, 8441] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8447ns | -94.7% | -95.4% | -95.4% | -95.4% | -1.9% |
| 2 | 8830ns | -95.0% | -95.6% | -95.5% | -95.5% | -6.1% |
| 3 | 8345ns | -94.7% | -95.3% | -95.3% | -95.3% | -0.5% |
| 4 | 8283ns | -94.7% | -95.3% | -95.3% | -95.3% | +0.7% |
| 5 | 8285ns | -94.7% | -95.3% | -95.3% | -95.3% | +0.1% |
| 6 | 8284ns | -94.7% | -95.3% | -95.3% | -95.3% | +0.1% |
| 7 | 8338ns | -94.7% | -95.3% | -95.3% | -95.3% | -0.5% |
| 8 | 9363ns | -95.3% | -95.9% | -95.8% | -95.8% | -9.8% |
| 9 | 8597ns | -94.8% | -95.4% | -95.5% | -95.4% | +1.0% |
| 10 | 8424ns | -94.7% | -95.4% | -95.4% | -95.3% | +0.2% |
| 11 | 8375ns | -95.3% | -95.3% | -94.7% | -95.4% | +0.7% |
| 12 | 9148ns | -95.7% | -95.7% | -95.1% | -95.7% | -9.4% |
| 13 | 10049ns | -96.1% | -96.1% | -95.6% | -96.1% | -17.5% |
| 14 | 10045ns | -96.1% | -96.1% | -95.6% | -96.1% | -17.5% |
| 15 | 10052ns | -96.1% | -96.1% | -95.5% | -96.1% | -17.6% |
| 16 | 9735ns | -96.0% | -96.0% | -95.4% | -96.0% | -14.8% |
| 17 | 8367ns | -95.3% | -95.3% | -94.7% | -95.3% | -0.6% |
| 18 | 8287ns | -95.3% | -95.3% | -94.6% | -95.3% | +0.0% |
| 19 | 8280ns | -95.3% | -95.3% | -94.7% | -95.2% | +0.0% |
| 20 | 8401ns | -95.3% | -95.3% | -94.8% | -95.3% | -0.7% |
| 21 | 8370ns | -95.4% | -95.3% | -95.3% | -95.3% | -0.1% |
| 22 | 8284ns | -95.3% | -95.2% | -95.3% | -95.3% | +0.2% |
| 23 | 9080ns | -95.7% | -95.7% | -95.7% | -95.7% | -8.8% |
| 24 | 9397ns | -95.8% | -95.8% | -95.8% | -95.8% | -11.9% |
| 25 | 9394ns | -95.3% | -95.8% | -95.8% | -95.8% | -11.9% |
| 26 | 9398ns | -94.9% | -95.8% | -95.8% | -95.9% | -11.9% |
| 27 | 9423ns | -94.9% | -95.9% | -95.8% | -95.9% | -10.4% |
| 28 | 9178ns | -94.8% | -95.7% | -95.7% | -95.7% | -9.7% |
| 29 | 8432ns | -94.4% | -95.3% | -95.4% | -95.3% | -0.5% |
| 30 | 8362ns | -94.4% | -95.3% | -95.3% | -95.4% | +6.3% |
| 31 | 10899ns | -95.7% | -96.4% | -95.3% | -95.3% | -20.0% |
| 32 | 10461ns | -95.5% | -96.3% | -95.1% | -95.1% | -16.2% |
| 33 | 10048ns | -95.2% | -96.1% | -94.9% | -94.9% | -12.8% |
| 34 | 10044ns | -95.3% | -96.1% | -94.9% | -94.9% | -13.2% |
| 35 | 10073ns | -95.3% | -96.1% | -94.9% | -94.9% | -13.4% |
| 36 | 10045ns | -95.2% | -96.1% | -94.8% | -94.9% | -13.1% |
| 37 | 10049ns | -95.3% | -96.1% | -94.8% | -94.9% | -13.1% |
| 38 | 8948ns | -94.6% | -95.6% | -94.2% | -94.3% | -2.3% |
| 39 | 8755ns | -94.6% | -95.5% | -94.1% | -94.1% | +0.6% |
| 40 | 8367ns | -94.3% | -95.3% | -93.8% | -93.9% | +12.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.658 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.918 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | -0.198 | ok |
| warm-container-minimum | 0.865 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.885 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.629 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 28/40, lost 8/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.0ns | 9073.5ns | 0.0% |  |
| warm-container-kernel | 3.5ns | 436.8ns | 0.8% |  |
| warm-container-lanes-deferred | 2.8ns | 391.3ns | 0.7% |  |
| warm-container-minimum | 3.1ns | 435.8ns | 0.7% |  |
| warm-container-native | 2.9ns | 422.2ns | 0.7% |  |
| warm-container-plusone | 2.7ns | 8467.6ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8298.2-10209.5 ns)
   8298.2 |###################################
   8393.8 |####################
   8489.4 |
   8584.9 |#####
   8680.5 |#####
   8776.1 |#####
   8871.6 |#####
   8967.2 |
   9062.7 |##########
   9158.3 |#####
   9253.9 |
   9349.4 |#########################
   9445.0 |
   9540.6 |
   9636.1 |
   9731.7 |#####
   9827.3 |
   9922.8 |
  10018.4 |########################################
  10113.9 |
  (6 below, 2 above range)

warm-container-kernel (n=40, range 389.7-477.5 ns)
    389.7 |########################################
    394.1 |####
    398.5 |
    402.9 |
    407.3 |
    411.7 |
    416.1 |
    420.4 |
    424.8 |
    429.2 |
    433.6 |
    438.0 |################
    442.4 |############################
    446.8 |
    451.2 |
    455.6 |
    459.9 |
    464.3 |
    468.7 |####################
    473.1 |########################
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 387.9-394.4 ns)
    387.9 |
    388.2 |########
    388.6 |########
    388.9 |
    389.2 |
    389.5 |################
    389.9 |########################
    390.2 |########################
    390.5 |################
    390.8 |
    391.1 |################################
    391.5 |################
    391.8 |########################################
    392.1 |
    392.4 |
    392.8 |########################
    393.1 |########
    393.4 |########################
    393.7 |
    394.0 |########
  (5 below, 4 above range)

warm-container-minimum (n=40, range 390.2-516.0 ns)
    390.2 |########################################
    396.5 |
    402.8 |
    409.1 |
    415.4 |
    421.7 |
    428.0 |
    434.3 |##
    440.5 |####################
    446.8 |##
    453.1 |
    459.4 |
    465.7 |
    472.0 |
    478.3 |
    484.6 |
    490.8 |
    497.1 |
    503.4 |
    509.7 |#################
  (4 below, 3 above range)

warm-container-native (n=40, range 388.9-515.0 ns)
    388.9 |########################################
    395.2 |
    401.5 |
    407.8 |
    414.1 |
    420.4 |
    426.7 |
    433.0 |
    439.3 |
    445.6 |
    451.9 |
    458.2 |
    464.6 |
    470.9 |
    477.2 |
    483.5 |
    489.8 |
    496.1 |
    502.4 |
    508.7 |########
  (3 below, 4 above range)

warm-container-plusone (n=40, range 8283.1-8858.2 ns)
   8283.1 |########################################
   8311.8 |##
   8340.6 |#######
   8369.3 |##
   8398.1 |
   8426.8 |#########
   8455.6 |
   8484.4 |
   8513.1 |
   8541.9 |
   8570.6 |
   8599.4 |
   8628.1 |
   8656.9 |##
   8685.6 |
   8714.4 |###########
   8743.2 |#######
   8771.9 |
   8800.7 |##
   8829.4 |
  (2 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.92 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.63 (measurement drift or warm-up artifact)
