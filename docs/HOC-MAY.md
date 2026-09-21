# Một bộ não nhỏ được huấn luyện thế nào

Tài liệu này giải thích từ đầu cách một model AI nhỏ hoạt động và được dạy, bám đúng vào mã và
đúng vào những con số đo được của repo này. Không có công thức nào ở đây trừu tượng hơn mức cần
thiết, và mọi con số đều lấy từ một lần chạy thật, ghi lại được.

Não trong thế giới này là loại nhỏ nhất còn đáng gọi là não: **101 đầu vào, 20 nơ-ron ẩn, 22 đầu
ra, 2.502 con số**. Bằng khoảng một phần triệu của một model ngôn ngữ nhỏ. Nó vừa đủ nhỏ để chạy
4.000 bản sao cùng lúc trên một máy tính bình thường, và vừa đủ lớn để làm được những việc không
tầm thường.

## 1. Một model là gì, nói cho gọn

Một model là **một danh sách số cộng với một quy tắc dùng danh sách đó**. Không có gì hơn.

Trong `sim/src/brain.rs`, danh sách số là `weights`, và quy tắc là hàm `think`. Quy tắc gồm ba
bước, lặp đi lặp lại:

1. Nhân mỗi đầu vào với một trọng số rồi cộng lại, ra 20 con số. Đây là "nhân rồi cộng", việc duy
   nhất mà mọi mạng nơ-ron làm.
2. Ép mỗi con số về khoảng từ -1 đến 1 bằng hàm `tanh`. Bước này gọi là **phi tuyến**, và nó là lý
   do một mạng nhiều lớp mạnh hơn một lớp. Bỏ nó đi thì hai lớp chồng lên nhau rút gọn lại thành
   một lớp, và mạng không học nổi bất cứ điều gì có dạng "nếu vừa đói vừa có người lạ gần thì...".
3. Lại nhân rồi cộng một lần nữa, ra 22 con số đầu ra. Sáu trong số đó là điểm cho sáu hành động;
   số cao nhất là việc sẽ làm.

Cả não tốn 2.460 phép nhân mỗi người mỗi tick. Tôi đo phần này chiếm khoảng **3%** thời gian một
tick, bằng cách dựng một bản bắt mỗi agent nghĩ hai lần rồi bỏ kết quả thừa.

## 2. Ba cách một model có thể học

Đây là chỗ hầu hết tài liệu nhảy thẳng vào "gradient descent" và bỏ qua mất bức tranh. Thế giới
này có đủ cả ba cách, và chúng khác nhau về chất.

**Di truyền.** Con nhận trọng số của cha mẹ, trộn lại và thêm nhiễu nhỏ. Ai sống lâu và sinh nhiều
thì trọng số của người đó lan ra. Không cá thể nào học gì cả; cả quần thể học. Đây là cách duy
nhất repo này có lúc đầu, và nó vẫn là nền.

**Học trong đời từ hậu quả.** Một cá thể đổi trọng số của chính mình dựa trên chuyện xảy ra với
nó. Đây là phần tài liệu này nói kỹ.

**Học từ người khác.** Một cá thể dịch trọng số của mình về phía trọng số của người sống tốt hơn.
Không cần hiểu, không cần thử, chỉ cần bắt chước. Cách này lan nhanh hơn di truyền vì không phải
đợi ai chết, và nó tạo ra thứ có thể gọi là văn hoá: một lối cư xử truyền đi mà không có gene nào
truyền theo.

## 3. Vấn đề thật sự: quy công cho quá khứ

Giả sử một agent hái quả ở tick 100 và no bụng ở tick 130. Làm sao não biết việc đáng thưởng là
việc làm ở tick 100, chứ không phải việc đang làm ở tick 130?

Luật học cũ của repo (Hebb có điều biến) **không biết**. Nó chỉ nối hoạt động đang diễn ra với
phần thưởng đang tới. Việc làm 30 tick trước đã trôi mất.

Đây gọi là **bài toán quy công theo thời gian**, và nó là vấn đề trung tâm của học tăng cường.
Cách giải chuẩn có hai mảnh, và cả hai đều đã có trong repo này.

### Mảnh thứ nhất: nhà phê bình

Thêm một đầu ra nữa, chỉ để trả lời một câu: **"tình cảnh lúc này đáng bao nhiêu?"** Gọi là giá
trị, ký hiệu V.

