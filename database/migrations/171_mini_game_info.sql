CREATE TABLE "MiniGameInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "LastRewardPoint" INTEGER,
    "BestRecordValue" INTEGER,
    "IsPossibleQuickReward" INTEGER,
    "WorldBestRecordOwnerIndex" BIGINT,
    "WorldBestRecordUserId" TEXT,
    "WorldBestRecordValue" INTEGER,
    "WorldBestRecordPlayInfo" TEXT
);