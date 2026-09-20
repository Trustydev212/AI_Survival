# Sổ quan sát: quy luật của xã hội thu nhỏ

Repo này là một thí nghiệm, không phải một trò chơi có kịch bản. Tài liệu này ghi những quy luật đã quan
sát được, bằng chứng cho từng quy luật, và những câu hỏi còn mở. Mỗi mục có ba phần: **phát biểu**, **bằng
chứng** (thí nghiệm nào, bao nhiêu thế giới), **độ tin**. Một quy luật chỉ được ghi khi có đối chứng: cùng
luật, cùng seed, khác đúng một thứ. Cách chạy đối chứng nằm ở cuối.

Quy ước độ tin: *gợi ý* (8 thế giới, hướng rõ nhưng khoảng tin cậy còn chứa 0), *có bằng chứng* (khoảng
tin cậy 95% của hiệu số không chứa 0), *đã lặp lại* (đúng ở hai đợt thí nghiệm khác nhau, ví dụ trước và
sau khi thêm biển).

## Những gì đã đo được

### 1. Thủ lĩnh là cái phanh, không phải động cơ

**Phát biểu.** Mệnh lệnh của thủ lĩnh làm xã hội bền hơn chủ yếu bằng cách kìm lại: đất được giữ tốt hơn,
ít bùng-vỡ hơn, chứ không phải đông hơn hay biết nhiều hơn.

**Bằng chứng.** Đối chứng `--no-orders` trên 12 thế giới (trước khi có biển): đất trung vị 87% có lệnh so với
66% không lệnh; kết cục xấu 3/12 so với 5/12. Lệnh được tuân nhiều nhất là *kiềm chế* (ngừng hái) và *góp
kho*. Xem README, mục "Đối chứng: có mệnh lệnh và không có mệnh lệnh".

**Độ tin.** Có bằng chứng về đất; gợi ý về kết cục. Cần lặp lại với biển và chế tác: `tools/lab.py run orders`.

### 2. Mệnh lệnh sống lâu hơn người ra lệnh

**Phát biểu.** Một lệnh được tuân đủ lâu thành tập quán, và tập quán tự lan giữa họ hàng khi thủ lĩnh đã
chết. Tập quán phổ biến nhất là *ở lại* và *kiềm chế*.

**Bằng chứng.** Cột `custom` trong mọi bảng kết cục: 8 thế giới sau khi có biển có tập quán *ở lại* từ 0% đến
66% dân số; các xã hội bùng-vỡ thường có tập quán yếu (1 đến 14%).

**Độ tin.** Gợi ý. Chưa có đối chứng tắt tập quán riêng (chưa có cờ); là việc nên làm.

### 3. Biển chia cắt bảo vệ, thuyền mở ra

**Phát biểu.** Biển không đi qua được làm chiến tranh và dịch lan chậm hơn giữa các bờ; đảo nhỏ là bẫy chết.
Khi thuyền xuất hiện, biển thành kho cá không cạn và những thế giới có thuyền là những thế giới hưng thịnh.

**Bằng chứng.** 8 thế giới có biển không thuyền: 5 hưng thịnh, 1 sụp đổ (đảo). Cùng 8 seed khi thêm chiều phát
minh đi biển: ba thế giới tìm ra thuyền, và đúng ba thế giới đó hưng thịnh; năm thế giới còn lại không đổi vì
không ai tìm ra thuyền. Seed 2 có lúc 316 người cùng đánh cá.

**Độ tin.** Gợi ý mạnh nhưng chưa tách được nhân quả: thế giới giàu có thể vừa dễ hưng thịnh vừa dễ tìm ra
thuyền. `tools/lab.py run sea` (biển rẻ, biển đắt) là bước tiếp.

### 4. Phát minh là hàm của số lần thử

**Phát biểu.** Với hệ vật liệu, số đồ vật một thế giới tìm ra tỉ lệ với số người và số lần họ thử, không
phải với thời gian. Xã hội nghèo (chưa từng vượt 1.000 dân) tìm ra khoảng 14 đồ vật trong 20.000 tick; xã
hội đông tìm ra trên 120.

**Bằng chứng.** 8 thế giới có chế tác: seed 1 và 3 (đỉnh 1.000) tìm ra 14 đồ vật mỗi seed; seed 2, 5, 6
(đỉnh 3.000 đến 5.800) tìm ra 124 đến 148. Tổng 543 đồ vật, không cái nào được viết sẵn.

