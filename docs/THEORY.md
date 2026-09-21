# Sổ quan sát: quy luật của xã hội thu nhỏ

Repo này là một thí nghiệm, không phải một trò chơi có kịch bản. Tài liệu này ghi những quy luật đã quan
sát được, bằng chứng cho từng quy luật, và những câu hỏi còn mở. Mỗi mục có ba phần: **phát biểu**, **bằng
chứng** (thí nghiệm nào, bao nhiêu thế giới), **độ tin**. Một quy luật chỉ được ghi khi có đối chứng: cùng
luật, cùng seed, khác đúng một thứ. Cách chạy đối chứng nằm ở cuối.

Quy ước độ tin: *gợi ý* (8 thế giới, hướng rõ nhưng khoảng tin cậy còn chứa 0), *có bằng chứng* (khoảng
tin cậy 95% của hiệu số không chứa 0), *đã lặp lại* (đúng ở hai đợt thí nghiệm khác nhau, ví dụ trước và
sau khi thêm biển).

## Những gì đã đo được

### 1. Thủ lĩnh là cái phanh, không phải động cơ (đã rút lại)

**Phát biểu.** Mệnh lệnh của thủ lĩnh làm xã hội bền hơn chủ yếu bằng cách kìm lại: đất được giữ tốt hơn,
ít bùng-vỡ hơn, chứ không phải đông hơn hay biết nhiều hơn.

**Bằng chứng.** Đối chứng `--no-orders` trên 12 thế giới (trước khi có biển): đất trung vị 87% có lệnh so với
66% không lệnh; kết cục xấu 3/12 so với 5/12. Lệnh được tuân nhiều nhất là *kiềm chế* (ngừng hái) và *góp
kho*. Xem README, mục "Đối chứng: có mệnh lệnh và không có mệnh lệnh".

**Lặp lại lần ba, trên mặc định cuối** (nghe cả người lạ, không bầy thú), 16 seed, docs/lab/customs.md bản
mới: **kết quả đảo chiều**. Không mệnh lệnh cho 12/16 kết cục tốt so với 9/16 có mệnh lệnh; biên độ bùng-vỡ
2,94 so với 3,39; kiến thức mỗi người 54,7 so với 36,7; đồ vật mỗi người 0,56 so với 0,47. Chỉ số duy nhất
có khoảng tin cậy không chứa 0 là chính mức tuân lệnh, tức là thứ bị tắt.

**Độ tin.** **Không lặp lại được.** Hai đợt đầu (12 và 16 thế giới, mặc định cũ) thấy mệnh lệnh giữ đất và
giảm bùng-vỡ; đợt ba trên mặc định cuối thấy ngược lại, và không đợt nào có khoảng tin cậy loại được 0 cho
các chỉ số ngoài tuân lệnh. Kết luận trung thực: **chưa có bằng chứng nào cho thấy mệnh lệnh giúp hay hại**,
và phát biểu "thủ lĩnh là cái phanh" phải rút lại. Cách đọc khả dĩ: khi thế giới còn nghèo cơ hội, cái phanh
có ích; khi đã có biển, vật liệu và người lạ nghe được nhau, cùng cái phanh ấy chặn mất đường đi. Nếu đúng
thì tác dụng của quyền lực phụ thuộc vào thế giới chứ không phải vào quyền lực, và đó là câu hỏi đáng theo
đuổi hơn câu hỏi ban đầu.

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

**Kiểm tra nhân quả**, `tools/lab.py run population --seeds 1-16` (docs/lab/population.md): cùng bản đồ,
xuất phát 300, 1.000 (mặc định) và 3.000 người. Ít người: phát minh trung vị 38,5 so với 124, hiệu số −39,5
với khoảng tin cậy 95% [−70,4, −6,4]; đồ vật mỗi đầu người 0,05 so với 0,47; kết cục tốt 4/16 so với 9/16,
5/16 tuyệt chủng. Nhiều người: phát minh 128, kiến thức mỗi đầu 66,8 so với 36,7, đồ vật 0,73, kết cục tốt
8/16, nhưng khoảng tin cậy của phát minh và kiến thức còn chứa 0 (+23,8, KTC [−3,9, +50,8]). Đường cong
bão hoà: từ 300 lên 1.000 người, phát minh tăng gấp ba; từ 1.000 lên 3.000, chỉ thêm vài phần trăm, dù đỉnh
dân số lên 4.750. Cái tăng tiếp ở xã hội đông không phải số thứ tìm ra mà là **số thứ mỗi người biết** và
cầm: kiến thức lan rộng hơn trong đám đông.

