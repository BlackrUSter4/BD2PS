CREATE TABLE "EvilCastleRogueLikeRankUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "UserExp" INTEGER,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "Rank" INTEGER,
    "Score" INTEGER,
    "TitleId" INTEGER,
    "CrystalDamage" BIGINT
);