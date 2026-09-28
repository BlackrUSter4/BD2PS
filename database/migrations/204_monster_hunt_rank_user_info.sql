CREATE TABLE "MonsterHuntRankUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "UserExp" INTEGER,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "Rank" INTEGER,
    "Score" REAL,
    "TitleId" INTEGER,
    "RankTopPercent" REAL
);