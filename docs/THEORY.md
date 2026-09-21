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

**Độ tin.** Có bằng chứng về đất; gợi ý về kết cục. **Đã lặp lại** với biển, chế tác và não mới (mục 10): không
mệnh lệnh thì biên độ bùng-vỡ 7,9 so với 3,9 và 6/16 thế giới tuyệt chủng so với 3/16.

### 2. Mệnh lệnh sống lâu hơn người ra lệnh

**Phát biểu.** Một lệnh được tuân đủ lâu thành tập quán, và tập quán tự lan giữa họ hàng khi thủ lĩnh đã
chết. Tập quán phổ biến nhất là *ở lại* và *kiềm chế*.

**Bằng chứng.** Cột `custom` trong mọi bảng kết cục: 8 thế giới sau khi có biển có tập quán *ở lại* từ 0% đến
66% dân số; các xã hội bùng-vỡ thường có tập quán yếu (1 đến 14%).

**Độ tin.** Có bằng chứng (mục 10): cờ `--no-customs` tắt riêng tập quán, và định cư tụt từ 92% xuống 38%.

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

### 9. Nghe người lạ đáng giá hơn nghe họ hàng

**Phát biểu.** Tôi đã đoán ngược. Giả thuyết là: tín hiệu tốn năng lượng và chỉ họ hàng nghe rõ thì chọn lọc
mới có lý do biến tiếng gọi thành ngôn ngữ. Thí nghiệm 16 seed cho thấy thế giới chỉ nghe họ hàng nghèo
đi hẳn, thế giới điếc cũng vậy, còn thế giới nghe cả người lạ biết gấp ba, làm đồ vật gấp sáu và đông
gấp hai. Tiếng gọi của người lạ là một giác quan: nó cho biết người trước mặt đang ở trạng thái gì,
điều mà số đếm "có mấy người lạ" không nói được.

**Bằng chứng.** `tools/lab.py run hearing --seeds 1-16` (docs/lab/hearing.md, ở đó nhánh `default` là chỉ
nghe họ hàng): kết cục tốt 8/16 chỉ-họ-hàng, 6/16 điếc, 7/16 nghe-người-lạ, nhưng trung vị dân số đỉnh
1.634 so với 1.720 so với 4.103; kiến thức 18,5 so với 25,9 so với 53,5; đồ vật mỗi đầu người 0,07 so với
0,05 so với 0,43. Nghe người lạ so với chỉ họ hàng: đồ vật mỗi đầu người +0,70 với KTC 95% [+0,21, +1,23],
tỉ lệ người có đồ +0,23 [+0,06, +0,41], thời đại cuối +1,8 [0,0, +3,6]. Thông tin tương hỗ tín hiệu và
hành động vẫn ở 0,01 đến 0,02 bit; một thế giới (seed 10, nghe người lạ) loé 0,27 bit ở tick 5.500 rồi tắt.
Độ ồn trung bình 1,45 đến 1,57 trên tối đa 2 dù nói tốn năng lượng: chọn lọc không tắt tiếng.

**Lặp lại trên mặc định cuối** (nghe cả người lạ, không bầy thú), 16 seed, docs/lab/hearing.md bản mới: kết
cục tốt 9/16 nghe hết, 6/16 điếc, 8/16 chỉ họ hàng; phát minh trung vị 124 so với 62 so với 77; đồ vật mỗi
đầu người 0,47 so với 0,11 so với 0,16. Cùng hướng với lần đầu, nhưng lần này mọi khoảng tin cậy 95% đều
chứa 0 (phát minh điếc so với nghe hết: −22,6, KTC [−54,3, +8,7]); phương sai giữa các thế giới lớn.

**Một điểm phương pháp.** Nhánh điếc là mô hình không cho chỉ số ngôn ngữ: ở đó tín hiệu hàng xóm vẫn được
ghi nhưng không đi vào não, nên mọi thông tin tương hỗ còn lại là do hoàn cảnh chung (cùng đói, cùng mùa),
không phải do nghe. Trung vị 0,00 bit điếc so với 0,02 nghe: phần "do nghe" chỉ khoảng 0,02 bit. Sự kiện
"tiếng gọi bắt đầu có nghĩa" cũng bắn được ở một thế giới điếc (seed 5), nên ngưỡng 0,2 bit một mình chưa
đủ để gọi là ngôn ngữ; từ đây phải so với nhánh điếc.

