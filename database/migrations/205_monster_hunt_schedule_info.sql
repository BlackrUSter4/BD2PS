CREATE TABLE "MonsterHuntScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "SeasonInfo" TEXT,
    "MonsterHuntId" INTEGER,
    "InfoOpenDay" INTEGER,
    "CalculateEndDate" BIGINT,
    "ErrorFlag" INTEGER,
    "IndependentFlag" INTEGER,
    "RankRewardGroupId" INTEGER
);