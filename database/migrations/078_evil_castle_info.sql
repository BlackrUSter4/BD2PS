CREATE TABLE "EvilCastleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Rank" INTEGER,
    "StageIndex" INTEGER,
    "Retry" INTEGER,
    "Point" INTEGER,
    "SeasonHighestPoint" INTEGER,
    "IsRewarded" INTEGER,
    "StageClearTime" INTEGER
);