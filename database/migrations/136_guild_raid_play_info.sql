CREATE TABLE "GuildRaidPlayInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "BossScore" BIGINT,
    "TotalScore" INTEGER,
    "TopPercent" REAL,
    "IsPlayRaidToday" INTEGER,
    "IsNormalBattlePlay" INTEGER,
    "BattleMode" TEXT,
    "Rank" INTEGER
);