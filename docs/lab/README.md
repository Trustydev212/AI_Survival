# Phòng thí nghiệm: mọi thí nghiệm ở một trang

Mỗi dòng là một câu hỏi; mỗi nhánh chỉ khác đối chứng đúng một cờ và chạy trên cùng dãy seed.
"Tốt" là số thế giới flourishing hoặc surviving. "Khác 0" là các chỉ số mà hiệu số so với đối
chứng có khoảng tin cậy 95% không chứa 0 (dấu là chiều của nhánh so với đối chứng).

| thí nghiệm | câu hỏi | nhánh: tốt | khác 0 |
|---|---|---|---|
| [learning](learning.md) | Học trong đời có đáng không? | default 9/16; no-learning 8/16; fast-learning 6/16 | no-learning: plastic −0.01; fast-learning: plastic +0.04 |
| [hearing](hearing.md) | Nghe nhau có ích gì? | default 9/16; deaf 6/16; kin-only 8/16 | deaf: signal_mi −0.04 |
| [population](population.md) | Đông người thì phát minh nhiều hơn không? | default 9/16; few 4/16; many 8/16 | few: innovations −39.50; few: signal_mi −0.03; few: equipped_share −0.19; few: crafts −37.69; many: peak_pop +3466; many: final_pop +1066; many: plastic +0.01 |
| [customs](customs.md) | Tập quán có tự giữ đất khi không cần thủ lĩnh không? | default 9/16; no-customs 7/16; no-orders 12/16 | no-customs: obedience −0.24; no-orders: obedience −0.43 |
| [crafting](crafting.md) | Chế tác có đáng cái giá của nó không? | default 9/16; no-crafting 7/16 | no-crafting: innovations −62.81; no-crafting: mean_known −35.22; no-crafting: soil_health +0.08; no-crafting: lived_soil +0.07; no-crafting: final_level −2.19; no-crafting: things_per_head −0.81; no-crafting: equipped_share −0.32; no-crafting: crafts −76.31 |
| [herds](herds.md) | Có việc cần nhiều tay thì tiếng gọi có nghĩa không? | default 9/16; herds 5/16 | herds: hunts +0.81 |
| [brains](brains.md) | Học từ hậu quả có làm xã hội khá hơn không? | default 9/16; sample-only 6/16; gradient 2/16; gradient-social 9/16; gradient-slow 0/0 | sample-only: plastic −0.01; gradient: peak_pop −2226; gradient: final_pop −848; gradient: innovations −54.38; gradient: mean_known −37.01; gradient: settled_share −0.35; gradient: obedience −0.33; gradient: breed_rate −5.46; gradient: final_level −3.44; gradient: plastic −0.01; gradient: signal_mi −0.04; gradient: signal_meaning −0.03; gradient: things_per_head −0.58; gradient: equipped_share −0.19; gradient: crafts −46.00; gradient: learn_rate −1.50; gradient: loudness −0.79; gradient: division_of_labour −0.26; gradient-social: peak_pop −1832; gradient-social: final_pop −568; gradient-social: settled_share −0.33; gradient-social: signal_mi −0.04 |

_Sinh bởi `python3 tools/lab.py index`. Báo cáo đầy đủ của từng thí nghiệm nằm trong file cùng tên._
