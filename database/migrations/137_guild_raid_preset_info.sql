CREATE TABLE "GuildRaidPresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PresetInfoIndex" TEXT -- References PresetInfo.InvenIndex
);