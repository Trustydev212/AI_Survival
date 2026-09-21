# Đông người thì phát minh nhiều hơn không?

Cùng bản đồ, xuất phát 1.000 người mặc định, so với 300 và 3.000.

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **few**: `--agents 300` · 16 thế giới
- **many**: `--agents 3000` · 16 thế giới

## Kết cục

| nhánh | boom and bust | boom and bust on dying land | collapsed | extinct | flourishing | flourishing on dying land | surviving | tốt |
|---|---|---|---|---|---|---|---|---|
| default | 4 | 0 | 1 | 2 | 9 | 0 | 0 | 9/16 |
| few | 6 | 1 | 0 | 5 | 2 | 0 | 2 | 4/16 |
| many | 3 | 0 | 3 | 1 | 8 | 1 | 0 | 8/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | few | many |
|---|---|---|---|
| peak_pop | 3624 | 1678 | 4750 |
| final_pop | 914 | 616 | 1098 |
| innovations | 124 | 38.50 | 128 |
| mean_known | 36.74 | 15.96 | 66.76 |
| soil_health | 0.98 | 1.00 | 0.94 |
| lived_soil | 0.96 | 0.99 | 0.95 |
| settled_share | 0.68 | 0.83 | 0.65 |
| obedience | 0.39 | 0.10 | 0.31 |
| breed_rate | 6.42 | 3.65 | 6.78 |
| swing | 3.39 | 4.64 | 3.31 |
| final_level | 6.50 | 4.00 | 7.00 |
| plastic | 0.01 | 0.01 | 0.02 |
| signal_mi | 0.02 | 0.01 | 0.01 |
| signal_meaning | 0.02 | – | – |
| things_per_head | 0.47 | 0.05 | 0.73 |
| equipped_share | 0.23 | 0.03 | 0.32 |
| crafts | 93.50 | 26.00 | 106 |
| learn_rate | 2.08 | 1.48 | 3.58 |
| loudness | 1.51 | 1.54 | 1.53 |
| hunts | 0.00 | 0.00 | 0.00 |
| division_of_labour | 0.49 | – | – |

## few so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -530 | [-2428, 1578] | chưa rõ |
| final_pop | -187 | [-786, 464] | chưa rõ |
| innovations | -39.50 | [-70.38, -6.38] | khác 0 |
| mean_known | -18.85 | [-43.55, 7.17] | chưa rõ |
| soil_health | 0.04 | [-0.08, 0.15] | chưa rõ |
| lived_soil | 0.04 | [-0.06, 0.13] | chưa rõ |
| settled_share | -0.01 | [-0.30, 0.27] | chưa rõ |
| obedience | -0.14 | [-0.39, 0.11] | chưa rõ |
| breed_rate | -2.55 | [-5.64, 0.64] | chưa rõ |
| swing | 0.46 | [-2.45, 3.05] | chưa rõ |
| final_level | -1.56 | [-3.50, 0.38] | chưa rõ |
| plastic | 0.00 | [-0.00, 0.01] | chưa rõ |
| signal_mi | -0.03 | [-0.06, -0.00] | khác 0 |
| things_per_head | -0.46 | [-0.98, 0.10] | chưa rõ |
| equipped_share | -0.19 | [-0.36, -0.00] | khác 0 |
| crafts | -37.69 | [-66.06, -6.69] | khác 0 |
| learn_rate | -0.38 | [-1.89, 1.14] | chưa rõ |
| loudness | -0.24 | [-0.72, 0.22] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |

