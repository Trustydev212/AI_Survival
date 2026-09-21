# Tập quán có tự giữ đất khi không cần thủ lĩnh không?

Tập quán hình thành và lan như thường (mặc định), so với tập quán không bao giờ lên tiếng (--no-customs), và so với tắt cả mệnh lệnh lẫn tập quán.

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **no-customs**: `--no-customs` · 16 thế giới
- **no-orders**: `--no-orders` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | surviving | tốt |
|---|---|---|---|---|---|---|
| default | 4 | 1 | 2 | 9 | 0 | 9/16 |
| no-customs | 5 | 1 | 3 | 6 | 1 | 7/16 |
| no-orders | 2 | 0 | 2 | 12 | 0 | 12/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | no-customs | no-orders |
|---|---|---|---|
| peak_pop | 3624 | 3400 | 3034 |
| final_pop | 914 | 758 | 818 |
| innovations | 124 | 118 | 120 |
| mean_known | 36.74 | 37.27 | 54.74 |
| soil_health | 0.98 | 0.99 | 0.97 |
| lived_soil | 0.96 | 0.99 | 0.96 |
| settled_share | 0.68 | 0.75 | 0.70 |
| obedience | 0.39 | 0.06 | 0.00 |
| breed_rate | 6.42 | 7.10 | 5.88 |
| swing | 3.39 | 3.44 | 2.94 |
| final_level | 6.50 | 6.50 | 7.00 |
| plastic | 0.01 | 0.01 | 0.01 |
| signal_mi | 0.02 | 0.03 | 0.04 |
| signal_meaning | 0.02 | 0.02 | 0.02 |
| things_per_head | 0.47 | 0.26 | 0.56 |
| equipped_share | 0.23 | 0.15 | 0.23 |
| crafts | 93.50 | 85.00 | 89.00 |
| learn_rate | 2.08 | 1.37 | 2.23 |
| loudness | 1.51 | 1.54 | 1.63 |
| hunts | 0.00 | 0.00 | 0.00 |
| division_of_labour | 0.49 | 0.49 | 0.52 |

## no-customs so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 34.94 | [-1554, 1516] | chưa rõ |
| final_pop | -52.25 | [-609, 505] | chưa rõ |
| innovations | -8.62 | [-43.62, 25.06] | chưa rõ |
| mean_known | -9.77 | [-34.19, 14.09] | chưa rõ |
| soil_health | 0.02 | [-0.09, 0.12] | chưa rõ |
| lived_soil | 0.01 | [-0.09, 0.11] | chưa rõ |
| settled_share | -0.03 | [-0.31, 0.23] | chưa rõ |
| obedience | -0.24 | [-0.45, -0.04] | khác 0 |
| breed_rate | -0.73 | [-3.78, 2.28] | chưa rõ |
| swing | -0.47 | [-3.25, 2.06] | chưa rõ |
| final_level | -0.62 | [-2.50, 1.19] | chưa rõ |
| plastic | -0.00 | [-0.01, 0.00] | chưa rõ |
| signal_mi | -0.02 | [-0.04, 0.01] | chưa rõ |
| signal_meaning | -0.00 | [-0.03, 0.03] | chưa rõ |
| things_per_head | -0.14 | [-0.71, 0.45] | chưa rõ |
| equipped_share | -0.05 | [-0.25, 0.16] | chưa rõ |
| crafts | -9.44 | [-41.25, 22.25] | chưa rõ |
| learn_rate | -0.37 | [-1.94, 1.17] | chưa rõ |
| loudness | -0.07 | [-0.48, 0.34] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.04 | [-0.20, 0.11] | chưa rõ |

