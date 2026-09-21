# Phòng thí nghiệm: mọi thí nghiệm ở một trang

Mỗi dòng là một câu hỏi; mỗi nhánh chỉ khác đối chứng đúng một cờ và chạy trên cùng dãy seed.
"Tốt" là số thế giới flourishing hoặc surviving. "Khác 0" là các chỉ số mà hiệu số so với đối
chứng có khoảng tin cậy 95% không chứa 0 (dấu là chiều của nhánh so với đối chứng).

| thí nghiệm | câu hỏi | nhánh: tốt | khác 0 |
|---|---|---|---|
| [learning](learning.md) | Học trong đời có đáng không? | default 9/16; no-learning 8/16; fast-learning 0/0 | no-learning: plastic −0.01 |
| [hearing](hearing.md) | Nghe nhau có ích gì? | default 9/16; deaf 6/16; kin-only 8/16 | deaf: signal_mi −0.04 |
| [customs](customs.md) | Tập quán có tự giữ đất khi không cần thủ lĩnh không? | default 9/16; no-customs 4/16; no-orders 5/16 | no-customs: obedience −0.36; no-customs: signal_mi −0.03; no-orders: obedience −0.43; no-orders: breed_rate −3.23; no-orders: swing +5.19 |
| [crafting](crafting.md) | Chế tác có đáng cái giá của nó không? | default 9/16; no-crafting 6/16 | no-crafting: innovations −62.12; no-crafting: mean_known −37.19; no-crafting: breed_rate −3.36; no-crafting: final_level −2.50; no-crafting: things_per_head −0.81; no-crafting: equipped_share −0.32; no-crafting: crafts −76.31 |
| [herds](herds.md) | Có việc cần nhiều tay thì tiếng gọi có nghĩa không? | default 9/16; herds 5/16 | herds: hunts +0.81 |

_Sinh bởi `python3 tools/lab.py index`. Báo cáo đầy đủ của từng thí nghiệm nằm trong file cùng tên._
