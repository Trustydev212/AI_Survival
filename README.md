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

- `stats_seed<N>.csv`: dân số, số dòng họ, Gini, số sinh, chết đói, bị giết, tấn công, chia sẻ, phân bố hành động, theo từng cửa sổ thời gian.
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
  render.rs   xuất PPM
  rng.rs      xorshift64* có seed
```

## Bước tiếp theo

1. Đo đa dạng chiến lược theo cụm gen thay vì theo dòng họ, để phát hiện hội tụ sớm.
2. Lớp văn hoá: công nghệ và tập quán lan truyền theo khoảng cách, tách khỏi gen.
3. Lớp hiển thị bằng Godot 4 đọc trạng thái từ lõi này, kèm camera tự bắt sự kiện.
4. Nhân matrix theo batch khi cần vượt 10.000 agent.
