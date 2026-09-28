CREATE TABLE "RecommendDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "UserExp" INTEGER,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "TitleId" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "SortValue" BIGINT
);