**Độ tin.** Có bằng chứng nhân quả cho chiều "ít người thì ít phát minh" (khoảng tin cậy không chứa 0);
chiều "đông hơn nữa thì nhiều hơn nữa" chưa rõ, và có lý do để nghĩ nó bão hoà: kho đồ vật có 128 chỗ và
số cách ghép có nghĩa là hữu hạn, nên thế giới đông chạm trần của vật lý trước khi chạm trần của người.
Một hệ quả đáng ghi: 300 người xuất phát là dưới ngưỡng sống của thế giới này (5/16 tuyệt chủng), tức là
có một **dân số tối thiểu** để một xã hội tự duy trì, và nó nằm giữa 300 và 1.000.

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

### 8. Học trong đời chưa chứng minh được là có ích (kết quả đầu không lặp lại)

**Phát biểu.** Bật học trong đời (phần dẻo Hebb có điều biến bởi phần thưởng) không đổi kết cục, dân số,
kiến thức hay đồ vật một cách đo được. Đợt đầu (8 thế giới, mặc định cũ chỉ họ hàng nghe) thấy đồ vật mỗi
đầu người tăng gấp đôi; đợt hai (16 thế giới, mặc định cuối nghe cả người lạ) không thấy lại. Học nhanh gấp
ba không tốt hơn, có phần kém hơn.

**Bằng chứng.** `tools/lab.py run learning --seeds 1-8` (docs/lab/learning.md): đồ vật mỗi đầu người 0,39
có học so với 0,21 không học, hiệu số +0,33 với khoảng tin cậy 95% [+0,01, +0,70]; tỉ lệ người có đồ 18%
so với 13%, hiệu số +0,15 với KTC [+0,00, +0,31]. Dân số, kiến thức, đất, kết cục: khoảng tin cậy còn chứa 0.
Học nhanh gấp ba: đồ vật mỗi đầu người 0,19, tức là không hơn không học.

**Lặp lại trên mặc định cuối**, 16 seed (docs/lab/learning.md bản mới): kết cục tốt 9/16 có học, 8/16 không
học, 6/16 học nhanh; đồ vật mỗi đầu người 0,47 so với 0,53 so với 0,34; kiến thức mỗi đầu 36,7 so với 62,5
so với 52,6; định cư 68% so với 89% so với 84%. Chỉ số duy nhất khác 0 là chính độ dẻo não (thứ được bật
tắt). Không học lại biết nhiều hơn và ở yên hơn, tuy khoảng tin cậy còn chứa 0.

**Độ tin.** Kết quả đầu không lặp lại; coi như chưa có bằng chứng. Cách đọc: khi người lạ nghe được nhau
(mặc định cuối), phần lớn cái mà học trong đời từng đem lại đã đến qua kênh nghe; học theo phần thưởng chỉ
dạy được việc cụ thể trong đời, và ngay việc đó cũng có đường khác. Tiến hoá vẫn giữ tốc độ học thấp
(`learn_rate` trung vị 2 đến 3 phần nghìn ở mọi nhánh): não mềm không được chọn lọc ưu ái.

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

### 10. Tập quán giữ người ở lại (đã rút lại), nhưng vẫn làm ra đồ vật

**Phát biểu.** Tắt tập quán (chúng vẫn hình thành và lan nhưng không bao giờ lên tiếng) làm tỉ lệ định cư
tụt từ 92% xuống 38%, số thế giới tuyệt chủng tăng từ 3 lên 5, kết cục tốt giảm từ 8/16 xuống 4/16. Tắt cả
mệnh lệnh lẫn tập quán: 6 tuyệt chủng, biên độ bùng-vỡ gấp đôi (7,9 so với 3,9). Tập quán không phải phần
phụ của thủ lĩnh: nó là cơ chế chính giữ một xã hội ở yên một chỗ khi thủ lĩnh đã chết.

**Bằng chứng.** `tools/lab.py run customs --seeds 1-16` (docs/lab/customs.md). Định cư −0,25 với KTC 95%
[−0,53, +0,04] khi tắt tập quán; tuân lệnh −0,19 [−0,39, −0,01]. Không mệnh lệnh: 5/16 tốt, biên độ 7,95.

**Lặp lại trên mặc định cuối**, 16 seed (docs/lab/customs.md bản mới): **không lặp lại được**. Tắt tập quán
cho 7/16 kết cục tốt so với 9/16, nhưng định cư 75% so với 68%, tức là nhỉnh hơn chứ không tụt. Cái tụt rõ
là đồ vật mỗi người, 0,26 so với 0,47. Lại chỉ có mức tuân lệnh là khác 0.

**Độ tin.** Phát biểu cũ về định cư **phải rút lại**: tỉ lệ 92% xuống 38% ở đợt trước không xuất hiện lại. Cái
còn sống sót qua hai đợt là một quan sát hẹp hơn: tắt tập quán thì người ta cầm ít đồ vật hơn, có lẽ vì đồ
vật cần ở yên một chỗ đủ lâu mới làm và giữ được. Mục 1 mất luôn chỗ dựa thứ hai của nó ở đây.

