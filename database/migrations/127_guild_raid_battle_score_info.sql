CREATE TABLE "GuildRaidBattleScoreInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "DefaultScore" INTEGER,
    "TurnBonusScore" INTEGER,
    "GolemLevelBonusScore" INTEGER,
    "SupportBonusScore" INTEGER,
    "GuildTotalScore" BIGINT
);