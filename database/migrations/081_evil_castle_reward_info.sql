CREATE TABLE "EvilCastleRewardInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EndSeasonInfoIndex" TEXT, -- References EvilCastleEndSeasonInfo.InvenIndex
    "EndSeasonTotalInfoIndex" BIGINT, -- References EvilCastleEndSeasonTotalInfo.InvenIndex
    "RewardInfoBundleIndex" BIGINT -- References RewardInfoBundle.InvenIndex
);