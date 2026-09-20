# Tập quán có tự giữ đất khi không cần thủ lĩnh không?

Tập quán hình thành và lan như thường (mặc định), so với tập quán không bao giờ lên tiếng (--no-customs), và so với tắt cả mệnh lệnh lẫn tập quán.

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **no-customs**: `--no-customs` · 16 thế giới
- **no-orders**: `--no-orders` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | surviving | tốt |
|---|---|---|---|---|---|---|
| default | 3 | 2 | 3 | 6 | 2 | 8/16 |
| no-customs | 2 | 5 | 5 | 4 | 0 | 4/16 |
| no-orders | 4 | 1 | 6 | 4 | 1 | 5/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | no-customs | no-orders |
|---|---|---|---|
| peak_pop | 1634 | 3522 | 3236 |
| final_pop | 232 | 336 | 886 |
| innovations | 44.50 | 44.50 | 50.50 |
| mean_known | 18.49 | 23.18 | 25.85 |
| soil_health | 1.00 | 1.00 | 1.00 |
| lived_soil | 0.97 | 0.99 | 0.99 |
| settled_share | 0.92 | 0.38 | 0.64 |
| obedience | 0.02 | 0.00 | 0.00 |
| breed_rate | 3.59 | 4.07 | 2.53 |
| swing | 3.89 | 3.80 | 7.95 |
| final_level | 4.50 | 5.00 | 5.50 |
| plastic | 0.00 | 0.00 | 0.01 |
| signal_mi | 0.01 | 0.00 | 0.01 |
| things_per_head | 0.07 | 0.12 | 0.06 |
| equipped_share | 0.06 | 0.06 | 0.04 |
| crafts | 21.50 | 27.00 | 31.50 |
| learn_rate | 1.33 | 1.54 | 1.72 |
| loudness | 1.57 | 1.50 | 1.47 |

## no-customs so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 1216 | [-815, 3243] | chưa rõ |
| final_pop | 305 | [-604, 1446] | chưa rõ |
| innovations | -2.75 | [-35.56, 30.50] | chưa rõ |
| mean_known | -3.36 | [-28.54, 22.59] | chưa rõ |
| soil_health | -0.04 | [-0.12, 0.04] | chưa rõ |
| lived_soil | -0.03 | [-0.11, 0.04] | chưa rõ |
| settled_share | -0.25 | [-0.53, 0.04] | chưa rõ |
| obedience | -0.19 | [-0.39, -0.01] | khác 0 |
| breed_rate | 0.37 | [-2.26, 2.86] | chưa rõ |
| swing | 1.48 | [-3.19, 6.45] | chưa rõ |
| final_level | -0.06 | [-2.06, 1.94] | chưa rõ |
| plastic | -0.00 | [-0.01, 0.00] | chưa rõ |
| signal_mi | -0.01 | [-0.03, 0.01] | chưa rõ |
| things_per_head | 0.35 | [-0.08, 0.88] | chưa rõ |
| equipped_share | 0.09 | [-0.06, 0.26] | chưa rõ |
| crafts | -0.81 | [-29.31, 28.56] | chưa rõ |
| learn_rate | 0.01 | [-1.07, 1.08] | chưa rõ |
| loudness | -0.27 | [-0.75, 0.21] | chưa rõ |

