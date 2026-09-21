# Học từ hậu quả có làm xã hội khá hơn không?

Luật Hebb như cũ (mặc định), so với actor-critic có vết đủ điều kiện, so với actor-critic cộng truyền lại cái đã học cho người khác, và so với nhánh đối chứng quan trọng nhất: vẫn chọn việc theo xác suất nhưng không học gì cả. Nhánh gradient-slow hạ bước học của bộ chọn việc xuống một phần mười, để tách xem cái giết thế giới là việc học hay là bước học quá dài.

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **sample-only**: `--gradient --actor-rate 0 --critic-rate 0` · 16 thế giới
- **gradient**: `--gradient` · 16 thế giới
- **gradient-social**: `--gradient --know-rate 0.3` · 16 thế giới
- **gradient-slow**: `--gradient --actor-rate 0.01` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | fallen | flourishing | flourishing on dying land | surviving | tốt |
|---|---|---|---|---|---|---|---|---|
| default | 5 | 0 | 2 | 0 | 4 | 0 | 5 | 9/16 |
| sample-only | 6 | 1 | 1 | 1 | 5 | 1 | 1 | 6/16 |
| gradient | 2 | 1 | 10 | 0 | 1 | 0 | 2 | 3/16 |
| gradient-social | 4 | 0 | 5 | 0 | 2 | 0 | 5 | 7/16 |
| gradient-slow | 2 | 2 | 1 | 1 | 5 | 1 | 4 | 9/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | sample-only | gradient | gradient-social | gradient-slow |
|---|---|---|---|---|---|
| peak_pop | 2648 | 4468 | 1000 | 1310 | 3508 |
| final_pop | 938 | 964 | 0.00 | 576 | 776 |
| innovations | 145 | 153 | 17.50 | 63.00 | 154 |
| mean_known | 36.42 | 74.07 | -0.00 | 30.59 | 64.19 |
| soil_health | 0.95 | 0.88 | 1.00 | 0.94 | 0.92 |
| lived_soil | 0.94 | 0.88 | 1.00 | 0.93 | 0.92 |
| settled_share | 0.46 | 0.80 | 0.00 | 0.03 | 0.54 |
| obedience | 0.25 | 0.50 | 0.00 | 0.09 | 0.31 |
| breed_rate | 8.87 | 6.61 | 0.00 | 2.18 | 4.50 |
| swing | 3.39 | 4.04 | 4.94 | 4.64 | 3.21 |
| final_level | 1.50 | 3.00 | 0.00 | 1.00 | 3.00 |
| plastic | 0.01 | 0.00 | -0.00 | 0.01 | 0.00 |
| signal_mi | 0.02 | 0.02 | 0.00 | 0.00 | 0.02 |
| signal_meaning | 0.03 | 0.03 | 0.00 | 0.01 | 0.02 |
| things_per_head | 0.76 | 0.60 | -0.00 | 0.49 | 0.67 |
| equipped_share | 0.32 | 0.20 | 0.00 | 0.29 | 0.30 |
| crafts | 132 | 124 | 17.50 | 42.00 | 142 |
| learn_rate | 2.06 | 3.11 | -0.00 | 1.65 | 2.89 |
| loudness | 1.54 | 1.51 | -0.00 | 1.53 | 1.58 |
| hunts | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| division_of_labour | 0.47 | 0.55 | 0.00 | 0.44 | 0.52 |

