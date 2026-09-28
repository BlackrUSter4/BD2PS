CREATE TABLE "PresetDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "DeckBaseInfoIndex" BIGINT, -- References DeckInfo.InvenIndex
    "CostumeInvenIndex" BIGINT,
    "EquipInfoIndex" TEXT, -- References PresetDeckEquipInfo.InvenIndex
    "Team" INTEGER
);