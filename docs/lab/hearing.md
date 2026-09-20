# Nghe nhau có ích gì?

Tín hiệu giữa các agent được nghe (mặc định) so với mọi người điếc (đầu vào tín hiệu bằng 0).

## Cách chạy

- **default**: `(mặc định)` · 8 thế giới
- **deaf**: `--hear-scale 0` · 8 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | tốt |
|---|---|---|---|---|---|
| default | 4 | 1 | 0 | 3 | 3/8 |
| deaf | 3 | 1 | 1 | 3 | 3/8 |

## Trung vị mỗi chỉ số

| chỉ số | default | deaf |
|---|---|---|
| peak_pop | 4460 | 4492 |
| final_pop | 1156 | 908 |
| innovations | 89.50 | 112 |
| mean_known | 33.60 | 49.50 |
| soil_health | 0.96 | 0.96 |
| lived_soil | 0.96 | 0.96 |
| settled_share | 0.84 | 0.84 |
| obedience | 0.22 | 0.46 |
| breed_rate | 5.71 | 4.14 |
| swing | 5.74 | 5.11 |
| final_level | 7.00 | 7.00 |
| plastic | 0.01 | 0.01 |
| signal_mi | 0.01 | 0.01 |
| things_per_head | 0.39 | 0.50 |
| equipped_share | 0.18 | 0.30 |
| crafts | 61.50 | 68.50 |

## deaf so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -617 | [-3077, 1798] | chưa rõ |
| final_pop | -539 | [-1506, 382] | chưa rõ |
| innovations | 13.25 | [-24.38, 47.75] | chưa rõ |
| mean_known | 2.15 | [-29.64, 31.57] | chưa rõ |
| soil_health | -0.02 | [-0.10, 0.04] | chưa rõ |
| lived_soil | -0.01 | [-0.08, 0.04] | chưa rõ |
| settled_share | -0.04 | [-0.34, 0.27] | chưa rõ |
| obedience | 0.11 | [-0.21, 0.42] | chưa rõ |
| breed_rate | -2.54 | [-4.64, -0.72] | khác 0 |
| swing | -3.52 | [-11.26, 1.85] | chưa rõ |
| final_level | -0.12 | [-2.12, 1.50] | chưa rõ |
| plastic | 0.01 | [-0.00, 0.01] | chưa rõ |
| signal_mi | 0.00 | [-0.02, 0.02] | chưa rõ |
| things_per_head | 0.12 | [-0.42, 0.74] | chưa rõ |
| equipped_share | 0.05 | [-0.18, 0.28] | chưa rõ |
| crafts | 5.38 | [-26.25, 35.62] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 416 | 1412 | 1071 | 2092 | 1156 |
| deaf | 247 | 1046 | 1954 | 2414 | 908 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 4.27 | 12.06 | 39.24 | 53.74 | 33.60 |
| deaf | 3.09 | 8.93 | 36.74 | 44.53 | 50.08 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| deaf | 0.01 | 0.01 | 0.01 | 0.01 | 0.02 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.05 | 0.05 | 0.03 | 0.04 | 0.01 |
| deaf | 0.01 | 0.04 | 0.02 | 0.01 | 0.02 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.52 | 0.64 | 0.45 | 0.40 | 0.39 |
| deaf | 0.44 | 0.46 | 0.50 | 0.45 | 0.55 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 0.97 | 0.98 | 0.96 | 0.96 |
| deaf | 1.00 | 0.99 | 0.97 | 0.92 | 0.96 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.88 | 0.84 | 0.90 | 0.84 | 0.84 |
| deaf | 0.87 | 0.77 | 0.90 | 0.85 | 0.86 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | MI tín hiệu |
|---|---|---|---|---|---|---|---|---|---|
| default | 1 | boom and bust | 3653 | 553 | 32.13 | 0.88 | 0.93 | 0.10 | 0.00 |
| default | 2 | boom and bust | 5235 | 3412 | 25.72 | 0.94 | 0.76 | 0.27 | 0.06 |
| default | 3 | boom and bust | 3686 | 426 | 55.45 | 0.90 | 0.52 | 0.63 | 0.00 |
| default | 4 | flourishing | 7203 | 2412 | 111 | 1.00 | 0.93 | 0.50 | 0.03 |
| default | 5 | flourishing | 7415 | 2841 | 98.36 | 1.00 | 0.94 | 0.13 | 0.01 |
| default | 6 | boom and bust | 2075 | 1636 | 14.93 | 0.98 | 0.83 | 1.47 | 0.00 |
| default | 7 | flourishing | 1857 | 494 | 32.17 | 0.88 | 0.05 | 1.15 | 0.01 |
| default | 8 | collapsed | 10084 | 676 | 35.03 | 0.98 | 0.85 | 0.23 | 0.00 |
| deaf | 1 | flourishing | 5070 | 2735 | 109 | 0.94 | 0.81 | 1.06 | 0.00 |
| deaf | 2 | boom and bust | 3567 | 540 | 42.62 | 0.99 | 0.86 | 0.06 | 0.00 |
| deaf | 3 | flourishing | 3915 | 1418 | 41.67 | 0.97 | 0.78 | 0.45 | 0.03 |
| deaf | 4 | flourishing | 6775 | 992 | 48.93 | 0.96 | 0.96 | 0.93 | 0.01 |
| deaf | 5 | collapsed | 6860 | 1035 | 68.52 | 0.97 | 0.94 | 0.55 | 0.03 |
| deaf | 6 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| deaf | 7 | boom and bust | 1822 | 591 | 61.38 | 0.86 | 0.24 | 2.18 | 0.03 |
| deaf | 8 | boom and bust | 7260 | 824 | 50.08 | 0.72 | 0.91 | 0.21 | 0.02 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
