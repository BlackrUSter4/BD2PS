-- PresetUseEquipInfo.equip_inven_index is a non-optional Rust model field with no backing
-- column (SELECT * would have errored the first time this table was ever queried). The proto
-- field is `repeated int64 equip_inven_index` per char, so one row per (Uid, CharInvenIndex,
-- equip value) is used, same one-row-per-list-element pattern as the Event/Deck fixes this
-- round. Never exercised before now (stub), safe to recreate.
DROP TABLE IF EXISTS "PresetUseEquipInfo";
CREATE TABLE "PresetUseEquipInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInvenIndex" BIGINT,
    "EquipInvenIndex" BIGINT NOT NULL
);
