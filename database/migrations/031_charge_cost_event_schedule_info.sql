CREATE TABLE "ChargeCostEventScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ScheduleIndex" INTEGER,
    "ItemType" INTEGER,
    "MaxCount" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT
);