# Cho thế giới chạy mãi

Một máy VPS nhỏ chạy sim không ngừng, cứ mười lăm phút đẩy một lát cắt lên GitHub Pages. Không cần
GPU, không cần gì đặc biệt: **hai lõi, hai GB RAM, mười GB đĩa là đủ**, khoảng bốn tới sáu đô một
tháng ở Hetzner, Vultr hay DigitalOcean.

## Vì sao phải chạy mãi

Mọi bảng số trong `docs/THEORY.md` đều đo trên một cửa sổ thời gian do người chọn, và mục 15 cho thấy
chính cái cửa sổ ấy quyết định câu trả lời: cùng một thế giới, dừng ở 20.000 tick thì "thịnh vượng",
chạy tới 60.000 thì "bùng và vỡ". Thế giới dài nhất từng chạy ở đây là 60.000 tick. Một VPS chạy liên
tục làm được vài triệu tick mỗi ngày.

## Dựng

```bash
sudo apt update && sudo apt install -y git build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y && . "$HOME/.cargo/env"

sudo git clone <repo> /opt/ai-survival && sudo chown -R "$USER" /opt/ai-survival
cd /opt/ai-survival/sim && cargo build --release && cd ..

./deploy/run.sh            # chạy thử ở tiền cảnh, Ctrl-C để dừng
```

Chạy nền qua systemd, sống lại sau khi khởi động lại máy:

```bash
sudo cp deploy/ai-survival@.service /etc/systemd/system/
sudo systemctl enable --now "ai-survival@$USER"
journalctl -u "ai-survival@$USER" -f
```

## Đưa lên web

`deploy/publish.sh` dựng một trang tĩnh gồm viewer cộng cửa sổ gần đây của thế giới, rồi **force push
thành đúng một commit** lên nhánh `gh-pages`. Một commit là cố ý: vài MB mỗi mười lăm phút sẽ làm repo
phình vô hạn, mà không lát cắt cũ nào đáng giữ. Thứ đáng giữ là biên niên sử và bảng số, chúng nhỏ và
nằm ở nhánh chính.

VPS cần quyền đẩy: tạo một deploy key trên GitHub với quyền ghi, rồi
`git remote set-url origin git@github.com:<bạn>/<repo>.git`.

**Một con số cần biết trước khi bật.** Ở `MAP=384`, một khung hình nặng khoảng nửa MB, nên một cửa sổ
sống đầy đủ là **cỡ 50 MB**, và nó được force push lại mỗi `PUBLISH` giây. Để mặc định 900 giây thì đó
là **khoảng 4-5 GB tải lên mỗi ngày**. Máy chịu được, nhưng nếu đường mạng có hạn mức hoặc GitHub kêu
thì vặn nhỏ lại mà không cần sửa file:

```bash
SNAP_EVERY=1000 ./deploy/run.sh     # 1/5 số khung hình, cửa sổ còn ~10 MB
PUBLISH=3600 ./deploy/run.sh        # đẩy một tiếng một lần thay vì 15 phút
```

Bật Pages trong Settings → Pages → Source: nhánh `gh-pages`. Sau đó mở:

```
https://<bạn>.github.io/<repo>/live.html
```

Trang đó tự mở đúng thế giới đang sống, ở chế độ theo dõi trực tiếp, và dùng được trên điện thoại.

## Thế giới nằm ở đâu

Trong `world/`, không nằm trong git vì nó nặng và đổi liên tục. Đo thật ở `MAP=384`:

| file | là gì |
|---|---|
| `world.bin` | toàn bộ thế giới, **68 MB**, ghi đè mỗi 50.000 tick (khoảng 6 phút) |
| `chronicle.txt` | mỗi dòng một nền văn minh: sống bao lâu, vươn tới đâu |
| `stats_gen<N>.csv` | bảng số của từng thế hệ |
| `events_gen<N>.txt` | sử ký của từng thế hệ |
| `live.bin`, `live.json` | cửa sổ gần đây cho viewer, **tới ~50 MB**, tự bắt đầu lại mỗi 20.000 tick |
| `ark.bin` | vài chục bộ não được mang qua cái chết của thế giới |
| `postmortem.txt` | vì sao mỗi nền văn minh chết, ở tick nào, và thứ sâu nhất họ từng làm ra |

## Vì sao họ chết

"Mọi người đã chết" không phải một nguyên nhân. Một thế giới chạy hàng năm chỉ đáng nhìn nếu mỗi cái
kết phân biệt được với cái kết khác. `postmortem.txt` đọc ngược lại từ chính bảng số của thế hệ đó và
ghi: chết đói trên đất đã kiệt, giết lẫn nhau, dịch bệnh, hay đơn giản là thôi không sinh con nữa. Kèm
theo là đỉnh dân số đạt ở tick nào, suy tàn kéo dài bao lâu, đất và thức ăn còn lại bao nhiêu, và thứ
sâu nhất nền văn minh ấy từng làm ra.

