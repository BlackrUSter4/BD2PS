CREATE TABLE "EquipStorageInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EquipInfoIndex" TEXT -- References EquipInfo.InvenIndex
);