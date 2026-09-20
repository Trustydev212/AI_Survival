# Học trong đời có đáng không?

Não có phần dẻo học theo phần thưởng (mặc định), so với tắt hẳn học trong đời và với học nhanh gấp ba.

## Cách chạy

- **default**: `(mặc định)` · 8 thế giới
- **no-learning**: `--learn-scale 0` · 8 thế giới
- **fast-learning**: `--learn-scale 3` · 8 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | flourishing | surviving | tốt |
|---|---|---|---|---|---|---|
| default | 4 | 1 | 0 | 3 | 0 | 3/8 |
| no-learning | 5 | 1 | 0 | 2 | 0 | 2/8 |
| fast-learning | 3 | 0 | 1 | 3 | 1 | 4/8 |

## Trung vị mỗi chỉ số

| chỉ số | default | no-learning | fast-learning |
|---|---|---|---|
| peak_pop | 4460 | 4282 | 4980 |
| final_pop | 1156 | 664 | 714 |
| innovations | 89.50 | 63.50 | 83.50 |
| mean_known | 33.60 | 31.45 | 40.43 |
| soil_health | 0.96 | 0.99 | 1.00 |
| lived_soil | 0.96 | 0.97 | 0.99 |
| settled_share | 0.84 | 0.91 | 0.90 |
| obedience | 0.22 | 0.42 | 0.17 |
| breed_rate | 5.71 | 3.02 | 7.71 |
| swing | 5.74 | 4.50 | 4.01 |
| final_level | 7.00 | 6.50 | 7.00 |
| plastic | 0.01 | 0.00 | 0.02 |
| signal_mi | 0.01 | 0.01 | 0.02 |
| things_per_head | 0.39 | 0.21 | 0.19 |
| equipped_share | 0.18 | 0.13 | 0.12 |
| crafts | 61.50 | 47.00 | 49.00 |

## no-learning so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -700 | [-3399, 2015] | chưa rõ |
| final_pop | -716 | [-1670, 174] | chưa rõ |
| innovations | -16.62 | [-51.88, 19.25] | chưa rõ |
| mean_known | -14.11 | [-44.87, 15.50] | chưa rõ |
| soil_health | 0.00 | [-0.07, 0.06] | chưa rõ |
| lived_soil | -0.01 | [-0.07, 0.05] | chưa rõ |
| settled_share | 0.14 | [-0.05, 0.38] | chưa rõ |
| obedience | 0.11 | [-0.19, 0.40] | chưa rõ |
| breed_rate | -2.32 | [-4.75, 0.12] | chưa rõ |
| swing | -3.72 | [-11.27, 1.17] | chưa rõ |
| final_level | -0.88 | [-2.62, 0.75] | chưa rõ |
| plastic | -0.01 | [-0.01, -0.01] | khác 0 |
| signal_mi | 0.02 | [-0.01, 0.08] | chưa rõ |
| things_per_head | -0.33 | [-0.70, -0.01] | khác 0 |
| equipped_share | -0.15 | [-0.31, -0.00] | khác 0 |
| crafts | -16.12 | [-45.25, 13.50] | chưa rõ |

## fast-learning so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -61.88 | [-3204, 3049] | chưa rõ |
| final_pop | -350 | [-1606, 1040] | chưa rõ |
| innovations | -8.75 | [-50.88, 31.25] | chưa rõ |
| mean_known | -3.18 | [-40.02, 30.88] | chưa rõ |
| soil_health | -0.00 | [-0.09, 0.07] | chưa rõ |
| lived_soil | 0.00 | [-0.08, 0.07] | chưa rõ |
| settled_share | 0.00 | [-0.33, 0.31] | chưa rõ |
| obedience | -0.13 | [-0.40, 0.10] | chưa rõ |
| breed_rate | 0.93 | [-2.38, 3.72] | chưa rõ |
| swing | -4.01 | [-11.61, 1.23] | chưa rõ |
| final_level | -1.25 | [-3.38, 0.62] | chưa rõ |
| plastic | 0.02 | [0.00, 0.04] | khác 0 |
| signal_mi | 0.01 | [-0.01, 0.03] | chưa rõ |
| things_per_head | -0.12 | [-0.63, 0.39] | chưa rõ |
| equipped_share | -0.08 | [-0.29, 0.14] | chưa rõ |
| crafts | -5.50 | [-42.62, 30.25] | chưa rõ |

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
| no-learning | 1 | boom and bust | 4471 | 337 | 29.15 | 0.98 | 0.88 | 0.15 | 0.01 |
| no-learning | 2 | boom and bust | 1993 | 307 | 20.11 | 0.98 | 0.95 | 0.11 | 0.21 |
| no-learning | 3 | collapsed | 1000 | 21.00 | 10.90 | 1.00 | 1.00 | 0.10 | 0.00 |
| no-learning | 4 | flourishing | 5768 | 992 | 54.89 | 1.00 | 0.95 | 0.28 | 0.04 |
| no-learning | 5 | boom and bust | 4094 | 1541 | 99.48 | 0.81 | 0.82 | 0.51 | 0.01 |
| no-learning | 6 | boom and bust | 9364 | 2015 | 40.53 | 1.00 | 0.93 | 0.02 | 0.03 |
| no-learning | 7 | boom and bust | 1000 | 91.00 | 2.93 | 1.00 | 0.76 | 0.36 | 0.00 |
| no-learning | 8 | flourishing | 7914 | 1420 | 33.76 | 0.81 | 0.66 | 0.27 | 0.00 |
| fast-learning | 1 | boom and bust | 4369 | 694 | 72.16 | 0.70 | 0.31 | 1.35 | 0.02 |
| fast-learning | 2 | flourishing | 5590 | 734 | 31.24 | 1.00 | 0.93 | 0.01 | 0.06 |
| fast-learning | 3 | flourishing | 1000 | 344 | 10.43 | 1.00 | 0.98 | 0.01 | 0.01 |
| fast-learning | 4 | flourishing | 7356 | 1731 | 101 | 1.00 | 0.95 | 0.39 | 0.04 |
| fast-learning | 5 | boom and bust | 10452 | 4967 | 107 | 0.85 | 0.81 | 1.39 | 0.02 |
| fast-learning | 6 | surviving | 1000 | 39.00 | 8.15 | 1.00 | 1.00 | 0.23 | 0.06 |
| fast-learning | 7 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 |
| fast-learning | 8 | boom and bust | 9946 | 1144 | 49.62 | 1.00 | 0.87 | 0.14 | 0.00 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