Một ví dụ thật, và nó ngược với điều tôi đoán:

```
== generation 0, tick 4545: grew old with too few born ==
peaked at 111 people on tick 500, then 4045 ticks of decline. In its last stretch:
0 starved, 0 killed, 0 taken by plague, 1 died old, 0 born.
Soil 100% at its peak and 100% at the end, 105505 food left standing.
Deepest thing made: Kuhelin (3 deep): bind(hollow(clay), hollow(sharpen(wood))) -> vessel
```

Họ không chết đói. Đất còn nguyên vẹn, thức ăn còn hơn trăm nghìn đơn vị. Họ chỉ **thôi sinh con**, rồi
già đi suốt bốn nghìn tick. Nếu chỉ ghi "mọi người đã chết" thì không ai biết điều đó.

## Nhật ký đọc được từ xa

`deploy/publish-log.sh` đẩy bốn file lên nhánh `world-log` của chính repo, mỗi lần một commit duy
nhất bị ghi đè: `status.json`, `chronicle.txt`, `postmortem.txt` và `ark.bin`. Tổng vài trăm KB.
Ghi đè là cố ý, vì giữ lịch sử thì một năm chạy thành vài GB, mà lịch sử thật đã nằm trong nội dung
biên niên sử rồi.

Nó chạy cùng nhịp với việc đưa lên web, mười lăm phút một lần, và hỏng thì bỏ qua: thế giới không
được dừng chỉ vì mạng trục trặc.

Nhờ đó bất kỳ ai, kể cả một phiên làm việc mới, cũng đọc được thế giới đang ở đâu và các nền văn minh
đã chết vì gì, mà không cần đăng nhập vào máy. Và `ark.bin` cho phép chấm điểm bộ não từ xa:

```bash
git fetch origin world-log && git checkout origin/world-log -- ark.bin
sim --eval ark.bin --ticks 2500
```

## Thứ duy nhất sống sót qua ngày tận thế

Trước đây khi mọi người chết, thế giới mới bắt đầu với bộ não **ngẫu nhiên hoàn toàn**. Chạy một triệu
tick mà cứ vài chục nghìn tick lại xoá sạch thì không tích luỹ được gì: mỗi nền văn minh học lại đúng
những bài học đầu tiên.

`ark.bin` giữ khoảng hai chục bộ não làm tốt nhất, mỗi dòng họ một bộ, đo bằng thước duy nhất thế giới
tự có: **để lại bao nhiêu con, so với những người đang sinh đẻ cùng lúc đó**. Phần "so với" không phải
chi tiết nhỏ. Trước đây đo bằng số con tuyệt đối, và đó là một kỷ lục không bao giờ hạ: dòng họ nào đã
sống qua một thời sung túc thì mọi kẻ đến sau phải vượt kỷ lục ấy, nên ark đóng cửa. Chạy 32.000 tick
ghi ra file **giống hệt từng byte** file ở tick 15.500 — hơn nửa lần chạy không tích luỹ được gì.

Một nửa số bộ lạc của thế giới mới sinh ra từ ark kèm đột biến, nửa còn lại vẫn từ nhiễu. Cố ý chia
đôi: toàn bộ từ ark thì mọi thế giới thành bản sao của nhau, toàn bộ từ nhiễu thì không có gì tích luỹ. Chọn lọc nhờ đó vươn qua được cái chết của cả nền văn minh.

Thử sáu ca liên tiếp: thế hệ 0 sống 4.545 tick rồi chết, thế hệ 1 dựng từ 24 bộ não mang qua và sống
hơn 19.000 tick, vẫn đang sống. Một mẫu thì chưa nói lên điều gì; biên niên sử chạy dài mới trả lời được.

Muốn giữ lại biên niên sử thì thỉnh thoảng commit `world/chronicle.txt` vào repo bằng tay. Đó là thứ
duy nhất ở đây đáng đọc lại sau nhiều năm.

## Dừng và nối lại

Dừng lúc nào cũng được, mất nhiều nhất 50.000 tick — cỡ vài phút. Bật lại là nó tự nhặt `world.bin`
lên chạy tiếp, đúng như chưa từng dừng: `tools/check.py` kiểm điều đó mỗi lần chạy.

Hai bản `run.sh` cùng trỏ vào một thư mục thì thay nhau ghi đè một file lưu và thế giới bắt đầu chạy
**lùi**. Chuyện đó đã xảy ra một lần ở đây (tick 16.000 tụt về 12.000, phải bỏ cả thư mục), nên giờ
`run.sh` giữ một khoá: bản thứ hai in ra lý do rồi thoát.

Khi nào luật thế giới đổi phiên bản (`sim/src/version.rs`), bản lưu cũ **không** được đọc — nó in lý
do rồi bắt đầu một thế giới mới. `ark.bin` thì vẫn giữ, nên anh mất một nền văn minh đang chạy chứ
không mất kho não đã tích luỹ.
