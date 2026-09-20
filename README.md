# AI Survival

Một sandbox lịch sử loài người, nơi toàn bộ cư dân là AI. Không có người chơi điều khiển.
Bạn tạo thế giới, thả các bộ lạc vào, rồi xem tiến hoá chọn ra chiến lược sinh tồn nào:
hái lượm, tích trữ, chia sẻ với họ hàng, hay đi cướp.

Repo này hiện chứa **bước 1: lõi mô phỏng headless** (`sim/`). Chưa có đồ hoạ engine,
chỉ có xuất ảnh PPM và CSV để nhìn nhanh và phân tích.

## Chạy thử

```bash
cd sim
cargo build --release
./target/release/sim --seed 3 --ticks 20000 --image-every 5000
./target/release/sim --help
```

1000 agent, 20.000 tick chạy khoảng 25 giây trên một nhân CPU (khoảng 800 tick/giây,
lúc cao điểm hơn 3000 agent). Không phụ thuộc crate ngoài.

Kết quả ghi vào `out/`:

- `stats_seed<N>.csv`: dân số, số dòng họ, Gini, số sinh, chết đói, bị giết, tấn công, chia sẻ, phân bố hành động, entropy chiến lược, theo từng cửa sổ thời gian.
- `frame_<tick>.ppm`: bản đồ. Xanh lá là thức ăn, đỏ là ruộng đang canh tác, chấm màu là agent với màu do gen quy định.
- `events_seed<N>.txt`: sử ký. Khám phá đầu tiên, dòng họ thống trị hoặc tuyệt chủng, nạn đói, chiến tranh, tầng lớp chiến binh xuất hiện, làng định cư đầu tiên.

Cùng seed luôn cho cùng kết quả, nên có thể replay và so sánh thí nghiệm.

## Cách thế giới vận hành

**Thế giới** là lưới ô cuộn tròn. Mỗi ô có độ màu mỡ cố định, sinh ra từ value noise
rồi ngưỡng hoá để đất tốt tụ thành từng vùng và khoảng 40% bản đồ gần như cằn cỗi.
Thức ăn mọc lại theo độ màu mỡ và theo mùa. Mùa đông giảm tốc độ mọc xuống 20%.

**Agent** có năng lượng, kho dự trữ, tuổi, dòng họ, kiến thức, và một bộ gen. Gen gồm trọng số
của một mạng thần kinh 24 input, 16 ẩn, 8 output (536 tham số) cộng một "màu" ba chiều.
Nhận diện họ hàng dựa trên khoảng cách màu.

**Mỗi tick**, agent nhìn thấy: năng lượng, tuổi, kho, thức ăn tại chỗ và gradient thức ăn,
agent gần nhất (hướng, khoảng cách, độ họ hàng, chênh lệch sức mạnh, kho của nó),
số hàng xóm là họ hàng và không họ hàng, mùa, mình vừa bị đánh hay chưa, mình biết công nghệ gì,
và ô đang đứng có phải ruộng không.
Não trả về hướng di chuyển, một cổng đi hay ở, và một trong năm hành động:

| Hành động | Tác dụng |
|---|---|
| gather | Lấy thức ăn từ ô đang đứng, dư thì cất vào kho |
| attack | Đánh agent gần nhất trong tầm. Ai khoẻ hơn dễ thắng. Thắng thì cướp kho và gây sát thương |
| share | Cho họ hàng gần nhất một phần kho |
| repro | Nếu đủ năng lượng, sinh con. Con thừa hưởng gen có đột biến |
| rest | Giảm tiêu hao năng lượng |

Không có hàm thưởng. Ai sinh được nhiều con thì gen của họ tồn tại. Đó là toàn bộ thuật toán học.
Khi dân số sụp dưới ngưỡng, vài agent gen ngẫu nhiên "nhập cư" để thí nghiệm không chết hẳn.

## Lớp văn hoá

Kiến thức không nằm trong gen. Trẻ sinh ra không biết gì và học từ người đứng cạnh,
họ hàng dạy dễ hơn người lạ. Vì vậy kiến thức có thể mất đi qua mùa đông khi dân số tan rã.

| Công nghệ | Khám phá khi | Tác dụng |
|---|---|---|
| tools | hái lượm | hái nhanh gấp 1,5 |
| farming | hái lượm trên đất tốt, đã có tools | đứng yên từ 5 tick trở lên thì vùng 3x3 quanh mình thành ruộng, mọc lại nhanh gấp 12, hái được cả 9 ô |
| weapons | tấn công, đã có tools | mạnh hơn khi đánh, cướp và gây sát thương gấp 1,5 |
| cooking | nghỉ ngơi, đã có tools | tiêu hao năng lượng giảm 25% |

Ruộng cần được chăm liên tục, bỏ đi là mất dần. Du mục đi ngang không tạo ra ruộng.
Cờ `--start-tech 15` cho bộ lạc khởi đầu biết hết mọi công nghệ, dùng để đối chứng.

## Đo đa dạng chiến lược

Dòng họ không nói lên cách sống: một dòng họ có thể chứa cả nông dân lẫn kẻ cướp.
Nên mỗi agent giữ một **hồ sơ hành vi**: tần suất năm hành động và mức di chuyển,
suy giảm theo hàm mũ, tức là thứ nó thực sự làm gần đây chứ không phải gen nói gì.

Mỗi cửa sổ thống kê, lõi chạy k-means xác định trên hồ sơ này, gộp các cụm gần nhau,
rồi báo:

- số cụm và nhãn mỗi cụm, ví dụ `gather71 attack19 nomadic74`
- **entropy chiến lược** và **số cách sống hiệu dụng** (2 mũ entropy). Về 1.0 nghĩa là cả xã hội đã hội tụ về một kiểu.
- của cải, tuổi trung bình và số dòng họ góp mặt trong mỗi cụm

