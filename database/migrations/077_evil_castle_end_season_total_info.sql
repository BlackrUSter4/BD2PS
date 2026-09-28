CREATE TABLE "EvilCastleEndSeasonTotalInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Rank" INTEGER,
    "Point" INTEGER,
    "IsRewarded" INTEGER,
    "RewardInfoIndex" TEXT -- References RewardInfo.InvenIndex
);