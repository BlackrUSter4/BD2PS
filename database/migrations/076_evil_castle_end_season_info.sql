CREATE TABLE "EvilCastleEndSeasonInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Rank" INTEGER,
    "StageIndex" INTEGER,
    "Point" INTEGER,
    "IsRewarded" INTEGER,
    "RewardInfoIndex" TEXT -- References RewardInfo.InvenIndex
);