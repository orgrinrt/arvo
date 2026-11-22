# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe2 beats baseline by 21% (significant)

bitpack-contend-pipe2 is -141.27 us (21%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 530206.6 ns median (-20.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.26x (fastest 530206.6 ns, slowest 669791.2 ns)

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
| Counter frequency | 2112755232 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 670455ns | 670044ns | 639507ns | 663296ns | 722879ns | base |
| bitpack-contend-d16-control | 685161ns | 670155ns | 639874ns | 674373ns | 762812ns | +2.19% |
| bitpack-contend-packed-simd | 586222ns | 585490ns | 544722ns | 579754ns | 647126ns | -12.56% |
| bitpack-contend-pipe2 | 535287ns | 530374ns | 515168ns | 528925ns | 574491ns | -20.16% |
| bitpack-contend-pipe4 | 564150ns | 564751ns | 529086ns | 563079ns | 602427ns | -15.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 670131ns | 639269ns | 722369ns | base | 6.259 |
| bitpack-contend-d16-control | 684889ns | 639721ns | 762447ns | +2.20% | 6.124 |
| bitpack-contend-packed-simd | 585925ns | 544457ns | 646873ns | -12.57% | 7.158 |
| bitpack-contend-pipe2 | 535059ns | 514967ns | 574154ns | -20.16% | 7.839 |
| bitpack-contend-pipe4 | 563861ns | 528920ns | 602093ns | -15.86% | 7.439 |

## Performance model

- Peak throughput: **8.145 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.263 | 76.9% |
| bitpack-contend-d16-control | 6.262 | 76.9% |
| bitpack-contend-packed-simd | 7.168 | 88.0% |
| bitpack-contend-pipe2 | 7.911 | 97.1% |
| bitpack-contend-pipe4 | 7.431 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 670455ns | 670455ns | base |
| bitpack-contend-d16-control | 685161ns | 685161ns | +2.19% |
| bitpack-contend-packed-simd | 586222ns | 586222ns | -12.56% |
| bitpack-contend-pipe2 | 535287ns | 535287ns | -20.16% |
| bitpack-contend-pipe4 | 564150ns | 564150ns | -15.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 669709ns | base | --- | [640961, 671224] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 669791ns | no significant difference | [-1000, +30850]ns | [668742, 671922] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-packed-simd | 585114ns | -65362.7ns (-9.8%) | [-83720, -52769]ns | [573168, 586897] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 530207ns | -141268.9ns (-21.1%) | [-148491, -122425]ns | [517565, 530628] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 564465ns | -126849.3ns (-18.9%) | [-129591, -108899]ns | [531269, 593384] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 639861ns | +4.9% | -9.5% | -19.1% | -6.8% |
| 1 | 2 | 639285ns | +0.0% | -8.2% | -17.0% | -17.0% |
| 1 | 3 | 670926ns | -0.3% | -2.0% | -23.2% | -20.8% |
| 1 | 4 | 721991ns | +5.3% | -22.9% | -20.5% | -17.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.194 | ok |
| bitpack-contend-d16-control | 0.018 | ok |
| bitpack-contend-packed-simd | -0.486 | moderate- |
| bitpack-contend-pipe2 | -0.284 | moderate- |
| bitpack-contend-pipe4 | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 2/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 38.0ns | 670130.9ns | 0.0% |  |
| bitpack-contend-d16-control | 100.9ns | 684888.6ns | 0.0% |  |
| bitpack-contend-packed-simd | 30.4ns | 585925.3ns | 0.0% |  |
| bitpack-contend-pipe2 | 52.9ns | 535058.9ns | 0.0% |  |
| bitpack-contend-pipe4 | 22.5ns | 563861.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 639269.1-722369.2 ns)
  639269.1 |########################################
  643424.1 |###
  647579.1 |###
  651734.1 |
  655889.2 |
  660044.2 |
  664199.2 |###
  668354.2 |#################################
  672509.2 |
  676664.2 |###
  680819.2 |
  684974.2 |
  689129.2 |
  693284.2 |
  697439.2 |
  701594.2 |
  705749.2 |
  709904.2 |
  714059.2 |
  718214.2 |#######################
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 639721.1-762446.9 ns)
  639721.1 |########
  645857.4 |##
  651993.7 |
  658130.0 |
  664266.2 |########################################
  670402.5 |####################
  676538.8 |
  682675.1 |
  688811.4 |
  694947.7 |
  701084.0 |
  707220.3 |##
  713356.6 |
  719492.9 |
  725629.2 |
  731765.5 |
  737901.8 |##
  744038.0 |
  750174.3 |
  756310.6 |####################
  (5 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 544456.9-646873.0 ns)
  544456.9 |
  549577.7 |
  554698.5 |########################################
  559819.4 |
  564940.2 |####
  570061.0 |#################
  575181.8 |####
  580302.6 |########
  585423.4 |########################################
  590544.2 |####
  595665.0 |####
  600785.8 |########
  605906.6 |
  611027.4 |
  616148.2 |
  621269.0 |####
  626389.8 |
  631510.6 |
  636631.4 |
  641752.2 |
  (3 below, 6 above range)

bitpack-contend-pipe2 (n=40, range 514966.9-574154.1 ns)
  514966.9 |########################################
  517926.2 |############
  520885.6 |
  523845.0 |
  526804.3 |####
  529763.7 |########################################
  532723.0 |####
  535682.4 |
  538641.8 |
  541601.1 |
  544560.5 |
  547519.8 |
  550479.2 |
  553438.6 |
  556397.9 |
  559357.3 |
  562316.6 |
  565276.0 |
  568235.4 |
  571194.7 |################################
  (5 below, 2 above range)

bitpack-contend-pipe4 (n=40, range 528919.8-602093.1 ns)
  528919.8 |########################################
  532578.4 |
  536237.1 |##
  539895.8 |
  543554.4 |
  547213.1 |
  550871.8 |
  554530.4 |
  558189.1 |
  561847.8 |
  565506.4 |
  569165.1 |
  572823.8 |
  576482.5 |
  580141.1 |
  583799.8 |
  587458.5 |
  591117.1 |####################
  594775.8 |###############
  598434.5 |##
  (1 below, 3 above range)

```
