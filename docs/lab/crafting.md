# Chế tác có đáng cái giá của nó không?

Não có hành động chế tác và vật lý vật liệu (mặc định), so với không ai làm được gì (--no-crafting: hành động chế tác thành nghỉ).

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **no-crafting**: `--no-crafting` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | surviving | tốt |
|---|---|---|---|---|---|---|
| default | 3 | 2 | 3 | 6 | 2 | 8/16 |
| no-crafting | 7 | 1 | 2 | 2 | 4 | 6/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | no-crafting |
|---|---|---|
| peak_pop | 1634 | 3955 |
| final_pop | 232 | 580 |
| innovations | 44.50 | 26.50 |
| mean_known | 18.49 | 10.71 |
| soil_health | 1.00 | 1.00 |
| lived_soil | 0.97 | 0.99 |
| settled_share | 0.92 | 0.89 |
| obedience | 0.02 | 0.29 |
| breed_rate | 3.59 | 3.30 |
| swing | 3.89 | 4.32 |
| final_level | 4.50 | 3.00 |
| plastic | 0.00 | 0.01 |
| signal_mi | 0.01 | 0.03 |
| things_per_head | 0.07 | 0.00 |
| equipped_share | 0.06 | 0.00 |
| crafts | 21.50 | 0.00 |
| learn_rate | 1.33 | 1.67 |
| loudness | 1.57 | 1.55 |

