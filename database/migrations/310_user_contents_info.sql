CREATE TABLE "UserContentsInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "TitleId" INTEGER,
    "PortraitCostumeId" INTEGER,
    "Greeting" TEXT,
    "PvpSeason" INTEGER,
    "PvpVp" INTEGER,
    "PvpRank" INTEGER,
    "MonsterhuntRank" INTEGER,
    "LikeCount" INTEGER,
    "IsAllPrivate" INTEGER,
    "Options" INTEGER NOT NULL, -- Parallel array with options, sort_id
    "IsFriend" INTEGER,
    "RoomInfoIndex" BIGINT, -- References MyRoomUserInfo.InvenIndex
    "TotalWarScore" BIGINT,
    "TotalBattlePower" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "MyRoomLikeCount" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "GuildRaidRank" INTEGER,
    "GuildRaidScore" BIGINT,
    "EvilCastleGreedTowerTopFloor" INTEGER,
    "EvilCastleRageTowerTopFloor" INTEGER,
    "EvilCastleEnvyTowerTopFloor" INTEGER,
    "EvilCastleRogueLikeLevel" INTEGER,
    "AchievementLevel" INTEGER,
    "SortId" INTEGER NOT NULL, -- Parallel array with options, sort_id
    "IdCardInfoIndex" BIGINT, -- References IdCardInfo.InvenIndex
    "MonsterhuntRankTopPercent" REAL,
    "SupporterInfoIndex" TEXT -- References CostumeBaseInfo.InvenIndex
);