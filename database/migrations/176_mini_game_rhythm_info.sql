CREATE TABLE "MiniGameRhythmInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PlayInfoIndex" TEXT, -- References MiniGameRhythmPlayInfo.InvenIndex
    "RewardInfoBundleIndex" BIGINT -- References RewardInfoBundle.InvenIndex
);