Bảng cuối mỗi lần chạy liệt kê các cụm. Cột `strat` trong bảng theo tick là số cách sống hiệu dụng.

## Những gì đã quan sát được

Chạy 3 seed, cùng tham số mặc định, 20.000 tick:

| Seed | Kết cục | Tấn công / 2000 tick cuối | Chia sẻ | Ghi chú |
|---|---|---|---|---|
| 1 | Hai dòng họ lớn cùng tồn tại, hoà bình | 5.252 | 135 | Nghỉ ngơi 20% thời gian |
| 2 | Hai dòng họ lớn, hoà bình | 1.690 | 645 | Một dòng họ nghỉ 35% thời gian |
| 3 | Một dòng họ thống trị 89%, xã hội chiến tranh | 34.348 | 1.366 | Kho trung bình cao gấp 5 lần seed khác nhờ cướp |

Ở seed 3, số vụ tấn công tăng đều từ 20.000 lên 34.000 mỗi cửa sổ trong khi số chết đói
giảm từ 1.500 xuống 200. Cướp bóc trở thành cách sống chính khi nó hiệu quả hơn hái lượm.

Trong mọi seed, mùa đông đẩy tấn công và chết đói tăng vọt, đúng với dự đoán rằng xung đột
nảy sinh từ khan hiếm chứ không cần thưởng cho việc đánh nhau.

Phân cụm chiến lược ở tick 20.000 cho thấy phân hoá **bên trong** xã hội:

| Seed | Cách sống hiệu dụng | Các cụm đáng chú ý |
|---|---|---|
| 3 | 3.3 | 59% hái lượm thuần; 11% hái lượm kiêm cướp (attack 19%); 6% cướp là chính (attack 38%); 1.6% chiến binh (attack 65%). Tất cả trong cùng 2 dòng họ. |
| 1 | 5.3 | 58% hái lượm; 30% hái lượm xen nghỉ; 10% nghỉ là chính (rest 60%), nghèo hơn hẳn (của cải 40 so với 80). |
| 2 | 3.9 | 57% hái lượm; một cụm 7% "sinh sản là chính" (repro 35%); một cụm 7% chuyên chia sẻ với họ hàng. |

Trước khi có lớp văn hoá, chưa seed nào định cư: mọi cụm đều di chuyển trên 45% tốc độ tối đa.

### Cách mạng nông nghiệp

Với lớp văn hoá, 40.000 tick, tham số mặc định:

| Seed | farming khám phá | Định cư bắt đầu | Tick 40.000 |
|---|---|---|---|
| 3 | tick 9.596 | tick 30.000, 40% dân số ở yên | 80% ở yên, 8.700 ô ruộng, tấn công giảm từ 70.000 xuống 4.800 mỗi cửa sổ, chia sẻ tăng từ 5.000 lên 138.000 |
| 1 | tick 4.369 | tick 10.000, 32% dân số ở yên | 63% ở yên, 8.500 ô ruộng, dân số chạm trần 4.000 |

Không ai bảo agent phải định cư. Nông nghiệp chỉ làm cho ở yên trở nên có lợi, và tiến hoá tìm ra
việc đó sau vài nghìn tick. Khi làng xuất hiện thì bạo lực giảm, chia sẻ tăng, và các cụm chiến lược
mới xuất hiện: "sinh sản là chính" (repro 90%) và "chia sẻ là chính" (share 70%), là những kiểu sống
không tồn tại nổi thời du mục. Ảnh cuối seed 3 cho thấy các ô ruộng đỏ tụ thành làng trên đất tốt,
du mục còn lại lang thang ở vùng cằn giữa các làng.

### Ba bài học khi cân bằng

1. Xác suất khám phá phải tính theo agent-tick. 1000 agent với xác suất 0,0002 mỗi tick tìm ra công cụ ngay tick 1.
2. Nếu đi ngang qua cũng chăm được ruộng thì 4000 du mục biến cả bản đồ thành ruộng mà không ai cần ở lại.
3. "Đứng yên" phải là một quyết định dễ đột biến. Với hai output tanh bão hoà, đứng yên là điểm có xác suất
   gần bằng không và tiến hoá không bao giờ tới. Tách thành một output đi hay ở theo dấu là đủ.

## Cấu trúc mã

```
sim/src/
  config.rs   toàn bộ luật thế giới và tham số, đọc từ CLI
  world.rs    sinh địa hình, thức ăn, mùa
  brain.rs    gen, mạng thần kinh, đột biến, độ họ hàng
  agent.rs    trạng thái agent và quyết định mỗi tick
  spatial.rs  spatial hash cho truy vấn hàng xóm O(k)
  sim.rs      vòng lặp: mọc lại, cảm nhận, nghĩ, hành động, chết, nhập cư
  stats.rs    thống kê cửa sổ, entropy hành động, Gini, CSV
  strategy.rs phân cụm hành vi k-means, gộp cụm, entropy chiến lược
  events.rs   sử ký: khám phá, thống trị, tuyệt chủng, nạn đói, chiến tranh, tầng lớp mới
  render.rs   xuất PPM
  rng.rs      xorshift64* có seed
```

## Bước tiếp theo

1. Sở hữu và phòng thủ: ruộng có chủ, tường hoặc kho chung, để xem làng có bảo vệ nhau không.
2. Lớp hiển thị bằng Godot 4 đọc trạng thái từ lõi này, camera bám theo sử ký.
3. Trần dân số 4.000 đang chạm sau nông nghiệp. Nâng trần cần tối ưu thêm, ví dụ nhân matrix theo batch.
