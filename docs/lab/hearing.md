# Nghe nhau có ích gì?

Tín hiệu tốn năng lượng và chỉ họ hàng nghe rõ (mặc định), so với mọi người điếc (đầu vào tín hiệu bằng 0), và so với nghe cả người lạ.

_Lưu ý: lúc chạy báo cáo này, nhánh `default` là cấu hình "chỉ họ hàng nghe rõ" (`--hear-strangers 0`). Vì kết quả dưới đây, mặc định của sim đã đổi thành nghe cả người lạ; nhánh `hear-strangers` ở đây chính là mặc định hiện nay._

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **deaf**: `--hear-scale 0` · 16 thế giới
- **hear-strangers**: `--hear-strangers 1` · 16 thế giới

## Kết cục

| nhánh | boom and bust | boom and bust on dying land | collapsed | extinct | flourishing | surviving | tốt |
|---|---|---|---|---|---|---|---|
| default | 3 | 0 | 2 | 3 | 6 | 2 | 8/16 |
| deaf | 5 | 0 | 0 | 5 | 6 | 0 | 6/16 |
| hear-strangers | 4 | 1 | 2 | 2 | 7 | 0 | 7/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | deaf | hear-strangers |
|---|---|---|---|
| peak_pop | 1634 | 1720 | 4103 |
| final_pop | 232 | 446 | 1048 |
| innovations | 44.50 | 45.50 | 104 |
| mean_known | 18.49 | 25.93 | 53.53 |
| soil_health | 1.00 | 0.99 | 0.97 |
| lived_soil | 0.97 | 0.98 | 0.97 |
| settled_share | 0.92 | 0.82 | 0.75 |
| obedience | 0.02 | 0.04 | 0.19 |
| breed_rate | 3.59 | 3.38 | 4.84 |
| swing | 3.89 | 3.52 | 4.19 |
| final_level | 4.50 | 5.50 | 7.00 |
| plastic | 0.00 | 0.01 | 0.01 |
| signal_mi | 0.01 | 0.00 | 0.02 |
| things_per_head | 0.07 | 0.05 | 0.43 |
| equipped_share | 0.06 | 0.03 | 0.24 |
| crafts | 21.50 | 26.50 | 82.00 |
| learn_rate | 1.33 | 1.96 | 1.81 |
| loudness | 1.57 | 1.45 | 1.45 |

## deaf so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 285 | [-1556, 2120] | chưa rõ |
| final_pop | 244 | [-487, 1089] | chưa rõ |
| innovations | -2.75 | [-35.75, 29.06] | chưa rõ |
| mean_known | 0.40 | [-26.06, 25.90] | chưa rõ |
| soil_health | -0.02 | [-0.10, 0.05] | chưa rõ |
| lived_soil | -0.02 | [-0.09, 0.05] | chưa rõ |
| settled_share | -0.15 | [-0.43, 0.15] | chưa rõ |
| obedience | 0.01 | [-0.25, 0.26] | chưa rõ |
| breed_rate | -0.51 | [-2.91, 1.73] | chưa rõ |
| swing | 0.51 | [-3.85, 5.36] | chưa rõ |
| final_level | -0.06 | [-2.12, 2.00] | chưa rõ |
| plastic | -0.00 | [-0.01, 0.01] | chưa rõ |
| signal_mi | -0.01 | [-0.03, 0.00] | chưa rõ |
| things_per_head | 0.18 | [-0.12, 0.53] | chưa rõ |
| equipped_share | 0.07 | [-0.07, 0.22] | chưa rõ |
| crafts | -2.06 | [-30.62, 25.44] | chưa rõ |
| learn_rate | 0.24 | [-0.89, 1.46] | chưa rõ |
| loudness | -0.29 | [-0.77, 0.19] | chưa rõ |

