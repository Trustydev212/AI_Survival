# Dự án này dạy được gì cho một sinh viên AI

Viết cho người học máy, không phải cho người chơi game. Mỗi mục dưới đây trỏ tới mã thật và tới một
kết quả đo được trong repo này, vì một khái niệm chỉ thành kỹ năng khi bạn đã thấy nó sai trước mắt.

## 1. Hàm mất mát không phải mục tiêu

Chạy `python3 tools/learning.py`. Nó quét tốc độ học của bộ não gradient và in ra bảng. Lần chạy hiện
tại trong `docs/lab/learning-curves.md`:

| cấu hình | dân cuối | kiến thức | sai số dự báo |
|---|---|---|---|
| actor 0,003 | 937 | 24,8 | 0,072 |
| actor 0,3 | **1** | **0,0** | **0,011** |

Cấu hình đoán giỏi nhất là cấu hình giết sạch thế giới. Đây là sai lầm phổ biến nhất của máy học ứng
dụng, và ở đây bạn nhìn thấy nó bằng số của chính mình thay vì đọc trong sách.

## 1b. Và đây là phép đo chứng minh model thật sự học được

Câu hỏi nặng nhất với một dự án kiểu này: **cái mạng đó có giỏi lên thật không, hay chỉ là xã hội may
mắn?** Mọi chỉ số xã hội đều trộn bộ não với đất đai, hàng xóm và vận may. Phép đo đúng phải tách bộ não
ra khỏi thế giới đã sinh ra nó.

`--eval` làm việc đó: lấy các bộ não trong ark, thả hai mươi bản sao của **từng bộ** vào một thế giới
mới tinh 64×64 mà không bộ não nào từng sống ở đó, không có dòng họ nào khác, năm lần thử mỗi bộ, rồi
đếm còn lại bao nhiêu người sau 2.500 tick. Đó là **tập kiểm định tách rời** theo đúng nghĩa thông thường.

```bash
sim --forever --ticks 30000 --out world/     # huấn luyện
sim --eval world/ark.bin --ticks 2500        # chấm điểm
```

| bộ não | điểm trung bình |
|---|---|
| ngẫu nhiên, chưa huấn luyện | **9,3** |
| từ ark, sau 8.000 tick | 72,4 |
| từ ark, sau 32.000 tick | **331,9** |

Gấp ba mươi lăm lần bộ não ngẫu nhiên, và **điểm tăng theo thời gian huấn luyện**. Đây là đường học của
tiến hoá thần kinh, đo trên dữ liệu chưa từng thấy. Mạng này học thật.

**Một chỗ phải phân biệt cho đúng.** Con số trên chứng minh **tiến hoá trọng số** hiệu quả. Nó **không**
chứng minh phần học trong đời (Hebb hay actor-critic) đóng góp thêm; mục 3 dưới đây cho thấy phần đó vẫn
chưa thắng nổi đường cơ sở ngẫu nhiên. Hai cơ chế khác nhau, hai kết luận khác nhau, và gộp chúng lại là
sai. Tôi đã từng gộp.

## 2. Siêu tham số không phải chú thích cuối trang

Cùng một thuật toán, đổi mỗi bước học: 0,01 thì xã hội ngang đối chứng, 0,1 thì mười trên mười sáu thế
giới tuyệt chủng (docs/lab/brains.md). Tôi đã kết luận sai một lần vì dò tham số trên một seed và ba
nghìn tick rồi kiểm định trên mười sáu seed và hai mươi nghìn tick. Mục 13 của `docs/THEORY.md` ghi lại
cả kết luận sai lẫn lý do.

## 3. Đường cơ sở tầm thường thường thắng

Nhánh **chọn việc theo xác suất mà không học gì** cho kiến thức mỗi người 74,1 so với 36,4 của bộ não
biết học, khoảng tin cậy không chứa 0. Nếu phương pháp của bạn không thắng nổi ngẫu nhiên thì phương
pháp chưa chứng minh được gì. Luôn chạy nhánh đó.

