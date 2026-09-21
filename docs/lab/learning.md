# Học trong đời có đáng không?

Não có phần dẻo học theo phần thưởng (mặc định), so với tắt hẳn học trong đời và với học nhanh gấp ba.

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **no-learning**: `--learn-scale 0` · 16 thế giới
- **fast-learning**: `--learn-scale 3` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | fallen | flourishing | flourishing on dying land | surviving | tốt |
|---|---|---|---|---|---|---|---|---|
| default | 4 | 1 | 2 | 0 | 9 | 0 | 0 | 9/16 |
| no-learning | 5 | 2 | 0 | 1 | 7 | 0 | 1 | 8/16 |
| fast-learning | 7 | 2 | 0 | 0 | 6 | 1 | 0 | 6/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | no-learning | fast-learning |
|---|---|---|---|
| peak_pop | 3624 | 3943 | 3730 |
| final_pop | 914 | 986 | 1062 |
| innovations | 124 | 121 | 128 |
| mean_known | 36.74 | 62.50 | 52.63 |
| soil_health | 0.98 | 0.94 | 0.96 |
| lived_soil | 0.96 | 0.96 | 0.95 |
| settled_share | 0.68 | 0.89 | 0.84 |
| obedience | 0.39 | 0.52 | 0.25 |
| breed_rate | 6.42 | 5.38 | 6.97 |
| swing | 3.39 | 3.06 | 3.82 |
| final_level | 6.50 | 7.00 | 7.00 |
| plastic | 0.01 | 0.00 | 0.05 |
| signal_mi | 0.02 | 0.04 | 0.04 |
| things_per_head | 0.47 | 0.53 | 0.34 |
| equipped_share | 0.23 | 0.24 | 0.16 |
| crafts | 93.50 | 94.00 | 103 |
| learn_rate | 2.08 | 3.07 | 3.24 |
| loudness | 1.51 | 1.56 | 1.62 |
| hunts | 0.00 | 0.00 | 0.00 |

## no-learning so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 594 | [-1125, 2290] | chưa rõ |
| final_pop | -51.38 | [-566, 409] | chưa rõ |
| innovations | 2.75 | [-29.50, 33.19] | chưa rõ |
| mean_known | 9.66 | [-16.75, 33.37] | chưa rõ |
| soil_health | 0.01 | [-0.09, 0.12] | chưa rõ |
| lived_soil | 0.01 | [-0.08, 0.10] | chưa rõ |
| settled_share | 0.17 | [-0.09, 0.42] | chưa rõ |
| obedience | 0.04 | [-0.17, 0.24] | chưa rõ |
| breed_rate | -0.40 | [-3.63, 2.84] | chưa rõ |
| swing | -1.48 | [-4.10, 0.59] | chưa rõ |
| final_level | 0.62 | [-0.94, 2.12] | chưa rõ |
| plastic | -0.01 | [-0.01, -0.01] | khác 0 |
| signal_mi | 0.01 | [-0.03, 0.05] | chưa rõ |
| things_per_head | 0.02 | [-0.56, 0.60] | chưa rõ |
| equipped_share | 0.01 | [-0.18, 0.20] | chưa rõ |
| crafts | 0.12 | [-30.69, 29.75] | chưa rõ |
| learn_rate | 0.32 | [-1.00, 1.54] | chưa rõ |
| loudness | 0.23 | [-0.02, 0.52] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |

