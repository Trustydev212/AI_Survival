# Học từ hậu quả có làm xã hội khá hơn không?

Luật Hebb như cũ (mặc định), so với actor-critic có vết đủ điều kiện, so với actor-critic cộng truyền lại cái đã học cho người khác, và so với nhánh đối chứng quan trọng nhất: vẫn chọn việc theo xác suất nhưng không học gì cả.

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **sample-only**: `--gradient --actor-rate 0 --critic-rate 0` · 16 thế giới
- **gradient**: `--gradient` · 16 thế giới
- **gradient-social**: `--gradient --know-rate 0.3` · 16 thế giới

## Kết cục

| nhánh | boom and bust | boom and bust on dying land | collapsed | extinct | flourishing | flourishing on dying land | tốt |
|---|---|---|---|---|---|---|---|
| default | 4 | 0 | 1 | 2 | 9 | 0 | 9/16 |
| sample-only | 6 | 1 | 1 | 1 | 6 | 1 | 6/16 |
| gradient | 3 | 0 | 1 | 10 | 2 | 0 | 2/16 |
| gradient-social | 2 | 0 | 0 | 5 | 9 | 0 | 9/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | sample-only | gradient | gradient-social |
|---|---|---|---|---|
| peak_pop | 3624 | 4524 | 1000 | 1310 |
| final_pop | 914 | 1032 | 0.00 | 473 |
| innovations | 124 | 128 | 17.50 | 63.00 |
| mean_known | 36.74 | 64.72 | -0.00 | 29.86 |
| soil_health | 0.98 | 0.91 | 1.00 | 0.97 |
| lived_soil | 0.96 | 0.91 | 1.00 | 0.97 |
| settled_share | 0.68 | 0.85 | 0.00 | 0.04 |
| obedience | 0.39 | 0.37 | 0.00 | 0.09 |
| breed_rate | 6.42 | 7.77 | 0.00 | 2.55 |
| swing | 3.39 | 3.94 | 4.94 | 3.75 |
| final_level | 6.50 | 7.00 | 0.00 | 5.50 |
| plastic | 0.01 | 0.00 | -0.00 | 0.01 |
| signal_mi | 0.02 | 0.03 | 0.00 | 0.00 |
| signal_meaning | 0.02 | 0.02 | 0.00 | 0.01 |
| things_per_head | 0.47 | 0.29 | -0.00 | 0.51 |
| equipped_share | 0.23 | 0.11 | 0.00 | 0.30 |
| crafts | 93.50 | 87.50 | 17.50 | 41.50 |
| learn_rate | 2.08 | 3.20 | -0.00 | 1.78 |
| loudness | 1.51 | 1.50 | -0.00 | 1.55 |
| hunts | 0.00 | 0.00 | 0.00 | 0.00 |
| division_of_labour | 0.49 | 0.52 | 0.00 | 0.44 |

## sample-only so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 1712 | [-170, 3590] | chưa rõ |
| final_pop | 307 | [-292, 947] | chưa rõ |
| innovations | 16.94 | [-11.75, 43.62] | chưa rõ |
| mean_known | 15.92 | [-8.66, 38.50] | chưa rõ |
| soil_health | -0.07 | [-0.19, 0.05] | chưa rõ |
| lived_soil | -0.07 | [-0.17, 0.04] | chưa rõ |
| settled_share | 0.02 | [-0.26, 0.29] | chưa rõ |
| obedience | -0.03 | [-0.25, 0.18] | chưa rõ |
| breed_rate | 0.50 | [-2.61, 3.42] | chưa rõ |
| swing | -0.35 | [-3.40, 2.82] | chưa rõ |
| final_level | 1.12 | [-0.31, 2.56] | chưa rõ |
| plastic | -0.01 | [-0.01, -0.01] | khác 0 |
| signal_mi | -0.01 | [-0.04, 0.02] | chưa rõ |
| signal_meaning | -0.01 | [-0.04, 0.01] | chưa rõ |
| things_per_head | 0.12 | [-0.52, 0.77] | chưa rõ |
| equipped_share | 0.02 | [-0.20, 0.24] | chưa rõ |
| crafts | 7.75 | [-21.12, 35.25] | chưa rõ |
| learn_rate | 0.77 | [-0.68, 2.15] | chưa rõ |
| loudness | 0.06 | [-0.27, 0.39] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.00 | [-0.14, 0.14] | chưa rõ |

