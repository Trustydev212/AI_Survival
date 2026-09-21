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

### 13. Cái đầu học từ hậu quả của chính mình thì hiệu quả và hết tò mò

**Phát biểu.** Thay luật Hebb bằng actor-critic có vết đủ điều kiện làm kiến thức mỗi người rơi
khoảng bốn lần, trong khi vẫn sống được. Thủ phạm không phải sự ngẫu nhiên của chính sách, mà là
chính việc học.

**Bằng chứng.** Một thế giới (seed 3, 3.000 tick), cùng bản đồ, bốn cách học: Hebb cho dân số 2.125
và kiến thức 47,3; chỉ chọn việc theo xác suất mà **không học gì** cho 2.826 và 43,3; bật học
gradient cho 1.436 và 10,1; thêm kênh truyền lại cái đã học cho 1.226 và 5,1. Nhánh không học là
nhánh quyết định: nó chứng minh việc rút thăm hành động không hề có hại, nên phần kiến thức mất đi
là do học. Nới tầm nhìn của nhà phê bình từ 20 tick lên 100 tick không cứu được (kiến thức 7,7);
chỉ liều hơn mới kéo lại số đồ vật mỗi người (2,3) chứ không kéo lại kiến thức.

**Cách đọc.** Phần thưởng của thế giới này là của cải và tâm trạng tăng **ngay trong tick đó**. Chế
tác tốn ngay và trả công muộn, rải rác. Một cái đầu học từ hậu quả trước mắt học được rằng chế tác
là việc dở, và bỏ. Cái gì không được trả công thì không được học, dù nhìn xa đến đâu.

**Độ tin.** Mới một thế giới, chưa chạy 16 seed; coi là dấu hiệu mạnh chứ chưa phải bằng chứng.
Một điều phải nói rõ: trong nhánh gradient chỉ các đầu ra hành động học trong đời, còn trí nhớ,
mệnh lệnh và tín hiệu thì không, nên mọi so sánh về ngôn ngữ giữa hai nhánh là không công bằng.

**Bài học kỹ thuật, ghi lại vì nó tốn nhiều lần chạy.** Bản đầu cập nhật trọng số theo đúng công
thức sách vở và giết sạch mọi thế giới: dân số 1.000 xuống 62, phân công lao động vọt lên 0,98,
tức mỗi người khoá cứng vào một hành động rồi chết. Nguyên nhân là vết tích luỹ dài gấp khoảng bảy
lần một bước đơn lẻ, khiến nhà phê bình dao động và bơm nhiễu vào bộ chọn việc. Sửa bằng cách chuẩn
hoá cả hai bước theo độ lớn của chính cái vết. Sau đó sai số dự báo xuống 0,08 và không còn sụp
trên cả dải rộng. Chi tiết trong docs/HOC-MAY.md.

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