## sample-only so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 1329 | [-83.06, 2784] | chưa rõ |
| final_pop | 143 | [-190, 459] | chưa rõ |
| innovations | 30.81 | [-24.38, 83.69] | chưa rõ |
| mean_known | 31.04 | [0.74, 60.77] | khác 0 |
| soil_health | -0.03 | [-0.13, 0.08] | chưa rõ |
| lived_soil | -0.03 | [-0.12, 0.07] | chưa rõ |
| settled_share | 0.07 | [-0.20, 0.33] | chưa rõ |
| obedience | 0.11 | [-0.12, 0.33] | chưa rõ |
| breed_rate | -1.08 | [-3.88, 1.66] | chưa rõ |
| swing | -1.18 | [-3.78, 0.88] | chưa rõ |
| final_level | 1.19 | [-0.06, 2.44] | chưa rõ |
| plastic | -0.01 | [-0.01, -0.01] | khác 0 |
| signal_mi | 0.01 | [-0.02, 0.03] | chưa rõ |
| signal_meaning | -0.01 | [-0.03, 0.01] | chưa rõ |
| things_per_head | 0.05 | [-0.57, 0.68] | chưa rõ |
| equipped_share | -0.00 | [-0.20, 0.20] | chưa rõ |
| crafts | 20.56 | [-30.56, 68.88] | chưa rõ |
| learn_rate | 0.71 | [-0.73, 2.09] | chưa rõ |
| loudness | 0.08 | [-0.24, 0.41] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | 0.03 | [-0.11, 0.16] | chưa rõ |

## gradient so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -1875 | [-2889, -950] | khác 0 |
| final_pop | -660 | [-947, -360] | khác 0 |
| innovations | -88.62 | [-138, -39.94] | khác 0 |
| mean_known | -38.20 | [-61.99, -15.04] | khác 0 |
| soil_health | 0.07 | [-0.03, 0.16] | chưa rõ |
| lived_soil | 0.06 | [-0.03, 0.14] | chưa rõ |
| settled_share | -0.30 | [-0.55, -0.04] | khác 0 |
| obedience | -0.24 | [-0.43, -0.05] | khác 0 |
| breed_rate | -6.27 | [-8.59, -3.93] | khác 0 |
| swing | 3.81 | [-0.49, 8.33] | chưa rõ |
| final_level | -1.69 | [-2.69, -0.75] | khác 0 |
| plastic | -0.01 | [-0.01, -0.00] | khác 0 |
| signal_mi | -0.03 | [-0.05, -0.01] | khác 0 |
| signal_meaning | -0.04 | [-0.06, -0.01] | khác 0 |
| things_per_head | -0.63 | [-1.15, -0.12] | khác 0 |
| equipped_share | -0.20 | [-0.39, -0.01] | khác 0 |
| crafts | -73.69 | [-117, -29.56] | khác 0 |
| learn_rate | -1.47 | [-2.92, -0.04] | khác 0 |
| loudness | -0.78 | [-1.20, -0.33] | khác 0 |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.24 | [-0.40, -0.06] | khác 0 |

## gradient-social so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -1487 | [-2558, -521] | khác 0 |
| final_pop | -356 | [-669, -62.62] | khác 0 |
| innovations | -51.50 | [-105, 0.19] | chưa rõ |
| mean_known | -13.60 | [-42.72, 16.64] | chưa rõ |
| soil_health | 0.00 | [-0.09, 0.11] | chưa rõ |
| lived_soil | 0.00 | [-0.09, 0.10] | chưa rõ |
| settled_share | -0.26 | [-0.49, -0.01] | khác 0 |
| obedience | -0.14 | [-0.35, 0.06] | chưa rõ |
| breed_rate | -3.65 | [-6.91, -0.04] | khác 0 |
| swing | 1.35 | [-2.03, 4.76] | chưa rõ |
| final_level | -0.81 | [-2.06, 0.50] | chưa rõ |
| plastic | 0.00 | [-0.01, 0.01] | chưa rõ |
| signal_mi | -0.01 | [-0.03, 0.00] | chưa rõ |
| signal_meaning | -0.02 | [-0.04, 0.00] | chưa rõ |
| things_per_head | 0.10 | [-0.60, 0.76] | chưa rõ |
| equipped_share | 0.06 | [-0.18, 0.29] | chưa rõ |
| crafts | -41.44 | [-89.44, 5.31] | chưa rõ |
| learn_rate | -0.60 | [-2.09, 0.80] | chưa rõ |
| loudness | -0.24 | [-0.70, 0.19] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.09 | [-0.26, 0.07] | chưa rõ |

