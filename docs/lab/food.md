# Thức ăn có phải là cái tạo nhịp không?

Mục 15 và thí nghiệm pace: xã hội dao động, và đất không phải thủ phạm. Tương quan trễ trên bốn thế giới dài chỉ ra thức ăn: dân số đông thì 500 tick sau thức ăn cạn, r = -0,79. Đó là vòng tiêu thụ - tài nguyên cổ điển có độ trễ. Nếu đúng thì tốc độ mọc lại của thức ăn phải đổi được nhịp: mọc nhanh thì đuổi kịp và êm, mọc chậm thì vọt và sập. Mọc lại mặc định 0,08 mỗi tick; hai nhánh kia gấp đôi và một nửa. 60.000 tick.

_Đo trên thế giới phiên bản **v5** (xem sim/src/version.rs). Kết quả đo trên phiên bản khác không so được với bảng này._

## Cách chạy

- **default**: `--hear-strangers 1` · 4 thế giới
- **fast-regrow**: `--regrow 0.16` · 4 thế giới
- **slow-regrow**: `--regrow 0.04` · 4 thế giới

## Kết cục

| nhánh | boom and bust | extinct | fallen | flourishing | flourishing on dying land | tốt |
|---|---|---|---|---|---|---|
| default | 2 | 1 | 0 | 1 | 0 | 1/4 |
| fast-regrow | 1 | 1 | 1 | 1 | 0 | 1/4 |
| slow-regrow | 1 | 1 | 0 | 0 | 2 | 0/4 |

## Trung vị mỗi chỉ số

| chỉ số | default | fast-regrow | slow-regrow |
|---|---|---|---|
| peak_pop | 2679 | 6677 | 1344 |
| final_pop | 687 | 1761 | 486 |
| innovations | 206 | 266 | 182 |
| mean_known | 43.81 | 27.86 | 54.29 |
| soil_health | 0.95 | 0.82 | 0.64 |
| lived_soil | 0.94 | 0.83 | 0.65 |
| settled_share | 0.42 | 0.70 | 0.04 |
| obedience | 0.35 | 0.31 | 0.02 |
| breed_rate | 6.76 | 13.82 | 6.96 |
| swing | 9.54 | 4.32 | 6.67 |
| final_level | 2.50 | 1.50 | 2.00 |
| plastic | 0.01 | 0.01 | 0.01 |
| signal_mi | 0.03 | 0.02 | 0.01 |
| signal_meaning | 0.03 | 0.02 | 0.10 |
| things_per_head | 0.24 | 0.12 | 1.36 |
| equipped_share | 0.15 | 0.07 | 0.45 |
| crafts | 144 | 89.50 | 143 |
| learn_rate | 1.60 | 1.37 | 1.91 |
| loudness | 1.65 | 1.69 | 1.59 |
| hunts | 0.00 | 0.00 | 0.00 |
| division_of_labour | 0.45 | 0.45 | 0.37 |

## fast-regrow so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 3475 | [-306, 7080] | chưa rõ |
| final_pop | 782 | [-225, 1638] | chưa rõ |
| innovations | 17.25 | [-156, 177] | chưa rõ |
| mean_known | -12.36 | [-50.07, 26.90] | chưa rõ |
| soil_health | -0.06 | [-0.24, 0.14] | chưa rõ |
| lived_soil | -0.06 | [-0.22, 0.13] | chưa rõ |
| settled_share | 0.10 | [-0.39, 0.54] | chưa rõ |
| obedience | -0.07 | [-0.59, 0.50] | chưa rõ |
| breed_rate | 6.16 | [0.12, 11.94] | khác 0 |
| swing | -6.15 | [-12.72, -0.46] | khác 0 |
| final_level | -0.50 | [-2.00, 1.25] | chưa rõ |
| plastic | -0.00 | [-0.01, 0.01] | chưa rõ |
| signal_mi | -0.00 | [-0.04, 0.04] | chưa rõ |
| signal_meaning | -0.01 | [-0.03, 0.02] | chưa rõ |
| things_per_head | -0.39 | [-1.11, 0.11] | chưa rõ |
| equipped_share | -0.15 | [-0.43, 0.07] | chưa rõ |
| crafts | -53.50 | [-180, 68.50] | chưa rõ |
| learn_rate | -0.10 | [-2.04, 1.80] | chưa rõ |
| loudness | 0.03 | [-0.86, 0.92] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.04 | [-0.34, 0.28] | chưa rõ |

