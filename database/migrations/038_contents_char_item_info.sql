CREATE TABLE "ContentsCharItemInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInvenIndex" BIGINT,
    "EquipInfoIndex" TEXT, -- References ContentsEquipInfo.InvenIndex
    "ConnectPotentialCostume" INTEGER
);