## gradient-slow so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 822 | [-510, 2143] | chưa rõ |
| final_pop | 32.06 | [-331, 420] | chưa rõ |
| innovations | 21.38 | [-36.50, 75.88] | chưa rõ |
| mean_known | 14.97 | [-12.48, 40.59] | chưa rõ |
| soil_health | -0.03 | [-0.15, 0.08] | chưa rõ |
| lived_soil | -0.03 | [-0.13, 0.07] | chưa rõ |
| settled_share | 0.05 | [-0.22, 0.30] | chưa rõ |
| obedience | 0.10 | [-0.14, 0.32] | chưa rõ |
| breed_rate | -1.97 | [-4.77, 0.79] | chưa rõ |
| swing | -1.44 | [-4.10, 0.72] | chưa rõ |
| final_level | 0.56 | [-0.56, 1.62] | chưa rõ |
| plastic | -0.01 | [-0.01, -0.01] | khác 0 |
| signal_mi | 0.01 | [-0.02, 0.03] | chưa rõ |
| signal_meaning | -0.02 | [-0.04, 0.00] | chưa rõ |
| things_per_head | 0.11 | [-0.54, 0.76] | chưa rõ |
| equipped_share | 0.05 | [-0.16, 0.26] | chưa rõ |
| crafts | 10.81 | [-40.12, 58.94] | chưa rõ |
| learn_rate | 0.35 | [-1.17, 1.79] | chưa rõ |
| loudness | 0.10 | [-0.21, 0.43] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | 0.04 | [-0.09, 0.18] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 794 | 1652 | 938 |
| sample-only | 690 | 1226 | 1118 | 2152 | 964 |
| gradient | 130 | 33.50 | 3.00 | 0.00 | 0.00 |
| gradient-social | 154 | 159 | 282 | 867 | 576 |
| gradient-slow | 344 | 1237 | 946 | 1576 | 776 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.02 | 72.50 | 56.83 |
| sample-only | 4.98 | 12.28 | 54.53 | 77.58 | 76.69 |
| gradient | 1.93 | 2.03 | 4.00 | 21.35 | 28.36 |
| gradient-social | 1.82 | 1.38 | 9.47 | 34.55 | 47.39 |
| gradient-slow | 4.79 | 13.54 | 64.82 | 87.00 | 64.41 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| sample-only | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| gradient | 0.02 | 0.01 | 0.01 | 0.01 | 0.01 |
| gradient-social | 0.02 | 0.01 | 0.01 | 0.02 | 0.02 |
| gradient-slow | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.05 | 0.04 | 0.03 | 0.03 |
| sample-only | 0.04 | 0.04 | 0.04 | 0.03 | 0.03 |
| gradient | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| gradient-social | 0.00 | 0.03 | 0.00 | 0.02 | 0.03 |
| gradient-slow | 0.06 | 0.05 | 0.02 | 0.02 | 0.02 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.02 | 0.06 | 0.04 |
| sample-only | 0.03 | 0.03 | 0.03 | 0.02 | 0.03 |
| gradient | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| gradient-social | 0.02 | 0.03 | 0.03 | 0.03 | 0.02 |
| gradient-slow | 0.02 | 0.01 | 0.02 | 0.02 | 0.02 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.85 | 1.01 | 1.00 |
| sample-only | 0.62 | 1.03 | 1.20 | 0.44 | 0.98 |
| gradient | 0.75 | 0.55 | 0.53 | 0.36 | 0.27 |
| gradient-social | 0.83 | 0.44 | 0.69 | 1.23 | 1.50 |
| gradient-slow | 0.65 | 0.93 | 0.75 | 0.78 | 0.89 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.92 |
| sample-only | 1.00 | 0.98 | 0.92 | 0.87 | 0.87 |
| gradient | 1.00 | 1.00 | 1.00 | 0.96 | 0.88 |
| gradient-social | 1.00 | 1.00 | 0.98 | 0.95 | 0.84 |
| gradient-slow | 1.00 | 0.98 | 0.95 | 0.87 | 0.92 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.64 | 0.70 | 0.66 | 0.62 |
| sample-only | 0.54 | 0.39 | 0.66 | 0.81 | 0.88 |
| gradient | 0.91 | 0.87 | 0.80 | 0.88 | 0.54 |
| gradient-social | 0.88 | 0.59 | 0.47 | 0.41 | 0.23 |
| gradient-slow | 0.65 | 0.56 | 0.82 | 0.81 | 0.58 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.17 | 3.01 |
| sample-only | 3.20 | 3.14 | 3.20 | 3.31 | 3.16 |
| gradient | 3.43 | 3.40 | 2.55 | 2.19 | 2.00 |
| gradient-social | 3.34 | 2.44 | 2.46 | 3.14 | 3.18 |
| gradient-slow | 3.41 | 3.43 | 2.99 | 3.01 | 2.91 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.57 | 1.58 |
| sample-only | 1.49 | 1.53 | 1.52 | 1.54 | 1.52 |
| gradient | 1.56 | 1.44 | 1.60 | 1.62 | 1.57 |
| gradient-social | 1.55 | 1.52 | 1.60 | 1.63 | 1.64 |
| gradient-slow | 1.53 | 1.53 | 1.54 | 1.53 | 1.58 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.72 | 0.57 | 0.53 | 0.51 | 0.48 |
| sample-only | 0.55 | 0.49 | 0.53 | 0.51 | 0.55 |
| gradient | 0.76 | 0.62 | 0.51 | 0.56 | 0.60 |
| gradient-social | 0.73 | 0.61 | 0.47 | 0.48 | 0.50 |
| gradient-slow | 0.59 | 0.51 | 0.51 | 0.54 | 0.54 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | hiểu (bit) | nghĩa (bit) |
|---|---|---|---|---|---|---|---|---|---|---|
| default | 1 | surviving | 1332 | 983 | 38.66 | 0.93 | 0.09 | 1.46 | 0.04 | 0.09 |
| default | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 3 | flourishing | 2398 | 992 | 118 | 0.84 | 0.54 | 0.95 | 0.04 | 0.04 |
| default | 4 | surviving | 4330 | 1037 | 22.86 | 0.98 | 0.96 | 0.03 | 0.00 | 0.08 |
| default | 5 | boom and bust | 2898 | 1038 | 92.08 | 0.56 | 0.35 | 1.58 | 0.02 | 0.02 |
| default | 6 | boom and bust | 4115 | 431 | 34.18 | 0.76 | 0.32 | 1.05 | 0.03 | 0.00 |
| default | 7 | surviving | 1000 | 523 | 28.75 | 0.97 | 0.01 | 3.09 | 0.03 | 0.01 |
| default | 8 | flourishing | 6291 | 912 | 99.67 | 0.71 | 0.38 | 1.54 | 0.03 | 0.00 |
| default | 9 | boom and bust | 7298 | 963 | 86.16 | 0.99 | 0.91 | 0.21 | 0.03 | 0.07 |
| default | 10 | boom and bust | 4472 | 1951 | 16.52 | 0.99 | 0.96 | 0.17 | 0.10 | 0.03 |
| default | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 12 | surviving | 1858 | 1105 | 19.02 | 0.98 | 0.94 | 0.10 | 0.02 | 0.00 |
| default | 13 | boom and bust | 4484 | 610 | 104 | 0.87 | 0.70 | 0.56 | 0.02 | 0.03 |
| default | 14 | flourishing | 2120 | 594 | 90.01 | 0.54 | 0.07 | 1.66 | 0.00 | 0.11 |
| default | 15 | surviving | 1132 | 558 | 14.63 | 1.00 | 0.99 | 0.00 | 0.13 | 0.06 |
| default | 16 | flourishing | 3524 | 1740 | 75.00 | 0.92 | 0.75 | 1.87 | 0.01 | 0.11 |
| sample-only | 1 | fallen | 2200 | 1007 | 71.45 | 0.73 | 0.11 | 2.41 | 0.03 | 0.04 |
| sample-only | 2 | flourishing | 4989 | 1134 | 62.46 | 0.96 | 0.89 | 0.12 | 0.00 | 0.02 |
| sample-only | 3 | boom and bust | 4687 | 1638 | 175 | 0.86 | 0.93 | 0.10 | 0.06 | 0.03 |
| sample-only | 4 | flourishing | 4249 | 1419 | 65.15 | 0.91 | 0.95 | 0.12 | 0.02 | 0.04 |
| sample-only | 5 | boom and bust | 5843 | 838 | 87.52 | 0.87 | 0.46 | 1.08 | 0.02 | 0.01 |
| sample-only | 6 | boom and bust | 5017 | 526 | 76.69 | 0.98 | 0.93 | 0.19 | 0.00 | 0.04 |
| sample-only | 7 | flourishing on dying land | 3277 | 1657 | 181 | 0.48 | 0.00 | 2.15 | 0.01 | 0.07 |
| sample-only | 8 | boom and bust | 5014 | 920 | 117 | 0.89 | 0.34 | 1.59 | 0.02 | 0.00 |
| sample-only | 9 | flourishing | 3901 | 1297 | 107 | 0.81 | 0.91 | 0.02 | 0.03 | 0.01 |
| sample-only | 10 | collapsed | 11157 | 434 | 55.77 | 0.99 | 0.88 | 0.21 | 0.05 | 0.11 |
| sample-only | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| sample-only | 12 | surviving | 2339 | 640 | 47.94 | 0.70 | 0.12 | 2.58 | 0.16 | 0.01 |
| sample-only | 13 | boom and bust | 3882 | 1430 | 101 | 0.84 | 0.71 | 0.98 | 0.11 | 0.01 |
| sample-only | 14 | flourishing | 1870 | 744 | 99.27 | 0.57 | 0.01 | 2.39 | 0.04 | 0.04 |
| sample-only | 15 | boom and bust | 5215 | 826 | 18.30 | 0.99 | 0.97 | 0.01 | 0.05 | 0.03 |
| sample-only | 16 | flourishing | 5869 | 1208 | 71.22 | 0.99 | 0.91 | 1.06 | 0.02 | 0.04 |
| gradient | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 2 | collapsed | 1000 | 35.00 | 2.17 | 1.00 | 1.00 | 0.03 | 0.00 | 0.02 |
| gradient | 3 | flourishing | 1984 | 469 | 92.19 | 0.53 | 0.05 | 1.97 | 0.00 | 0.00 |
| gradient | 4 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 5 | boom and bust | 1000 | 200 | 4.73 | 1.00 | 0.94 | 0.36 | 0.00 | 0.01 |
| gradient | 6 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 7 | boom and bust | 1149 | 212 | 25.14 | 0.81 | 0.14 | 0.18 | 0.03 | 0.06 |
| gradient | 8 | surviving | 2784 | 1330 | 31.58 | 0.96 | 0.93 | 0.17 | 0.02 | 0.00 |
| gradient | 9 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 10 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 12 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 13 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 14 | surviving | 1342 | 628 | 72.85 | 0.78 | 0.06 | 1.53 | 0.00 | 0.00 |
| gradient | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient | 16 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 1 | boom and bust | 1336 | 606 | 4.22 | 0.98 | 0.04 | 0.17 | 0.03 | 0.01 |
| gradient-social | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 3 | surviving | 1589 | 546 | 38.09 | 0.72 | 0.04 | 2.31 | 0.00 | 0.00 |
| gradient-social | 4 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 5 | boom and bust | 2872 | 336 | 166 | 0.52 | 0.63 | 2.25 | 0.04 | 0.01 |
| gradient-social | 6 | surviving | 1284 | 792 | 25.00 | 0.96 | 0.69 | 0.27 | 0.02 | 0.02 |
| gradient-social | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 8 | surviving | 1000 | 499 | 10.45 | 1.00 | 0.82 | 0.06 | 0.03 | 0.02 |
| gradient-social | 9 | surviving | 1065 | 666 | 47.39 | 0.92 | 0.02 | 2.08 | 0.03 | 0.03 |
| gradient-social | 10 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 11 | flourishing | 1917 | 855 | 66.39 | 0.87 | 0.50 | 1.44 | 0.07 | 0.00 |
| gradient-social | 12 | surviving | 1834 | 820 | 36.18 | 0.84 | 0.02 | 1.81 | 0.00 | 0.07 |
| gradient-social | 13 | boom and bust | 2001 | 616 | 89.74 | 0.72 | 0.00 | 3.25 | 0.01 | 0.10 |
| gradient-social | 14 | flourishing | 1874 | 1140 | 83.96 | 0.76 | 0.23 | 1.50 | 0.00 | 0.02 |
| gradient-social | 15 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-social | 16 | boom and bust | 3690 | 859 | 55.24 | 0.83 | 0.90 | 0.71 | 0.04 | 0.04 |
| gradient-slow | 1 | flourishing on dying land | 2557 | 698 | 94.49 | 0.36 | 0.02 | 3.13 | 0.01 | 0.06 |
| gradient-slow | 2 | surviving | 5408 | 2370 | 15.29 | 0.94 | 0.97 | 0.00 | 0.06 | 0.01 |
| gradient-slow | 3 | flourishing | 4604 | 852 | 119 | 0.92 | 0.99 | 0.01 | 0.06 | 0.05 |
| gradient-slow | 4 | surviving | 2733 | 1591 | 37.62 | 0.98 | 0.92 | 0.12 | 0.02 | 0.05 |
| gradient-slow | 5 | fallen | 5540 | 608 | 54.10 | 0.62 | 0.18 | 1.41 | 0.00 | 0.01 |
| gradient-slow | 6 | surviving | 3591 | 536 | 45.48 | 0.87 | 0.35 | 2.23 | 0.13 | 0.00 |
| gradient-slow | 7 | collapsed | 5324 | 457 | 115 | 0.93 | 0.89 | 0.02 | 0.04 | 0.05 |
| gradient-slow | 8 | flourishing | 3345 | 765 | 108 | 0.94 | 0.58 | 0.91 | 0.01 | 0.00 |
| gradient-slow | 9 | boom and bust | 5231 | 787 | 65.08 | 0.89 | 0.49 | 0.45 | 0.02 | 0.02 |
| gradient-slow | 10 | boom and bust | 9135 | 936 | 20.79 | 0.96 | 0.87 | 0.19 | 0.00 | 0.02 |
| gradient-slow | 11 | flourishing | 2275 | 1760 | 103 | 0.76 | 0.01 | 2.14 | 0.02 | 0.03 |
| gradient-slow | 12 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| gradient-slow | 13 | collapsed | 3424 | 287 | 63.97 | 0.73 | 0.43 | 2.10 | 0.11 | 0.00 |
| gradient-slow | 14 | flourishing | 2047 | 625 | 129 | 0.64 | 0.11 | 2.05 | 0.00 | 0.03 |
| gradient-slow | 15 | surviving | 2245 | 818 | 44.01 | 0.99 | 0.96 | 0.40 | 0.05 | 0.00 |
| gradient-slow | 16 | flourishing | 3942 | 860 | 64.41 | 0.98 | 0.95 | 0.89 | 0.06 | 0.04 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
