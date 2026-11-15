# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (20.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 9.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 53% (significant)

warm-container-minimum is -10.89 us (53%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.2x slower than the field

warm-container-headroom (20.54 us) is 2.2x the fastest (9.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (9.43 us) and warm-container-native (9.47 us) differ by 0.45%, inside the noise, even though the wider field spreads 117.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.70)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (114% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 114% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-container-minimum** at 9430.7 ns median (-54.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.18x (fastest 9430.7 ns, slowest 20541.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 20758ns | 20640ns | 20256ns | 20656ns | 21567ns | base |
| warm-container-kernel | 9700ns | 9635ns | 9354ns | 9644ns | 10214ns | -53.27% |
| warm-container-minimum | 9565ns | 9497ns | 9345ns | 9525ns | 9903ns | -53.92% |
| warm-container-native | 9578ns | 9536ns | 9364ns | 9561ns | 9844ns | -53.86% |
| warm-container-plusone | 20722ns | 20571ns | 20199ns | 20608ns | 21586ns | -0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 20646ns | 20159ns | 21426ns | base | 6.745 |
| warm-container-kernel | 9630ns | 9293ns | 10130ns | -53.36% | 14.461 |
| warm-container-minimum | 9495ns | 9283ns | 9816ns | -54.01% | 14.668 |
| warm-container-native | 9512ns | 9305ns | 9773ns | -53.93% | 14.642 |
| warm-container-plusone | 20613ns | 20100ns | 21464ns | -0.16% | 6.756 |

## Performance model

- Peak throughput: **15.003 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 6.780 | 45.2% |
| warm-container-kernel | 14.554 | 97.0% |
| warm-container-minimum | 14.767 | 98.4% |
| warm-container-native | 14.701 | 98.0% |
| warm-container-plusone | 6.802 | 45.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 20758ns | 20758ns | base |
| warm-container-kernel | 9700ns | 9700ns | -53.27% |
| warm-container-minimum | 9565ns | 9565ns | -53.92% |
| warm-container-native | 9578ns | 9578ns | -53.86% |
| warm-container-plusone | 20722ns | 20722ns | -0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 20542ns | base | --- | [20455, 20634] | --- | --- | --- | --- |
| warm-container-kernel | 9569ns | -11049.2ns (-53.8%) | [-11181, -10860]ns | [9410, 9777] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 9431ns | -11133.6ns (-54.2%) | [-11289, -10971]ns | [9372, 9558] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 9473ns | -10994.9ns (-53.5%) | [-11260, -10807]ns | [9443, 9519] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 20475ns | no significant difference | [-248, +94]ns | [20333, 20563] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 20069ns | -52.8% | -51.6% | -53.0% | +2.0% |
| 2 | 20243ns | -51.6% | -52.7% | -53.2% | +1.1% |
| 3 | 20134ns | -49.2% | -53.4% | -53.3% | +0.2% |
| 4 | 20538ns | -52.3% | -54.1% | -54.2% | -1.8% |
| 5 | 20103ns | -51.3% | -52.4% | -53.0% | +2.9% |
| 6 | 20476ns | -49.9% | -54.3% | -52.5% | +0.2% |
| 7 | 21146ns | -53.6% | -55.8% | -54.4% | -4.6% |
| 8 | 20546ns | -48.8% | -54.1% | -52.3% | -2.2% |
| 9 | 20556ns | -51.9% | -54.0% | -52.4% | -2.1% |
| 10 | 20298ns | -52.9% | -53.3% | -51.2% | -1.0% |
| 11 | 20346ns | -53.6% | -52.0% | -52.0% | +0.7% |
| 12 | 20389ns | -53.0% | -51.5% | -52.0% | +0.4% |
| 13 | 20500ns | -51.9% | -52.4% | -52.5% | -0.1% |
| 14 | 20407ns | -52.0% | -52.2% | -53.0% | -0.0% |
| 15 | 21262ns | -53.9% | -54.1% | -55.6% | -3.5% |
| 16 | 21674ns | -51.0% | -54.9% | -56.3% | -2.6% |
| 17 | 21265ns | -54.1% | -52.6% | -55.4% | -0.8% |
| 18 | 21166ns | -54.5% | -54.6% | -55.2% | +0.5% |
| 19 | 21475ns | -54.5% | -56.1% | -55.9% | -1.4% |
| 20 | 20772ns | -52.9% | -54.4% | -54.3% | +1.6% |
| 21 | 20948ns | -53.4% | -55.7% | -54.2% | -4.0% |
| 22 | 20655ns | -53.5% | -54.6% | -53.8% | -2.5% |
| 23 | 20491ns | -54.0% | -54.7% | -53.3% | -1.6% |
| 24 | 20432ns | -54.1% | -54.6% | -53.8% | -0.4% |
| 25 | 20233ns | -54.1% | -53.4% | -53.1% | +4.8% |
| 26 | 21802ns | -57.1% | -56.2% | -56.8% | -2.9% |
| 27 | 21618ns | -54.4% | -55.8% | -56.1% | -7.0% |
| 28 | 20274ns | -53.5% | -52.3% | -53.3% | +2.5% |
| 29 | 20565ns | -54.5% | -52.7% | -53.0% | +8.1% |
| 30 | 20462ns | -53.8% | -54.2% | -54.3% | +2.5% |
| 31 | 20583ns | -54.7% | -54.9% | -53.9% | -0.3% |
| 32 | 20612ns | -54.8% | -54.9% | -54.9% | -1.5% |
| 33 | 20712ns | -55.3% | -54.7% | -55.3% | -3.0% |
| 34 | 20051ns | -53.7% | -53.6% | -53.8% | -0.0% |
| 35 | 20681ns | -55.2% | -55.1% | -55.1% | +8.5% |
| 36 | 20690ns | -55.0% | -55.2% | -55.0% | +1.5% |
| 37 | 20562ns | -54.4% | -54.2% | -54.6% | -0.6% |
| 38 | 20496ns | -54.4% | -54.7% | -52.7% | -0.3% |
| 39 | 20448ns | -54.2% | -54.3% | -53.8% | +0.7% |
| 40 | 20167ns | -54.0% | -53.6% | -53.7% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.422 | moderate+ |
| warm-container-kernel | 0.531 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.699 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.684 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.288 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 21/40, lost 17/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.8ns | 20646.2ns | 0.0% |  |
| warm-container-kernel | 3.5ns | 9630.2ns | 0.0% |  |
| warm-container-minimum | 3.1ns | 9494.7ns | 0.0% |  |
| warm-container-native | 3.2ns | 9511.5ns | 0.0% |  |
| warm-container-plusone | 3.4ns | 20613.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 20159.3-21426.0 ns)
  20159.3 |########
  20222.6 |########################
  20285.9 |################
  20349.3 |################
  20412.6 |########################
  20476.0 |########################################
  20539.3 |########################################
  20602.6 |################
  20666.0 |########################
  20729.3 |########
  20792.6 |
  20856.0 |
  20919.3 |########
  20982.6 |
  21046.0 |
  21109.3 |################
  21172.6 |
  21236.0 |################
  21299.3 |
  21362.7 |
  (4 below, 4 above range)

warm-container-kernel (n=40, range 9293.4-10130.1 ns)
   9293.4 |###############
   9335.2 |####################
   9377.1 |##########
   9418.9 |####################
   9460.7 |#####
   9502.6 |
   9544.4 |##########
   9586.3 |##########
   9628.1 |
   9669.9 |
   9711.8 |
   9753.6 |########################################
   9795.4 |##########
   9837.3 |##########
   9879.1 |#####
   9921.0 |
   9962.8 |
  10004.6 |
  10046.5 |
  10088.3 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 9282.6-9815.9 ns)
   9282.6 |########################################
   9309.3 |
   9335.9 |########################################
   9362.6 |########################################
   9389.3 |##########
   9415.9 |########################################
   9442.6 |####################
   9469.3 |##########
   9495.9 |
   9522.6 |
   9549.3 |########################################
   9575.9 |
   9602.6 |##########
   9629.3 |
   9655.9 |##########
   9682.6 |
   9709.2 |####################
   9735.9 |####################
   9762.6 |##############################
   9789.2 |
  (5 below, 2 above range)

warm-container-native (n=40, range 9305.1-9773.5 ns)
   9305.1 |################
   9328.6 |########
   9352.0 |########
   9375.4 |
   9398.8 |########################
   9422.2 |########################
   9445.7 |########################################
   9469.1 |########################################
   9492.5 |################
   9515.9 |########
   9539.3 |########
   9562.7 |########
   9586.2 |########
   9609.6 |
   9633.0 |########
   9656.4 |########
   9679.8 |########
   9703.2 |########
   9726.7 |########
   9750.1 |
  (4 below, 5 above range)

warm-container-plusone (n=40, range 20100.2-21464.4 ns)
  20100.2 |########################################
  20168.4 |#############
  20236.6 |######
  20304.8 |#############
  20373.0 |#############
  20441.3 |########################################
  20509.5 |####################
  20577.7 |######
  20645.9 |######
  20714.1 |######
  20782.3 |
  20850.5 |
  20918.7 |######
  20986.9 |######
  21055.2 |####################
  21123.4 |#############
  21191.6 |######
  21259.8 |######
  21328.0 |
  21396.2 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.68 (measurement drift or warm-up artifact)
