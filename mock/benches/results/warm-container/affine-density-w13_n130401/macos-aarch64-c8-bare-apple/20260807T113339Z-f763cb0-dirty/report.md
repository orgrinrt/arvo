# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (8.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 317 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel beats baseline by 94% (significant)

warm-container-kernel is -7.69 us (94%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 25.8x slower than the field

warm-container-headroom (8.17 us) is 25.8x the fastest (317 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.84)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-kernel, warm-container-native} vs {warm-container-minimum, warm-container-plusone, warm-container-headroom} (2293% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-kernel, warm-container-native} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-headroom} with a 2293% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 25.8x the fastest

Fastest warm-container-lanes-deferred (317 ns) to slowest warm-container-headroom (8.17 us): 25.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (7 ns, 0.09%)

warm-container-plusone differs from baseline warm-container-headroom by 7 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 317.1 ns median (-96.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 25.77x (fastest 317.1 ns, slowest 8172.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8254ns | 8241ns | 7919ns | 8219ns | 8692ns | base |
| warm-container-kernel | 387ns | 386ns | 375ns | 387ns | 399ns | -95.31% |
| warm-container-lanes-deferred | 380ns | 377ns | 373ns | 378ns | 393ns | -95.39% |
| warm-container-minimum | 8117ns | 8012ns | 7873ns | 8016ns | 8662ns | -1.66% |
| warm-container-native | 440ns | 398ns | 382ns | 410ns | 590ns | -94.66% |
| warm-container-plusone | 8329ns | 8187ns | 7890ns | 8268ns | 8951ns | +0.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8183ns | 7857ns | 8608ns | base | 2.002 |
| warm-container-kernel | 325ns | 315ns | 334ns | -96.03% | 50.430 |
| warm-container-lanes-deferred | 319ns | 313ns | 331ns | -96.10% | 51.283 |
| warm-container-minimum | 8049ns | 7809ns | 8584ns | -1.63% | 2.036 |
| warm-container-native | 364ns | 319ns | 476ns | -95.55% | 45.015 |
| warm-container-plusone | 8256ns | 7826ns | 8878ns | +0.90% | 1.985 |

## Performance model

- Peak throughput: **52.333 Gops/s** (warm-container-lanes-deferred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.005 | 3.8% |
| warm-container-kernel | 50.412 | 96.3% |
| warm-container-lanes-deferred | 51.668 | 98.7% |
| warm-container-minimum | 2.062 | 3.9% |
| warm-container-native | 49.335 | 94.3% |
| warm-container-plusone | 2.021 | 3.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8254ns | 8254ns | base |
| warm-container-kernel | 387ns | 387ns | -95.31% |
| warm-container-lanes-deferred | 380ns | 380ns | -95.39% |
| warm-container-minimum | 8117ns | 8117ns | -1.66% |
| warm-container-native | 440ns | 440ns | -94.66% |
| warm-container-plusone | 8329ns | 8329ns | +0.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8173ns | base | --- | [8041, 8229] | --- | --- | --- | --- |
| warm-container-kernel | 325ns | -7846.7ns (-96.0%) | [-7901, -7723]ns | [318, 332] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 317ns | -7852.1ns (-96.1%) | [-7907, -7728]ns | [315, 320] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 7946ns | -149.2ns (-1.8%) | [-235, -65]ns | [7882, 7987] | YES | 0.0207 | 0.0166 | 0 |
| warm-container-native | 332ns | -7796.2ns (-95.4%) | [-7901, -7660]ns | [324, 360] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8108ns | no significant difference | [-105, +113]ns | [8006, 8236] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8226ns | -96.1% | -95.9% | -4.3% | -95.9% | -3.5% |
| 2 | 8542ns | -96.3% | -96.2% | -7.0% | -96.1% | -8.6% |
| 3 | 8275ns | -96.2% | -96.1% | -2.9% | -93.7% | -5.7% |
| 4 | 8261ns | -96.1% | -96.1% | -3.3% | -92.5% | -4.0% |
| 5 | 8285ns | -96.2% | -96.1% | -4.1% | -96.0% | -5.7% |
| 6 | 8172ns | -96.1% | -96.1% | -3.5% | -93.3% | -3.3% |
| 7 | 8656ns | -96.3% | -96.3% | -9.8% | -96.1% | -7.3% |
| 8 | 8767ns | -96.4% | -96.3% | +24.0% | -96.1% | -6.9% |
| 9 | 8525ns | -96.3% | -96.2% | -8.4% | -93.2% | +1.4% |
| 10 | 8021ns | -96.0% | -96.0% | -2.4% | -95.9% | +6.6% |
| 11 | 8303ns | -96.0% | -96.0% | -0.9% | -96.1% | +5.5% |
| 12 | 8223ns | -96.0% | -95.9% | -1.4% | -96.1% | +0.0% |
| 13 | 8224ns | -95.9% | -96.0% | -1.8% | -96.1% | -0.0% |
| 14 | 8232ns | -95.9% | -96.0% | -3.4% | -96.1% | -0.1% |
| 15 | 8120ns | -95.9% | -95.9% | -2.0% | -96.0% | +4.1% |
| 16 | 8174ns | -95.9% | -96.0% | -1.1% | -95.9% | -0.8% |
| 17 | 8316ns | -96.0% | -96.1% | +1.1% | -96.2% | -1.1% |
| 18 | 8239ns | -95.9% | -96.2% | +1.7% | -96.1% | -1.9% |
| 19 | 8225ns | -96.0% | -96.2% | +0.3% | -96.1% | -1.4% |
| 20 | 8427ns | -96.1% | -96.3% | -2.5% | -96.2% | +6.3% |
| 21 | 7856ns | -95.8% | -96.0% | +3.5% | -95.8% | +0.9% |
| 22 | 7803ns | -95.7% | -96.0% | +0.2% | -95.9% | +2.7% |
| 23 | 7805ns | -95.8% | -96.0% | +0.4% | -95.9% | +5.7% |
| 24 | 7834ns | -95.7% | -96.0% | -0.0% | -95.9% | +1.4% |
| 25 | 8123ns | -95.9% | -96.1% | -3.8% | -96.0% | -3.9% |
| 26 | 7958ns | -95.8% | -96.0% | -1.8% | -95.9% | -1.9% |
| 27 | 8121ns | -95.9% | -96.1% | -3.7% | -96.1% | -3.9% |
| 28 | 8206ns | -96.0% | -96.2% | -2.8% | -96.1% | -2.3% |
| 29 | 8022ns | -95.8% | -96.1% | +1.5% | -96.0% | -0.3% |
| 30 | 7934ns | -95.8% | -96.0% | +0.7% | -95.9% | -0.8% |
| 31 | 8845ns | -96.4% | -96.5% | -9.9% | -95.7% | +0.1% |
| 32 | 8782ns | -96.4% | -96.4% | -11.1% | -95.6% | +0.7% |
| 33 | 7952ns | -96.0% | -96.0% | -1.9% | -95.1% | +11.3% |
| 34 | 8103ns | -96.1% | -96.1% | -3.7% | -95.2% | +9.2% |
| 35 | 8059ns | -96.1% | -96.1% | +1.4% | -95.3% | +9.8% |
| 36 | 7846ns | -96.0% | -96.0% | +0.4% | -95.1% | +12.8% |
| 37 | 7823ns | -96.0% | -95.9% | +1.3% | -95.1% | +3.6% |
| 38 | 8012ns | -96.1% | -96.0% | -0.7% | -95.2% | -0.9% |
| 39 | 8015ns | -96.0% | -96.1% | -1.1% | -95.2% | +9.4% |
| 40 | 7988ns | -96.0% | -96.1% | -1.0% | -95.2% | +12.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.455 | moderate+ |
| warm-container-kernel | 0.839 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.741 | HIGH+ (drift/warm-up) |
| warm-container-minimum | -0.066 | ok |
| warm-container-native | 0.206 | moderate+ |
| warm-container-plusone | 0.540 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 27/40, lost 12/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 20/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.5ns | 8182.6ns | 0.0% |  |
| warm-container-kernel | 2.3ns | 324.9ns | 0.7% |  |
| warm-container-lanes-deferred | 2.7ns | 319.5ns | 0.9% |  |
| warm-container-minimum | 2.8ns | 8049.0ns | 0.0% |  |
| warm-container-native | 3.4ns | 364.0ns | 0.9% |  |
| warm-container-plusone | 3.5ns | 8255.9ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7856.7-8607.6 ns)
   7856.7 |
   7894.3 |
   7931.8 |####################
   7969.4 |######
   8006.9 |##########################
   8044.5 |######
   8082.0 |######
   8119.5 |####################
   8157.1 |#############
   8194.6 |########################################
   8232.2 |#############
   8269.7 |####################
   8307.3 |######
   8344.8 |
   8382.3 |
   8419.9 |######
   8457.4 |
   8495.0 |######
   8532.5 |######
   8570.1 |
  (6 below, 4 above range)

warm-container-kernel (n=40, range 315.2-334.4 ns)
    315.2 |##########################
    316.1 |#################################
    317.1 |##########################
    318.1 |#############
    319.0 |######
    320.0 |######
    320.9 |
    321.9 |
    322.9 |
    323.8 |
    324.8 |
    325.7 |
    326.7 |
    327.6 |
    328.6 |
    329.6 |######
    330.5 |
    331.5 |#################################
    332.4 |########################################
    333.4 |##########################
  (3 below, 4 above range)

warm-container-lanes-deferred (n=40, range 313.1-330.9 ns)
    313.1 |########################################
    314.0 |########################################
    314.9 |##############################
    315.7 |##############################
    316.6 |########################################
    317.5 |####################
    318.4 |##########
    319.3 |##############################
    320.2 |##############################
    321.1 |
    322.0 |
    322.9 |##########
    323.7 |
    324.6 |##########
    325.5 |####################
    326.4 |
    327.3 |
    328.2 |##########
    329.1 |
    330.0 |##########
  (3 below, 4 above range)

warm-container-minimum (n=40, range 7808.6-8584.1 ns)
   7808.6 |########################################
   7847.4 |#################
   7886.1 |#################
   7924.9 |############################
   7963.7 |######################
   8002.5 |#####
   8041.2 |#####
   8080.0 |###########
   8118.8 |###########
   8157.6 |#####
   8196.3 |###########
   8235.1 |#####
   8273.9 |
   8312.7 |
   8351.5 |#####
   8390.2 |#####
   8429.0 |
   8467.8 |
   8506.6 |
   8545.3 |
  (5 below, 1 above range)

warm-container-native (n=40, range 318.8-476.5 ns)
    318.8 |########################################
    326.7 |#####################
    334.6 |###
    342.4 |
    350.3 |
    358.2 |
    366.1 |
    374.0 |######
    381.9 |########################
    389.7 |
    397.6 |
    405.5 |
    413.4 |
    421.3 |
    429.2 |
    437.1 |
    444.9 |
    452.8 |
    460.7 |
    468.6 |
  (5 below, 4 above range)

warm-container-plusone (n=40, range 7826.1-8878.0 ns)
   7826.1 |######
   7878.7 |####################
   7931.3 |####################
   7983.9 |##########################
   8036.5 |######
   8089.1 |####################
   8141.7 |######
   8194.3 |#################################
   8246.9 |
   8299.5 |
   8352.1 |
   8404.7 |######
   8457.3 |
   8509.9 |######
   8562.5 |
   8615.1 |######
   8667.7 |
   8720.3 |#############
   8772.8 |
   8825.4 |########################################
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-container-native**: CV=20.1% (high variance, measurements may be unstable)
- **warm-container-plusone**: autocorrelation=0.54 (measurement drift or warm-up artifact)
