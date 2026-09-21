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
sudo cp deploy/ai-survival.service /etc/systemd/system/
sudo systemctl enable --now ai-survival
journalctl -u ai-survival -f
```

## Đưa lên web

`deploy/publish.sh` dựng một trang tĩnh gồm viewer cộng cửa sổ gần đây của thế giới, rồi **force push
thành đúng một commit** lên nhánh `gh-pages`. Một commit là cố ý: vài MB mỗi mười lăm phút sẽ làm repo
phình vô hạn, mà không lát cắt cũ nào đáng giữ. Thứ đáng giữ là biên niên sử và bảng số, chúng nhỏ và
nằm ở nhánh chính.

VPS cần quyền đẩy: tạo một deploy key trên GitHub với quyền ghi, rồi
`git remote set-url origin git@github.com:<bạn>/<repo>.git`.

Bật Pages trong Settings → Pages → Source: nhánh `gh-pages`. Sau đó mở:

```
https://<bạn>.github.io/<repo>/live.html
```

Trang đó tự mở đúng thế giới đang sống, ở chế độ theo dõi trực tiếp, và dùng được trên điện thoại.

## Thế giới nằm ở đâu

Trong `world/`, không nằm trong git vì nó nặng vài chục MB và đổi liên tục:

| file | là gì |
|---|---|
| `world.bin` | toàn bộ thế giới, lưu lại mỗi 5.000 tick |
| `chronicle.txt` | mỗi dòng một nền văn minh: sống bao lâu, vươn tới đâu |
| `stats_gen<N>.csv` | bảng số của từng thế hệ |
| `events_gen<N>.txt` | sử ký của từng thế hệ |
| `live.bin`, `live.json` | cửa sổ gần đây cho viewer, tự bắt đầu lại mỗi 20.000 tick |

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

## Thứ duy nhất sống sót qua ngày tận thế

Trước đây khi mọi người chết, thế giới mới bắt đầu với bộ não **ngẫu nhiên hoàn toàn**. Chạy một triệu
tick mà cứ vài chục nghìn tick lại xoá sạch thì không tích luỹ được gì: mỗi nền văn minh học lại đúng
những bài học đầu tiên.

`ark.bin` giữ khoảng hai chục bộ não làm tốt nhất, đo bằng thước duy nhất thế giới tự có là **để lại bao
nhiêu con**, mỗi dòng họ một bộ. Một nửa số bộ lạc của thế giới mới sinh ra từ đó kèm đột biến, nửa còn
lại vẫn từ nhiễu. Cố ý chia đôi: toàn bộ từ ark thì mọi thế giới thành bản sao của nhau, toàn bộ từ nhiễu
thì không có gì tích luỹ. Chọn lọc nhờ đó vươn qua được cái chết của cả nền văn minh.

Thử sáu ca liên tiếp: thế hệ 0 sống 4.545 tick rồi chết, thế hệ 1 dựng từ 24 bộ não mang qua và sống
hơn 19.000 tick, vẫn đang sống. Một mẫu thì chưa nói lên điều gì; biên niên sử chạy dài mới trả lời được.

Muốn giữ lại biên niên sử thì thỉnh thoảng commit `world/chronicle.txt` vào repo bằng tay. Đó là thứ
duy nhất ở đây đáng đọc lại sau nhiều năm.

## Dừng và nối lại

Dừng lúc nào cũng được, mất nhiều nhất 5.000 tick. Bật lại là nó tự nhặt `world.bin` lên chạy tiếp,
đúng như chưa từng dừng: `tools/check.py` kiểm điều đó mỗi lần chạy.
