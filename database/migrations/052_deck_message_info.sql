CREATE TABLE "DeckMessageInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInfoIndex" BIGINT, -- References CharInfo.InvenIndex
    "EquipInfoIndex" TEXT, -- References EquipInfo.InvenIndex
    "CostumeInfoIndex" TEXT, -- References CostumeInfo.InvenIndex
    "CharAwakeInfoIndex" BIGINT -- References CharAwakeInfo.InvenIndex
);