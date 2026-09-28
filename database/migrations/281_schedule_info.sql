CREATE TABLE "ScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ContentId" INTEGER,
    "CurrentSeasonIndex" INTEGER,
    "NextSeasonIndex" INTEGER
);
