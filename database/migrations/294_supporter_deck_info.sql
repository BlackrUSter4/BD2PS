CREATE TABLE "SupporterDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "TitleId" INTEGER,
    "PortraitCostumeId" INTEGER,
    "Greeting" TEXT,
    "IsFriend" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "PortraitCostumeDesignId" INTEGER,
    "UsageCount" INTEGER
);