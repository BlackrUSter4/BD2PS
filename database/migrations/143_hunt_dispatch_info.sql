CREATE TABLE "HuntDispatchInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "HuntDispatchGroupId" INTEGER,
    "HuntDispatchId" INTEGER,
    "Count" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT,
    "DecreaseFreeApCount" INTEGER,
    "DecreaseCashApCount" INTEGER
);