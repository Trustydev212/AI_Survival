# Có việc cần nhiều tay thì tiếng gọi có nghĩa không?

Không có bầy thú (mặc định) so với có bầy thú chỉ hạ được khi ít nhất hai người đánh gần như cùng lúc (--herd-density 3.5).

## Cách chạy

- **default**: `(mặc định)` · 16 thế giới
- **herds**: `--herd-density 3.5` · 16 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | flourishing on dying land | surviving | tốt |
|---|---|---|---|---|---|---|---|
| default | 4 | 1 | 2 | 9 | 0 | 0 | 9/16 |
| herds | 3 | 2 | 5 | 4 | 1 | 1 | 5/16 |

## Trung vị mỗi chỉ số

| chỉ số | default | herds |
|---|---|---|
| peak_pop | 3624 | 2545 |
| final_pop | 914 | 548 |
| innovations | 124 | 77.00 |
| mean_known | 36.74 | 38.34 |
| soil_health | 0.98 | 0.98 |
| lived_soil | 0.96 | 0.98 |
| settled_share | 0.68 | 0.15 |
| obedience | 0.39 | 0.06 |
| breed_rate | 6.42 | 3.01 |
| swing | 3.39 | 3.64 |
| final_level | 6.50 | 6.00 |
| plastic | 0.01 | 0.01 |
| signal_mi | 0.02 | 0.02 |
| signal_meaning | 0.02 | – |
| things_per_head | 0.47 | 0.09 |
| equipped_share | 0.23 | 0.05 |
| crafts | 93.50 | 58.50 |
| learn_rate | 2.08 | 2.56 |
| loudness | 1.51 | 1.50 |
| hunts | 0.00 | 0.00 |
| division_of_labour | 0.49 | – |

## herds so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 12.25 | [-1655, 1696] | chưa rõ |
| final_pop | -42.62 | [-750, 707] | chưa rõ |
| innovations | -15.38 | [-48.00, 18.06] | chưa rõ |
| mean_known | -5.53 | [-31.73, 20.53] | chưa rõ |
| soil_health | -0.03 | [-0.16, 0.10] | chưa rõ |
| lived_soil | -0.03 | [-0.14, 0.08] | chưa rõ |
| settled_share | -0.15 | [-0.43, 0.12] | chưa rõ |
| obedience | -0.20 | [-0.42, 0.02] | chưa rõ |
| breed_rate | -2.36 | [-5.56, 0.69] | chưa rõ |
| swing | -0.07 | [-3.47, 3.95] | chưa rõ |
| final_level | -1.00 | [-2.88, 0.88] | chưa rõ |
| plastic | -0.00 | [-0.01, 0.00] | chưa rõ |
| signal_mi | -0.01 | [-0.05, 0.02] | chưa rõ |
| things_per_head | -0.16 | [-0.74, 0.44] | chưa rõ |
| equipped_share | -0.07 | [-0.27, 0.14] | chưa rõ |
| crafts | -12.88 | [-43.56, 19.19] | chưa rõ |
| learn_rate | -0.28 | [-1.77, 1.20] | chưa rõ |
| loudness | -0.27 | [-0.71, 0.16] | chưa rõ |
| hunts | 0.81 | [0.12, 1.75] | khác 0 |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 162 | 280 | 993 | 1886 | 914 |
| herds | 218 | 514 | 756 | 1257 | 548 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 3.54 | 7.14 | 28.55 | 68.84 | 45.31 |
| herds | 3.58 | 12.06 | 57.11 | 84.16 | 55.09 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| herds | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.04 | 0.06 | 0.04 | 0.03 | 0.03 |
| herds | 0.04 | 0.01 | 0.04 | 0.02 | 0.03 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.03 | 0.05 | 0.03 |
| herds | – | – | – | – | – |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.49 | 0.38 | 0.76 | 0.92 | 0.97 |
| herds | 0.81 | 0.53 | 1.25 | 1.16 | 0.89 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.98 | 0.93 | 0.97 |
| herds | 1.00 | 0.99 | 0.90 | 0.85 | 0.84 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.85 | 0.68 | 0.75 | 0.69 | 0.76 |
| herds | 0.76 | 0.76 | 0.49 | 0.80 | 0.82 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.62 | 2.40 | 3.06 | 3.24 | 3.05 |
| herds | 2.99 | 3.13 | 4.00 | 3.06 | 3.93 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.57 | 1.50 | 1.54 | 1.58 | 1.55 |
| herds | 1.55 | 1.55 | 1.54 | 1.60 | 1.60 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.72 | 0.55 | 0.54 | 0.50 | 0.51 |
| herds | – | – | – | – | – |

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
| herds | 1 | flourishing on dying land | 2367 | 624 | 55.09 | 0.39 | 0.02 | 2.49 | 0.02 | – |
| herds | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| herds | 3 | boom and bust | 2723 | 565 | 88.22 | 0.64 | 0.57 | 1.34 | 0.05 | – |
| herds | 4 | boom and bust | 6584 | 1465 | 23.46 | 1.00 | 0.92 | 0.03 | 0.03 | – |
| herds | 5 | flourishing | 4350 | 1027 | 53.04 | 0.81 | 0.26 | 1.37 | 0.07 | – |
| herds | 6 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| herds | 7 | flourishing | 5063 | 4020 | 104 | 0.50 | 0.00 | 2.51 | 0.01 | – |
| herds | 8 | flourishing | 5172 | 4009 | 117 | 0.84 | 0.96 | 0.19 | 0.01 | – |
| herds | 9 | collapsed | 7511 | 531 | 54.26 | 0.89 | 0.82 | 0.12 | 0.04 | – |
| herds | 10 | boom and bust | 3595 | 1906 | 23.64 | 1.00 | 0.96 | 0.05 | 0.03 | – |
| herds | 11 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| herds | 12 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| herds | 13 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | – |
| herds | 14 | flourishing | 2268 | 384 | 71.50 | 0.64 | 0.05 | 1.41 | 0.00 | – |
| herds | 15 | surviving | 1000 | 244 | 4.75 | 1.00 | 1.00 | 0.00 | 0.11 | – |
| herds | 16 | collapsed | 8989 | 1035 | 82.26 | 0.97 | 0.82 | 0.89 | 0.13 | – |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