## slow-regrow so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -785 | [-2385, 998] | chưa rõ |
| final_pop | -256 | [-723, 238] | chưa rõ |
| innovations | -16.75 | [-200, 163] | chưa rõ |
| mean_known | 12.82 | [-42.66, 69.43] | chưa rõ |
| soil_health | -0.27 | [-0.68, 0.12] | chưa rõ |
| lived_soil | -0.24 | [-0.62, 0.12] | chưa rõ |
| settled_share | -0.20 | [-0.68, 0.33] | chưa rõ |
| obedience | -0.15 | [-0.69, 0.50] | chưa rõ |
| breed_rate | -0.06 | [-6.27, 6.26] | chưa rõ |
| swing | -2.90 | [-11.36, 4.88] | chưa rõ |
| final_level | 0.00 | [-1.75, 2.00] | chưa rõ |
| plastic | 0.01 | [-0.01, 0.02] | chưa rõ |
| signal_mi | -0.01 | [-0.04, 0.02] | chưa rõ |
| signal_meaning | 0.06 | [0.00, 0.10] | khác 0 |
| things_per_head | 0.97 | [-0.51, 2.54] | chưa rõ |
| equipped_share | 0.22 | [-0.29, 0.72] | chưa rõ |
| crafts | -2.25 | [-165, 160] | chưa rõ |
| learn_rate | 0.93 | [-1.77, 3.95] | chưa rõ |
| loudness | -0.05 | [-0.90, 0.82] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.03 | [-0.36, 0.33] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 284 | 172 | 387 | 1202 | 988 |
| fast-regrow | 732 | 3517 | 1231 | 2378 | 1004 |
| slow-regrow | 130 | 356 | 266 | 501 | 276 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 5.29 | 6.81 | 6.94 | 25.48 | 38.66 |
| fast-regrow | 8.60 | 26.98 | 40.94 | 75.94 | 59.95 |
| slow-regrow | 1.72 | 4.68 | 14.03 | 24.52 | 26.32 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.02 | 0.02 | 0.02 |
| fast-regrow | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| slow-regrow | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.04 | 0.06 | 0.04 |
| fast-regrow | 0.06 | 0.05 | 0.04 | 0.02 | 0.02 |
| slow-regrow | 0.01 | 0.03 | 0.07 | 0.03 | 0.02 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.07 | 0.05 | 0.02 | 0.06 | 0.08 |
| fast-regrow | 0.07 | 0.02 | 0.03 | 0.01 | 0.05 |
| slow-regrow | 0.03 | 0.03 | 0.08 | 0.09 | 0.04 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.44 | 0.68 | 1.00 | 1.28 | 0.95 |
| fast-regrow | 0.41 | 0.46 | 0.24 | 0.06 | 0.06 |
| slow-regrow | 0.74 | 0.96 | 1.14 | 1.50 | 1.54 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.99 | 0.95 | 0.93 |
| fast-regrow | 0.98 | 0.96 | 0.95 | 0.92 | 0.96 |
| slow-regrow | 1.00 | 0.99 | 0.96 | 0.93 | 0.87 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.91 | 0.78 | 0.53 | 0.58 | 0.54 |
| fast-regrow | 0.96 | 0.86 | 0.91 | 0.88 | 0.93 |
| slow-regrow | 0.81 | 0.53 | 0.45 | 0.51 | 0.51 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.55 | 2.68 | 3.69 | 3.88 | 3.59 |
| fast-regrow | 2.40 | 2.06 | 1.50 | 1.77 | 1.88 |
| slow-regrow | 2.76 | 3.01 | 3.01 | 3.02 | 3.06 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.58 | 1.50 | 1.56 | 1.58 | 1.61 |
| fast-regrow | 1.54 | 1.55 | 1.69 | 1.69 | 1.62 |
| slow-regrow | 1.58 | 1.53 | 1.52 | 1.53 | 1.55 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.74 | 0.66 | 0.52 | 0.49 | 0.49 |
| fast-regrow | 0.74 | 0.53 | 0.57 | 0.56 | 0.58 |
| slow-regrow | 0.76 | 0.36 | 0.36 | 0.36 | 0.39 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | hiểu (bit) | nghĩa (bit) |
|---|---|---|---|---|---|---|---|---|---|---|
| default | 1 | flourishing | 1806 | 666 | 84.19 | 0.63 | 0.26 | 1.57 | 0.06 | 0.02 |
| default | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 3 | boom and bust | 3552 | 708 | 59.92 | 0.93 | 0.59 | 0.44 | 0.04 | 0.04 |
| default | 4 | boom and bust | 4330 | 1235 | 27.70 | 0.97 | 0.93 | 0.04 | 0.02 | 0.05 |
| fast-regrow | 1 | fallen | 10233 | 1854 | 32.72 | 0.63 | 0.69 | 0.14 | 0.01 | 0.01 |
| fast-regrow | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| fast-regrow | 3 | flourishing | 4604 | 2215 | 66.64 | 0.78 | 0.70 | 0.09 | 0.07 | 0.03 |
| fast-regrow | 4 | boom and bust | 8750 | 1668 | 23.01 | 0.87 | 0.80 | 0.26 | 0.04 | 0.04 |
| slow-regrow | 1 | flourishing on dying land | 1457 | 615 | 94.92 | 0.17 | 0.00 | 2.71 | 0.01 | 0.13 |
| slow-regrow | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| slow-regrow | 3 | flourishing on dying land | 1231 | 539 | 115 | 0.28 | 0.07 | 3.21 | 0.01 | 0.09 |
| slow-regrow | 4 | boom and bust | 3861 | 432 | 13.67 | 1.00 | 0.92 | 0.02 | 0.06 | 0.11 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