### 11. Chế tác mua tri thức bằng dân số và bằng đất

**Phát biểu.** Tắt chế tác (hành động chế tác thành nghỉ) làm số phát minh giảm 37 và kiến thức mỗi đầu
người giảm 27, đúng như phải thế; nhưng kết cục chỉ đổi từ 8/16 xuống 6/16 tốt, và dân số đỉnh trung vị
lại cao hơn khi không chế tác (3.955 so với 1.634). Trong 20.000 tick, đồ vật là chi phí (trọng lượng, năng
lượng thử) nhiều hơn là lợi ích cho sự sống còn; lợi ích của chúng là tri thức và mức phát triển.

**Bằng chứng.** `tools/lab.py run crafting --seeds 1-16` (docs/lab/crafting.md): phát minh −36,9 với KTC 95%
[−62,8, −12,8]; kiến thức −27,1 [−46,5, −8,8]; dân số đỉnh +1.337 [−720, +3.495]; thời đại cuối −1,4 [−3,1, +0,1].

**Lặp lại trên mặc định cuối**, 16 seed (docs/lab/crafting.md bản mới): **lặp lại, và mạnh hơn**. Phát minh
−62,8 với KTC 95% [−87,1, −37,8]; kiến thức −35,2 [−54,3, −17,4]; đồ vật mỗi người −0,81 [−1,25, −0,42];
thời đại cuối −2,19 [−3,62, −0,62]. Và cái giá hiện ra rõ hơn: không chế tác thì dân số đỉnh 4.516 so với
3.624, đất khoẻ hơn (+0,08, KTC [0,00, +0,17]) và định cư 93% so với 68%. Sáu chỉ số có khoảng tin cậy
không chứa 0, nhiều nhất trong mọi thí nghiệm của repo này.

**Độ tin.** **Có bằng chứng, đã lặp lại hai đợt.** Đây là kết quả chắc nhất của repo: chế tác mua tri thức
bằng dân số và bằng đất. Một xã hội không biết làm đồ vật thì đông hơn, ở yên hơn, đất tốt hơn, và dốt hơn.
Câu hỏi tiếp: chạy 60.000 tick để xem chi phí sớm có đổi thành lợi ích muộn không.

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

### 13. Bước học dài giết thế giới, việc học thì không

**Phát biểu.** Thay luật Hebb bằng actor-critic có vết đủ điều kiện **không** làm xã hội kém đi, miễn là
bước học đủ ngắn. Ở bước 0,1 thì 10 trên 16 thế giới tuyệt chủng; ở bước 0,01, cùng một cách học ấy ngang
bằng luật cũ về kết cục và biết gần gấp đôi. Phát biểu trước của tôi, rằng cái đầu học từ hậu quả thì hết
tò mò, **sai**: nó là hiện tượng của một tham số đặt sai, không phải của việc học.

**Bằng chứng.** `tools/lab.py run brains --seeds 1-16` trên thế giới v5 (docs/lab/brains.md):

| nhánh | kết cục tốt | tuyệt chủng | phát minh | kiến thức mỗi người | dân đỉnh |
|---|---|---|---|---|---|
| Hebb (mặc định) | 9/16 | 2 | 145 | 36,4 | 2.648 |
| chỉ lấy mẫu, không học | 6/16 | 1 | 153 | **74,1** | **4.468** |
| gradient, bước 0,1 | 3/16 | **10** | 17,5 | 0 | 1.000 |
| gradient + truyền nghề | 7/16 | 5 | 63 | 30,6 | 1.310 |
| gradient, bước 0,01 | **9/16** | 1 | **154** | 64,2 | 3.508 |

Nhánh bước 0,1 kém hơn mặc định ở tám chỉ số với khoảng tin cậy không chứa 0. Nhánh bước 0,01 chỉ khác
mặc định đúng ở độ dẻo não, tức là ở chính thứ được bật, và không khác ở bất kỳ chỉ số kết cục nào.

**Kết quả sạch nhất lại nằm ở nhánh đối chứng.** Chọn việc theo xác suất mà **không học gì cả** cho kiến
thức mỗi người 74,1 so với 36,4 của mặc định, hiệu số +31,0 với khoảng tin cậy [+0,7, +60,8], tức là khác
0. Đổi lại kết cục tốt chỉ 6/16 so với 9/16. Đọc thẳng: **một chút ngẫu nhiên trong hành động mua được
tri thức bằng sự ổn định**. Ai cũng làm đúng việc tối ưu thì không ai thử việc mới.

**Độ tin.** Có bằng chứng cho hai điều: bước học dài thì giết, và ngẫu nhiên thì làm giàu tri thức. Chưa
có bằng chứng rằng học bằng gradient hơn luật Hebb; nó chỉ ngang. Truyền nghề ngang cứu được nhánh hỏng
(7/16 so với 3/16) nhưng thua học chậm, nên giả thuyết cũ của tôi rằng gradient cộng bắt chước sẽ thắng
tất cả cũng **sai**.

