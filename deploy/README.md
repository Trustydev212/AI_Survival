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

Muốn giữ lại biên niên sử thì thỉnh thoảng commit `world/chronicle.txt` vào repo bằng tay. Đó là thứ
duy nhất ở đây đáng đọc lại sau nhiều năm.

## Dừng và nối lại

Dừng lúc nào cũng được, mất nhiều nhất 5.000 tick. Bật lại là nó tự nhặt `world.bin` lên chạy tiếp,
đúng như chưa từng dừng: `tools/check.py` kiểm điều đó mỗi lần chạy.
