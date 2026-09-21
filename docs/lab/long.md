# Chạy dài gấp ba thì có gì xuất hiện muộn không?

60.000 tick thay vì 20.000. Ba câu hỏi trong một lần chạy: tiếng gọi có mang nghĩa khi có đủ thời gian không (mọi lần loé lên trước đây đều muộn), trần 512 phát minh có bị chạm không, và cái giá sớm của chế tác có đổi thành lợi ích muộn không. Nhánh bare bỏ hết khung xã hội, vì ở 8.000 tick nó có đuôi phân phối cao nhất về nghĩa. Nhánh đối chứng ghi rõ --hear-strangers 1, đúng bằng giá trị mặc định, chỉ để nó có thư mục riêng và không đè lên kết quả 20.000 tick.

_Đo trên thế giới phiên bản **v5** (xem sim/src/version.rs). Kết quả đo trên phiên bản khác không so được với bảng này._

## Cách chạy

- **default**: `--hear-strangers 1` · 4 thế giới
- **bare**: `--bare` · 4 thế giới

## Kết cục

| nhánh | boom and bust | boom and bust on dying land | collapsed | extinct | flourishing | tốt |
|---|---|---|---|---|---|---|
| default | 2 | 0 | 0 | 1 | 1 | 1/4 |
| bare | 0 | 1 | 1 | 2 | 0 | 0/4 |

## Trung vị mỗi chỉ số

| chỉ số | default | bare |
|---|---|---|
| peak_pop | 2679 | 2691 |
| final_pop | 687 | 338 |
| innovations | 206 | 70.50 |
| mean_known | 43.81 | 12.64 |
| soil_health | 0.95 | 0.99 |
| lived_soil | 0.94 | 0.99 |
| settled_share | 0.42 | 0.14 |
| obedience | 0.35 | 0.00 |
| breed_rate | 6.76 | 3.08 |
| swing | 9.54 | 14.21 |
| final_level | 2.50 | 0.50 |
| plastic | 0.01 | 0.00 |
| signal_mi | 0.03 | 0.00 |
| signal_meaning | 0.03 | 0.00 |
| things_per_head | 0.24 | 0.00 |
| equipped_share | 0.15 | 0.00 |
| crafts | 144 | 28.50 |
| learn_rate | 1.60 | 0.32 |
| loudness | 1.65 | 0.83 |
| hunts | 0.00 | 0.00 |
| division_of_labour | 0.45 | 0.19 |

## bare so với default: hiệu số trung bình và khoảng tin cậy 95% (bootstrap)

| chỉ số | hiệu số | KTC 95% | đọc |
|---|---|---|---|
| peak_pop | 928 | [-2068, 4166] | chưa rõ |
| final_pop | -244 | [-819, 373] | chưa rõ |
| innovations | -70.50 | [-235, 111] | chưa rõ |
| mean_known | -25.72 | [-61.14, 11.70] | chưa rõ |
| soil_health | -0.02 | [-0.29, 0.21] | chưa rõ |
| lived_soil | -0.01 | [-0.26, 0.20] | chưa rõ |
| settled_share | -0.15 | [-0.61, 0.38] | chưa rõ |
| obedience | -0.41 | [-0.83, 0.00] | chưa rõ |
| breed_rate | -0.48 | [-8.24, 8.13] | chưa rõ |
| swing | 7.38 | [-4.35, 20.84] | chưa rõ |
| final_level | -1.25 | [-2.50, 0.25] | chưa rõ |
| plastic | -0.00 | [-0.01, 0.01] | chưa rõ |
| signal_mi | -0.03 | [-0.05, -0.00] | khác 0 |
| signal_meaning | -0.02 | [-0.04, 0.01] | chưa rõ |
| things_per_head | -0.13 | [-0.91, 0.74] | chưa rõ |
| equipped_share | -0.08 | [-0.39, 0.30] | chưa rõ |
| crafts | -64.25 | [-185, 91.50] | chưa rõ |
| learn_rate | -0.33 | [-2.50, 2.22] | chưa rõ |
| loudness | -0.40 | [-1.30, 0.85] | chưa rõ |
| hunts | 0.00 | [0.00, 0.00] | chưa rõ |
| division_of_labour | -0.15 | [-0.49, 0.22] | chưa rõ |

## Theo thời gian (trung vị các thế giới; thế giới đã chết tính dân số 0)

**pop**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 284 | 172 | 387 | 1202 | 988 |
| bare | 151 | 168 | 348 | 1978 | 342 |

**mean_known**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 5.29 | 6.81 | 6.94 | 25.48 | 38.66 |
| bare | 2.51 | 2.99 | 3.00 | 78.72 | 51.77 |

**plastic**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.01 | 0.01 | 0.02 | 0.02 | 0.02 |
| bare | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |

**signal_mi**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.03 | 0.03 | 0.04 | 0.06 | 0.04 |
| bare | 0.03 | 0.02 | 0.04 | 0.07 | 0.05 |

**signal_meaning**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.07 | 0.05 | 0.02 | 0.06 | 0.08 |
| bare | 0.04 | 0.02 | 0.00 | 0.06 | 0.07 |

**things_per_head**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.44 | 0.68 | 1.00 | 1.28 | 0.95 |
| bare | 0.40 | 0.38 | 0.33 | 0.13 | 0.54 |

**soil_health**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.00 | 1.00 | 0.99 | 0.95 | 0.93 |
| bare | 1.00 | 1.00 | 1.00 | 0.85 | 0.93 |

**settled_share**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.91 | 0.78 | 0.53 | 0.58 | 0.54 |
| bare | 0.93 | 0.78 | 0.88 | 0.88 | 0.68 |

**learn_rate**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 2.55 | 2.68 | 3.69 | 3.88 | 3.59 |
| bare | 2.53 | 2.94 | 3.00 | 1.11 | 1.93 |

**loudness**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 1.58 | 1.50 | 1.56 | 1.58 | 1.61 |
| bare | 1.52 | 1.50 | 1.64 | 1.58 | 1.60 |

**division_of_labour**

| nhánh | tick 2500 | tick 5000 | tick 10000 | tick 15000 | tick 20000 |
|---|---|---|---|---|---|
| default | 0.74 | 0.66 | 0.52 | 0.49 | 0.49 |
| bare | 0.76 | 0.57 | 0.49 | 0.62 | 0.58 |

## Từng thế giới

| nhánh | seed | kết cục | đỉnh | cuối | biết | đất | ở yên | đồ/người | hiểu (bit) | nghĩa (bit) |
|---|---|---|---|---|---|---|---|---|---|---|
| default | 1 | flourishing | 1806 | 666 | 84.19 | 0.63 | 0.26 | 1.57 | 0.06 | 0.02 |
| default | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| default | 3 | boom and bust | 3552 | 708 | 59.92 | 0.93 | 0.59 | 0.44 | 0.04 | 0.04 |
| default | 4 | boom and bust | 4330 | 1235 | 27.70 | 0.97 | 0.93 | 0.04 | 0.02 | 0.05 |
| bare | 1 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| bare | 2 | extinct | 1000 | 0.00 | -0.00 | 1.00 | 0.00 | -0.00 | 0.00 | 0.00 |
| bare | 3 | boom and bust on dying land | 4382 | 677 | 43.67 | 0.47 | 0.28 | 1.51 | 0.00 | 0.00 |
| bare | 4 | collapsed | 8017 | 955 | 25.27 | 0.98 | 0.90 | 0.00 | 0.02 | 0.04 |

_Sinh bởi tools/lab.py. Kết cục: flourishing, surviving là tốt; boom and bust, collapsed, extinct là xấu._