**Bài học phương pháp, đắt nhất phiên này.** Tôi dò bước học trên một seed và 3.000 tick, thấy ổn, rồi kết
luận trên 16 seed và 20.000 tick rằng việc học tự nó có hại. Sai lầm không nằm ở chỗ thiếu dữ liệu mà ở
chỗ **dò tham số và kiểm định trên hai thang thời gian khác nhau**. Chế độ sàng lọc sinh ra từ đây.

### 14. Trần của thế giới, không phải trần của xã hội

**Phát biểu.** Mọi thế giới thịnh vượng đều ngừng phát minh ở khoảng tick 6.500 rồi đứng im suốt phần đời
còn lại, dù dân số vẫn lên xuống bình thường. Nguyên nhân không nằm trong xã hội mà nằm trong mã: kiến thức
được lưu trong một số nguyên 128 bit, nên một thế giới không thể giữ quá 128 phát minh cùng lúc.

**Bằng chứng.** Seed 3, 20.000 tick: số phát minh 16 ở tick 500, 69 ở tick 2.500, 115 ở tick 4.500, rồi
**128 từ tick 6.500 cho tới hết**. Kiến thức mỗi người dừng quanh 100 đến 119. Nhà vẫn tiếp tục mọc (3 lên
1.011) và ruộng vẫn mở, nên nhìn thì xã hội còn sống, nhưng không có ý tưởng mới nào nữa.

**Sau khi gỡ trần** (bitset rộng 512 bit, cùng seed, cùng mọi thứ khác): phát minh 128 lên **243** ở tick
20.000 và vẫn đang lên; kiến thức mỗi người 118 lên 135; thế giới vẫn thịnh vượng và đất còn tốt hơn (84%
so với 61%). Đường phát triển không còn nằm ngang.

**Độ tin.** Chắc chắn về nguyên nhân, vì đó là một hằng số trong mã chứ không phải một hiện tượng. Chưa biết
trần mới nằm ở đâu: 243 sau 20.000 tick chưa chạm 512, nên phải chạy 60.000 tick mới biết thế giới tự bão
hoà ở đâu, hay bão hoà là do vật lý vật liệu chứ không do chỗ chứa.

**Bài học.** Người dùng phát hiện ra điều này chỉ bằng cách ngồi xem, trước khi có bất kỳ chỉ số nào chỉ ra.
Một giới hạn kỹ thuật đọc y hệt một quy luật xã hội: "xã hội phát triển tới hạn rồi dừng" là một kết luận
nghe rất hợp lý, và hoàn toàn sai. Mọi phát biểu trong sổ này cần được hỏi lại một lần: đây là thế giới, hay
là chỗ chứa của tôi?

### 15. Xã hội ở đây không tìm được cân bằng, và càng già càng lắc mạnh

**Phát biểu.** Đây là câu hỏi ban đầu của cả dự án: để tự do thì xã hội có tìm được thế cân bằng không.
Chạy gấp ba thời gian thì câu trả lời là **không**. Không sụp đổ, không đi lên rồi đứng yên, mà dao động,
và biên độ dao động lớn dần theo tuổi của thế giới.

**Bằng chứng.** `tools/lab.py run long --seeds 1-4 --ticks 60000` (docs/lab/long.md), thế giới v5:

| | 20.000 tick (16 thế giới) | 60.000 tick (4 thế giới) |
|---|---|---|
| kết cục tốt | 9/16 | **1/4** |
| biên độ dao động | 3,4 | 9,5 |
| phát minh | 145 | 206 |
| kiến thức mỗi người | 36,4 | 43,8 |

**Nhưng phải trừ đi một phần là ảo.** Biên độ được đo trên một phần ba cuối của lần chạy, nên chạy dài gấp
ba thì cửa sổ đo cũng dài gấp ba, và một cửa sổ dài hơn đương nhiên bắt được nhiều cực trị hơn. Đo lại trên
**cùng một độ dài cửa sổ** (6.700 tick cuối, đúng bằng đuôi của một lần chạy 20.000):

| cách đo | biên độ trung vị |
|---|---|
| lần chạy 20.000, đuôi 6.700 tick | 3,4 |
| lần chạy 60.000, đuôi 6.700 tick | **5,5** |
| lần chạy 60.000, đuôi 20.000 tick | 9,5 |

Nên phần 3,4 lên 5,5 là **thật**: cùng một thước, thế giới già lắc mạnh hơn thế giới trẻ. Phần 5,5 lên 9,5
là do cửa sổ, không phải do thế giới.