## no-orders so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 835 | [-1531, 3748] | chưa rõ |
| final_pop | 145 | [-455, 722] | chưa rõ |
| innovations | -4.25 | [-35.81, 26.88] | chưa rõ |
| mean_known | -5.30 | [-29.81, 18.66] | chưa rõ |
| soil_health | -0.01 | [-0.07, 0.05] | chưa rõ |
| lived_soil | -0.00 | [-0.06, 0.05] | chưa rõ |
| settled_share | -0.23 | [-0.51, 0.07] | chưa rõ |
| obedience | -0.25 | [-0.45, -0.08] | khác 0 |
| breed_rate | -0.19 | [-2.83, 2.27] | chưa rõ |
| swing | 4.51 | [-0.46, 9.67] | chưa rõ |
| final_level | -0.31 | [-2.50, 1.75] | chưa rõ |
| plastic | 0.00 | [-0.00, 0.01] | chưa rõ |
| signal_mi | 0.00 | [-0.02, 0.02] | chưa rõ |
| things_per_head | 0.16 | [-0.16, 0.55] | chưa rõ |
| equipped_share | 0.03 | [-0.09, 0.15] | chưa rõ |
| crafts | -2.25 | [-28.81, 24.62] | chưa rõ |
| learn_rate | 0.61 | [-0.73, 2.02] | chưa rõ |
| loudness | -0.36 | [-0.87, 0.15] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 180 | 192 | 410 | 722 | 232 |
| no-customs | 153 | 356 | 348 | 1348 | 336 |
| no-orders | 92.50 | 180 | 308 | 900 | 886 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.86 | 7.93 | 14.02 | 31.26 | 34.65 |
| no-customs | 3.50 | 7.91 | 13.67 | 40.27 | 42.33 |
| no-orders | 2.24 | 3.98 | 6.55 | 22.77 | 56.02 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-customs | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| no-orders | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.06 | 0.02 | 0.02 | 0.01 | 0.01 |
| no-customs | 0.07 | 0.03 | 0.02 | 0.02 | 0.01 |
| no-orders | 0.03 | 0.04 | 0.03 | 0.01 | 0.03 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.45 | 0.31 | 0.13 | 0.14 | 0.16 |
| no-customs | 0.37 | 0.36 | 0.44 | 0.33 | 0.26 |
| no-orders | 0.56 | 0.18 | 0.39 | 0.35 | 0.26 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.99 | 0.96 | 1.00 |
| no-customs | 1.00 | 1.00 | 0.99 | 0.97 | 0.98 |
| no-orders | 1.00 | 1.00 | 1.00 | 0.99 | 0.98 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.93 | 0.96 | 0.96 | 0.96 | 0.95 |
| no-customs | 0.93 | 0.91 | 0.91 | 0.92 | 0.90 |
| no-orders | 0.88 | 0.92 | 0.90 | 0.84 | 0.89 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.63 | 2.71 | 1.92 | 1.84 | 1.97 |
| no-customs | 2.48 | 2.31 | 2.15 | 2.30 | 2.22 |
| no-orders | 2.65 | 2.59 | 2.98 | 3.00 | 3.27 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.58 | 1.63 | 1.73 | 1.73 | 1.74 |
| no-customs | 1.59 | 1.67 | 1.58 | 1.50 | 1.58 |
| no-orders | 1.59 | 1.56 | 1.58 | 1.53 | 1.58 |

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
| no-customs | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-customs | 2 | collapsed | 9280 | 495 | 24.84 | 0.99 | 0.89 | 0.15 | 0.06 |
| no-customs | 3 | collapsed | 4834 | 394 | 78.66 | 0.64 | 0.07 | 2.94 | 0.00 |
| no-customs | 4 | collapsed | 7218 | 447 | 53.31 | 0.98 | 0.92 | 0.56 | 0.00 |
| no-customs | 5 | flourishing | 5718 | 1037 | 90.30 | 0.98 | 0.69 | 1.03 | 0.01 |
| no-customs | 6 | boom and bust | 9607 | 3599 | 21.52 | 1.00 | 0.90 | 0.02 | 0.02 |
| no-customs | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-customs | 8 | collapsed | 6105 | 277 | 42.33 | 1.00 | 0.94 | 0.26 | 0.00 |
| no-customs | 9 | collapsed | 1000 | 18.00 | 6.44 | 1.00 | 1.00 | 0.06 | 0.00 |
| no-customs | 10 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-customs | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-customs | 12 | flourishing | 9447 | 7694 | 114 | 0.59 | 0.03 | 1.27 | 0.03 |
| no-customs | 13 | flourishing | 1000 | 241 | 20.26 | 1.00 | 1.00 | 0.15 | 0.03 |
| no-customs | 14 | flourishing | 2210 | 717 | 70.82 | 0.64 | 0.02 | 2.87 | 0.04 |
| no-customs | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-customs | 16 | boom and bust | 7471 | 1203 | 28.96 | 0.98 | 0.92 | 0.09 | 0.01 |
| no-orders | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-orders | 2 | boom and bust | 2839 | 2577 | 51.78 | 0.99 | 0.72 | 0.70 | 0.03 |
| no-orders | 3 | flourishing | 3633 | 1846 | 93.65 | 0.70 | 0.17 | 2.47 | 0.02 |
| no-orders | 4 | boom and bust | 4583 | 1842 | 60.70 | 0.99 | 0.93 | 0.58 | 0.10 |
| no-orders | 5 | flourishing | 3851 | 1083 | 88.46 | 0.72 | 0.55 | 1.70 | 0.00 |
| no-orders | 6 | surviving | 1000 | 85.00 | 4.61 | 1.00 | 0.99 | 0.05 | 0.03 |
| no-orders | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-orders | 8 | collapsed | 21094 | 1069 | 60.26 | 1.00 | 0.76 | 0.28 | 0.01 |
| no-orders | 9 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-orders | 10 | boom and bust | 4751 | 1195 | 22.50 | 1.00 | 0.87 | 0.07 | 0.07 |
| no-orders | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-orders | 12 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-orders | 13 | flourishing | 4220 | 1507 | 29.21 | 0.97 | 0.93 | 0.04 | 0.00 |
| no-orders | 14 | flourishing | 4967 | 1662 | 74.63 | 0.97 | 0.92 | 0.22 | 0.02 |
| no-orders | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| no-orders | 16 | boom and bust | 5855 | 704 | 34.92 | 0.79 | 0.96 | 0.25 | 0.06 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
