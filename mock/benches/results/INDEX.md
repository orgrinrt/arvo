# Bench Results Index

Generated from the runs under `results/`, one directory per run at `<bench>/<cell>/<machine class>/<run>/`, and regenerated from them on every run. Each run holds its samples, meta, report and summary.

## bit-set/bit-set_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210524Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bit_set_host | 76 ns | 1.00x |
| bit_set_tier | 76 ns | 1.00x |

- [20261120T210524Z-eb17dd15](bit-set/bit-set_n256/linux-x86_64-c16-bare-intel/20261120T210524Z-eb17dd15/report.md)

## bit-set/bit-set_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210524Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bit_set_host | 61 ns | 1.32x |
| bit_set_tier | 46 ns | 1.00x |

- [20261120T210524Z-eb17dd15](bit-set/bit-set_n64/linux-x86_64-c16-bare-intel/20261120T210524Z-eb17dd15/report.md)

## bitpack-carrier-width/bitpack-carrier-width_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210813Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-carrier-d16 | 163.57 us | 1.36x |
| bitpack-carrier-d16-control | 159.50 us | 1.33x |
| bitpack-carrier-d32 | 120.00 us | 1.00x |
| bitpack-carrier-d64 | 196.72 us | 1.64x |
| bitpack-carrier-packed | 127.76 us | 1.06x |
| bitpack-carrier-packed-simd | 131.78 us | 1.10x |

- [20261120T210813Z-eb17dd15](bitpack-carrier-width/bitpack-carrier-width_n1048576/linux-x86_64-c16-bare-intel/20261120T210813Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T105223Z-fbac19c-dirty](bitpack-carrier-width/bitpack-carrier-width_n1048576/macos-aarch64-c8-bare-apple/20260808T105223Z-fbac19c-dirty/report.md)

## bitpack-carrier-width/bitpack-carrier-width_n131072

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210813Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-carrier-d16 | 19.94 us | 1.62x |
| bitpack-carrier-d16-control | 19.94 us | 1.62x |
| bitpack-carrier-d32 | 15.01 us | 1.22x |
| bitpack-carrier-d64 | 12.29 us | 1.00x |
| bitpack-carrier-packed | 17.88 us | 1.46x |
| bitpack-carrier-packed-simd | 16.06 us | 1.31x |

- [20261120T210813Z-eb17dd15](bitpack-carrier-width/bitpack-carrier-width_n131072/linux-x86_64-c16-bare-intel/20261120T210813Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T104942Z-fbac19c-dirty](bitpack-carrier-width/bitpack-carrier-width_n131072/macos-aarch64-c8-bare-apple/20260808T104942Z-fbac19c-dirty/report.md)

## bitpack-carrier-width/bitpack-carrier-width_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210813Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-carrier-d16 | 2.51 us | 1.32x |
| bitpack-carrier-d16-control | 3.06 us | 1.62x |
| bitpack-carrier-d32 | 1.90 us | 1.00x |
| bitpack-carrier-d64 | 3.73 us | 1.97x |
| bitpack-carrier-packed | 2.44 us | 1.29x |
| bitpack-carrier-packed-simd | 2.46 us | 1.30x |

- [20261120T210813Z-eb17dd15](bitpack-carrier-width/bitpack-carrier-width_n16384/linux-x86_64-c16-bare-intel/20261120T210813Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T104823Z-fbac19c-dirty](bitpack-carrier-width/bitpack-carrier-width_n16384/macos-aarch64-c8-bare-apple/20260808T104823Z-fbac19c-dirty/report.md)

## bitpack-carrier-width/bitpack-carrier-width_n2097152

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210813Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-carrier-d16 | 319.10 us | 1.25x |
| bitpack-carrier-d16-control | 327.26 us | 1.28x |
| bitpack-carrier-d32 | 296.73 us | 1.16x |
| bitpack-carrier-d64 | 558.74 us | 2.18x |
| bitpack-carrier-packed | 255.78 us | 1.00x |
| bitpack-carrier-packed-simd | 261.70 us | 1.02x |

- [20261120T210813Z-eb17dd15](bitpack-carrier-width/bitpack-carrier-width_n2097152/linux-x86_64-c16-bare-intel/20261120T210813Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T105642Z-fbac19c-dirty](bitpack-carrier-width/bitpack-carrier-width_n2097152/macos-aarch64-c8-bare-apple/20260808T105642Z-fbac19c-dirty/report.md)

## bitpack-carrier-width/bitpack-carrier-width_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210813Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-carrier-d16 | 693.47 us | 1.36x |
| bitpack-carrier-d16-control | 656.34 us | 1.28x |
| bitpack-carrier-d32 | 699.79 us | 1.37x |
| bitpack-carrier-d64 | 1.38 ms | 2.69x |
| bitpack-carrier-packed | 511.25 us | 1.00x |
| bitpack-carrier-packed-simd | 592.39 us | 1.16x |

- [20261120T210813Z-eb17dd15](bitpack-carrier-width/bitpack-carrier-width_n4194304/linux-x86_64-c16-bare-intel/20261120T210813Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T110434Z-fbac19c-dirty](bitpack-carrier-width/bitpack-carrier-width_n4194304/macos-aarch64-c8-bare-apple/20260808T110434Z-fbac19c-dirty/report.md)

## bitpack-carrier-width/bitpack-carrier-width_n8388608

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T210813Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-carrier-d16 | 1.42 ms | 1.21x |
| bitpack-carrier-d16-control | 1.42 ms | 1.21x |
| bitpack-carrier-d32 | 1.66 ms | 1.42x |
| bitpack-carrier-d64 | 2.85 ms | 2.44x |
| bitpack-carrier-packed | 1.17 ms | 1.00x |
| bitpack-carrier-packed-simd | 1.18 ms | 1.01x |

- [20261120T210813Z-eb17dd15](bitpack-carrier-width/bitpack-carrier-width_n8388608/linux-x86_64-c16-bare-intel/20261120T210813Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T111844Z-fbac19c-dirty](bitpack-carrier-width/bitpack-carrier-width_n8388608/macos-aarch64-c8-bare-apple/20260808T111844Z-fbac19c-dirty/report.md)

## bitpack-contend/best_n41943041

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T221231Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 666.54 us | 1.29x |
| bitpack-contend-d16-padal | 648.08 us | 1.25x |
| bitpack-contend-d32 | 700.86 us | 1.36x |
| bitpack-contend-d32-padal | 706.93 us | 1.37x |
| bitpack-contend-packed-simd | 516.84 us | 1.00x |
| bitpack-contend-pipe4 | 529.19 us | 1.02x |

- [20261120T221231Z-eb17dd15](bitpack-contend/best_n41943041/linux-x86_64-c16-bare-intel/20261120T221231Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T141230Z-0988bb7-dirty](bitpack-contend/best_n41943041/macos-aarch64-c8-bare-apple/20260808T141230Z-0988bb7-dirty/report.md)

## bitpack-contend/best_n41943044

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T221231Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 263.23 us | 1.48x |
| bitpack-contend-d16-padal | 256.29 us | 1.44x |
| bitpack-contend-d32 | 308.54 us | 1.73x |
| bitpack-contend-d32-padal | 319.79 us | 1.79x |
| bitpack-contend-packed-simd | 189.37 us | 1.06x |
| bitpack-contend-pipe4 | 178.42 us | 1.00x |

- [20261120T221231Z-eb17dd15](bitpack-contend/best_n41943044/linux-x86_64-c16-bare-intel/20261120T221231Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T142238Z-0988bb7-dirty](bitpack-contend/best_n41943044/macos-aarch64-c8-bare-apple/20260808T142238Z-0988bb7-dirty/report.md)

## bitpack-contend/best_n83886081

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T221231Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 1.42 ms | 1.24x |
| bitpack-contend-d16-padal | 1.43 ms | 1.24x |
| bitpack-contend-d32 | 1.65 ms | 1.43x |
| bitpack-contend-d32-padal | 1.64 ms | 1.42x |
| bitpack-contend-packed-simd | 1.19 ms | 1.03x |
| bitpack-contend-pipe4 | 1.15 ms | 1.00x |

- [20261120T221231Z-eb17dd15](bitpack-contend/best_n83886081/linux-x86_64-c16-bare-intel/20261120T221231Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T143815Z-0988bb7-dirty](bitpack-contend/best_n83886081/macos-aarch64-c8-bare-apple/20260808T143815Z-0988bb7-dirty/report.md)

## bitpack-contend/best_n83886084

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T221231Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 593.41 us | 1.29x |
| bitpack-contend-d16-padal | 592.37 us | 1.28x |
| bitpack-contend-d32 | 827.38 us | 1.79x |
| bitpack-contend-d32-padal | 837.76 us | 1.82x |
| bitpack-contend-packed-simd | 461.88 us | 1.00x |
| bitpack-contend-pipe4 | 461.07 us | 1.00x |

- [20261120T221231Z-eb17dd15](bitpack-contend/best_n83886084/linux-x86_64-c16-bare-intel/20261120T221231Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T145710Z-0988bb7-dirty](bitpack-contend/best_n83886084/macos-aarch64-c8-bare-apple/20260808T145710Z-0988bb7-dirty/report.md)

## bitpack-contend/contention_n10485761

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 166.54 us | 1.29x |
| bitpack-contend-d16-control | 165.59 us | 1.28x |
| bitpack-contend-d32 | 129.30 us | 1.00x |
| bitpack-contend-d64 | 215.73 us | 1.67x |
| bitpack-contend-packed | 132.38 us | 1.02x |
| bitpack-contend-packed-simd | 130.74 us | 1.01x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n10485761/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T115608Z-3454060-dirty](bitpack-contend/contention_n10485761/macos-aarch64-c8-bare-apple/20260808T115608Z-3454060-dirty/report.md)

## bitpack-contend/contention_n10485762

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 82.75 us | 1.26x |
| bitpack-contend-d16-control | 84.35 us | 1.29x |
| bitpack-contend-d32 | 65.80 us | 1.01x |
| bitpack-contend-d64 | 112.27 us | 1.72x |
| bitpack-contend-packed | 65.44 us | 1.00x |
| bitpack-contend-packed-simd | 65.91 us | 1.01x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n10485762/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T115846Z-3454060-dirty](bitpack-contend/contention_n10485762/macos-aarch64-c8-bare-apple/20260808T115846Z-3454060-dirty/report.md)

## bitpack-contend/contention_n10485764

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 62.55 us | 1.40x |
| bitpack-contend-d16-control | 62.55 us | 1.40x |
| bitpack-contend-d32 | 45.11 us | 1.01x |
| bitpack-contend-d64 | 50.58 us | 1.13x |
| bitpack-contend-packed | 44.80 us | 1.00x |
| bitpack-contend-packed-simd | 47.71 us | 1.06x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n10485764/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T120135Z-3454060-dirty](bitpack-contend/contention_n10485764/macos-aarch64-c8-bare-apple/20260808T120135Z-3454060-dirty/report.md)

## bitpack-contend/contention_n163841

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 3.05 us | 2.23x |
| bitpack-contend-d16-control | 3.07 us | 2.24x |
| bitpack-contend-d32 | 2.31 us | 1.68x |
| bitpack-contend-d64 | 1.37 us | 1.00x |
| bitpack-contend-packed | 2.47 us | 1.80x |
| bitpack-contend-packed-simd | 2.47 us | 1.80x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n163841/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T115115Z-3454060-dirty](bitpack-contend/contention_n163841/macos-aarch64-c8-bare-apple/20260808T115115Z-3454060-dirty/report.md)

## bitpack-contend/contention_n163842

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 1.95 us | 1.19x |
| bitpack-contend-d16-control | 1.71 us | 1.04x |
| bitpack-contend-d32 | 3.16 us | 1.93x |
| bitpack-contend-d64 | 2.38 us | 1.45x |
| bitpack-contend-packed | 1.79 us | 1.09x |
| bitpack-contend-packed-simd | 1.64 us | 1.00x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n163842/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T115224Z-3454060-dirty](bitpack-contend/contention_n163842/macos-aarch64-c8-bare-apple/20260808T115224Z-3454060-dirty/report.md)

## bitpack-contend/contention_n163844

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 1.40 us | 1.00x |
| bitpack-contend-d16-control | 2.29 us | 1.64x |
| bitpack-contend-d32 | 2.60 us | 1.86x |
| bitpack-contend-d64 | 2.47 us | 1.76x |
| bitpack-contend-packed | 2.71 us | 1.94x |
| bitpack-contend-packed-simd | 2.96 us | 2.12x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n163844/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T115328Z-3454060-dirty](bitpack-contend/contention_n163844/macos-aarch64-c8-bare-apple/20260808T115328Z-3454060-dirty/report.md)

## bitpack-contend/contention_n41943041

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 709.23 us | 1.37x |
| bitpack-contend-d16-control | 662.89 us | 1.28x |
| bitpack-contend-d32 | 778.25 us | 1.51x |
| bitpack-contend-d64 | 1.34 ms | 2.60x |
| bitpack-contend-packed | 532.49 us | 1.03x |
| bitpack-contend-packed-simd | 516.24 us | 1.00x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n41943041/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T120917Z-3454060-dirty](bitpack-contend/contention_n41943041/macos-aarch64-c8-bare-apple/20260808T120917Z-3454060-dirty/report.md)

## bitpack-contend/contention_n41943042

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 412.42 us | 1.35x |
| bitpack-contend-d16-control | 405.27 us | 1.33x |
| bitpack-contend-d32 | 431.84 us | 1.42x |
| bitpack-contend-d64 | 874.43 us | 2.87x |
| bitpack-contend-packed | 304.74 us | 1.00x |
| bitpack-contend-packed-simd | 330.27 us | 1.08x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n41943042/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T121656Z-3454060-dirty](bitpack-contend/contention_n41943042/macos-aarch64-c8-bare-apple/20260808T121656Z-3454060-dirty/report.md)

## bitpack-contend/contention_n41943044

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 251.95 us | 1.29x |
| bitpack-contend-d16-control | 271.86 us | 1.39x |
| bitpack-contend-d32 | 313.21 us | 1.61x |
| bitpack-contend-d64 | 673.08 us | 3.45x |
| bitpack-contend-packed | 195.05 us | 1.00x |
| bitpack-contend-packed-simd | 200.10 us | 1.03x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n41943044/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T122600Z-3454060-dirty](bitpack-contend/contention_n41943044/macos-aarch64-c8-bare-apple/20260808T122600Z-3454060-dirty/report.md)

## bitpack-contend/contention_n83886081

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 1.60 ms | 1.39x |
| bitpack-contend-d16-control | 1.42 ms | 1.23x |
| bitpack-contend-d32 | 1.63 ms | 1.42x |
| bitpack-contend-d64 | 2.88 ms | 2.50x |
| bitpack-contend-packed | 1.15 ms | 1.00x |
| bitpack-contend-packed-simd | 1.17 ms | 1.01x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n83886081/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T124012Z-3454060-dirty](bitpack-contend/contention_n83886081/macos-aarch64-c8-bare-apple/20260808T124012Z-3454060-dirty/report.md)

## bitpack-contend/contention_n83886082

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 883.20 us | 1.25x |
| bitpack-contend-d16-control | 890.48 us | 1.26x |
| bitpack-contend-d32 | 1.06 ms | 1.51x |
| bitpack-contend-d64 | 1.90 ms | 2.69x |
| bitpack-contend-packed | 704.61 us | 1.00x |
| bitpack-contend-packed-simd | 762.22 us | 1.08x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n83886082/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T125513Z-3454060-dirty](bitpack-contend/contention_n83886082/macos-aarch64-c8-bare-apple/20260808T125513Z-3454060-dirty/report.md)

## bitpack-contend/contention_n83886084

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261120T235005Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 532.13 us | 1.15x |
| bitpack-contend-d16-control | 578.56 us | 1.25x |
| bitpack-contend-d32 | 837.74 us | 1.82x |
| bitpack-contend-d64 | 1.47 ms | 3.19x |
| bitpack-contend-packed | 461.53 us | 1.00x |
| bitpack-contend-packed-simd | 464.00 us | 1.01x |

- [20261120T235005Z-eb17dd15](bitpack-contend/contention_n83886084/linux-x86_64-c16-bare-intel/20261120T235005Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T131256Z-3454060-dirty](bitpack-contend/contention_n83886084/macos-aarch64-c8-bare-apple/20260808T131256Z-3454060-dirty/report.md)

## bitpack-contend/decode_n163841

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T024351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 3.07 us | 1.24x |
| bitpack-contend-d16-control | 2.53 us | 1.03x |
| bitpack-contend-packed-simd | 2.47 us | 1.00x |
| bitpack-contend-pipe2 | 2.46 us | 1.00x |
| bitpack-contend-pipe4 | 2.47 us | 1.00x |

- [20261121T024351Z-eb17dd15](bitpack-contend/decode_n163841/linux-x86_64-c16-bare-intel/20261121T024351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T132034Z-7da8523-dirty](bitpack-contend/decode_n163841/macos-aarch64-c8-bare-apple/20260808T132034Z-7da8523-dirty/report.md)

## bitpack-contend/decode_n163844

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T024351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 3.63 us | 1.84x |
| bitpack-contend-d16-control | 1.97 us | 1.00x |
| bitpack-contend-packed-simd | 2.88 us | 1.46x |
| bitpack-contend-pipe2 | 2.60 us | 1.32x |
| bitpack-contend-pipe4 | 2.81 us | 1.43x |

- [20261121T024351Z-eb17dd15](bitpack-contend/decode_n163844/linux-x86_64-c16-bare-intel/20261121T024351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T132133Z-7da8523-dirty](bitpack-contend/decode_n163844/macos-aarch64-c8-bare-apple/20260808T132133Z-7da8523-dirty/report.md)

## bitpack-contend/decode_n41943041

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T024351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 672.79 us | 1.30x |
| bitpack-contend-d16-control | 672.45 us | 1.30x |
| bitpack-contend-packed-simd | 516.56 us | 1.00x |
| bitpack-contend-pipe2 | 516.06 us | 1.00x |
| bitpack-contend-pipe4 | 521.38 us | 1.01x |

- [20261121T024351Z-eb17dd15](bitpack-contend/decode_n41943041/linux-x86_64-c16-bare-intel/20261121T024351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T132802Z-7da8523-dirty](bitpack-contend/decode_n41943041/macos-aarch64-c8-bare-apple/20260808T132802Z-7da8523-dirty/report.md)

## bitpack-contend/decode_n41943044

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T024351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 272.60 us | 1.37x |
| bitpack-contend-d16-control | 253.72 us | 1.27x |
| bitpack-contend-packed-simd | 199.85 us | 1.00x |
| bitpack-contend-pipe2 | 200.56 us | 1.00x |
| bitpack-contend-pipe4 | 199.63 us | 1.00x |

- [20261121T024351Z-eb17dd15](bitpack-contend/decode_n41943044/linux-x86_64-c16-bare-intel/20261121T024351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T133554Z-7da8523-dirty](bitpack-contend/decode_n41943044/macos-aarch64-c8-bare-apple/20260808T133554Z-7da8523-dirty/report.md)

## bitpack-contend/decode_n83886081

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T024351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 1.42 ms | 1.23x |
| bitpack-contend-d16-control | 1.42 ms | 1.23x |
| bitpack-contend-packed-simd | 1.16 ms | 1.00x |
| bitpack-contend-pipe2 | 1.16 ms | 1.00x |
| bitpack-contend-pipe4 | 1.17 ms | 1.01x |

- [20261121T024351Z-eb17dd15](bitpack-contend/decode_n83886081/linux-x86_64-c16-bare-intel/20261121T024351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T134757Z-7da8523-dirty](bitpack-contend/decode_n83886081/macos-aarch64-c8-bare-apple/20260808T134757Z-7da8523-dirty/report.md)

## bitpack-contend/decode_n83886084

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T024351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-contend-d16 | 572.71 us | 1.34x |
| bitpack-contend-d16-control | 595.67 us | 1.39x |
| bitpack-contend-packed-simd | 463.56 us | 1.08x |
| bitpack-contend-pipe2 | 428.97 us | 1.00x |
| bitpack-contend-pipe4 | 467.14 us | 1.09x |

- [20261121T024351Z-eb17dd15](bitpack-contend/decode_n83886084/linux-x86_64-c16-bare-intel/20261121T024351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T140234Z-7da8523-dirty](bitpack-contend/decode_n83886084/macos-aarch64-c8-bare-apple/20260808T140234Z-7da8523-dirty/report.md)

## bitpack-decoder-shape/bitpack-decoder-shape_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T041041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-plan-naive | 8.55 us | 4.28x |
| bitpack-plan-native | 2.50 us | 1.25x |
| bitpack-plan-simd | 2.00 us | 1.00x |
| bitpack-plan-windowed | 2.00 us | 1.00x |

- [20261121T041041Z-eb17dd15](bitpack-decoder-shape/bitpack-decoder-shape_n16384/linux-x86_64-c16-bare-intel/20261121T041041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T165954Z-3d8f615-dirty](bitpack-decoder-shape/bitpack-decoder-shape_n16384/macos-aarch64-c8-bare-apple/20260804T165954Z-3d8f615-dirty/report.md)

## bitpack-decoder-shape/bitpack-decoder-shape_n262144

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T041041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-plan-naive | 142.52 us | 4.42x |
| bitpack-plan-native | 39.92 us | 1.24x |
| bitpack-plan-simd | 32.21 us | 1.00x |
| bitpack-plan-windowed | 32.21 us | 1.00x |

- [20261121T041041Z-eb17dd15](bitpack-decoder-shape/bitpack-decoder-shape_n262144/linux-x86_64-c16-bare-intel/20261121T041041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170031Z-3d8f615-dirty](bitpack-decoder-shape/bitpack-decoder-shape_n262144/macos-aarch64-c8-bare-apple/20260804T170031Z-3d8f615-dirty/report.md)

## bitpack-decoder-shape/bitpack-decoder-shape_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T041041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-plan-naive | 35.57 us | 4.42x |
| bitpack-plan-native | 9.98 us | 1.24x |
| bitpack-plan-simd | 8.05 us | 1.00x |
| bitpack-plan-windowed | 8.05 us | 1.00x |

- [20261121T041041Z-eb17dd15](bitpack-decoder-shape/bitpack-decoder-shape_n65536/linux-x86_64-c16-bare-intel/20261121T041041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170001Z-3d8f615-dirty](bitpack-decoder-shape/bitpack-decoder-shape_n65536/macos-aarch64-c8-bare-apple/20260804T170001Z-3d8f615-dirty/report.md)

## bitpack-decoder-shape/bitpack-decoder-shape_n98304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T041041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-plan-naive | 51.97 us | 4.31x |
| bitpack-plan-native | 14.95 us | 1.24x |
| bitpack-plan-simd | 12.07 us | 1.00x |
| bitpack-plan-windowed | 12.07 us | 1.00x |

- [20261121T041041Z-eb17dd15](bitpack-decoder-shape/bitpack-decoder-shape_n98304/linux-x86_64-c16-bare-intel/20261121T041041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170010Z-3d8f615-dirty](bitpack-decoder-shape/bitpack-decoder-shape_n98304/macos-aarch64-c8-bare-apple/20260804T170010Z-3d8f615-dirty/report.md)

## bitpack-footprint/dense_n1048576

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T041844Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 163.62 us | 1.01x |
| bitpack-footprint-dense-alt | 162.11 us | 1.00x |

- [20261114T113539Z-d0161800](bitpack-footprint/dense_n1048576/linux-x86_64-c16-bare-intel/20261114T113539Z-d0161800/report.md)
- [20261121T041844Z-eb17dd15](bitpack-footprint/dense_n1048576/linux-x86_64-c16-bare-intel/20261121T041844Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T021722Z-75710b6-dirty](bitpack-footprint/dense_n1048576/macos-aarch64-c8-bare-apple/20260805T021722Z-75710b6-dirty/report.md)

## bitpack-footprint/dense_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T041844Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 2.51 us | 1.00x |
| bitpack-footprint-dense-alt | 2.51 us | 1.00x |

- [20261114T113300Z-d0161800](bitpack-footprint/dense_n16384/linux-x86_64-c16-bare-intel/20261114T113300Z-d0161800/report.md)
- [20261121T041844Z-eb17dd15](bitpack-footprint/dense_n16384/linux-x86_64-c16-bare-intel/20261121T041844Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T021605Z-75710b6-dirty](bitpack-footprint/dense_n16384/macos-aarch64-c8-bare-apple/20260805T021605Z-75710b6-dirty/report.md)

## bitpack-footprint/dense_n33554432

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T041844Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 5.91 ms | 1.00x |
| bitpack-footprint-dense-alt | 5.92 ms | 1.00x |

- [20261114T121047Z-d0161800](bitpack-footprint/dense_n33554432/linux-x86_64-c16-bare-intel/20261114T121047Z-d0161800/report.md)
- [20261121T041844Z-eb17dd15](bitpack-footprint/dense_n33554432/linux-x86_64-c16-bare-intel/20261121T041844Z-eb17dd15/report.md)

## bitpack-footprint/dense_n4194304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T041844Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 679.09 us | 1.00x |
| bitpack-footprint-dense-alt | 676.92 us | 1.00x |