**Hệ quả cho mọi số khác trong sổ.** Gần như tất cả đo ở 20.000 tick, tức là đo lúc thế giới còn trẻ và còn
êm. Số thế giới "tốt" trong mọi bảng là số thế giới đang ở pha lên khi ta bấm dừng. Điều này không làm các
so sánh giữa hai nhánh sai, vì mọi nhánh đều bị cắt ở cùng một chỗ, nhưng nó làm mọi **con số tuyệt đối**
lạc quan quá mức.

**Thế giới trần trụi thua hẳn ở đường dài.** Cùng 60.000 tick, nhánh `--bare` (không thủ lĩnh, không mệnh
lệnh, không tập quán): 0/4 kết cục tốt, phát minh 70,5 so với 206, kiến thức 12,6 so với 43,8, và tín hiệu
**bằng 0 ở cả hai đầu**. Giả thuyết "bỏ cái nạng đi thì ngôn ngữ mọc lên" bị bác lần thứ hai, lần này ở
thang thời gian mà nó đáng lẽ có cơ hội nhất. Khung xã hội viết tay không phải thứ chặn ngôn ngữ.

**Độ tin.** Bốn thế giới mỗi nhánh, nên đây là dấu hiệu mạnh chứ chưa phải bằng chứng theo chuẩn của sổ
này. Nhưng hướng thì nhất quán ở cả bốn, và phần tách cửa sổ ở trên loại được lời giải thích tầm thường.

### 16. Đất không phải cái tạo nhịp; thức ăn thì có vẻ là

**Phát biểu.** Mục 15 cho thấy xã hội dao động và càng già càng lắc mạnh. Nghi can đầu tiên là đất, vì
hái nhiều thì đất mòn, mòn thì đói, đói thì chết bớt, chết bớt thì đất hồi. **Sai.** Vòng qua đất không
điều khiển nhịp. Nghi can đúng hơn là vòng tiêu thụ tài nguyên qua chính thức ăn.

**Bằng chứng, phần bác bỏ.** `tools/lab.py run pace --seeds 1-4 --ticks 60000` (docs/lab/pace.md): đất
mòn nhanh gấp ba cho biên độ 3,55, mặc định 9,54, đất bền gấp mười 6,89. Nếu đất tạo nhịp thì đất bền phải
làm êm hẳn; nó không. Còn nhánh đất mỏng manh "êm" chỉ vì thế giới ở đó nhỏ và chết sớm: 2 tuyệt chủng, 1
suy tàn, kiến thức 17,8 so với 43,8. Chỉ số khác 0 đều là những thứ hiển nhiên phải đổi khi vặn đất.

**Bằng chứng, phần chỉ điểm.** Tương quan trễ giữa thức ăn và dân số trên các thế giới dài: **dân số dẫn
trước thức ăn 500 tick với r = −0,79** ở hai trên ba thế giới đo được. Đọc bằng lời: đông người thì nửa
nghìn tick sau thức ăn cạn. Đó là dấu vân tay của một vòng tiêu thụ tài nguyên có độ trễ, thứ luôn sinh
dao động. Tương quan giữa đất và dân số thì lung tung, từ −0,73 đến 0,00.

**Dự đoán ghi trước, và kết quả.** Dự đoán viết ra trước khi chạy: nếu thức ăn tạo nhịp thì vặn tốc độ mọc
lại phải đổi được dao động, mọc nhanh thì tài nguyên đuổi kịp miệng ăn và biên độ giảm, mọc chậm thì vọt
lên rồi sập mạnh hơn. `tools/lab.py run food --seeds 1-4 --ticks 60000` (docs/lab/food.md):

| nhánh | biên độ dao động | dân đỉnh | đất còn | kiến thức |
|---|---|---|---|---|
| mọc gấp đôi (0,16) | **4,32** | 6.677 | 82% | 27,9 |
| mặc định (0,08) | 9,54 | 2.679 | 95% | 43,8 |
| mọc một nửa (0,04) | 6,67 | 1.344 | 64% | 54,3 |

**Nửa trước của dự đoán đúng, và đúng ở mức đo được.** Thức ăn mọc nhanh gấp đôi làm biên độ giảm 6,15 với
khoảng tin cậy 95% [−12,72, −0,46], không chứa 0. Cùng với đó dân số đỉnh tăng gấp hai lần rưỡi và tỉ lệ
sinh tăng 6,16 [+0,12, +11,94]. Đây là lần đầu trong sổ này một dự đoán định lượng được viết ra trước rồi
mới đo, và nó đúng.

**Nửa sau thì không kiểm được.** Mọc chậm cho biên độ 6,67, tức là cũng thấp hơn mặc định chứ không cao hơn.
Nhưng những thế giới ấy chỉ đạt đỉnh 1.344 người: quá nhỏ để lắc mạnh, đúng cái bẫy đã gặp ở nhánh đất mỏng
manh. Không thể tách "êm vì cân bằng" khỏi "êm vì nghèo" bằng thiết kế này.