**Độ tin.** Có bằng chứng (chênh lệch một bậc), nhưng là tương quan; `tools/lab.py run population` (300 so
với 3.000 người xuất phát) kiểm tra nhân quả.

### 5. Lửa đến từ đá, luôn luôn

**Phát biểu.** Trong mọi thế giới tìm ra lửa, lửa đến từ việc đập hai thứ rất cứng vào nhau (đá với đá,
đá với lưỡi đá), không bao giờ từ gỗ trước. Sành và kim loại chỉ đến sau lửa, và chỉ ở nơi có đất sét hay
quặng trong tầm nhặt.

**Bằng chứng.** 7/8 thế giới có lửa, sớm nhất tick 94, muộn nhất tick 1.707. Sành (`fire(hollow(clay))`) ở seed
5 tick 3.666; kim loại (`sharpen(ore)` rồi nung) hiếm hơn nhiều.

**Độ tin.** Đây là hệ quả của vật lý được thiết kế (chỉ vật rất cứng đập nhau mới bật lửa), nên nó là kiểm
chứng vật lý hơn là quy luật xã hội. Điều thú vị là **thứ tự**: xã hội nào cũng có vũ khí và bình chứa trước
lửa, công cụ có cán sau lửa hàng trăm tick, tấm chắn muộn nhất.

### 6. Nhiều đường tới cùng một thứ

**Phát biểu.** Cùng một công dụng được tìm ra bằng nhiều công thức khác nhau ở các thế giới khác nhau, và
đôi khi trong cùng một thế giới.

**Bằng chứng.** Thuyền: `hollow(wood)`, `hollow(bind(wood, fibre))`, `sharpen(bind(fibre, hollow(wood)))`.
Nơi trú: `bind(stone, stone, clay)`, `bind(wood, wood, clay)`, `bind(stone, fibre, ore)`. Rìu
`bind(wood, fibre, sharpen(stone))` xuất hiện độc lập ở nhiều seed.

**Độ tin.** Quan sát trực tiếp từ sử ký; không cần đối chứng.

### 7. Mọi tiến bộ đều có hoá đơn

**Phát biểu.** Không có phát minh nào miễn phí: ý tưởng trả bằng tiêu hao hoặc đất, đồ vật trả bằng trọng
lượng (15 đến 27% tiêu hao cho ai mang) và bằng đất nếu là công cụ đào. Xã hội thành công quá nhanh có thể tự
hủy: seed 8 lên 20.265 dân rồi rơi xuống 580 với đất còn 100% nhưng kiến thức 4,6 mỗi đầu người, tức là
bùng nổ dân số mà không có kỹ thuật.

**Bằng chứng.** Bảng kết cục 8 thế giới có thuyền (README). Cột `known` và `soil%` cạnh nhau.

**Độ tin.** Gợi ý. Cần thí nghiệm tách "đông" khỏi "biết": `tools/lab.py run population`.

### 8. Học trong đời làm ra người dùng đồ vật

**Phát biểu.** Bật học trong đời (phần dẻo Hebb có điều biến bởi phần thưởng) không đổi rõ kết cục hay dân số
với 8 thế giới, nhưng làm số đồ vật mỗi đầu người tăng gấp đôi và nhiều người cầm đồ hơn. Học nhanh gấp ba
không tốt hơn học vừa.

**Bằng chứng.** `tools/lab.py run learning --seeds 1-8` (docs/lab/learning.md): đồ vật mỗi đầu người 0,39
có học so với 0,21 không học, hiệu số +0,33 với khoảng tin cậy 95% [+0,01, +0,70]; tỉ lệ người có đồ 18%
so với 13%, hiệu số +0,15 với KTC [+0,00, +0,31]. Dân số, kiến thức, đất, kết cục: khoảng tin cậy còn chứa 0.
Học nhanh gấp ba: đồ vật mỗi đầu người 0,19, tức là không hơn không học.

**Độ tin.** Có bằng chứng cho đồ vật; chưa rõ cho phần còn lại. Cách đọc: học theo phần thưởng dạy được
một việc cụ thể trong đời (chế tác rồi dùng thứ mình làm ra), nhưng không dạy được chiến lược sống, thứ vẫn
thuộc về tiến hoá.

### 9. Nghe nhau chưa có ích, và tiếng gọi mới chỉ loé lên

