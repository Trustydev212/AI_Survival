# Chế tác có đáng cái giá của nó không?

Não có hành động chế tác và vật lý vật liệu (mặc định), so với không ai làm được gì (--no-crafting: hành động chế tác thành nghỉ).

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **no-crafting**: `--no-crafting` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | surviving | tốt |
|---|---|---|---|---|---|---|
| default | 4 | 1 | 2 | 9 | 0 | 9/16 |
| no-crafting | 5 | 2 | 2 | 5 | 2 | 7/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | no-crafting |
|---|---|---|
| peak_pop | 3624 | 4516 |
| final_pop | 914 | 1147 |
| innovations | 124 | 25.00 |
| mean_known | 36.74 | 12.83 |
| soil_health | 0.98 | 1.00 |
| lived_soil | 0.96 | 0.99 |
| settled_share | 0.68 | 0.93 |
| obedience | 0.39 | 0.18 |
| breed_rate | 6.42 | 4.13 |
| swing | 3.39 | 4.50 |
| final_level | 6.50 | 3.00 |
| plastic | 0.01 | 0.01 |
| signal_mi | 0.02 | 0.03 |
| signal_meaning | 0.02 | 0.02 |
| things_per_head | 0.47 | 0.00 |
| equipped_share | 0.23 | 0.00 |
| crafts | 93.50 | 0.00 |
| learn_rate | 2.08 | 1.91 |
| loudness | 1.51 | 1.53 |
| hunts | 0.00 | 0.00 |
| division_of_labour | 0.49 | 0.64 |