**Một điều ngoài dự đoán, đáng theo.** Thế giới thức ăn dồi dào có tiếng gọi **mang nghĩa hơn**: +0,06 bit
với khoảng tin cậy [0,00, +0,10]. Nhỏ, chạm 0, nhưng đây là lần đầu một can thiệp làm chỉ số nghĩa nhúc
nhích theo hướng tốt. Giả thuyết: dư dả thì nói được mà không chết đói, vì tiếng gọi tốn năng lượng.

**Độ tin.** Phần bác bỏ đất: bốn thế giới mỗi nhánh, hướng nhất quán, dấu hiệu mạnh. Phần thức ăn tạo nhịp:
**có bằng chứng** cho chiều mọc nhanh, khoảng tin cậy không chứa 0; chiều mọc chậm còn lẫn với hiệu ứng
dân số nhỏ. Vòng tiêu thụ tài nguyên là lời giải thích tốt nhất hiện có cho việc xã hội ở đây không tìm
được cân bằng.

### 17. Bản đồ là cái trần của mọi thứ khác

**Phát biểu.** Kích thước bản đồ, thứ tôi chưa bao giờ nghĩ là một biến, hoá ra ràng buộc gần như mọi con
số trong sổ này. Cùng số người xuất phát, đất rộng gấp bốn cho dân số đỉnh gấp gần bốn, kiến thức mỗi người
gấp gần năm và số phát minh gấp ba.

**Bằng chứng.** `tools/lab.py screen room --seeds 1-8 --ticks 8000` (8 thế giới mỗi nhánh):

| bản đồ | kết cục tốt | dân đỉnh | kiến thức | phát minh |
|---|---|---|---|---|
| 192×192 (mặc định) | 4/8 | 1.704 | 11,9 | 32,5 |
| 288×288 | 6/8 | 3.643 | 19,4 | 55 |
| 384×384 | 5/8 | 6.191 | 55,7 | 106 |

Dân số đỉnh +4.576 với khoảng tin cậy không chứa 0; dân số cuối +1.744 cũng vậy. Nối với mục 4 (phát minh
là hàm của số lần thử) và mục 16 (thức ăn tạo nhịp) thì mạch rất rõ: đất rộng nuôi nhiều người, nhiều
người thử nhiều hơn, nên biết nhiều hơn.

**Cái giá.** 280 tick mỗi giây trên bản đồ 384×384 so với 912 trên 192×192, tức chậm hơn ba lần rưỡi. Với
một máy chạy liên tục, đổi ba tick lấy một xã hội giàu gấp năm là món hời.

**Độ tin.** Mới sàng lọc 8 thế giới nhưng hai chỉ số dân số có khoảng tin cậy không chứa 0 và hướng nhất
quán ở cả ba nhánh. Hệ quả thực tế: **mọi kết quả trong sổ này đo trên một thế giới chật**, và con số tuyệt
đối của chúng thấp hơn thế giới rộng rãi rất nhiều. Các so sánh giữa hai nhánh vẫn đúng vì cùng bị chật
như nhau.

### 18. Giả thuyết ngôn ngữ thứ tư: chưa thấy, và có lý do chính đáng để chưa thấy

**Phát biểu.** Cho não một ô nhớ vị trí và cho thế giới những nguồn lợi giàu không nhìn thấy từ xa, tức là
lần đầu tiên có thứ **đáng nói**, vẫn chưa làm tiếng gọi mang nghĩa trong 8.000 tick.

**Bằng chứng.** `tools/lab.py screen finds` (8 thế giới mỗi nhánh): nghĩa của tiếng gọi 0,027 ở nhánh
thường, 0,014 khi có nguồn lợi, 0,044 khi có nguồn lợi mà điếc. So nghe với điếc trong thế giới có nguồn
lợi: không chỉ số nào có khoảng tin cậy loại được 0.

**Nhưng nguồn lợi làm thế giới giàu hẳn**: kiến thức mỗi người 43,4 so với 6,1, phát minh 87,5 so với 36.
Cơ chế hoạt động; chỉ có phần ngôn ngữ là chưa.

**Cách đọc, và chỗ khác với ba lần trước.** Ba giả thuyết trước bị bác vì cơ chế có sẵn mà tiến hoá không
dùng. Lần này **cổng ghi nhớ vừa mới sinh ra**: tiến hoá mới có 8.000 tick để tìm ra rằng nên bật nó, rồi
lại phải tìm ra rằng nên kêu lên khi bật. Hai bước, trên một hành vi chưa từng tồn tại. Sàng lọc 8.000 tick
không đủ để bác điều đó, và nói nó đã bị bác là sai phương pháp.

**Độ tin.** Chưa kết luận. Đây là giả thuyết đầu tiên phải giao cho thế giới chạy liên tục hàng triệu tick
chứ không phải cho một lần sàng lọc, và đó cũng là lý do đáng chạy VPS.

