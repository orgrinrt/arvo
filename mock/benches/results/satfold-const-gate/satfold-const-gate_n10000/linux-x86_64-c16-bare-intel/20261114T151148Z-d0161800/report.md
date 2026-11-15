# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-gate-false) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-gate-false has the worst median (25.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes16-3 at 3.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes16-3 beats baseline by 87% (significant)

satfold-lanes16-3 is -21.97 us (87%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-gate-false is an outlier: 7.6x slower than the field

satfold-gate-false (25.32 us) is 7.6x the fastest (3.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-3 is fastest but the noisiest (CV 25.0%)

satfold-lanes16-3 wins on median (3.35 us) yet has the highest variance (CV 25.0%), while satfold-gate-false is the steadiest (CV 0.2%, 25.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-seq shows alternating (throttle bounce) (autocorr -0.50)

satfold-seq's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} vs {satfold-seq, satfold-gate-false} (582% apart)

The field splits into a fast tier {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} and a slow tier {satfold-seq, satfold-gate-false} with a 582% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.6x the fastest

Fastest satfold-lanes16-3 (3.35 us) to slowest satfold-gate-false (25.32 us): 7.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-3 is inconsistent: worst-20% is 2.2x its best-20%

satfold-lanes16-3's best 20% of batches run at 1.78 us but its worst 20% at 3.97 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16-3** at 3352.7 ns median (-86.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 7.55x (fastest 3352.7 ns, slowest 25323.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 25417ns | 25397ns | 25381ns | 25400ns | 25503ns | base |
| satfold-gate-true | 3525ns | 3874ns | 2066ns | 3779ns | 4225ns | -86.13% |
| satfold-lanes16 | 3623ns | 3728ns | 2595ns | 3728ns | 4339ns | -85.74% |
| satfold-lanes16-3 | 3333ns | 3560ns | 1896ns | 3505ns | 4253ns | -86.89% |
| satfold-seq | 25012ns | 24959ns | 24912ns | 24987ns | 25189ns | -1.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 25339ns | 25308ns | 25409ns | base | 1.293 |
| satfold-gate-true | 3307ns | 1945ns | 3936ns | -86.95% | 9.908 |
| satfold-lanes16 | 3401ns | 2444ns | 4026ns | -86.58% | 9.635 |
| satfold-lanes16-3 | 3129ns | 1785ns | 3970ns | -87.65% | 10.471 |
| satfold-seq | 24932ns | 24837ns | 25106ns | -1.61% | 1.314 |

## Performance model

- Peak throughput: **18.358 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.294 | 7.0% |
| satfold-gate-true | 8.983 | 48.9% |
| satfold-lanes16 | 9.326 | 50.8% |
| satfold-lanes16-3 | 9.774 | 53.2% |
| satfold-seq | 1.317 | 7.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 25417ns | 25417ns | base |
| satfold-gate-true | 3525ns | 3525ns | -86.13% |
| satfold-lanes16 | 3623ns | 3623ns | -85.74% |
| satfold-lanes16-3 | 3333ns | 3333ns | -86.89% |
| satfold-seq | 25012ns | 25012ns | -1.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 25324ns | base | --- | [25321, 25329] | --- | --- | --- | --- |
| satfold-gate-true | 3648ns | -21703.0ns (-85.7%) | [-21965, -21651]ns | [3365, 3681] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 3514ns | -21836.8ns (-86.2%) | [-21970, -21632]ns | [3346, 3704] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 3353ns | -21968.1ns (-86.7%) | [-22333, -21644]ns | [2991, 3677] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 24879ns | -444.8ns (-1.8%) | [-467, -404]ns | [24851, 24976] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 25332ns | -86.1% | -85.2% | -85.4% | -1.9% |
| 1 | 2 | 25326ns | -86.3% | -85.5% | -85.5% | -1.4% |
| 1 | 3 | 25323ns | -85.6% | -87.2% | -89.5% | -1.8% |
| 1 | 4 | 25318ns | -85.5% | -86.5% | -86.7% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | 0.191 | ok |
| satfold-gate-true | 0.157 | ok |
| satfold-lanes16 | 0.127 | ok |
| satfold-lanes16-3 | -0.175 | ok |
| satfold-seq | -0.503 | HIGH- (thermal bounce) |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 23.8ns | 25338.6ns | 0.1% |  |
| satfold-gate-true | 75.5ns | 3307.1ns | 2.3% |  |
| satfold-lanes16 | 78.7ns | 3401.0ns | 2.3% |  |
| satfold-lanes16-3 | 70.6ns | 3129.3ns | 2.3% |  |
| satfold-seq | 23.7ns | 24931.9ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 25308.4-25408.6 ns)
  25308.4 |########
  25313.4 |#################
  25318.4 |########################################
  25323.4 |##########################
  25328.4 |####
  25333.4 |########################################
  25338.4 |
  25343.4 |
  25348.5 |####
  25353.5 |
  25358.5 |
  25363.5 |
  25368.5 |
  25373.5 |
  25378.5 |
  25383.5 |
  25388.6 |
  25393.6 |
  25398.6 |
  25403.6 |
  (5 below, 3 above range)

satfold-gate-true (n=40, range 1945.5-3936.1 ns)
   1945.5 |
   2045.0 |
   2144.5 |
   2244.1 |###
   2343.6 |
   2443.1 |
   2542.6 |
   2642.2 |
   2741.7 |###
   2841.2 |
   2940.8 |
   3040.3 |###
   3139.8 |###
   3239.4 |###############
   3338.9 |#########
   3438.4 |###
   3538.0 |######
   3637.5 |########################################
   3737.0 |#########
   3836.5 |###
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 2443.7-4025.8 ns)
   2443.7 |
   2522.8 |
   2601.9 |
   2681.0 |
   2760.1 |################
   2839.2 |
   2918.4 |########
   2997.5 |################################
   3076.6 |########
   3155.7 |########
   3234.8 |
   3313.9 |########################
   3393.0 |########################
   3472.1 |########################
   3551.2 |
   3630.3 |########################################
   3709.4 |################################
   3788.5 |########################################
   3867.6 |########
   3946.7 |########
  (3 below, 3 above range)

satfold-lanes16-3 (n=40, range 1785.0-3969.6 ns)
   1785.0 |
   1894.2 |###
   2003.4 |
   2112.7 |
   2221.9 |
   2331.1 |
   2440.4 |######
   2549.6 |##########
   2658.8 |###
   2768.1 |######
   2877.3 |###
   2986.5 |##########
   3095.7 |###
   3205.0 |###
   3314.2 |##########
   3423.4 |
   3532.7 |
   3641.9 |########################################
   3751.1 |###
   3860.4 |###
  (4 below, 4 above range)

satfold-seq (n=40, range 24837.5-25106.4 ns)
  24837.5 |########################################
  24850.9 |############
  24864.4 |########
  24877.8 |################
  24891.3 |
  24904.7 |
  24918.1 |
  24931.6 |
  24945.0 |####
  24958.5 |####
  24971.9 |####################
  24985.4 |####
  24998.8 |####
  25012.3 |############
  25025.7 |
  25039.2 |
  25052.6 |
  25066.1 |
  25079.5 |####
  25092.9 |
  (4 below, 4 above range)

```

## Diagnostics

- **satfold-gate-true**: CV=24.8% (high variance, measurements may be unstable)
- **satfold-lanes16-3**: CV=26.8% (high variance, measurements may be unstable)
