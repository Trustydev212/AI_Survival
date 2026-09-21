# Phòng thí nghiệm: mọi thí nghiệm ở một trang

Mỗi dòng là một câu hỏi; mỗi nhánh chỉ khác đối chứng đúng một cờ và chạy trên cùng dãy seed.
"Tốt" là số thế giới flourishing hoặc surviving. "Khác 0" là các chỉ số mà hiệu số so với đối
chứng có khoảng tin cậy 95% không chứa 0 (dấu là chiều của nhánh so với đối chứng).

| thí nghiệm | câu hỏi | nhánh: tốt | khác 0 |
|---|---|---|---|
| [learning](learning.md) | Học trong đời có đáng không? | default 9/16; no-learning 8/16; fast-learning 6/16 | no-learning: final_level +3.50; no-learning: plastic −0.01; fast-learning: final_level +4.06; fast-learning: plastic +0.04; fast-learning: loudness +0.26 |
| [hearing](hearing.md) | Nghe nhau có ích gì? | default 9/16; deaf 6/16; kin-only 8/16 | deaf: innovations −62.56; deaf: final_level +2.12; deaf: signal_mi −0.02; deaf: crafts −58.88; kin-only: innovations −52.62; kin-only: final_level +2.38; kin-only: crafts −46.50 |
| [population](population.md) | Đông người thì phát minh nhiều hơn không? | default 9/16; few 4/16; many 8/16 | few: innovations −79.44; few: equipped_share −0.21; few: crafts −70.88; many: peak_pop +3789; many: final_pop +1257; many: final_level +4.06 |
| [customs](customs.md) | Tập quán có tự giữ đất khi không cần thủ lĩnh không? | default 9/16; no-customs 7/16; no-orders 12/16 | no-customs: innovations −48.56; no-customs: final_level +2.25; no-orders: obedience −0.34; no-orders: final_level +3.25 |
| [crafting](crafting.md) | Chế tác có đáng cái giá của nó không? | default 9/16; no-crafting 7/16 | no-crafting: peak_pop +2528; no-crafting: innovations −103; no-crafting: mean_known −39.82; no-crafting: soil_health +0.08; no-crafting: lived_soil +0.08; no-crafting: settled_share +0.27; no-crafting: breed_rate −3.14; no-crafting: things_per_head −0.89; no-crafting: equipped_share −0.33; no-crafting: crafts −110 |
| [herds](herds.md) | Có việc cần nhiều tay thì tiếng gọi có nghĩa không? | default 9/16; herds 5/16 | herds: innovations −55.31; herds: final_level +1.88; herds: crafts −46.06; herds: hunts +0.81 |
| [brains](brains.md) | Học từ hậu quả có làm xã hội khá hơn không? | default 9/16; sample-only 6/16; gradient 3/16; gradient-social 7/16; gradient-slow 9/16 | sample-only: mean_known +31.04; sample-only: plastic −0.01; gradient: peak_pop −1875; gradient: final_pop −660; gradient: innovations −88.62; gradient: mean_known −38.20; gradient: settled_share −0.30; gradient: obedience −0.24; gradient: breed_rate −6.27; gradient: final_level −1.69; gradient: plastic −0.01; gradient: signal_mi −0.03; gradient: signal_meaning −0.04; gradient: things_per_head −0.63; gradient: equipped_share −0.20; gradient: crafts −73.69; gradient: learn_rate −1.47; gradient: loudness −0.78; gradient: division_of_labour −0.24; gradient-social: peak_pop −1487; gradient-social: final_pop −356; gradient-social: settled_share −0.26; gradient-social: breed_rate −3.65; gradient-slow: plastic −0.01 |

_Sinh bởi `python3 tools/lab.py index`. Báo cáo đầy đủ của từng thí nghiệm nằm trong file cùng tên._
