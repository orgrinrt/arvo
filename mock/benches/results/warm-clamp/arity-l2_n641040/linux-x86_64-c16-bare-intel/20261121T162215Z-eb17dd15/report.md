# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 11% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-accfit (233.23 us) leads warm-clamp-accfit-dyn (258.33 us) by 11%, a clear separation rather than a photo finish. CV 18.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 47% (significant)

warm-clamp-accfit is -205.19 us (47%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 2.0x slower than the field

warm-clamp-minimum (473.44 us) is 2.0x the fastest (233.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit is fastest but the noisiest (CV 18.9%)

warm-clamp-accfit wins on median (233.23 us) yet has the highest variance (CV 18.9%), while warm-clamp-minimum is the steadiest (CV 8.1%, 473.44 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.80)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-accfit-dyn} vs {warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-head, warm-clamp-minimum} (64% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-head, warm-clamp-minimum} with a 64% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit is inconsistent: worst-20% is 1.7x its best-20%

warm-clamp-accfit's best 20% of batches run at 184.72 us but its worst 20% at 305.29 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 233234.4 ns median (-46.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.03x (fastest 233234.4 ns, slowest 473441.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112705153 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 447838ns | 432760ns | 397938ns | 431278ns | 547420ns | base |
| warm-clamp-accfit | 236957ns | 233431ns | 184924ns | 231436ns | 305553ns | -47.09% |
| warm-clamp-accfit-dyn | 265402ns | 258530ns | 205116ns | 263293ns | 332014ns | -40.74% |
| warm-clamp-head | 458262ns | 433887ns | 410382ns | 430741ns | 588702ns | +2.33% |
| warm-clamp-min-lanes | 426487ns | 425110ns | 354169ns | 420796ns | 515878ns | -4.77% |
| warm-clamp-minimum | 471583ns | 473677ns | 428018ns | 468943ns | 523068ns | +5.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 447584ns | 397752ns | 547105ns | base | 2.343 |
| warm-clamp-accfit | 236753ns | 184718ns | 305294ns | -47.10% | 4.429 |
| warm-clamp-accfit-dyn | 265123ns | 204916ns | 331665ns | -40.77% | 3.955 |
| warm-clamp-head | 457546ns | 409826ns | 588043ns | +2.23% | 2.292 |
| warm-clamp-min-lanes | 426189ns | 353997ns | 515459ns | -4.78% | 2.460 |
| warm-clamp-minimum | 471314ns | 427775ns | 522732ns | +5.30% | 2.225 |

## Performance model

- Peak throughput: **5.677 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.424 | 42.7% |
| warm-clamp-accfit | 4.496 | 79.2% |
| warm-clamp-accfit-dyn | 4.059 | 71.5% |
| warm-clamp-head | 2.422 | 42.7% |
| warm-clamp-min-lanes | 2.468 | 43.5% |
| warm-clamp-minimum | 2.215 | 39.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 447838ns | 447838ns | base |
| warm-clamp-accfit | 236957ns | 236957ns | -47.09% |
| warm-clamp-accfit-dyn | 265402ns | 265402ns | -40.74% |
| warm-clamp-head | 458262ns | 458262ns | +2.33% |
| warm-clamp-min-lanes | 426487ns | 426487ns | -4.77% |
| warm-clamp-minimum | 471583ns | 471583ns | +5.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 432495ns | base | --- | [403104, 442588] | --- | --- | --- | --- |
| warm-clamp-accfit | 233234ns | -205194.7ns (-47.4%) | [-239992, -197820]ns | [199254, 257280] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 258327ns | -169572.4ns (-39.2%) | [-213491, -152059]ns | [244151, 267656] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 433019ns | no significant difference | [-28647, +36132]ns | [414452, 441643] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-min-lanes | 424864ns | no significant difference | [-9480, +26770]ns | [421655, 427456] | no | 0.1923 | 0.1539 | 0 |
| warm-clamp-minimum | 473441ns | +32126.1ns (+7.4%) | [+27383, +36202]ns | [471154, 477649] | YES | 0.0003 | 0.0002 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 397958ns | -50.0% | -40.5% | +33.2% | +6.9% | +7.8% |
| 1 | 2 | 403104ns | -33.6% | -32.7% | +9.4% | +4.7% | +17.0% |
| 1 | 3 | 442017ns | -58.2% | -35.8% | -6.9% | +6.2% | +8.1% |
| 1 | 4 | 534611ns | -51.7% | -50.5% | -22.5% | -33.6% | -10.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.150 | ok |
| warm-clamp-accfit | -0.803 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.041 | ok |
| warm-clamp-head | 0.105 | ok |
| warm-clamp-min-lanes | -0.446 | moderate- |
| warm-clamp-minimum | 0.021 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 41.5ns | 447583.6ns | 0.0% |  |
| warm-clamp-accfit | 29.9ns | 236752.8ns | 0.0% |  |
| warm-clamp-accfit-dyn | 109.2ns | 265122.7ns | 0.0% |  |
| warm-clamp-head | 154.5ns | 457545.6ns | 0.0% |  |
| warm-clamp-min-lanes | 31.1ns | 426188.9ns | 0.0% |  |
| warm-clamp-minimum | 70.6ns | 471314.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 397751.7-547105.5 ns)
  397751.7 |########################################
  405219.4 |###
  412687.1 |
  420154.8 |
  427622.4 |###
  435090.1 |###############
  442557.8 |###############
  450025.5 |
  457493.2 |
  464960.9 |
  472428.6 |
  479896.3 |
  487364.0 |
  494831.7 |
  502299.3 |
  509767.0 |
  517234.7 |
  524702.4 |#########
  532170.1 |###############
  539637.8 |###
  (5 below, 1 above range)

