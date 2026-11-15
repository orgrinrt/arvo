# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (153 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-min-lanes at 69 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-minimum beats baseline by 56% (significant)

warm-clamp-minimum is -85 ns (56%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.2x slower than the field

warm-clamp-acc64 (153 ns) is 2.2x the fastest (69 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes is fastest but the noisiest (CV 24.7%)

warm-clamp-min-lanes wins on median (69 ns) yet has the highest variance (CV 24.7%), while warm-clamp-accfit is the steadiest (CV 1.1%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-min-lanes, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-min-lanes (69 ns) and warm-clamp-minimum (69 ns) differ by 0.00%, inside the noise, even though the wider field spreads 122.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows warm-up / thermal drift (autocorr +0.88)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum} vs {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64} (118% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum} and a slow tier {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64} with a 118% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit's comparison is tie-heavy (10% tied pairs)

10% of paired samples for warm-clamp-accfit are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### warm-clamp-head's edge over baseline is significant but tiny (-2 ns, 1.38%)

warm-clamp-head differs from baseline warm-clamp-acc64 by -2 ns (1.38%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 68.7 ns median (-55.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.22x (fastest 68.7 ns, slowest 152.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 228ns | 213ns | 208ns | 218ns | 277ns | base |
| warm-clamp-accfit | 210ns | 210ns | 207ns | 210ns | 214ns | -7.71% |
| warm-clamp-accfit-dyn | 222ns | 212ns | 208ns | 213ns | 263ns | -2.81% |
| warm-clamp-head | 211ns | 211ns | 208ns | 211ns | 215ns | -7.26% |
| warm-clamp-min-lanes | 133ns | 131ns | 128ns | 131ns | 147ns | -41.51% |
| warm-clamp-minimum | 131ns | 131ns | 128ns | 131ns | 134ns | -42.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 163ns | 148ns | 198ns | base | 50.301 |
| warm-clamp-accfit | 150ns | 148ns | 153ns | -7.86% | 54.590 |
| warm-clamp-accfit-dyn | 158ns | 148ns | 187ns | -3.24% | 51.983 |
| warm-clamp-head | 151ns | 148ns | 155ns | -7.19% | 54.198 |
| warm-clamp-min-lanes | 71ns | 66ns | 85ns | -56.11% | 114.609 |
| warm-clamp-minimum | 69ns | 66ns | 71ns | -57.85% | 119.334 |

## Performance model

- Peak throughput: **124.310 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 53.648 | 43.2% |
| warm-clamp-accfit | 54.613 | 43.9% |
| warm-clamp-accfit-dyn | 54.090 | 43.5% |
| warm-clamp-head | 54.252 | 43.6% |
| warm-clamp-min-lanes | 119.243 | 95.9% |
| warm-clamp-minimum | 119.243 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 228ns | 228ns | base |
| warm-clamp-accfit | 210ns | 210ns | -7.71% |
| warm-clamp-accfit-dyn | 222ns | 222ns | -2.81% |
| warm-clamp-head | 211ns | 211ns | -7.26% |
| warm-clamp-min-lanes | 133ns | 133ns | -41.51% |
| warm-clamp-minimum | 131ns | 131ns | -42.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 153ns | base | --- | [151, 154] | --- | --- | --- | --- |
| warm-clamp-accfit | 150ns | -2.3ns (-1.5%) | [-5, -0]ns | [149, 151] | YES | 0.0231 | 0.0139 | 2 |
| warm-clamp-accfit-dyn | 151ns | no significant difference | [-5, +0]ns | [149, 153] | no | 0.1081 | 0.1081 | 1 |
| warm-clamp-head | 151ns | no significant difference | [-4, +0]ns | [150, 152] | no | 0.1081 | 0.1081 | 1 |
| warm-clamp-min-lanes | 69ns | -83.8ns (-54.8%) | [-86, -82]ns | [68, 70] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 69ns | -83.6ns (-54.7%) | [-86, -82]ns | [68, 70] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 150ns | +0.3% | -1.1% | +0.9% | -57.0% | -54.6% |
| 2 | 149ns | +0.5% | +3.4% | -1.1% | -55.0% | -55.3% |
| 3 | 149ns | +0.0% | +2.2% | +3.6% | -51.4% | -54.8% |
| 4 | 149ns | +1.7% | +2.5% | -0.9% | -56.4% | -53.1% |
| 5 | 150ns | +2.5% | +2.8% | -0.3% | -51.8% | -52.9% |
| 6 | 149ns | -0.6% | +0.8% | +0.5% | -55.0% | -52.8% |
| 7 | 149ns | +0.9% | +2.0% | +2.3% | -52.9% | -53.5% |
| 8 | 147ns | +3.9% | +1.7% | +2.8% | -53.0% | -52.4% |
| 9 | 152ns | +0.5% | +0.3% | +0.3% | -53.1% | -56.4% |
| 10 | 150ns | +0.8% | -0.3% | -0.9% | -54.5% | -53.9% |
| 11 | 153ns | -4.0% | +40.5% | -1.0% | -54.3% | -52.7% |
| 12 | 157ns | -3.2% | +37.2% | -3.8% | -57.2% | -53.7% |
| 13 | 155ns | -1.1% | +39.2% | -2.7% | -56.7% | -58.6% |
| 14 | 153ns | -2.4% | +42.8% | -1.4% | -54.7% | -56.7% |
| 15 | 148ns | +0.3% | +12.1% | +4.3% | -55.4% | -52.3% |
| 16 | 153ns | -0.3% | -3.5% | -2.2% | -56.0% | -55.4% |
| 17 | 150ns | -0.5% | -0.8% | +1.4% | -54.6% | -54.3% |
| 18 | 154ns | -2.5% | -4.1% | +0.5% | -54.9% | -56.0% |
| 19 | 153ns | -1.4% | +0.0% | +2.2% | -54.3% | -53.8% |
| 20 | 153ns | -3.3% | -3.3% | -0.3% | -55.4% | -56.0% |
| 21 | 150ns | -0.3% | +1.7% | +0.5% | -54.3% | -53.2% |
| 22 | 156ns | -4.5% | -5.3% | -5.6% | -59.2% | -58.1% |
| 23 | 149ns | +2.6% | -0.5% | +4.2% | -50.1% | -55.1% |
| 24 | 148ns | +1.4% | -0.3% | +2.8% | -52.9% | -54.4% |
| 25 | 152ns | -1.6% | -1.1% | -1.4% | -53.2% | -55.6% |
| 26 | 152ns | -0.3% | -0.3% | -2.2% | -54.1% | -54.7% |
| 27 | 151ns | +0.0% | -2.2% | +0.0% | -55.6% | -54.8% |
| 28 | 153ns | -3.0% | -3.5% | -1.4% | -56.0% | -56.5% |
| 29 | 152ns | -2.8% | -2.2% | +1.1% | -52.7% | -54.7% |
| 30 | 152ns | -2.8% | -3.8% | -3.3% | -55.0% | -54.1% |
| 31 | 198ns | -24.4% | -22.7% | -24.8% | -66.4% | -66.6% |
| 32 | 198ns | -24.6% | -24.2% | -26.3% | -64.2% | -64.6% |
| 33 | 200ns | -24.6% | -25.9% | -21.5% | -66.4% | -63.7% |
| 34 | 198ns | -24.9% | -21.7% | -23.8% | -64.8% | -66.5% |
| 35 | 198ns | -24.1% | -21.5% | -24.7% | -65.6% | -64.8% |
| 36 | 198ns | -25.0% | -22.0% | -24.0% | -10.9% | -64.7% |
| 37 | 196ns | -23.4% | -24.3% | -23.6% | -63.8% | -64.7% |
| 38 | 196ns | -24.4% | -21.6% | -22.9% | -65.0% | -65.4% |
| 39 | 197ns | -25.2% | -22.7% | -21.0% | -65.5% | -65.3% |
| 40 | 198ns | -23.2% | -21.9% | -23.2% | -63.9% | -64.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.884 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.018 | ok |
| warm-clamp-accfit-dyn | 0.748 | HIGH+ (drift/warm-up) |
| warm-clamp-head | -0.064 | ok |
| warm-clamp-min-lanes | -0.021 | ok |
| warm-clamp-minimum | -0.011 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 27/40, lost 11/40
- **warm-clamp-accfit-dyn**: won 25/40, lost 14/40
- **warm-clamp-head**: won 25/40, lost 14/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.3ns | 162.9ns | 1.4% |  |
| warm-clamp-accfit | 1.9ns | 150.1ns | 1.3% |  |
| warm-clamp-accfit-dyn | 2.0ns | 157.6ns | 1.3% |  |
| warm-clamp-head | 2.1ns | 151.2ns | 1.4% |  |
| warm-clamp-min-lanes | 2.4ns | 71.5ns | 3.4% |  |
| warm-clamp-minimum | 2.6ns | 68.6ns | 3.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 148.4-197.9 ns)
    148.4 |####################################
    150.9 |########################################
    153.4 |######
    155.9 |######
    158.3 |
    160.8 |
    163.3 |
    165.8 |
    168.2 |
    170.7 |
    173.2 |
    175.6 |
    178.1 |
    180.6 |
    183.1 |
    185.5 |
    188.0 |
    190.5 |
    193.0 |
    195.4 |#######################
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 147.8-152.6 ns)
    147.8 |######
    148.1 |#################################
    148.3 |
    148.6 |#############
    148.8 |
    149.0 |#################################
    149.3 |
    149.5 |#############
    149.8 |
    150.0 |########################################
    150.2 |#############
    150.5 |
    150.7 |######
    150.9 |
    151.2 |##########################
    151.4 |
    151.7 |####################
    151.9 |
    152.1 |
    152.4 |#############
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 147.7-186.8 ns)
    147.7 |########################################
    149.7 |###########
    151.6 |######################
    153.6 |####################
    155.5 |
    157.5 |
    159.4 |
    161.4 |
    163.3 |
    165.3 |##
    167.2 |
    169.2 |
    171.1 |
    173.1 |
    175.0 |
    177.0 |
    178.9 |
    180.9 |
    182.9 |
    184.8 |
  (2 below, 4 above range)

warm-clamp-head (n=40, range 147.7-155.1 ns)
    147.7 |##########
    148.1 |##########
    148.5 |####################
    148.8 |####################
    149.2 |
    149.6 |##########
    149.9 |##############################
    150.3 |####################
    150.7 |########################################
    151.0 |########################################
    151.4 |##############################
    151.8 |##############################
    152.1 |
    152.5 |##########
    152.9 |##########
    153.2 |##########
    153.6 |##########
    154.0 |
    154.3 |##########
    154.7 |####################
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 65.9-85.1 ns)
     65.9 |#####
     66.9 |########################################
     67.8 |########################################
     68.8 |##############################
     69.7 |###############
     70.7 |#########################
     71.7 |###############
     72.6 |
     73.6 |#####
     74.5 |
     75.5 |
     76.5 |
     77.4 |
     78.4 |
     79.3 |
     80.3 |
     81.3 |
     82.2 |
     83.2 |
     84.1 |
  (4 below, 1 above range)

warm-clamp-minimum (n=40, range 66.0-71.2 ns)
     66.0 |##########################
     66.2 |
     66.5 |####################
     66.8 |
     67.0 |
     67.3 |##########################
     67.5 |
     67.8 |#############
     68.1 |##########################
     68.3 |
     68.6 |####################
     68.8 |
     69.1 |####################
     69.4 |#############
     69.6 |
     69.9 |########################################
     70.1 |####################
     70.4 |
     70.7 |######
     70.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: CV=23.8% (high variance, measurements may be unstable)