**Phát biểu.** Với não hiện tại, cho mọi người điếc (đầu vào tín hiệu bằng 0) không làm xã hội tệ đi; thế
giới điếc còn sinh sản ít hơn và biết nhiều hơn một chút. Tín hiệu có nghĩa chỉ xuất hiện thoáng qua: một
thế giới trong tám vượt 0,3 bit ở tick 1.500 rồi mất.

**Bằng chứng.** `tools/lab.py run hearing --seeds 1-8` (docs/lab/hearing.md): kết cục 3/8 tốt ở cả hai nhánh;
sinh sản mỗi 1000 tick 4,1 điếc so với 5,7 nghe, hiệu số −2,5 với KTC 95% [−4,6, −0,7]; kiến thức trung vị
49,5 so với 33,6 nhưng khoảng tin cậy chứa 0. Thông tin tương hỗ tín hiệu–hành động trung vị 0,01 bit ở cả
hai nhánh (nhánh điếc vẫn phát tín hiệu, chỉ không nghe). Sự kiện "tiếng gọi bắt đầu có nghĩa" bắn ở seed 4
tick 1.500 (0,31 bit) và không lặp lại.

**Độ tin.** Gợi ý. Cách đọc: kênh tín hiệu đang tồn tại như tiếng ồn; chọn lọc chưa tìm ra cách dùng nó,
hoặc 20.000 tick là quá ngắn. Đây là câu hỏi mở số 1, và là chỗ đáng thử nhất: ví dụ chỉ nghe họ hàng, hay
cho tín hiệu tốn năng lượng để nói dối có giá.

## Những câu hỏi mở

1. **Ngôn ngữ có xuất hiện không?** Tín hiệu hai chiều không có nghĩa định sẵn. Chỉ số: thông tin tương hỗ
   giữa tín hiệu nghe được và hành động kế tiếp, đã hiệu chỉnh thiên lệch. Đã thấy nó loé lên (0,31 bit, seed 4,
   tick 1.500) rồi tắt (mục 9). Câu hỏi: điều kiện nào giữ nó lại, và họ hàng với người lạ có nghe khác nhau
   không (lừa dối?).
2. **Học trong đời học được gì?** Đã biết nó làm ra người dùng đồ vật (mục 8). Chưa biết tiến hoá đẩy tốc độ
   học đi đâu: `plastic` trung vị 0,01 với học mặc định, tức là đa số não học chậm. Câu hỏi: có dòng họ nào
   tiến hoá ra não mềm hẳn không, và họ sống ra sao?
3. **Phân công lao động có nổi lên không?** Có người chỉ chế tác, người chỉ hái không? Chỉ số: entropy của
   ngăn đồ vật theo dòng họ, và số vật liệu cho nhau (`material_gifts`).
4. **Tập quán có tự bảo vệ đất không khi tắt hẳn thủ lĩnh?** Cần cờ tắt tập quán riêng.
5. **Có "đêm trường" không?** Sự kiện *lãng quên* ghi lúc kiến thức tụt quá nửa. Bao nhiêu thế giới quên rồi
   tìm lại được, và tìm lại bằng cùng công thức hay công thức khác?

## Cách chạy một thí nghiệm

```bash
cd sim && cargo build --release && cd ..
python3 tools/lab.py list                    # các thí nghiệm có sẵn
python3 tools/lab.py run orders --seeds 1-16 # chạy mọi nhánh rồi viết docs/lab/orders.md
python3 tools/lab.py run all --seeds 1-8
```

Mỗi thí nghiệm trong `tools/experiments.json` là một câu hỏi với vài nhánh chỉ khác nhau đúng một cờ; các
nhánh chạy trên cùng dãy seed nên khác biệt là do cờ, không do bản đồ. Báo cáo ghi kết cục từng nhánh, trung
vị mọi chỉ số, và hiệu số trung bình so với đối chứng kèm khoảng tin cậy 95% bằng bootstrap. Muốn thêm câu
hỏi mới: thêm một mục vào file JSON; muốn thêm chỉ số mới: thêm vào bảng kết cục trong `sim/src/main.rs` và
vào danh sách `METRICS` trong `tools/lab.py`.

Tám thế giới cho biết hướng; mười sáu trở lên mới bắt đầu cho biết độ lớn. Mỗi thế giới 20.000 tick mất
khoảng một phút trên máy 4 nhân.