## no-crafting so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 2204 | [-157, 4612] | chưa rõ |
| final_pop | 124 | [-473, 700] | chưa rõ |
| innovations | -62.81 | [-87.12, -37.75] | khác 0 |
| mean_known | -35.22 | [-54.31, -17.42] | khác 0 |
| soil_health | 0.08 | [0.00, 0.17] | khác 0 |
| lived_soil | 0.07 | [0.00, 0.15] | khác 0 |
| settled_share | 0.22 | [-0.03, 0.46] | chưa rõ |
| obedience | -0.06 | [-0.31, 0.20] | chưa rõ |
| breed_rate | -2.63 | [-5.43, 0.10] | chưa rõ |
| swing | 1.04 | [-2.26, 4.33] | chưa rõ |
| final_level | -2.19 | [-3.62, -0.62] | khác 0 |
| plastic | -0.00 | [-0.01, 0.00] | chưa rõ |
| signal_mi | -0.01 | [-0.04, 0.02] | chưa rõ |
| signal_meaning | -0.01 | [-0.04, 0.01] | chưa rõ |
| things_per_head | -0.81 | [-1.25, -0.42] | khác 0 |
| equipped_share | -0.32 | [-0.46, -0.18] | khác 0 |
| crafts | -76.31 | [-98.75, -53.31] | khác 0 |
| learn_rate | -0.20 | [-1.55, 1.11] | chưa rõ |
| loudness | 0.00 | [-0.37, 0.37] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | 0.09 | [-0.07, 0.24] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 993 | 1886 | 914 |
| no-crafting | 327 | 1068 | 1266 | 2557 | 1147 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.55 | 68.84 | 45.31 |
| no-crafting | 0.00 | 0.08 | 3.35 | 8.54 | 15.07 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-crafting | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.04 | 0.06 | 0.04 | 0.03 | 0.03 |
| no-crafting | 0.07 | 0.04 | 0.06 | 0.05 | 0.04 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.03 | 0.05 | 0.03 |
| no-crafting | 0.05 | 0.04 | 0.03 | 0.03 | 0.02 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.76 | 0.92 | 0.97 |
| no-crafting | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.97 |
| no-crafting | 1.00 | 0.99 | 0.99 | 0.99 | 0.99 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.68 | 0.75 | 0.69 | 0.76 |
| no-crafting | 0.87 | 0.81 | 0.92 | 0.91 | 0.95 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.24 | 3.05 |
| no-crafting | 3.23 | 2.60 | 2.11 | 1.97 | 2.29 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.58 | 1.55 |
| no-crafting | 1.57 | 1.53 | 1.52 | 1.47 | 1.55 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.72 | 0.55 | 0.54 | 0.50 | 0.51 |
| no-crafting | 0.65 | 0.58 | 0.60 | 0.60 | 0.66 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | hiểu (bit) | nghĩa (bit) |
|---|---|---|---|---|---|---|---|---|---|---|
| default | 1 | flourishing | 1465 | 906 | 42.78 | 0.96 | 0.26 | 1.28 | 0.07 | 0.02 |
| default | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 3 | flourishing | 3777 | 1682 | 114 | 0.61 | 0.66 | 1.19 | 0.02 | 0.08 |
| default | 4 | flourishing | 4330 | 1037 | 22.86 | 0.98 | 0.96 | 0.03 | 0.00 | 0.08 |
| default | 5 | flourishing | 3491 | 1235 | 83.14 | 0.67 | 0.30 | 1.31 | 0.02 | 0.02 |
| default | 6 | boom and bust | 4115 | 367 | 30.71 | 0.63 | 0.19 | 1.41 | 0.14 | 0.00 |
| default | 7 | flourishing | 1000 | 562 | 28.90 | 0.98 | 0.03 | 2.69 | 0.07 | 0.04 |
| default | 8 | flourishing | 8628 | 3100 | 120 | 0.95 | 0.97 | 0.10 | 0.03 | 0.00 |
| default | 9 | collapsed | 8108 | 248 | 47.85 | 0.98 | 0.90 | 0.18 | 0.00 | 0.11 |
| default | 10 | boom and bust | 4472 | 1951 | 16.52 | 0.99 | 0.96 | 0.17 | 0.10 | 0.03 |
| default | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 12 | flourishing | 1858 | 1105 | 19.02 | 0.98 | 0.94 | 0.10 | 0.02 | 0.00 |
| default | 13 | boom and bust | 3948 | 922 | 70.64 | 1.00 | 0.82 | 0.75 | 0.00 | 0.00 |
| default | 14 | flourishing | 2346 | 800 | 69.78 | 0.53 | 0.14 | 1.59 | 0.12 | 0.01 |
| default | 15 | flourishing | 1132 | 558 | 14.63 | 1.00 | 0.99 | 0.00 | 0.13 | 0.06 |
| default | 16 | boom and bust | 3756 | 2019 | 86.07 | 0.87 | 0.70 | 2.19 | 0.02 | 0.16 |
| no-crafting | 1 | surviving | 1000 | 308 | 5.28 | 1.00 | 0.99 | 0.00 | 0.00 | 0.01 |
| no-crafting | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-crafting | 3 | flourishing | 5245 | 1042 | 16.48 | 0.99 | 0.69 | 0.00 | 0.06 | 0.02 |
| no-crafting | 4 | flourishing | 6088 | 1481 | 13.84 | 0.98 | 0.98 | 0.00 | 0.04 | 0.05 |
| no-crafting | 5 | boom and bust | 4090 | 1695 | 4.32 | 0.87 | 0.61 | 0.00 | 0.03 | 0.02 |
| no-crafting | 6 | collapsed | 10903 | 875 | 21.46 | 1.00 | 0.91 | 0.00 | 0.00 | 0.00 |
| no-crafting | 7 | boom and bust | 4436 | 652 | 26.17 | 0.90 | 0.95 | 0.00 | 0.00 | 0.03 |
| no-crafting | 8 | boom and bust | 9461 | 1252 | 20.49 | 1.00 | 0.95 | 0.00 | 0.00 | 0.01 |
| no-crafting | 9 | collapsed | 16577 | 1400 | 24.72 | 0.97 | 0.95 | 0.00 | 0.02 | 0.02 |
| no-crafting | 10 | boom and bust | 4392 | 3528 | 3.11 | 0.98 | 0.70 | 0.00 | 0.07 | 0.01 |
| no-crafting | 11 | boom and bust | 2885 | 503 | 9.75 | 0.91 | 0.79 | 0.00 | 0.09 | 0.00 |
| no-crafting | 12 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-crafting | 13 | flourishing | 9043 | 2525 | 21.28 | 1.00 | 0.97 | 0.00 | 0.09 | 0.07 |
| no-crafting | 14 | flourishing | 4595 | 1504 | 11.82 | 0.78 | 0.85 | 0.00 | 0.04 | 0.02 |
| no-crafting | 15 | surviving | 1000 | 355 | 7.78 | 1.00 | 0.99 | 0.00 | 0.09 | 0.06 |
| no-crafting | 16 | flourishing | 7982 | 1359 | 16.30 | 1.00 | 0.97 | 0.00 | 0.04 | 0.10 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
