CREATE TABLE "GuildRaidSeasonRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Rank" INTEGER,
    "Score" BIGINT,
    "GuildIndex" BIGINT,
    "GuildName" TEXT,
    "Message" TEXT,
    "Icon" INTEGER,
    "IconColor" TEXT,
    "FlagGrade" INTEGER,
    "MemberCount" INTEGER,
    "OverKillDamage" BIGINT
);