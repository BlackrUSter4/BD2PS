CREATE TABLE "MiniGameBoardInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "ScaffoldGroupId" INTEGER,
    "ScaffoldId" INTEGER,
    "CompleteCount" INTEGER
);