## hear-strangers so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 1961 | [-185, 4292] | chưa rõ |
| final_pop | 453 | [-174, 1100] | chưa rõ |
| innovations | 26.44 | [-4.69, 57.12] | chưa rõ |
| mean_known | 17.48 | [-7.86, 42.18] | chưa rõ |
| soil_health | -0.09 | [-0.20, 0.00] | chưa rõ |
| lived_soil | -0.07 | [-0.17, 0.01] | chưa rõ |
| settled_share | -0.14 | [-0.39, 0.13] | chưa rõ |
| obedience | 0.03 | [-0.20, 0.26] | chưa rõ |
| breed_rate | 2.43 | [-0.47, 5.39] | chưa rõ |
| swing | -0.01 | [-3.71, 3.45] | chưa rõ |
| final_level | 1.81 | [0.00, 3.56] | chưa rõ |
| plastic | 0.00 | [-0.00, 0.01] | chưa rõ |
| signal_mi | 0.01 | [-0.02, 0.03] | chưa rõ |
| things_per_head | 0.70 | [0.21, 1.23] | khác 0 |
| equipped_share | 0.23 | [0.06, 0.41] | khác 0 |
| crafts | 23.62 | [-3.19, 50.31] | chưa rõ |
| learn_rate | 0.73 | [-0.40, 1.93] | chưa rõ |
| loudness | -0.03 | [-0.44, 0.38] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 180 | 192 | 410 | 722 | 232 |
| deaf | 142 | 156 | 346 | 744 | 446 |
| hear-strangers | 183 | 728 | 1229 | 1840 | 1048 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.86 | 7.93 | 14.02 | 31.26 | 34.65 |
| deaf | 2.45 | 5.67 | 16.80 | 28.38 | 55.84 |
| hear-strangers | 3.11 | 8.43 | 29.93 | 57.00 | 55.90 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| deaf | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| hear-strangers | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.06 | 0.02 | 0.02 | 0.01 | 0.01 |
| deaf | 0.05 | 0.01 | 0.01 | 0.01 | 0.00 |
| hear-strangers | 0.06 | 0.06 | 0.03 | 0.03 | 0.02 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.45 | 0.31 | 0.13 | 0.14 | 0.16 |
| deaf | 0.26 | 0.27 | 0.25 | 0.30 | 0.56 |
| hear-strangers | 0.54 | 0.46 | 0.94 | 0.77 | 0.67 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.99 | 0.96 | 1.00 |
| deaf | 1.00 | 1.00 | 0.99 | 0.99 | 0.99 |
| hear-strangers | 1.00 | 1.00 | 0.98 | 0.98 | 0.94 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.93 | 0.96 | 0.96 | 0.96 | 0.95 |
| deaf | 0.91 | 0.89 | 0.94 | 0.93 | 0.93 |
| hear-strangers | 0.90 | 0.76 | 0.88 | 0.75 | 0.82 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.63 | 2.71 | 1.92 | 1.84 | 1.97 |
| deaf | 2.62 | 2.63 | 2.68 | 2.59 | 2.51 |
| hear-strangers | 2.74 | 2.56 | 1.98 | 2.24 | 2.13 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.58 | 1.63 | 1.73 | 1.73 | 1.74 |
| deaf | 1.54 | 1.50 | 1.57 | 1.63 | 1.66 |
| hear-strangers | 1.60 | 1.60 | 1.60 | 1.54 | 1.47 |

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
| deaf | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| deaf | 2 | boom and bust | 4201 | 1210 | 71.96 | 1.00 | 0.95 | 0.66 | 0.04 |
| deaf | 3 | flourishing | 3870 | 320 | 75.55 | 0.50 | 0.03 | 2.25 | 0.00 |
| deaf | 4 | flourishing | 6031 | 1180 | 86.39 | 0.67 | 0.77 | 1.51 | 0.00 |
| deaf | 5 | flourishing | 7009 | 2285 | 99.71 | 1.00 | 0.93 | 0.65 | 0.01 |
| deaf | 6 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| deaf | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| deaf | 8 | boom and bust | 9038 | 1813 | 32.85 | 0.98 | 0.76 | 0.04 | 0.00 |
| deaf | 9 | boom and bust | 1822 | 795 | 19.01 | 0.99 | 0.98 | 0.06 | 0.03 |
| deaf | 10 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| deaf | 11 | flourishing | 1619 | 549 | 36.84 | 0.99 | 0.95 | 0.63 | 0.00 |
| deaf | 12 | boom and bust | 1000 | 113 | 9.98 | 1.00 | 0.99 | 0.03 | 0.00 |
| deaf | 13 | flourishing | 1192 | 749 | 11.98 | 0.99 | 0.97 | 0.01 | 0.01 |
| deaf | 14 | boom and bust | 4170 | 342 | 55.84 | 0.98 | 0.87 | 0.31 | 0.00 |
| deaf | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| deaf | 16 | flourishing | 9038 | 5792 | 112 | 0.92 | 0.90 | 0.56 | 0.03 |
| hear-strangers | 1 | boom and bust | 3806 | 1444 | 30.54 | 0.98 | 0.94 | 0.14 | 0.09 |
| hear-strangers | 2 | boom and bust | 6672 | 986 | 41.33 | 0.99 | 0.91 | 0.08 | 0.09 |
| hear-strangers | 3 | flourishing | 4079 | 909 | 74.00 | 0.85 | 0.71 | 0.88 | 0.02 |
| hear-strangers | 4 | flourishing | 4627 | 1225 | 107 | 1.00 | 0.94 | 1.21 | 0.01 |
| hear-strangers | 5 | flourishing | 7087 | 3536 | 104 | 0.72 | 0.85 | 0.41 | 0.01 |
| hear-strangers | 6 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| hear-strangers | 7 | boom and bust | 3738 | 1231 | 57.50 | 0.92 | 0.32 | 2.64 | 0.07 |
| hear-strangers | 8 | flourishing | 6550 | 1310 | 62.66 | 1.00 | 0.90 | 0.29 | 0.02 |
| hear-strangers | 9 | collapsed | 1000 | 3.00 | 16.00 | 1.00 | 1.00 | 1.00 | 0.00 |
| hear-strangers | 10 | flourishing | 5061 | 1109 | 44.51 | 0.96 | 0.53 | 0.46 | 0.04 |
| hear-strangers | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| hear-strangers | 12 | flourishing | 3650 | 2800 | 108 | 0.71 | 0.03 | 2.80 | 0.03 |
| hear-strangers | 13 | flourishing | 7310 | 2403 | 52.76 | 0.92 | 0.96 | 0.06 | 0.00 |
| hear-strangers | 14 | collapsed | 4127 | 391 | 86.15 | 0.39 | 0.37 | 2.62 | 0.05 |
| hear-strangers | 15 | boom and bust on dying land | 2820 | 466 | 45.85 | 0.50 | 0.03 | 1.94 | 0.02 |
| hear-strangers | 16 | boom and bust | 18289 | 685 | 54.30 | 0.99 | 0.80 | 0.35 | 0.01 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