**Độ tin.** Đã lặp lại về hướng ở hai đợt (16 seed mỗi đợt), có bằng chứng ở đợt đầu, chưa ở đợt hai. Vì kết
quả này, mặc định của sim là nghe cả người lạ. Ngôn ngữ (tín hiệu mang nghĩa bền vững) vẫn chưa xuất hiện;
cái đã xuất hiện là **đọc trạng thái người khác qua tiếng gọi**, một bước trước ngôn ngữ.

### 10. Tập quán giữ người ở lại, và ở lại là sống

**Phát biểu.** Tắt tập quán (chúng vẫn hình thành và lan nhưng không bao giờ lên tiếng) làm tỉ lệ định cư
tụt từ 92% xuống 38%, số thế giới tuyệt chủng tăng từ 3 lên 5, kết cục tốt giảm từ 8/16 xuống 4/16. Tắt cả
mệnh lệnh lẫn tập quán: 6 tuyệt chủng, biên độ bùng-vỡ gấp đôi (7,9 so với 3,9). Tập quán không phải phần
phụ của thủ lĩnh: nó là cơ chế chính giữ một xã hội ở yên một chỗ khi thủ lĩnh đã chết.

**Bằng chứng.** `tools/lab.py run customs --seeds 1-16` (docs/lab/customs.md). Định cư −0,25 với KTC 95%
[−0,53, +0,04] khi tắt tập quán; tuân lệnh −0,19 [−0,39, −0,01]. Không mệnh lệnh: 5/16 tốt, biên độ 7,95.

**Độ tin.** Có bằng chứng cho tuân lệnh; gợi ý mạnh cho định cư và kết cục (16 seed, khoảng tin cậy chạm 0).
Mục 1 (thủ lĩnh là cái phanh) được **lặp lại** lần thứ hai ở đây: không mệnh lệnh thì bùng-vỡ mạnh gấp đôi.

### 11. Chế tác làm thế giới biết nhiều hơn, chưa chắc sống lâu hơn

**Phát biểu.** Tắt chế tác (hành động chế tác thành nghỉ) làm số phát minh giảm 37 và kiến thức mỗi đầu
người giảm 27, đúng như phải thế; nhưng kết cục chỉ đổi từ 8/16 xuống 6/16 tốt, và dân số đỉnh trung vị
lại cao hơn khi không chế tác (3.955 so với 1.634). Trong 20.000 tick, đồ vật là chi phí (trọng lượng, năng
lượng thử) nhiều hơn là lợi ích cho sự sống còn; lợi ích của chúng là tri thức và mức phát triển.

**Bằng chứng.** `tools/lab.py run crafting --seeds 1-16` (docs/lab/crafting.md): phát minh −36,9 với KTC 95%
[−62,8, −12,8]; kiến thức −27,1 [−46,5, −8,8]; dân số đỉnh +1.337 [−720, +3.495]; thời đại cuối −1,4 [−3,1, +0,1].

**Độ tin.** Có bằng chứng cho tri thức; gợi ý cho dân số. Câu hỏi tiếp: chạy 60.000 tick để xem chi phí
sớm có đổi thành lợi ích muộn không.

### 12. Một việc cần nhiều tay không đủ để sinh ra lời nói

**Phát biểu.** Thêm bầy thú chỉ hạ được khi ít nhất hai người đánh gần cùng lúc không làm tín hiệu có nghĩa
hơn (thông tin tương hỗ trung vị 0,02 bit ở cả hai nhánh) và gần như không ai săn được chung: 12/16 thế
giới không có cuộc săn thành công nào, thế giới nhiều nhất có 6. Cái bầy thú làm được là kéo người rời
làng: định cư trung vị tụt từ 68% xuống 15%, đồ vật mỗi đầu người từ 0,47 xuống 0,09, kết cục tốt từ 9/16
xuống 5/16, tuyệt chủng từ 2 lên 5.

**Bằng chứng.** `tools/lab.py run herds --seeds 1-16` (docs/lab/herds.md: `default` không bầy thú, `herds` có). Trung
vị khác nhau nhiều nhưng phương sai lớn: hiệu số trung bình định cư +0,15 với KTC 95% [−0,14, +0,42], tuân
lệnh +0,20 [−0,02, +0,42]. Sự kiện "tiếng gọi bắt đầu có nghĩa" bắn ở 3/16 thế giới có bầy thú (seed 5, 14,
16; cao nhất 0,134 bit) so với 1/16 không có, nhưng đều tắt sau đó.