Tại sao cần? Vì phần thưởng thô không nói lên điều gì. Một agent đang đứng giữa đồng quả chín thì
tick nào cũng được thưởng, dù nó làm gì. Nếu cứ thấy thưởng là khen việc vừa làm, thì mọi việc làm
trong hoàn cảnh tốt đều được khen, kể cả việc dở. Nhà phê bình cho ta một **mốc so sánh**: thay vì
hỏi "có được thưởng không", ta hỏi "có được thưởng **hơn mức đáng lẽ** không".

Con số đó gọi là **sai số dự báo thời gian**, viết là `td`:

```
td  =  thưởng vừa nhận  +  γ × (giá trị lúc này)  -  (giá trị lúc trước)
```

γ (gamma) là mức coi trọng tương lai, mặc định 0,95, nghĩa là nhìn xa khoảng 20 tick.

Đọc bằng lời: *"những gì thực sự xảy ra tốt hơn hay tệ hơn điều tôi đã tưởng?"* Dương là bất ngờ
tốt. Âm là bất ngờ xấu. Bằng không là đúng như dự đoán, và khi đó **không học gì cả**, điều này
đúng: một thế giới đã đoán được thì không dạy ta điều gì.

Nhà phê bình trong repo này sinh ra trắng tinh ở mỗi đời người, học trong đời, và **không di
truyền**. Nó cũng không nằm trong gene, nên mọi thế giới đã chạy từ trước vẫn tái hiện y hệt.

### Mảnh thứ hai: vết đủ điều kiện

Giữ một bản ghi mờ dần về **những trọng số nào đã góp phần vào các lựa chọn gần đây**. Mỗi tick,
bản ghi cũ nhân với một hệ số nhỏ hơn 1 rồi cộng thêm phần của tick này. Việc làm càng lâu thì dấu
vết càng mờ.

Khi `td` đến, nó nhân vào toàn bộ vết. Nghĩa là một phần thưởng đến muộn vẫn tìm được đường về
đúng những lựa chọn đã dẫn tới nó, mờ dần theo độ xa. Hệ số mờ là γ nhân λ (mặc định 0,95 × 0,9 =
0,855), cho tầm với khoảng 7 tick.

Đây chính là thứ luật Hebb không có, và là toàn bộ khác biệt giữa "phản xạ" và "học".

## 4. Gradient, nói cho dễ hiểu

Người ta hay nói "huấn luyện là đi xuống theo gradient". Thực chất là thế này.

Với mỗi trọng số, hỏi: **"nếu tôi tăng con số này lên một chút, xác suất tôi chọn việc vừa rồi
tăng hay giảm?"** Câu trả lời cho từng trọng số, gộp lại, chính là gradient.

Với lớp cuối và cách tính xác suất bằng softmax, câu trả lời rút gọn lại đẹp đến bất ngờ:

```
với việc đã chọn:      (1 - xác suất của nó)  ×  hoạt động của nơ-ron ẩn
với các việc không chọn:  (0 - xác suất của nó)  ×  hoạt động của nơ-ron ẩn
```

Đọc bằng lời: *"nâng cái tôi đã làm lên, hạ những cái tôi cũng đã thấy hấp dẫn xuống, mỗi thứ theo
mức độ tôi sẽ ngạc nhiên nếu đã làm nó."* Việc nào đằng nào cũng gần như chắc chắn được chọn thì
gần như không được nâng thêm, vì chẳng học được gì mới từ nó.

Đó là toàn bộ "gradient" ở đây. Nó nằm trong hàm `trace_step` của `brain.rs`, dài đúng mười dòng.

## 5. Vì sao phải ngẫu nhiên

Não cũ chọn việc có điểm cao nhất, luôn luôn. Một não như thế **không học được bằng hậu quả**, vì
nó không bao giờ biết những việc nó không làm thì ra sao. Không thử thì không có dữ liệu.

Nên não gradient chọn việc theo xác suất, bằng **softmax**: điểm cao thì hay được chọn, điểm thấp
vẫn còn cơ hội. Nhiệt độ quyết định mức liều: nhiệt độ về 0 thì thành "luôn chọn cái cao nhất",
nhiệt độ cao thì thành ném xúc xắc.

Việc rút thăm này phải **tái lập được**, nếu không cùng một seed sẽ ra hai thế giới khác nhau tuỳ
số luồng CPU. Nên nó không dùng bộ sinh ngẫu nhiên chung, mà băm từ *ai đang hỏi* và *lúc nào*
(hàm `roll`). Cùng một agent, cùng một tuổi, luôn ra cùng một con số.