## no-orders so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 180 | [-1584, 1827] | chưa rõ |
| final_pop | -95.06 | [-616, 390] | chưa rõ |
| innovations | 1.50 | [-31.06, 32.88] | chưa rõ |
| mean_known | 3.78 | [-20.98, 27.11] | chưa rõ |
| soil_health | 0.01 | [-0.10, 0.12] | chưa rõ |
| lived_soil | 0.00 | [-0.10, 0.10] | chưa rõ |
| settled_share | -0.00 | [-0.27, 0.27] | chưa rõ |
| obedience | -0.43 | [-0.60, -0.27] | khác 0 |
| breed_rate | -0.75 | [-3.70, 2.02] | chưa rõ |
| swing | -1.69 | [-4.32, 0.47] | chưa rõ |
| final_level | 0.38 | [-1.31, 2.00] | chưa rõ |
| plastic | 0.00 | [-0.00, 0.01] | chưa rõ |
| signal_mi | -0.00 | [-0.04, 0.03] | chưa rõ |
| signal_meaning | -0.01 | [-0.04, 0.01] | chưa rõ |
| things_per_head | -0.00 | [-0.61, 0.62] | chưa rõ |
| equipped_share | -0.01 | [-0.22, 0.19] | chưa rõ |
| crafts | -2.31 | [-32.88, 27.38] | chưa rõ |
| learn_rate | 0.05 | [-1.41, 1.51] | chưa rõ |
| loudness | 0.07 | [-0.32, 0.45] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.00 | [-0.15, 0.15] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 993 | 1886 | 914 |
| no-customs | 148 | 196 | 608 | 1474 | 758 |
| no-orders | 238 | 777 | 844 | 1714 | 818 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.55 | 68.84 | 45.31 |
| no-customs | 3.37 | 6.88 | 16.39 | 59.48 | 49.11 |
| no-orders | 2.51 | 7.39 | 47.96 | 70.38 | 59.49 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-customs | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-orders | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.04 | 0.06 | 0.04 | 0.03 | 0.03 |
| no-customs | 0.02 | 0.03 | 0.04 | 0.02 | 0.03 |
| no-orders | 0.03 | 0.04 | 0.04 | 0.03 | 0.04 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.03 | 0.05 | 0.03 |
| no-customs | 0.03 | 0.03 | 0.02 | 0.04 | 0.03 |
| no-orders | 0.01 | 0.02 | 0.02 | 0.03 | 0.03 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.76 | 0.92 | 0.97 |
| no-customs | 0.51 | 0.53 | 0.60 | 0.51 | 0.40 |
| no-orders | 0.52 | 0.84 | 1.24 | 0.48 | 0.60 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.97 |
| no-customs | 1.00 | 1.00 | 1.00 | 0.97 | 0.97 |
| no-orders | 1.00 | 0.99 | 0.99 | 0.94 | 0.96 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.68 | 0.75 | 0.69 | 0.76 |
| no-customs | 0.85 | 0.85 | 0.73 | 0.82 | 0.79 |
| no-orders | 0.67 | 0.62 | 0.68 | 0.83 | 0.77 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.24 | 3.05 |
| no-customs | 2.79 | 1.96 | 2.14 | 1.81 | 1.83 |
| no-orders | 2.73 | 2.87 | 2.94 | 2.24 | 2.33 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.58 | 1.55 |
| no-customs | 1.53 | 1.52 | 1.58 | 1.64 | 1.58 |
| no-orders | 1.57 | 1.60 | 1.65 | 1.69 | 1.68 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.72 | 0.55 | 0.54 | 0.50 | 0.51 |
| no-customs | 0.68 | 0.50 | 0.50 | 0.53 | 0.52 |
| no-orders | 0.62 | 0.54 | 0.55 | 0.56 | 0.54 |

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
| no-customs | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-customs | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-customs | 3 | flourishing | 5476 | 3433 | 104 | 0.81 | 0.89 | 0.20 | 0.03 | 0.03 |
| no-customs | 4 | boom and bust | 4860 | 1940 | 20.82 | 0.97 | 0.97 | 0.04 | 0.05 | 0.02 |
| no-customs | 5 | flourishing | 2891 | 1080 | 51.60 | 0.69 | 0.14 | 1.74 | 0.01 | 0.00 |
| no-customs | 6 | collapsed | 6900 | 709 | 33.23 | 0.84 | 0.29 | 1.19 | 0.03 | 0.00 |
| no-customs | 7 | flourishing | 1000 | 664 | 41.32 | 0.95 | 0.01 | 3.06 | 0.02 | 0.06 |
| no-customs | 8 | flourishing | 6831 | 1730 | 108 | 0.61 | 0.79 | 0.65 | 0.03 | 0.00 |
| no-customs | 9 | boom and bust | 5690 | 1008 | 49.11 | 0.99 | 0.84 | 0.31 | 0.04 | 0.04 |
| no-customs | 10 | boom and bust | 1000 | 808 | 7.78 | 1.00 | 0.94 | 0.16 | 0.03 | 0.01 |
| no-customs | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-customs | 12 | boom and bust | 5075 | 1265 | 14.53 | 1.00 | 0.94 | 0.03 | 0.05 | 0.05 |
| no-customs | 13 | boom and bust | 4469 | 613 | 59.57 | 1.00 | 0.77 | 0.40 | 0.04 | 0.02 |
| no-customs | 14 | flourishing | 2884 | 572 | 63.84 | 0.53 | 0.02 | 1.45 | 0.00 | 0.16 |
| no-customs | 15 | surviving | 1000 | 663 | 4.87 | 1.00 | 0.98 | 0.00 | 0.12 | 0.05 |
| no-customs | 16 | flourishing | 3909 | 1171 | 50.78 | 1.00 | 0.74 | 1.53 | 0.02 | 0.13 |
| no-orders | 1 | flourishing | 1000 | 448 | 17.90 | 0.99 | 0.95 | 0.22 | 0.16 | 0.00 |
| no-orders | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-orders | 3 | flourishing | 5919 | 1171 | 95.32 | 0.84 | 0.78 | 0.53 | 0.08 | 0.04 |
| no-orders | 4 | flourishing | 3033 | 2288 | 23.75 | 0.95 | 0.94 | 0.15 | 0.03 | 0.01 |
| no-orders | 5 | flourishing | 3034 | 1323 | 97.78 | 0.51 | 0.36 | 1.19 | 0.00 | 0.01 |
| no-orders | 6 | flourishing | 4556 | 1515 | 82.55 | 0.99 | 0.76 | 0.61 | 0.01 | 0.04 |
| no-orders | 7 | flourishing | 1000 | 712 | 50.69 | 0.96 | 0.02 | 2.98 | 0.04 | 0.07 |
| no-orders | 8 | flourishing | 9560 | 1396 | 60.20 | 0.99 | 0.55 | 0.60 | 0.04 | 0.03 |
| no-orders | 9 | flourishing | 7733 | 1832 | 102 | 0.97 | 0.96 | 0.12 | 0.04 | 0.04 |
| no-orders | 10 | boom and bust | 6095 | 652 | 31.41 | 0.99 | 0.87 | 0.12 | 0.05 | 0.03 |
| no-orders | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| no-orders | 12 | boom and bust | 2225 | 364 | 45.87 | 0.65 | 0.08 | 2.41 | 0.00 | 0.00 |
| no-orders | 13 | flourishing | 3122 | 1381 | 72.32 | 0.91 | 0.65 | 0.72 | 0.04 | 0.04 |
| no-orders | 14 | flourishing | 2612 | 528 | 75.22 | 0.59 | 0.03 | 2.22 | 0.01 | 0.01 |
| no-orders | 15 | flourishing | 1000 | 436 | 12.73 | 1.00 | 0.99 | 0.00 | 0.17 | 0.02 |
| no-orders | 16 | flourishing | 4410 | 925 | 58.79 | 0.96 | 0.87 | 1.08 | 0.03 | 0.11 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