## no-crafting so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 1337 | [-720, 3495] | chưa rõ |
| final_pop | 4.69 | [-514, 482] | chưa rõ |
| innovations | -36.94 | [-62.75, -12.75] | khác 0 |
| mean_known | -27.14 | [-46.49, -8.83] | khác 0 |
| soil_health | 0.01 | [-0.05, 0.06] | chưa rõ |
| lived_soil | 0.02 | [-0.04, 0.07] | chưa rõ |
| settled_share | 0.05 | [-0.21, 0.31] | chưa rõ |
| obedience | 0.12 | [-0.14, 0.37] | chưa rõ |
| breed_rate | -0.31 | [-2.40, 1.49] | chưa rõ |
| swing | 0.61 | [-3.28, 4.57] | chưa rõ |
| final_level | -1.44 | [-3.06, 0.12] | chưa rõ |
| plastic | 0.00 | [-0.00, 0.01] | chưa rõ |
| signal_mi | 0.02 | [-0.01, 0.04] | chưa rõ |
| things_per_head | -0.23 | [-0.39, -0.10] | khác 0 |
| equipped_share | -0.13 | [-0.21, -0.06] | khác 0 |
| crafts | -50.81 | [-71.00, -31.75] | khác 0 |
| learn_rate | 0.71 | [-0.42, 1.91] | chưa rõ |
| loudness | -0.01 | [-0.43, 0.41] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 180 | 192 | 410 | 722 | 232 |
| no-crafting | 175 | 605 | 1456 | 1473 | 580 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.86 | 7.93 | 14.02 | 31.26 | 34.65 |
| no-crafting | 0.00 | 0.00 | 3.32 | 11.58 | 10.79 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-crafting | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.06 | 0.02 | 0.02 | 0.01 | 0.01 |
| no-crafting | 0.08 | 0.05 | 0.04 | 0.03 | 0.03 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.45 | 0.31 | 0.13 | 0.14 | 0.16 |
| no-crafting | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.99 | 0.96 | 1.00 |
| no-crafting | 1.00 | 1.00 | 1.00 | 0.99 | 1.00 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.93 | 0.96 | 0.96 | 0.96 | 0.95 |
| no-crafting | 0.84 | 0.83 | 0.92 | 0.94 | 0.91 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.63 | 2.71 | 1.92 | 1.84 | 1.97 |
| no-crafting | 2.65 | 2.46 | 2.34 | 2.08 | 1.94 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.58 | 1.63 | 1.73 | 1.73 | 1.74 |
| no-crafting | 1.57 | 1.53 | 1.52 | 1.52 | 1.59 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | MI tín hiệu |
|---|---|---|---|---|---|---|---|---|---|
| default | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| default | 2 | boom and bust | 1000 | 194 | 7.46 | 1.00 | 0.97 | 0.87 | 0.11 |
| default | 3 | collapsed | 1000 | 31.00 | 16.16 | 1.00 | 0.97 | 0.07 | 0.00 |
| default | 4 | boom and bust | 6532 | 822 | 93.96 | 0.94 | 0.93 | 0.48 | 0.01 |
| default | 5 | flourishing | 3632 | 1651 | 94.45 | 0.96 | 0.82 | 0.46 | 0.03 |
| default | 6 | surviving | 1000 | 52.00 | 8.81 | 1.00 | 1.00 | 0.00 | 0.00 |
| default | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| default | 8 | flourishing | 7879 | 1668 | 49.16 | 0.98 | 0.88 | 0.07 | 0.00 |
| default | 9 | collapsed | 5964 | 269 | 20.81 | 1.00 | 0.99 | 0.06 | 0.00 |
| default | 10 | surviving | 1000 | 47.00 | 0.13 | 1.00 | 0.98 | 0.00 | 0.09 |
| default | 11 | boom and bust | 4361 | 1194 | 34.65 | 0.96 | 0.88 | 0.40 | 0.02 |
| default | 12 | flourishing | 2269 | 1076 | 75.02 | 0.74 | 0.18 | 0.90 | 0.01 |
| default | 13 | flourishing | 1000 | 100 | 13.99 | 1.00 | 0.98 | 0.05 | 0.03 |
| default | 14 | flourishing | 6849 | 3102 | 105 | 0.79 | 0.91 | 0.16 | 0.01 |
| default | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| default | 16 | flourishing | 3950 | 1043 | 86.25 | 1.00 | 0.95 | 0.23 | 0.04 |
| no-crafting | 1 | surviving | 1000 | 653 | 3.56 | 0.99 | 0.99 | 0.00 | 0.02 |
| no-crafting | 2 | boom and bust | 12701 | 786 | 17.31 | 1.00 | 0.94 | 0.00 | 0.07 |
| no-crafting | 3 | boom and bust | 4794 | 557 | 10.70 | 0.63 | 0.83 | 0.00 | 0.04 |
| no-crafting | 4 | collapsed | 7158 | 378 | 20.97 | 0.99 | 0.94 | 0.00 | 0.10 |
| no-crafting | 5 | boom and bust | 8916 | 1319 | 16.22 | 1.00 | 0.90 | 0.00 | 0.07 |
| no-crafting | 6 | surviving | 1000 | 125 | 2.34 | 1.00 | 1.00 | 0.00 | 0.10 |
| no-crafting | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-crafting | 8 | boom and bust | 7083 | 603 | 10.85 | 1.00 | 0.86 | 0.00 | 0.00 |
| no-crafting | 9 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-crafting | 10 | surviving | 1000 | 435 | 5.64 | 1.00 | 0.98 | 0.00 | 0.00 |
| no-crafting | 11 | flourishing | 2075 | 472 | 10.73 | 1.00 | 1.00 | 0.00 | 0.03 |
| no-crafting | 12 | boom and bust | 1351 | 696 | 4.47 | 0.99 | 0.41 | 0.00 | 0.13 |
| no-crafting | 13 | flourishing | 4563 | 1154 | 22.80 | 0.95 | 0.88 | 0.00 | 0.04 |
| no-crafting | 14 | boom and bust | 7592 | 1461 | 15.39 | 1.00 | 0.91 | 0.00 | 0.01 |
| no-crafting | 15 | surviving | 3347 | 2175 | 9.22 | 0.96 | 0.82 | 0.00 | 0.03 |
| no-crafting | 16 | boom and bust | 6245 | 510 | 21.11 | 1.00 | 0.80 | 0.00 | 0.00 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
