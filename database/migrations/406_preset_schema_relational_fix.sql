-- Preset(Info/DeckInfo/DeckEquipInfo) previously used a vague TEXT comma-list ("DeckInfoIndex"/
-- "EquipInfoIndex") with no real FK back from child to parent, and PresetDeckInfo had no
-- Position/Sequence columns even though PresetDeckDBInfo.deck_base_info is a full DeckDBInfo
-- (char_inven_index+position+sequence). Recreated relationally, matching the already-working
-- ColosseumPresetInfo/ColosseumPresetDeckInfo/ColosseumPresetDeckEquipInfo schema shape exactly.
-- Never exercised before now (stub), safe to recreate.
DROP TABLE IF EXISTS "PresetInfo";
CREATE TABLE "PresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Slot" INTEGER NOT NULL,
    "PresetName" TEXT,
    "PresetResourceId" INTEGER,
    "PresetResourceColor" INTEGER
);

DROP TABLE IF EXISTS "PresetDeckInfo";
CREATE TABLE "PresetDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "PresetInfoIndex" INTEGER NOT NULL,
    "CharInvenIndex" BIGINT NOT NULL,
    "Position" INTEGER,
    "Sequence" INTEGER,
    "CostumeInvenIndex" BIGINT,
    "Team" INTEGER
);

DROP TABLE IF EXISTS "PresetDeckEquipInfo";
CREATE TABLE "PresetDeckEquipInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "PresetDeckInfoIndex" INTEGER NOT NULL,
    "EquipType" INTEGER,
    "EquipInvenIndex" BIGINT
);
