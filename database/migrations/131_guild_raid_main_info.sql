CREATE TABLE "GuildRaidMainInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER,
    "RaidDay" INTEGER,
    "TodayNormalBattleCount" INTEGER,
    "UserScore" BIGINT,
    "LastScoreRewardId" INTEGER,
    "GuildTotalScore" BIGINT,
    "GuildTopPercent" REAL,
    "GolemLevel" INTEGER,
    "GolemExp" INTEGER,
    "ObtainableSeasonReward" INTEGER,
    "TodaySupporterUseCount" INTEGER,
    "TotalSupporterRentalCount" INTEGER,
    "TopGuildScore" BIGINT,
    "PlayDay" INTEGER NOT NULL,
    "ScheduleHistoryInfoIndex" TEXT, -- References ContentScheduleHistory.InvenIndex
    "FlagGradeVersionInfoIndex" TEXT, -- References GuildRaidFlagGradeVersionInfo.InvenIndex
    "GuildRank" INTEGER
);