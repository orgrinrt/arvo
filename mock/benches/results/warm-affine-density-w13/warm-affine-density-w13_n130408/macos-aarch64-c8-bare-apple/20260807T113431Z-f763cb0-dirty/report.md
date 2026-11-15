# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (13.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-lanes-deferred at 401 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-lanes-deferred beats baseline by 97% (significant)

warm-container-lanes-deferred is -13.28 us (97%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 34.2x slower than the field

warm-container-headroom (13.70 us) is 34.2x the fastest (401 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-lanes-deferred, warm-container-native) are a dead heat (<1%)

warm-container-lanes-deferred (401 ns) and warm-container-native (402 ns) differ by 0.21%, inside the noise, even though the wider field spreads 3318.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.92)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.92, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-lanes-deferred, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-minimum, warm-container-headroom} (2895% apart)

The field splits into a fast tier {warm-container-lanes-deferred, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-minimum, warm-container-headroom} with a 2895% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 34.2x the fastest

Fastest warm-container-lanes-deferred (401 ns) to slowest warm-container-headroom (13.70 us): 34.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 1.5x its best-20%

warm-container-headroom's best 20% of batches run at 13.11 us but its worst 20% at 20.00 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 400.8 ns median (-97.1% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 34.19x (fastest 400.8 ns, slowest 13702.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 15338ns | 13808ns | 13194ns | 14422ns | 20227ns | base |
| warm-container-kernel | 529ns | 514ns | 454ns | 508ns | 668ns | -96.55% |
| warm-container-lanes-deferred | 504ns | 462ns | 454ns | 474ns | 648ns | -96.71% |
| warm-container-minimum | 14695ns | 13519ns | 13091ns | 14113ns | 18046ns | -4.19% |
| warm-container-native | 506ns | 466ns | 453ns | 480ns | 637ns | -96.70% |
| warm-container-plusone | 14214ns | 13358ns | 13078ns | 13632ns | 17097ns | -7.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 15215ns | 13109ns | 20002ns | base | 4.846 |
| warm-container-kernel | 458ns | 392ns | 583ns | -96.99% | 161.139 |
| warm-container-lanes-deferred | 435ns | 391ns | 558ns | -97.14% | 169.318 |
| warm-container-minimum | 14595ns | 13010ns | 17901ns | -4.08% | 5.052 |
| warm-container-native | 435ns | 391ns | 544ns | -97.14% | 169.551 |
| warm-container-plusone | 14118ns | 12990ns | 16975ns | -7.21% | 5.222 |

## Performance model

