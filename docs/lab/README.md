# Phòng thí nghiệm: mọi thí nghiệm ở một trang

Mỗi dòng là một câu hỏi; mỗi nhánh chỉ khác đối chứng đúng một cờ và chạy trên cùng dãy seed.
"Tốt" là số thế giới flourishing hoặc surviving. "Khác 0" là các chỉ số mà hiệu số so với đối
chứng có khoảng tin cậy 95% không chứa 0 (dấu là chiều của nhánh so với đối chứng).

| thí nghiệm | câu hỏi | nhánh: tốt | khác 0 |
|---|---|---|---|
| [learning](learning.md) | Học trong đời có đáng không? | default 0/0; no-learning 8/16; fast-learning 6/16 | – |
| [hearing](hearing.md) | Nghe nhau có ích gì? | default 0/0; deaf 6/16; kin-only 8/16 | – |
| [population](population.md) | Đông người thì phát minh nhiều hơn không? | default 0/0; few 4/16; many 8/16 | – |
| [customs](customs.md) | Tập quán có tự giữ đất khi không cần thủ lĩnh không? | default 0/0; no-customs 7/16; no-orders 12/16 | – |
| [crafting](crafting.md) | Chế tác có đáng cái giá của nó không? | default 0/0; no-crafting 7/16 | – |
| [herds](herds.md) | Có việc cần nhiều tay thì tiếng gọi có nghĩa không? | default 0/0; herds 5/16 | – |

_Sinh bởi `python3 tools/lab.py index`. Báo cáo đầy đủ của từng thí nghiệm nằm trong file cùng tên._