## Những câu hỏi mở

1. **Ngôn ngữ có xuất hiện không?** Chưa. Cái đã có là đọc trạng thái người khác qua tiếng gọi (mục 9), và
   hai lần loé lên 0,27 đến 0,31 bit rồi tắt. Giả thuyết "chỉ họ hàng nghe thì ngôn ngữ sẽ ra" đã bị bác. Giả
   thuyết tiếp theo đáng thử: tín hiệu chỉ có nghĩa khi có việc cần phối hợp mà một người không làm nổi. Thế
   giới có **bầy thú** chỉ ngã khi ít nhất hai người đánh gần cùng lúc (mục 12): chưa đủ, và còn có hại. Giả
   thuyết tiếp theo: việc phối hợp phải rẻ khi thử và chỉ đắt khi bỏ dở, ví dụ dựng một công trình lớn cần
   nhiều người góp vật liệu, không ai mất gì nếu góp một mình.
   **Giả thuyết cái nạng, và lần nhìn đầu tiên bác nó.** Lập luận: chừng nào mệnh lệnh còn điều phối
   giúp, tiếng gọi không có việc gì để làm, nên phải lấy hết khung xã hội đi thì ngôn ngữ mới có lý do
   tồn tại. Cờ `--bare` bỏ hẳn thủ lĩnh, mệnh lệnh và tập quán, chỉ để lại thân thể, sáu hành động và
   tiếng gọi. Sàng lọc 8 thế giới, 8.000 tick: nghĩa trung vị 0,012 so với 0,015 của mặc định, hiểu
   0,007 so với 0,019, tức là **không khá hơn, phần hiểu còn tệ đi**; kiến thức mỗi người 3,6 so với
   11,9. Thế giới cao nhất ở nhánh trần có nghĩa 0,125 so với 0,065, nên vẫn còn một chút đáng nhìn ở
   đuôi phân phối, nhưng trung vị nói không.
   Cách đọc: bỏ cái nạng đi không đủ. Một tiếng gọi chỉ đáng phát khi có việc mà biết thêm một điều từ
   người khác thì làm được còn không biết thì không, và thế giới này chưa có việc nào như thế. Điều
   cần thử tiếp không phải nhiều seed hơn mà là **chạy dài hơn nhiều**, vì trong mọi đợt trước, những
   lần tiếng gọi loé lên có nghĩa đều xảy ra muộn.

   Từ giờ ngôn ngữ được đo ở **hai đầu kênh**: `signal_meaning` là thông tin tương hỗ giữa điều một người
   nói và tình trạng của chính người đó (đói hay no, sợ hay không: sáu lớp), tức là tiếng gọi *có nội dung*;
   `signal_mi` là giữa điều nghe được và việc làm ngay sau, tức là tiếng gọi *được hiểu*. Một tiếng gọi có
   thể có nội dung mà không ai hiểu, hoặc "được hiểu" mà không có nội dung khi hàng xóm chỉ cùng cảnh ngộ.
   Sự kiện "tiếng gọi bắt đầu có nghĩa" nay đòi cả hai từ 0,2 bit. Trong một thế giới thử (seed 3, 3.000
   tick), nội dung tăng dần từ 0 lên 0,09 bit khi dân số lên 2.000, trong khi hiểu đứng ở 0,08: tiếng gọi
   bắt đầu phản ánh người nói trước khi ai đó dùng được nó.
2. **Học trong đời học được gì?** Chưa đo được gì bền (mục 8): lợi ích ở đợt đầu biến mất khi người lạ nghe
   được nhau. Tiến hoá không đẩy tốc độ học lên: `plastic` trung vị 0,01, `learn_rate` 2 đến 3 phần nghìn.
   Câu hỏi còn lại: có việc nào trong thế giới này mà chỉ học trong đời mới làm nổi, còn tiến hoá và nghe thì
   không? Nếu không có, học là chi phí thuần.
6. **Trả công cho tò mò thì được gì?** Mục 13 cho thấy cái đầu học từ hậu quả bỏ hẳn chế tác. Thử
   thêm một phần thưởng nhỏ cho việc biết thêm điều mới sẽ cứu được sáng tạo, nhưng lúc đó ta đang
   tự tay viết vào thế giới đúng cái ta muốn quan sát nó tự mọc lên. Câu hỏi thật: có cách đặt phần
   thưởng nào chỉ nói về sinh tồn mà vẫn để sáng tạo sống sót không?
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

## Thế giới có phiên bản

Mỗi con số trong sổ này là số đo của **một** thế giới, và luật của thế giới ấy đã đổi nhiều lần: biển
tới, rồi vật liệu và chế tác, rồi nghe được người lạ, rồi trần kiến thức. Mỗi lần như vậy mọi bảng đo
trước đó chết lặng lẽ, và hơn một phát biểu trong sổ đã phải rút lại **chỉ vì lý do đó**, không vì lý do
nào khác.