- Peak throughput: **188.756 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.381 | 2.9% |
| warm-container-kernel | 166.316 | 88.1% |
| warm-container-lanes-deferred | 183.952 | 97.5% |
| warm-container-minimum | 5.495 | 2.9% |
| warm-container-native | 183.563 | 97.2% |
| warm-container-plusone | 5.553 | 2.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 15338ns | 15338ns | base |
| warm-container-kernel | 529ns | 529ns | -96.55% |
| warm-container-lanes-deferred | 504ns | 504ns | -96.71% |
| warm-container-minimum | 14695ns | 14695ns | -4.19% |
| warm-container-native | 506ns | 506ns | -96.70% |
| warm-container-plusone | 14214ns | 14214ns | -7.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 13702ns | base | --- | [13411, 14770] | --- | --- | --- | --- |
| warm-container-kernel | 443ns | -13277.0ns (-96.9%) | [-14371, -13017]ns | [405, 445] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 401ns | -13304.2ns (-97.1%) | [-14379, -13013]ns | [395, 402] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 13418ns | -409.1ns (-3.0%) | [-1017, -92]ns | [13155, 14758] | YES | 0.0385 | 0.0385 | 0 |
| warm-container-native | 402ns | -13303.7ns (-97.1%) | [-14378, -13017]ns | [395, 420] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 13278ns | -653.1ns (-4.8%) | [-1246, -262]ns | [13102, 13669] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 16176ns | -96.8% | -96.8% | +9.2% | -96.8% | +15.0% |
| 2 | 19062ns | -97.0% | -97.3% | -9.1% | -97.2% | -19.2% |
| 3 | 20315ns | -97.4% | -97.4% | -13.0% | -97.3% | -13.5% |
| 4 | 19625ns | -97.4% | -97.4% | -7.7% | -97.4% | -7.8% |
| 5 | 19933ns | -97.1% | -97.2% | -12.6% | -97.4% | -14.5% |
| 6 | 19260ns | -97.3% | -97.0% | -3.8% | -97.3% | -14.2% |
| 7 | 25277ns | -97.9% | -97.6% | -28.4% | -97.8% | -42.5% |
| 8 | 18303ns | -97.2% | -96.9% | -5.9% | -97.1% | -18.8% |
| 9 | 17548ns | -97.0% | -96.8% | +1.7% | -96.7% | -5.7% |
| 10 | 18238ns | -97.2% | -97.1% | -2.0% | -96.9% | -12.3% |
| 11 | 13150ns | -96.6% | -96.9% | +0.4% | -96.8% | +2.3% |
| 12 | 13664ns | -96.8% | -97.1% | -3.9% | -96.9% | -2.4% |
| 13 | 14762ns | -97.0% | -97.3% | -11.9% | -97.1% | -8.0% |
| 14 | 14901ns | -97.0% | -97.3% | -12.4% | -97.2% | -10.9% |
| 15 | 14537ns | -96.9% | -97.3% | -9.6% | -97.1% | -8.3% |
| 16 | 13159ns | -96.6% | -97.0% | -0.9% | -96.9% | -1.3% |
| 17 | 13056ns | -96.6% | -97.0% | -0.5% | -96.9% | -0.6% |
| 18 | 13558ns | -96.7% | -97.1% | -4.1% | -97.0% | -2.3% |
| 19 | 13711ns | -96.8% | -97.1% | -1.5% | -97.0% | -5.3% |
| 20 | 13665ns | -96.7% | -97.1% | -3.1% | -97.0% | -4.8% |
| 21 | 13232ns | -97.0% | -97.0% | +16.0% | -97.0% | -1.8% |
| 22 | 13303ns | -97.0% | -97.0% | +13.9% | -97.1% | -1.6% |
| 23 | 13643ns | -97.1% | -97.0% | +8.0% | -97.1% | +8.0% |
| 24 | 13432ns | -97.1% | -97.0% | +10.1% | -97.0% | +9.8% |
| 25 | 13280ns | -97.0% | -97.0% | +11.7% | -97.0% | +8.0% |
| 26 | 13355ns | -97.1% | -97.0% | +6.5% | -97.0% | -1.0% |
| 27 | 13004ns | -97.0% | -96.9% | +1.2% | -96.9% | -0.1% |
| 28 | 13162ns | -97.0% | -96.9% | -0.9% | -97.0% | +0.8% |
| 29 | 13107ns | -97.0% | -96.9% | +0.8% | -97.0% | +4.3% |
| 30 | 13002ns | -97.0% | -97.0% | +2.6% | -96.9% | +5.1% |
| 31 | 15615ns | -97.4% | -97.5% | -10.5% | -97.5% | -16.0% |
| 32 | 14765ns | -97.3% | -97.3% | -8.6% | -97.3% | -11.3% |
| 33 | 14776ns | -97.3% | -97.4% | -11.1% | -97.3% | -11.7% |
| 34 | 14151ns | -96.7% | -97.2% | -6.8% | -97.2% | -7.3% |
| 35 | 13390ns | -97.0% | -97.1% | -3.0% | -97.1% | -3.0% |
| 36 | 14436ns | -97.2% | -97.3% | -9.8% | -97.3% | -8.9% |
| 37 | 13268ns | -96.8% | -97.1% | -2.1% | -97.1% | -2.1% |
| 38 | 13375ns | -93.1% | -97.1% | +4.5% | -97.1% | -2.4% |
| 39 | 16714ns | -97.5% | -97.7% | -21.9% | -97.7% | -22.0% |
| 40 | 13693ns | -97.0% | -97.1% | -4.7% | -97.1% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.735 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.190 | ok |
| warm-container-lanes-deferred | 0.921 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.848 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.865 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.717 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 27/40, lost 13/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 32/40, lost 8/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 6.3ns | 15215.1ns | 0.0% |  |
| warm-container-kernel | 2.6ns | 457.5ns | 0.6% |  |
| warm-container-lanes-deferred | 2.6ns | 435.4ns | 0.6% |  |
| warm-container-minimum | 3.7ns | 14594.7ns | 0.0% |  |
| warm-container-native | 3.2ns | 434.8ns | 0.7% |  |
| warm-container-plusone | 4.1ns | 14118.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 13109.0-20001.5 ns)
  13109.0 |########################################
  13453.6 |#####################
  13798.3 |
  14142.9 |#######
  14487.5 |##############
  14832.1 |###
  15176.8 |
  15521.4 |###
  15866.0 |###
  16210.6 |
  16555.3 |###
  16899.9 |
  17244.5 |###
  17589.1 |
  17933.8 |###
  18278.4 |###
  18623.0 |
  18967.6 |#######
  19312.3 |###
  19656.9 |###
  (4 below, 2 above range)

