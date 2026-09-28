CREATE TABLE "SupporterInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "SupporterInfoIndex" TEXT, -- References CostumeBaseInfo.InvenIndex
    "GuildSupporterInfoIndex" TEXT -- References CostumeBaseInfo.InvenIndex
);