## 6. Bài học đắt nhất: bước học phải tự chuẩn hoá

Bản đầu tiên tôi viết theo đúng sách: `trọng số += tốc_độ × td × vết`. Nó **giết sạch mọi thế
giới**. Dân số rơi từ 1.000 xuống 62.

Chuyện gì đã xảy ra, đọc được ngay từ chỉ số phân công lao động, vọt từ 0,55 lên 0,98: mỗi người
khoá chặt vào đúng một hành động rồi chết đói. Đó là dấu vân tay của một **vòng phản hồi dương**.

Nguyên nhân: vết là tổng tích luỹ, nên nó dài ra tới khoảng 7 lần một bước đơn lẻ. Bước học vì thế
lớn gấp 7 lần dự tính. Nhà phê bình vọt qua đích, quay lại vọt quá đích phía kia, sai số không bao
giờ lắng, và cái sai số dao động ấy lại được bơm thẳng vào bộ chọn việc dưới dạng nhiễu. Bộ chọn
việc nâng bừa một việc lên, việc đó được chọn nhiều hơn, lại được nâng nữa.

| bước học | dân số ở tick 1.500 | sai số dự báo | phân công lao động |
|---|---|---|---|
| 0,001 | 906 | 0,12 | bình thường |
| 0,003 | 119 | 1,49 | 0,84 |
| 0,01 | 91 | 3,08 | 0,94 |
| 0,03 | 70 | 2,32 | 0,98 |

Cách sửa không phải là hạ tốc độ xuống thật thấp, vì như thế thì không học được gì. Cách sửa là
**chia cho độ lớn của chính cái vết**:

- Nhà phê bình sửa đúng một phần cố định của sai số của chính nó, mặc định một phần mười. Cái này
  trong xử lý tín hiệu gọi là bước chuẩn hoá, và nó ổn định bất kể tín hiệu to hay nhỏ.
- Bộ chọn việc bước một đoạn **dài cố định** theo hướng mà vết chỉ, thay vì một đoạn tỉ lệ với độ
  dài của vết. Nhờ vậy một khoảnh khắc ồn ào không thể quật ngã nó.

Sau khi sửa, cùng dải tốc độ đó không còn sụp nữa, và sai số dự báo xuống còn 0,08. Nhà phê bình
đã thật sự biết đoán:

| bước học (đã chuẩn hoá) | dân số ở tick 1.500 | sai số dự báo | phân công lao động |
|---|---|---|---|
| 0,001 | 665 | 0,075 | 0,51 |
| 0,01 | 638 | 0,076 | 0,51 |
| 0,03 | 704 | 0,095 | 0,44 |
| 0,1 | 524 | 0,069 | 0,55 |
| 0,3 | 126 | 0,017 | 0,84 |

Trên 0,3 vẫn còn vách. Đó là điều bình thường của học tăng cường, và là lý do người ta luôn báo
cáo tốc độ học kèm kết quả.

## 7. Kết quả đầu tiên, và nó không như mong đợi

Chạy một thế giới 3.000 tick, cùng seed, bốn cách học:

| cách | dân số | kiến thức mỗi người | đồ vật mỗi người |
|---|---|---|---|
| Hebb (như cũ) | 2.125 | 47,3 | – |
| chỉ lấy mẫu ngẫu nhiên, **không học** | 2.826 | 43,3 | 2,22 |
| gradient, bước 0,1 | 1.436 | 10,1 | – |
| gradient cộng học từ người khác | 1.226 | 5,1 | 1,25 |

Nhánh thứ hai là nhánh quan trọng nhất, và nó là lý do phải luôn có nhánh đối chứng. Nó cho thấy
việc chọn việc theo xác suất **không hề có hại**, thậm chí còn hơi tốt. Vậy thứ làm kiến thức rơi
từ 43 xuống 10 chính là **việc học**, chứ không phải sự ngẫu nhiên.

Vì sao? Vì phần thưởng trong thế giới này là của cải và tâm trạng **tăng ngay trong tick đó**. Chế
tác thì tốn năng lượng ngay lập tức và chỉ trả công về sau, rải rác. Một cái đầu học từ hậu quả
trước mắt sẽ học được rằng chế tác là việc dở, và bỏ.

