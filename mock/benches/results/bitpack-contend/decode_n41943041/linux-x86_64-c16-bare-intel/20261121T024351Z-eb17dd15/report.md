# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (672.79 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-pipe2 at 516.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-contend-packed-simd beats baseline by 23% (significant)

bitpack-contend-packed-simd is -154.76 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-pipe2, bitpack-contend-packed-simd) are a dead heat (<1%)

bitpack-contend-pipe2 (516.06 us) and bitpack-contend-packed-simd (516.56 us) differ by 0.10%, inside the noise, even though the wider field spreads 30.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-contend-pipe2, bitpack-contend-packed-simd, bitpack-contend-pipe4} vs {bitpack-contend-d16-control, bitpack-contend-d16} (29% apart)

The field splits into a fast tier {bitpack-contend-pipe2, bitpack-contend-packed-simd, bitpack-contend-pipe4} and a slow tier {bitpack-contend-d16-control, bitpack-contend-d16} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 516057.8 ns median (-23.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.30x (fastest 516057.8 ns, slowest 672789.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112791945 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 688047ns | 673067ns | 643462ns | 675888ns | 769107ns | base |
| bitpack-contend-d16-control | 685157ns | 672710ns | 669936ns | 675934ns | 728047ns | -0.42% |
| bitpack-contend-packed-simd | 518030ns | 516792ns | 515765ns | 517189ns | 522820ns | -24.71% |
| bitpack-contend-pipe2 | 525044ns | 516254ns | 515441ns | 519425ns | 551502ns | -23.69% |
| bitpack-contend-pipe4 | 545263ns | 521555ns | 516367ns | 530681ns | 617903ns | -20.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 687773ns | 643167ns | 768860ns | base | 6.098 |
| bitpack-contend-d16-control | 684886ns | 669713ns | 727695ns | -0.42% | 6.124 |
| bitpack-contend-packed-simd | 517848ns | 515588ns | 522634ns | -24.71% | 8.099 |
| bitpack-contend-pipe2 | 524824ns | 515287ns | 551189ns | -23.69% | 7.992 |
| bitpack-contend-pipe4 | 545068ns | 516198ns | 617612ns | -20.75% | 7.695 |

## Performance model

- Peak throughput: **8.140 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.234 | 76.6% |
| bitpack-contend-d16-control | 6.237 | 76.6% |
| bitpack-contend-packed-simd | 8.120 | 99.8% |
| bitpack-contend-pipe2 | 8.128 | 99.9% |
| bitpack-contend-pipe4 | 8.045 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 688047ns | 688047ns | base |
| bitpack-contend-d16-control | 685157ns | 685157ns | -0.42% |
| bitpack-contend-packed-simd | 518030ns | 518030ns | -24.71% |
| bitpack-contend-pipe2 | 525044ns | 525044ns | -23.69% |
| bitpack-contend-pipe4 | 545263ns | 545263ns | -20.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 672789ns | base | --- | [655696, 685724] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 672450ns | no significant difference | [-15772, +16288]ns | [671181, 674016] | no | 1.0000 | 1.0000 | 0 |
| bitpack-contend-packed-simd | 516560ns | -154762.4ns (-23.0%) | [-165369, -139715]ns | [515918, 517070] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 516058ns | -137414.1ns (-20.4%) | [-141398, -135134]ns | [515690, 518010] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 521382ns | -136322.0ns (-20.3%) | [-140384, -126665]ns | [516716, 531305] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 759738ns | -11.5% | -32.1% | -32.1% | -30.1% |
| 1 | 2 | 685460ns | -2.3% | -24.1% | -19.7% | -10.9% |
| 1 | 3 | 643479ns | +13.3% | -19.7% | -19.9% | -19.7% |
| 1 | 4 | 656253ns | +2.5% | -21.4% | -21.4% | -21.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.155 | ok |
| bitpack-contend-d16-control | -0.424 | moderate- |
| bitpack-contend-packed-simd | -0.356 | moderate- |
| bitpack-contend-pipe2 | -0.426 | moderate- |
| bitpack-contend-pipe4 | -0.312 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 31.9ns | 687773.4ns | 0.0% |  |
| bitpack-contend-d16-control | 32.2ns | 684886.4ns | 0.0% |  |
| bitpack-contend-packed-simd | 30.2ns | 517848.0ns | 0.0% |  |
| bitpack-contend-pipe2 | 32.9ns | 524824.1ns | 0.0% |  |
| bitpack-contend-pipe4 | 20.0ns | 545067.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 643166.7-768859.7 ns)
  643166.7 |###############################
  649451.3 |######################
  655736.0 |##########################
  662020.6 |
  668305.3 |
  674589.9 |
  680874.6 |########################################
  687159.2 |####
  693443.9 |
  699728.5 |
  706013.2 |
  712297.8 |
  718582.5 |
  724867.1 |
  731151.8 |####
  737436.4 |
  743721.1 |
  750005.7 |
  756290.4 |###############################
  762575.0 |####
  (2 below, 1 above range)

bitpack-contend-d16-control (n=40, range 669713.2-727695.5 ns)
  669713.2 |########################################
  672612.3 |##############
  675511.4 |##
  678410.5 |
  681309.6 |
  684208.8 |
  687107.9 |
  690007.0 |
  692906.1 |
  695805.2 |
  698704.3 |
  701603.5 |
  704502.6 |
  707401.7 |
  710300.8 |######
  713199.9 |
  716099.0 |
  718998.1 |
  721897.3 |##
  724796.4 |
  (3 below, 6 above range)

bitpack-contend-packed-simd (n=40, range 515587.8-522633.8 ns)
  515587.8 |########################################
  515940.1 |########
  516292.4 |#################
  516644.7 |#############
  516997.0 |#################
  517349.3 |
  517701.6 |
  518053.9 |
  518406.2 |
  518758.5 |
  519110.8 |#############
  519463.1 |#############
  519815.4 |####
  520167.7 |########
  520520.0 |####
  520872.3 |
  521224.6 |
  521576.9 |
  521929.2 |########
  522281.5 |
  (5 below, 1 above range)

bitpack-contend-pipe2 (n=40, range 515286.8-551189.1 ns)
  515286.8 |########################################
  517081.9 |#
  518877.0 |###
  520672.1 |###
  522467.2 |
  524262.3 |
  526057.4 |
  527852.6 |
  529647.7 |
  531442.8 |
  533237.9 |
  535033.0 |
  536828.1 |
  538623.3 |
  540418.4 |
  542213.5 |#
  544008.6 |
  545803.7 |
  547598.8 |#
  549393.9 |############
  (3 below, 1 above range)

bitpack-contend-pipe4 (n=40, range 516198.3-617612.1 ns)
  516198.3 |########################################
  521269.0 |##
  526339.6 |###############
  531410.3 |####
  536481.0 |
  541551.7 |
  546622.4 |
  551693.1 |
  556763.8 |
  561834.5 |
  566905.2 |
  571975.9 |
  577046.6 |
  582117.3 |
  587187.9 |
  592258.6 |
  597329.3 |
  602400.0 |
  607470.7 |#################
  612541.4 |
  (2 below, 2 above range)

```
