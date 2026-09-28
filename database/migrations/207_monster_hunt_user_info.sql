CREATE TABLE "MonsterHuntUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER,
    "MonsterHuntId" INTEGER,
    "Level" INTEGER,
    "StartHp" BIGINT,
    "HighestFirstTurnDamage" INTEGER,
    "HighestHp" BIGINT,
    "HighestHpDate" BIGINT,
    "CurrentLevelHighestDamage" BIGINT,
    "DailyHighestDamage" BIGINT,
    "SeasonReward" INTEGER,
    "DailyRewardLevel" INTEGER,
    "DailyRewardDate" BIGINT
);