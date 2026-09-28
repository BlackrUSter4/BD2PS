CREATE TABLE "GuildRaidMemberRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Rank" INTEGER,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "Score" BIGINT,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "TitleId" INTEGER,
    "OverKillDamage" BIGINT
);