**Độ tin.** Gợi ý. Cách đọc: với não hiện tại, "đánh khi không có ai gần" bỗng thành "săn" là một cám dỗ
đắt (mỗi đòn tốn 1,5 năng lượng, bầy bỏ chạy) mà chọn lọc chưa kịp học cách phối hợp trong 20.000 tick.
Giả thuyết "có việc cần phối hợp thì tiếng gọi có nghĩa" **chưa được ủng hộ**; có thể cần việc phối hợp
không phạt nặng người thử, hoặc cần thời gian dài hơn nhiều. Vì kết quả này, bầy thú tắt mặc định và giữ
lại như một cờ thí nghiệm.

## Những câu hỏi mở

1. **Ngôn ngữ có xuất hiện không?** Chưa. Cái đã có là đọc trạng thái người khác qua tiếng gọi (mục 9), và
   hai lần loé lên 0,27 đến 0,31 bit rồi tắt. Giả thuyết "chỉ họ hàng nghe thì ngôn ngữ sẽ ra" đã bị bác. Giả
   thuyết tiếp theo đáng thử: tín hiệu chỉ có nghĩa khi có việc cần phối hợp mà một người không làm nổi. Thế
   giới có **bầy thú** chỉ ngã khi ít nhất hai người đánh gần cùng lúc (mục 12): chưa đủ, và còn có hại. Giả
   thuyết tiếp theo: việc phối hợp phải rẻ khi thử và chỉ đắt khi bỏ dở, ví dụ dựng một công trình lớn cần
   nhiều người góp vật liệu, không ai mất gì nếu góp một mình.
   Từ giờ ngôn ngữ được đo ở **hai đầu kênh**: `signal_meaning` là thông tin tương hỗ giữa điều một người
   nói và tình trạng của chính người đó (đói hay no, sợ hay không: sáu lớp), tức là tiếng gọi *có nội dung*;
   `signal_mi` là giữa điều nghe được và việc làm ngay sau, tức là tiếng gọi *được hiểu*. Một tiếng gọi có
   thể có nội dung mà không ai hiểu, hoặc "được hiểu" mà không có nội dung khi hàng xóm chỉ cùng cảnh ngộ.
   Sự kiện "tiếng gọi bắt đầu có nghĩa" nay đòi cả hai từ 0,2 bit. Trong một thế giới thử (seed 3, 3.000
   tick), nội dung tăng dần từ 0 lên 0,09 bit khi dân số lên 2.000, trong khi hiểu đứng ở 0,08: tiếng gọi
   bắt đầu phản ánh người nói trước khi ai đó dùng được nó.
2. **Học trong đời học được gì?** Đã biết nó làm ra người dùng đồ vật (mục 8). Chưa biết tiến hoá đẩy tốc độ
   học đi đâu: `plastic` trung vị 0,01 với học mặc định, tức là đa số não học chậm. Câu hỏi: có dòng họ nào
   tiến hoá ra não mềm hẳn không, và họ sống ra sao?
3. **Phân công lao động có nổi lên không?** Có người chỉ chế tác, người chỉ hái không? Từ giờ có chỉ số
   `division_of_labour` (0 đến 1): thông tin tương hỗ giữa *ai* và *làm gì*, chia cho entropy của việc làm,
   tính trên hồ sơ hành động có quên dần của từng người (Gorelick và cộng sự). 0 là mọi người sống cùng một
   kiểu pha trộn; 1 là mỗi người một việc và người này khác người kia. Thế giới thử (seed 3, 3.000 tick) đứng
   ở 0,55 đến 0,70 ngay từ đầu: các cá thể đã chuyên môn hoá mạnh, nhưng chưa biết đó là chuyên môn theo
   dòng họ, theo tuổi hay theo hoàn cảnh. Chỉ số phụ: số vật liệu cho nhau (`material_gifts`).
4. **Tập quán có tự bảo vệ đất không khi tắt hẳn thủ lĩnh?** Đã trả lời một phần ở mục 10: tập quán giữ người
   ở lại; đất thì cả hai nhánh đều 100% trong 20.000 tick, chưa phân biệt được.
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
