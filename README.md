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
- `frame_<tick>.ppm`: bản đồ. Xanh lá là thức ăn, chấm màu là agent với màu do gen quy định.

Cùng seed luôn cho cùng kết quả, nên có thể replay và so sánh thí nghiệm.

## Cách thế giới vận hành

**Thế giới** là lưới ô cuộn tròn. Mỗi ô có độ màu mỡ cố định, sinh ra từ value noise
rồi ngưỡng hoá để đất tốt tụ thành từng vùng và khoảng 40% bản đồ gần như cằn cỗi.
Thức ăn mọc lại theo độ màu mỡ và theo mùa. Mùa đông giảm tốc độ mọc xuống 20%.

**Agent** có năng lượng, kho dự trữ, tuổi, dòng họ, và một bộ gen. Gen gồm trọng số
của một mạng thần kinh 20 input, 16 ẩn, 7 output (455 tham số) cộng một "màu" ba chiều.
Nhận diện họ hàng dựa trên khoảng cách màu.

**Mỗi tick**, agent nhìn thấy: năng lượng, tuổi, kho, thức ăn tại chỗ và gradient thức ăn,
agent gần nhất (hướng, khoảng cách, độ họ hàng, chênh lệch sức mạnh, kho của nó),
số hàng xóm là họ hàng và không họ hàng, mùa, và mình vừa bị đánh hay chưa.
Não trả về hướng di chuyển và một trong năm hành động:

| Hành động | Tác dụng |
|---|---|
| gather | Lấy thức ăn từ ô đang đứng, dư thì cất vào kho |
| attack | Đánh agent gần nhất trong tầm. Ai khoẻ hơn dễ thắng. Thắng thì cướp kho và gây sát thương |
| share | Cho họ hàng gần nhất một phần kho |
| repro | Nếu đủ năng lượng, sinh con. Con thừa hưởng gen có đột biến |
| rest | Giảm tiêu hao năng lượng |

Không có hàm thưởng. Ai sinh được nhiều con thì gen của họ tồn tại. Đó là toàn bộ thuật toán học.
Khi dân số sụp dưới ngưỡng, vài agent gen ngẫu nhiên "nhập cư" để thí nghiệm không chết hẳn.

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

Chưa seed nào tiến hoá ra lối sống định cư: mọi cụm đều di chuyển trên 45% tốc độ tối đa.
Với thức ăn mọc lại chậm và không có nông nghiệp, đi lang thang vẫn là tối ưu. Đây là mốc
để so sánh khi thêm lớp công nghệ.

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
  render.rs   xuất PPM
  rng.rs      xorshift64* có seed
```

## Bước tiếp theo

1. Lớp văn hoá: công nghệ và tập quán lan truyền theo khoảng cách, tách khỏi gen. Nông nghiệp là công nghệ đầu tiên vì nó là thứ duy nhất khiến định cư có lợi.
2. Nhật ký sự kiện: trận đánh đầu tiên, dòng họ tuyệt chủng, cụm chiến lược mới xuất hiện, để camera sau này bám theo.
3. Lớp hiển thị bằng Godot 4 đọc trạng thái từ lõi này.
4. Nhân matrix theo batch khi cần vượt 10.000 agent.
