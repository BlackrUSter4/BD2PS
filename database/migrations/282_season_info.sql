CREATE TABLE "SeasonInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT,
    "ErrorFlag" INTEGER,
    "ReturnFlag" INTEGER,
    "RankRewardGroupId" INTEGER
);