## gradient so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -2226 | [-3445, -1137] | khác 0 |
| final_pop | -848 | [-1287, -445] | khác 0 |
| innovations | -54.38 | [-81.56, -25.25] | khác 0 |
| mean_known | -37.01 | [-57.67, -17.79] | khác 0 |
| soil_health | 0.06 | [-0.03, 0.16] | chưa rõ |
| lived_soil | 0.05 | [-0.03, 0.14] | chưa rõ |
| settled_share | -0.35 | [-0.60, -0.08] | khác 0 |
| obedience | -0.33 | [-0.53, -0.13] | khác 0 |
| breed_rate | -5.46 | [-8.09, -2.97] | khác 0 |
| swing | 4.03 | [-0.32, 8.52] | chưa rõ |
| final_level | -3.44 | [-5.12, -1.62] | khác 0 |
| plastic | -0.01 | [-0.01, -0.00] | khác 0 |
| signal_mi | -0.04 | [-0.07, -0.02] | khác 0 |
| signal_meaning | -0.03 | [-0.06, -0.01] | khác 0 |
| things_per_head | -0.58 | [-1.06, -0.11] | khác 0 |
| equipped_share | -0.19 | [-0.38, -0.00] | khác 0 |
| crafts | -46.00 | [-71.75, -18.56] | khác 0 |
| learn_rate | -1.50 | [-2.93, -0.11] | khác 0 |
| loudness | -0.79 | [-1.21, -0.32] | khác 0 |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.26 | [-0.42, -0.08] | khác 0 |