warm-clamp-accfit (n=40, range 184717.8-305293.9 ns)
  184717.8 |####
  190746.6 |
  196775.4 |########################################
  202804.2 |####
  208833.0 |
  214861.8 |####
  220890.6 |
  226919.5 |########
  232948.3 |####
  238977.1 |
  245005.9 |####
  251034.7 |################
  257063.5 |################
  263092.3 |####
  269121.1 |
  275149.9 |####
  281178.7 |
  287207.5 |
  293236.3 |####
  299265.1 |########
  (5 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 204916.2-331664.9 ns)
  204916.2 |
  211253.6 |
  217591.1 |######
  223928.5 |#############
  230265.9 |#############
  236603.4 |##########################
  242940.8 |####################
  249278.2 |
  255615.7 |########################################
  261953.1 |##########################
  268290.6 |######
  274628.0 |######
  280965.4 |
  287302.9 |
  293640.3 |
  299977.7 |
  306315.2 |##########################
  312652.6 |##########################
  318990.1 |
  325327.5 |
  (4 below, 4 above range)

warm-clamp-head (n=40, range 409826.4-588043.4 ns)
  409826.4 |########################################
  418737.2 |#########
  427648.1 |
  436558.9 |########################
  445469.8 |#########
  454380.6 |###
  463291.5 |
  472202.3 |
  481113.2 |###
  490024.0 |
  498934.9 |
  507845.7 |
  516756.6 |
  525667.4 |
  534578.3 |###
  543489.1 |
  552400.0 |
  561310.8 |
  570221.7 |
  579132.5 |
  (4 below, 6 above range)

warm-clamp-min-lanes (n=40, range 353996.5-515459.1 ns)
  353996.5 |##################
  362069.7 |
  370142.8 |
  378215.9 |
  386289.1 |
  394362.2 |
  402435.3 |
  410508.4 |##
  418581.6 |########################################
  426654.7 |########
  434727.8 |########
  442800.9 |##
  450874.1 |
  458947.2 |
  467020.3 |
  475093.5 |
  483166.6 |##
  491239.7 |########
  499312.8 |##
  507386.0 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 427774.5-522732.1 ns)
  427774.5 |##################
  432522.4 |###
  437270.3 |
  442018.2 |
  446766.0 |
  451513.9 |##########
  456261.8 |
  461009.7 |
  465757.6 |
  470505.4 |########################################
  475253.3 |#############################
  480001.2 |#######
  484749.1 |
  489496.9 |###
  494244.8 |
  498992.7 |
  503740.6 |###
  508488.5 |
  513236.3 |###
  517984.2 |###
  (4 below, 2 above range)

```
