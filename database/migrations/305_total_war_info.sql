CREATE TABLE "TotalWarInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ScoreInfoIndex" TEXT, -- References BattleDamageInfo.InvenIndex
    "TopPercent" REAL,
    "TopRankerScore" BIGINT,
    "EngineType" TEXT
);