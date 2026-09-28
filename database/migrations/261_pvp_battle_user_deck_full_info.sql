CREATE TABLE "PvpBattleUserDeckFullInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInfoIndex" TEXT, -- References CharInfo.InvenIndex
    "CostumeInfoIndex" TEXT, -- References CostumeInfo.InvenIndex
    "EquipInfoIndex" TEXT, -- References EquipInfo.InvenIndex
    "BuffStatInfoIndex" TEXT, -- References PictorialBuffStatInfo.InvenIndex
    "AwakeInfoIndex" TEXT -- References CharAwakeInfo.InvenIndex
);