Nên luật giờ mang số phiên bản, trong `sim/src/version.rs`. Sim in nó ra ở dòng đầu mỗi lần chạy
(`world=v5`), ghi nó vào cột đầu của mọi bảng kết quả, và mỗi báo cáo trong docs/lab ghi rõ nó đo trên
thế giới nào. Quan trọng nhất: **phòng thí nghiệm từ chối so hai nhánh khác phiên bản** và bảo chạy lại,
thay vì lặng lẽ lấy trung bình của hai vũ trụ khác nhau.

Đổi hành vi thì tăng số phiên bản trong cùng commit đó, ghi một dòng vào NOTE và HISTORY. Nếu vân tay
trong `tools/check.py` dịch chuyển mà số phiên bản không dịch theo, bộ kiểm tra sẽ nói thẳng ra. Đó
đúng là cái nó sinh ra để bắt.

| phiên bản | thế giới |
|---|---|
| v1 | biển và thuyền |
| v2 | vật liệu, chế tác, đồ vật, nơi trú |
| v3 | tắt bầy thú, nghe được người lạ |
| v4 | não học bằng gradient, truyền nghề ngang |
| v5 | 512 phát minh, mỗi thế giới tự đặt tên |

## Biết mình làm hỏng gì trong 15 giây

Trước khi có mục này, cách duy nhất để biết một thay đổi làm hỏng thứ gì là chạy một thí nghiệm và
chờ mười lăm phút. Mục 14 cho thấy cái giá của việc đó: sửa một trần trong mã xong là mọi con số đo
trước đều phải đo lại, mà không có cách nào biết cái gì đã đổi ngoài chạy lại tất cả.

`python3 tools/check.py` chạy trong khoảng mười lăm giây và trả lời đúng hai câu hỏi hay sai nhất:

- **Có gì vỡ không.** Mười ba bài kiểm tra đơn vị cho phần logic thuần: bộ bit kiến thức (đặt, đếm,
  duyệt, và quan trọng nhất là bỏ qua ô vượt giới hạn thay vì quay vòng về ô 0), softmax của chính
  sách, giới hạn một bước học (đúng cái lỗi đã giết mọi thế giới), độ phai của vết, thông tin tương
  hỗ, và việc mỗi thế giới tự đặt tên riêng.
- **Thế giới nào đổi hành vi.** Bảy cấu hình chạy trên bản đồ nhỏ, toàn bộ bảng thống kê được băm
  thành một vân tay. Vì sim tất định, vân tay không đổi chứng minh thế giới giống hệt tới chữ số
  cuối; vân tay đổi thì nó gọi tên đúng cấu hình đã dịch chuyển.

Đổi hành vi có chủ ý thì `--bless` ghi lại mốc mới, và commit khi đó mang theo bằng chứng chính xác
những thế giới nào đã dịch chuyển. Đây là thứ đáng lẽ phải có từ đầu.

## Chạy nhanh hơn: sàng lọc trước, chạy đầy đủ sau

Một nhánh đầy đủ là 16 thế giới nhân 20.000 tick, khoảng 19 phút trên một máy bốn lõi. Bốn nhánh là hơn một
tiếng. Chờ chừng đó chỉ để biết một ý tưởng không đi đến đâu là lãng phí.

Chi phí dồn về cuối, vì càng về sau càng đông người. Đo trên 16 thế giới mặc định:

| chạy tới tick | phần chi phí |
|---|---|
| 2.500 | 3,4% |
| 5.000 | 12,0% |
| 8.000 | 22,9% |
| 10.000 | 34,4% |
| 15.000 | 68,5% |

Cắt xuống 8.000 tick và 8 seed còn khoảng **một phần chín** công sức. Câu hỏi là cắt như thế có đổi kết luận
không. Kiểm tra lại trên mọi thí nghiệm đã chạy đầy đủ, bằng cách hỏi thứ hạng các nhánh ở tick T có trùng
thứ hạng cuối cùng không:

- Nơi hiệu ứng lớn (chế tác, thí nghiệm bộ não, dân số) thứ hạng đã đúng từ tick 2.500 đến 8.000.
- Nơi thứ hạng còn nhảy ở tick 8.000 (học trong đời, nghe nhau, tập quán, bầy thú) thì chạy đủ 20.000 tick
  cũng **không** tìm ra hiệu ứng nào đáng báo cáo.

Nói cách khác: sàng lọc tách được nhánh là lý do để chạy đầy đủ; sàng lọc không tách được là lý do để dừng,
không phải lý do để chạy lâu hơn. Lệnh:

```bash
python3 tools/lab.py screen brains          # 8 seed, 8.000 tick, vài phút
python3 tools/lab.py run brains --seeds 1-16   # chỉ khi sàng lọc tách được
```

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
