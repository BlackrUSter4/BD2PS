CREATE TABLE "DelayedVisibilityScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Type" TEXT,
    "Id" INTEGER,
    "TableId" INTEGER,
    "OpenDate" BIGINT
);
