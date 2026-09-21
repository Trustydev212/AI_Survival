# Đất có phải là cái tạo nhịp dao động không?

Mục 15: xã hội không tìm được cân bằng mà lắc, càng già càng mạnh. Nghi can là vòng phản hồi âm có độ trễ qua đất: hái nhiều thì đất mòn, mòn thì đói, đói thì chết bớt, chết bớt thì đất hồi. Nếu đúng thì làm đất mòn nhanh hơn phải lắc mạnh hơn, và làm đất bền hơn phải lắc êm hơn. 60.000 tick vì dao động chỉ lộ ra ở đường dài. Nhánh đối chứng ghi rõ giá trị mặc định để có thư mục riêng.

_Đo trên thế giới phiên bản **v5** (xem sim/src/version.rs). Kết quả đo trên phiên bản khác không so được với bảng này._

## Cách chạy

- **default**: `--soil-drain 0.001` · 4 thế giới
- **fragile**: `--soil-drain 0.003` · 4 thế giới
- **tough**: `--soil-drain 0.0001` · 4 thế giới

## Kết cục

| nhánh | boom and bust | collapsed | extinct | fallen | flourishing | tốt |
|---|---|---|---|---|---|---|
| default | 2 | 0 | 1 | 0 | 1 | 1/4 |
| fragile | 0 | 1 | 2 | 1 | 0 | 0/4 |
| tough | 3 | 0 | 1 | 0 | 0 | 0/4 |

## Trung vị mỗi chỉ số

| chỉ số | default | fragile | tough |
|---|---|---|---|
| peak_pop | 2679 | 1611 | 2940 |
| final_pop | 687 | 184 | 1152 |
| innovations | 206 | 103 | 186 |
| mean_known | 43.81 | 17.76 | 30.17 |
| soil_health | 0.95 | 0.69 | 1.00 |
| lived_soil | 0.94 | 0.71 | 1.00 |
| settled_share | 0.42 | 0.00 | 0.59 |
| obedience | 0.35 | 0.00 | 0.33 |
| breed_rate | 6.76 | 6.20 | 10.63 |
| swing | 9.54 | 3.55 | 6.89 |
| final_level | 2.50 | 0.50 | 1.50 |
| plastic | 0.01 | 0.00 | 0.01 |
| signal_mi | 0.03 | 0.00 | 0.01 |
| signal_meaning | 0.03 | 0.02 | 0.01 |
| things_per_head | 0.24 | 0.80 | 0.09 |
| equipped_share | 0.15 | 0.32 | 0.07 |
| crafts | 144 | 82.50 | 90.50 |
| learn_rate | 1.60 | 0.23 | 1.34 |
| loudness | 1.65 | 0.74 | 1.71 |
| hunts | 0.00 | 0.00 | 0.00 |
| division_of_labour | 0.45 | 0.18 | 0.44 |

## fragile so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | -190 | [-2310, 2256] | chưa rõ |
| final_pop | -459 | [-909, 36.00] | chưa rõ |
| innovations | -71.50 | [-240, 98.00] | chưa rõ |
| mean_known | -15.62 | [-57.55, 29.98] | chưa rõ |
| soil_health | -0.24 | [-0.63, 0.13] | chưa rõ |
| lived_soil | -0.22 | [-0.58, 0.13] | chưa rõ |
| settled_share | -0.39 | [-0.76, -0.04] | khác 0 |
| obedience | -0.16 | [-0.71, 0.50] | chưa rõ |
| breed_rate | -0.02 | [-7.65, 7.62] | chưa rõ |
| swing | -5.03 | [-12.30, 2.23] | chưa rõ |
| final_level | -1.25 | [-2.50, 0.25] | chưa rõ |
| plastic | 0.00 | [-0.01, 0.02] | chưa rõ |
| signal_mi | -0.03 | [-0.05, -0.00] | khác 0 |
| signal_meaning | -0.00 | [-0.03, 0.03] | chưa rõ |
| things_per_head | 0.38 | [-0.77, 1.38] | chưa rõ |
| equipped_share | 0.15 | [-0.31, 0.61] | chưa rõ |
| crafts | -55.00 | [-190, 81.50] | chưa rõ |
| learn_rate | 0.21 | [-2.50, 3.66] | chưa rõ |
| loudness | -0.51 | [-1.35, 0.67] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.20 | [-0.50, 0.13] | chưa rõ |