- [20261114T113924Z-d0161800](bitpack-footprint/dense_n4194304/linux-x86_64-c16-bare-intel/20261114T113924Z-d0161800/report.md)
- [20261121T041844Z-eb17dd15](bitpack-footprint/dense_n4194304/linux-x86_64-c16-bare-intel/20261121T041844Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T022003Z-75710b6-dirty](bitpack-footprint/dense_n4194304/macos-aarch64-c8-bare-apple/20260805T022003Z-75710b6-dirty/report.md)

## bitpack-footprint/dense_n65536

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T041844Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 11.09 us | 1.00x |
| bitpack-footprint-dense-alt | 11.08 us | 1.00x |

- [20261114T113402Z-d0161800](bitpack-footprint/dense_n65536/linux-x86_64-c16-bare-intel/20261114T113402Z-d0161800/report.md)
- [20261121T041844Z-eb17dd15](bitpack-footprint/dense_n65536/linux-x86_64-c16-bare-intel/20261121T041844Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T021627Z-75710b6-dirty](bitpack-footprint/dense_n65536/macos-aarch64-c8-bare-apple/20260805T021627Z-75710b6-dirty/report.md)

## bitpack-footprint/dense_n7000000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T041844Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 1.19 ms | 1.00x |
| bitpack-footprint-dense-alt | 1.24 ms | 1.04x |

- [20261114T114504Z-d0161800](bitpack-footprint/dense_n7000000/linux-x86_64-c16-bare-intel/20261114T114504Z-d0161800/report.md)
- [20261121T041844Z-eb17dd15](bitpack-footprint/dense_n7000000/linux-x86_64-c16-bare-intel/20261121T041844Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T022433Z-350953f-dirty](bitpack-footprint/dense_n7000000/macos-aarch64-c8-bare-apple/20260805T022433Z-350953f-dirty/report.md)

## bitpack-footprint/headtohead_n1048576

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T045728Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 163.83 us | 1.24x |
| bitpack-footprint-dense-alt | 163.54 us | 1.24x |
| bitpack-footprint-packed | 131.78 us | 1.00x |
| bitpack-footprint-packed-naive | 668.51 us | 5.07x |

- [20261114T121814Z-d0161800](bitpack-footprint/headtohead_n1048576/linux-x86_64-c16-bare-intel/20261114T121814Z-d0161800/report.md)
- [20261121T045728Z-eb17dd15](bitpack-footprint/headtohead_n1048576/linux-x86_64-c16-bare-intel/20261121T045728Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T103316Z-f32abe4-dirty](bitpack-footprint/headtohead_n1048576/macos-aarch64-c8-bare-apple/20260808T103316Z-f32abe4-dirty/report.md)

## bitpack-footprint/headtohead_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T045728Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 3.05 us | 1.25x |
| bitpack-footprint-dense-alt | 3.05 us | 1.25x |
| bitpack-footprint-packed | 2.45 us | 1.00x |
| bitpack-footprint-packed-naive | 11.60 us | 4.75x |

- [20261114T121306Z-d0161800](bitpack-footprint/headtohead_n16384/linux-x86_64-c16-bare-intel/20261114T121306Z-d0161800/report.md)
- [20261121T045728Z-eb17dd15](bitpack-footprint/headtohead_n16384/linux-x86_64-c16-bare-intel/20261121T045728Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T103048Z-f32abe4-dirty](bitpack-footprint/headtohead_n16384/macos-aarch64-c8-bare-apple/20260808T103048Z-f32abe4-dirty/report.md)

## bitpack-footprint/headtohead_n4194304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T045728Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 666.13 us | 1.26x |
| bitpack-footprint-dense-alt | 680.15 us | 1.29x |
| bitpack-footprint-packed | 528.28 us | 1.00x |
| bitpack-footprint-packed-naive | 2.70 ms | 5.11x |

- [20261114T122557Z-d0161800](bitpack-footprint/headtohead_n4194304/linux-x86_64-c16-bare-intel/20261114T122557Z-d0161800/report.md)
- [20261121T045728Z-eb17dd15](bitpack-footprint/headtohead_n4194304/linux-x86_64-c16-bare-intel/20261121T045728Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T103813Z-f32abe4-dirty](bitpack-footprint/headtohead_n4194304/macos-aarch64-c8-bare-apple/20260808T103813Z-f32abe4-dirty/report.md)

## bitpack-footprint/headtohead_n65536

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T045728Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 11.09 us | 1.13x |
| bitpack-footprint-dense-alt | 11.08 us | 1.13x |
| bitpack-footprint-packed | 9.84 us | 1.00x |
| bitpack-footprint-packed-naive | 47.54 us | 4.83x |

- [20261114T121503Z-d0161800](bitpack-footprint/headtohead_n65536/linux-x86_64-c16-bare-intel/20261114T121503Z-d0161800/report.md)
- [20261121T045728Z-eb17dd15](bitpack-footprint/headtohead_n65536/linux-x86_64-c16-bare-intel/20261121T045728Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T103132Z-f32abe4-dirty](bitpack-footprint/headtohead_n65536/macos-aarch64-c8-bare-apple/20260808T103132Z-f32abe4-dirty/report.md)

## bitpack-footprint/headtohead_n7000000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T045728Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-dense | 1.24 ms | 1.29x |
| bitpack-footprint-dense-alt | 1.20 ms | 1.25x |
| bitpack-footprint-packed | 958.52 us | 1.00x |
| bitpack-footprint-packed-naive | 4.53 ms | 4.73x |

- [20261114T123736Z-d0161800](bitpack-footprint/headtohead_n7000000/linux-x86_64-c16-bare-intel/20261114T123736Z-d0161800/report.md)
- [20261121T045728Z-eb17dd15](bitpack-footprint/headtohead_n7000000/linux-x86_64-c16-bare-intel/20261121T045728Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T104559Z-fbac19c-dirty](bitpack-footprint/headtohead_n7000000/macos-aarch64-c8-bare-apple/20260808T104559Z-fbac19c-dirty/report.md)

## bitpack-footprint/packed_n1048576

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T052421Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-packed | 131.59 us | 1.00x |
| bitpack-footprint-packed-naive | 679.80 us | 5.17x |

- [20261114T124140Z-d0161800](bitpack-footprint/packed_n1048576/linux-x86_64-c16-bare-intel/20261114T124140Z-d0161800/report.md)
- [20261121T052421Z-eb17dd15](bitpack-footprint/packed_n1048576/linux-x86_64-c16-bare-intel/20261121T052421Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T020712Z-75710b6-dirty](bitpack-footprint/packed_n1048576/macos-aarch64-c8-bare-apple/20260805T020712Z-75710b6-dirty/report.md)

## bitpack-footprint/packed_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T052421Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-packed | 2.45 us | 1.00x |
| bitpack-footprint-packed-naive | 11.55 us | 4.72x |

- [20261114T123856Z-d0161800](bitpack-footprint/packed_n16384/linux-x86_64-c16-bare-intel/20261114T123856Z-d0161800/report.md)
- [20261121T052421Z-eb17dd15](bitpack-footprint/packed_n16384/linux-x86_64-c16-bare-intel/20261121T052421Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T020541Z-75710b6-dirty](bitpack-footprint/packed_n16384/macos-aarch64-c8-bare-apple/20260805T020541Z-75710b6-dirty/report.md)

## bitpack-footprint/packed_n33554432

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T052421Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-packed | 4.75 ms | 1.00x |
| bitpack-footprint-packed-naive | 21.87 ms | 4.60x |

- [20261114T131859Z-d0161800](bitpack-footprint/packed_n33554432/linux-x86_64-c16-bare-intel/20261114T131859Z-d0161800/report.md)
- [20261121T052421Z-eb17dd15](bitpack-footprint/packed_n33554432/linux-x86_64-c16-bare-intel/20261121T052421Z-eb17dd15/report.md)

## bitpack-footprint/packed_n4194304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T052421Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-packed | 541.76 us | 1.00x |
| bitpack-footprint-packed-naive | 2.70 ms | 4.99x |

- [20261114T124543Z-d0161800](bitpack-footprint/packed_n4194304/linux-x86_64-c16-bare-intel/20261114T124543Z-d0161800/report.md)
- [20261121T052421Z-eb17dd15](bitpack-footprint/packed_n4194304/linux-x86_64-c16-bare-intel/20261121T052421Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T021036Z-75710b6-dirty](bitpack-footprint/packed_n4194304/macos-aarch64-c8-bare-apple/20260805T021036Z-75710b6-dirty/report.md)

## bitpack-footprint/packed_n65536

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T052421Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-packed | 8.91 us | 1.00x |
| bitpack-footprint-packed-naive | 42.74 us | 4.80x |

- [20261114T123959Z-d0161800](bitpack-footprint/packed_n65536/linux-x86_64-c16-bare-intel/20261114T123959Z-d0161800/report.md)
- [20261121T052421Z-eb17dd15](bitpack-footprint/packed_n65536/linux-x86_64-c16-bare-intel/20261121T052421Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T020606Z-75710b6-dirty](bitpack-footprint/packed_n65536/macos-aarch64-c8-bare-apple/20260805T020606Z-75710b6-dirty/report.md)

## bitpack-footprint/packed_n7000000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T052421Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-footprint-packed | 934.99 us | 1.00x |
| bitpack-footprint-packed-naive | 4.54 ms | 4.85x |

- [20261114T125151Z-d0161800](bitpack-footprint/packed_n7000000/linux-x86_64-c16-bare-intel/20261114T125151Z-d0161800/report.md)
- [20261121T052421Z-eb17dd15](bitpack-footprint/packed_n7000000/linux-x86_64-c16-bare-intel/20261121T052421Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260805T021544Z-75710b6-dirty](bitpack-footprint/packed_n7000000/macos-aarch64-c8-bare-apple/20260805T021544Z-75710b6-dirty/report.md)

## bitpack-kernel-amortisation/bitpack-kernel-amortisation_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T060628Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-mac-naive | 11.72 us | 1.49x |
| bitpack-mac-narrow | 7.85 us | 1.00x |
| bitpack-mac-native | 8.59 us | 1.10x |
| bitpack-mac-simd | 7.85 us | 1.00x |
| bitpack-mac-windowed | 7.85 us | 1.00x |

- [20261121T060628Z-eb17dd15](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n16384/linux-x86_64-c16-bare-intel/20261121T060628Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170047Z-3d8f615-dirty](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n16384/macos-aarch64-c8-bare-apple/20260804T170047Z-3d8f615-dirty/report.md)

## bitpack-kernel-amortisation/bitpack-kernel-amortisation_n262144

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T060628Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-mac-naive | 191.36 us | 1.53x |
| bitpack-mac-narrow | 125.48 us | 1.00x |
| bitpack-mac-native | 139.29 us | 1.11x |
| bitpack-mac-simd | 128.68 us | 1.03x |
| bitpack-mac-windowed | 127.32 us | 1.01x |

- [20261121T060628Z-eb17dd15](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n262144/linux-x86_64-c16-bare-intel/20261121T060628Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170135Z-3d8f615-dirty](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n262144/macos-aarch64-c8-bare-apple/20260804T170135Z-3d8f615-dirty/report.md)

## bitpack-kernel-amortisation/bitpack-kernel-amortisation_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T060628Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-mac-naive | 48.30 us | 1.51x |
| bitpack-mac-narrow | 32.08 us | 1.00x |
| bitpack-mac-native | 34.82 us | 1.09x |
| bitpack-mac-simd | 32.15 us | 1.00x |
| bitpack-mac-windowed | 32.15 us | 1.00x |

- [20261121T060628Z-eb17dd15](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n65536/linux-x86_64-c16-bare-intel/20261121T060628Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170056Z-3d8f615-dirty](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n65536/macos-aarch64-c8-bare-apple/20260804T170056Z-3d8f615-dirty/report.md)

## bitpack-kernel-amortisation/bitpack-kernel-amortisation_n98304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T060628Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-mac-naive | 70.45 us | 1.46x |
| bitpack-mac-narrow | 48.23 us | 1.00x |
| bitpack-mac-native | 52.22 us | 1.08x |
| bitpack-mac-simd | 48.22 us | 1.00x |
| bitpack-mac-windowed | 48.20 us | 1.00x |

- [20261121T060628Z-eb17dd15](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n98304/linux-x86_64-c16-bare-intel/20261121T060628Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T170108Z-3d8f615-dirty](bitpack-kernel-amortisation/bitpack-kernel-amortisation_n98304/macos-aarch64-c8-bare-apple/20260804T170108Z-3d8f615-dirty/report.md)

## bitpack-random-sum/bitpack-random-sum_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T061624Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-rand | 7.49 us | 1.72x |
| bitpack-native-rand | 4.37 us | 1.00x |
| bitpack-zeropad-rand | 18.35 us | 4.20x |

- [20261114T132358Z-d0161800](bitpack-random-sum/bitpack-random-sum_n16384/linux-x86_64-c16-bare-intel/20261114T132358Z-d0161800/report.md)
- [20261121T061624Z-eb17dd15](bitpack-random-sum/bitpack-random-sum_n16384/linux-x86_64-c16-bare-intel/20261121T061624Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113652Z-25f736b-dirty](bitpack-random-sum/bitpack-random-sum_n16384/macos-aarch64-c8-bare-apple/20260804T113652Z-25f736b-dirty/report.md)

## bitpack-random-sum/bitpack-random-sum_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T061624Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-rand | 552 ns | 1.55x |
| bitpack-native-rand | 356 ns | 1.00x |
| bitpack-zeropad-rand | 1.44 us | 4.04x |

- [20261114T132115Z-d0161800](bitpack-random-sum/bitpack-random-sum_n256/linux-x86_64-c16-bare-intel/20261114T132115Z-d0161800/report.md)
- [20261121T061624Z-eb17dd15](bitpack-random-sum/bitpack-random-sum_n256/linux-x86_64-c16-bare-intel/20261121T061624Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113648Z-25f736b-dirty](bitpack-random-sum/bitpack-random-sum_n256/macos-aarch64-c8-bare-apple/20260804T113648Z-25f736b-dirty/report.md)

## bitpack-random-sum/bitpack-random-sum_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T061624Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-rand | 5.07 us | 1.40x |
| bitpack-native-rand | 3.61 us | 1.00x |
| bitpack-zeropad-rand | 4.60 us | 1.27x |

- [20261114T132237Z-d0161800](bitpack-random-sum/bitpack-random-sum_n4096/linux-x86_64-c16-bare-intel/20261114T132237Z-d0161800/report.md)
- [20261121T061624Z-eb17dd15](bitpack-random-sum/bitpack-random-sum_n4096/linux-x86_64-c16-bare-intel/20261121T061624Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113649Z-25f736b-dirty](bitpack-random-sum/bitpack-random-sum_n4096/macos-aarch64-c8-bare-apple/20260804T113649Z-25f736b-dirty/report.md)

## bitpack-sequential-sum/bitpack-sequential-sum_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T062054Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-seq | 7.48 us | 2.98x |
| bitpack-native-seq | 2.51 us | 1.00x |
| bitpack-zeropad-seq | 8.55 us | 3.41x |

