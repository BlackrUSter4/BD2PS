CREATE TABLE "MiniGameSurvivalInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "ActiveCharId" INTEGER NOT NULL, -- Parallel array with active_char_id, active_map_group_id
    "ActiveMapGroupId" INTEGER NOT NULL, -- Parallel array with active_char_id, active_map_group_id
    "UserRankScore" INTEGER,
    "TopRankOwnerIndex" BIGINT,
    "TopRankUserId" TEXT,
    "TopRankScore" INTEGER,
    "StageClearInfoIndex" TEXT, -- References MiniGameSurvivalStageClearInfo.InvenIndex
    "CollectionInfoIndex" TEXT, -- References MiniGameSurvivalCollectionInfo.InvenIndex
    "UpgradeInfoIndex" TEXT -- References MiniGameSurvivalUpgradeInfo.InvenIndex
);