## many so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 3466 | [1090, 5988] | khác 0 |
| final_pop | 1066 | [41.38, 2338] | khác 0 |
| innovations | 23.81 | [-3.88, 50.75] | chưa rõ |
| mean_known | 21.97 | [-3.45, 46.23] | chưa rõ |
| soil_health | -0.02 | [-0.14, 0.09] | chưa rõ |
| lived_soil | -0.03 | [-0.13, 0.07] | chưa rõ |
| settled_share | -0.02 | [-0.28, 0.25] | chưa rõ |
| obedience | -0.10 | [-0.32, 0.12] | chưa rõ |
| breed_rate | 0.77 | [-2.21, 3.49] | chưa rõ |
| swing | 0.19 | [-3.27, 4.48] | chưa rõ |
| final_level | 1.19 | [-0.31, 2.62] | chưa rõ |
| plastic | 0.01 | [0.00, 0.01] | khác 0 |
| signal_mi | -0.02 | [-0.05, 0.00] | chưa rõ |
| things_per_head | 0.33 | [-0.36, 1.03] | chưa rõ |
| equipped_share | 0.09 | [-0.12, 0.30] | chưa rõ |
| crafts | 17.12 | [-10.50, 43.44] | chưa rõ |
| learn_rate | 0.91 | [-0.59, 2.32] | chưa rõ |
| loudness | 0.09 | [-0.24, 0.43] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 993 | 1886 | 914 |
| few | 58.50 | 86.50 | 290 | 721 | 616 |
| many | 1011 | 2868 | 868 | 2007 | 1098 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.55 | 68.84 | 45.31 |
| few | 1.24 | 2.64 | 19.59 | 29.63 | 28.47 |
| many | 7.78 | 42.15 | 55.01 | 81.58 | 68.86 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| few | 0.01 | 0.01 | 0.01 | 0.02 | 0.02 |
| many | 0.01 | 0.02 | 0.01 | 0.02 | 0.02 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.04 | 0.06 | 0.04 | 0.03 | 0.03 |
| few | 0.00 | 0.02 | 0.04 | 0.03 | 0.01 |
| many | 0.04 | 0.03 | 0.03 | 0.03 | 0.01 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.03 | 0.05 | 0.03 |
| few | – | – | – | – | – |
| many | – | – | – | – | – |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.76 | 0.92 | 0.97 |
| few | 0.44 | 0.13 | 0.20 | 0.17 | 0.19 |
| many | 0.79 | 0.70 | 0.63 | 0.73 | 0.89 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.97 |
| few | 1.00 | 1.00 | 0.96 | 0.96 | 1.00 |
| many | 0.99 | 0.96 | 0.93 | 0.85 | 0.94 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.68 | 0.75 | 0.69 | 0.76 |
| few | 0.89 | 0.69 | 0.73 | 0.76 | 0.91 |
| many | 0.64 | 0.69 | 0.79 | 0.78 | 0.73 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.24 | 3.05 |
| few | 2.94 | 3.48 | 3.36 | 3.25 | 3.42 |
| many | 3.28 | 3.21 | 3.29 | 3.87 | 3.85 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.58 | 1.55 |
| few | 1.55 | 1.58 | 1.57 | 1.56 | 1.65 |
| many | 1.61 | 1.58 | 1.54 | 1.48 | 1.55 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.72 | 0.55 | 0.54 | 0.50 | 0.51 |
| few | – | – | – | – | – |
| many | – | – | – | – | – |

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
| few | 1 | extinct | 300 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| few | 2 | boom and bust | 4201 | 828 | 26.50 | 1.00 | 0.84 | 0.08 | 0.06 | – |
| few | 3 | boom and bust | 497 | 404 | 5.41 | 1.00 | 0.92 | 0.27 | 0.01 | – |
| few | 4 | extinct | 300 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| few | 5 | flourishing | 2858 | 1037 | 27.47 | 1.00 | 0.94 | 0.03 | 0.07 | – |
| few | 6 | flourishing | 4723 | 3659 | 102 | 0.55 | 0.02 | 2.68 | 0.03 | – |
| few | 7 | boom and bust on dying land | 3653 | 1778 | 105 | 0.49 | 0.52 | 1.85 | 0.06 | – |
| few | 8 | extinct | 300 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| few | 9 | boom and bust | 4644 | 894 | 28.47 | 0.82 | 0.93 | 0.07 | 0.01 | – |
| few | 10 | extinct | 300 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| few | 11 | extinct | 300 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| few | 12 | boom and bust | 12611 | 873 | 44.40 | 1.00 | 0.91 | 0.30 | 0.01 | – |
| few | 13 | surviving | 300 | 195 | 5.00 | 1.00 | 0.98 | 0.01 | 0.01 | – |
| few | 14 | boom and bust | 4142 | 977 | 81.30 | 0.92 | 0.85 | 0.22 | 0.00 | – |
| few | 15 | surviving | 390 | 321 | 4.91 | 1.00 | 0.97 | 0.00 | 0.00 | – |
| few | 16 | boom and bust | 6433 | 2534 | 34.70 | 0.96 | 0.82 | 0.19 | 0.01 | – |
| many | 1 | extinct | 3000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| many | 2 | boom and bust | 3306 | 2812 | 108 | 0.82 | 0.08 | 3.77 | 0.01 | – |
| many | 3 | collapsed | 18322 | 1369 | 68.86 | 0.94 | 0.73 | 0.56 | 0.01 | – |
| many | 4 | flourishing | 6056 | 2853 | 101 | 0.99 | 0.88 | 0.57 | 0.02 | – |
| many | 5 | flourishing | 8119 | 4404 | 112 | 0.95 | 0.93 | 0.25 | 0.03 | – |
| many | 6 | flourishing | 7537 | 3461 | 118 | 0.96 | 0.91 | 0.39 | 0.02 | – |
| many | 7 | collapsed | 3920 | 391 | 78.57 | 0.99 | 0.97 | 0.12 | 0.01 | – |
| many | 8 | flourishing | 7876 | 2227 | 32.37 | 1.00 | 0.94 | 0.11 | 0.01 | – |
| many | 9 | flourishing on dying land | 3072 | 826 | 76.43 | 0.47 | 0.28 | 1.84 | 0.01 | – |
| many | 10 | flourishing | 16781 | 9350 | 122 | 0.88 | 0.58 | 1.82 | 0.02 | – |
| many | 11 | boom and bust | 4632 | 814 | 43.37 | 0.57 | 0.09 | 1.54 | 0.08 | – |
| many | 12 | flourishing | 4867 | 2128 | 62.56 | 1.00 | 0.92 | 0.98 | 0.05 | – |
| many | 13 | boom and bust | 4326 | 716 | 24.53 | 0.76 | 0.23 | 0.89 | 0.00 | – |
| many | 14 | flourishing | 4245 | 818 | 64.66 | 0.76 | 0.14 | 2.51 | 0.01 | – |
| many | 15 | flourishing | 3000 | 587 | 53.66 | 0.67 | 0.02 | 2.88 | 0.01 | – |
| many | 16 | collapsed | 10821 | 793 | 52.19 | 0.98 | 0.88 | 0.08 | 0.04 | – |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
