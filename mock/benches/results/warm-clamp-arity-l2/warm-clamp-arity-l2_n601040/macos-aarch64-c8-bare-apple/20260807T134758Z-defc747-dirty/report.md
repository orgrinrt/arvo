# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes is an outlier: 4.7x slower than the field

warm-clamp-min-lanes (508.75 us) is 4.7x the fastest (108.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit-dyn shows warm-up / thermal drift (autocorr +0.79)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn} vs {warm-clamp-head, warm-clamp-min-lanes} (139% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-head, warm-clamp-min-lanes} with a 139% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest warm-clamp-minimum (108.90 us) to slowest warm-clamp-min-lanes (508.75 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-minimum** at 108897.1 ns median (-4.3% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.67x (fastest 108897.1 ns, slowest 508750.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 113309ns | 114121ns | 109640ns | 113316ns | 116958ns | base |
| warm-clamp-accfit | 110197ns | 111679ns | 105833ns | 110134ns | 114747ns | -2.75% |
| warm-clamp-accfit-dyn | 137442ns | 134264ns | 129704ns | 134996ns | 152516ns | +21.30% |
| warm-clamp-head | 322781ns | 321038ns | 316646ns | 321141ns | 333835ns | +184.87% |
| warm-clamp-min-lanes | 510845ns | 509543ns | 502720ns | 510427ns | 520223ns | +350.84% |
| warm-clamp-minimum | 109093ns | 109207ns | 101466ns | 109076ns | 116770ns | -3.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 112953ns | 109382ns | 116507ns | base | 9.283 |
| warm-clamp-accfit | 109927ns | 105626ns | 114457ns | -2.68% | 9.539 |
| warm-clamp-accfit-dyn | 137039ns | 129245ns | 152062ns | +21.32% | 7.652 |
| warm-clamp-head | 321486ns | 315056ns | 332529ns | +184.62% | 3.262 |
| warm-clamp-min-lanes | 510243ns | 502180ns | 519689ns | +351.73% | 2.055 |
| warm-clamp-minimum | 108792ns | 101212ns | 116440ns | -3.68% | 9.638 |

## Performance model

- Peak throughput: **10.360 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.219 | 89.0% |
| warm-clamp-accfit | 9.414 | 90.9% |
| warm-clamp-accfit-dyn | 7.829 | 75.6% |
| warm-clamp-head | 3.275 | 31.6% |
| warm-clamp-min-lanes | 2.061 | 19.9% |
| warm-clamp-minimum | 9.629 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 113309ns | 113309ns | base |
| warm-clamp-accfit | 110197ns | 110197ns | -2.75% |
| warm-clamp-accfit-dyn | 137442ns | 137442ns | +21.30% |
| warm-clamp-head | 322781ns | 322781ns | +184.87% |
| warm-clamp-min-lanes | 510845ns | 510845ns | +350.84% |
| warm-clamp-minimum | 109093ns | 109093ns | -3.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 113738ns | base | --- | [111227, 114191] | --- | --- | --- | --- |
| warm-clamp-accfit | 111386ns | -4517.7ns (-4.0%) | [-5406, -2184]ns | [107002, 111784] | YES | 0.0080 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 133938ns | +21127.9ns (+18.6%) | [+18547, +25468]ns | [132630, 135732] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 320200ns | +206477.2ns (+181.5%) | [+205309, +208618]ns | [318339, 321496] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 508751ns | +396914.3ns (+349.0%) | [+394882, +400483]ns | [506968, 513865] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 108897ns | no significant difference | [-7766, +205]ns | [107525, 111214] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 113538ns | -1.7% | +17.1% | +178.5% | +343.0% | -0.4% |
| 2 | 113995ns | -2.1% | +16.6% | +185.7% | +339.1% | -4.5% |
| 3 | 114244ns | -2.4% | +16.0% | +179.5% | +340.2% | -5.4% |
| 4 | 114742ns | -2.5% | +15.6% | +187.0% | +360.5% | -4.2% |
| 5 | 115533ns | -3.3% | +15.1% | +171.1% | +334.2% | -5.2% |
| 6 | 115853ns | -3.9% | +14.4% | +170.0% | +334.4% | +2.0% |
| 7 | 116004ns | -3.8% | +14.5% | +169.9% | +331.8% | -6.4% |
| 8 | 116565ns | -4.1% | +13.8% | +171.4% | +331.7% | -7.7% |
| 9 | 107470ns | +4.1% | +24.1% | +213.1% | +369.6% | +0.6% |
| 10 | 107286ns | +5.5% | +23.6% | +227.8% | +368.1% | +1.5% |
| 11 | 110343ns | +2.1% | +25.7% | +190.9% | +359.5% | +1.2% |
| 12 | 111995ns | +1.1% | +20.6% | +183.2% | +354.0% | -0.8% |
| 13 | 114230ns | -1.3% | +17.9% | +177.1% | +343.5% | +1.7% |
| 14 | 109897ns | +2.6% | +23.3% | +195.4% | +364.0% | +2.1% |
| 15 | 109768ns | +3.4% | +22.5% | +190.0% | +362.9% | +1.4% |
| 16 | 109898ns | +2.6% | +23.0% | +196.0% | +361.4% | +1.2% |
| 17 | 109674ns | +4.9% | +25.6% | +190.3% | +365.7% | +1.4% |
| 18 | 110721ns | +5.6% | +22.7% | +187.2% | +375.7% | +0.5% |
| 19 | 110904ns | +2.5% | +26.3% | +186.0% | +357.7% | +1.1% |
| 20 | 113759ns | +1.8% | +19.2% | +180.0% | +348.4% | +0.1% |
| 21 | 117088ns | -9.5% | +10.5% | +175.8% | +341.0% | -13.9% |
| 22 | 114360ns | -7.8% | +12.9% | +178.7% | +350.5% | -9.2% |
| 23 | 110928ns | -4.9% | +16.3% | +191.5% | +364.0% | -8.0% |
| 24 | 111562ns | -5.4% | +15.8% | +183.6% | +361.5% | -3.4% |
| 25 | 111075ns | -4.5% | +16.6% | +190.5% | +363.8% | -9.2% |
| 26 | 111092ns | -4.7% | +16.2% | +188.3% | +363.0% | -9.0% |
| 27 | 111137ns | -4.6% | +16.2% | +184.6% | +362.4% | -9.3% |
| 28 | 110807ns | -4.9% | +16.8% | +185.2% | +363.9% | -9.1% |
| 29 | 111317ns | -5.2% | +17.2% | +184.5% | +361.6% | -8.6% |
| 30 | 111546ns | -5.2% | +16.3% | +185.5% | +360.8% | -9.0% |
| 31 | 114147ns | -6.1% | +30.5% | +181.0% | +345.9% | -9.6% |
| 32 | 113880ns | -6.2% | +40.4% | +182.5% | +345.2% | -9.1% |
| 33 | 115487ns | -0.9% | +31.9% | +178.2% | +338.8% | -7.0% |
| 34 | 115618ns | -7.5% | +31.1% | +183.7% | +339.7% | -5.7% |
| 35 | 119912ns | -10.8% | +16.4% | +167.0% | +322.6% | -10.7% |
| 36 | 114152ns | -6.1% | +23.1% | +180.5% | +344.1% | -6.3% |
| 37 | 114784ns | -6.8% | +29.7% | +179.2% | +358.3% | +8.5% |
| 38 | 113818ns | -6.0% | +32.1% | +183.0% | +352.0% | +3.6% |
| 39 | 113717ns | -2.6% | +35.5% | +198.0% | +352.1% | +0.2% |
| 40 | 115268ns | -5.6% | +30.5% | +181.8% | +343.4% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.594 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.723 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.792 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.265 | moderate+ |
| warm-clamp-min-lanes | 0.217 | moderate+ |
| warm-clamp-minimum | 0.602 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 29/40, lost 11/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 25/40, lost 15/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.6ns | 112952.9ns | 0.0% |  |
| warm-clamp-accfit | 3.0ns | 109926.6ns | 0.0% |  |
| warm-clamp-accfit-dyn | 4.9ns | 137039.0ns | 0.0% |  |
| warm-clamp-head | 4.1ns | 321485.6ns | 0.0% |  |
| warm-clamp-min-lanes | 5.9ns | 510242.8ns | 0.0% |  |
| warm-clamp-minimum | 3.2ns | 108792.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 109382.2-116507.5 ns)
  109382.2 |########
  109738.4 |########################
  110094.7 |########
  110451.0 |################
  110807.2 |########################################
  111163.5 |########
  111519.8 |################
  111876.0 |########
  112232.3 |
  112588.6 |
  112944.8 |
  113301.1 |########
  113657.4 |########################################
  114013.6 |########################################
  114369.9 |
  114726.2 |################
  115082.4 |########
  115438.7 |########################
  115795.0 |################
  116151.2 |
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 105625.6-114457.1 ns)
  105625.6 |#################################
  106067.1 |
  106508.7 |######
  106950.3 |########################################
  107391.9 |
  107833.4 |
  108275.0 |
  108716.6 |######
  109158.2 |
  109599.8 |
  110041.3 |
  110482.9 |######
  110924.5 |######
  111366.1 |########################################
  111807.6 |#############
  112249.2 |
  112690.8 |##########################
  113132.3 |####################
  113573.9 |######
  114015.5 |######
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 129245.4-152062.2 ns)
  129245.4 |################################
  130386.2 |########
  131527.0 |########################################
  132667.9 |########################################
  133808.7 |################
  134949.6 |########################################
  136090.4 |
  137231.3 |########
  138372.1 |########
  139512.9 |########################
  140653.8 |
  141794.6 |
  142935.5 |
  144076.3 |
  145217.2 |
  146358.0 |
  147498.9 |
  148639.7 |################
  149780.5 |################
  150921.4 |########
  (5 below, 3 above range)

