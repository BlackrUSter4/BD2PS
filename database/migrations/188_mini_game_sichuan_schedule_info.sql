CREATE TABLE "MiniGameSichuanScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "InfoIndex" TEXT, -- References MiniGameSichuanInfo.InvenIndex
    "WorldBestRecordOwnerIndex" BIGINT,
    "WorldBestRecordUserId" TEXT,
    "WorldBestRecordValue" REAL,
    "BestRecordValue" REAL,
    "IsBlock" INTEGER
);