warm-container-kernel (n=40, range 392.0-582.9 ns)
    392.0 |########################################
    401.5 |############
    411.1 |########
    420.6 |
    430.2 |
    439.7 |########################################
    449.3 |
    458.8 |
    468.3 |####
    477.9 |
    487.4 |
    497.0 |
    506.5 |####
    516.1 |########################
    525.6 |####
    535.2 |
    544.7 |
    554.3 |####
    563.8 |####
    573.4 |
  (3 below, 1 above range)

warm-container-lanes-deferred (n=40, range 391.5-558.1 ns)
    391.5 |########################################
    399.8 |######################
    408.1 |#####
    416.5 |
    424.8 |
    433.1 |
    441.4 |
    449.8 |
    458.1 |
    466.4 |
    474.8 |
    483.1 |
    491.4 |
    499.8 |
    508.1 |
    516.4 |############
    524.7 |
    533.1 |
    541.4 |
    549.7 |
  (3 below, 5 above range)

warm-container-minimum (n=40, range 13009.7-17900.8 ns)
  13009.7 |########################################
  13254.3 |##
  13498.8 |#####
  13743.4 |#####
  13988.0 |##
  14232.5 |
  14477.1 |
  14721.6 |########
  14966.2 |##
  15210.7 |##
  15455.3 |
  15699.8 |
  15944.4 |
  16189.0 |
  16433.5 |
  16678.1 |
  16922.6 |
  17167.2 |#####
  17411.7 |##
  17656.3 |###########
  (5 below, 3 above range)

warm-container-native (n=40, range 390.6-544.1 ns)
    390.6 |########################################
    398.3 |##################
    405.9 |###
    413.6 |##################
    421.3 |
    429.0 |
    436.6 |
    444.3 |
    452.0 |
    459.7 |
    467.3 |
    475.0 |
    482.7 |
    490.3 |
    498.0 |
    505.7 |
    513.4 |############
    521.0 |###
    528.7 |###
    536.4 |
  (4 below, 4 above range)

warm-container-plusone (n=40, range 12989.9-16975.0 ns)
  12989.9 |########################################
  13189.1 |#####################
  13388.4 |#######
  13587.6 |#######
  13786.9 |
  13986.1 |
  14185.4 |###
  14384.6 |###
  14583.9 |#######
  14783.2 |###
  14982.4 |
  15181.7 |
  15380.9 |###
  15580.2 |
  15779.4 |
  15978.7 |###
  16177.9 |
  16377.2 |#######
  16576.5 |
  16775.7 |
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=20.1% (high variance, measurements may be unstable)
- **warm-container-lanes-deferred**: autocorrelation=0.92 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.72 (measurement drift or warm-up artifact)
