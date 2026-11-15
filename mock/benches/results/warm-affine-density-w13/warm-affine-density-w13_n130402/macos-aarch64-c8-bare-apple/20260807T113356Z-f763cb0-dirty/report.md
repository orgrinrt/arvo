# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-native at 395 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 97% (significant)

warm-container-kernel is -8.20 us (97%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 21.4x slower than the field

warm-container-headroom (8.46 us) is 21.4x the fastest (395 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-native (395 ns) and warm-container-lanes-deferred (396 ns) differ by 0.35%, inside the noise, even though the wider field spreads 2043.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.74)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-lanes-deferred, warm-container-kernel} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (1942% apart)

The field splits into a fast tier {warm-container-native, warm-container-lanes-deferred, warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 1942% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 21.4x the fastest

Fastest warm-container-native (395 ns) to slowest warm-container-headroom (8.46 us): 21.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 394.8 ns median (-95.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 21.43x (fastest 394.8 ns, slowest 8460.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8830ns | 8530ns | 8353ns | 8688ns | 9732ns | base |
| warm-container-kernel | 493ns | 479ns | 455ns | 485ns | 554ns | -94.42% |
| warm-container-lanes-deferred | 464ns | 460ns | 453ns | 461ns | 483ns | -94.75% |
| warm-container-minimum | 8595ns | 8483ns | 8350ns | 8458ns | 9251ns | -2.66% |
| warm-container-native | 465ns | 458ns | 451ns | 461ns | 491ns | -94.73% |
| warm-container-plusone | 8516ns | 8482ns | 8348ns | 8475ns | 8808ns | -3.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8760ns | 8288ns | 9650ns | base | 2.805 |
| warm-container-kernel | 426ns | 391ns | 481ns | -95.14% | 57.736 |
| warm-container-lanes-deferred | 400ns | 390ns | 417ns | -95.44% | 61.458 |
| warm-container-minimum | 8527ns | 8285ns | 9178ns | -2.66% | 2.882 |
| warm-container-native | 401ns | 389ns | 424ns | -95.42% | 61.308 |
| warm-container-plusone | 8452ns | 8289ns | 8737ns | -3.52% | 2.908 |

## Performance model

- Peak throughput: **63.123 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 24576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.905 | 4.6% |
| warm-container-kernel | 59.607 | 94.4% |
| warm-container-lanes-deferred | 62.029 | 98.3% |
| warm-container-minimum | 2.920 | 4.6% |
| warm-container-native | 62.249 | 98.6% |
| warm-container-plusone | 2.920 | 4.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8830ns | 8830ns | base |
| warm-container-kernel | 493ns | 493ns | -94.42% |
| warm-container-lanes-deferred | 464ns | 464ns | -94.75% |
| warm-container-minimum | 8595ns | 8595ns | -2.66% |
| warm-container-native | 465ns | 465ns | -94.73% |
| warm-container-plusone | 8516ns | 8516ns | -3.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8460ns | base | --- | [8421, 8700] | --- | --- | --- | --- |
| warm-container-kernel | 412ns | -8045.6ns (-95.1%) | [-8308, -8014]ns | [400, 415] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 396ns | -8059.1ns (-95.3%) | [-8296, -8027]ns | [393, 401] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8417ns | -16.9ns (-0.2%) | [-268, -3]ns | [8306, 8432] | YES | 0.0481 | 0.0385 | 0 |
| warm-container-native | 395ns | -8060.2ns (-95.3%) | [-8301, -8020]ns | [393, 399] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8418ns | no significant difference | [-300, +12]ns | [8344, 8460] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8285ns | -95.2% | -95.2% | +22.1% | -94.5% | +3.8% |
| 2 | 8815ns | -95.5% | -95.4% | +11.7% | -95.3% | -3.4% |
| 3 | 10166ns | -95.1% | -95.9% | -6.3% | -96.0% | -18.4% |
| 4 | 9803ns | -95.5% | -95.9% | -8.2% | -95.8% | -14.3% |
| 5 | 9464ns | -94.8% | -95.6% | -11.1% | -95.7% | -10.8% |
| 6 | 9398ns | -95.8% | -95.6% | -10.3% | -95.6% | -10.1% |
| 7 | 9506ns | -95.9% | -95.7% | -10.9% | -95.7% | -12.8% |
| 8 | 9456ns | -95.9% | -95.6% | -9.8% | -95.6% | -12.1% |
| 9 | 8806ns | -95.6% | -95.3% | -4.4% | -95.3% | -5.9% |
| 10 | 8589ns | -95.5% | -95.2% | -3.2% | -95.2% | -3.5% |
| 11 | 8553ns | -94.4% | -95.4% | -3.1% | -95.4% | +3.8% |
| 12 | 9893ns | -95.2% | -96.0% | -16.1% | -96.1% | -14.7% |
| 13 | 8287ns | -94.3% | -95.2% | -0.1% | -95.3% | +4.1% |
| 14 | 8373ns | -94.3% | -95.3% | -0.8% | -95.3% | +2.7% |
| 15 | 8294ns | -94.3% | -95.3% | -0.1% | -95.3% | -0.1% |
| 16 | 9075ns | -94.8% | -95.7% | -7.1% | -95.7% | -7.9% |
| 17 | 9453ns | -94.9% | -95.9% | -12.2% | -95.9% | -11.2% |
| 18 | 9442ns | -94.9% | -95.9% | -12.0% | -95.9% | -11.3% |
| 19 | 9460ns | -95.0% | -95.8% | -12.4% | -95.9% | -12.2% |
| 20 | 9398ns | -94.9% | -95.9% | -11.0% | -95.9% | -4.1% |
| 21 | 8287ns | -95.2% | -95.3% | -0.0% | -95.3% | +2.1% |
| 22 | 8287ns | -95.2% | -95.3% | +0.1% | -95.3% | +2.2% |
| 23 | 8295ns | -95.2% | -94.7% | -0.1% | -95.3% | +1.4% |
| 24 | 8296ns | -95.1% | -95.3% | -0.1% | -95.3% | -0.1% |
| 25 | 8423ns | -95.2% | -95.4% | -1.6% | -95.3% | -1.6% |
| 26 | 8287ns | -95.3% | -95.3% | -0.1% | -95.3% | +0.0% |
| 27 | 8283ns | -95.2% | -95.3% | +0.0% | -95.3% | +0.6% |
| 28 | 8318ns | -95.2% | -95.3% | +0.9% | -95.3% | -0.1% |
| 29 | 8295ns | -95.2% | -95.3% | +1.5% | -95.2% | -0.1% |
| 30 | 8595ns | -95.4% | -95.4% | -1.2% | -95.5% | -3.2% |
| 31 | 8419ns | -95.1% | -95.3% | +0.5% | -95.3% | -0.0% |
| 32 | 8417ns | -95.1% | -95.2% | +0.1% | -95.3% | +1.2% |
| 33 | 8450ns | -95.1% | -95.2% | -0.3% | -95.2% | -0.4% |
| 34 | 8471ns | -95.1% | -95.3% | -0.5% | -95.3% | +0.0% |
| 35 | 8433ns | -95.1% | -95.2% | -0.1% | -95.3% | -0.1% |
| 36 | 8435ns | -95.1% | -95.3% | +0.2% | -94.6% | +0.3% |
| 37 | 8484ns | -95.1% | -95.3% | +2.3% | -95.1% | +1.1% |
| 38 | 8433ns | -95.1% | -95.3% | +3.8% | -95.3% | +5.0% |
| 39 | 8441ns | -95.1% | -95.3% | +3.8% | -95.3% | +3.6% |
| 40 | 8546ns | -95.2% | -95.4% | +2.2% | -95.4% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.596 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.593 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.303 | moderate+ |
| warm-container-minimum | 0.737 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.402 | moderate+ |
| warm-container-plusone | 0.259 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 23/40, lost 11/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 19/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.1ns | 8760.3ns | 0.0% |  |
| warm-container-kernel | 2.7ns | 425.7ns | 0.6% |  |
| warm-container-lanes-deferred | 2.3ns | 399.9ns | 0.6% |  |
| warm-container-minimum | 2.6ns | 8527.0ns | 0.0% |  |
| warm-container-native | 2.5ns | 400.9ns | 0.6% |  |
| warm-container-plusone | 2.8ns | 8451.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8288.1-9650.2 ns)
   8288.1 |############################
   8356.2 |######################
   8424.3 |########################################
   8492.4 |###########
   8560.5 |###########
   8628.6 |
   8696.7 |
   8764.8 |###########
   8832.9 |
   8901.1 |
   8969.2 |
   9037.3 |#####
   9105.4 |
   9173.5 |
   9241.6 |
   9309.7 |
   9377.8 |#################
   9445.9 |############################
   9514.0 |
   9582.1 |
  (6 below, 3 above range)

warm-container-kernel (n=40, range 391.0-480.8 ns)
    391.0 |#################
    395.5 |############################
    400.0 |######################
    404.4 |
    408.9 |########################################
    413.4 |#################
    417.9 |
    422.4 |
    426.9 |
    431.4 |
    435.9 |#####
    440.4 |
    444.8 |
    449.3 |
    453.8 |
    458.3 |
    462.8 |
    467.3 |#####
    471.8 |########################################
    476.3 |###########
  (5 below, 2 above range)

warm-container-lanes-deferred (n=40, range 390.5-416.6 ns)
    390.5 |########################################
    391.8 |##########################
    393.1 |#############
    394.4 |####################
    395.7 |#############
    397.0 |######
    398.3 |####################
    399.6 |
    400.9 |#############
    402.2 |#############
    403.5 |######
    404.9 |######
    406.2 |
    407.5 |######
    408.8 |
    410.1 |
    411.4 |####################
    412.7 |####################
    414.0 |
    415.3 |######
  (4 below, 1 above range)

warm-container-minimum (n=40, range 8284.6-9177.8 ns)
   8284.6 |########################################
   8329.2 |##
   8373.9 |###########
   8418.5 |####################
   8463.2 |########
   8507.9 |##
   8552.5 |
   8597.2 |
   8641.9 |##
   8686.5 |
   8731.2 |########
   8775.8 |
   8820.5 |
   8865.2 |
   8909.8 |
   8954.5 |
   8999.1 |##
   9043.8 |
   9088.5 |
   9133.1 |
  (2 below, 3 above range)

warm-container-native (n=40, range 389.3-423.8 ns)
    389.3 |########################################
    391.1 |#####
    392.8 |#########################
    394.5 |#########################
    396.2 |##########
    398.0 |##########
    399.7 |
    401.4 |#####
    403.1 |
    404.8 |
    406.6 |#####
    408.3 |#####
    410.0 |####################
    411.7 |####################
    413.5 |
    415.2 |
    416.9 |
    418.6 |
    420.4 |
    422.1 |
  (4 below, 2 above range)

warm-container-plusone (n=40, range 8289.0-8736.9 ns)
   8289.0 |########################################
   8311.4 |#############
   8333.8 |
   8356.1 |########
   8378.5 |########
   8400.9 |#################
   8423.3 |#############
   8445.7 |#############
   8468.1 |####
   8490.5 |
   8512.9 |########
   8535.3 |
   8557.7 |########
   8580.1 |########
   8602.5 |
   8624.9 |####
   8647.3 |
   8669.7 |
   8692.1 |
   8714.5 |
  (2 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.74 (measurement drift or warm-up artifact)