## gradient-social so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -1832 | [-3097, -722] | khác 0 |
| final_pop | -568 | [-1025, -154] | khác 0 |
| innovations | -25.56 | [-57.56, 5.88] | chưa rõ |
| mean_known | -18.39 | [-42.34, 3.69] | chưa rõ |
| soil_health | 0.03 | [-0.06, 0.13] | chưa rõ |
| lived_soil | 0.01 | [-0.07, 0.10] | chưa rõ |
| settled_share | -0.33 | [-0.57, -0.08] | khác 0 |
| obedience | -0.22 | [-0.44, 0.01] | chưa rõ |
| breed_rate | -2.86 | [-6.17, 0.68] | chưa rõ |
| swing | 0.97 | [-2.39, 4.60] | chưa rõ |
| final_level | -1.25 | [-3.31, 0.62] | chưa rõ |
| plastic | 0.00 | [-0.00, 0.01] | chưa rõ |
| signal_mi | -0.04 | [-0.06, -0.01] | khác 0 |
| signal_meaning | -0.02 | [-0.05, 0.00] | chưa rõ |
| things_per_head | 0.17 | [-0.50, 0.81] | chưa rõ |
| equipped_share | 0.07 | [-0.17, 0.29] | chưa rõ |
| crafts | -22.06 | [-52.25, 7.75] | chưa rõ |
| learn_rate | -0.61 | [-2.08, 0.77] | chưa rõ |
| loudness | -0.25 | [-0.71, 0.18] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.13 | [-0.29, 0.03] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 993 | 1886 | 914 |
| sample-only | 690 | 1031 | 1104 | 2344 | 1032 |
| gradient | 130 | 33.50 | 3.00 | 0.00 | 0.00 |
| gradient-social | 154 | 159 | 282 | 706 | 473 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.55 | 68.84 | 45.31 |
| sample-only | 4.98 | 12.28 | 53.33 | 83.88 | 65.75 |
| gradient | 1.93 | 2.03 | 4.00 | 21.35 | 28.36 |
| gradient-social | 1.82 | 1.38 | 9.47 | 35.18 | 46.70 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| sample-only | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| gradient | 0.02 | 0.01 | 0.01 | 0.01 | 0.01 |
| gradient-social | 0.02 | 0.01 | 0.01 | 0.02 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.04 | 0.06 | 0.04 | 0.03 | 0.03 |
| sample-only | 0.04 | 0.05 | 0.03 | 0.03 | 0.03 |
| gradient | 0.00 | 0.00 | 0.00 | 0.01 | 0.01 |
| gradient-social | 0.00 | 0.03 | 0.00 | 0.02 | 0.01 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.03 | 0.05 | 0.03 |
| sample-only | 0.03 | 0.03 | 0.03 | 0.03 | 0.02 |
| gradient | 0.00 | 0.00 | 0.00 | 0.00 | 0.01 |
| gradient-social | 0.02 | 0.03 | 0.03 | 0.03 | 0.02 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.76 | 0.92 | 0.97 |
| sample-only | 0.62 | 1.03 | 0.93 | 0.60 | 0.40 |
| gradient | 0.75 | 0.55 | 0.53 | 0.36 | 0.27 |
| gradient-social | 0.83 | 0.44 | 0.69 | 1.23 | 1.58 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.97 |
| sample-only | 1.00 | 0.98 | 0.93 | 0.85 | 0.90 |
| gradient | 1.00 | 1.00 | 1.00 | 0.96 | 0.88 |
| gradient-social | 1.00 | 1.00 | 0.98 | 0.96 | 0.84 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.68 | 0.75 | 0.69 | 0.76 |
| sample-only | 0.54 | 0.48 | 0.71 | 0.86 | 0.88 |
| gradient | 0.91 | 0.87 | 0.80 | 0.88 | 0.54 |
| gradient-social | 0.88 | 0.59 | 0.49 | 0.32 | 0.08 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.24 | 3.05 |
| sample-only | 3.20 | 3.14 | 3.20 | 3.45 | 3.34 |
| gradient | 3.43 | 3.40 | 2.55 | 2.19 | 2.00 |
| gradient-social | 3.34 | 2.44 | 2.46 | 3.13 | 3.10 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.58 | 1.55 |
| sample-only | 1.50 | 1.52 | 1.53 | 1.54 | 1.51 |
| gradient | 1.56 | 1.44 | 1.60 | 1.62 | 1.57 |
| gradient-social | 1.55 | 1.52 | 1.58 | 1.59 | 1.63 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.72 | 0.55 | 0.54 | 0.50 | 0.51 |
| sample-only | 0.55 | 0.49 | 0.54 | 0.53 | 0.53 |
| gradient | 0.76 | 0.62 | 0.51 | 0.53 | 0.60 |
| gradient-social | 0.73 | 0.61 | 0.51 | 0.49 | 0.46 |

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
| sample-only | 1 | flourishing on dying land | 2536 | 673 | 33.41 | 0.48 | 0.02 | 2.65 | 0.05 | 0.08 |
| sample-only | 2 | boom and bust | 4801 | 1854 | 55.89 | 0.94 | 0.94 | 0.11 | 0.01 | 0.01 |
| sample-only | 3 | boom and bust | 10278 | 4293 | 107 | 0.99 | 0.89 | 0.17 | 0.03 | 0.03 |
| sample-only | 4 | flourishing | 2904 | 2324 | 65.75 | 0.93 | 0.96 | 0.11 | 0.03 | 0.04 |
| sample-only | 5 | flourishing | 5128 | 677 | 85.47 | 0.57 | 0.13 | 2.31 | 0.00 | 0.00 |
| sample-only | 6 | boom and bust | 10758 | 448 | 53.43 | 0.98 | 0.90 | 0.02 | 0.00 | 0.03 |
| sample-only | 7 | flourishing | 2948 | 2765 | 105 | 0.65 | 0.00 | 2.06 | 0.01 | 0.06 |
| sample-only | 8 | flourishing | 4108 | 1798 | 86.50 | 0.63 | 0.55 | 1.02 | 0.02 | 0.01 |
| sample-only | 9 | boom and bust | 6394 | 1079 | 93.83 | 0.80 | 0.90 | 0.17 | 0.05 | 0.01 |
| sample-only | 10 | collapsed | 11157 | 1081 | 38.07 | 1.00 | 0.83 | 0.03 | 0.08 | 0.00 |
| sample-only | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| sample-only | 12 | flourishing | 1969 | 985 | 41.80 | 0.72 | 0.17 | 2.61 | 0.14 | 0.02 |
| sample-only | 13 | boom and bust | 4247 | 867 | 87.52 | 0.90 | 0.88 | 0.40 | 0.05 | 0.00 |
| sample-only | 14 | boom and bust on dying land | 2504 | 557 | 85.87 | 0.40 | 0.02 | 2.24 | 0.10 | 0.02 |
| sample-only | 15 | boom and bust | 5215 | 826 | 18.30 | 0.99 | 0.97 | 0.01 | 0.05 | 0.03 |
| sample-only | 16 | flourishing | 5869 | 1184 | 63.69 | 0.98 | 0.92 | 1.08 | 0.02 | 0.07 |
| gradient | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 2 | collapsed | 1000 | 35.00 | 2.17 | 1.00 | 1.00 | 0.03 | 0.00 | 0.02 |
| gradient | 3 | boom and bust | 1534 | 474 | 56.02 | 0.62 | 0.05 | 1.45 | 0.01 | 0.01 |
| gradient | 4 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 5 | boom and bust | 1000 | 200 | 4.73 | 1.00 | 0.94 | 0.36 | 0.00 | 0.01 |
| gradient | 6 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 7 | boom and bust | 1149 | 212 | 25.14 | 0.81 | 0.14 | 0.18 | 0.03 | 0.06 |
| gradient | 8 | flourishing | 2784 | 1330 | 31.58 | 0.96 | 0.93 | 0.17 | 0.02 | 0.00 |
| gradient | 9 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 10 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 12 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 13 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 14 | flourishing | 1342 | 668 | 54.50 | 0.76 | 0.08 | 1.52 | 0.01 | 0.00 |
| gradient | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 16 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 1 | boom and bust | 1336 | 606 | 4.22 | 0.98 | 0.04 | 0.17 | 0.03 | 0.01 |
| gradient-social | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 3 | flourishing | 1589 | 296 | 34.72 | 0.71 | 0.08 | 2.52 | 0.00 | 0.00 |
| gradient-social | 4 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 5 | flourishing | 2088 | 1283 | 82.94 | 1.00 | 0.41 | 2.09 | 0.03 | 0.02 |
| gradient-social | 6 | flourishing | 1284 | 792 | 25.00 | 0.96 | 0.69 | 0.27 | 0.02 | 0.02 |
| gradient-social | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 8 | flourishing | 1000 | 499 | 10.45 | 1.00 | 0.82 | 0.06 | 0.03 | 0.02 |
| gradient-social | 9 | flourishing | 1134 | 614 | 48.37 | 0.84 | 0.04 | 2.46 | 0.01 | 0.02 |
| gradient-social | 10 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 11 | flourishing | 1804 | 447 | 46.70 | 0.95 | 0.53 | 1.24 | 0.02 | 0.00 |
| gradient-social | 12 | flourishing | 1834 | 787 | 35.95 | 0.83 | 0.01 | 1.58 | 0.00 | 0.06 |
| gradient-social | 13 | flourishing | 2693 | 427 | 70.24 | 0.70 | 0.02 | 2.90 | 0.00 | 0.07 |
| gradient-social | 14 | flourishing | 1660 | 848 | 56.13 | 0.77 | 0.06 | 1.69 | 0.00 | 0.00 |
| gradient-social | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 16 | boom and bust | 3690 | 811 | 57.36 | 0.83 | 0.90 | 0.76 | 0.00 | 0.04 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
