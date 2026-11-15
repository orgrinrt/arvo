# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 11% faster than the next best (warm-clamp-accfit)

warm-clamp-min-lanes (247.16 us) leads warm-clamp-accfit (274.90 us) by 11%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.78)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-minimum, warm-clamp-head} vs {warm-clamp-accfit-dyn} (28% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-minimum, warm-clamp-head} and a slow tier {warm-clamp-accfit-dyn} with a 28% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 247161.5 ns median (-12.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.48x (fastest 247161.5 ns, slowest 367026.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 283760ns | 282377ns | 281171ns | 283098ns | 288334ns | base |
| warm-clamp-accfit | 281048ns | 275140ns | 273583ns | 277667ns | 298655ns | -0.96% |
| warm-clamp-accfit-dyn | 367783ns | 367853ns | 363803ns | 367793ns | 371731ns | +29.61% |
| warm-clamp-head | 296308ns | 287756ns | 285601ns | 290737ns | 323726ns | +4.42% |
| warm-clamp-min-lanes | 248311ns | 247548ns | 242707ns | 247416ns | 256600ns | -12.49% |
| warm-clamp-minimum | 283998ns | 282331ns | 279516ns | 283087ns | 291211ns | +0.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 283332ns | 280769ns | 287943ns | base | 3.701 |
| warm-clamp-accfit | 280249ns | 271957ns | 297989ns | -1.09% | 3.742 |
| warm-clamp-accfit-dyn | 366915ns | 363262ns | 371110ns | +29.50% | 2.858 |
| warm-clamp-head | 294597ns | 283606ns | 322252ns | +3.98% | 3.559 |
| warm-clamp-min-lanes | 247871ns | 242340ns | 256066ns | -12.52% | 4.230 |
| warm-clamp-minimum | 283481ns | 279031ns | 290646ns | +0.05% | 3.699 |

## Performance model

- Peak throughput: **4.327 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.720 | 86.0% |
| warm-clamp-accfit | 3.814 | 88.2% |
| warm-clamp-accfit-dyn | 2.857 | 66.0% |
| warm-clamp-head | 3.658 | 84.5% |
| warm-clamp-min-lanes | 4.242 | 98.0% |
| warm-clamp-minimum | 3.720 | 86.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 283760ns | 283760ns | base |
| warm-clamp-accfit | 281048ns | 281048ns | -0.96% |
| warm-clamp-accfit-dyn | 367783ns | 367783ns | +29.61% |
| warm-clamp-head | 296308ns | 296308ns | +4.42% |
| warm-clamp-min-lanes | 248311ns | 248311ns | -12.49% |
| warm-clamp-minimum | 283998ns | 283998ns | +0.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 281851ns | base | --- | [281662, 283168] | --- | --- | --- | --- |
| warm-clamp-accfit | 274899ns | -6309.6ns (-2.2%) | [-7565, -4134]ns | [273379, 279669] | YES | 0.0080 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 367027ns | +83464.8ns (+29.6%) | [+81655, +85600]ns | [365033, 367607] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 286664ns | +4190.2ns (+1.5%) | [+2416, +8198]ns | [284676, 289730] | YES | 0.0003 | 0.0002 | 0 |
| warm-clamp-min-lanes | 247161ns | -36756.1ns (-13.0%) | [-38199, -35469]ns | [244869, 248008] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 281904ns | no significant difference | [-2469, +1455]ns | [280669, 283912] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 282460ns | -3.0% | +28.3% | +7.5% | -9.2% | -0.3% |
| 2 | 281783ns | -3.4% | +29.1% | +1.7% | -8.9% | +0.8% |
| 3 | 281656ns | -3.4% | +29.6% | +13.3% | -11.5% | +0.1% |
| 4 | 281446ns | -3.5% | +29.4% | +31.7% | -9.0% | +0.1% |
| 5 | 281598ns | -3.5% | +29.1% | +18.6% | -11.2% | +1.1% |
| 6 | 286008ns | -5.0% | +27.1% | +8.9% | -10.8% | -0.9% |
| 7 | 281669ns | -3.1% | +28.9% | +9.2% | -9.4% | +0.2% |
| 8 | 281238ns | -3.0% | +29.0% | +9.3% | -9.0% | +1.2% |
| 9 | 281398ns | -3.5% | +29.6% | +5.6% | -8.9% | +0.2% |
| 10 | 291643ns | -6.8% | +24.5% | +6.5% | -12.1% | -2.5% |
| 11 | 281832ns | -2.1% | +30.4% | +1.8% | -14.3% | +0.3% |
| 12 | 281917ns | -1.4% | +30.1% | +6.4% | -14.4% | -0.5% |
| 13 | 283876ns | -2.4% | +29.9% | +1.8% | -15.0% | -1.4% |
| 14 | 281723ns | -1.9% | +30.5% | +2.3% | -13.2% | -0.7% |
| 15 | 281995ns | -2.6% | +30.3% | +3.0% | -13.5% | -0.8% |
| 16 | 281919ns | -2.1% | +30.5% | +7.9% | -14.0% | -0.7% |
| 17 | 281644ns | +24.9% | +30.4% | +0.7% | -13.1% | +0.8% |
| 18 | 284118ns | +0.4% | +29.4% | -0.4% | -14.8% | -1.2% |
| 19 | 281848ns | +1.0% | +30.5% | +0.4% | -14.1% | -0.5% |
| 20 | 281868ns | +0.6% | +30.3% | +0.8% | -13.4% | +0.1% |
| 21 | 281855ns | -3.0% | +31.6% | +0.9% | -13.1% | +3.4% |
| 22 | 281738ns | -3.0% | +31.5% | +0.8% | -12.7% | +3.2% |
| 23 | 281550ns | -2.9% | +31.6% | +0.9% | -10.7% | +3.3% |
| 24 | 280640ns | -2.5% | +32.2% | +1.2% | -12.8% | +3.7% |
| 25 | 281238ns | -2.8% | +32.1% | +0.8% | -13.0% | +3.0% |
| 26 | 280648ns | -2.6% | +32.0% | +1.4% | -12.6% | +3.3% |
| 27 | 280369ns | -2.5% | +32.7% | +1.2% | -12.8% | +3.6% |
| 28 | 280230ns | -1.9% | +32.3% | +3.1% | -12.6% | +3.6% |
| 29 | 281606ns | -2.4% | +31.6% | +1.5% | -13.0% | +3.1% |
| 30 | 280393ns | -2.0% | +32.4% | +1.4% | -12.6% | +3.4% |
| 31 | 287448ns | +0.3% | +26.8% | +2.8% | -13.8% | -3.0% |
| 32 | 285685ns | +3.9% | +28.7% | +10.9% | -13.3% | -2.5% |
| 33 | 288330ns | +0.9% | +26.4% | -0.5% | -13.9% | -3.2% |
| 34 | 287791ns | +0.6% | +26.8% | -0.4% | -14.0% | -2.5% |
| 35 | 287889ns | -0.5% | +26.8% | -1.0% | -13.4% | -3.2% |
| 36 | 286636ns | +0.5% | +28.0% | +0.0% | -13.7% | -2.0% |
| 37 | 286082ns | +1.4% | +27.4% | -0.4% | -13.6% | -2.5% |
| 38 | 287213ns | +0.4% | +27.6% | -1.1% | -13.9% | -2.9% |
| 39 | 286592ns | -0.2% | +27.5% | -0.7% | -13.8% | -2.0% |
| 40 | 285690ns | -1.6% | +27.9% | -0.4% | -11.5% | -2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.463 | moderate+ |
| warm-clamp-accfit | 0.254 | moderate+ |
| warm-clamp-accfit-dyn | 0.752 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.608 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.652 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.781 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 29/40, lost 11/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 8/40, lost 31/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 20/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 4.7ns | 283331.5ns | 0.0% |  |
| warm-clamp-accfit | 10.2ns | 280249.0ns | 0.0% |  |
| warm-clamp-accfit-dyn | 4.1ns | 366914.8ns | 0.0% |  |
| warm-clamp-head | 3.7ns | 294596.8ns | 0.0% |  |
| warm-clamp-min-lanes | 4.5ns | 247871.1ns | 0.0% |  |
| warm-clamp-minimum | 4.9ns | 283481.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 280769.3-287942.7 ns)
  280769.3 |
  281128.0 |################
  281486.6 |########################################
  281845.3 |########################
  282204.0 |####
  282562.6 |
  282921.3 |
  283280.0 |
  283638.6 |####
  283997.3 |####
  284356.0 |
  284714.6 |
  285073.3 |
  285432.0 |########
  285790.6 |########
  286149.3 |
  286508.0 |########
  286866.6 |####
  287225.3 |####
  287584.0 |########
  (5 below, 2 above range)

warm-clamp-accfit (n=40, range 271956.6-297988.8 ns)
  271956.6 |####################
  273258.2 |########################################
  274559.8 |####################
  275861.4 |###############
  277163.0 |##########
  278464.6 |
  279766.2 |
  281067.8 |#####
  282369.5 |#####
  283671.1 |#####
  284972.7 |##########
  286274.3 |#####
  287575.9 |###############
  288877.5 |##########
  290179.1 |#####
  291480.7 |
  292782.4 |
  294084.0 |
  295385.6 |
  296687.2 |#####
  (5 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 363262.4-371110.3 ns)
  363262.4 |################
  363654.8 |########
  364047.2 |################
  364439.6 |########################
  364832.0 |########################
  365224.4 |################
  365616.7 |
  366009.1 |
  366401.5 |################
  366793.9 |################
  367186.3 |########################################
  367578.7 |########################
  367971.1 |
  368363.5 |########
  368755.9 |
  369148.3 |
  369540.7 |
  369933.1 |
  370325.5 |################################
  370717.9 |########################
  (4 below, 3 above range)

warm-clamp-head (n=40, range 283606.1-322252.0 ns)
  283606.1 |########################################
  285538.4 |##################
  287470.7 |#########
  289403.0 |###
  291335.3 |
  293267.6 |
  295199.9 |######
  297132.2 |
  299064.5 |###
  300996.8 |
  302929.1 |######
  304861.4 |
  306793.7 |######
  308725.9 |
  310658.2 |######
  312590.5 |
  314522.8 |
  316455.1 |###
  318387.4 |###
  320319.7 |
  (4 below, 2 above range)

warm-clamp-min-lanes (n=40, range 242339.9-256066.4 ns)
  242339.9 |#####
  243026.2 |
  243712.6 |##########
  244398.9 |########################################
  245085.2 |##########
  245771.5 |#####
  246457.9 |#####
  247144.2 |##############################
  247830.5 |#####
  248516.8 |#####
  249203.2 |#####
  249889.5 |#####
  250575.8 |
  251262.1 |#####
  251948.5 |
  252634.8 |#####
  253321.1 |
  254007.4 |
  254693.8 |##########
  255380.1 |#####
  (5 below, 5 above range)

warm-clamp-minimum (n=40, range 279030.6-290646.4 ns)
  279030.6 |##############################
  279611.4 |########################################
  280192.2 |########################################
  280773.0 |####################
  281353.8 |########################################
  281934.6 |####################
  282515.4 |##########
  283096.2 |##########
  283676.9 |####################
  284257.7 |##############################
  284838.5 |
  285419.3 |
  286000.1 |
  286580.9 |
  287161.7 |
  287742.5 |
  288323.3 |
  288904.0 |
  289484.8 |##############################
  290065.6 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.78 (measurement drift or warm-up artifact)
