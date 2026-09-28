CREATE TABLE "EquipPresetCharInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInvenIndex" BIGINT,
    "PresetInfoIndex" TEXT -- References EquipPresetInfo.InvenIndex
);