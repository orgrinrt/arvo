# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe4 beats baseline by 20% (significant)

bitpack-contend-pipe4 is -134.75 us (20%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d32 shows alternating (throttle bounce) (autocorr -0.79)

bitpack-contend-d32's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-packed-simd, bitpack-contend-pipe4} vs {bitpack-contend-d16, bitpack-contend-d16-padal, bitpack-contend-d32-padal, bitpack-contend-d32} (25% apart)

The field splits into a fast tier {bitpack-contend-packed-simd, bitpack-contend-pipe4} and a slow tier {bitpack-contend-d16, bitpack-contend-d16-padal, bitpack-contend-d32-padal, bitpack-contend-d32} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-packed-simd** at 516249.2 ns median (-22.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.45x (fastest 516249.2 ns, slowest 750909.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112792238 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 654892ns | 665840ns | 638504ns | 655111ns | 670622ns | base |
| bitpack-contend-d16-padal | 674415ns | 676130ns | 665731ns | 675667ns | 679342ns | +2.98% |
| bitpack-contend-d32 | 749646ns | 751754ns | 694483ns | 746036ns | 815637ns | +14.47% |
| bitpack-contend-d32-padal | 759357ns | 749528ns | 690169ns | 751079ns | 853381ns | +15.95% |
| bitpack-contend-packed-simd | 523770ns | 516402ns | 515726ns | 522033ns | 537022ns | -20.02% |
| bitpack-contend-pipe4 | 528175ns | 530658ns | 518328ns | 529268ns | 534745ns | -19.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 654646ns | 638331ns | 670273ns | base | 6.407 |
| bitpack-contend-d16-padal | 674160ns | 665495ns | 679106ns | +2.98% | 6.222 |
| bitpack-contend-d32 | 748813ns | 693658ns | 814805ns | +14.38% | 5.601 |
| bitpack-contend-d32-padal | 758632ns | 689431ns | 852708ns | +15.88% | 5.529 |
| bitpack-contend-packed-simd | 523572ns | 515580ns | 536812ns | -20.02% | 8.011 |
| bitpack-contend-pipe4 | 527976ns | 518098ns | 534518ns | -19.35% | 7.944 |

## Performance model

- Peak throughput: **8.135 Gops/s** (bitpack-contend-packed-simd; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.302 | 77.5% |
| bitpack-contend-d16-padal | 6.206 | 76.3% |
| bitpack-contend-d32 | 5.586 | 68.7% |
| bitpack-contend-d32-padal | 5.601 | 68.9% |
| bitpack-contend-packed-simd | 8.125 | 99.9% |
| bitpack-contend-pipe4 | 7.906 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 654892ns | 654892ns | base |
| bitpack-contend-d16-padal | 674415ns | 674415ns | +2.98% |
| bitpack-contend-d32 | 749646ns | 749646ns | +14.47% |
| bitpack-contend-d32-padal | 759357ns | 759357ns | +15.95% |
| bitpack-contend-packed-simd | 523770ns | 523770ns | -20.02% |
| bitpack-contend-pipe4 | 528175ns | 528175ns | -19.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 665582ns | base | --- | [639380, 667848] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 675862ns | +11385.8ns (+1.7%) | [+8203, +33718]ns | [674362, 676814] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 750910ns | +83048.4ns (+12.5%) | [+57451, +125592]ns | [706553, 776685] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 748788ns | +93565.4ns (+14.1%) | [+56998, +123081]ns | [696424, 790414] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 516249ns | -128672.8ns (-19.3%) | [-131569, -122818]ns | [515744, 530618] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 530498ns | -134746.9ns (-20.2%) | [-138976, -109013]ns | [530338, 530622] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 667609ns | +1.4% | +14.1% | +18.3% | -19.6% | -22.4% |
| 1 | 2 | 638378ns | +6.1% | +9.2% | +7.9% | -19.2% | -16.3% |
| 1 | 3 | 639486ns | +5.3% | +25.1% | +8.9% | -17.9% | -17.0% |
| 1 | 4 | 668901ns | +0.3% | +5.0% | +27.3% | -22.9% | -20.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.249 | moderate- |
| bitpack-contend-d16-padal | 0.275 | moderate+ |
| bitpack-contend-d32 | -0.788 | HIGH- (thermal bounce) |
| bitpack-contend-d32-padal | -0.212 | moderate- |
| bitpack-contend-packed-simd | -0.442 | moderate- |
| bitpack-contend-pipe4 | -0.286 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 0/4, lost 4/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d32-padal**: won 0/4, lost 4/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 50.5ns | 654645.6ns | 0.0% |  |
| bitpack-contend-d16-padal | 41.3ns | 674159.8ns | 0.0% |  |
| bitpack-contend-d32 | 81.5ns | 748813.0ns | 0.0% |  |
| bitpack-contend-d32-padal | 49.1ns | 758632.1ns | 0.0% |  |
| bitpack-contend-packed-simd | 28.6ns | 523571.5ns | 0.0% |  |
| bitpack-contend-pipe4 | 19.3ns | 527975.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 638331.2-670273.0 ns)
  638331.2 |########################################
  639928.3 |##
  641525.4 |
  643122.5 |
  644719.6 |
  646316.7 |
  647913.8 |##
  649510.9 |
  651108.0 |
  652705.0 |
  654302.1 |
  655899.2 |
  657496.3 |
  659093.4 |
  660690.5 |
  662287.6 |
  663884.7 |
  665481.8 |###########
  667078.8 |############################
  668675.9 |##############
  (3 below, 2 above range)

bitpack-contend-d16-padal (n=40, range 665495.2-679106.3 ns)
  665495.2 |
  666175.8 |
  666856.3 |
  667536.9 |######
  668217.4 |######
  668898.0 |
  669578.6 |#############
  670259.1 |######
  670939.7 |
  671620.2 |#############
  672300.8 |#############
  672981.3 |#############
  673661.9 |######
  674342.4 |####################
  675023.0 |#############
  675703.5 |####################
  676384.1 |########################################
  677064.6 |##########################
  677745.2 |####################
  678425.7 |######
  (2 below, 4 above range)

bitpack-contend-d32 (n=40, range 693657.6-814804.9 ns)
  693657.6 |########################################
  699715.0 |########
  705772.3 |########
  711829.7 |
  717887.1 |####
  723944.4 |
  730001.8 |########
  736059.2 |
  742116.5 |
  748173.9 |#############
  754231.2 |####
  760288.6 |
  766346.0 |#############
  772403.3 |
  778460.7 |####
  784518.1 |
  790575.4 |####
  796632.8 |###############################
  802690.2 |#############
  808747.5 |
  (3 below, 2 above range)

bitpack-contend-d32-padal (n=40, range 689431.2-852707.7 ns)
  689431.2 |########################################
  697595.0 |############
  705758.9 |
  713922.7 |####
  722086.5 |
  730250.3 |
  738414.1 |
  746578.0 |
  754741.8 |
  762905.6 |
  771069.4 |
  779233.3 |####
  787397.1 |############################
  795560.9 |####
  803724.7 |
  811888.6 |####
  820052.4 |
  828216.2 |
  836380.0 |
  844543.8 |############################
  (6 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 515580.2-536812.4 ns)
  515580.2 |########################################
  516641.8 |
  517703.4 |
  518765.0 |##
  519826.6 |##
  520888.2 |
  521949.9 |##
  523011.5 |
  524073.1 |
  525134.7 |
  526196.3 |
  527257.9 |
  528319.5 |
  529381.1 |
  530442.7 |###############
  531504.3 |
  532565.9 |
  533627.5 |
  534689.1 |
  535750.8 |############
  (5 below, 5 above range)

bitpack-contend-pipe4 (n=40, range 518098.2-534518.4 ns)
  518098.2 |####################
  518919.2 |###
  519740.2 |
  520561.2 |
  521382.2 |###
  522203.2 |
  523024.3 |
  523845.3 |
  524666.3 |
  525487.3 |
  526308.3 |
  527129.3 |
  527950.3 |
  528771.3 |
  529592.4 |##########
  530413.4 |########################################
  531234.4 |######
  532055.4 |###
  532876.4 |###
  533697.4 |#######################
  (4 below, 2 above range)

```