## fast-learning so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 1426 | [-524, 3320] | chưa rõ |
| final_pop | 268 | [-311, 848] | chưa rõ |
| innovations | 12.31 | [-18.62, 42.06] | chưa rõ |
| mean_known | 14.08 | [-11.22, 37.70] | chưa rõ |
| soil_health | -0.02 | [-0.15, 0.10] | chưa rõ |
| lived_soil | -0.03 | [-0.14, 0.09] | chưa rõ |
| settled_share | 0.10 | [-0.17, 0.36] | chưa rõ |
| obedience | -0.06 | [-0.30, 0.19] | chưa rõ |
| breed_rate | 0.74 | [-2.25, 3.58] | chưa rõ |
| swing | -1.13 | [-3.74, 0.88] | chưa rõ |
| final_level | 1.19 | [-0.19, 2.62] | chưa rõ |
| plastic | 0.04 | [0.02, 0.05] | khác 0 |
| signal_mi | -0.00 | [-0.03, 0.02] | chưa rõ |
| things_per_head | 0.01 | [-0.63, 0.66] | chưa rõ |
| equipped_share | 0.00 | [-0.21, 0.21] | chưa rõ |
| crafts | 7.88 | [-22.25, 37.25] | chưa rõ |
| learn_rate | 0.88 | [-0.68, 2.41] | chưa rõ |
| loudness | 0.25 | [-0.00, 0.55] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 993 | 1886 | 914 |
| no-learning | 357 | 1408 | 1059 | 1957 | 986 |
| fast-learning | 520 | 1172 | 1004 | 1774 | 1062 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.55 | 68.84 | 45.31 |
| no-learning | 4.22 | 11.96 | 43.46 | 68.96 | 62.50 |
| fast-learning | 3.46 | 10.04 | 43.49 | 73.22 | 52.63 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-learning | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| fast-learning | 0.03 | 0.03 | 0.04 | 0.05 | 0.05 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.04 | 0.06 | 0.04 | 0.03 | 0.03 |
| no-learning | 0.06 | 0.07 | 0.05 | 0.03 | 0.04 |
| fast-learning | 0.03 | 0.04 | 0.04 | 0.04 | 0.04 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.76 | 0.92 | 0.97 |
| no-learning | 0.64 | 1.13 | 0.97 | 0.56 | 0.53 |
| fast-learning | 0.62 | 0.50 | 0.77 | 0.38 | 0.34 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.97 |
| no-learning | 1.00 | 0.99 | 0.95 | 0.93 | 0.94 |
| fast-learning | 1.00 | 0.99 | 0.99 | 0.97 | 0.96 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.68 | 0.75 | 0.69 | 0.76 |
| no-learning | 0.54 | 0.42 | 0.72 | 0.80 | 0.89 |
| fast-learning | 0.76 | 0.51 | 0.69 | 0.80 | 0.84 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.24 | 3.05 |
| no-learning | 2.93 | 2.70 | 2.88 | 2.86 | 3.07 |
| fast-learning | 2.91 | 2.53 | 2.36 | 3.27 | 3.24 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.58 | 1.55 |
| no-learning | 1.52 | 1.50 | 1.52 | 1.54 | 1.56 |
| fast-learning | 1.54 | 1.56 | 1.54 | 1.61 | 1.62 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | MI tín hiệu |
|---|---|---|---|---|---|---|---|---|---|
| default | 1 | flourishing | 1465 | 906 | 42.78 | 0.96 | 0.26 | 1.28 | 0.07 |
| default | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| default | 3 | flourishing | 3777 | 1682 | 114 | 0.61 | 0.66 | 1.19 | 0.02 |
| default | 4 | flourishing | 4330 | 1037 | 22.86 | 0.98 | 0.96 | 0.03 | 0.00 |
| default | 5 | flourishing | 3491 | 1235 | 83.14 | 0.67 | 0.30 | 1.31 | 0.02 |
| default | 6 | boom and bust | 4115 | 367 | 30.71 | 0.63 | 0.19 | 1.41 | 0.14 |
| default | 7 | flourishing | 1000 | 562 | 28.90 | 0.98 | 0.03 | 2.69 | 0.07 |
| default | 8 | flourishing | 8628 | 3100 | 120 | 0.95 | 0.97 | 0.10 | 0.03 |
| default | 9 | collapsed | 8108 | 248 | 47.85 | 0.98 | 0.90 | 0.18 | 0.00 |
| default | 10 | boom and bust | 4472 | 1951 | 16.52 | 0.99 | 0.96 | 0.17 | 0.10 |
| default | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| default | 12 | flourishing | 1858 | 1105 | 19.02 | 0.98 | 0.94 | 0.10 | 0.02 |
| default | 13 | boom and bust | 3948 | 922 | 70.64 | 1.00 | 0.82 | 0.75 | 0.00 |
| default | 14 | flourishing | 2346 | 800 | 69.78 | 0.53 | 0.14 | 1.59 | 0.12 |
| default | 15 | flourishing | 1132 | 558 | 14.63 | 1.00 | 0.99 | 0.00 | 0.13 |
| default | 16 | boom and bust | 3756 | 2019 | 86.07 | 0.87 | 0.70 | 2.19 | 0.02 |
| no-learning | 1 | surviving | 1000 | 254 | 8.86 | 1.00 | 0.92 | 0.95 | 0.00 |
| no-learning | 2 | collapsed | 5533 | 330 | 48.03 | 0.73 | 0.03 | 2.58 | 0.04 |
| no-learning | 3 | flourishing | 2930 | 1558 | 91.19 | 0.76 | 0.72 | 0.52 | 0.02 |
| no-learning | 4 | fallen | 1000 | 337 | 4.98 | 1.00 | 0.92 | 0.16 | 0.05 |
| no-learning | 5 | boom and bust | 5975 | 1755 | 92.80 | 0.84 | 0.77 | 0.54 | 0.01 |
| no-learning | 6 | collapsed | 10565 | 1063 | 55.85 | 1.00 | 0.91 | 0.11 | 0.01 |
| no-learning | 7 | boom and bust | 1000 | 74.00 | 12.54 | 1.00 | 0.99 | 0.42 | 0.08 |
| no-learning | 8 | boom and bust | 5966 | 1918 | 103 | 0.96 | 0.77 | 0.90 | 0.01 |
| no-learning | 9 | boom and bust | 5559 | 954 | 72.91 | 0.98 | 0.89 | 0.26 | 0.02 |
| no-learning | 10 | boom and bust | 4838 | 1019 | 25.06 | 0.93 | 0.88 | 0.28 | 0.05 |
| no-learning | 11 | flourishing | 5293 | 1984 | 91.18 | 1.00 | 0.93 | 0.23 | 0.03 |
| no-learning | 12 | flourishing | 1905 | 779 | 42.41 | 0.63 | 0.01 | 2.85 | 0.06 |
| no-learning | 13 | flourishing | 3893 | 1105 | 92.53 | 0.93 | 0.79 | 0.83 | 0.06 |
| no-learning | 14 | flourishing | 2647 | 840 | 69.15 | 0.64 | 0.02 | 1.73 | 0.02 |
| no-learning | 15 | flourishing | 1837 | 239 | 17.03 | 1.00 | 0.99 | 0.01 | 0.28 |
| no-learning | 16 | flourishing | 3993 | 1461 | 93.00 | 0.90 | 0.92 | 0.99 | 0.09 |
| fast-learning | 1 | boom and bust | 3927 | 1627 | 37.44 | 0.98 | 0.93 | 0.08 | 0.02 |
| fast-learning | 2 | boom and bust | 1023 | 419 | 5.49 | 1.00 | 0.94 | 0.00 | 0.10 |
| fast-learning | 3 | collapsed | 3286 | 275 | 31.16 | 0.77 | 0.04 | 2.13 | 0.01 |
| fast-learning | 4 | boom and bust | 8787 | 1176 | 42.11 | 1.00 | 0.94 | 0.09 | 0.04 |
| fast-learning | 5 | boom and bust | 5591 | 1928 | 98.25 | 0.87 | 0.79 | 0.40 | 0.01 |
| fast-learning | 6 | flourishing | 1974 | 1375 | 86.38 | 1.00 | 0.86 | 0.25 | 0.03 |
| fast-learning | 7 | boom and bust | 3123 | 2371 | 95.89 | 0.70 | 0.00 | 2.72 | 0.00 |
| fast-learning | 8 | flourishing | 7988 | 2234 | 96.67 | 1.00 | 0.89 | 0.29 | 0.03 |
| fast-learning | 9 | boom and bust | 11703 | 3666 | 106 | 1.00 | 0.94 | 0.20 | 0.02 |
| fast-learning | 10 | collapsed | 10739 | 930 | 35.30 | 1.00 | 0.81 | 0.08 | 0.04 |
| fast-learning | 11 | boom and bust | 3534 | 1168 | 96.31 | 0.70 | 0.60 | 0.49 | 0.05 |
| fast-learning | 12 | flourishing on dying land | 2687 | 932 | 44.64 | 0.37 | 0.00 | 2.97 | 0.11 |
| fast-learning | 13 | flourishing | 4124 | 956 | 54.44 | 0.94 | 0.70 | 0.40 | 0.01 |
| fast-learning | 14 | flourishing | 1665 | 608 | 50.82 | 0.54 | 0.10 | 1.43 | 0.04 |
| fast-learning | 15 | flourishing | 1634 | 331 | 13.22 | 1.00 | 0.99 | 0.00 | 0.08 |
| fast-learning | 16 | flourishing | 5449 | 787 | 97.56 | 0.91 | 0.89 | 1.71 | 0.05 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
