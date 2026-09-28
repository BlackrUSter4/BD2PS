CREATE TABLE "GuildRaidBossBattleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GuildTotalScore" BIGINT,
    "GuildTopPercent" REAL,
    "HighestLevel" INTEGER,
    "HighestScore" BIGINT,
    "TopMemberOwnerIndex" BIGINT,
    "TopMemberUserId" TEXT,
    "TopMemberScore" INTEGER
);