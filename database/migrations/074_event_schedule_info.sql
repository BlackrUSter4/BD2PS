CREATE TABLE "EventScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "EventType" TEXT,
    "EventId" INTEGER,
    "EventSubId" INTEGER,
    "StartDate" BIGINT,
    "EndDate" BIGINT,
    "IsActive" INTEGER
);