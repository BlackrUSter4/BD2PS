CREATE TABLE "MonsterHuntPresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PresetInfoIndex" TEXT -- References PresetInfo.InvenIndex
);