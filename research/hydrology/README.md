# Freshwater evidence ingestion

Al Wusta and Alathar have primary sediment/diatom evidence for freshwater, not merely inferred rainfall. Their source chronology and site locations are now linked in observations.json. Exact source-member hashes and the reviewed2021correction are retained separately. The two site locations come from2021Supplementary Table1, which warns coordinates are approximate in some cases.

Neither a fossil age nor a sediment's uncertainty interval identifies a continuous period of water availability. Alathar also shows occasional desiccation. Records therefore retain null activation interval, volume and polygon; they are not passed into travel eligibility as quantitative water fields. Water quantity, active duration, access footprint, age model and coverage still require work. No unknown area is coded waterless.

The first empirical food-departure candidate failed held-out evaluation. The Rust food ledger in population/resources.rs separately prepares the accounting mechanism: edible-kcal harvest, explicit replenishment, consumption and spoilage balance, with unknown standing stock kept distinct from zero. It accepts measured harvest without inferring how much food remains. It contains no ancient food production, storage decay, caloric requirements, regrowth or famine parameters, and is not connected to the regional population run yet.