Nói gọn lại thành một câu đáng ghi vào sổ: **một cái đầu học từ hậu quả của chính mình sẽ trở nên
hiệu quả và hết tò mò.** Đây không phải lỗi cài đặt; đây là một tính chất thật của cách đặt phần
thưởng, và nó cũng là lý do người ta phải nghĩ ra phần thưởng cho sự tò mò trong ngành AI.

Câu hỏi tiếp theo hiển nhiên: nới tầm nhìn của nhà phê bình ra xa hơn thì có cứu được không? Tôi
thử luôn, cùng seed, cùng 3.000 tick:

| tầm nhìn γ | dân số | kiến thức mỗi người | đồ vật mỗi người |
|---|---|---|---|
| 0,80 (khoảng 5 tick) | 1.253 | 8,1 | 1,55 |
| 0,95 (mặc định, khoảng 20 tick) | 1.436 | 10,1 | – |
| 0,99 (khoảng 100 tick) | 1.052 | 7,7 | 1,40 |
| 0,95 nhưng liều hơn (nhiệt độ 0,5) | 1.406 | 10,2 | 2,30 |

**Không cứu được.** Nhìn xa gấp năm lần không làm kiến thức nhúc nhích, thậm chí hơi tệ hơn. Chỉ
có liều hơn mới kéo số đồ vật mỗi người về mức cũ, mà kiến thức thì vẫn nằm im ở mức một phần tư.

Đọc kết quả này cho đúng: vấn đề không nằm ở chỗ nhà phê bình nhìn gần. Nó nằm ở chỗ **phần thưởng
không hề chứa khái niệm "biết thêm một điều"**. Cái gì không được trả công thì không được học, dù
nhìn xa đến đâu. Muốn có tò mò thì phải trả công cho tò mò, và lúc đó ta lại đang tự tay viết vào
thế giới cái mà lẽ ra ta muốn quan sát nó tự mọc lên. Đây đúng là chỗ khó thật của ngành, không
phải một lỗi của repo này.

## 8. Những gì tài liệu này cố tình đơn giản hoá

Nói thẳng để bạn không bị hiểu sai khi đọc tài liệu khác:

- **Chỉ lớp cuối học trong đời.** Lớp ẩn 20 nơ-ron do tiến hoá nặn, không đổi trong một đời người.
  Làm thế vì rẻ hơn nhiều (không cần truyền ngược qua lớp ẩn), ổn định hơn nhiều, và tốn chưa tới
  1% thời gian một tick. Trong ngành đây gọi là học tuyến tính trên đặc trưng có sẵn, và nó là một
  lựa chọn chính đáng, không phải đường tắt.
- **Chỉ đầu ra hành động học.** Các đầu ra khác (trí nhớ, mệnh lệnh, tín hiệu) trong nhánh gradient
  không học trong đời. Nghĩa là mọi so sánh về ngôn ngữ giữa nhánh Hebb và nhánh gradient đều
  không công bằng, và phải nói rõ điều đó.
- **Không có lô dữ liệu, không có kỷ nguyên huấn luyện.** Model ở đây học trực tuyến, mỗi tick một
  bước, giữa lúc đang sống. Model lớn thì huấn luyện theo lô, trên dữ liệu tĩnh, trước khi dùng.
  Toán nền giống nhau; nhịp thì khác hẳn.

## 9. Tự chạy thử

```bash
cd sim && cargo build --release && cd ..

# não cũ
sim/target/release/sim --seed 3 --ticks 3000 --out out

# não gradient
sim/target/release/sim --seed 3 --ticks 3000 --gradient --out out

# ngẫu nhiên nhưng không học: nhánh đối chứng quan trọng nhất
sim/target/release/sim --seed 3 --ticks 3000 --gradient --actor-rate 0 --critic-rate 0 --out out

# thêm kênh học từ người khác
sim/target/release/sim --seed 3 --ticks 3000 --gradient --know-rate 0.3 --out out
```

Các cột đáng theo dõi trong `out/stats_seed3.csv`: `td_error` (nhà phê bình đoán sai bao nhiêu,
càng xuống càng tốt), `mean_value` (nó định giá hiện tại ra sao), `plastic` (học được bao nhiêu),
`division_of_labour` (vọt lên gần 1 là dấu hiệu sụp do khoá cứng một hành động), `know_gifts` (bao
nhiêu lần có người truyền lại cái mình học được).
