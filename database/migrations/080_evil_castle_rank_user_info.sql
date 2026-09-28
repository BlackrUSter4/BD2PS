CREATE TABLE "EvilCastleRankUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "UserExp" INTEGER,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "Rank" INTEGER,
    "Point" INTEGER,
    "TitleId" INTEGER,
    "Date" BIGINT
);