- [20261114T132826Z-d0161800](bitpack-sequential-sum/bitpack-sequential-sum_n16384/linux-x86_64-c16-bare-intel/20261114T132826Z-d0161800/report.md)
- [20261121T062054Z-eb17dd15](bitpack-sequential-sum/bitpack-sequential-sum_n16384/linux-x86_64-c16-bare-intel/20261121T062054Z-eb17dd15/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261115T134932Z-ab7fcd71`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-seq | 10.57 us | 3.39x |
| bitpack-native-seq | 3.11 us | 1.00x |
| bitpack-zeropad-seq | 10.57 us | 3.39x |

- [20261115T134932Z-ab7fcd71](bitpack-sequential-sum/bitpack-sequential-sum_n16384/linux-x86_64-c4-container-intel/20261115T134932Z-ab7fcd71/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T114415Z-25f736b-dirty](bitpack-sequential-sum/bitpack-sequential-sum_n16384/macos-aarch64-c8-bare-apple/20260804T114415Z-25f736b-dirty/report.md)

## bitpack-sequential-sum/bitpack-sequential-sum_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T062054Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-seq | 546 ns | 2.53x |
| bitpack-native-seq | 215 ns | 1.00x |
| bitpack-zeropad-seq | 551 ns | 2.56x |

- [20261114T132543Z-d0161800](bitpack-sequential-sum/bitpack-sequential-sum_n256/linux-x86_64-c16-bare-intel/20261114T132543Z-d0161800/report.md)
- [20261121T062054Z-eb17dd15](bitpack-sequential-sum/bitpack-sequential-sum_n256/linux-x86_64-c16-bare-intel/20261121T062054Z-eb17dd15/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261115T134932Z-ab7fcd71`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-seq | 172 ns | 2.32x |
| bitpack-native-seq | 74 ns | 1.00x |
| bitpack-zeropad-seq | 203 ns | 2.74x |

- [20261115T134932Z-ab7fcd71](bitpack-sequential-sum/bitpack-sequential-sum_n256/linux-x86_64-c4-container-intel/20261115T134932Z-ab7fcd71/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T114412Z-25f736b-dirty](bitpack-sequential-sum/bitpack-sequential-sum_n256/macos-aarch64-c8-bare-apple/20260804T114412Z-25f736b-dirty/report.md)

## bitpack-sequential-sum/bitpack-sequential-sum_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T062054Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-seq | 2.75 us | 1.28x |
| bitpack-native-seq | 2.52 us | 1.17x |
| bitpack-zeropad-seq | 2.15 us | 1.00x |

- [20261114T132705Z-d0161800](bitpack-sequential-sum/bitpack-sequential-sum_n4096/linux-x86_64-c16-bare-intel/20261114T132705Z-d0161800/report.md)
- [20261121T062054Z-eb17dd15](bitpack-sequential-sum/bitpack-sequential-sum_n4096/linux-x86_64-c16-bare-intel/20261121T062054Z-eb17dd15/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261115T134932Z-ab7fcd71`

| variant | median | vs best |
|---|---|---|
| bitpack-aligned-seq | 2.50 us | 2.94x |
| bitpack-native-seq | 851 ns | 1.00x |
| bitpack-zeropad-seq | 2.87 us | 3.37x |

- [20261115T134932Z-ab7fcd71](bitpack-sequential-sum/bitpack-sequential-sum_n4096/linux-x86_64-c4-container-intel/20261115T134932Z-ab7fcd71/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T114413Z-25f736b-dirty](bitpack-sequential-sum/bitpack-sequential-sum_n4096/macos-aarch64-c8-bare-apple/20260804T114413Z-25f736b-dirty/report.md)

## bitpack-wide/bitpack-wide_n167772161

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T062530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-wide-d16 | 2.92 ms | 1.26x |
| bitpack-wide-d16-control | 2.92 ms | 1.26x |
| bitpack-wide-d16-padal | 2.93 ms | 1.26x |
| bitpack-wide-pipe4 | 2.33 ms | 1.00x |

- [20261121T062530Z-eb17dd15](bitpack-wide/bitpack-wide_n167772161/linux-x86_64-c16-bare-intel/20261121T062530Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T154425Z-41d2de2-dirty](bitpack-wide/bitpack-wide_n167772161/macos-aarch64-c8-bare-apple/20260808T154425Z-41d2de2-dirty/report.md)

## bitpack-wide/bitpack-wide_n167772164

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T062530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-wide-d16 | 1.13 ms | 1.22x |
| bitpack-wide-d16-control | 1.12 ms | 1.21x |
| bitpack-wide-d16-padal | 1.13 ms | 1.21x |
| bitpack-wide-pipe4 | 928.68 us | 1.00x |

- [20261121T062530Z-eb17dd15](bitpack-wide/bitpack-wide_n167772164/linux-x86_64-c16-bare-intel/20261121T062530Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T160518Z-0f456bb-dirty](bitpack-wide/bitpack-wide_n167772164/macos-aarch64-c8-bare-apple/20260808T160518Z-0f456bb-dirty/report.md)

## bitpack-wide/bitpack-wide_n335544321

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T062530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-wide-d16 | 5.92 ms | 1.24x |
| bitpack-wide-d16-control | 5.91 ms | 1.24x |
| bitpack-wide-d16-padal | 5.91 ms | 1.24x |
| bitpack-wide-pipe4 | 4.76 ms | 1.00x |

- [20261121T062530Z-eb17dd15](bitpack-wide/bitpack-wide_n335544321/linux-x86_64-c16-bare-intel/20261121T062530Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T163816Z-cf8dcf0-dirty](bitpack-wide/bitpack-wide_n335544321/macos-aarch64-c8-bare-apple/20260808T163816Z-cf8dcf0-dirty/report.md)

## bitpack-wide/bitpack-wide_n335544324

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T062530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-wide-d16 | 2.27 ms | 1.25x |
| bitpack-wide-d16-control | 2.28 ms | 1.26x |
| bitpack-wide-d16-padal | 2.27 ms | 1.25x |
| bitpack-wide-pipe4 | 1.82 ms | 1.00x |

- [20261121T062530Z-eb17dd15](bitpack-wide/bitpack-wide_n335544324/linux-x86_64-c16-bare-intel/20261121T062530Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T171740Z-f4cd8e2-dirty](bitpack-wide/bitpack-wide_n335544324/macos-aarch64-c8-bare-apple/20260808T171740Z-f4cd8e2-dirty/report.md)

## bitpack-wide/bitpack-wide_n83886081

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T062530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-wide-d16 | 1.42 ms | 1.24x |
| bitpack-wide-d16-control | 1.42 ms | 1.24x |
| bitpack-wide-d16-padal | 1.42 ms | 1.24x |
| bitpack-wide-pipe4 | 1.14 ms | 1.00x |

- [20261121T062530Z-eb17dd15](bitpack-wide/bitpack-wide_n83886081/linux-x86_64-c16-bare-intel/20261121T062530Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T151656Z-41d2de2-dirty](bitpack-wide/bitpack-wide_n83886081/macos-aarch64-c8-bare-apple/20260808T151656Z-41d2de2-dirty/report.md)

## bitpack-wide/bitpack-wide_n83886084

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T062530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-wide-d16 | 603.30 us | 1.34x |
| bitpack-wide-d16-control | 570.89 us | 1.26x |
| bitpack-wide-d16-padal | 584.99 us | 1.30x |
| bitpack-wide-pipe4 | 451.40 us | 1.00x |

- [20261121T062530Z-eb17dd15](bitpack-wide/bitpack-wide_n83886084/linux-x86_64-c16-bare-intel/20261121T062530Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T152720Z-41d2de2-dirty](bitpack-wide/bitpack-wide_n83886084/macos-aarch64-c8-bare-apple/20260808T152720Z-41d2de2-dirty/report.md)

## bitpack-write-contend/race_n20971501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T100423Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-dense | 485.21 us | 1.00x |
| bitpack-write-guarded | 7.31 ms | 15.08x |
| bitpack-write-unsound | 6.40 ms | 13.18x |

- [20261114T133718Z-d0161800](bitpack-write-contend/race_n20971501/linux-x86_64-c16-bare-intel/20261114T133718Z-d0161800/report.md)
- [20261121T100423Z-eb17dd15](bitpack-write-contend/race_n20971501/linux-x86_64-c16-bare-intel/20261121T100423Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T091501Z-49e39db2-dirty](bitpack-write-contend/race_n20971501/macos-aarch64-c8-bare-apple/20260809T091501Z-49e39db2-dirty/report.md)

## bitpack-write-contend/race_n20971502

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T100423Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-dense | 428.13 us | 1.00x |
| bitpack-write-guarded | 4.42 ms | 10.33x |

- [20261114T133931Z-d0161800](bitpack-write-contend/race_n20971502/linux-x86_64-c16-bare-intel/20261114T133931Z-d0161800/report.md)
- [20261121T100423Z-eb17dd15](bitpack-write-contend/race_n20971502/linux-x86_64-c16-bare-intel/20261121T100423Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T091645Z-b934e87d-dirty](bitpack-write-contend/race_n20971502/macos-aarch64-c8-bare-apple/20260809T091645Z-b934e87d-dirty/report.md)

## bitpack-write-contend/race_n20971504

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T100423Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-dense | 458.60 us | 1.00x |
| bitpack-write-guarded | 3.64 ms | 7.93x |

- [20261114T134202Z-d0161800](bitpack-write-contend/race_n20971504/linux-x86_64-c16-bare-intel/20261114T134202Z-d0161800/report.md)
- [20261121T100423Z-eb17dd15](bitpack-write-contend/race_n20971504/linux-x86_64-c16-bare-intel/20261121T100423Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T091805Z-d4f404a0-dirty](bitpack-write-contend/race_n20971504/macos-aarch64-c8-bare-apple/20260809T091805Z-d4f404a0-dirty/report.md)

## bitpack-write-contend/race_n655341

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T100423Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-dense | 10.06 us | 1.00x |
| bitpack-write-guarded | 244.46 us | 24.31x |
| bitpack-write-unsound | 194.09 us | 19.30x |

- [20261114T133051Z-d0161800](bitpack-write-contend/race_n655341/linux-x86_64-c16-bare-intel/20261114T133051Z-d0161800/report.md)
- [20261121T100423Z-eb17dd15](bitpack-write-contend/race_n655341/linux-x86_64-c16-bare-intel/20261121T100423Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T091153Z-d67fa506-dirty](bitpack-write-contend/race_n655341/macos-aarch64-c8-bare-apple/20260809T091153Z-d67fa506-dirty/report.md)

## bitpack-write-contend/race_n655342

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T100423Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-dense | 17.49 us | 1.00x |
| bitpack-write-guarded | 185.32 us | 10.60x |

- [20261114T133157Z-d0161800](bitpack-write-contend/race_n655342/linux-x86_64-c16-bare-intel/20261114T133157Z-d0161800/report.md)
- [20261121T100423Z-eb17dd15](bitpack-write-contend/race_n655342/linux-x86_64-c16-bare-intel/20261121T100423Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T091200Z-94f959ba-dirty](bitpack-write-contend/race_n655342/macos-aarch64-c8-bare-apple/20260809T091200Z-94f959ba-dirty/report.md)

## bitpack-write-contend/race_n655344

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T100423Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-dense | 29.78 us | 1.00x |
| bitpack-write-guarded | 172.56 us | 5.79x |

- [20261114T133300Z-d0161800](bitpack-write-contend/race_n655344/linux-x86_64-c16-bare-intel/20261114T133300Z-d0161800/report.md)
- [20261121T100423Z-eb17dd15](bitpack-write-contend/race_n655344/linux-x86_64-c16-bare-intel/20261121T100423Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T091208Z-94f959ba-dirty](bitpack-write-contend/race_n655344/macos-aarch64-c8-bare-apple/20260809T091208Z-94f959ba-dirty/report.md)

## bitpack-write-contend/safe_n20971521

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T101705Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-aligned | 6.28 ms | 14.93x |
| bitpack-write-dense | 420.76 us | 1.00x |
| bitpack-write-windowed | 3.86 ms | 9.16x |

- [20261114T135050Z-d0161800](bitpack-write-contend/safe_n20971521/linux-x86_64-c16-bare-intel/20261114T135050Z-d0161800/report.md)
- [20261121T101705Z-eb17dd15](bitpack-write-contend/safe_n20971521/linux-x86_64-c16-bare-intel/20261121T101705Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T092345Z-d4f404a0-dirty](bitpack-write-contend/safe_n20971521/macos-aarch64-c8-bare-apple/20260809T092345Z-d4f404a0-dirty/report.md)

## bitpack-write-contend/safe_n20971522

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T101705Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-aligned | 3.94 ms | 9.04x |
| bitpack-write-dense | 436.25 us | 1.00x |
| bitpack-write-windowed | 2.74 ms | 6.28x |

- [20261114T135403Z-d0161800](bitpack-write-contend/safe_n20971522/linux-x86_64-c16-bare-intel/20261114T135403Z-d0161800/report.md)
- [20261121T101705Z-eb17dd15](bitpack-write-contend/safe_n20971522/linux-x86_64-c16-bare-intel/20261121T101705Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T092532Z-d4f404a0-dirty](bitpack-write-contend/safe_n20971522/macos-aarch64-c8-bare-apple/20260809T092532Z-d4f404a0-dirty/report.md)

## bitpack-write-contend/safe_n20971524

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T101705Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-aligned | 3.37 ms | 8.60x |
| bitpack-write-dense | 392.09 us | 1.00x |
| bitpack-write-windowed | 2.61 ms | 6.65x |

- [20261114T135715Z-d0161800](bitpack-write-contend/safe_n20971524/linux-x86_64-c16-bare-intel/20261114T135715Z-d0161800/report.md)
- [20261121T101705Z-eb17dd15](bitpack-write-contend/safe_n20971524/linux-x86_64-c16-bare-intel/20261121T101705Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T092811Z-d4f404a0-dirty](bitpack-write-contend/safe_n20971524/macos-aarch64-c8-bare-apple/20260809T092811Z-d4f404a0-dirty/report.md)

## bitpack-write-contend/safe_n655361

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T101705Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-aligned | 194.06 us | 19.34x |
| bitpack-write-dense | 10.04 us | 1.00x |
| bitpack-write-windowed | 133.62 us | 13.32x |

- [20261114T134359Z-d0161800](bitpack-write-contend/safe_n655361/linux-x86_64-c16-bare-intel/20261114T134359Z-d0161800/report.md)
- [20261121T101705Z-eb17dd15](bitpack-write-contend/safe_n655361/linux-x86_64-c16-bare-intel/20261121T101705Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T092112Z-d4f404a0-dirty](bitpack-write-contend/safe_n655361/macos-aarch64-c8-bare-apple/20260809T092112Z-d4f404a0-dirty/report.md)

## bitpack-write-contend/safe_n655362

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T101705Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-aligned | 156.09 us | 10.61x |
| bitpack-write-dense | 14.71 us | 1.00x |
| bitpack-write-windowed | 107.54 us | 7.31x |

- [20261114T134533Z-d0161800](bitpack-write-contend/safe_n655362/linux-x86_64-c16-bare-intel/20261114T134533Z-d0161800/report.md)
- [20261121T101705Z-eb17dd15](bitpack-write-contend/safe_n655362/linux-x86_64-c16-bare-intel/20261121T101705Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T092122Z-d4f404a0-dirty](bitpack-write-contend/safe_n655362/macos-aarch64-c8-bare-apple/20260809T092122Z-d4f404a0-dirty/report.md)

## bitpack-write-contend/safe_n655364

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T101705Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| bitpack-write-aligned | 123.10 us | 5.46x |
| bitpack-write-dense | 22.54 us | 1.00x |
| bitpack-write-windowed | 127.51 us | 5.66x |

- [20261114T134705Z-d0161800](bitpack-write-contend/safe_n655364/linux-x86_64-c16-bare-intel/20261114T134705Z-d0161800/report.md)
- [20261121T101705Z-eb17dd15](bitpack-write-contend/safe_n655364/linux-x86_64-c16-bare-intel/20261121T101705Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260809T092133Z-d4f404a0-dirty](bitpack-write-contend/safe_n655364/macos-aarch64-c8-bare-apple/20260809T092133Z-d4f404a0-dirty/report.md)

## contract-sum/contract-sum_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T103243Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| contract_sum_concrete | 48 ns | 1.00x |
| contract_sum_generic | 48 ns | 1.00x |

- [20261121T103243Z-eb17dd15](contract-sum/contract-sum_n256/linux-x86_64-c16-bare-intel/20261121T103243Z-eb17dd15/report.md)

## contract-sum/contract-sum_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T103243Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| contract_sum_concrete | 54 ns | 1.00x |
| contract_sum_generic | 54 ns | 1.00x |

- [20261121T103243Z-eb17dd15](contract-sum/contract-sum_n64/linux-x86_64-c16-bare-intel/20261121T103243Z-eb17dd15/report.md)

## decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n0

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T103528Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-radix10 | 4.24 us | 1.00x |
| quantiser-radix2 | 9.28 us | 2.19x |

- [20261121T103528Z-eb17dd15](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n0/linux-x86_64-c16-bare-intel/20261121T103528Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113652Z-25f736b-dirty](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n0/macos-aarch64-c8-bare-apple/20260804T113652Z-25f736b-dirty/report.md)

## decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n2

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T103528Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-radix10 | 6.23 us | 1.00x |
| quantiser-radix2 | 9.48 us | 1.52x |

- [20261121T103528Z-eb17dd15](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n2/linux-x86_64-c16-bare-intel/20261121T103528Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113653Z-25f736b-dirty](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n2/macos-aarch64-c8-bare-apple/20260804T113653Z-25f736b-dirty/report.md)

## decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n20

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T103528Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-radix10 | 7.26 us | 1.00x |
| quantiser-radix2 | 12.42 us | 1.71x |

- [20261121T103528Z-eb17dd15](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n20/linux-x86_64-c16-bare-intel/20261121T103528Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113654Z-25f736b-dirty](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n20/macos-aarch64-c8-bare-apple/20260804T113654Z-25f736b-dirty/report.md)

## decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n8

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T103528Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-radix10 | 6.85 us | 1.00x |
| quantiser-radix2 | 11.22 us | 1.64x |

- [20261121T103528Z-eb17dd15](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n8/linux-x86_64-c16-bare-intel/20261121T103528Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113653Z-25f736b-dirty](decimal-quantiser-radix-sweep/decimal-quantiser-radix-sweep_n8/macos-aarch64-c8-bare-apple/20260804T113653Z-25f736b-dirty/report.md)

## fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n1024

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113655Z-25f736b-dirty](fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n1024/macos-aarch64-c8-bare-apple/20260804T113655Z-25f736b-dirty/report.md)

## fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n256

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113654Z-25f736b-dirty](fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n256/macos-aarch64-c8-bare-apple/20260804T113654Z-25f736b-dirty/report.md)

## fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n4096

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113655Z-25f736b-dirty](fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n4096/macos-aarch64-c8-bare-apple/20260804T113655Z-25f736b-dirty/report.md)

## fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n64

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113654Z-25f736b-dirty](fnv1a-vs-xxhash3/fnv1a-vs-xxhash3_n64/macos-aarch64-c8-bare-apple/20260804T113654Z-25f736b-dirty/report.md)

## numeral-ratio/conversion_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T104105Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| numeral_ratio_conversion_hand | 446 ns | 1.00x |
| numeral_ratio_conversion_room | 921 ns | 2.07x |
| numeral_ratio_conversion_wide | 8.36 us | 18.74x |

- [20261121T104105Z-eb17dd15](numeral-ratio/conversion_n256/linux-x86_64-c16-bare-intel/20261121T104105Z-eb17dd15/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261119T224053Z-1dd38711`

| variant | median | vs best |
|---|---|---|
| numeral_ratio_conversion_hand | 112 ns | 1.00x |
| numeral_ratio_conversion_room | 177 ns | 1.58x |
| numeral_ratio_conversion_wide | 8.36 us | 74.61x |

- [20261119T224053Z-1dd38711](numeral-ratio/conversion_n256/linux-x86_64-c4-container-intel/20261119T224053Z-1dd38711/report.md)

## numeral-ratio/product_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T104317Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| numeral_ratio_product_hand | 614 ns | 1.00x |
| numeral_ratio_product_room | 944 ns | 1.54x |
| numeral_ratio_product_wide | 10.20 us | 16.61x |

- [20261121T104317Z-eb17dd15](numeral-ratio/product_n256/linux-x86_64-c16-bare-intel/20261121T104317Z-eb17dd15/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261119T224053Z-1dd38711`

| variant | median | vs best |
|---|---|---|
| numeral_ratio_product_hand | 153 ns | 1.00x |
| numeral_ratio_product_room | 172 ns | 1.12x |
| numeral_ratio_product_wide | 10.92 us | 71.37x |

- [20261119T224053Z-1dd38711](numeral-ratio/product_n256/linux-x86_64-c4-container-intel/20261119T224053Z-1dd38711/report.md)

## numeral-ratio/quotient_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T104530Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| numeral_ratio_quotient_hand | 597 ns | 1.00x |
| numeral_ratio_quotient_room | 5.14 us | 8.61x |
| numeral_ratio_quotient_wide | 10.61 us | 17.77x |

- [20261121T104530Z-eb17dd15](numeral-ratio/quotient_n256/linux-x86_64-c16-bare-intel/20261121T104530Z-eb17dd15/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261119T224053Z-1dd38711`

| variant | median | vs best |
|---|---|---|
| numeral_ratio_quotient_hand | 153 ns | 1.00x |
| numeral_ratio_quotient_room | 1.37 us | 9.01x |
| numeral_ratio_quotient_wide | 12.10 us | 79.28x |

- [20261119T224053Z-1dd38711](numeral-ratio/quotient_n256/linux-x86_64-c4-container-intel/20261119T224053Z-1dd38711/report.md)

## packed-word/packed-word_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T104740Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| packed_word_host | 171 ns | 1.29x |
| packed_word_word | 132 ns | 1.00x |

- [20261121T104740Z-eb17dd15](packed-word/packed-word_n256/linux-x86_64-c16-bare-intel/20261121T104740Z-eb17dd15/report.md)

## packed-word/packed-word_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T104740Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| packed_word_host | 66 ns | 1.00x |
| packed_word_word | 67 ns | 1.01x |

- [20261121T104740Z-eb17dd15](packed-word/packed-word_n64/linux-x86_64-c16-bare-intel/20261121T104740Z-eb17dd15/report.md)

## quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n0

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105020Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-hardware | 1.06 us | 1.00x |
| quantiser-software | 3.91 us | 3.68x |

- [20261114T144626Z-d0161800](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n0/linux-x86_64-c16-bare-intel/20261114T144626Z-d0161800/report.md)
- [20261121T105020Z-eb17dd15](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n0/linux-x86_64-c16-bare-intel/20261121T105020Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113656Z-25f736b-dirty](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n0/macos-aarch64-c8-bare-apple/20260804T113656Z-25f736b-dirty/report.md)

## quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n10

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105020Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-hardware | 798 ns | 1.00x |
| quantiser-software | 3.75 us | 4.70x |

- [20261114T144724Z-d0161800](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n10/linux-x86_64-c16-bare-intel/20261114T144724Z-d0161800/report.md)
- [20261121T105020Z-eb17dd15](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n10/linux-x86_64-c16-bare-intel/20261121T105020Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113656Z-25f736b-dirty](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n10/macos-aarch64-c8-bare-apple/20260804T113656Z-25f736b-dirty/report.md)

## quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105020Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-hardware | 627 ns | 1.00x |
| quantiser-software | 3.42 us | 5.46x |

- [20261114T145102Z-d0161800](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n100/linux-x86_64-c16-bare-intel/20261114T145102Z-d0161800/report.md)
- [20261121T105020Z-eb17dd15](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n100/linux-x86_64-c16-bare-intel/20261121T105020Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113657Z-25f736b-dirty](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n100/macos-aarch64-c8-bare-apple/20260804T113657Z-25f736b-dirty/report.md)

## quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n25

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105020Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-hardware | 622 ns | 1.00x |
| quantiser-software | 4.32 us | 6.95x |

- [20261114T144818Z-d0161800](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n25/linux-x86_64-c16-bare-intel/20261114T144818Z-d0161800/report.md)
- [20261121T105020Z-eb17dd15](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n25/linux-x86_64-c16-bare-intel/20261121T105020Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113656Z-25f736b-dirty](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n25/macos-aarch64-c8-bare-apple/20260804T113656Z-25f736b-dirty/report.md)

## quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n50

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105020Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-hardware | 706 ns | 1.00x |
| quantiser-software | 3.94 us | 5.57x |

- [20261114T144913Z-d0161800](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n50/linux-x86_64-c16-bare-intel/20261114T144913Z-d0161800/report.md)
- [20261121T105020Z-eb17dd15](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n50/linux-x86_64-c16-bare-intel/20261121T105020Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113657Z-25f736b-dirty](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n50/macos-aarch64-c8-bare-apple/20260804T113657Z-25f736b-dirty/report.md)

## quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n75

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105020Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| quantiser-hardware | 1.09 us | 1.00x |
| quantiser-software | 3.75 us | 3.44x |

- [20261114T145007Z-d0161800](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n75/linux-x86_64-c16-bare-intel/20261114T145007Z-d0161800/report.md)
- [20261121T105020Z-eb17dd15](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n75/linux-x86_64-c16-bare-intel/20261121T105020Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113657Z-25f736b-dirty](quantiser-vs-fadd-subnormal-sweep/quantiser-vs-fadd-subnormal-sweep_n75/macos-aarch64-c8-bare-apple/20260804T113657Z-25f736b-dirty/report.md)

## satfold/align-l1_n10010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105635Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 25.32 us | 12.33x |
| satfold-lanes16 | 3.57 us | 1.74x |
| satfold-lanes16-constl | 3.51 us | 1.71x |
| satfold-lanes4-idx | 3.20 us | 1.56x |
| satfold-lanes64 | 2.05 us | 1.00x |
| satfold-neon | 2.32 us | 1.13x |
| satfold-nolaw | 8.61 us | 4.19x |
| satfold-seq | 25.32 us | 12.33x |

- [20261114T150252Z-d0161800](satfold/align-l1_n10010/linux-x86_64-c16-bare-intel/20261114T150252Z-d0161800/report.md)
- [20261121T105635Z-eb17dd15](satfold/align-l1_n10010/linux-x86_64-c16-bare-intel/20261121T105635Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193659Z-3fd5eab8-dirty](satfold/align-l1_n10010/macos-aarch64-c8-bare-apple/20260813T193659Z-3fd5eab8-dirty/report.md)

## satfold/align-l1_n12010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105635Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 24.88 us | 22.51x |
| satfold-lanes16 | 2.26 us | 2.04x |
| satfold-lanes16-constl | 2.25 us | 2.04x |
| satfold-lanes4-idx | 2.36 us | 2.13x |
| satfold-lanes64 | 1.16 us | 1.05x |
| satfold-neon | 1.11 us | 1.00x |
| satfold-nolaw | 12.40 us | 11.22x |
| satfold-seq | 22.32 us | 20.19x |

- [20261114T150634Z-d0161800](satfold/align-l1_n12010/linux-x86_64-c16-bare-intel/20261114T150634Z-d0161800/report.md)
- [20261121T105635Z-eb17dd15](satfold/align-l1_n12010/linux-x86_64-c16-bare-intel/20261121T105635Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193714Z-3fd5eab8-dirty](satfold/align-l1_n12010/macos-aarch64-c8-bare-apple/20260813T193714Z-3fd5eab8-dirty/report.md)

## satfold/align-l1_n3010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105635Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 4.43 us | 1.03x |
| satfold-lanes16 | 4.35 us | 1.01x |
| satfold-lanes16-constl | 5.30 us | 1.23x |
| satfold-lanes4-idx | 8.12 us | 1.88x |
| satfold-lanes64 | 4.32 us | 1.00x |
| satfold-neon | 5.27 us | 1.22x |
| satfold-nolaw | 4.32 us | 1.00x |
| satfold-seq | 5.14 us | 1.19x |

- [20261114T145522Z-d0161800](satfold/align-l1_n3010/linux-x86_64-c16-bare-intel/20261114T145522Z-d0161800/report.md)
- [20261121T105635Z-eb17dd15](satfold/align-l1_n3010/linux-x86_64-c16-bare-intel/20261121T105635Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193627Z-3fd5eab8-dirty](satfold/align-l1_n3010/macos-aarch64-c8-bare-apple/20260813T193627Z-3fd5eab8-dirty/report.md)

## satfold/align-l1_n7010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T105635Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 15.63 us | 7.78x |
| satfold-lanes16 | 2.63 us | 1.31x |
| satfold-lanes16-constl | 3.13 us | 1.56x |
| satfold-lanes4-idx | 2.52 us | 1.26x |
| satfold-lanes64 | 2.11 us | 1.05x |
| satfold-neon | 2.01 us | 1.00x |
| satfold-nolaw | 7.41 us | 3.69x |
| satfold-seq | 15.65 us | 7.80x |

- [20261114T145909Z-d0161800](satfold/align-l1_n7010/linux-x86_64-c16-bare-intel/20261114T145909Z-d0161800/report.md)
- [20261121T105635Z-eb17dd15](satfold/align-l1_n7010/linux-x86_64-c16-bare-intel/20261121T105635Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193643Z-3fd5eab8-dirty](satfold/align-l1_n7010/macos-aarch64-c8-bare-apple/20260813T193643Z-3fd5eab8-dirty/report.md)

## satfold/const-gate_n10000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T111159Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-gate-false | 20.42 us | 6.12x |
| satfold-gate-true | 3.33 us | 1.00x |
| satfold-lanes16 | 3.35 us | 1.00x |
| satfold-lanes16-3 | 3.59 us | 1.08x |
| satfold-seq | 20.94 us | 6.28x |

- [20261114T151148Z-d0161800](satfold/const-gate_n10000/linux-x86_64-c16-bare-intel/20261114T151148Z-d0161800/report.md)
- [20261121T111159Z-eb17dd15](satfold/const-gate_n10000/linux-x86_64-c16-bare-intel/20261121T111159Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194802Z-3fd5eab8-dirty](satfold/const-gate_n10000/macos-aarch64-c8-bare-apple/20260813T194802Z-3fd5eab8-dirty/report.md)

## satfold/const-gate_n12000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T111159Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-gate-false | 27.29 us | 15.37x |
| satfold-gate-true | 2.20 us | 1.24x |
| satfold-lanes16 | 1.92 us | 1.08x |
| satfold-lanes16-3 | 1.78 us | 1.00x |
| satfold-seq | 24.80 us | 13.97x |

- [20261114T151409Z-d0161800](satfold/const-gate_n12000/linux-x86_64-c16-bare-intel/20261114T151409Z-d0161800/report.md)
- [20261121T111159Z-eb17dd15](satfold/const-gate_n12000/linux-x86_64-c16-bare-intel/20261121T111159Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194813Z-3fd5eab8-dirty](satfold/const-gate_n12000/macos-aarch64-c8-bare-apple/20260813T194813Z-3fd5eab8-dirty/report.md)

## satfold/const-gate_n7000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T111159Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-gate-false | 14.06 us | 5.63x |
| satfold-gate-true | 2.51 us | 1.00x |
| satfold-lanes16 | 2.50 us | 1.00x |
| satfold-lanes16-3 | 3.04 us | 1.22x |
| satfold-seq | 16.12 us | 6.45x |

- [20261114T150923Z-d0161800](satfold/const-gate_n7000/linux-x86_64-c16-bare-intel/20261114T150923Z-d0161800/report.md)
- [20261121T111159Z-eb17dd15](satfold/const-gate_n7000/linux-x86_64-c16-bare-intel/20261121T111159Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194752Z-3fd5eab8-dirty](satfold/const-gate_n7000/macos-aarch64-c8-bare-apple/20260813T194752Z-3fd5eab8-dirty/report.md)

## satfold/length-dram-long_n12100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T133351Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 11.56 ms | 23.67x |
| satfold-lanes16 | 527.59 us | 1.08x |
| satfold-lanes16-constl | 534.96 us | 1.10x |
| satfold-lanes4-idx | 1.10 ms | 2.26x |
| satfold-lanes64 | 488.35 us | 1.00x |
| satfold-neon | 532.83 us | 1.09x |
| satfold-nolaw | 8.93 ms | 18.29x |
| satfold-seq | 11.57 ms | 23.69x |

- [20261114T170451Z-d0161800](satfold/length-dram-long_n12100/linux-x86_64-c16-bare-intel/20261114T170451Z-d0161800/report.md)
- [20261121T133351Z-eb17dd15](satfold/length-dram-long_n12100/linux-x86_64-c16-bare-intel/20261121T133351Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T191637Z-3fd5eab8-dirty](satfold/length-dram-long_n12100/macos-aarch64-c8-bare-apple/20260813T191637Z-3fd5eab8-dirty/report.md)

## satfold/length-dram-wrap_n12101

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T142424Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 424.99 us | 1.00x |
| satfold-lanes16 | 527.89 us | 1.24x |
| satfold-lanes16-constl | 528.33 us | 1.24x |
| satfold-lanes4-idx | 1.11 ms | 2.60x |
| satfold-lanes64 | 478.42 us | 1.13x |
| satfold-neon | 490.16 us | 1.15x |
| satfold-nolaw | 752.13 us | 1.77x |
| satfold-seq | 459.55 us | 1.08x |

- [20261114T175013Z-d0161800](satfold/length-dram-wrap_n12101/linux-x86_64-c16-bare-intel/20261114T175013Z-d0161800/report.md)
- [20261121T142424Z-eb17dd15](satfold/length-dram-wrap_n12101/linux-x86_64-c16-bare-intel/20261121T142424Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193424Z-3fd5eab8-dirty](satfold/length-dram-wrap_n12101/macos-aarch64-c8-bare-apple/20260813T193424Z-3fd5eab8-dirty/report.md)

## satfold/length-dram_n3100

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T111931Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 7.31 ms | 1.37x |
| satfold-lanes16 | 7.68 ms | 1.44x |
| satfold-lanes16-constl | 7.50 ms | 1.41x |
| satfold-lanes4-idx | 5.33 ms | 1.00x |
| satfold-lanes64 | 7.20 ms | 1.35x |
| satfold-neon | 7.22 ms | 1.36x |
| satfold-nolaw | 7.29 ms | 1.37x |
| satfold-seq | 7.29 ms | 1.37x |

- [20261121T111931Z-eb17dd15](satfold/length-dram_n3100/linux-x86_64-c16-bare-intel/20261121T111931Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T180535Z-3fd5eab8-dirty](satfold/length-dram_n3100/macos-aarch64-c8-bare-apple/20260813T180535Z-3fd5eab8-dirty/report.md)

## satfold/length-dram_n7100

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261121T111931Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 7.42 ms | 6.67x |
| satfold-lanes16 | 1.66 ms | 1.49x |
| satfold-lanes16-constl | 1.43 ms | 1.29x |
| satfold-lanes4-idx | 1.11 ms | 1.00x |
| satfold-lanes64 | 1.63 ms | 1.47x |
| satfold-neon | 1.62 ms | 1.46x |
| satfold-nolaw | 6.75 ms | 6.07x |
| satfold-seq | 7.44 ms | 6.68x |

- [20261121T111931Z-eb17dd15](satfold/length-dram_n7100/linux-x86_64-c16-bare-intel/20261121T111931Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T183617Z-3fd5eab8-dirty](satfold/length-dram_n7100/macos-aarch64-c8-bare-apple/20260813T183617Z-3fd5eab8-dirty/report.md)

## satfold/length-l1-wrap_n10001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T160041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 809 ns | 1.02x |
| satfold-lanes16 | 1.87 us | 2.35x |
| satfold-lanes16-constl | 2.07 us | 2.61x |
| satfold-lanes4-idx | 2.99 us | 3.76x |
| satfold-lanes64 | 865 ns | 1.09x |
| satfold-neon | 829 ns | 1.04x |
| satfold-neon8 | 795 ns | 1.00x |
| satfold-nolaw | 3.16 us | 3.97x |
| satfold-seq | 946 ns | 1.19x |

- [20261114T185800Z-d0161800](satfold/length-l1-wrap_n10001/linux-x86_64-c16-bare-intel/20261114T185800Z-d0161800/report.md)
- [20261121T160041Z-eb17dd15](satfold/length-l1-wrap_n10001/linux-x86_64-c16-bare-intel/20261121T160041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194228Z-3fd5eab8-dirty](satfold/length-l1-wrap_n10001/macos-aarch64-c8-bare-apple/20260813T194228Z-3fd5eab8-dirty/report.md)

## satfold/length-l1-wrap_n1001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T160041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 2.43 us | 1.00x |
| satfold-lanes16 | 2.44 us | 1.01x |
| satfold-lanes16-constl | 2.46 us | 1.02x |
| satfold-lanes4-idx | 2.42 us | 1.00x |
| satfold-lanes64 | 2.43 us | 1.01x |
| satfold-neon | 2.43 us | 1.01x |
| satfold-neon8 | 2.42 us | 1.00x |
| satfold-nolaw | 2.42 us | 1.00x |
| satfold-seq | 2.44 us | 1.01x |

- [20261114T184527Z-d0161800](satfold/length-l1-wrap_n1001/linux-x86_64-c16-bare-intel/20261114T184527Z-d0161800/report.md)
- [20261121T160041Z-eb17dd15](satfold/length-l1-wrap_n1001/linux-x86_64-c16-bare-intel/20261121T160041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194136Z-3fd5eab8-dirty](satfold/length-l1-wrap_n1001/macos-aarch64-c8-bare-apple/20260813T194136Z-3fd5eab8-dirty/report.md)

## satfold/length-l1-wrap_n12001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T160041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 856 ns | 1.00x |
| satfold-lanes16 | 1.83 us | 2.14x |
| satfold-lanes16-constl | 1.83 us | 2.13x |
| satfold-lanes4-idx | 2.31 us | 2.69x |
| satfold-lanes64 | 1.21 us | 1.42x |
| satfold-neon | 1.24 us | 1.45x |
| satfold-neon8 | 1.16 us | 1.36x |
| satfold-nolaw | 3.46 us | 4.04x |
| satfold-seq | 901 ns | 1.05x |

- [20261114T190208Z-d0161800](satfold/length-l1-wrap_n12001/linux-x86_64-c16-bare-intel/20261114T190208Z-d0161800/report.md)
- [20261121T160041Z-eb17dd15](satfold/length-l1-wrap_n12001/linux-x86_64-c16-bare-intel/20261121T160041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194246Z-3fd5eab8-dirty](satfold/length-l1-wrap_n12001/macos-aarch64-c8-bare-apple/20260813T194246Z-3fd5eab8-dirty/report.md)

## satfold/length-l1-wrap_n3001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T160041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 3.86 us | 1.11x |
| satfold-lanes16 | 3.49 us | 1.00x |
| satfold-lanes16-constl | 3.63 us | 1.04x |
| satfold-lanes4-idx | 3.56 us | 1.02x |
| satfold-lanes64 | 3.66 us | 1.05x |
| satfold-neon | 3.60 us | 1.03x |
| satfold-neon8 | 3.66 us | 1.05x |
| satfold-nolaw | 3.53 us | 1.01x |
| satfold-seq | 3.97 us | 1.14x |

- [20261114T184940Z-d0161800](satfold/length-l1-wrap_n3001/linux-x86_64-c16-bare-intel/20261114T184940Z-d0161800/report.md)
- [20261121T160041Z-eb17dd15](satfold/length-l1-wrap_n3001/linux-x86_64-c16-bare-intel/20261121T160041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194153Z-3fd5eab8-dirty](satfold/length-l1-wrap_n3001/macos-aarch64-c8-bare-apple/20260813T194153Z-3fd5eab8-dirty/report.md)

## satfold/length-l1-wrap_n7001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T160041Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 1.59 us | 1.13x |
| satfold-lanes16 | 1.41 us | 1.00x |
| satfold-lanes16-constl | 1.60 us | 1.14x |
| satfold-lanes4-idx | 1.57 us | 1.11x |
| satfold-lanes64 | 1.45 us | 1.03x |
| satfold-neon | 1.53 us | 1.09x |
| satfold-neon8 | 1.59 us | 1.13x |
| satfold-nolaw | 1.45 us | 1.03x |
| satfold-seq | 1.45 us | 1.03x |

- [20261114T185350Z-d0161800](satfold/length-l1-wrap_n7001/linux-x86_64-c16-bare-intel/20261114T185350Z-d0161800/report.md)
- [20261121T160041Z-eb17dd15](satfold/length-l1-wrap_n7001/linux-x86_64-c16-bare-intel/20261121T160041Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194210Z-3fd5eab8-dirty](satfold/length-l1-wrap_n7001/macos-aarch64-c8-bare-apple/20260813T194210Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n1000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 6.35 us | 1.15x |
| satfold-lanes16 | 5.53 us | 1.00x |
| satfold-lanes16-constl | 5.84 us | 1.06x |
| satfold-lanes4-idx | 5.98 us | 1.08x |
| satfold-lanes64 | 5.92 us | 1.07x |
| satfold-neon | 6.03 us | 1.09x |
| satfold-neon8 | 5.99 us | 1.08x |
| satfold-nolaw | 6.07 us | 1.10x |
| satfold-seq | 6.39 us | 1.16x |

- [20261114T175458Z-d0161800](satfold/length-l1_n1000/linux-x86_64-c16-bare-intel/20261114T175458Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n1000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193801Z-3fd5eab8-dirty](satfold/length-l1_n1000/macos-aarch64-c8-bare-apple/20260813T193801Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n10000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 20.93 us | 11.24x |
| satfold-lanes16 | 3.14 us | 1.69x |
| satfold-lanes16-constl | 2.93 us | 1.58x |
| satfold-lanes4-idx | 2.97 us | 1.60x |
| satfold-lanes64 | 1.96 us | 1.05x |
| satfold-neon | 1.86 us | 1.00x |
| satfold-neon8 | 1.98 us | 1.06x |
| satfold-nolaw | 8.69 us | 4.67x |
| satfold-seq | 24.48 us | 13.15x |

- [20261114T183238Z-d0161800](satfold/length-l1_n10000/linux-x86_64-c16-bare-intel/20261114T183238Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n10000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194043Z-3fd5eab8-dirty](satfold/length-l1_n10000/macos-aarch64-c8-bare-apple/20260813T194043Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n11000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 24.48 us | 20.74x |
| satfold-lanes16 | 2.70 us | 2.29x |
| satfold-lanes16-constl | 2.48 us | 2.10x |
| satfold-lanes4-idx | 2.94 us | 2.49x |
| satfold-lanes64 | 1.28 us | 1.08x |
| satfold-neon | 1.18 us | 1.00x |
| satfold-neon8 | 1.26 us | 1.07x |
| satfold-nolaw | 13.44 us | 11.38x |
| satfold-seq | 22.05 us | 18.68x |

- [20261114T183646Z-d0161800](satfold/length-l1_n11000/linux-x86_64-c16-bare-intel/20261114T183646Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n11000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194101Z-3fd5eab8-dirty](satfold/length-l1_n11000/macos-aarch64-c8-bare-apple/20260813T194101Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n12000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 27.29 us | 27.22x |
| satfold-lanes16 | 2.14 us | 2.14x |
| satfold-lanes16-constl | 2.02 us | 2.01x |
| satfold-lanes4-idx | 2.35 us | 2.34x |
| satfold-lanes64 | 1.00 us | 1.00x |
| satfold-neon | 1.14 us | 1.14x |
| satfold-neon8 | 1.14 us | 1.13x |
| satfold-nolaw | 12.43 us | 12.40x |
| satfold-seq | 27.29 us | 27.22x |

- [20261114T184054Z-d0161800](satfold/length-l1_n12000/linux-x86_64-c16-bare-intel/20261114T184054Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n12000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194119Z-3fd5eab8-dirty](satfold/length-l1_n12000/macos-aarch64-c8-bare-apple/20260813T194119Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n2000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 5.07 us | 1.17x |
| satfold-lanes16 | 5.47 us | 1.26x |
| satfold-lanes16-constl | 5.26 us | 1.21x |
| satfold-lanes4-idx | 5.23 us | 1.20x |
| satfold-lanes64 | 5.39 us | 1.24x |
| satfold-neon | 4.45 us | 1.02x |
| satfold-neon8 | 5.43 us | 1.25x |
| satfold-nolaw | 4.35 us | 1.00x |
| satfold-seq | 5.16 us | 1.19x |

- [20261114T175912Z-d0161800](satfold/length-l1_n2000/linux-x86_64-c16-bare-intel/20261114T175912Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n2000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193820Z-3fd5eab8-dirty](satfold/length-l1_n2000/macos-aarch64-c8-bare-apple/20260813T193820Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n3000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 5.24 us | 1.11x |
| satfold-lanes16 | 5.04 us | 1.07x |
| satfold-lanes16-constl | 5.26 us | 1.11x |
| satfold-lanes4-idx | 8.69 us | 1.84x |
| satfold-lanes64 | 5.33 us | 1.13x |
| satfold-neon | 5.43 us | 1.15x |
| satfold-neon8 | 5.15 us | 1.09x |
| satfold-nolaw | 5.14 us | 1.09x |
| satfold-seq | 4.72 us | 1.00x |

- [20261114T180323Z-d0161800](satfold/length-l1_n3000/linux-x86_64-c16-bare-intel/20261114T180323Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n3000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193838Z-3fd5eab8-dirty](satfold/length-l1_n3000/macos-aarch64-c8-bare-apple/20260813T193838Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n4000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 4.33 us | 1.01x |
| satfold-lanes16 | 5.09 us | 1.19x |
| satfold-lanes16-constl | 5.04 us | 1.18x |
| satfold-lanes4-idx | 7.43 us | 1.74x |
| satfold-lanes64 | 4.30 us | 1.01x |
| satfold-neon | 4.42 us | 1.03x |
| satfold-neon8 | 5.32 us | 1.24x |
| satfold-nolaw | 4.28 us | 1.00x |
| satfold-seq | 5.10 us | 1.19x |

- [20261114T180733Z-d0161800](satfold/length-l1_n4000/linux-x86_64-c16-bare-intel/20261114T180733Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n4000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193856Z-3fd5eab8-dirty](satfold/length-l1_n4000/macos-aarch64-c8-bare-apple/20260813T193856Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n5000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 4.94 us | 1.19x |
| satfold-lanes16 | 5.59 us | 1.35x |
| satfold-lanes16-constl | 4.59 us | 1.11x |
| satfold-lanes4-idx | 6.13 us | 1.48x |
| satfold-lanes64 | 5.00 us | 1.21x |
| satfold-neon | 4.96 us | 1.20x |
| satfold-neon8 | 4.96 us | 1.20x |
| satfold-nolaw | 4.14 us | 1.00x |
| satfold-seq | 4.94 us | 1.19x |

- [20261114T181144Z-d0161800](satfold/length-l1_n5000/linux-x86_64-c16-bare-intel/20261114T181144Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n5000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193913Z-3fd5eab8-dirty](satfold/length-l1_n5000/macos-aarch64-c8-bare-apple/20260813T193913Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n6000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 13.31 us | 5.51x |
| satfold-lanes16 | 5.39 us | 2.23x |
| satfold-lanes16-constl | 5.44 us | 2.25x |
| satfold-lanes4-idx | 2.41 us | 1.00x |
| satfold-lanes64 | 9.86 us | 4.09x |
| satfold-neon | 10.62 us | 4.40x |
| satfold-neon8 | 9.87 us | 4.09x |
| satfold-nolaw | 5.52 us | 2.29x |
| satfold-seq | 14.90 us | 6.17x |

- [20261114T181555Z-d0161800](satfold/length-l1_n6000/linux-x86_64-c16-bare-intel/20261114T181555Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n6000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193931Z-3fd5eab8-dirty](satfold/length-l1_n6000/macos-aarch64-c8-bare-apple/20260813T193931Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n7000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 15.65 us | 8.03x |
| satfold-lanes16 | 2.89 us | 1.48x |
| satfold-lanes16-constl | 3.02 us | 1.55x |
| satfold-lanes4-idx | 2.16 us | 1.11x |
| satfold-lanes64 | 3.63 us | 1.86x |
| satfold-neon | 1.95 us | 1.00x |
| satfold-neon8 | 1.95 us | 1.00x |
| satfold-nolaw | 7.41 us | 3.80x |
| satfold-seq | 17.21 us | 8.83x |

- [20261114T182006Z-d0161800](satfold/length-l1_n7000/linux-x86_64-c16-bare-intel/20261114T182006Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n7000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T193948Z-3fd5eab8-dirty](satfold/length-l1_n7000/macos-aarch64-c8-bare-apple/20260813T193948Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n8000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 14.24 us | 7.07x |
| satfold-lanes16 | 3.31 us | 1.64x |
| satfold-lanes16-constl | 3.35 us | 1.66x |
| satfold-lanes4-idx | 2.27 us | 1.13x |
| satfold-lanes64 | 2.02 us | 1.00x |
| satfold-neon | 2.02 us | 1.00x |
| satfold-neon8 | 2.02 us | 1.00x |
| satfold-nolaw | 7.64 us | 3.79x |
| satfold-seq | 16.02 us | 7.95x |

- [20261114T182417Z-d0161800](satfold/length-l1_n8000/linux-x86_64-c16-bare-intel/20261114T182417Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n8000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194006Z-3fd5eab8-dirty](satfold/length-l1_n8000/macos-aarch64-c8-bare-apple/20260813T194006Z-3fd5eab8-dirty/report.md)

## satfold/length-l1_n9000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T151001Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| satfold-iterfold | 18.23 us | 6.64x |
| satfold-lanes16 | 4.10 us | 1.49x |
| satfold-lanes16-constl | 3.95 us | 1.44x |
| satfold-lanes4-idx | 4.44 us | 1.62x |
| satfold-lanes64 | 3.20 us | 1.16x |
| satfold-neon | 2.75 us | 1.00x |
| satfold-neon8 | 3.22 us | 1.17x |
| satfold-nolaw | 8.00 us | 2.91x |
| satfold-seq | 22.27 us | 8.11x |

- [20261114T182827Z-d0161800](satfold/length-l1_n9000/linux-x86_64-c16-bare-intel/20261114T182827Z-d0161800/report.md)
- [20261121T151001Z-eb17dd15](satfold/length-l1_n9000/linux-x86_64-c16-bare-intel/20261121T151001Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260813T194025Z-3fd5eab8-dirty](satfold/length-l1_n9000/macos-aarch64-c8-bare-apple/20260813T194025Z-3fd5eab8-dirty/report.md)

## spectral-bisection/spectral-bisection_n16

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113645Z-25f736b-dirty](spectral-bisection/spectral-bisection_n16/macos-aarch64-c8-bare-apple/20260804T113645Z-25f736b-dirty/report.md)

## spectral-bisection/spectral-bisection_n32

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113646Z-25f736b-dirty](spectral-bisection/spectral-bisection_n32/macos-aarch64-c8-bare-apple/20260804T113646Z-25f736b-dirty/report.md)

## spectral-bisection/spectral-bisection_n64

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113647Z-25f736b-dirty](spectral-bisection/spectral-bisection_n64/macos-aarch64-c8-bare-apple/20260804T113647Z-25f736b-dirty/report.md)

## structural-decomposition/structural-decomposition_n16

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113639Z-25f736b-dirty](structural-decomposition/structural-decomposition_n16/macos-aarch64-c8-bare-apple/20260804T113639Z-25f736b-dirty/report.md)

## structural-decomposition/structural-decomposition_n32

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113640Z-25f736b-dirty](structural-decomposition/structural-decomposition_n32/macos-aarch64-c8-bare-apple/20260804T113640Z-25f736b-dirty/report.md)

## structural-decomposition/structural-decomposition_n64

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260804T113640Z-25f736b-dirty](structural-decomposition/structural-decomposition_n64/macos-aarch64-c8-bare-apple/20260804T113640Z-25f736b-dirty/report.md)

## warm-clamp/arity-l2_n131040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T162215Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 121.32 us | 1.78x |
| warm-clamp-accfit | 68.08 us | 1.00x |
| warm-clamp-accfit-dyn | 70.63 us | 1.04x |
| warm-clamp-head | 72.46 us | 1.06x |
| warm-clamp-min-lanes | 169.66 us | 2.49x |
| warm-clamp-minimum | 166.90 us | 2.45x |

- [20261114T194724Z-d0161800](warm-clamp/arity-l2_n131040/linux-x86_64-c16-bare-intel/20261114T194724Z-d0161800/report.md)
- [20261121T162215Z-eb17dd15](warm-clamp/arity-l2_n131040/linux-x86_64-c16-bare-intel/20261121T162215Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T133543Z-ccf0509-dirty](warm-clamp/arity-l2_n131040/macos-aarch64-c8-bare-apple/20260807T133543Z-ccf0509-dirty/report.md)

## warm-clamp/arity-l2_n161040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T162215Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 118.77 us | 1.83x |
| warm-clamp-accfit | 64.73 us | 1.00x |
| warm-clamp-accfit-dyn | 67.82 us | 1.05x |
| warm-clamp-head | 67.03 us | 1.04x |
| warm-clamp-min-lanes | 243.73 us | 3.77x |
| warm-clamp-minimum | 242.15 us | 3.74x |

- [20261114T195309Z-d0161800](warm-clamp/arity-l2_n161040/linux-x86_64-c16-bare-intel/20261114T195309Z-d0161800/report.md)
- [20261121T162215Z-eb17dd15](warm-clamp/arity-l2_n161040/linux-x86_64-c16-bare-intel/20261121T162215Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T133939Z-ccf0509-dirty](warm-clamp/arity-l2_n161040/macos-aarch64-c8-bare-apple/20260807T133939Z-ccf0509-dirty/report.md)

## warm-clamp/arity-l2_n321040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T162215Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 106.10 us | 1.04x |
| warm-clamp-accfit | 102.05 us | 1.00x |
| warm-clamp-accfit-dyn | 107.34 us | 1.05x |
| warm-clamp-head | 128.59 us | 1.26x |
| warm-clamp-min-lanes | 365.02 us | 3.58x |
| warm-clamp-minimum | 393.25 us | 3.85x |

- [20261114T195913Z-d0161800](warm-clamp/arity-l2_n321040/linux-x86_64-c16-bare-intel/20261114T195913Z-d0161800/report.md)
- [20261121T162215Z-eb17dd15](warm-clamp/arity-l2_n321040/linux-x86_64-c16-bare-intel/20261121T162215Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T134405Z-defc747](warm-clamp/arity-l2_n321040/macos-aarch64-c8-bare-apple/20260807T134405Z-defc747/report.md)

## warm-clamp/arity-l2_n601040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T162215Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 129.68 us | 1.00x |
| warm-clamp-accfit | 257.32 us | 1.98x |
| warm-clamp-accfit-dyn | 144.05 us | 1.11x |
| warm-clamp-head | 560.59 us | 4.32x |
| warm-clamp-min-lanes | 612.71 us | 4.72x |
| warm-clamp-minimum | 142.69 us | 1.10x |

- [20261114T200600Z-d0161800](warm-clamp/arity-l2_n601040/linux-x86_64-c16-bare-intel/20261114T200600Z-d0161800/report.md)
- [20261121T162215Z-eb17dd15](warm-clamp/arity-l2_n601040/linux-x86_64-c16-bare-intel/20261121T162215Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T134758Z-defc747-dirty](warm-clamp/arity-l2_n601040/macos-aarch64-c8-bare-apple/20260807T134758Z-defc747-dirty/report.md)

## warm-clamp/arity-l2_n641040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T162215Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 432.50 us | 1.85x |
| warm-clamp-accfit | 233.23 us | 1.00x |
| warm-clamp-accfit-dyn | 258.33 us | 1.11x |
| warm-clamp-head | 433.02 us | 1.86x |
| warm-clamp-min-lanes | 424.86 us | 1.82x |
| warm-clamp-minimum | 473.44 us | 2.03x |

- [20261114T201252Z-d0161800](warm-clamp/arity-l2_n641040/linux-x86_64-c16-bare-intel/20261114T201252Z-d0161800/report.md)
- [20261121T162215Z-eb17dd15](warm-clamp/arity-l2_n641040/linux-x86_64-c16-bare-intel/20261121T162215Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T135301Z-defc747-dirty](warm-clamp/arity-l2_n641040/macos-aarch64-c8-bare-apple/20260807T135301Z-defc747-dirty/report.md)

## warm-clamp/arity-l2_n81040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T162215Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 118.62 us | 1.00x |
| warm-clamp-accfit | 277.48 us | 2.34x |
| warm-clamp-accfit-dyn | 263.16 us | 2.22x |
| warm-clamp-head | 285.59 us | 2.41x |
| warm-clamp-min-lanes | 250.12 us | 2.11x |
| warm-clamp-minimum | 249.88 us | 2.11x |

- [20261114T194138Z-d0161800](warm-clamp/arity-l2_n81040/linux-x86_64-c16-bare-intel/20261114T194138Z-d0161800/report.md)
- [20261121T162215Z-eb17dd15](warm-clamp/arity-l2_n81040/linux-x86_64-c16-bare-intel/20261121T162215Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T133149Z-ccf0509-dirty](warm-clamp/arity-l2_n81040/macos-aarch64-c8-bare-apple/20260807T133149Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w13_n130010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T170029Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.98 us | 1.44x |
| warm-clamp-accfit | 3.16 us | 1.14x |
| warm-clamp-accfit-dyn | 3.71 us | 1.34x |
| warm-clamp-head | 2.77 us | 1.00x |
| warm-clamp-min-lanes | 3.93 us | 1.42x |
| warm-clamp-minimum | 3.14 us | 1.13x |

- [20261114T201608Z-d0161800](warm-clamp/arity-w13_n130010/linux-x86_64-c16-bare-intel/20261114T201608Z-d0161800/report.md)
- [20261121T170029Z-eb17dd15](warm-clamp/arity-w13_n130010/linux-x86_64-c16-bare-intel/20261121T170029Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124757Z-ccf0509-dirty](warm-clamp/arity-w13_n130010/macos-aarch64-c8-bare-apple/20260807T124757Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w13_n130020

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T170029Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.59 us | 1.87x |
| warm-clamp-accfit | 1.93 us | 1.00x |
| warm-clamp-accfit-dyn | 2.35 us | 1.22x |
| warm-clamp-head | 3.32 us | 1.72x |
| warm-clamp-min-lanes | 2.38 us | 1.23x |
| warm-clamp-minimum | 2.34 us | 1.22x |

- [20261114T201901Z-d0161800](warm-clamp/arity-w13_n130020/linux-x86_64-c16-bare-intel/20261114T201901Z-d0161800/report.md)
- [20261121T170029Z-eb17dd15](warm-clamp/arity-w13_n130020/linux-x86_64-c16-bare-intel/20261121T170029Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124813Z-ccf0509-dirty](warm-clamp/arity-w13_n130020/macos-aarch64-c8-bare-apple/20260807T124813Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w13_n130030

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T170029Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.65 us | 1.56x |
| warm-clamp-accfit | 2.33 us | 1.00x |
| warm-clamp-accfit-dyn | 2.34 us | 1.00x |
| warm-clamp-head | 2.55 us | 1.09x |
| warm-clamp-min-lanes | 3.54 us | 1.52x |
| warm-clamp-minimum | 2.33 us | 1.00x |

- [20261114T202151Z-d0161800](warm-clamp/arity-w13_n130030/linux-x86_64-c16-bare-intel/20261114T202151Z-d0161800/report.md)
- [20261121T170029Z-eb17dd15](warm-clamp/arity-w13_n130030/linux-x86_64-c16-bare-intel/20261121T170029Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124830Z-ccf0509-dirty](warm-clamp/arity-w13_n130030/macos-aarch64-c8-bare-apple/20260807T124830Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w13_n130040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T170029Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.72 us | 2.21x |
| warm-clamp-accfit | 2.25 us | 1.34x |
| warm-clamp-accfit-dyn | 2.25 us | 1.34x |
| warm-clamp-head | 1.68 us | 1.00x |
| warm-clamp-min-lanes | 4.44 us | 2.64x |
| warm-clamp-minimum | 3.94 us | 2.34x |

- [20261114T202441Z-d0161800](warm-clamp/arity-w13_n130040/linux-x86_64-c16-bare-intel/20261114T202441Z-d0161800/report.md)
- [20261121T170029Z-eb17dd15](warm-clamp/arity-w13_n130040/linux-x86_64-c16-bare-intel/20261121T170029Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124846Z-ccf0509-dirty](warm-clamp/arity-w13_n130040/macos-aarch64-c8-bare-apple/20260807T124846Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w13_n130060

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T170029Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.25 us | 4.02x |
| warm-clamp-accfit | 1.70 us | 2.10x |
| warm-clamp-accfit-dyn | 1.69 us | 2.09x |
| warm-clamp-head | 810 ns | 1.00x |
| warm-clamp-min-lanes | 3.23 us | 3.99x |
| warm-clamp-minimum | 5.65 us | 6.99x |

- [20261114T202731Z-d0161800](warm-clamp/arity-w13_n130060/linux-x86_64-c16-bare-intel/20261114T202731Z-d0161800/report.md)
- [20261121T170029Z-eb17dd15](warm-clamp/arity-w13_n130060/linux-x86_64-c16-bare-intel/20261121T170029Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124902Z-ccf0509-dirty](warm-clamp/arity-w13_n130060/macos-aarch64-c8-bare-apple/20260807T124902Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w13_n130080

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T170029Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.32 us | 4.45x |
| warm-clamp-accfit | 1.28 us | 1.72x |
| warm-clamp-accfit-dyn | 1.53 us | 2.06x |
| warm-clamp-head | 745 ns | 1.00x |
| warm-clamp-min-lanes | 2.07 us | 2.78x |
| warm-clamp-minimum | 3.26 us | 4.38x |

- [20261114T203021Z-d0161800](warm-clamp/arity-w13_n130080/linux-x86_64-c16-bare-intel/20261114T203021Z-d0161800/report.md)
- [20261121T170029Z-eb17dd15](warm-clamp/arity-w13_n130080/linux-x86_64-c16-bare-intel/20261121T170029Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124918Z-ccf0509-dirty](warm-clamp/arity-w13_n130080/macos-aarch64-c8-bare-apple/20260807T124918Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w16_n160010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T171800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.23 us | 1.23x |
| warm-clamp-accfit | 2.86 us | 1.09x |
| warm-clamp-accfit-dyn | 2.62 us | 1.00x |
| warm-clamp-head | 2.94 us | 1.12x |
| warm-clamp-min-lanes | 2.63 us | 1.00x |
| warm-clamp-minimum | 3.30 us | 1.26x |

- [20261114T203332Z-d0161800](warm-clamp/arity-w16_n160010/linux-x86_64-c16-bare-intel/20261114T203332Z-d0161800/report.md)
- [20261121T171800Z-eb17dd15](warm-clamp/arity-w16_n160010/linux-x86_64-c16-bare-intel/20261121T171800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124934Z-ccf0509-dirty](warm-clamp/arity-w16_n160010/macos-aarch64-c8-bare-apple/20260807T124934Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w16_n160020

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T171800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.75 us | 1.60x |
| warm-clamp-accfit | 4.27 us | 1.82x |
| warm-clamp-accfit-dyn | 3.70 us | 1.58x |
| warm-clamp-head | 3.91 us | 1.67x |
| warm-clamp-min-lanes | 3.03 us | 1.30x |
| warm-clamp-minimum | 2.34 us | 1.00x |

- [20261114T203625Z-d0161800](warm-clamp/arity-w16_n160020/linux-x86_64-c16-bare-intel/20261114T203625Z-d0161800/report.md)
- [20261121T171800Z-eb17dd15](warm-clamp/arity-w16_n160020/linux-x86_64-c16-bare-intel/20261121T171800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124950Z-ccf0509-dirty](warm-clamp/arity-w16_n160020/macos-aarch64-c8-bare-apple/20260807T124950Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w16_n160030

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T171800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.07 us | 1.45x |
| warm-clamp-accfit | 2.65 us | 1.25x |
| warm-clamp-accfit-dyn | 2.66 us | 1.25x |
| warm-clamp-head | 2.54 us | 1.20x |
| warm-clamp-min-lanes | 2.33 us | 1.10x |
| warm-clamp-minimum | 2.12 us | 1.00x |

- [20261114T203916Z-d0161800](warm-clamp/arity-w16_n160030/linux-x86_64-c16-bare-intel/20261114T203916Z-d0161800/report.md)
- [20261121T171800Z-eb17dd15](warm-clamp/arity-w16_n160030/linux-x86_64-c16-bare-intel/20261121T171800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125006Z-ccf0509-dirty](warm-clamp/arity-w16_n160030/macos-aarch64-c8-bare-apple/20260807T125006Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w16_n160040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T171800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.22 us | 2.24x |
| warm-clamp-accfit | 1.96 us | 1.37x |
| warm-clamp-accfit-dyn | 1.93 us | 1.34x |
| warm-clamp-head | 1.44 us | 1.00x |
| warm-clamp-min-lanes | 2.34 us | 1.63x |
| warm-clamp-minimum | 2.32 us | 1.62x |

- [20261114T204206Z-d0161800](warm-clamp/arity-w16_n160040/linux-x86_64-c16-bare-intel/20261114T204206Z-d0161800/report.md)
- [20261121T171800Z-eb17dd15](warm-clamp/arity-w16_n160040/linux-x86_64-c16-bare-intel/20261121T171800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125022Z-ccf0509-dirty](warm-clamp/arity-w16_n160040/macos-aarch64-c8-bare-apple/20260807T125022Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w16_n160060

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T171800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.25 us | 4.27x |
| warm-clamp-accfit | 1.69 us | 2.22x |
| warm-clamp-accfit-dyn | 1.69 us | 2.22x |
| warm-clamp-head | 762 ns | 1.00x |
| warm-clamp-min-lanes | 1.60 us | 2.10x |
| warm-clamp-minimum | 2.78 us | 3.65x |

- [20261114T204456Z-d0161800](warm-clamp/arity-w16_n160060/linux-x86_64-c16-bare-intel/20261114T204456Z-d0161800/report.md)
- [20261121T171800Z-eb17dd15](warm-clamp/arity-w16_n160060/linux-x86_64-c16-bare-intel/20261121T171800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125038Z-ccf0509-dirty](warm-clamp/arity-w16_n160060/macos-aarch64-c8-bare-apple/20260807T125038Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w16_n160080

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T171800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.79 us | 5.56x |
| warm-clamp-accfit | 1.70 us | 2.50x |
| warm-clamp-accfit-dyn | 1.62 us | 2.37x |
| warm-clamp-head | 682 ns | 1.00x |
| warm-clamp-min-lanes | 1.30 us | 1.91x |
| warm-clamp-minimum | 2.39 us | 3.50x |

- [20261114T204746Z-d0161800](warm-clamp/arity-w16_n160080/linux-x86_64-c16-bare-intel/20261114T204746Z-d0161800/report.md)
- [20261121T171800Z-eb17dd15](warm-clamp/arity-w16_n160080/linux-x86_64-c16-bare-intel/20261121T171800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125053Z-ccf0509-dirty](warm-clamp/arity-w16_n160080/macos-aarch64-c8-bare-apple/20260807T125053Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w32_n320010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T173537Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.18 us | 1.00x |
| warm-clamp-accfit | 3.92 us | 1.80x |
| warm-clamp-accfit-dyn | 3.70 us | 1.70x |
| warm-clamp-head | 2.63 us | 1.21x |
| warm-clamp-min-lanes | 2.40 us | 1.10x |
| warm-clamp-minimum | 2.88 us | 1.32x |

- [20261114T205057Z-d0161800](warm-clamp/arity-w32_n320010/linux-x86_64-c16-bare-intel/20261114T205057Z-d0161800/report.md)
- [20261121T173537Z-eb17dd15](warm-clamp/arity-w32_n320010/linux-x86_64-c16-bare-intel/20261121T173537Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125123Z-ccf0509-dirty](warm-clamp/arity-w32_n320010/macos-aarch64-c8-bare-apple/20260807T125123Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w32_n320020

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T173537Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.84 us | 1.57x |
| warm-clamp-accfit | 3.81 us | 1.56x |
| warm-clamp-accfit-dyn | 3.55 us | 1.45x |
| warm-clamp-head | 3.36 us | 1.37x |
| warm-clamp-min-lanes | 2.45 us | 1.00x |
| warm-clamp-minimum | 2.90 us | 1.18x |

- [20261114T205350Z-d0161800](warm-clamp/arity-w32_n320020/linux-x86_64-c16-bare-intel/20261114T205350Z-d0161800/report.md)
- [20261121T173537Z-eb17dd15](warm-clamp/arity-w32_n320020/linux-x86_64-c16-bare-intel/20261121T173537Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125139Z-ccf0509-dirty](warm-clamp/arity-w32_n320020/macos-aarch64-c8-bare-apple/20260807T125139Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w32_n320030

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T173537Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.41 us | 1.36x |
| warm-clamp-accfit | 3.15 us | 1.26x |
| warm-clamp-accfit-dyn | 3.43 us | 1.37x |
| warm-clamp-head | 2.50 us | 1.00x |
| warm-clamp-min-lanes | 4.36 us | 1.74x |
| warm-clamp-minimum | 3.93 us | 1.57x |

- [20261114T205641Z-d0161800](warm-clamp/arity-w32_n320030/linux-x86_64-c16-bare-intel/20261114T205641Z-d0161800/report.md)
- [20261121T173537Z-eb17dd15](warm-clamp/arity-w32_n320030/linux-x86_64-c16-bare-intel/20261121T173537Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125155Z-ccf0509-dirty](warm-clamp/arity-w32_n320030/macos-aarch64-c8-bare-apple/20260807T125155Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w32_n320040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T173537Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.14 us | 1.19x |
| warm-clamp-accfit | 2.98 us | 1.13x |
| warm-clamp-accfit-dyn | 3.35 us | 1.27x |
| warm-clamp-head | 2.63 us | 1.00x |
| warm-clamp-min-lanes | 3.18 us | 1.21x |
| warm-clamp-minimum | 3.63 us | 1.38x |

- [20261114T205931Z-d0161800](warm-clamp/arity-w32_n320040/linux-x86_64-c16-bare-intel/20261114T205931Z-d0161800/report.md)
- [20261121T173537Z-eb17dd15](warm-clamp/arity-w32_n320040/linux-x86_64-c16-bare-intel/20261121T173537Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125211Z-ccf0509-dirty](warm-clamp/arity-w32_n320040/macos-aarch64-c8-bare-apple/20260807T125211Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w32_n320060

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T173537Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.77 us | 1.12x |
| warm-clamp-accfit | 2.74 us | 1.11x |
| warm-clamp-accfit-dyn | 2.81 us | 1.14x |
| warm-clamp-head | 2.82 us | 1.15x |
| warm-clamp-min-lanes | 4.15 us | 1.69x |
| warm-clamp-minimum | 2.46 us | 1.00x |

- [20261114T210221Z-d0161800](warm-clamp/arity-w32_n320060/linux-x86_64-c16-bare-intel/20261114T210221Z-d0161800/report.md)
- [20261121T173537Z-eb17dd15](warm-clamp/arity-w32_n320060/linux-x86_64-c16-bare-intel/20261121T173537Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125227Z-ccf0509-dirty](warm-clamp/arity-w32_n320060/macos-aarch64-c8-bare-apple/20260807T125227Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w32_n320080

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T173537Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.62 us | 1.20x |
| warm-clamp-accfit | 2.43 us | 1.11x |
| warm-clamp-accfit-dyn | 2.60 us | 1.19x |
| warm-clamp-head | 2.59 us | 1.19x |
| warm-clamp-min-lanes | 3.80 us | 1.74x |
| warm-clamp-minimum | 2.18 us | 1.00x |

- [20261114T210511Z-d0161800](warm-clamp/arity-w32_n320080/linux-x86_64-c16-bare-intel/20261114T210511Z-d0161800/report.md)
- [20261121T173537Z-eb17dd15](warm-clamp/arity-w32_n320080/linux-x86_64-c16-bare-intel/20261121T173537Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125244Z-ccf0509-dirty](warm-clamp/arity-w32_n320080/macos-aarch64-c8-bare-apple/20260807T125244Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w60_n600010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T175309Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.10 us | 1.26x |
| warm-clamp-accfit | 3.10 us | 1.27x |
| warm-clamp-accfit-dyn | 3.01 us | 1.23x |
| warm-clamp-head | 3.06 us | 1.25x |
| warm-clamp-min-lanes | 2.45 us | 1.00x |
| warm-clamp-minimum | 2.75 us | 1.12x |

- [20261114T210823Z-d0161800](warm-clamp/arity-w60_n600010/linux-x86_64-c16-bare-intel/20261114T210823Z-d0161800/report.md)
- [20261121T175309Z-eb17dd15](warm-clamp/arity-w60_n600010/linux-x86_64-c16-bare-intel/20261121T175309Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125300Z-ccf0509-dirty](warm-clamp/arity-w60_n600010/macos-aarch64-c8-bare-apple/20260807T125300Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w60_n600020

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T175309Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.87 us | 1.56x |
| warm-clamp-accfit | 3.34 us | 1.35x |
| warm-clamp-accfit-dyn | 3.52 us | 1.42x |
| warm-clamp-head | 3.06 us | 1.23x |
| warm-clamp-min-lanes | 2.48 us | 1.00x |
| warm-clamp-minimum | 3.15 us | 1.27x |

- [20261114T211117Z-d0161800](warm-clamp/arity-w60_n600020/linux-x86_64-c16-bare-intel/20261114T211117Z-d0161800/report.md)
- [20261121T175309Z-eb17dd15](warm-clamp/arity-w60_n600020/linux-x86_64-c16-bare-intel/20261121T175309Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125316Z-ccf0509-dirty](warm-clamp/arity-w60_n600020/macos-aarch64-c8-bare-apple/20260807T125316Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w60_n600030

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T175309Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.70 us | 1.37x |
| warm-clamp-accfit | 2.50 us | 1.26x |
| warm-clamp-accfit-dyn | 2.70 us | 1.37x |
| warm-clamp-head | 3.04 us | 1.54x |
| warm-clamp-min-lanes | 1.98 us | 1.00x |
| warm-clamp-minimum | 2.42 us | 1.22x |

- [20261114T211408Z-d0161800](warm-clamp/arity-w60_n600030/linux-x86_64-c16-bare-intel/20261114T211408Z-d0161800/report.md)
- [20261121T175309Z-eb17dd15](warm-clamp/arity-w60_n600030/linux-x86_64-c16-bare-intel/20261121T175309Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125333Z-ccf0509-dirty](warm-clamp/arity-w60_n600030/macos-aarch64-c8-bare-apple/20260807T125333Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w60_n600040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T175309Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.41 us | 1.08x |
| warm-clamp-accfit | 2.34 us | 1.05x |
| warm-clamp-accfit-dyn | 2.23 us | 1.00x |
| warm-clamp-head | 3.05 us | 1.37x |
| warm-clamp-min-lanes | 2.46 us | 1.10x |
| warm-clamp-minimum | 2.41 us | 1.08x |

- [20261114T211658Z-d0161800](warm-clamp/arity-w60_n600040/linux-x86_64-c16-bare-intel/20261114T211658Z-d0161800/report.md)
- [20261121T175309Z-eb17dd15](warm-clamp/arity-w60_n600040/linux-x86_64-c16-bare-intel/20261121T175309Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125350Z-ccf0509-dirty](warm-clamp/arity-w60_n600040/macos-aarch64-c8-bare-apple/20260807T125350Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w60_n600060

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T175309Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 5.49 us | 2.05x |
| warm-clamp-accfit | 3.85 us | 1.44x |
| warm-clamp-accfit-dyn | 4.09 us | 1.53x |
| warm-clamp-head | 3.04 us | 1.14x |
| warm-clamp-min-lanes | 2.68 us | 1.00x |
| warm-clamp-minimum | 5.30 us | 1.98x |

- [20261114T211949Z-d0161800](warm-clamp/arity-w60_n600060/linux-x86_64-c16-bare-intel/20261114T211949Z-d0161800/report.md)
- [20261121T175309Z-eb17dd15](warm-clamp/arity-w60_n600060/linux-x86_64-c16-bare-intel/20261121T175309Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125406Z-ccf0509-dirty](warm-clamp/arity-w60_n600060/macos-aarch64-c8-bare-apple/20260807T125406Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w60_n600080

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T175309Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.61 us | 1.73x |
| warm-clamp-accfit | 2.09 us | 1.00x |
| warm-clamp-accfit-dyn | 2.67 us | 1.28x |
| warm-clamp-head | 3.08 us | 1.47x |
| warm-clamp-min-lanes | 2.24 us | 1.07x |
| warm-clamp-minimum | 3.07 us | 1.47x |

- [20261114T212240Z-d0161800](warm-clamp/arity-w60_n600080/linux-x86_64-c16-bare-intel/20261114T212240Z-d0161800/report.md)
- [20261121T175309Z-eb17dd15](warm-clamp/arity-w60_n600080/linux-x86_64-c16-bare-intel/20261121T175309Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125421Z-ccf0509-dirty](warm-clamp/arity-w60_n600080/macos-aarch64-c8-bare-apple/20260807T125421Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w64_n640010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T181045Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 4.00 us | 1.94x |
| warm-clamp-accfit | 2.06 us | 1.00x |
| warm-clamp-accfit-dyn | 2.06 us | 1.00x |
| warm-clamp-head | 3.05 us | 1.48x |
| warm-clamp-min-lanes | 3.79 us | 1.84x |
| warm-clamp-minimum | 4.16 us | 2.02x |

- [20261114T212556Z-d0161800](warm-clamp/arity-w64_n640010/linux-x86_64-c16-bare-intel/20261114T212556Z-d0161800/report.md)
- [20261121T181045Z-eb17dd15](warm-clamp/arity-w64_n640010/linux-x86_64-c16-bare-intel/20261121T181045Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125437Z-ccf0509-dirty](warm-clamp/arity-w64_n640010/macos-aarch64-c8-bare-apple/20260807T125437Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w64_n640020

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T181045Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.32 us | 1.00x |
| warm-clamp-accfit | 2.66 us | 1.15x |
| warm-clamp-accfit-dyn | 2.45 us | 1.05x |
| warm-clamp-head | 2.52 us | 1.09x |
| warm-clamp-min-lanes | 2.34 us | 1.01x |
| warm-clamp-minimum | 2.56 us | 1.11x |

- [20261114T212850Z-d0161800](warm-clamp/arity-w64_n640020/linux-x86_64-c16-bare-intel/20261114T212850Z-d0161800/report.md)
- [20261121T181045Z-eb17dd15](warm-clamp/arity-w64_n640020/linux-x86_64-c16-bare-intel/20261121T181045Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125453Z-ccf0509-dirty](warm-clamp/arity-w64_n640020/macos-aarch64-c8-bare-apple/20260807T125453Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w64_n640030

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T181045Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.53 us | 1.17x |
| warm-clamp-accfit | 3.71 us | 1.23x |
| warm-clamp-accfit-dyn | 3.94 us | 1.30x |
| warm-clamp-head | 3.03 us | 1.00x |
| warm-clamp-min-lanes | 4.45 us | 1.47x |
| warm-clamp-minimum | 3.76 us | 1.24x |

- [20261114T213141Z-d0161800](warm-clamp/arity-w64_n640030/linux-x86_64-c16-bare-intel/20261114T213141Z-d0161800/report.md)
- [20261121T181045Z-eb17dd15](warm-clamp/arity-w64_n640030/linux-x86_64-c16-bare-intel/20261121T181045Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125510Z-ccf0509-dirty](warm-clamp/arity-w64_n640030/macos-aarch64-c8-bare-apple/20260807T125510Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w64_n640040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T181045Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.87 us | 1.79x |
| warm-clamp-accfit | 2.82 us | 1.31x |
| warm-clamp-accfit-dyn | 2.16 us | 1.00x |
| warm-clamp-head | 3.06 us | 1.41x |
| warm-clamp-min-lanes | 3.11 us | 1.44x |
| warm-clamp-minimum | 3.70 us | 1.71x |

- [20261114T213432Z-d0161800](warm-clamp/arity-w64_n640040/linux-x86_64-c16-bare-intel/20261114T213432Z-d0161800/report.md)
- [20261121T181045Z-eb17dd15](warm-clamp/arity-w64_n640040/linux-x86_64-c16-bare-intel/20261121T181045Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125526Z-ccf0509-dirty](warm-clamp/arity-w64_n640040/macos-aarch64-c8-bare-apple/20260807T125526Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w64_n640060

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T181045Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.14 us | 1.14x |
| warm-clamp-accfit | 1.88 us | 1.00x |
| warm-clamp-accfit-dyn | 1.89 us | 1.01x |
| warm-clamp-head | 3.03 us | 1.62x |
| warm-clamp-min-lanes | 3.47 us | 1.85x |
| warm-clamp-minimum | 4.12 us | 2.20x |

- [20261114T213723Z-d0161800](warm-clamp/arity-w64_n640060/linux-x86_64-c16-bare-intel/20261114T213723Z-d0161800/report.md)
- [20261121T181045Z-eb17dd15](warm-clamp/arity-w64_n640060/linux-x86_64-c16-bare-intel/20261121T181045Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125542Z-ccf0509-dirty](warm-clamp/arity-w64_n640060/macos-aarch64-c8-bare-apple/20260807T125542Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w64_n640080

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T181045Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.44 us | 1.10x |
| warm-clamp-accfit | 2.22 us | 1.00x |
| warm-clamp-accfit-dyn | 2.23 us | 1.01x |
| warm-clamp-head | 3.08 us | 1.39x |
| warm-clamp-min-lanes | 3.39 us | 1.53x |
| warm-clamp-minimum | 2.46 us | 1.11x |

- [20261114T214014Z-d0161800](warm-clamp/arity-w64_n640080/linux-x86_64-c16-bare-intel/20261114T214014Z-d0161800/report.md)
- [20261121T181045Z-eb17dd15](warm-clamp/arity-w64_n640080/linux-x86_64-c16-bare-intel/20261121T181045Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125558Z-ccf0509-dirty](warm-clamp/arity-w64_n640080/macos-aarch64-c8-bare-apple/20260807T125558Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w8_n80010

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T182820Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.71 us | 1.55x |
| warm-clamp-accfit | 3.82 us | 1.59x |
| warm-clamp-accfit-dyn | 3.65 us | 1.52x |
| warm-clamp-head | 3.39 us | 1.41x |
| warm-clamp-min-lanes | 2.40 us | 1.00x |
| warm-clamp-minimum | 2.40 us | 1.00x |

- [20261114T214325Z-d0161800](warm-clamp/arity-w8_n80010/linux-x86_64-c16-bare-intel/20261114T214325Z-d0161800/report.md)
- [20261121T182820Z-eb17dd15](warm-clamp/arity-w8_n80010/linux-x86_64-c16-bare-intel/20261121T182820Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124655Z-ccf0509-dirty](warm-clamp/arity-w8_n80010/macos-aarch64-c8-bare-apple/20260807T124655Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w8_n80020

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T182820Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 4.56 us | 1.90x |
| warm-clamp-accfit | 3.76 us | 1.57x |
| warm-clamp-accfit-dyn | 3.87 us | 1.61x |
| warm-clamp-head | 5.50 us | 2.29x |
| warm-clamp-min-lanes | 2.40 us | 1.00x |
| warm-clamp-minimum | 2.40 us | 1.00x |

- [20261114T214618Z-d0161800](warm-clamp/arity-w8_n80020/linux-x86_64-c16-bare-intel/20261114T214618Z-d0161800/report.md)
- [20261121T182820Z-eb17dd15](warm-clamp/arity-w8_n80020/linux-x86_64-c16-bare-intel/20261121T182820Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124711Z-ccf0509-dirty](warm-clamp/arity-w8_n80020/macos-aarch64-c8-bare-apple/20260807T124711Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w8_n80030

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T182820Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 3.16 us | 1.35x |
| warm-clamp-accfit | 2.44 us | 1.04x |
| warm-clamp-accfit-dyn | 2.90 us | 1.24x |
| warm-clamp-head | 2.33 us | 1.00x |
| warm-clamp-min-lanes | 2.39 us | 1.02x |
| warm-clamp-minimum | 2.40 us | 1.03x |

- [20261114T214909Z-d0161800](warm-clamp/arity-w8_n80030/linux-x86_64-c16-bare-intel/20261114T214909Z-d0161800/report.md)
- [20261121T182820Z-eb17dd15](warm-clamp/arity-w8_n80030/linux-x86_64-c16-bare-intel/20261121T182820Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124726Z-ccf0509-dirty](warm-clamp/arity-w8_n80030/macos-aarch64-c8-bare-apple/20260807T124726Z-ccf0509-dirty/report.md)

## warm-clamp/arity-w8_n80040

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T182820Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 2.92 us | 1.95x |
| warm-clamp-accfit | 1.70 us | 1.13x |
| warm-clamp-accfit-dyn | 1.68 us | 1.12x |
| warm-clamp-head | 1.50 us | 1.00x |
| warm-clamp-min-lanes | 2.19 us | 1.46x |
| warm-clamp-minimum | 2.40 us | 1.60x |

- [20261114T215159Z-d0161800](warm-clamp/arity-w8_n80040/linux-x86_64-c16-bare-intel/20261114T215159Z-d0161800/report.md)
- [20261121T182820Z-eb17dd15](warm-clamp/arity-w8_n80040/linux-x86_64-c16-bare-intel/20261121T182820Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T124742Z-ccf0509-dirty](warm-clamp/arity-w8_n80040/macos-aarch64-c8-bare-apple/20260807T124742Z-ccf0509-dirty/report.md)

## warm-clamp/chain-l1_n130001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T184016Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 136 ns | 1.49x |
| warm-clamp-accfit | 179 ns | 1.97x |
| warm-clamp-accfit-dyn | 174 ns | 1.92x |
| warm-clamp-head | 169 ns | 1.85x |
| warm-clamp-min-lanes | 91 ns | 1.00x |
| warm-clamp-minimum | 111 ns | 1.22x |

- [20261114T215803Z-d0161800](warm-clamp/chain-l1_n130001/linux-x86_64-c16-bare-intel/20261114T215803Z-d0161800/report.md)
- [20261121T184016Z-eb17dd15](warm-clamp/chain-l1_n130001/linux-x86_64-c16-bare-intel/20261121T184016Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125629Z-ccf0509-dirty](warm-clamp/chain-l1_n130001/macos-aarch64-c8-bare-apple/20260807T125629Z-ccf0509-dirty/report.md)

## warm-clamp/chain-l1_n160001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T184016Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 219 ns | 2.55x |
| warm-clamp-accfit | 220 ns | 2.57x |
| warm-clamp-accfit-dyn | 180 ns | 2.10x |
| warm-clamp-head | 207 ns | 2.41x |
| warm-clamp-min-lanes | 86 ns | 1.00x |
| warm-clamp-minimum | 138 ns | 1.61x |

- [20261114T220053Z-d0161800](warm-clamp/chain-l1_n160001/linux-x86_64-c16-bare-intel/20261114T220053Z-d0161800/report.md)
- [20261121T184016Z-eb17dd15](warm-clamp/chain-l1_n160001/linux-x86_64-c16-bare-intel/20261121T184016Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125645Z-ccf0509-dirty](warm-clamp/chain-l1_n160001/macos-aarch64-c8-bare-apple/20260807T125645Z-ccf0509-dirty/report.md)

## warm-clamp/chain-l1_n320001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T184016Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 339 ns | 1.90x |
| warm-clamp-accfit | 382 ns | 2.14x |
| warm-clamp-accfit-dyn | 378 ns | 2.12x |
| warm-clamp-head | 409 ns | 2.29x |
| warm-clamp-min-lanes | 207 ns | 1.16x |
| warm-clamp-minimum | 179 ns | 1.00x |

- [20261114T220344Z-d0161800](warm-clamp/chain-l1_n320001/linux-x86_64-c16-bare-intel/20261114T220344Z-d0161800/report.md)
- [20261121T184016Z-eb17dd15](warm-clamp/chain-l1_n320001/linux-x86_64-c16-bare-intel/20261121T184016Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125701Z-ccf0509-dirty](warm-clamp/chain-l1_n320001/macos-aarch64-c8-bare-apple/20260807T125701Z-ccf0509-dirty/report.md)

## warm-clamp/chain-l1_n600001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T184016Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 340 ns | 1.00x |
| warm-clamp-accfit | 343 ns | 1.01x |
| warm-clamp-accfit-dyn | 357 ns | 1.05x |
| warm-clamp-head | 381 ns | 1.12x |
| warm-clamp-min-lanes | 373 ns | 1.10x |
| warm-clamp-minimum | 360 ns | 1.06x |

- [20261114T220635Z-d0161800](warm-clamp/chain-l1_n600001/linux-x86_64-c16-bare-intel/20261114T220635Z-d0161800/report.md)
- [20261121T184016Z-eb17dd15](warm-clamp/chain-l1_n600001/linux-x86_64-c16-bare-intel/20261121T184016Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125717Z-ccf0509-dirty](warm-clamp/chain-l1_n600001/macos-aarch64-c8-bare-apple/20260807T125717Z-ccf0509-dirty/report.md)

## warm-clamp/chain-l1_n640001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T184016Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 362 ns | 1.24x |
| warm-clamp-accfit | 303 ns | 1.03x |
| warm-clamp-accfit-dyn | 307 ns | 1.05x |
| warm-clamp-head | 324 ns | 1.11x |
| warm-clamp-min-lanes | 304 ns | 1.04x |
| warm-clamp-minimum | 293 ns | 1.00x |

- [20261114T220926Z-d0161800](warm-clamp/chain-l1_n640001/linux-x86_64-c16-bare-intel/20261114T220926Z-d0161800/report.md)
- [20261121T184016Z-eb17dd15](warm-clamp/chain-l1_n640001/linux-x86_64-c16-bare-intel/20261121T184016Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125733Z-ccf0509-dirty](warm-clamp/chain-l1_n640001/macos-aarch64-c8-bare-apple/20260807T125733Z-ccf0509-dirty/report.md)

## warm-clamp/chain-l1_n80001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T184016Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-clamp-acc64 | 136 ns | 2.46x |
| warm-clamp-accfit | 111 ns | 2.01x |
| warm-clamp-accfit-dyn | 89 ns | 1.61x |
| warm-clamp-head | 106 ns | 1.92x |
| warm-clamp-min-lanes | 56 ns | 1.01x |
| warm-clamp-minimum | 55 ns | 1.00x |

- [20261114T215510Z-d0161800](warm-clamp/chain-l1_n80001/linux-x86_64-c16-bare-intel/20261114T215510Z-d0161800/report.md)
- [20261121T184016Z-eb17dd15](warm-clamp/chain-l1_n80001/linux-x86_64-c16-bare-intel/20261121T184016Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T125613Z-ccf0509-dirty](warm-clamp/chain-l1_n80001/macos-aarch64-c8-bare-apple/20260807T125613Z-ccf0509-dirty/report.md)

## warm-container/affine-collapse-l1_n130403

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T185800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.95 us | 2.40x |
| warm-container-kernel | 1.65 us | 1.00x |
| warm-container-lanes-deferred | 1.65 us | 1.00x |
| warm-container-minimum | 1.91 us | 1.16x |
| warm-container-native | 1.75 us | 1.07x |
| warm-container-plusone | 2.59 us | 1.58x |

- [20261114T190858Z-d0161800](warm-container/affine-collapse-l1_n130403/linux-x86_64-c16-bare-intel/20261114T190858Z-d0161800/report.md)
- [20261121T185800Z-eb17dd15](warm-container/affine-collapse-l1_n130403/linux-x86_64-c16-bare-intel/20261121T185800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T112942Z-f763cb0-dirty](warm-container/affine-collapse-l1_n130403/macos-aarch64-c8-bare-apple/20260807T112942Z-f763cb0-dirty/report.md)

## warm-container/affine-collapse-l1_n160403

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T185800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.33 us | 2.87x |
| warm-container-kernel | 1.70 us | 1.13x |
| warm-container-lanes-deferred | 1.81 us | 1.20x |
| warm-container-minimum | 1.51 us | 1.00x |
| warm-container-native | 1.65 us | 1.09x |
| warm-container-plusone | 1.81 us | 1.20x |

- [20261114T191148Z-d0161800](warm-container/affine-collapse-l1_n160403/linux-x86_64-c16-bare-intel/20261114T191148Z-d0161800/report.md)
- [20261121T185800Z-eb17dd15](warm-container/affine-collapse-l1_n160403/linux-x86_64-c16-bare-intel/20261121T185800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T112957Z-f763cb0-dirty](warm-container/affine-collapse-l1_n160403/macos-aarch64-c8-bare-apple/20260807T112957Z-f763cb0-dirty/report.md)

## warm-container/affine-collapse-l1_n320403

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T185800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.16 us | 1.09x |
| warm-container-kernel | 2.99 us | 1.03x |
| warm-container-lanes-deferred | 3.53 us | 1.21x |
| warm-container-minimum | 2.91 us | 1.00x |
| warm-container-native | 3.47 us | 1.19x |
| warm-container-plusone | 4.68 us | 1.61x |

- [20261114T191438Z-d0161800](warm-container/affine-collapse-l1_n320403/linux-x86_64-c16-bare-intel/20261114T191438Z-d0161800/report.md)
- [20261121T185800Z-eb17dd15](warm-container/affine-collapse-l1_n320403/linux-x86_64-c16-bare-intel/20261121T185800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113013Z-f763cb0-dirty](warm-container/affine-collapse-l1_n320403/macos-aarch64-c8-bare-apple/20260807T113013Z-f763cb0-dirty/report.md)

## warm-container/affine-collapse-l1_n600403

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T185800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.44 us | 1.97x |
| warm-container-kernel | 1.75 us | 1.00x |
| warm-container-lanes-deferred | 1.85 us | 1.06x |
| warm-container-minimum | 4.23 us | 2.42x |
| warm-container-native | 4.01 us | 2.29x |
| warm-container-plusone | 1.88 us | 1.07x |

- [20261114T191729Z-d0161800](warm-container/affine-collapse-l1_n600403/linux-x86_64-c16-bare-intel/20261114T191729Z-d0161800/report.md)
- [20261121T185800Z-eb17dd15](warm-container/affine-collapse-l1_n600403/linux-x86_64-c16-bare-intel/20261121T185800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113029Z-f763cb0-dirty](warm-container/affine-collapse-l1_n600403/macos-aarch64-c8-bare-apple/20260807T113029Z-f763cb0-dirty/report.md)

## warm-container/affine-collapse-l1_n640403

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T185800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.32 us | 1.00x |
| warm-container-kernel | 4.21 us | 1.27x |
| warm-container-lanes-deferred | 4.18 us | 1.26x |
| warm-container-minimum | 3.84 us | 1.16x |
| warm-container-native | 4.07 us | 1.23x |
| warm-container-plusone | 4.05 us | 1.22x |

- [20261114T192019Z-d0161800](warm-container/affine-collapse-l1_n640403/linux-x86_64-c16-bare-intel/20261114T192019Z-d0161800/report.md)
- [20261121T185800Z-eb17dd15](warm-container/affine-collapse-l1_n640403/linux-x86_64-c16-bare-intel/20261121T185800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113045Z-f763cb0-dirty](warm-container/affine-collapse-l1_n640403/macos-aarch64-c8-bare-apple/20260807T113045Z-f763cb0-dirty/report.md)

## warm-container/affine-collapse-l1_n80403

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T185800Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.11 us | 4.45x |
| warm-container-kernel | 932 ns | 1.01x |
| warm-container-lanes-deferred | 931 ns | 1.01x |
| warm-container-minimum | 924 ns | 1.00x |
| warm-container-native | 927 ns | 1.00x |
| warm-container-plusone | 3.77 us | 4.08x |

- [20261114T190605Z-d0161800](warm-container/affine-collapse-l1_n80403/linux-x86_64-c16-bare-intel/20261114T190605Z-d0161800/report.md)
- [20261121T185800Z-eb17dd15](warm-container/affine-collapse-l1_n80403/linux-x86_64-c16-bare-intel/20261121T185800Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T112926Z-f763cb0-dirty](warm-container/affine-collapse-l1_n80403/macos-aarch64-c8-bare-apple/20260807T112926Z-f763cb0-dirty/report.md)

## warm-container/affine-density-w13_n130401

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T191531Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.84 us | 1.77x |
| warm-container-kernel | 1.04 us | 1.00x |
| warm-container-lanes-deferred | 1.04 us | 1.00x |
| warm-container-minimum | 4.55 us | 4.39x |
| warm-container-native | 1.09 us | 1.06x |
| warm-container-plusone | 4.06 us | 3.92x |

- [20261114T192330Z-d0161800](warm-container/affine-density-w13_n130401/linux-x86_64-c16-bare-intel/20261114T192330Z-d0161800/report.md)
- [20261121T191531Z-eb17dd15](warm-container/affine-density-w13_n130401/linux-x86_64-c16-bare-intel/20261121T191531Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113339Z-f763cb0-dirty](warm-container/affine-density-w13_n130401/macos-aarch64-c8-bare-apple/20260807T113339Z-f763cb0-dirty/report.md)

## warm-container/affine-density-w13_n130402

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T191531Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.02 us | 1.23x |
| warm-container-kernel | 1.64 us | 1.00x |
| warm-container-lanes-deferred | 1.81 us | 1.10x |
| warm-container-minimum | 2.39 us | 1.45x |
| warm-container-native | 1.85 us | 1.13x |
| warm-container-plusone | 4.19 us | 2.55x |

- [20261114T192623Z-d0161800](warm-container/affine-density-w13_n130402/linux-x86_64-c16-bare-intel/20261114T192623Z-d0161800/report.md)
- [20261121T191531Z-eb17dd15](warm-container/affine-density-w13_n130402/linux-x86_64-c16-bare-intel/20261121T191531Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113356Z-f763cb0-dirty](warm-container/affine-density-w13_n130402/macos-aarch64-c8-bare-apple/20260807T113356Z-f763cb0-dirty/report.md)

## warm-container/affine-density-w13_n130404

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T191531Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.81 us | 1.02x |
| warm-container-kernel | 1.78 us | 1.00x |
| warm-container-lanes-deferred | 1.84 us | 1.03x |
| warm-container-minimum | 1.80 us | 1.01x |
| warm-container-native | 2.11 us | 1.19x |
| warm-container-plusone | 3.55 us | 2.00x |

- [20261114T192912Z-d0161800](warm-container/affine-density-w13_n130404/linux-x86_64-c16-bare-intel/20261114T192912Z-d0161800/report.md)
- [20261121T191531Z-eb17dd15](warm-container/affine-density-w13_n130404/linux-x86_64-c16-bare-intel/20261121T191531Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113413Z-f763cb0-dirty](warm-container/affine-density-w13_n130404/macos-aarch64-c8-bare-apple/20260807T113413Z-f763cb0-dirty/report.md)

## warm-container/affine-density-w13_n130408

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T191531Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.36 us | 1.66x |
| warm-container-kernel | 1.44 us | 1.01x |
| warm-container-lanes-deferred | 1.42 us | 1.00x |
| warm-container-minimum | 4.50 us | 3.16x |
| warm-container-native | 1.76 us | 1.24x |
| warm-container-plusone | 3.54 us | 2.49x |

- [20261114T193202Z-d0161800](warm-container/affine-density-w13_n130408/linux-x86_64-c16-bare-intel/20261114T193202Z-d0161800/report.md)
- [20261121T191531Z-eb17dd15](warm-container/affine-density-w13_n130408/linux-x86_64-c16-bare-intel/20261121T191531Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113431Z-f763cb0-dirty](warm-container/affine-density-w13_n130408/macos-aarch64-c8-bare-apple/20260807T113431Z-f763cb0-dirty/report.md)

## warm-container/affine-density-w13_n130416

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T191531Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.38 us | 1.65x |
| warm-container-kernel | 1.44 us | 1.00x |
| warm-container-lanes-deferred | 1.58 us | 1.10x |
| warm-container-minimum | 4.05 us | 2.80x |
| warm-container-native | 1.57 us | 1.08x |
| warm-container-plusone | 4.26 us | 2.95x |

- [20261114T193452Z-d0161800](warm-container/affine-density-w13_n130416/linux-x86_64-c16-bare-intel/20261114T193452Z-d0161800/report.md)
- [20261121T191531Z-eb17dd15](warm-container/affine-density-w13_n130416/linux-x86_64-c16-bare-intel/20261121T191531Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113447Z-f763cb0-dirty](warm-container/affine-density-w13_n130416/macos-aarch64-c8-bare-apple/20260807T113447Z-f763cb0-dirty/report.md)

## warm-container/density-w13_n130001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T193010Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.87 us | 1.53x |
| warm-container-kernel | 4.09 us | 3.35x |
| warm-container-lanes-deferred | 4.08 us | 3.34x |
| warm-container-minimum | 3.59 us | 2.94x |
| warm-container-native | 1.22 us | 1.00x |
| warm-container-plusone | 4.26 us | 3.49x |

- [20261114T221236Z-d0161800](warm-container/density-w13_n130001/linux-x86_64-c16-bare-intel/20261114T221236Z-d0161800/report.md)
- [20261121T193010Z-eb17dd15](warm-container/density-w13_n130001/linux-x86_64-c16-bare-intel/20261121T193010Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111353Z-f763cb0-dirty](warm-container/density-w13_n130001/macos-aarch64-c8-bare-apple/20260807T111353Z-f763cb0-dirty/report.md)

## warm-container/density-w13_n130002

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T193010Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.80 us | 1.01x |
| warm-container-kernel | 2.54 us | 1.43x |
| warm-container-lanes-deferred | 3.69 us | 2.07x |
| warm-container-minimum | 4.47 us | 2.51x |
| warm-container-native | 1.80 us | 1.01x |
| warm-container-plusone | 1.78 us | 1.00x |

- [20261114T221529Z-d0161800](warm-container/density-w13_n130002/linux-x86_64-c16-bare-intel/20261114T221529Z-d0161800/report.md)
- [20261121T193010Z-eb17dd15](warm-container/density-w13_n130002/linux-x86_64-c16-bare-intel/20261121T193010Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111409Z-f763cb0-dirty](warm-container/density-w13_n130002/macos-aarch64-c8-bare-apple/20260807T111409Z-f763cb0-dirty/report.md)

## warm-container/density-w13_n130004

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T193010Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.24 us | 2.64x |
| warm-container-kernel | 4.41 us | 2.74x |
| warm-container-lanes-deferred | 4.60 us | 2.86x |
| warm-container-minimum | 3.29 us | 2.05x |
| warm-container-native | 1.61 us | 1.00x |
| warm-container-plusone | 4.34 us | 2.70x |

- [20261114T221819Z-d0161800](warm-container/density-w13_n130004/linux-x86_64-c16-bare-intel/20261114T221819Z-d0161800/report.md)
- [20261121T193010Z-eb17dd15](warm-container/density-w13_n130004/linux-x86_64-c16-bare-intel/20261121T193010Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111425Z-f763cb0-dirty](warm-container/density-w13_n130004/macos-aarch64-c8-bare-apple/20260807T111425Z-f763cb0-dirty/report.md)

## warm-container/density-w13_n130008

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T193010Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.33 us | 2.01x |
| warm-container-kernel | 5.02 us | 2.33x |
| warm-container-lanes-deferred | 2.16 us | 1.00x |
| warm-container-minimum | 4.32 us | 2.00x |
| warm-container-native | 2.68 us | 1.24x |
| warm-container-plusone | 4.31 us | 2.00x |

- [20261114T222109Z-d0161800](warm-container/density-w13_n130008/linux-x86_64-c16-bare-intel/20261114T222109Z-d0161800/report.md)
- [20261121T193010Z-eb17dd15](warm-container/density-w13_n130008/linux-x86_64-c16-bare-intel/20261121T193010Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111441Z-f763cb0-dirty](warm-container/density-w13_n130008/macos-aarch64-c8-bare-apple/20260807T111441Z-f763cb0-dirty/report.md)

## warm-container/density-w13_n130016

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T193010Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 7.95 us | 3.81x |
| warm-container-kernel | 2.08 us | 1.00x |
| warm-container-lanes-deferred | 3.06 us | 1.47x |
| warm-container-minimum | 7.16 us | 3.43x |
| warm-container-native | 4.21 us | 2.02x |
| warm-container-plusone | 7.94 us | 3.81x |

- [20261114T222359Z-d0161800](warm-container/density-w13_n130016/linux-x86_64-c16-bare-intel/20261114T222359Z-d0161800/report.md)
- [20261121T193010Z-eb17dd15](warm-container/density-w13_n130016/linux-x86_64-c16-bare-intel/20261121T193010Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111457Z-f763cb0-dirty](warm-container/density-w13_n130016/macos-aarch64-c8-bare-apple/20260807T111457Z-f763cb0-dirty/report.md)

## warm-container/density-w64_n640001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T194455Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.65 us | 1.00x |
| warm-container-kernel | 2.69 us | 1.02x |
| warm-container-minimum | 3.43 us | 1.29x |
| warm-container-native | 3.83 us | 1.44x |
| warm-container-plusone | 2.65 us | 1.00x |

- [20261114T222648Z-d0161800](warm-container/density-w64_n640001/linux-x86_64-c16-bare-intel/20261114T222648Z-d0161800/report.md)
- [20261121T194455Z-eb17dd15](warm-container/density-w64_n640001/linux-x86_64-c16-bare-intel/20261121T194455Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105927Z-f763cb0-dirty](warm-container/density-w64_n640001/macos-aarch64-c8-bare-apple/20260807T105927Z-f763cb0-dirty/report.md)

## warm-container/density-w64_n640002

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T194455Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.22 us | 1.44x |
| warm-container-kernel | 2.24 us | 1.00x |
| warm-container-minimum | 2.23 us | 1.00x |
| warm-container-native | 3.85 us | 1.72x |
| warm-container-plusone | 3.34 us | 1.50x |

- [20261114T222914Z-d0161800](warm-container/density-w64_n640002/linux-x86_64-c16-bare-intel/20261114T222914Z-d0161800/report.md)
- [20261121T194455Z-eb17dd15](warm-container/density-w64_n640002/linux-x86_64-c16-bare-intel/20261121T194455Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105941Z-f763cb0-dirty](warm-container/density-w64_n640002/macos-aarch64-c8-bare-apple/20260807T105941Z-f763cb0-dirty/report.md)

## warm-container/density-w64_n640004

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T194455Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 5.57 us | 2.90x |
| warm-container-kernel | 1.92 us | 1.00x |
| warm-container-minimum | 1.93 us | 1.01x |
| warm-container-native | 3.10 us | 1.61x |
| warm-container-plusone | 5.56 us | 2.90x |

- [20261114T223137Z-d0161800](warm-container/density-w64_n640004/linux-x86_64-c16-bare-intel/20261114T223137Z-d0161800/report.md)
- [20261121T194455Z-eb17dd15](warm-container/density-w64_n640004/linux-x86_64-c16-bare-intel/20261121T194455Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105956Z-f763cb0-dirty](warm-container/density-w64_n640004/macos-aarch64-c8-bare-apple/20260807T105956Z-f763cb0-dirty/report.md)

## warm-container/density-w64_n640008

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T194455Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 7.84 us | 2.26x |
| warm-container-kernel | 3.47 us | 1.00x |
| warm-container-minimum | 3.67 us | 1.06x |
| warm-container-native | 4.49 us | 1.29x |
| warm-container-plusone | 9.57 us | 2.76x |

- [20261114T223401Z-d0161800](warm-container/density-w64_n640008/linux-x86_64-c16-bare-intel/20261114T223401Z-d0161800/report.md)
- [20261121T194455Z-eb17dd15](warm-container/density-w64_n640008/linux-x86_64-c16-bare-intel/20261121T194455Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110010Z-f763cb0-dirty](warm-container/density-w64_n640008/macos-aarch64-c8-bare-apple/20260807T110010Z-f763cb0-dirty/report.md)

## warm-container/density-w64_n640016

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T194455Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 17.15 us | 2.45x |
| warm-container-kernel | 7.70 us | 1.10x |
| warm-container-minimum | 6.99 us | 1.00x |
| warm-container-native | 7.76 us | 1.11x |
| warm-container-plusone | 17.22 us | 2.46x |

- [20261114T223624Z-d0161800](warm-container/density-w64_n640016/linux-x86_64-c16-bare-intel/20261114T223624Z-d0161800/report.md)
- [20261121T194455Z-eb17dd15](warm-container/density-w64_n640016/linux-x86_64-c16-bare-intel/20261121T194455Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110024Z-f763cb0-dirty](warm-container/density-w64_n640016/macos-aarch64-c8-bare-apple/20260807T110024Z-f763cb0-dirty/report.md)

## warm-container/elementwise-width-l1_n130204

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T195721Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.22 us | 1.68x |
| warm-container-kernel | 1.98 us | 1.03x |
| warm-container-minimum | 1.94 us | 1.01x |
| warm-container-native | 2.12 us | 1.10x |
| warm-container-plusone | 1.92 us | 1.00x |

- [20261114T232228Z-d0161800](warm-container/elementwise-width-l1_n130204/linux-x86_64-c16-bare-intel/20261114T232228Z-d0161800/report.md)
- [20261121T195721Z-eb17dd15](warm-container/elementwise-width-l1_n130204/linux-x86_64-c16-bare-intel/20261121T195721Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110935Z-f763cb0-dirty](warm-container/elementwise-width-l1_n130204/macos-aarch64-c8-bare-apple/20260807T110935Z-f763cb0-dirty/report.md)

## warm-container/elementwise-width-l1_n160204

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T195721Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.29 us | 1.88x |
| warm-container-kernel | 1.79 us | 1.03x |
| warm-container-minimum | 1.79 us | 1.02x |
| warm-container-native | 1.75 us | 1.00x |
| warm-container-plusone | 3.79 us | 2.16x |

- [20261114T232450Z-d0161800](warm-container/elementwise-width-l1_n160204/linux-x86_64-c16-bare-intel/20261114T232450Z-d0161800/report.md)
- [20261121T195721Z-eb17dd15](warm-container/elementwise-width-l1_n160204/linux-x86_64-c16-bare-intel/20261121T195721Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110949Z-f763cb0-dirty](warm-container/elementwise-width-l1_n160204/macos-aarch64-c8-bare-apple/20260807T110949Z-f763cb0-dirty/report.md)

## warm-container/elementwise-width-l1_n320204

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T195721Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.33 us | 1.00x |
| warm-container-kernel | 2.98 us | 1.28x |
| warm-container-minimum | 2.94 us | 1.26x |
| warm-container-native | 3.51 us | 1.51x |
| warm-container-plusone | 2.33 us | 1.00x |

- [20261114T232713Z-d0161800](warm-container/elementwise-width-l1_n320204/linux-x86_64-c16-bare-intel/20261114T232713Z-d0161800/report.md)
- [20261121T195721Z-eb17dd15](warm-container/elementwise-width-l1_n320204/linux-x86_64-c16-bare-intel/20261121T195721Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111003Z-f763cb0-dirty](warm-container/elementwise-width-l1_n320204/macos-aarch64-c8-bare-apple/20260807T111003Z-f763cb0-dirty/report.md)

## warm-container/elementwise-width-l1_n600204

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T195721Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.27 us | 1.00x |
| warm-container-kernel | 2.32 us | 1.02x |
| warm-container-minimum | 2.32 us | 1.02x |
| warm-container-native | 2.32 us | 1.02x |
| warm-container-plusone | 2.37 us | 1.04x |

- [20261114T232936Z-d0161800](warm-container/elementwise-width-l1_n600204/linux-x86_64-c16-bare-intel/20261114T232936Z-d0161800/report.md)
- [20261121T195721Z-eb17dd15](warm-container/elementwise-width-l1_n600204/linux-x86_64-c16-bare-intel/20261121T195721Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111016Z-f763cb0-dirty](warm-container/elementwise-width-l1_n600204/macos-aarch64-c8-bare-apple/20260807T111016Z-f763cb0-dirty/report.md)

## warm-container/elementwise-width-l1_n640204

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T195721Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 5.25 us | 2.67x |
| warm-container-kernel | 1.97 us | 1.00x |
| warm-container-minimum | 2.06 us | 1.05x |
| warm-container-native | 2.04 us | 1.04x |
| warm-container-plusone | 5.26 us | 2.67x |

- [20261114T233200Z-d0161800](warm-container/elementwise-width-l1_n640204/linux-x86_64-c16-bare-intel/20261114T233200Z-d0161800/report.md)
- [20261121T195721Z-eb17dd15](warm-container/elementwise-width-l1_n640204/linux-x86_64-c16-bare-intel/20261121T195721Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111029Z-f763cb0-dirty](warm-container/elementwise-width-l1_n640204/macos-aarch64-c8-bare-apple/20260807T111029Z-f763cb0-dirty/report.md)

## warm-container/elementwise-width-l1_n80204

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T195721Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.11 us | 2.27x |
| warm-container-kernel | 956 ns | 1.02x |
| warm-container-minimum | 932 ns | 1.00x |
| warm-container-native | 1.06 us | 1.14x |
| warm-container-plusone | 2.11 us | 2.27x |

- [20261114T232003Z-d0161800](warm-container/elementwise-width-l1_n80204/linux-x86_64-c16-bare-intel/20261114T232003Z-d0161800/report.md)
- [20261121T195721Z-eb17dd15](warm-container/elementwise-width-l1_n80204/linux-x86_64-c16-bare-intel/20261121T195721Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110922Z-f763cb0-dirty](warm-container/elementwise-width-l1_n80204/macos-aarch64-c8-bare-apple/20260807T110922Z-f763cb0-dirty/report.md)

## warm-container/precise-elementwise-width-l1_n130304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T201208Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.37 us | 1.28x |
| warm-container-kernel | 2.49 us | 1.34x |
| warm-container-minimum | 1.97 us | 1.06x |
| warm-container-native | 1.97 us | 1.06x |
| warm-container-plusone | 1.85 us | 1.00x |

- [20261114T141804Z-d0161800](warm-container/precise-elementwise-width-l1_n130304/linux-x86_64-c16-bare-intel/20261114T141804Z-d0161800/report.md)
- [20261121T201208Z-eb17dd15](warm-container/precise-elementwise-width-l1_n130304/linux-x86_64-c16-bare-intel/20261121T201208Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110710Z-f763cb0-dirty](warm-container/precise-elementwise-width-l1_n130304/macos-aarch64-c8-bare-apple/20260807T110710Z-f763cb0-dirty/report.md)

## warm-container/precise-elementwise-width-l1_n160304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T201208Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.37 us | 2.00x |
| warm-container-kernel | 1.25 us | 1.05x |
| warm-container-minimum | 1.19 us | 1.00x |
| warm-container-native | 1.40 us | 1.18x |
| warm-container-plusone | 2.37 us | 2.00x |

- [20261114T142027Z-d0161800](warm-container/precise-elementwise-width-l1_n160304/linux-x86_64-c16-bare-intel/20261114T142027Z-d0161800/report.md)
- [20261121T201208Z-eb17dd15](warm-container/precise-elementwise-width-l1_n160304/linux-x86_64-c16-bare-intel/20261121T201208Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110723Z-f763cb0-dirty](warm-container/precise-elementwise-width-l1_n160304/macos-aarch64-c8-bare-apple/20260807T110723Z-f763cb0-dirty/report.md)

## warm-container/precise-elementwise-width-l1_n320304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T201208Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.76 us | 1.05x |
| warm-container-kernel | 3.92 us | 1.10x |
| warm-container-minimum | 3.57 us | 1.00x |
| warm-container-native | 3.74 us | 1.05x |
| warm-container-plusone | 3.76 us | 1.05x |

- [20261114T142249Z-d0161800](warm-container/precise-elementwise-width-l1_n320304/linux-x86_64-c16-bare-intel/20261114T142249Z-d0161800/report.md)
- [20261121T201208Z-eb17dd15](warm-container/precise-elementwise-width-l1_n320304/linux-x86_64-c16-bare-intel/20261121T201208Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110736Z-f763cb0-dirty](warm-container/precise-elementwise-width-l1_n320304/macos-aarch64-c8-bare-apple/20260807T110736Z-f763cb0-dirty/report.md)

## warm-container/precise-elementwise-width-l1_n600304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T201208Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 6.82 us | 2.21x |
| warm-container-kernel | 4.90 us | 1.59x |
| warm-container-minimum | 3.08 us | 1.00x |
| warm-container-native | 3.77 us | 1.22x |
| warm-container-plusone | 3.49 us | 1.13x |

- [20261114T142513Z-d0161800](warm-container/precise-elementwise-width-l1_n600304/linux-x86_64-c16-bare-intel/20261114T142513Z-d0161800/report.md)
- [20261121T201208Z-eb17dd15](warm-container/precise-elementwise-width-l1_n600304/linux-x86_64-c16-bare-intel/20261121T201208Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110750Z-f763cb0-dirty](warm-container/precise-elementwise-width-l1_n600304/macos-aarch64-c8-bare-apple/20260807T110750Z-f763cb0-dirty/report.md)

## warm-container/precise-elementwise-width-l1_n640304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T201208Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 7.80 us | 2.87x |
| warm-container-kernel | 2.71 us | 1.00x |
| warm-container-minimum | 2.94 us | 1.08x |
| warm-container-native | 3.02 us | 1.11x |
| warm-container-plusone | 7.08 us | 2.61x |

- [20261114T142736Z-d0161800](warm-container/precise-elementwise-width-l1_n640304/linux-x86_64-c16-bare-intel/20261114T142736Z-d0161800/report.md)
- [20261121T201208Z-eb17dd15](warm-container/precise-elementwise-width-l1_n640304/linux-x86_64-c16-bare-intel/20261121T201208Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110803Z-f763cb0-dirty](warm-container/precise-elementwise-width-l1_n640304/macos-aarch64-c8-bare-apple/20260807T110803Z-f763cb0-dirty/report.md)

## warm-container/precise-elementwise-width-l1_n80304

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T201208Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.58 us | 2.80x |
| warm-container-kernel | 594 ns | 1.05x |
| warm-container-minimum | 657 ns | 1.16x |
| warm-container-native | 566 ns | 1.00x |
| warm-container-plusone | 2.05 us | 3.62x |

- [20261114T141538Z-d0161800](warm-container/precise-elementwise-width-l1_n80304/linux-x86_64-c16-bare-intel/20261114T141538Z-d0161800/report.md)
- [20261121T201208Z-eb17dd15](warm-container/precise-elementwise-width-l1_n80304/linux-x86_64-c16-bare-intel/20261121T201208Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T110656Z-f763cb0-dirty](warm-container/precise-elementwise-width-l1_n80304/macos-aarch64-c8-bare-apple/20260807T110656Z-f763cb0-dirty/report.md)

## warm-container/precise-widening-theorem-l1_n130501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T202654Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.77 us | 1.19x |
| warm-container-kernel | 3.55 us | 1.52x |
| warm-container-lanes-deferred | 3.69 us | 1.59x |
| warm-container-minimum | 2.33 us | 1.00x |
| warm-container-native | 2.81 us | 1.21x |
| warm-container-plusone | 2.63 us | 1.13x |

- [20261114T143346Z-d0161800](warm-container/precise-widening-theorem-l1_n130501/linux-x86_64-c16-bare-intel/20261114T143346Z-d0161800/report.md)
- [20261121T202654Z-eb17dd15](warm-container/precise-widening-theorem-l1_n130501/linux-x86_64-c16-bare-intel/20261121T202654Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113118Z-f763cb0-dirty](warm-container/precise-widening-theorem-l1_n130501/macos-aarch64-c8-bare-apple/20260807T113118Z-f763cb0-dirty/report.md)

## warm-container/precise-widening-theorem-l1_n160501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T202654Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.77 us | 1.27x |
| warm-container-kernel | 3.72 us | 1.70x |
| warm-container-lanes-deferred | 3.27 us | 1.49x |
| warm-container-minimum | 2.19 us | 1.00x |
| warm-container-native | 2.76 us | 1.26x |
| warm-container-plusone | 2.83 us | 1.29x |

- [20261114T143636Z-d0161800](warm-container/precise-widening-theorem-l1_n160501/linux-x86_64-c16-bare-intel/20261114T143636Z-d0161800/report.md)
- [20261121T202654Z-eb17dd15](warm-container/precise-widening-theorem-l1_n160501/linux-x86_64-c16-bare-intel/20261121T202654Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113135Z-f763cb0-dirty](warm-container/precise-widening-theorem-l1_n160501/macos-aarch64-c8-bare-apple/20260807T113135Z-f763cb0-dirty/report.md)

## warm-container/precise-widening-theorem-l1_n320501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T202654Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.89 us | 1.00x |
| warm-container-kernel | 2.51 us | 1.33x |
| warm-container-lanes-deferred | 2.45 us | 1.30x |
| warm-container-minimum | 2.31 us | 1.23x |
| warm-container-native | 2.80 us | 1.48x |
| warm-container-plusone | 2.30 us | 1.22x |

- [20261114T143925Z-d0161800](warm-container/precise-widening-theorem-l1_n320501/linux-x86_64-c16-bare-intel/20261114T143925Z-d0161800/report.md)
- [20261121T202654Z-eb17dd15](warm-container/precise-widening-theorem-l1_n320501/linux-x86_64-c16-bare-intel/20261121T202654Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113152Z-f763cb0-dirty](warm-container/precise-widening-theorem-l1_n320501/macos-aarch64-c8-bare-apple/20260807T113152Z-f763cb0-dirty/report.md)

## warm-container/precise-widening-theorem-l1_n600501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T202654Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.87 us | 1.21x |
| warm-container-kernel | 3.78 us | 1.60x |
| warm-container-lanes-deferred | 4.14 us | 1.75x |
| warm-container-minimum | 2.36 us | 1.00x |
| warm-container-native | 2.80 us | 1.18x |
| warm-container-plusone | 2.69 us | 1.14x |

- [20261114T144216Z-d0161800](warm-container/precise-widening-theorem-l1_n600501/linux-x86_64-c16-bare-intel/20261114T144216Z-d0161800/report.md)
- [20261121T202654Z-eb17dd15](warm-container/precise-widening-theorem-l1_n600501/linux-x86_64-c16-bare-intel/20261121T202654Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113209Z-f763cb0-dirty](warm-container/precise-widening-theorem-l1_n600501/macos-aarch64-c8-bare-apple/20260807T113209Z-f763cb0-dirty/report.md)

## warm-container/precise-widening-theorem-l1_n640501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T202654Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.04 us | 1.20x |
| warm-container-kernel | 3.83 us | 1.14x |
| warm-container-lanes-deferred | 4.07 us | 1.21x |
| warm-container-minimum | 3.91 us | 1.16x |
| warm-container-native | 3.77 us | 1.12x |
| warm-container-plusone | 3.37 us | 1.00x |

- [20261114T144507Z-d0161800](warm-container/precise-widening-theorem-l1_n640501/linux-x86_64-c16-bare-intel/20261114T144507Z-d0161800/report.md)
- [20261121T202654Z-eb17dd15](warm-container/precise-widening-theorem-l1_n640501/linux-x86_64-c16-bare-intel/20261121T202654Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113225Z-f763cb0-dirty](warm-container/precise-widening-theorem-l1_n640501/macos-aarch64-c8-bare-apple/20260807T113225Z-f763cb0-dirty/report.md)

## warm-container/precise-widening-theorem-l1_n80501

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T202654Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.83 us | 1.22x |
| warm-container-kernel | 3.32 us | 1.42x |
| warm-container-lanes-deferred | 3.51 us | 1.51x |
| warm-container-minimum | 2.33 us | 1.00x |
| warm-container-native | 2.78 us | 1.19x |
| warm-container-plusone | 2.78 us | 1.19x |

- [20261114T143053Z-d0161800](warm-container/precise-widening-theorem-l1_n80501/linux-x86_64-c16-bare-intel/20261114T143053Z-d0161800/report.md)
- [20261121T202654Z-eb17dd15](warm-container/precise-widening-theorem-l1_n80501/linux-x86_64-c16-bare-intel/20261121T202654Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T113102Z-f763cb0-dirty](warm-container/precise-widening-theorem-l1_n80501/macos-aarch64-c8-bare-apple/20260807T113102Z-f763cb0-dirty/report.md)

## warm-container/precise-width-l1_n130103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T204426Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.01 us | 3.80x |
| warm-container-kernel | 1.06 us | 1.00x |
| warm-container-minimum | 7.46 us | 7.06x |
| warm-container-native | 7.74 us | 7.32x |
| warm-container-plusone | 7.75 us | 7.34x |

- [20261114T140323Z-d0161800](warm-container/precise-width-l1_n130103/linux-x86_64-c16-bare-intel/20261114T140323Z-d0161800/report.md)
- [20261121T204426Z-eb17dd15](warm-container/precise-width-l1_n130103/linux-x86_64-c16-bare-intel/20261121T204426Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105512Z-f763cb0-dirty](warm-container/precise-width-l1_n130103/macos-aarch64-c8-bare-apple/20260807T105512Z-f763cb0-dirty/report.md)

## warm-container/precise-width-l1_n160103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T204426Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 3.29 us | 56.97x |
| warm-container-kernel | 58 ns | 1.00x |
| warm-container-minimum | 2.36 us | 40.87x |
| warm-container-native | 2.87 us | 49.69x |
| warm-container-plusone | 3.29 us | 56.97x |

- [20261114T140545Z-d0161800](warm-container/precise-width-l1_n160103/linux-x86_64-c16-bare-intel/20261114T140545Z-d0161800/report.md)
- [20261121T204426Z-eb17dd15](warm-container/precise-width-l1_n160103/linux-x86_64-c16-bare-intel/20261121T204426Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105526Z-f763cb0-dirty](warm-container/precise-width-l1_n160103/macos-aarch64-c8-bare-apple/20260807T105526Z-f763cb0-dirty/report.md)

## warm-container/precise-width-l1_n320103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T204426Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.10 us | 79.67x |
| warm-container-kernel | 51 ns | 1.00x |
| warm-container-minimum | 2.87 us | 55.86x |
| warm-container-native | 2.88 us | 55.98x |
| warm-container-plusone | 4.09 us | 79.65x |

- [20261114T140808Z-d0161800](warm-container/precise-width-l1_n320103/linux-x86_64-c16-bare-intel/20261114T140808Z-d0161800/report.md)
- [20261121T204426Z-eb17dd15](warm-container/precise-width-l1_n320103/linux-x86_64-c16-bare-intel/20261121T204426Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105539Z-f763cb0-dirty](warm-container/precise-width-l1_n320103/macos-aarch64-c8-bare-apple/20260807T105539Z-f763cb0-dirty/report.md)

## warm-container/precise-width-l1_n600103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T204426Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 8.43 us | 8.38x |
| warm-container-kernel | 1.01 us | 1.00x |
| warm-container-minimum | 4.03 us | 4.01x |
| warm-container-native | 4.03 us | 4.01x |
| warm-container-plusone | 4.04 us | 4.01x |

- [20261114T141032Z-d0161800](warm-container/precise-width-l1_n600103/linux-x86_64-c16-bare-intel/20261114T141032Z-d0161800/report.md)
- [20261121T204426Z-eb17dd15](warm-container/precise-width-l1_n600103/linux-x86_64-c16-bare-intel/20261121T204426Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105552Z-f763cb0-dirty](warm-container/precise-width-l1_n600103/macos-aarch64-c8-bare-apple/20260807T105552Z-f763cb0-dirty/report.md)

## warm-container/precise-width-l1_n640103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T204426Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 9.35 us | 143.86x |
| warm-container-kernel | 65 ns | 1.00x |
| warm-container-minimum | 2.88 us | 44.30x |
| warm-container-native | 2.35 us | 36.17x |
| warm-container-plusone | 10.40 us | 159.99x |

- [20261114T141255Z-d0161800](warm-container/precise-width-l1_n640103/linux-x86_64-c16-bare-intel/20261114T141255Z-d0161800/report.md)
- [20261121T204426Z-eb17dd15](warm-container/precise-width-l1_n640103/linux-x86_64-c16-bare-intel/20261121T204426Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105605Z-f763cb0-dirty](warm-container/precise-width-l1_n640103/macos-aarch64-c8-bare-apple/20260807T105605Z-f763cb0-dirty/report.md)

## warm-container/precise-width-l1_n80103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T204426Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 5.07 us | 93.25x |
| warm-container-kernel | 54 ns | 1.00x |
| warm-container-minimum | 3.06 us | 56.28x |
| warm-container-native | 2.93 us | 53.94x |
| warm-container-plusone | 5.48 us | 100.73x |

- [20261114T140056Z-d0161800](warm-container/precise-width-l1_n80103/linux-x86_64-c16-bare-intel/20261114T140056Z-d0161800/report.md)
- [20261121T204426Z-eb17dd15](warm-container/precise-width-l1_n80103/linux-x86_64-c16-bare-intel/20261121T204426Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T105458Z-f763cb0-dirty](warm-container/precise-width-l1_n80103/macos-aarch64-c8-bare-apple/20260807T105458Z-f763cb0-dirty/report.md)

## warm-container/width-l1_n130003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T205918Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.87 us | 1.31x |
| warm-container-kernel | 3.91 us | 2.74x |
| warm-container-lanes-deferred | 4.43 us | 3.10x |
| warm-container-minimum | 3.09 us | 2.16x |
| warm-container-native | 1.43 us | 1.00x |
| warm-container-plusone | 3.42 us | 2.40x |

- [20261114T224228Z-d0161800](warm-container/width-l1_n130003/linux-x86_64-c16-bare-intel/20261114T224228Z-d0161800/report.md)
- [20261121T205918Z-eb17dd15](warm-container/width-l1_n130003/linux-x86_64-c16-bare-intel/20261121T205918Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111529Z-f763cb0-dirty](warm-container/width-l1_n130003/macos-aarch64-c8-bare-apple/20260807T111529Z-f763cb0-dirty/report.md)

## warm-container/width-l1_n160003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T205918Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.82 us | 1.63x |
| warm-container-kernel | 3.98 us | 2.30x |
| warm-container-lanes-deferred | 3.70 us | 2.14x |
| warm-container-minimum | 1.80 us | 1.04x |
| warm-container-native | 1.73 us | 1.00x |
| warm-container-plusone | 1.81 us | 1.04x |

- [20261114T224518Z-d0161800](warm-container/width-l1_n160003/linux-x86_64-c16-bare-intel/20261114T224518Z-d0161800/report.md)
- [20261121T205918Z-eb17dd15](warm-container/width-l1_n160003/linux-x86_64-c16-bare-intel/20261121T205918Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111545Z-f763cb0-dirty](warm-container/width-l1_n160003/macos-aarch64-c8-bare-apple/20260807T111545Z-f763cb0-dirty/report.md)

## warm-container/width-l1_n320003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T205918Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.77 us | 2.75x |
| warm-container-kernel | 3.16 us | 1.83x |
| warm-container-lanes-deferred | 3.16 us | 1.83x |
| warm-container-minimum | 3.08 us | 1.78x |
| warm-container-native | 3.57 us | 2.06x |
| warm-container-plusone | 1.73 us | 1.00x |

- [20261114T224808Z-d0161800](warm-container/width-l1_n320003/linux-x86_64-c16-bare-intel/20261114T224808Z-d0161800/report.md)
- [20261121T205918Z-eb17dd15](warm-container/width-l1_n320003/linux-x86_64-c16-bare-intel/20261121T205918Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111601Z-f763cb0-dirty](warm-container/width-l1_n320003/macos-aarch64-c8-bare-apple/20260807T111601Z-f763cb0-dirty/report.md)

## warm-container/width-l1_n600003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T205918Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 2.82 us | 1.74x |
| warm-container-kernel | 4.00 us | 2.47x |
| warm-container-lanes-deferred | 3.73 us | 2.30x |
| warm-container-minimum | 2.55 us | 1.57x |
| warm-container-native | 1.62 us | 1.00x |
| warm-container-plusone | 4.00 us | 2.47x |

- [20261114T225059Z-d0161800](warm-container/width-l1_n600003/linux-x86_64-c16-bare-intel/20261114T225059Z-d0161800/report.md)
- [20261121T205918Z-eb17dd15](warm-container/width-l1_n600003/linux-x86_64-c16-bare-intel/20261121T205918Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111618Z-f763cb0-dirty](warm-container/width-l1_n600003/macos-aarch64-c8-bare-apple/20260807T111618Z-f763cb0-dirty/report.md)

## warm-container/width-l1_n640003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T205918Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 4.05 us | 2.41x |
| warm-container-kernel | 3.53 us | 2.10x |
| warm-container-lanes-deferred | 1.68 us | 1.00x |
| warm-container-minimum | 3.62 us | 2.16x |
| warm-container-native | 2.18 us | 1.30x |
| warm-container-plusone | 4.05 us | 2.41x |

- [20261114T225349Z-d0161800](warm-container/width-l1_n640003/linux-x86_64-c16-bare-intel/20261114T225349Z-d0161800/report.md)
- [20261121T205918Z-eb17dd15](warm-container/width-l1_n640003/linux-x86_64-c16-bare-intel/20261121T205918Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111634Z-f763cb0-dirty](warm-container/width-l1_n640003/macos-aarch64-c8-bare-apple/20260807T111634Z-f763cb0-dirty/report.md)

## warm-container/width-l1_n80003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T205918Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 1.86 us | 1.81x |
| warm-container-kernel | 2.43 us | 2.35x |
| warm-container-lanes-deferred | 2.43 us | 2.35x |
| warm-container-minimum | 1.03 us | 1.00x |
| warm-container-native | 1.09 us | 1.06x |
| warm-container-plusone | 4.06 us | 3.94x |

- [20261114T223935Z-d0161800](warm-container/width-l1_n80003/linux-x86_64-c16-bare-intel/20261114T223935Z-d0161800/report.md)
- [20261121T205918Z-eb17dd15](warm-container/width-l1_n80003/linux-x86_64-c16-bare-intel/20261121T205918Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T111513Z-f763cb0-dirty](warm-container/width-l1_n80003/macos-aarch64-c8-bare-apple/20260807T111513Z-f763cb0-dirty/report.md)

## warm-container/width-l2_n131003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T211650Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 195.84 us | 3.59x |
| warm-container-minimum | 187.07 us | 3.43x |
| warm-container-native | 54.56 us | 1.00x |
| warm-container-plusone | 187.07 us | 3.43x |

- [20261114T230123Z-d0161800](warm-container/width-l2_n131003/linux-x86_64-c16-bare-intel/20261114T230123Z-d0161800/report.md)
- [20261121T211650Z-eb17dd15](warm-container/width-l2_n131003/linux-x86_64-c16-bare-intel/20261121T211650Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T103945Z-f763cb0-dirty](warm-container/width-l2_n131003/macos-aarch64-c8-bare-apple/20260807T103945Z-f763cb0-dirty/report.md)

## warm-container/width-l2_n161003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T211650Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 194.66 us | 3.58x |
| warm-container-minimum | 54.35 us | 1.00x |
| warm-container-native | 54.67 us | 1.01x |
| warm-container-plusone | 194.55 us | 3.58x |

- [20261114T230450Z-d0161800](warm-container/width-l2_n161003/linux-x86_64-c16-bare-intel/20261114T230450Z-d0161800/report.md)
- [20261121T211650Z-eb17dd15](warm-container/width-l2_n161003/linux-x86_64-c16-bare-intel/20261121T211650Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T104213Z-f763cb0-dirty](warm-container/width-l2_n161003/macos-aarch64-c8-bare-apple/20260807T104213Z-f763cb0-dirty/report.md)

## warm-container/width-l2_n321003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T211650Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 281.16 us | 2.58x |
| warm-container-minimum | 108.87 us | 1.00x |
| warm-container-native | 108.89 us | 1.00x |
| warm-container-plusone | 230.72 us | 2.12x |

- [20261114T230831Z-d0161800](warm-container/width-l2_n321003/linux-x86_64-c16-bare-intel/20261114T230831Z-d0161800/report.md)
- [20261121T211650Z-eb17dd15](warm-container/width-l2_n321003/linux-x86_64-c16-bare-intel/20261121T211650Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T104454Z-f763cb0-dirty](warm-container/width-l2_n321003/macos-aarch64-c8-bare-apple/20260807T104454Z-f763cb0-dirty/report.md)

## warm-container/width-l2_n601003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T211650Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 714.11 us | 3.62x |
| warm-container-minimum | 212.30 us | 1.08x |
| warm-container-native | 197.41 us | 1.00x |
| warm-container-plusone | 289.38 us | 1.47x |

- [20261114T231253Z-d0161800](warm-container/width-l2_n601003/linux-x86_64-c16-bare-intel/20261114T231253Z-d0161800/report.md)
- [20261121T211650Z-eb17dd15](warm-container/width-l2_n601003/linux-x86_64-c16-bare-intel/20261121T211650Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T104730Z-f763cb0-dirty](warm-container/width-l2_n601003/macos-aarch64-c8-bare-apple/20260807T104730Z-f763cb0-dirty/report.md)

## warm-container/width-l2_n641003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T211650Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 696.90 us | 3.35x |
| warm-container-minimum | 261.34 us | 1.25x |
| warm-container-native | 208.29 us | 1.00x |
| warm-container-plusone | 757.62 us | 3.64x |

- [20261114T231719Z-d0161800](warm-container/width-l2_n641003/linux-x86_64-c16-bare-intel/20261114T231719Z-d0161800/report.md)
- [20261121T211650Z-eb17dd15](warm-container/width-l2_n641003/linux-x86_64-c16-bare-intel/20261121T211650Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T104952Z-f763cb0-dirty](warm-container/width-l2_n641003/macos-aarch64-c8-bare-apple/20260807T104952Z-f763cb0-dirty/report.md)

## warm-container/width-l2_n81003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T211650Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| warm-container-headroom | 186.59 us | 7.24x |
| warm-container-minimum | 25.78 us | 1.00x |
| warm-container-native | 25.77 us | 1.00x |
| warm-container-plusone | 187.30 us | 7.27x |

- [20261114T225751Z-d0161800](warm-container/width-l2_n81003/linux-x86_64-c16-bare-intel/20261114T225751Z-d0161800/report.md)
- [20261121T211650Z-eb17dd15](warm-container/width-l2_n81003/linux-x86_64-c16-bare-intel/20261121T211650Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260807T103704Z-f763cb0-dirty](warm-container/width-l2_n81003/macos-aarch64-c8-bare-apple/20260807T103704Z-f763cb0-dirty/report.md)

## wide-rung/density-w200_n200001

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T214057Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 4.00 us | 1.19x |
| wide-rung-ragged | 3.36 us | 1.00x |
| wide-rung-ragged-overread | 4.22 us | 1.26x |
| wide-rung-wordround | 4.01 us | 1.20x |
| wide-rung-wordround-alias | 4.03 us | 1.20x |

- [20261114T233500Z-d0161800](wide-rung/density-w200_n200001/linux-x86_64-c16-bare-intel/20261114T233500Z-d0161800/report.md)
- [20261121T214057Z-eb17dd15](wide-rung/density-w200_n200001/linux-x86_64-c16-bare-intel/20261121T214057Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072608Z-3a02ce5-dirty](wide-rung/density-w200_n200001/macos-aarch64-c8-bare-apple/20260808T072608Z-3a02ce5-dirty/report.md)

## wide-rung/density-w200_n200002

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T214057Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 3.65 us | 1.00x |
| wide-rung-ragged | 4.71 us | 1.29x |
| wide-rung-ragged-overread | 4.57 us | 1.25x |
| wide-rung-wordround | 4.31 us | 1.18x |
| wide-rung-wordround-alias | 4.47 us | 1.23x |

- [20261114T233728Z-d0161800](wide-rung/density-w200_n200002/linux-x86_64-c16-bare-intel/20261114T233728Z-d0161800/report.md)
- [20261121T214057Z-eb17dd15](wide-rung/density-w200_n200002/linux-x86_64-c16-bare-intel/20261121T214057Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072628Z-3a02ce5-dirty](wide-rung/density-w200_n200002/macos-aarch64-c8-bare-apple/20260808T072628Z-3a02ce5-dirty/report.md)

## wide-rung/density-w200_n200004

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T214057Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 8.88 us | 1.22x |
| wide-rung-ragged | 7.39 us | 1.02x |
| wide-rung-ragged-overread | 9.32 us | 1.28x |
| wide-rung-wordround | 7.27 us | 1.00x |
| wide-rung-wordround-alias | 8.88 us | 1.22x |

- [20261114T233953Z-d0161800](wide-rung/density-w200_n200004/linux-x86_64-c16-bare-intel/20261114T233953Z-d0161800/report.md)
- [20261121T214057Z-eb17dd15](wide-rung/density-w200_n200004/linux-x86_64-c16-bare-intel/20261121T214057Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072649Z-3a02ce5-dirty](wide-rung/density-w200_n200004/macos-aarch64-c8-bare-apple/20260808T072649Z-3a02ce5-dirty/report.md)

## wide-rung/density-w200_n200008

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T214057Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 12.18 us | 1.00x |
| wide-rung-ragged | 14.85 us | 1.22x |
| wide-rung-ragged-overread | 13.92 us | 1.14x |
| wide-rung-wordround | 14.87 us | 1.22x |
| wide-rung-wordround-alias | 14.87 us | 1.22x |

- [20261114T234218Z-d0161800](wide-rung/density-w200_n200008/linux-x86_64-c16-bare-intel/20261114T234218Z-d0161800/report.md)
- [20261121T214057Z-eb17dd15](wide-rung/density-w200_n200008/linux-x86_64-c16-bare-intel/20261121T214057Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072709Z-3a02ce5-dirty](wide-rung/density-w200_n200008/macos-aarch64-c8-bare-apple/20260808T072709Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l1_n129000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T215100Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 2.29 us | 1.25x |
| wide-rung-ragged | 1.83 us | 1.00x |
| wide-rung-ragged-overread | 2.29 us | 1.25x |
| wide-rung-wordround | 1.89 us | 1.03x |
| wide-rung-wordround-alias | 1.89 us | 1.03x |

- [20261114T234502Z-d0161800](wide-rung/walk-l1_n129000/linux-x86_64-c16-bare-intel/20261114T234502Z-d0161800/report.md)
- [20261121T215100Z-eb17dd15](wide-rung/walk-l1_n129000/linux-x86_64-c16-bare-intel/20261121T215100Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072207Z-3a02ce5-dirty](wide-rung/walk-l1_n129000/macos-aarch64-c8-bare-apple/20260808T072207Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l1_n160000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T215100Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 2.95 us | 1.04x |
| wide-rung-ragged | 3.11 us | 1.10x |
| wide-rung-ragged-overread | 2.91 us | 1.03x |
| wide-rung-wordround | 2.83 us | 1.00x |
| wide-rung-wordround-alias | 3.03 us | 1.07x |

- [20261114T234728Z-d0161800](wide-rung/walk-l1_n160000/linux-x86_64-c16-bare-intel/20261114T234728Z-d0161800/report.md)
- [20261121T215100Z-eb17dd15](wide-rung/walk-l1_n160000/linux-x86_64-c16-bare-intel/20261121T215100Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072227Z-3a02ce5-dirty](wide-rung/walk-l1_n160000/macos-aarch64-c8-bare-apple/20260808T072227Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l1_n192000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T215100Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 3.08 us | 1.29x |
| wide-rung-ragged | 2.67 us | 1.12x |
| wide-rung-ragged-overread | 2.93 us | 1.23x |
| wide-rung-wordround | 2.96 us | 1.24x |
| wide-rung-wordround-alias | 2.39 us | 1.00x |

- [20261114T234951Z-d0161800](wide-rung/walk-l1_n192000/linux-x86_64-c16-bare-intel/20261114T234951Z-d0161800/report.md)
- [20261121T215100Z-eb17dd15](wide-rung/walk-l1_n192000/linux-x86_64-c16-bare-intel/20261121T215100Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072246Z-3a02ce5-dirty](wide-rung/walk-l1_n192000/macos-aarch64-c8-bare-apple/20260808T072246Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l1_n200000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T215100Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.76 us | 1.00x |
| wide-rung-ragged | 1.79 us | 1.02x |
| wide-rung-ragged-overread | 4.01 us | 2.28x |
| wide-rung-wordround | 3.77 us | 2.14x |
| wide-rung-wordround-alias | 1.76 us | 1.00x |

- [20261114T235216Z-d0161800](wide-rung/walk-l1_n200000/linux-x86_64-c16-bare-intel/20261114T235216Z-d0161800/report.md)
- [20261121T215100Z-eb17dd15](wide-rung/walk-l1_n200000/linux-x86_64-c16-bare-intel/20261121T215100Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072306Z-3a02ce5-dirty](wide-rung/walk-l1_n200000/macos-aarch64-c8-bare-apple/20260808T072306Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l1_n232000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T215100Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 2.15 us | 1.37x |
| wide-rung-ragged | 2.21 us | 1.41x |
| wide-rung-ragged-overread | 1.57 us | 1.00x |
| wide-rung-wordround | 1.87 us | 1.19x |
| wide-rung-wordround-alias | 1.86 us | 1.18x |

- [20261114T235441Z-d0161800](wide-rung/walk-l1_n232000/linux-x86_64-c16-bare-intel/20261114T235441Z-d0161800/report.md)
- [20261121T215100Z-eb17dd15](wide-rung/walk-l1_n232000/linux-x86_64-c16-bare-intel/20261121T215100Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072326Z-3a02ce5-dirty](wide-rung/walk-l1_n232000/macos-aarch64-c8-bare-apple/20260808T072326Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l1_n256000

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T215100Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.77 us | 1.00x |
| wide-rung-ragged | 1.77 us | 1.00x |
| wide-rung-ragged-overread | 1.77 us | 1.00x |
| wide-rung-wordround | 2.16 us | 1.22x |
| wide-rung-wordround-alias | 1.77 us | 1.00x |

- [20261114T235704Z-d0161800](wide-rung/walk-l1_n256000/linux-x86_64-c16-bare-intel/20261114T235704Z-d0161800/report.md)
- [20261121T215100Z-eb17dd15](wide-rung/walk-l1_n256000/linux-x86_64-c16-bare-intel/20261121T215100Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072345Z-3a02ce5-dirty](wide-rung/walk-l1_n256000/macos-aarch64-c8-bare-apple/20260808T072345Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l2_n129100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T220600Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 715.28 us | 1.56x |
| wide-rung-ragged | 459.85 us | 1.00x |
| wide-rung-ragged-overread | 551.36 us | 1.20x |
| wide-rung-wordround | 588.88 us | 1.28x |
| wide-rung-wordround-alias | 619.48 us | 1.35x |

- [20261115T000552Z-d0161800](wide-rung/walk-l2_n129100/linux-x86_64-c16-bare-intel/20261115T000552Z-d0161800/report.md)
- [20261121T220600Z-eb17dd15](wide-rung/walk-l2_n129100/linux-x86_64-c16-bare-intel/20261121T220600Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T075306Z-3a02ce5-dirty](wide-rung/walk-l2_n129100/macos-aarch64-c8-bare-apple/20260808T075306Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l2_n160100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T220600Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 536.26 us | 1.83x |
| wide-rung-ragged | 293.29 us | 1.00x |
| wide-rung-ragged-overread | 311.00 us | 1.06x |
| wide-rung-wordround | 363.88 us | 1.24x |
| wide-rung-wordround-alias | 364.33 us | 1.24x |

- [20261115T001416Z-d0161800](wide-rung/walk-l2_n160100/linux-x86_64-c16-bare-intel/20261115T001416Z-d0161800/report.md)
- [20261121T220600Z-eb17dd15](wide-rung/walk-l2_n160100/linux-x86_64-c16-bare-intel/20261121T220600Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T075633Z-3a02ce5-dirty](wide-rung/walk-l2_n160100/macos-aarch64-c8-bare-apple/20260808T075633Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l2_n192100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T220600Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 543.25 us | 1.51x |
| wide-rung-ragged | 383.21 us | 1.07x |
| wide-rung-ragged-overread | 414.29 us | 1.15x |
| wide-rung-wordround | 377.36 us | 1.05x |
| wide-rung-wordround-alias | 358.79 us | 1.00x |

- [20261115T002249Z-d0161800](wide-rung/walk-l2_n192100/linux-x86_64-c16-bare-intel/20261115T002249Z-d0161800/report.md)
- [20261121T220600Z-eb17dd15](wide-rung/walk-l2_n192100/linux-x86_64-c16-bare-intel/20261121T220600Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T075943Z-3a02ce5-dirty](wide-rung/walk-l2_n192100/macos-aarch64-c8-bare-apple/20260808T075943Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l2_n200100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T220600Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 702.72 us | 1.32x |
| wide-rung-ragged | 574.96 us | 1.08x |
| wide-rung-ragged-overread | 532.40 us | 1.00x |
| wide-rung-wordround | 737.14 us | 1.38x |
| wide-rung-wordround-alias | 669.13 us | 1.26x |

- [20261115T003126Z-d0161800](wide-rung/walk-l2_n200100/linux-x86_64-c16-bare-intel/20261115T003126Z-d0161800/report.md)
- [20261121T220600Z-eb17dd15](wide-rung/walk-l2_n200100/linux-x86_64-c16-bare-intel/20261121T220600Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T080240Z-3a02ce5-dirty](wide-rung/walk-l2_n200100/macos-aarch64-c8-bare-apple/20260808T080240Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l2_n232100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T220600Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 699.85 us | 1.09x |
| wide-rung-ragged | 673.18 us | 1.05x |
| wide-rung-ragged-overread | 639.29 us | 1.00x |
| wide-rung-wordround | 703.27 us | 1.10x |
| wide-rung-wordround-alias | 696.10 us | 1.09x |

- [20261115T004002Z-d0161800](wide-rung/walk-l2_n232100/linux-x86_64-c16-bare-intel/20261115T004002Z-d0161800/report.md)
- [20261121T220600Z-eb17dd15](wide-rung/walk-l2_n232100/linux-x86_64-c16-bare-intel/20261121T220600Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T080558Z-3a02ce5-dirty](wide-rung/walk-l2_n232100/macos-aarch64-c8-bare-apple/20260808T080558Z-3a02ce5-dirty/report.md)

## wide-rung/walk-l2_n256100

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T220600Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 752.29 us | 1.07x |
| wide-rung-ragged | 706.97 us | 1.01x |
| wide-rung-ragged-overread | 708.82 us | 1.01x |
| wide-rung-wordround | 705.72 us | 1.00x |
| wide-rung-wordround-alias | 702.69 us | 1.00x |

- [20261115T004852Z-d0161800](wide-rung/walk-l2_n256100/linux-x86_64-c16-bare-intel/20261115T004852Z-d0161800/report.md)
- [20261121T220600Z-eb17dd15](wide-rung/walk-l2_n256100/linux-x86_64-c16-bare-intel/20261121T220600Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T080902Z-3a02ce5-dirty](wide-rung/walk-l2_n256100/macos-aarch64-c8-bare-apple/20260808T080902Z-3a02ce5-dirty/report.md)

## wide-rung/width-l1_n129003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T225724Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 4.21 us | 1.14x |
| wide-rung-ragged | 4.52 us | 1.22x |
| wide-rung-ragged-overread | 4.46 us | 1.20x |
| wide-rung-wordround | 3.70 us | 1.00x |
| wide-rung-wordround-alias | 4.51 us | 1.22x |

- [20261115T005140Z-d0161800](wide-rung/width-l1_n129003/linux-x86_64-c16-bare-intel/20261115T005140Z-d0161800/report.md)
- [20261121T225724Z-eb17dd15](wide-rung/width-l1_n129003/linux-x86_64-c16-bare-intel/20261121T225724Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072405Z-3a02ce5-dirty](wide-rung/width-l1_n129003/macos-aarch64-c8-bare-apple/20260808T072405Z-3a02ce5-dirty/report.md)

## wide-rung/width-l1_n160003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T225724Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 3.96 us | 1.20x |
| wide-rung-ragged | 3.30 us | 1.00x |
| wide-rung-ragged-overread | 4.32 us | 1.31x |
| wide-rung-wordround | 3.32 us | 1.01x |
| wide-rung-wordround-alias | 3.87 us | 1.17x |

- [20261115T005407Z-d0161800](wide-rung/width-l1_n160003/linux-x86_64-c16-bare-intel/20261115T005407Z-d0161800/report.md)
- [20261121T225724Z-eb17dd15](wide-rung/width-l1_n160003/linux-x86_64-c16-bare-intel/20261121T225724Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072426Z-3a02ce5-dirty](wide-rung/width-l1_n160003/macos-aarch64-c8-bare-apple/20260808T072426Z-3a02ce5-dirty/report.md)

## wide-rung/width-l1_n192003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T225724Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 4.11 us | 1.23x |
| wide-rung-ragged | 4.08 us | 1.22x |
| wide-rung-ragged-overread | 3.34 us | 1.00x |
| wide-rung-wordround | 4.07 us | 1.22x |
| wide-rung-wordround-alias | 4.02 us | 1.20x |

- [20261115T005630Z-d0161800](wide-rung/width-l1_n192003/linux-x86_64-c16-bare-intel/20261115T005630Z-d0161800/report.md)
- [20261121T225724Z-eb17dd15](wide-rung/width-l1_n192003/linux-x86_64-c16-bare-intel/20261121T225724Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072445Z-3a02ce5-dirty](wide-rung/width-l1_n192003/macos-aarch64-c8-bare-apple/20260808T072445Z-3a02ce5-dirty/report.md)

## wide-rung/width-l1_n200003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T225724Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 5.59 us | 1.07x |
| wide-rung-ragged | 5.24 us | 1.00x |
| wide-rung-ragged-overread | 5.83 us | 1.11x |
| wide-rung-wordround | 6.04 us | 1.15x |
| wide-rung-wordround-alias | 6.31 us | 1.20x |

- [20261115T005851Z-d0161800](wide-rung/width-l1_n200003/linux-x86_64-c16-bare-intel/20261115T005851Z-d0161800/report.md)
- [20261121T225724Z-eb17dd15](wide-rung/width-l1_n200003/linux-x86_64-c16-bare-intel/20261121T225724Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072507Z-3a02ce5-dirty](wide-rung/width-l1_n200003/macos-aarch64-c8-bare-apple/20260808T072507Z-3a02ce5-dirty/report.md)

## wide-rung/width-l1_n232003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T225724Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 5.35 us | 1.00x |
| wide-rung-ragged | 6.78 us | 1.27x |
| wide-rung-ragged-overread | 6.37 us | 1.19x |
| wide-rung-wordround | 6.46 us | 1.21x |
| wide-rung-wordround-alias | 6.46 us | 1.21x |

- [20261115T010117Z-d0161800](wide-rung/width-l1_n232003/linux-x86_64-c16-bare-intel/20261115T010117Z-d0161800/report.md)
- [20261121T225724Z-eb17dd15](wide-rung/width-l1_n232003/linux-x86_64-c16-bare-intel/20261121T225724Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072527Z-3a02ce5-dirty](wide-rung/width-l1_n232003/macos-aarch64-c8-bare-apple/20260808T072527Z-3a02ce5-dirty/report.md)

## wide-rung/width-l1_n256003

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T225724Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 6.61 us | 1.17x |
| wide-rung-ragged | 5.64 us | 1.00x |
| wide-rung-ragged-overread | 5.64 us | 1.00x |
| wide-rung-wordround | 6.65 us | 1.18x |
| wide-rung-wordround-alias | 6.88 us | 1.22x |

- [20261115T010341Z-d0161800](wide-rung/width-l1_n256003/linux-x86_64-c16-bare-intel/20261115T010341Z-d0161800/report.md)
- [20261121T225724Z-eb17dd15](wide-rung/width-l1_n256003/linux-x86_64-c16-bare-intel/20261121T225724Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T072548Z-3a02ce5-dirty](wide-rung/width-l1_n256003/macos-aarch64-c8-bare-apple/20260808T072548Z-3a02ce5-dirty/report.md)

## wide-rung/width-l2_n129103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T231222Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.06 ms | 1.19x |
| wide-rung-ragged | 1.02 ms | 1.15x |
| wide-rung-ragged-overread | 889.91 us | 1.00x |
| wide-rung-wordround | 1.03 ms | 1.16x |
| wide-rung-wordround-alias | 984.13 us | 1.11x |

- [20261115T011251Z-d0161800](wide-rung/width-l2_n129103/linux-x86_64-c16-bare-intel/20261115T011251Z-d0161800/report.md)
- [20261121T231222Z-eb17dd15](wide-rung/width-l2_n129103/linux-x86_64-c16-bare-intel/20261121T231222Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T073116Z-3a02ce5-dirty](wide-rung/width-l2_n129103/macos-aarch64-c8-bare-apple/20260808T073116Z-3a02ce5-dirty/report.md)

## wide-rung/width-l2_n160103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T231222Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.00 ms | 1.13x |
| wide-rung-ragged | 886.54 us | 1.00x |
| wide-rung-ragged-overread | 942.00 us | 1.06x |
| wide-rung-wordround | 921.38 us | 1.04x |
| wide-rung-wordround-alias | 900.75 us | 1.02x |

- [20261115T012143Z-d0161800](wide-rung/width-l2_n160103/linux-x86_64-c16-bare-intel/20261115T012143Z-d0161800/report.md)
- [20261121T231222Z-eb17dd15](wide-rung/width-l2_n160103/linux-x86_64-c16-bare-intel/20261121T231222Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T073509Z-3a02ce5-dirty](wide-rung/width-l2_n160103/macos-aarch64-c8-bare-apple/20260808T073509Z-3a02ce5-dirty/report.md)

## wide-rung/width-l2_n192103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T231222Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.00 ms | 1.19x |
| wide-rung-ragged | 922.03 us | 1.10x |
| wide-rung-ragged-overread | 898.23 us | 1.07x |
| wide-rung-wordround | 838.42 us | 1.00x |
| wide-rung-wordround-alias | 958.07 us | 1.14x |

- [20261115T013040Z-d0161800](wide-rung/width-l2_n192103/linux-x86_64-c16-bare-intel/20261115T013040Z-d0161800/report.md)
- [20261121T231222Z-eb17dd15](wide-rung/width-l2_n192103/linux-x86_64-c16-bare-intel/20261121T231222Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T073842Z-3a02ce5-dirty](wide-rung/width-l2_n192103/macos-aarch64-c8-bare-apple/20260808T073842Z-3a02ce5-dirty/report.md)

## wide-rung/width-l2_n200103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T231222Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.36 ms | 1.08x |
| wide-rung-ragged | 1.37 ms | 1.09x |
| wide-rung-ragged-overread | 1.35 ms | 1.08x |
| wide-rung-wordround | 1.34 ms | 1.07x |
| wide-rung-wordround-alias | 1.26 ms | 1.00x |

- [20261115T013944Z-d0161800](wide-rung/width-l2_n200103/linux-x86_64-c16-bare-intel/20261115T013944Z-d0161800/report.md)
- [20261121T231222Z-eb17dd15](wide-rung/width-l2_n200103/linux-x86_64-c16-bare-intel/20261121T231222Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T074221Z-3a02ce5-dirty](wide-rung/width-l2_n200103/macos-aarch64-c8-bare-apple/20260808T074221Z-3a02ce5-dirty/report.md)

## wide-rung/width-l2_n232103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T231222Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.42 ms | 1.09x |
| wide-rung-ragged | 1.50 ms | 1.15x |
| wide-rung-ragged-overread | 1.49 ms | 1.14x |
| wide-rung-wordround | 1.31 ms | 1.00x |
| wide-rung-wordround-alias | 1.35 ms | 1.04x |

- [20261115T014855Z-d0161800](wide-rung/width-l2_n232103/linux-x86_64-c16-bare-intel/20261115T014855Z-d0161800/report.md)
- [20261121T231222Z-eb17dd15](wide-rung/width-l2_n232103/linux-x86_64-c16-bare-intel/20261121T231222Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T074618Z-3a02ce5-dirty](wide-rung/width-l2_n232103/macos-aarch64-c8-bare-apple/20260808T074618Z-3a02ce5-dirty/report.md)

## wide-rung/width-l2_n256103

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261121T231222Z-eb17dd15`

| variant | median | vs best |
|---|---|---|
| wide-rung-align16 | 1.50 ms | 1.01x |
| wide-rung-ragged | 1.50 ms | 1.01x |
| wide-rung-ragged-overread | 1.49 ms | 1.00x |
| wide-rung-wordround | 1.50 ms | 1.01x |
| wide-rung-wordround-alias | 1.49 ms | 1.00x |

- [20261115T015814Z-d0161800](wide-rung/width-l2_n256103/linux-x86_64-c16-bare-intel/20261115T015814Z-d0161800/report.md)
- [20261121T231222Z-eb17dd15](wide-rung/width-l2_n256103/linux-x86_64-c16-bare-intel/20261121T231222Z-eb17dd15/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260808T074950Z-3a02ce5-dirty](wide-rung/width-l2_n256103/macos-aarch64-c8-bare-apple/20260808T074950Z-3a02ce5-dirty/report.md)
