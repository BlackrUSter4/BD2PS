CREATE TABLE "GuildSupporterBattleCharInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInfoIndex" BIGINT, -- References CharInfo.InvenIndex
    "CostumeInfoIndex" TEXT, -- References CostumeInfo.InvenIndex
    "EquipInfoIndex" TEXT, -- References EquipInfo.InvenIndex
    "AwakeInfoIndex" BIGINT -- References CharAwakeInfo.InvenIndex
);