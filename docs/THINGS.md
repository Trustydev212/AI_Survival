# Mọi thứ các thế giới từng làm ra

Tổng hợp từ **324 thế giới** đã chạy, **26,673 lần phát minh**. Không món nào được viết sẵn:
công thức là thứ vật liệu cho phép, do các agent thử ra. Sinh bằng `python3 tools/things.py`.

## Họ làm ra cái gì

| loại | số lần được phát minh | phần |
|---|---|---|
| vũ khí | 8,522 | 32% |
| nơi trú | 7,427 | 28% |
| đồ đựng | 3,156 | 12% |
| thuyền | 3,072 | 12% |
| công cụ | 1,953 | 7% |
| lửa | 1,824 | 7% |
| giáp | 719 | 3% |

## Công thức hay gặp nhất

Cùng một công thức được tìm lại ở nhiều thế giới khác nhau, mỗi nơi một cái tên khác.

| công thức | số thế giới tìm ra |
|---|---|
| `sharpen(stone)` | 297 |
| `hollow(stone)` | 296 |
| `hollow(wood)` | 295 |
| `sharpen(wood)` | 293 |
| `hollow(sharpen(wood))` | 272 |
| `bind(wood, fibre)` | 268 |
| `hollow(clay)` | 268 |
| `sharpen(bone)` | 247 |
| `hollow(bone)` | 235 |
| `sharpen(bind(wood, fibre))` | 224 |
| `hollow(bind(wood, fibre))` | 223 |
| `bind(stone, fibre)` | 222 |

## Sâu nhất mỗi loại

Số tầng là số lần một thứ đã làm lại bị đem đi chế tiếp.

| loại | tầng | công thức | tick |
|---|---|---|---|
| nơi trú | 14 | `bind(fibre, bind(fibre, bind(fibre, sharpen(wood), hollow(sharpen(wood))), sharpen(bind(wood, stone, fibre))), bind(fibre, sharpen(bone), bind(fibre, sharpen(wood), hollow(sharpen(wood)))))` | 11,576 |
| công cụ | 10 | `bind(fibre, bind(fibre, hollow(sharpen(wood))), bind(fibre, bind(fibre, hollow(sharpen(wood))), bind(fibre, fibre, hollow(bone))))` | 18,839 |
| thuyền | 10 | `bind(fibre, bind(fibre, hollow(sharpen(wood))), bind(fibre, bind(fibre, hollow(sharpen(wood))), bind(fibre, hollow(bone))))` | 11,170 |
| vũ khí | 9 | `bind(fibre, bind(wood, fibre, hollow(sharpen(wood))), bind(fibre, bind(wood, fibre, fibre), bind(wood, fibre, bind(wood, fibre, sharpen(stone)))))` | 14,138 |
| đồ đựng | 6 | `bind(fibre, bind(fibre, hollow(wood)), bind(fibre, fire(hollow(clay))))` | 19,642 |
| lửa | 6 | `strike(stone, bind(fibre, bind(wood, fibre, fibre), bind(clay, sharpen(stone), bind(wood, fibre))))` | 8,056 |
| giáp | 1 | `bind(wood, fibre)` | 134 |

## Thao tác và vật liệu

| thao tác | lần dùng | | vật liệu | lần dùng |
|---|---|---|---|---|
| `bind` | 33,260 | | `fibre` | 18,330 |
| `hollow` | 16,506 | | `wood` | 16,634 |
| `sharpen` | 13,962 | | `stone` | 11,447 |
| `fire` | 2,136 | | `clay` | 9,401 |
| `strike` | 868 | | `bone` | 5,215 |
|  |  | | `ore` | 1,743 |

## Bao nhiêu tầng

| tầng | số phát minh |
|---|---|
| 1 | 6,481 |
| 2 | 10,265 |
| 3 | 4,927 |
| 4 | 2,325 |
| 5 | 1,387 |
| 6 | 720 |
| 7 | 313 |
| 8 | 158 |
| 9 | 65 |
| 10 | 16 |
| 11 | 10 |
| 12 | 4 |
| 13 | 1 |
| 14 | 1 |

_Phát minh sớm nhất từng ghi được: `sharpen(wood)` ở tick 1._
