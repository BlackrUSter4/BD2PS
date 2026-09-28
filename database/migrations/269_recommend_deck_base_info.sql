CREATE TABLE "RecommendDeckBaseInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "BattlePower" INTEGER,
    "DeckInfoIndex" TEXT, -- References DeckInfo.InvenIndex
    "TotalWarDeckInfoIndex" TEXT, -- References TotalWarDeckInfo.InvenIndex
    "CharInfoIndex" TEXT, -- References CharInfo.InvenIndex
    "CostumeInfoIndex" TEXT, -- References CostumeInfo.InvenIndex
    "EquipInfoIndex" TEXT, -- References EquipInfo.InvenIndex
    "AwakeInfoIndex" TEXT -- References CharAwakeInfo.InvenIndex
);