warm-clamp-head (n=40, range 315056.4-332529.3 ns)
  315056.4 |
  315930.0 |########################################
  316803.7 |###########
  317677.3 |############################
  318551.0 |###########
  319424.6 |#################
  320298.3 |#################
  321171.9 |###########
  322045.6 |###########
  322919.2 |###########
  323792.8 |#####
  324666.5 |###########
  325540.1 |#####
  326413.8 |
  327287.4 |#####
  328161.1 |
  329034.7 |#####
  329908.4 |
  330782.0 |
  331655.7 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 502179.6-519689.2 ns)
  502179.6 |#################
  503055.1 |###########
  503930.5 |#####
  504806.0 |
  505681.5 |
  506557.0 |########################################
  507432.5 |###########
  508307.9 |#################
  509183.4 |###########
  510058.9 |#####
  510934.4 |#####
  511809.9 |
  512685.3 |
  513560.8 |########################################
  514436.3 |######################
  515311.8 |
  516187.3 |#####
  517062.7 |
  517938.2 |
  518813.7 |
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 101212.1-116440.1 ns)
  101212.1 |#############
  101973.5 |######
  102734.9 |#############
  103496.3 |######
  104257.7 |
  105019.1 |
  105780.5 |
  106541.9 |#############
  107303.3 |####################
  108064.7 |####################
  108826.1 |##########################
  109587.5 |######
  110348.9 |
  111110.3 |########################################
  111871.7 |#############
  112633.1 |######
  113394.5 |####################
  114155.9 |
  114917.3 |
  115678.7 |######
  (5 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.60 (measurement drift or warm-up artifact)