## 4. Thiết kế phần thưởng quyết định hành vi, không phải kiến trúc

Phần thưởng ở đây là của cải và tâm trạng tăng **ngay trong tick đó**. Hệ quả đo được: não học từ hậu
quả bỏ hẳn chế tác, vì chế tác tốn ngay và trả công muộn. Nới tầm nhìn của nhà phê bình từ 20 tick lên
100 tick không cứu được. Cái gì không được trả công thì không được học.

## 5. Tính hợp lệ của phép đo

Hai lần repo này tưởng đã tìm ra quy luật xã hội, và cả hai lần đó là lỗi của tôi.

- "Xã hội phát triển tới hạn rồi dừng": thật ra kiến thức lưu trong một số nguyên 128 bit (mục 14).
- "Thế giới này thịnh vượng": thật ra nhãn kết cục phụ thuộc vào lúc ta bấm dừng (mục 15).

Bài học: trước khi tin một hiện tượng, hỏi xem nó là thế giới hay là chỗ chứa của mình.

## 6. Tái lập và kiểm soát phiên bản của thí nghiệm

`tools/check.py` chạy mười lăm giây: bài kiểm tra đơn vị, phép thử nối lại thế giới, và **vân tay hành
vi** của bảy cấu hình. Vân tay đổi mà số phiên bản thế giới không đổi thì nó báo lỗi. Bảy trên mười tám
mục trong sổ lý thuyết đã phải rút lại, phần lớn vì luật đổi dưới chân thí nghiệm trước khi có cơ chế này.

## 7. Học tăng cường, đọc từ mã chứ không từ ký hiệu

`sim/src/brain.rs` có đủ và ngắn:

| khái niệm | ở đâu | dài |
|---|---|---|
| lượt truyền xuôi | `think` | 40 dòng |
| chính sách softmax | `act_probs` | 15 dòng |
| sai số dự báo thời gian | `learn_td` | 20 dòng |
| vết đủ điều kiện | `trace_step` | 15 dòng |

`docs/HOC-MAY.md` giải thích từng mảnh từ số không, kể cả lý do bước học phải tự chuẩn hoá: bản viết
đúng công thức sách vở đã giết sạch mọi thế giới, và bảng các lần chạy hỏng nằm ngay trong đó.

## 8. Tiến hoá thần kinh, và giới hạn của nó

Trọng số ở đây do chọn lọc quyết định, không do gradient qua toàn mạng. Ba trần đã đo được:

1. **Không có tái tổ hợp.** Sinh sản vốn vô tính, nên hai dòng họ mỗi bên giải được một nửa không bao
   giờ ghép được. Cờ `--mates` vừa gỡ trần này.
2. **Thế giới không có bài toán nào cần trí thông minh.** Xem mục 3.
3. **Kiến trúc cố định** 105 → 20 → 23.

Thứ tự đó quan trọng: làm mạng to hơn khi trần 1 và 2 còn đó thì chỉ tốn máy.

## 9. Thiết kế thí nghiệm và thống kê

`tools/lab.py` chạy nhánh đối chứng trên cùng dãy seed và báo hiệu số trung bình kèm khoảng tin cậy 95%
bằng bootstrap. Chế độ `screen` tốn một phần chín công sức, và tính hợp lệ của nó được kiểm ngược trên
các thí nghiệm đã chạy đầy đủ. Đọc `docs/lab/README.md` để thấy cái nào tách được nhánh, cái nào không.

## 10. Thứ không có ở đây

Nói thẳng để bạn không mất thời gian tìm: không có truyền ngược qua nhiều lớp, không có tối ưu hoá theo
lô, không có tập kiểm định, không có transformer. Nếu bạn cần những thứ đó thì đây không phải chỗ. Cái
đây có là **kỷ luật thực nghiệm**: đối chứng, khoảng tin cậy, vân tay tái lập, phiên bản thế giới, và
một cuốn sổ ghi cả những phát biểu đã phải rút lại. Trong một dự án máy học thật, đó mới là phần khó.