## tough so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 484 | [-1630, 2728] | chưa rõ |
| final_pop | 257 | [-440, 894] | chưa rõ |
| innovations | -20.50 | [-180, 142] | chưa rõ |
| mean_known | -14.25 | [-51.71, 23.44] | chưa rõ |
| soil_health | 0.12 | [0.01, 0.29] | khác 0 |
| lived_soil | 0.12 | [0.02, 0.27] | khác 0 |
| settled_share | 0.09 | [-0.40, 0.58] | chưa rõ |
| obedience | -0.04 | [-0.59, 0.53] | chưa rõ |
| breed_rate | 5.02 | [-3.14, 13.48] | chưa rõ |
| swing | 6.05 | [-9.01, 27.05] | chưa rõ |
| final_level | -0.50 | [-2.00, 1.25] | chưa rõ |
| plastic | 0.00 | [-0.01, 0.01] | chưa rõ |
| signal_mi | -0.01 | [-0.04, 0.04] | chưa rõ |
| signal_meaning | -0.02 | [-0.04, 0.01] | chưa rõ |
| things_per_head | -0.24 | [-0.96, 0.45] | chưa rõ |
| equipped_share | -0.06 | [-0.38, 0.28] | chưa rõ |
| crafts | -45.00 | [-176, 81.75] | chưa rõ |
| learn_rate | 0.78 | [-1.90, 3.97] | chưa rõ |
| loudness | 0.04 | [-0.85, 0.92] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | 0.01 | [-0.34, 0.37] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 284 | 172 | 387 | 1202 | 988 |
| fragile | 187 | 92.00 | 174 | 466 | 152 |
| tough | 190 | 153 | 207 | 817 | 454 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 5.29 | 6.81 | 6.94 | 25.48 | 38.66 |
| fragile | 2.90 | 2.80 | 16.85 | 42.81 | 46.75 |
| tough | 3.39 | 5.95 | 8.95 | 12.57 | 23.70 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.02 | 0.02 | 0.02 |
| fragile | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| tough | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.04 | 0.06 | 0.04 |
| fragile | 0.01 | 0.00 | 0.01 | 0.02 | 0.03 |
| tough | 0.04 | 0.08 | 0.11 | 0.06 | 0.09 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.07 | 0.05 | 0.02 | 0.06 | 0.08 |
| fragile | 0.09 | 0.02 | 0.02 | 0.05 | 0.03 |
| tough | 0.05 | 0.03 | 0.04 | 0.00 | 0.00 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.44 | 0.68 | 1.00 | 1.28 | 0.95 |
| fragile | 0.48 | 0.12 | 1.08 | 1.14 | 1.21 |
| tough | 0.23 | 0.25 | 0.24 | 0.22 | 0.48 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.99 | 0.95 | 0.93 |
| fragile | 0.98 | 0.98 | 0.73 | 0.52 | 0.56 |
| tough | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.91 | 0.78 | 0.53 | 0.58 | 0.54 |
| fragile | 0.95 | 0.91 | 0.48 | 0.49 | 0.47 |
| tough | 0.99 | 0.97 | 0.97 | 0.94 | 0.93 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.55 | 2.68 | 3.69 | 3.88 | 3.59 |
| fragile | 2.99 | 3.14 | 3.60 | 3.60 | 3.79 |
| tough | 2.03 | 2.16 | 2.36 | 2.23 | 2.28 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.58 | 1.50 | 1.56 | 1.58 | 1.61 |
| fragile | 1.57 | 1.52 | 1.39 | 1.44 | 1.47 |
| tough | 1.52 | 1.66 | 1.60 | 1.64 | 1.56 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.74 | 0.66 | 0.52 | 0.49 | 0.49 |
| fragile | 0.78 | 0.22 | 0.58 | 0.58 | 0.55 |
| tough | 0.81 | 0.67 | 0.80 | 0.74 | 0.65 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | hiểu (bit) | nghĩa (bit) |
|---|---|---|---|---|---|---|---|---|---|---|
| default | 1 | flourishing | 1806 | 666 | 84.19 | 0.63 | 0.26 | 1.57 | 0.06 | 0.02 |
| default | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 3 | boom and bust | 3552 | 708 | 59.92 | 0.93 | 0.59 | 0.44 | 0.04 | 0.04 |
| default | 4 | boom and bust | 4330 | 1235 | 27.70 | 0.97 | 0.93 | 0.04 | 0.02 | 0.05 |
| fragile | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| fragile | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| fragile | 3 | fallen | 2222 | 367 | 73.80 | 0.19 | 0.01 | 1.97 | 0.00 | 0.04 |
| fragile | 4 | collapsed | 5707 | 405 | 35.53 | 0.38 | 0.22 | 1.59 | 0.01 | 0.06 |
| tough | 1 | boom and bust | 3906 | 1335 | 21.06 | 1.00 | 0.78 | 0.11 | 0.08 | 0.00 |
| tough | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| tough | 3 | boom and bust | 1975 | 1313 | 54.49 | 1.00 | 0.40 | 0.91 | 0.02 | 0.01 |
| tough | 4 | boom and bust | 5745 | 990 | 39.29 | 1.00 | 0.97 | 0.08 | 0.00 | 0.03 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
