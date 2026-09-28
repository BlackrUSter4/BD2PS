CREATE TABLE "MiniGameActionInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "MyBestRecordIndex" TEXT, -- References MiniGameActionMyBestRecord.InvenIndex
    "ClearMissionId" INTEGER NOT NULL
);